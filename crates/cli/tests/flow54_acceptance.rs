//! **The M53 fix pass's done-picture acceptance suite** — the pass driven end to end
//! through the **real `jigc` binary** (`design/worked-examples.md` → flow 54; roadmap →
//! Milestone 53, Increment 6; the arm set is
//! `completions/artifacts/M53/acceptance-design.md` → The arms, adopted at
//! `settle-record.md` D1–D5 and amended by the design review §1–§14).
//!
//! **The claim the pass proves is one claim:** *the four tier-1 rows of the rc.16 per-axis
//! review are closed, each over its class's axis rather than its reported repro — no byte
//! jigc did not write dies behind `milestone finalize` or behind a displacement that
//! failed, no un-concluded pick is concluded on the user's behalf, and no work unit is
//! minted at a fabricated identity.*
//!
//! Increments 1–5 shipped each fix with its own axis suite; this suite is the **composite
//! acceptance** that ties the pass into five done-picture arms — **each arm stating which
//! kind of set it iterates**, and each stating what it adds over the axis suites beside it,
//! because an arm that re-runs a shipped axis proves the axis twice and the pass once.
//!
//! The five arms:
//!
//!   (1) **The milestone area's plant shapes × the doors that stand over it** — six
//!       manufactured plant shapes (**manufactured and it says so**: *where a foreign byte
//!       can sit under `.jigc/milestones/<id>/`* is a property of a fixture no registry
//!       enumerates) crossed with [`DESTROYING_DOORS`] read through its own
//!       [`Disposition`] axis, plus `engine::milestone::materialize` on a **blocked**
//!       finalize. *Adds over `milestone_merged_complement.rs` /
//!       `merged_area_selective_clear.rs` / `milestone_boundary_displacement.rs`:* those
//!       own the complement predicate, the join's selective clear and the landed boundary's
//!       pairs; this arm drives **every member of the door registry through its own
//!       disposition** over **one** planted shape space — the displacing door once, each
//!       refusing door twice (without its consent and with it) — and states a verdict for
//!       every member that does **not** stand over this area rather than skipping it.
//!
//!   (2) **The move × unwind × area-kind space** — `{all · some · none move} × {unwind ok ·
//!       fault on the pin · fault on a later member} × {task area · sub-task areas ·
//!       milestone area}`, **manufactured and it says so** (the two failure points are
//!       decided — the pin is member 0 of the writer registry's row, the tree member is a
//!       later one — and the move axis is a property of the parking home's state). *Adds
//!       over `finalize_displacement.rs` / `milestone_boundary_displacement.rs`:* each owns
//!       one **door's** nine coordinates; this arm carries the **one** 27-coordinate table
//!       across all three area kinds, drives every reachable coordinate at the door that
//!       settles that kind of area, names the six a filesystem cannot produce, and asserts
//!       the one fact neither can state alone — the advisory's **surface asymmetry**, on
//!       `findings` at `task finalize` and on **stderr only** at `milestone finalize`, at
//!       the same coordinate.
//!
//!   (3) **The residual × the by-id and enumerating doors** — `{empty dir · foreign file ·
//!       foreign docs/<ty>:<slug>.md}` (manufactured: the three shapes differ in what the
//!       *destroying* doors make of the same directory) × `{plain id · sub-task of an open
//!       milestone · sub-task of a joined milestone}` × [`WORK_UNIT_ID_DOORS`] (**a
//!       code-side registry** whose rows carry their own runnable argv) ∪ the enumerating
//!       doors, **derived** from `engine::state::list_active_task_ids`' production callers
//!       and stated as a derivation. *Adds over `residual_area_roster.rs` /
//!       `work_unit_unknown_envelope.rs` / `residual_area_mint.rs`:* those own the roster,
//!       the envelope shape and the mint refusal; this arm drives **the whole cross in one
//!       corpus per placement** — every by-id row over every shape — and asserts the
//!       composite none of them can: that the roster, the orientation view and the by-id
//!       doors agree, that each row answers **its own family's** shipped pair, and that a
//!       `jigc task discard` over a joined milestone's residual commits **nothing**.
//!
//!   (4) **The four uncommitted-pick states × the acting doors** — Increment 4's new
//!       [`GitState`] variants crossed with [`BEHALF_DOORS`]' acting rows (**a total
//!       classification** of every clap leaf, fenced ⇔ against the tree). *Adds over
//!       `repo_posture.rs` / `posture_door_axis.rs`:* those own the probe's members and the
//!       state × door refusal sweep; this arm asserts the **damage** half — that the
//!       per-worktree `.git/MERGE_MSG` and the index are **byte-unchanged** after every
//!       acting door's refusal, closing the swallow, the message-only kill and the index
//!       contamination in one table — and then runs the emitted abandon argv verbatim and
//!       finds the picked changes still in the working tree.
//!
//!   (5) **The mint doors × the degenerate titles** — [`MINT_DOORS`] (**a code-side
//!       registry**, three prose rows and two `Exempt(reason)` rows) ∪ `milestone
//!       add-from-spec` (which reaches `engine::milestone::add_task`; **stated as a
//!       derivation**) crossed with `{"" · whitespace · punctuation · non-Latin script ·
//!       stopword-only}`. *Adds over `work_unit_id_axis.rs` / `milestone_envelope_arm.rs` /
//!       `start_compose.rs`:* those own the per-door refusal and the reject arm; this arm
//!       drives the **union** — the registry plus the door no registry row names — and
//!       carries §12's boundary cell, that outside a repository `milestone create ""` still
//!       answers the one not-in-repo answer rather than the new guard's.
//!
//! **What gets no arm, recorded as a decision** (the M46 Increment 9 / M48 Increment 11 /
//! M49 Increment 12 / M51 Increments 9–11 / M52 Increments 2·8·10·11 precedent — *an
//! increment that mints no verb, finding or route carries nothing for a done-picture walk
//! to reach, and manufacturing an arm would be a walk written to have an arm rather than to
//! prove a claim*):
//!
//!   * **Increment 6** is this suite, the two ledgers, the goldens and the fold-back.
//!
//! **Bounds, stated here rather than discovered later:**
//!
//!   * The arms are **headless by construction**: no genuine concurrent process races a
//!     teardown. The racer is the design's named one — an in-transaction `pre-commit` hook
//!     that changes the area's mode and exits 0.
//!   * The `chmod`-based fault cells declare the **CI platform bound**: run as `root` the
//!     permission bits do not bind, and each such cell then fails **loudly** rather than
//!     passing vacuously, because it names the area it expects to still be standing.
//!   * The git-state cells are **git 2.54.0's on-disk contract**
//!     (`support::git_state::EXPECTATIONS`; `implementation/decisions-pending.md` → the
//!     rc.16 wave, deferral (a)).
//!   * **D1's declared bounds**: a foreign *directory* inside `merged/` moves whole, never
//!     merged; a milestone id and a task id share one `.jigc/displaced/` namespace, which
//!     `design/storage.md` carries as a stated provenance ambiguity.
//!   * **D2's declared bounds**: the advisory is **stderr-only** at `milestone finalize`
//!     until 2.0 (the landed arm is pinned at `Object(&["committed"])`), and a fault on the
//!     pin leaves a whole area standing, listed as active — truthfully a task jigc could
//!     not tear down.
//!   * An arm that reds is **a finding about the pass landed, with its reason** — never a
//!     narrowed set. A coordinate that cannot be driven is stated on the coordinate with
//!     its datum.
//!
//! Isolation: every arm either rides the shared [`support::trial_corpus`] substrate, the
//! [`support::git_state`] builder, or builds its own throwaway repo — all of which `git
//! init`, set a per-repo identity, repoint `$HOME` and scrub `JIGC_PACK_DIR`.

#![cfg(unix)]

use crate::support;

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use serde_json::Value;

// ═════════════════════════════════════════════════════════════════════════════
// Shared helpers
// ═════════════════════════════════════════════════════════════════════════════

/// The marker every planted byte carries, so survival is scanned through the real `grep`
/// rather than claimed by the same code that wrote the bytes.
const KEEP: &str = "JIGC-FLOW54-KEEP";

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-flow54-{tag}-{}-{:?}",
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
        // Hand the write bits back first: a fault cell may have left a 0555 directory.
        let _ = restore_modes(&self.0);
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// Walk `root` and set every directory back to 0755 — the teardown half of the permission
/// games the fault coordinates need.
fn restore_modes(root: &Path) -> std::io::Result<()> {
    if root.is_dir() {
        let _ = set_mode(root, 0o755);
        for entry in fs::read_dir(root)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                restore_modes(&entry.path())?;
            }
        }
    }
    Ok(())
}

/// Set `path`'s mode. Returns `false` when the path is gone, which several teardown paths
/// tolerate.
fn set_mode(path: &Path, mode: u32) -> bool {
    let Ok(meta) = fs::metadata(path) else {
        return false;
    };
    let mut perms = meta.permissions();
    std::os::unix::fs::PermissionsExt::set_mode(&mut perms, mode);
    fs::set_permissions(path, perms).is_ok()
}

/// Both of an invocation's streams, joined — the surface a reader actually meets.
fn surface(out: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    )
}

/// Run `git <args>` in `cwd`, asserting success and returning stdout.
fn git(cwd: &Path, args: &[&str]) -> String {
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
    String::from_utf8_lossy(&out.stdout).into_owned()
}

/// Run the real binary in `repo` with `$HOME = home` and no inherited `JIGC_PACK_DIR`.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("spawn the jigc binary")
}

/// Run the real binary and assert it exited 0.
fn jigc_ok(repo: &Path, home: &Path, args: &[&str]) -> String {
    let out = jigc(repo, home, args);
    assert!(
        out.status.success(),
        "`jigc {}` must exit 0; surface:\n{}",
        args.join(" "),
        surface(&out),
    );
    String::from_utf8_lossy(&out.stdout).into_owned()
}

/// Every file under `repo` whose contents carry [`KEEP`], repo-relative and sorted.
///
/// `command grep` rather than the shell's, which honours `.gitignore` and would therefore
/// see nothing at all under `.jigc/` — the one directory every arm here plants into.
fn kept(repo: &Path) -> Vec<String> {
    let out = Command::new("grep")
        .args(["-rlE", KEEP, "."])
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

/// Parse a payload as JSON, surfacing the bytes on failure.
fn json(payload: &str) -> Value {
    serde_json::from_str(payload)
        .unwrap_or_else(|err| panic!("the payload is JSON ({err}); got:\n{payload}"))
}

// ═════════════════════════════════════════════════════════════════════════════
// The fan-out fixture — arms 1 and 2's substrate
// ═════════════════════════════════════════════════════════════════════════════

/// The milestone every fan-out fixture mints.
const MILESTONE: &str = "cache-rework";

/// Its single sub-task. One rather than two: every assertion below is per area, so a second
/// sub-task multiplies the fixture's cost and proves nothing the first does not — and the
/// *many-areas* half is exactly what `milestone_boundary_displacement.rs` owns.
const SUB_INTENT: &str = "Area low";

/// A real git repo with one commit — the ground every fixture below stands on.
fn init_bare_repo(root: &Path) {
    git(root, &["init", "-q"]);
    git(root, &["config", "user.email", "test@example.com"]);
    git(root, &["config", "user.name", "Test"]);
    fs::write(root.join("README.md"), "hello\n").expect("write README.md");
    git(root, &["add", "."]);
    git(root, &["commit", "-q", "-m", "initial"]);
}

/// …plus the project cascade layer — `jigc milestone`'s door-top precondition since M52
/// Increment 8, minted by hand where a full `jigc setup` is not what the fixture is about.
fn init_repo(root: &Path) {
    init_bare_repo(root);
    support::mint_project_layer(root);
}

/// A set-up repo carrying one **unprovisioned** milestone and nothing else — the fixture
/// arm 1's *refusing* doors need, because a provisioned worktree holding staged code makes
/// `milestone.dirty-worktree` / `uninstall.dirty-worktree` the refusal that fires and the
/// foreign bytes never get asked about.
fn milestone_only(tag: &str) -> (TempDir, TempDir) {
    let repo = TempDir::new(&format!("repo-{tag}"));
    let home = TempDir::new(&format!("home-{tag}"));
    init_bare_repo(repo.path());
    jigc_ok(repo.path(), home.path(), &["setup"]);
    jigc_ok(
        repo.path(),
        home.path(),
        &["milestone", "create", "Cache rework"],
    );
    (repo, home)
}

/// The milestone's own working area.
fn milestone_area(repo: &Path) -> PathBuf {
    repo.join(".jigc").join("milestones").join(MILESTONE)
}

/// A sub-task's working area.
fn task_area(repo: &Path, sub: &str) -> PathBuf {
    repo.join(".jigc").join("tasks").join(sub)
}

/// A work unit's parking home under `.jigc/displaced/`.
fn parking(repo: &Path, unit: &str) -> PathBuf {
    repo.join(".jigc").join("displaced").join(unit)
}

/// A plain, ref-free ADR body — the promotable contribution that keeps a boundary from
/// refusing `milestone.zero-contribution`.
fn adr_body(title: &str) -> String {
    format!(
        "---\nstatus: accepted\ndate: 2026-06-04\n---\n\n# {title}\n\n## Context\n\nForces.\n\n\
         ## Options\n\nAlternatives were weighed and rejected.\n\n## Decision\n\nDo the thing.\n\n\
         ## Consequences\n\nTradeoffs.\n"
    )
}

/// Stage a doc body plus its provenance bit into a sub-task's staged area.
fn stage_doc(repo: &Path, sub: &str, address: &str, body: &str) {
    let docs = task_area(repo, sub).join("docs");
    fs::create_dir_all(&docs).expect("mk the sub-task docs area");
    fs::write(docs.join(format!("{address}.md")), body).expect("write the staged body");
    let manifest = docs.join("provenance.json");
    let mut record: Value = match fs::read_to_string(&manifest) {
        Ok(s) => serde_json::from_str(&s).expect("the provenance manifest parses"),
        Err(_) => serde_json::json!({ "docs": {} }),
    };
    record["docs"][address] = Value::String("created".to_string());
    fs::write(
        &manifest,
        serde_json::to_string_pretty(&record).expect("serialize the manifest"),
    )
    .expect("write the provenance manifest");
}

/// A provisioned fan-out with one sub-task whose ADR and commit doc are staged and whose
/// worktree carries a staged code change — the shape a landed `milestone finalize` needs.
struct FanOut {
    repo: TempDir,
    home: TempDir,
    sub: String,
}

impl FanOut {
    fn build(tag: &str) -> Self {
        let repo = TempDir::new(&format!("repo-{tag}"));
        let home = TempDir::new(&format!("home-{tag}"));
        init_repo(repo.path());

        let fanout = FanOut {
            repo,
            home,
            sub: String::new(),
        };
        fanout.run_ok(&["milestone", "create", "Cache rework"]);
        let ack = fanout.run_ok(&["milestone", "add-task", MILESTONE, SUB_INTENT]);
        let sub = ack
            .split_once("added task:")
            .map(|(_, rest)| {
                rest.split_whitespace()
                    .next()
                    .expect("the ack names the sub-task id")
                    .to_owned()
            })
            .unwrap_or_else(|| panic!("`milestone add-task` names the task it minted:\n{ack}"));

        let repo = fanout.repo.path().to_path_buf();
        stage_doc(&repo, &sub, "adr:low-policy", &adr_body("Low policy"));
        stage_doc(
            &repo,
            &sub,
            &format!("commit:{sub}"),
            &format!(
                "---\ntype: feat\n---\n\n# {sub}\n\n## Summary\n\nrework the low cache path\n\n\
                 ## Body\n\nDriven by flow 54.\n\n## Trailers\n"
            ),
        );
        fanout.run_ok(&["milestone", "provision", MILESTONE]);

        // A staged code change in the fan-out worktree, so the boundary has a diff to land.
        let worktree = repo.join(".jigc").join("worktrees").join(&sub);
        fs::write(worktree.join("low.rs"), "pub fn low() {}\n").expect("write the worktree file");
        git(&worktree, &["add", "low.rs"]);

        FanOut {
            repo: fanout.repo,
            home: fanout.home,
            sub,
        }
    }

    fn repo(&self) -> &Path {
        self.repo.path()
    }

    fn home(&self) -> &Path {
        self.home.path()
    }

    fn run(&self, args: &[&str]) -> Output {
        jigc(self.repo(), self.home(), args)
    }

    fn run_ok(&self, args: &[&str]) -> String {
        jigc_ok(self.repo(), self.home(), args)
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 1 — the milestone area's shape space × DESTROYING_DOORS (through Disposition)
// ═════════════════════════════════════════════════════════════════════════════

use cli::milestone::{DESTROYING_DOORS, DestroyingDoor, Disposition};

/// **The six plant shapes — manufactured, and it says so.** *Where a third party's byte can
/// sit under `.jigc/milestones/<id>/`* is a property of a fixture; no registry enumerates
/// it. What the registry fixes is the membership *rule*
/// (`engine::state::foreign_area_paths`, whose milestone branch walks `merged/` and calls
/// `staged_doc_id` alone jigc's); these are the positions that rule has to answer, and each
/// one is a place the walk reached for the first time in M53 Increment 1.
const PLANTS: &[(&str, &str)] = &[
    (
        "NOTES.md",
        "JIGC-FLOW54-KEEP the operator's milestone scratch\n",
    ),
    (
        "merged/top.txt",
        "JIGC-FLOW54-KEEP a third party at the join staging top\n",
    ),
    (
        "merged/docs/adr.md",
        "JIGC-FLOW54-KEEP no `:` — no writer produced this name\n",
    ),
    (
        "merged/docs/provenance.json",
        "JIGC-FLOW54-KEEP materialize writes no manifest\n",
    ),
    (
        "merged/docs/deep.txt",
        "JIGC-FLOW54-KEEP beside a materialized body\n",
    ),
    (
        "merged/scratch/perf.txt",
        "JIGC-FLOW54-KEEP inside a directory that moves whole\n",
    ),
];

/// The complement **entries** [`PLANTS`] makes — the unit a door names, refuses over or
/// moves, sorted as `engine::state::foreign_area_paths` returns them. `merged/scratch` is
/// one entry, never `merged/scratch/perf.txt`: a foreign directory moves whole (D1's
/// declared bound).
const ENTRIES: &[&str] = &[
    "NOTES.md",
    "merged/docs/adr.md",
    "merged/docs/deep.txt",
    "merged/docs/provenance.json",
    "merged/scratch",
    "merged/top.txt",
];

/// Whether a [`DESTROYING_DOORS`] member stands over the **milestone** working area — and,
/// when it does not, the verdict that says so.
///
/// It is a **stated verdict per row, never a skip**: a member left out of the cross with no
/// answer is a door nobody adjudicated, which is the shape this pass exists to close. A
/// seventh door added to the registry hits the `other =>` panic and has to answer.
enum Standing {
    /// The door removes the milestone working area, and answers for what is in it.
    Stands,
    /// The door's destroyed path is somewhere else, with the reason it is.
    Outside(&'static str),
}

fn standing(door: &DestroyingDoor) -> Standing {
    match door.verb {
        "jigc milestone discard" | "jigc uninstall" | "jigc milestone finalize" => Standing::Stands,
        "jigc milestone provision" => Standing::Outside(
            "its destroyed path is a leftover at a sub-task's `.jigc/worktrees/<sub>` path, \
             removed before `git worktree add` — the milestone area is not its subject and \
             it never removes one",
        ),
        "jigc task discard" => Standing::Outside(
            "its subject is one task's `.jigc/tasks/<id>/` area; a milestone id names no \
             task, and the door refuses before it resolves",
        ),
        "jigc task finalize" => Standing::Outside(
            "the per-task commit boundary removes the task's own area and refuses a \
             milestone sub-task outright (`finalize.milestone-sub-task`), so it reaches \
             `.jigc/milestones/<id>/` at no arm",
        ),
        other => panic!(
            "`{other}` is a destroying door with no verdict about the milestone working \
             area — a new member owes this arm the answer, exactly as it already owes \
             `Disposition` a value"
        ),
    }
}

/// Plant [`PLANTS`] into the milestone area, and assert the before-control finds them — a
/// scan that finds nothing before proves nothing after.
fn plant_milestone(repo: &Path) {
    for (rel, body) in PLANTS {
        let at = milestone_area(repo).join(rel);
        fs::create_dir_all(at.parent().expect("a plant has a parent")).expect("mk plant parent");
        fs::write(&at, body).expect("write the plant");
    }
    assert_eq!(
        kept(repo).len(),
        PLANTS.len(),
        "the before-control must FIND every plant",
    );
}

/// **Arm 1** — every member of the destroying-door registry, driven through its own
/// [`Disposition`] over one planted shape space in the milestone working area.
#[test]
fn arm1_the_milestone_areas_shape_space_at_every_door_that_stands_over_it() {
    let mut stood = 0_usize;
    let mut outside = 0_usize;

    for door in DESTROYING_DOORS {
        match standing(door) {
            Standing::Outside(reason) => {
                outside += 1;
                assert!(
                    reason.len() > 40,
                    "[{}] a row outside this arm's subject carries a REASON, not an absence",
                    door.verb,
                );
                continue;
            }
            Standing::Stands => stood += 1,
        }

        match door.disposition {
            Disposition::Displace => {
                // `jigc milestone finalize` — no consent to offer, so the answer is the move.
                let fanout = FanOut::build("arm1-displace");
                let repo = fanout.repo().to_path_buf();
                plant_milestone(&repo);
                let before = kept(&repo);

                let out = fanout.run(&["milestone", "finalize", MILESTONE, "--format", "json"]);
                let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
                let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
                assert!(
                    out.status.success(),
                    "[{}] the boundary must land at exit 0;\nstdout:\n{stdout}\nstderr:\n{stderr}",
                    door.verb,
                );

                // Every plant survives at `.jigc/displaced/<milestone-id>/<relative>`,
                // byte-intact and with its relative path preserved.
                for (rel, body) in PLANTS {
                    let at = parking(&repo, MILESTONE).join(rel);
                    let got = fs::read_to_string(&at).unwrap_or_else(|err| {
                        panic!(
                            "[{}] `{rel}` must survive at {at:?}: {err}\nstderr:\n{stderr}",
                            door.verb,
                        )
                    });
                    assert_eq!(&got, body, "[{}] `{rel}` survives BYTE-INTACT", door.verb);
                }
                assert_eq!(
                    kept(&repo).len(),
                    before.len(),
                    "[{}] the whole shape space survives the landed boundary",
                    door.verb,
                );

                // …named on stderr and on `committed.displaced`, one pair per ENTRY.
                let pairs: BTreeSet<(String, String)> = json(&stdout)["committed"]["displaced"]
                    .as_array()
                    .unwrap_or_else(|| {
                        panic!(
                            "[{}] the landed envelope carries `displaced`:\n{stdout}",
                            door.verb
                        )
                    })
                    .iter()
                    .map(|p| {
                        (
                            p["from"].as_str().expect("`from` is a string").to_owned(),
                            p["to"].as_str().expect("`to` is a string").to_owned(),
                        )
                    })
                    .collect();
                let expected: BTreeSet<(String, String)> = ENTRIES
                    .iter()
                    .map(|entry| {
                        (
                            format!(".jigc/milestones/{MILESTONE}/{entry}"),
                            format!(".jigc/displaced/{MILESTONE}/{entry}"),
                        )
                    })
                    .collect();
                assert_eq!(
                    pairs, expected,
                    "[{}] every complement ENTRY is named on the envelope — `merged/scratch` \
                     whole, never its contents;\nstdout:\n{stdout}",
                    door.verb,
                );
                for (from, to) in &expected {
                    assert!(
                        stderr.contains(from.as_str()) && stderr.contains(to.as_str()),
                        "[{}] stderr must name the move `{from}` → `{to}`;\nstderr:\n{stderr}",
                        door.verb,
                    );
                }
            }
            Disposition::Refuse { consent } => {
                // Driven twice: without the consent it refuses and takes nothing; with it,
                // the door does its act and still names what it took.
                let argv: Vec<&str> = match door.verb {
                    "jigc milestone discard" => vec!["milestone", "discard", MILESTONE],
                    "jigc uninstall" => vec!["uninstall"],
                    other => panic!("[{other}] a refusing member of this arm needs its argv"),
                };

                let (repo_dir, home_dir) = milestone_only("arm1-refuse");
                let repo = repo_dir.path().to_path_buf();
                let home = home_dir.path().to_path_buf();
                plant_milestone(&repo);
                let before = kept(&repo);

                let refused = jigc(&repo, &home, &argv);
                let text = surface(&refused);
                assert!(
                    !refused.status.success(),
                    "[{}] must refuse over a third party's bytes;\n{text}",
                    door.verb,
                );
                let code = door
                    .codes
                    .iter()
                    .find(|code| text.contains(**code) && code.ends_with("foreign-bytes"))
                    .unwrap_or_else(|| {
                        panic!(
                            "[{}] the refusal carries this door's own foreign-bytes identity, \
                             out of its registry row's `codes`;\n{text}",
                            door.verb,
                        )
                    });
                for entry in ENTRIES {
                    assert!(
                        text.contains(&format!(".jigc/milestones/{MILESTONE}/{entry}")),
                        "[{}] `{code}` names EVERY path it would destroy, `{entry}` included;\n{text}",
                        door.verb,
                    );
                }
                assert_eq!(
                    kept(&repo),
                    before,
                    "[{}] a refusal takes NOTHING;\n{text}",
                    door.verb,
                );
                assert!(
                    text.contains(consent),
                    "[{}] the refusal's route names the one consent this door takes;\n{text}",
                    door.verb,
                );

                // …and with the consent.
                let mut consented = argv.clone();
                consented.push(consent);
                let forced = jigc(&repo, &home, &consented);
                let text = surface(&forced);
                assert!(
                    forced.status.success(),
                    "[{}] the consented run must do the door's act;\n{text}",
                    door.verb,
                );
                for entry in ENTRIES {
                    assert!(
                        text.contains(&format!(".jigc/milestones/{MILESTONE}/{entry}")),
                        "[{}] the consented run still NAMES each path it took, `{entry}` \
                         included — the loss is made visible;\n{text}",
                        door.verb,
                    );
                }
            }
            Disposition::Narrate => panic!(
                "[{}] no member holds `Disposition::Narrate` today (`settle-record.md` §18 \
                 moved the last one); a row that takes it owes this arm its cell",
                door.verb,
            ),
        }
    }

    assert_eq!(
        stood + outside,
        DESTROYING_DOORS.len(),
        "every registry row is either driven here or carries a stated verdict",
    );
    assert_eq!(
        stood, 3,
        "three doors stand over the milestone working area"
    );
}

/// **Arm 1, the `materialize` half** — a boundary that **blocks** takes nothing, and is not
/// blocked *by* what it left alone.
///
/// The join's staging tree is rebuilt on every run, and until M53 Increment 1 that rebuild
/// was a `remove_dir_all(merged/)`: a third party's byte under `merged/` died on the way to
/// a gate that then refused the run anyway. The two halves are asserted apart here — nothing
/// died, **and** the block is about the sub-task's own unfilled prose.
#[test]
fn arm1_a_blocked_boundary_leaves_every_plant_and_is_not_blocked_by_one() {
    let fanout = FanOut::build("arm1-blocked");
    let repo = fanout.repo().to_path_buf();

    // Make the sub-task's contributed ADR non-conformant — a required section removed, so
    // the boundary's merged-state conformance validation blocks before any teardown runs.
    let staged = task_area(&repo, &fanout.sub)
        .join("docs")
        .join("adr:low-policy.md");
    let body = fs::read_to_string(&staged).expect("read the staged ADR");
    fs::write(
        &staged,
        body.replace("## Decision\n\nDo the thing.\n\n", ""),
    )
    .expect("remove a required section");

    plant_milestone(&repo);
    let before = kept(&repo);

    let out = fanout.run(&["milestone", "finalize", MILESTONE, "--format", "json"]);
    let text = surface(&out);
    assert_eq!(
        out.status.code(),
        Some(3),
        "the blocked boundary exits 3 — the content-gate seam;\n{text}",
    );
    assert_eq!(
        kept(&repo),
        before,
        "a boundary that blocks takes nothing — `materialize`'s rebuild removes only what \
         `staged_doc_id` recognises;\n{text}",
    );
    assert!(
        !text.contains("foreign-bytes"),
        "…and it is not blocked BY a third party's bytes: the block is the gate's;\n{text}",
    );
    assert!(
        milestone_area(&repo).join("merged").exists(),
        "the staging tree is still there — the run blocked, it did not tear down",
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 2 — {all · some · none move} × {unwind} × {area kind}
// ═════════════════════════════════════════════════════════════════════════════

use support::trial_corpus::{State, TrialCorpus};

/// How much of an area's complement reached `.jigc/displaced/<unit-id>/`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Moves {
    All,
    /// Some moved and some did not — the value between *all* and *none*, and the one the
    /// narration's count was blind to.
    Some_,
    None_,
}

/// What `engine::state::unwind_area` did with jigc's **own** members.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Unwind {
    Ok,
    /// The removal failed on the writer registry's member 0, the base pin — so the area
    /// stands **whole**, pin included.
    FaultOnPin,
    /// The removal failed on a later member, after the pin was already gone.
    FaultOnLater,
}

/// Which working area a coordinate is about — the third factor, and the one neither axis
/// suite could carry alone.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum AreaKind {
    /// `.jigc/tasks/<id>/`, settled by `jigc task finalize`.
    Task,
    /// `.jigc/tasks/<sub-id>/`, settled by the milestone boundary.
    SubTask,
    /// `.jigc/milestones/<id>/`, settled by the same boundary in the same run.
    Milestone,
}

impl AreaKind {
    const ALL: &'static [AreaKind] = &[AreaKind::Task, AreaKind::SubTask, AreaKind::Milestone];

    fn label(self) -> &'static str {
        match self {
            AreaKind::Task => "task area",
            AreaKind::SubTask => "sub-task areas",
            AreaKind::Milestone => "milestone area",
        }
    }
}

/// One coordinate of the **manufactured** 27-cell space, with its reachability.
///
/// **Manufactured, and it says so.** The two failure points are decided — the pin is member
/// 0 of the writer registry's row for the area kind, the tree member is a later one — and
/// the move axis is a property of the *parking home's* state, which no registry enumerates.
struct Coordinate {
    moves: Moves,
    unwind: Unwind,
    kind: AreaKind,
    /// `None` when the coordinate is driven; `Some(reason)` when no filesystem produces it.
    unreachable: Option<&'static str>,
}

/// **The whole space** — `3 × 3 × 3`, built here rather than written out, with the six
/// coordinates a filesystem cannot make carrying their reason.
fn coordinates() -> Vec<Coordinate> {
    let mut out = Vec::new();
    for &kind in AreaKind::ALL {
        for &moves in &[Moves::All, Moves::Some_, Moves::None_] {
            for &unwind in &[Unwind::Ok, Unwind::FaultOnPin, Unwind::FaultOnLater] {
                let unreachable = match (moves, unwind) {
                    (Moves::All, Unwind::FaultOnPin) => Some(
                        "the pin's `remove_file` and every entry's `fs::rename` out of the \
                         area draw on the SAME bit — write permission on the area — so an \
                         area whose pin cannot be removed is an area no entry can leave",
                    ),
                    (Moves::Some_, Unwind::FaultOnPin) => Some(
                        "the same bit, for the same reason: with the area writable the pin \
                         goes, and without it no entry moves, so `some` and `fault on the \
                         pin` cannot hold at once",
                    ),
                    _ => None,
                };
                out.push(Coordinate {
                    moves,
                    unwind,
                    kind,
                    unreachable,
                });
            }
        }
    }
    out
}

/// The advisory this pass mints — the `finalize` family's `foreign-bytes` member.
const KEPT_CODE: &str = "finalize.foreign-bytes";

/// A plant at the area root: the depth every door reached before M53.
const ROOT_KEEP: (&str, &str) = ("root-kept.txt", "JIGC-FLOW54-KEEP root\n");

/// A plant one directory down, whose complement **entry** is its parent.
const NESTED_KEEP: (&str, &str) = ("analysis/perf.txt", "JIGC-FLOW54-KEEP nested\n");

/// A plant inside the task row's one tree member, whose own rule decides entry by entry —
/// and the entry whose move the `some` coordinate stops.
const DOCS_KEEP: (&str, &str) = ("docs/non-md.txt", "JIGC-FLOW54-KEEP under docs\n");

/// The milestone row's tree member, two levels deep.
const MERGED_KEEP: (&str, &str) = ("merged/top.txt", "JIGC-FLOW54-KEEP under merged\n");

/// The plants a coordinate puts in an area of `kind`.
fn plants_for(moves: Moves, unwind: Unwind, kind: AreaKind) -> Vec<(&'static str, &'static str)> {
    let tree_member = match kind {
        AreaKind::Task | AreaKind::SubTask => DOCS_KEEP,
        AreaKind::Milestone => MERGED_KEEP,
    };
    match (moves, unwind) {
        // The `some` coordinate needs one entry that lands and one that cannot.
        (Moves::Some_, _) => vec![ROOT_KEEP, tree_member],
        // A fault on a later member makes the tree member unreadable, so a plant inside it
        // would confuse the move coordinate with the fault.
        (_, Unwind::FaultOnLater) if kind == AreaKind::Milestone => vec![ROOT_KEEP],
        _ => vec![ROOT_KEEP, NESTED_KEEP],
    }
}

/// The tree member whose removal is the later-member fault, per area kind.
fn later_member(area: &Path, kind: AreaKind) -> PathBuf {
    match kind {
        AreaKind::Task | AreaKind::SubTask => area.join("docs"),
        AreaKind::Milestone => area.join("merged").join("docs"),
    }
}

/// Occupy `<repo>/.jigc/displaced/<unit>` with a regular file — no entry's parking parent
/// can be created, so **nothing** moves.
fn block_parking(repo: &Path, unit: &str) {
    let home = repo.join(".jigc").join("displaced");
    fs::create_dir_all(&home).expect("mk the parking root");
    fs::write(home.join(unit), "not a directory\n").expect("occupy the parking home");
}

/// Occupy one entry's parking **parent**, so exactly that entry's move cannot land.
fn occupy_partial(repo: &Path, unit: &str, parent_rel: &str) {
    let at = parking(repo, unit).join(parent_rel);
    fs::create_dir_all(at.parent().expect("the occupant has a parent")).expect("mk parking home");
    fs::write(&at, "not a directory\n").expect("occupy the parking parent");
}

/// Install a `pre-commit` hook that chmods each target to 0555 **inside** the transaction —
/// the only moment the fault can be made, since the finalize writes its message temp file
/// into the area before the commit and a mode set beforehand would fail the run rather than
/// the teardown.
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

/// Plant `plants` into `area`.
fn plant_into(area: &Path, plants: &[(&str, &str)]) {
    for (rel, body) in plants {
        let at = area.join(rel);
        fs::create_dir_all(at.parent().expect("a plant has a parent")).expect("mk plant parent");
        fs::write(&at, body).expect("write the plant");
    }
}

/// Drive the `{moves, unwind}` coordinate at the **task** door, and assert it.
fn drive_task_coordinate(moves: Moves, unwind: Unwind) {
    let label = format!("{moves:?} × {unwind:?} × task area");
    let corpus = TrialCorpus::build(State::Fresh);
    let task = corpus.start_workflow("quick-fix", "Cap the retry budget");
    let repo = corpus.repo();
    let area = task_area(&repo, &task);

    let plants = plants_for(moves, unwind, AreaKind::Task);
    plant_into(&area, &plants);

    match moves {
        Moves::None_ => block_parking(&repo, &task),
        Moves::Some_ if unwind == Unwind::Ok => occupy_partial(&repo, &task, "docs"),
        // Under a later-member fault the `docs/` plant cannot move anyway: a rename needs
        // the SOURCE directory writable, and that is the directory the fault is made in.
        _ => {}
    }

    let faulted: Vec<PathBuf> = match unwind {
        Unwind::Ok => Vec::new(),
        Unwind::FaultOnPin => vec![area.clone()],
        Unwind::FaultOnLater => vec![later_member(&area, AreaKind::Task)],
    };
    if !faulted.is_empty() {
        install_faulting_hook(&repo, &faulted);
    }

    // A real diff for the commit to carry.
    fs::write(repo.join("README.md"), "hello\nretry cap = 3\n").expect("edit the tracked file");
    corpus.git(&["add", "README.md"]);
    for (field, value) in [("type", "fix"), ("scope", "cli")] {
        corpus.jigc_ok(&[
            "doc",
            "set-field",
            &format!("commit:{task}#{field}"),
            "--value",
            value,
            "--task",
            &task,
        ]);
    }
    corpus.set_slot(&format!("commit:{task}#summary"), &task, "cap the retries");
    corpus.set_slot(&format!("commit:{task}#body"), &task, "Driven by flow 54.");

    let before = kept(&repo);
    assert_eq!(
        before.len(),
        plants.len(),
        "[{label}] the before-control must FIND the plants; got {before:?}",
    );

    let out = corpus.jigc(&["task", "finalize", &task, "--format", "json"]);
    for target in &faulted {
        set_mode(target, 0o755);
    }
    set_mode(&area, 0o755);
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();

    assert!(
        out.status.success(),
        "[{label}] the commit is truth and the teardown is best-effort — the door lands at \
         exit 0;\nstdout:\n{stdout}\nstderr:\n{stderr}",
    );
    assert_eq!(
        kept(&repo).len(),
        before.len(),
        "[{label}] every planted byte survives — moved aside or left standing, never taken;\n\
         stderr:\n{stderr}",
    );

    let standing = area.exists();
    let expected_standing = !(moves == Moves::All && unwind == Unwind::Ok);
    assert_eq!(
        standing, expected_standing,
        "[{label}] an area jigc could not empty of a third party's bytes is LEFT, and one it \
         emptied is gone;\nstderr:\n{stderr}",
    );

    // **The surface asymmetry, this door's half**: the advisory rides `findings` on the
    // landed envelope here, and stderr-only at the boundary (asserted in its own cell).
    let envelope = json(&stdout);
    let advisories = envelope["findings"]
        .as_array()
        .unwrap_or_else(|| panic!("[{label}] the landed envelope carries `findings`:\n{stdout}"))
        .iter()
        .filter(|f| f["code"].as_str() == Some(KEPT_CODE))
        .count();
    assert_eq!(
        advisories,
        usize::from(expected_standing),
        "[{label}] exactly one `{KEPT_CODE}` per area left standing, on the FINDINGS key;\n\
         stdout:\n{stdout}",
    );
    if expected_standing {
        let finding = envelope["findings"]
            .as_array()
            .expect("findings array")
            .iter()
            .find(|f| f["code"].as_str() == Some(KEPT_CODE))
            .expect("the advisory is there");
        assert_eq!(
            finding["key"]["target"].as_str(),
            Some(format!("task:{task}").as_str()),
            "[{label}] …keyed at its own work unit;\nstdout:\n{stdout}",
        );
        assert_eq!(
            area.join("base.json").exists(),
            unwind == Unwind::FaultOnPin,
            "[{label}] a fault ON the pin leaves the area whole; a clean unwind and a fault \
             on a LATER member both take the pin first",
        );
    }
}

/// Drive the `{moves, unwind}` coordinate at the **milestone boundary**, which settles the
/// sub-task areas and the milestone's own area in one run — so one run drives two of the
/// space's three area kinds, and the assertions are made per area.
fn drive_boundary_coordinate(moves: Moves, unwind: Unwind) {
    let label = format!("{moves:?} × {unwind:?} × boundary");
    let fanout = FanOut::build("arm2");
    let repo = fanout.repo().to_path_buf();
    let sub = fanout.sub.clone();

    let sub_plants = plants_for(moves, unwind, AreaKind::SubTask);
    let ms_plants = plants_for(moves, unwind, AreaKind::Milestone);
    plant_into(&task_area(&repo, &sub), &sub_plants);
    plant_into(&milestone_area(&repo), &ms_plants);

    match moves {
        Moves::None_ => {
            block_parking(&repo, &sub);
            block_parking(&repo, MILESTONE);
        }
        Moves::Some_ if unwind == Unwind::Ok => {
            occupy_partial(&repo, &sub, "docs");
            occupy_partial(&repo, MILESTONE, "merged");
        }
        _ => {}
    }

    let areas = [task_area(&repo, &sub), milestone_area(&repo)];
    let faulted: Vec<PathBuf> = match unwind {
        Unwind::Ok => Vec::new(),
        Unwind::FaultOnPin => areas.to_vec(),
        Unwind::FaultOnLater => vec![
            later_member(&areas[0], AreaKind::SubTask),
            later_member(&areas[1], AreaKind::Milestone),
        ],
    };
    if !faulted.is_empty() {
        install_faulting_hook(&repo, &faulted);
    }

    let planted = sub_plants.len() + ms_plants.len();
    let before = kept(&repo);
    assert_eq!(
        before.len(),
        planted,
        "[{label}] the before-control must FIND the plants; got {before:?}",
    );

    let out = fanout.run(&["milestone", "finalize", MILESTONE, "--format", "json"]);
    for target in &faulted {
        set_mode(target, 0o755);
    }
    for area in &areas {
        set_mode(area, 0o755);
    }
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();

    assert!(
        out.status.success(),
        "[{label}] the boundary lands at exit 0;\nstdout:\n{stdout}\nstderr:\n{stderr}",
    );
    assert_eq!(
        kept(&repo).len(),
        before.len(),
        "[{label}] every planted byte survives the landed boundary;\nstderr:\n{stderr}",
    );

    let expected_standing = !(moves == Moves::All && unwind == Unwind::Ok);
    let standing: Vec<&PathBuf> = areas.iter().filter(|a| a.exists()).collect();
    assert_eq!(
        standing.len(),
        if expected_standing { areas.len() } else { 0 },
        "[{label}] both settled areas answer the same way;\nstderr:\n{stderr}",
    );

    // **The surface asymmetry, the boundary's half** — stderr only, because the landed arm
    // is pinned at `Object(&["committed"])` (D2's declared bound, held to 2.0).
    let mut keys: Vec<String> = json(&stdout)
        .as_object()
        .expect("the landed envelope is an object")
        .keys()
        .cloned()
        .collect();
    keys.sort();
    assert_eq!(
        keys,
        vec!["committed".to_string()],
        "[{label}] the landed boundary envelope stays `Object([\"committed\"])` — the \
         advisory does not move it;\nstdout:\n{stdout}",
    );
    assert_eq!(
        stderr.matches(KEPT_CODE).count(),
        standing.len(),
        "[{label}] exactly one `{KEPT_CODE}` per area left standing, on stderr;\n\
         stderr:\n{stderr}",
    );
    for area in &standing {
        let unit = area
            .file_name()
            .expect("an area has a name")
            .to_string_lossy();
        let target = if area.starts_with(repo.join(".jigc").join("milestones")) {
            format!("milestone:{unit}")
        } else {
            format!("task:{unit}")
        };
        assert!(
            stderr.contains(&target),
            "[{label}] …each keyed at its OWN work unit (`{target}`), so a boundary that \
             settles two areas hands a reader two keys;\nstderr:\n{stderr}",
        );
        assert_eq!(
            area.join("base.json").exists(),
            unwind == Unwind::FaultOnPin,
            "[{label}] the pin's fate is what separates the two fault coordinates",
        );
    }
}

/// **Arm 2, the space itself** — the 27 coordinates are enumerated, and the six no
/// filesystem produces carry their reason rather than being silently absent.
#[test]
fn arm2_the_move_unwind_area_space_is_twenty_seven_and_names_its_gaps() {
    let space = coordinates();
    assert_eq!(
        space.len(),
        27,
        "`{{all · some · none}} × {{ok · fault on the pin · fault on a later member}} × \
         {{task · sub-task · milestone}}` is twenty-seven coordinates",
    );
    let unreachable: Vec<&Coordinate> = space.iter().filter(|c| c.unreachable.is_some()).collect();
    assert_eq!(
        unreachable.len(),
        6,
        "two coordinates per area kind are unreachable — one bit governs both the pin's \
         removal and every entry's rename out of the area",
    );
    for coordinate in unreachable {
        let reason = coordinate.unreachable.expect("filtered to Some");
        assert!(
            reason.len() > 60,
            "[{:?} × {:?} × {}] an unreachable coordinate carries a REASON, not a gap",
            coordinate.moves,
            coordinate.unwind,
            coordinate.kind.label(),
        );
    }
}

/// **Arm 2, driven** — every reachable coordinate, at the door that settles its area kind.
///
/// The `{moves, unwind}` pairs are **derived from the space**, never listed a second time:
/// a coordinate added or re-classified moves the driving with it. Each pair is driven twice
/// — once at `jigc task finalize` (the `task area` kind) and once at
/// `jigc milestone finalize`, which settles the `sub-task areas` and the `milestone area`
/// kinds in the **same run**, because they are one boundary — and the set of coordinates
/// those runs cover is asserted **equal** to the reachable set, so a coordinate that no run
/// reaches reddens rather than being counted.
#[test]
fn arm2_every_reachable_coordinate_keeps_every_un_moved_byte() {
    let space = coordinates();
    let reachable: BTreeSet<(Moves, Unwind, AreaKind)> = space
        .iter()
        .filter(|c| c.unreachable.is_none())
        .map(|c| (c.moves, c.unwind, c.kind))
        .collect();
    assert_eq!(
        reachable.len(),
        21,
        "twenty-one of the twenty-seven coordinates are reachable",
    );

    let mut pairs: Vec<(Moves, Unwind)> = reachable.iter().map(|(m, u, _)| (*m, *u)).collect();
    pairs.sort_unstable();
    pairs.dedup();
    assert_eq!(pairs.len(), 7, "seven `{{moves, unwind}}` pairs carry them");

    let mut covered: BTreeSet<(Moves, Unwind, AreaKind)> = BTreeSet::new();
    for (moves, unwind) in pairs {
        drive_task_coordinate(moves, unwind);
        covered.insert((moves, unwind, AreaKind::Task));
        // One boundary run settles both of the other kinds, so it covers both coordinates.
        drive_boundary_coordinate(moves, unwind);
        covered.insert((moves, unwind, AreaKind::SubTask));
        covered.insert((moves, unwind, AreaKind::Milestone));
    }
    assert_eq!(
        covered, reachable,
        "every reachable coordinate is covered by a run, and no run claims one the space \
         calls unreachable",
    );
}

/// **Arm 2's named cell D1×D2** — a foreign byte **under `merged/`** whose move failed
/// survives a landed `milestone finalize`.
///
/// This is the cell that makes Increment 1's `unwind_area` milestone arm *reachable*: until
/// Increment 2 the teardown after a failed displacement was a `remove_dir_all`, so the walk
/// Increment 1 taught it had nothing left to walk.
#[test]
fn arm2_a_failed_merged_move_survives_a_landed_boundary() {
    let fanout = FanOut::build("arm2-d1xd2");
    let repo = fanout.repo().to_path_buf();
    plant_into(&milestone_area(&repo), &[MERGED_KEEP]);
    block_parking(&repo, MILESTONE);
    let before = kept(&repo);
    assert_eq!(before.len(), 1, "one plant, under `merged/`");

    let out = fanout.run(&["milestone", "finalize", MILESTONE]);
    let text = surface(&out);
    assert!(
        out.status.success(),
        "the boundary lands at exit 0;\n{text}"
    );
    assert_eq!(
        kept(&repo),
        before,
        "the byte under `merged/` whose move failed is STILL THERE;\n{text}",
    );
    assert!(
        milestone_area(&repo).join(MERGED_KEEP.0).exists(),
        "…at its original path, not moved and not merged;\n{text}",
    );
    assert!(text.contains(KEPT_CODE), "…and the door says so;\n{text}",);
}

/// **Arm 2's named cell — the hook-writes-during-the-commit shape.** Zero move failures (the
/// displacement ran before the hook wrote), and the unwind still answers `Foreign`: the
/// advisory's path set is a `foreign_area_paths` re-read taken **after** the unwind, not the
/// set the displacement enumerated.
#[test]
fn arm2_a_hook_write_during_the_commit_survives_the_teardown_that_could_not_enumerate_it() {
    let corpus = TrialCorpus::build(State::Fresh);
    let task = corpus.start_workflow("quick-fix", "Cap the retry budget");
    let repo = corpus.repo();
    let area = task_area(&repo, &task);

    // The marker is written **split** into two adjacent shell literals, which `sh`
    // concatenates: the file the hook writes carries `KEEP`, while the hook's own source
    // does not match the scan — otherwise the before-control would find the hook itself and
    // the cell would be asserting about the wrong file.
    let (head, tail) = KEEP.split_at(KEEP.len() - "-KEEP".len());
    assert_eq!(format!("{head}{tail}"), KEEP, "the split reassembles");
    let hook = repo.join(".git").join("hooks").join("pre-commit");
    fs::create_dir_all(hook.parent().expect("hooks dir")).expect("mk hooks dir");
    fs::write(
        &hook,
        format!(
            "#!/bin/sh\nprintf '{head}''{tail} written inside the transaction\\n' > \
             {area}/LATE.txt\nchmod 0333 {area}\nexit 0\n",
            area = area.display(),
        ),
    )
    .expect("install the late-writing hook");
    set_mode(&hook, 0o755);

    fs::write(repo.join("README.md"), "hello\nretry cap = 3\n").expect("edit the tracked file");
    corpus.git(&["add", "README.md"]);
    for (field, value) in [("type", "fix"), ("scope", "cli")] {
        corpus.jigc_ok(&[
            "doc",
            "set-field",
            &format!("commit:{task}#{field}"),
            "--value",
            value,
            "--task",
            &task,
        ]);
    }
    corpus.set_slot(&format!("commit:{task}#summary"), &task, "cap the retries");
    corpus.set_slot(&format!("commit:{task}#body"), &task, "Driven by flow 54.");

    assert!(kept(&repo).is_empty(), "nothing is planted before the run");

    let out = corpus.jigc(&["task", "finalize", &task, "--format", "json"]);
    set_mode(&area, 0o755);
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    assert!(
        out.status.success(),
        "the finalize lands;\nstdout:\n{stdout}\nstderr:\n{stderr}",
    );
    assert!(
        !stderr.contains("could not be moved"),
        "**zero move failures**: the displacement never enumerated an entry to fail on, \
         which is exactly why the advisory cannot be composed from that set;\n\
         stderr:\n{stderr}",
    );
    assert_eq!(
        kept(&repo),
        vec![format!(".jigc/tasks/{task}/LATE.txt")],
        "…and the byte the hook wrote INSIDE the transaction is still where it was \
         written — `unwind_area` removes jigc's members BY NAME;\nstdout:\n{stdout}",
    );
    let advisories = json(&stdout)["findings"]
        .as_array()
        .expect("the landed envelope carries `findings`")
        .iter()
        .filter(|f| f["code"].as_str() == Some(KEPT_CODE))
        .count();
    assert_eq!(
        advisories, 1,
        "…so the area survives its own teardown and one `{KEPT_CODE}` names it, composed \
         from `AreaUnwind::Foreign` rather than from a move set that is empty here;\n\
         stdout:\n{stdout}",
    );
}

/// **Arm 2's named cell — an area jigc could not tear down holds no file JIGC wrote**
/// (M53 completion audit, fix 1).
///
/// Every other cell of this arm asserts what happens to a *third party's* bytes. None of them
/// asks the complementary question, and that is the gap this one closes: the complement is
/// what the surface names and what every later destroying door refuses over, so a file jigc
/// itself wrote that is not on its own registry row is a lie on one surface and a dead end at
/// three doors.
///
/// Driven at `45427083`, before the fix: `jigc task finalize` landed at exit 0 printing
/// `finalize.foreign-bytes` over `.jigc/tasks/<id>/finalize-message.tmp` — jigc's own commit
/// message transient, named as *a path jigc did not write* — and `jigc task discard <id>`
/// then refused at **exit 1** over it, demanding `--force` to remove jigc's own file. The
/// racer is Increment 2's own: a `pre-commit` hook that `chmod 0555`s the area inside the
/// transaction, which is the one moment at which the fault can be made — it defeats the
/// best-effort `remove_file` of the transient *and* the teardown, in one bit.
#[test]
fn arm2_an_area_left_standing_holds_no_file_jigc_wrote() {
    let corpus = TrialCorpus::build(State::Fresh);
    let task = corpus.start_workflow("quick-fix", "Cap the retry budget");
    let repo = corpus.repo();
    let area = task_area(&repo, &task);

    let hook = repo.join(".git").join("hooks").join("pre-commit");
    fs::create_dir_all(hook.parent().expect("hooks dir")).expect("mk hooks dir");
    fs::write(
        &hook,
        format!("#!/bin/sh\nchmod 0555 {}\nexit 0\n", area.display()),
    )
    .expect("install the area-freezing hook");
    set_mode(&hook, 0o755);

    fs::write(repo.join("README.md"), "hello\nretry cap = 3\n").expect("edit the tracked file");
    corpus.git(&["add", "README.md"]);
    for (field, value) in [("type", "fix"), ("scope", "cli")] {
        corpus.jigc_ok(&[
            "doc",
            "set-field",
            &format!("commit:{task}#{field}"),
            "--value",
            value,
            "--task",
            &task,
        ]);
    }
    corpus.set_slot(&format!("commit:{task}#summary"), &task, "cap the retries");
    corpus.set_slot(&format!("commit:{task}#body"), &task, "Driven by flow 54.");

    let out = corpus.jigc(&["task", "finalize", &task, "--format", "json"]);
    // Hand the write bit back before anything else reads or removes the tree, and take the
    // hook out so the follow-up door below is not re-frozen by it.
    set_mode(&area, 0o755);
    fs::remove_file(&hook).expect("remove the hook");
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    assert!(
        out.status.success(),
        "the commit is truth and the teardown is best-effort, so the finalize lands;\n\
         stdout:\n{stdout}\nstderr:\n{stderr}",
    );

    // (1) The cell's own manufacture, asserted rather than assumed: the area really did
    //     survive its teardown, else there is no complement to ask about.
    assert!(
        area.is_dir(),
        "the 0555 racer must really leave the area standing — a removed area makes every \
         assertion below vacuous;\nstdout:\n{stdout}\nstderr:\n{stderr}",
    );

    // (2) The claim: the complement of an area left standing holds NOTHING jigc wrote.
    let complement = engine::state::foreign_area_paths(&area, engine::state::WorkArea::Task)
        .expect("the standing area enumerates");
    assert_eq!(
        complement,
        Vec::<PathBuf>::new(),
        "an area jigc could not tear down holds only jigc's own files here — nothing was \
         planted — so its complement is empty; a name in it is a file jigc wrote that its own \
         registry row does not know about;\nstdout:\n{stdout}\nstderr:\n{stderr}",
    );

    // (3) …so the advisory, if the door raises one, says the area holds nothing but jigc's
    //     own working files, and names no path as foreign.
    let advisories: Vec<String> = json(&stdout)["findings"]
        .as_array()
        .expect("the landed envelope carries `findings`")
        .iter()
        .filter(|f| f["code"].as_str() == Some(KEPT_CODE))
        .map(|f| f["message"].as_str().unwrap_or_default().to_owned())
        .collect();
    for message in &advisories {
        assert!(
            !message.contains(engine::state::FINALIZE_MESSAGE_FILE),
            "…and no surface names jigc's own commit-message transient as a path jigc did \
             not write;\nadvisory:\n{message}",
        );
        assert!(
            message.contains("nothing but jigc's own working files"),
            "…the advisory over an area holding only jigc's files says exactly that;\n\
             advisory:\n{message}",
        );
    }

    // (4) …and no later destroying door refuses over jigc's own file. This is the symptom an
    //     operator actually meets: the area outlives the commit, and `task discard` is the
    //     verb the advisory's route sends them to.
    //
    //     The assertion is *not* exit 0, and the reason is a driven one: this door's
    //     staged-prose guard is asked next and refuses over the staged `commit:<id>.md` the
    //     failed teardown left behind — a different, shipped condition about **authored
    //     prose**, with its own code and its own runnable route. What must be gone is the
    //     refusal over jigc's own transient, which at `45427083` was
    //     `task-discard.foreign-bytes` naming `finalize-message.tmp`.
    let discard = corpus.jigc(&["task", "discard", &task]);
    let refusal = surface(&discard);
    assert!(
        !refusal.contains("task-discard.foreign-bytes"),
        "`jigc task discard {task}` must not answer *foreign bytes* over an area holding \
         nothing but jigc's own files — that refusal demands `--force` to remove a file jigc \
         wrote;\nsurface:\n{refusal}",
    );
    assert!(
        !refusal.contains(engine::state::FINALIZE_MESSAGE_FILE),
        "…and no destroying door names jigc's own commit-message transient at all;\n\
         surface:\n{refusal}",
    );
}

/// **Arm 2's named cell — the `Take` carve-out.** `jigc milestone discard --force` still
/// removes a sub-task area's foreign byte and still acks `workbench removed`: the consenting
/// door's arm is untouched by the `Displace` doors' change.
#[test]
fn arm2_milestone_discard_force_still_takes_a_sub_task_areas_foreign_byte() {
    let fanout = FanOut::build("arm2-take");
    let repo = fanout.repo().to_path_buf();
    plant_into(&task_area(&repo, &fanout.sub), &[ROOT_KEEP]);
    assert_eq!(kept(&repo).len(), 1, "one plant in the sub-task area");

    let out = fanout.run(&["milestone", "discard", MILESTONE, "--force"]);
    let text = surface(&out);
    assert!(out.status.success(), "the consented discard runs;\n{text}");
    assert!(
        kept(&repo).is_empty(),
        "a `Refuse` door WITH its consent takes the bytes — the carve-out is deliberate;\n{text}",
    );
    assert!(
        text.contains("workbench removed"),
        "…and still acks what it did;\n{text}",
    );
}

/// **Arm 2's three zero-false-fire controls (§13)** — an ordinary task lifecycle, a
/// migration task, and an ordinary post-join milestone area each unwind to `Removed`, mint
/// no advisory and leave nothing behind.
///
/// Without these, every assertion above is satisfied by a door that calls **everything**
/// foreign.
#[test]
fn arm2_the_three_controls_mint_no_advisory_and_leave_nothing() {
    // (1) An ordinary task lifecycle, with the doc verbs that write into the area — a
    //     created doc, its authored prose, an in-task retitle and the preview gate.
    let corpus = TrialCorpus::build(State::Fresh);
    let task = corpus.start_workflow("single-task", "Cap the retry budget");
    let repo = corpus.repo();
    corpus.jigc_ok(&[
        "doc",
        "create",
        "adr",
        "--title",
        "Retry cap",
        "--task",
        &task,
    ]);
    for section in ["context", "options", "decision", "consequences"] {
        corpus.set_slot(
            &format!("adr:retry-cap#{section}"),
            &task,
            "Prose the control authored.",
        );
    }
    corpus.jigc_ok(&[
        "doc",
        "rename",
        "adr:retry-cap",
        "--to",
        "Retry cap policy",
        "--task",
        &task,
    ]);
    fs::write(repo.join("README.md"), "hello\nretry cap = 3\n").expect("edit the tracked file");
    corpus.git(&["add", "README.md"]);
    for (field, value) in [("type", "fix"), ("scope", "cli")] {
        corpus.jigc_ok(&[
            "doc",
            "set-field",
            &format!("commit:{task}#{field}"),
            "--value",
            value,
            "--task",
            &task,
        ]);
    }
    corpus.set_slot(&format!("commit:{task}#summary"), &task, "cap the retries");
    corpus.set_slot(&format!("commit:{task}#body"), &task, "Driven by flow 54.");
    corpus.jigc_ok(&["task", "validate", &task]);
    let out = corpus.jigc(&["task", "finalize", &task, "--format", "json"]);
    let text = surface(&out);
    assert!(
        out.status.success(),
        "[control 1] the lifecycle lands;\n{text}"
    );
    assert!(
        !text.contains(KEPT_CODE),
        "[control 1] jigc never calls its OWN files foreign;\n{text}",
    );
    assert!(
        !task_area(&repo, &task).exists(),
        "[control 1] the area is gone;\n{text}",
    );
    assert!(
        !repo.join(".jigc").join("displaced").exists(),
        "[control 1] nothing was parked, so the parking root was never created;\n{text}",
    );

    // (2) An ordinary post-join milestone area — `merged/` and all.
    let fanout = FanOut::build("arm2-control3");
    let repo = fanout.repo().to_path_buf();
    let out = fanout.run(&["milestone", "finalize", MILESTONE, "--format", "json"]);
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    assert!(
        out.status.success(),
        "[control 2] the boundary lands;\nstdout:\n{stdout}\nstderr:\n{stderr}",
    );
    assert!(
        !stderr.contains(KEPT_CODE),
        "[control 2] `merged/` is jigc's own staging tree;\nstderr:\n{stderr}",
    );
    assert_eq!(
        json(&stdout)["committed"]["displaced"]
            .as_array()
            .map(Vec::len),
        Some(0),
        "[control 2] `displaced` is present ALWAYS and empty here;\nstdout:\n{stdout}",
    );
    assert!(
        !repo.join(".jigc").join("displaced").exists(),
        "[control 2] the parking root is not created for an empty move set",
    );
    assert!(
        !milestone_area(&repo).exists() && !task_area(&repo, &fanout.sub).exists(),
        "[control 2] both areas are torn down",
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 3 — the residual × the by-id and enumerating doors
// ═════════════════════════════════════════════════════════════════════════════

use cli::cli::{WORK_UNIT_ID_DOOR_PAYLOAD, WORK_UNIT_ID_DOORS, WORK_UNIT_ID_SLOT};

/// The three residual **shapes** — manufactured, and they differ in exactly the property a
/// roster fix must not quietly change: what a *destroying* door makes of the same directory.
#[derive(Clone, Copy)]
enum Shape {
    /// Nothing at all — the bare `mkdir`, and what an interrupted teardown leaves.
    EmptyDir,
    /// One file jigc did not write, at the area root.
    ForeignFile,
    /// One file under `docs/` wearing a **staged-instance name** — the only shape jigc's
    /// own writer could have produced, so the foreign-bytes probe does not name it and the
    /// pin is the only thing distinguishing it from a live task's staged area.
    ForeignStagedName,
}

impl Shape {
    const ALL: &'static [Shape] = &[
        Shape::EmptyDir,
        Shape::ForeignFile,
        Shape::ForeignStagedName,
    ];

    fn label(self) -> &'static str {
        match self {
            Shape::EmptyDir => "empty dir",
            Shape::ForeignFile => "a foreign file",
            Shape::ForeignStagedName => "a foreign docs/<ty>:<slug>.md",
        }
    }

    /// Plant the shape at `area`, replacing whatever is there.
    fn plant(self, area: &Path) {
        let _ = fs::remove_dir_all(area);
        fs::create_dir_all(area).expect("plant the residual area");
        match self {
            Shape::EmptyDir => {}
            Shape::ForeignFile => {
                fs::write(area.join("notes.txt"), "a third party's bytes\n")
                    .expect("plant the foreign file");
            }
            Shape::ForeignStagedName => {
                let docs = area.join("docs");
                fs::create_dir_all(&docs).expect("plant the residual docs/ tree");
                fs::write(docs.join("adr:leftover.md"), "# Leftover\n")
                    .expect("plant the staged-looking body");
            }
        }
        assert!(
            !area.join("base.json").exists(),
            "a planted residual carries no base pin, or this arm proves nothing",
        );
    }
}

/// Which family a [`WORK_UNIT_ID_DOORS`] row's id belongs to — read off the **clap
/// argument** the id arrives through, never off the verb path, so a milestone verb that
/// grew a `--task` scope would be classified by what it takes.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Family {
    Task,
    Milestone,
}

impl Family {
    fn of(arg: &str) -> Family {
        match arg {
            "task" | "id" => Family::Task,
            "milestone_id" => Family::Milestone,
            other => panic!(
                "`{other}` is a work-unit-id argument with no family — a fourth member owes \
                 this arm the answer its doors give"
            ),
        }
    }

    /// The `(code, target)` pair this family's absence projects, with the code read from
    /// the crate that mints it rather than re-spelled here.
    fn key(self, id: &str) -> (String, String) {
        match self {
            Family::Task => ("finalize.no-task".to_string(), format!("task:{id}")),
            Family::Milestone => (
                engine::milestone::UNKNOWN_MILESTONE_CODE.to_string(),
                format!("milestone:{id}"),
            ),
        }
    }

    fn area_dir(self) -> &'static str {
        match self {
            Family::Task => "tasks",
            Family::Milestone => "milestones",
        }
    }
}

/// **The enumerating doors, derived and stated as a derivation.** The production callers of
/// `engine::state::list_active_task_ids` — the single enumeration source of truth — are the
/// roster (`cli::task`), the orientation view (`cli::orient`), the `also open:` block
/// (`cli::start`), the staged-in hints (`cli::doc`) and `cli::rename`'s staged scope. The
/// first three are the ones that *print an id*, which is what this arm asks about; the
/// last two are asserted at their own cells below.
fn assert_no_enumerating_door_names(repo: &Path, home: &Path, ids: &[String], placement: &str) {
    let names = |surface_name: &str, text: &str| {
        for id in ids {
            assert!(
                !text.contains(id.as_str()),
                "[{placement}] `{surface_name}` named the residual `{id}` — a directory with \
                 no base pin is not a task at any door; got:\n{text}",
            );
        }
    };

    let roster = jigc_ok(repo, home, &["task", "list"]);
    names("jigc task list", &roster);
    assert!(
        roster.contains("no active tasks"),
        "[{placement}] the roster over residuals alone is the EMPTY roster, not a silent \
         one; got:\n{roster}",
    );

    let raw = jigc_ok(repo, home, &["task", "list", "--format", "json"]);
    names("jigc task list --format json", &raw);
    assert_eq!(
        json(&raw).as_array().map(Vec::len),
        Some(0),
        "[{placement}] the machine roster is the empty array; got:\n{raw}",
    );

    let orientation = jigc_ok(repo, home, &["start"]);
    names("jigc start", &orientation);
    let view = jigc_ok(repo, home, &["start", "--format", "json"]);
    names("jigc start --format json", &view);
    assert_eq!(
        json(&view)["state"],
        Value::String("clean".into()),
        "[{placement}] the versioned orientation envelope tags the state clean; got:\n{view}",
    );

    // Last, because it mints: the `also open:` block a work-starting form appends.
    let started = jigc_ok(repo, home, &["start", "Add a rate limiter"]);
    names("jigc start \"<intent>\"", &started);
    assert!(
        !started.contains("also open:"),
        "[{placement}] nothing was open, so the block renders no bytes at all; got:\n{started}",
    );
}

/// Drive every [`WORK_UNIT_ID_DOORS`] row of `family` against `id`, and assert the row
/// answers its own family's shipped pair with the residual sentence and no host path.
fn assert_by_id_doors_answer(
    repo: &Path,
    home: &Path,
    family: Family,
    id: &str,
    cell: &str,
    host_root: &str,
) -> usize {
    let (code, target) = family.key(id);
    let listed = format!(".jigc/{}/{id}", family.area_dir());
    let sentence = engine::state::residual_area_note(
        &listed,
        match family {
            Family::Task => "task",
            Family::Milestone => "milestone",
        },
    );
    let mut rows = 0_usize;

    for row in WORK_UNIT_ID_DOORS {
        if Family::of(row.arg) != family {
            continue;
        }
        rows += 1;
        let argv: Vec<String> = row
            .argv
            .iter()
            .map(|token| {
                if *token == WORK_UNIT_ID_SLOT {
                    id.to_owned()
                } else {
                    (*token).to_owned()
                }
            })
            .collect();
        let mut with_format: Vec<&str> = argv.iter().map(String::as_str).collect();
        with_format.extend(["--format", "json"]);
        let door = format!("jigc {}", row.door.join(" "));

        let out = jigc(repo, home, &with_format);
        let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
        let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
        assert!(
            !out.status.success(),
            "[{cell}] `{door}` must refuse over a residual;\nstdout:\n{stdout}\nstderr:\n{stderr}",
        );
        assert!(
            stdout.trim().is_empty(),
            "[{cell}] `{door}` puts nothing on stdout when it rejects;\nstdout:\n{stdout}",
        );
        let document = json(stderr.trim());
        let findings = document["findings"]
            .as_array()
            .unwrap_or_else(|| {
                panic!("[{cell}] `{door}` answers the FINDINGS envelope; got:\n{stderr}")
            })
            .clone();
        assert_eq!(
            findings.len(),
            1,
            "[{cell}] `{door}` answers one condition with one finding; got:\n{stderr}",
        );
        assert_eq!(
            (
                findings[0]["key"]["code"]
                    .as_str()
                    .unwrap_or_default()
                    .to_owned(),
                findings[0]["key"]["target"]
                    .as_str()
                    .unwrap_or_default()
                    .to_owned(),
            ),
            (code.clone(), target.clone()),
            "[{cell}] `{door}` owes its own FAMILY's shipped pair; got:\n{stderr}",
        );
        assert!(
            stderr.contains(&sentence),
            "[{cell}] `{door}` carries the residual sentence from its one home; got:\n{stderr}",
        );
        assert!(
            stderr.contains(&listed),
            "[{cell}] `{door}` names the repo-relative area to clear; got:\n{stderr}",
        );
        assert!(
            !stderr.contains(host_root),
            "[{cell}] `{door}` printed the HOST filesystem (law 1); got:\n{stderr}",
        );
    }
    rows
}

/// `jigc milestone add-task <milestone> "<intent>"`, returning the **minted** sub-task id
/// read off the ack rather than re-slugged in test code.
fn add_sub_task(repo: &Path, home: &Path, milestone: &str, intent: &str) -> String {
    let ack = jigc_ok(repo, home, &["milestone", "add-task", milestone, intent]);
    let (_, rest) = ack
        .split_once("added task:")
        .unwrap_or_else(|| panic!("`milestone add-task` names the task it minted; got:\n{ack}"));
    rest.split_whitespace()
        .next()
        .expect("the ack's task id is one token")
        .to_string()
}

/// A corpus with the payload file the two `--from-file` rows read, so those rows fault on
/// the **id** and never on a missing file.
fn residual_corpus(state: State) -> TrialCorpus {
    let corpus = TrialCorpus::build(state);
    fs::write(
        corpus.repo().join(WORK_UNIT_ID_DOOR_PAYLOAD),
        "title: Axis\nsections: []\n",
    )
    .expect("write the door payload");
    corpus
}

/// **Arm 3, placement 1 — a plain id**, plus the milestone family's own residual.
#[test]
fn arm3_a_plain_residual_is_a_work_unit_at_no_door() {
    let corpus = residual_corpus(State::CommittedSingletons);
    let repo = corpus.repo();
    let home = corpus.home();
    let host_root = repo.to_string_lossy().into_owned();

    let task_ids: Vec<String> = ["stray-alpha", "stray-beta", "stray-gamma"]
        .iter()
        .map(|s| (*s).to_string())
        .collect();
    let milestone_ids: Vec<String> = ["stray-mile-a", "stray-mile-b", "stray-mile-c"]
        .iter()
        .map(|s| (*s).to_string())
        .collect();
    for (shape, id) in Shape::ALL.iter().zip(&task_ids) {
        shape.plant(&repo.join(".jigc").join("tasks").join(id));
    }
    for (shape, id) in Shape::ALL.iter().zip(&milestone_ids) {
        shape.plant(&repo.join(".jigc").join("milestones").join(id));
    }

    assert_no_enumerating_door_names(&repo, &home, &task_ids, "plain id");

    let mut task_rows = 0;
    for (shape, id) in Shape::ALL.iter().zip(&task_ids) {
        task_rows = assert_by_id_doors_answer(
            &repo,
            &home,
            Family::Task,
            id,
            &format!("plain id · {}", shape.label()),
            &host_root,
        );
    }
    let mut milestone_rows = 0;
    for (shape, id) in Shape::ALL.iter().zip(&milestone_ids) {
        milestone_rows = assert_by_id_doors_answer(
            &repo,
            &home,
            Family::Milestone,
            id,
            &format!("plain id · {}", shape.label()),
            &host_root,
        );
    }
    assert_eq!(
        task_rows + milestone_rows,
        WORK_UNIT_ID_DOORS.len(),
        "every row of the registry is driven — the two families partition it",
    );

    // The same-slug mint refuses with the residual sentence rather than minting over it.
    let refused = jigc(
        &repo,
        &home,
        &["start", "--workflow", "quick-fix", "Stray alpha"],
    );
    let text = surface(&refused);
    assert!(
        !refused.status.success(),
        "the mint over a residual refuses;\n{text}"
    );
    assert!(
        text.contains(&engine::state::residual_area_note(
            ".jigc/tasks/stray-alpha",
            "task"
        )),
        "…with the residual sentence, from the one home;\n{text}",
    );

    // `jigc rename` PROCEEDS over a task residual — `first_dir_name` now asks the pin, so a
    // leftover no longer wedges an unrelated verb.
    let renamed = jigc(
        &repo,
        &home,
        &["rename", "adr:cache-policy", "--to", "Cache policy v2"],
    );
    let text = surface(&renamed);
    assert!(
        !text.contains("stray-alpha"),
        "`jigc rename` must not be wedged by a residual;\n{text}",
    );
}

/// **Arm 3, placements 2 and 3 — a sub-task of an open milestone and of a joined one**, and
/// the named cell: `jigc task discard` over a joined milestone's residual commits nothing.
#[test]
fn arm3_a_sub_task_residual_is_a_work_unit_at_no_door_open_or_joined() {
    for joined in [false, true] {
        let placement = if joined {
            "sub-task of a joined milestone"
        } else {
            "sub-task of an open milestone"
        };
        let corpus = residual_corpus(State::CommittedSingletons);
        let repo = corpus.repo();
        let home = corpus.home();
        let host_root = repo.to_string_lossy().into_owned();

        jigc_ok(&repo, &home, &["milestone", "create", "Cache rework"]);
        let ids: Vec<String> = ["Warm the read cache", "Shard the index", "Trim the log"]
            .iter()
            .map(|intent| add_sub_task(&repo, &home, MILESTONE, intent))
            .collect();

        if joined {
            // One sub-task contributes a promotable doc, so the boundary is not a
            // `milestone.zero-contribution` refusal.
            stage_doc(&repo, &ids[0], "adr:warm-policy", &adr_body("Warm policy"));
            jigc_ok(&repo, &home, &["milestone", "finalize", MILESTONE]);
            let record = jigc_ok(
                &repo,
                &home,
                &["doc", "show", &format!("milestone-record:{MILESTONE}")],
            );
            assert!(
                record.contains("status: joined"),
                "the milestone must really be joined; got:\n{record}",
            );
        }

        for (shape, id) in Shape::ALL.iter().zip(&ids) {
            shape.plant(&repo.join(".jigc").join("tasks").join(id));
        }

        assert_no_enumerating_door_names(&repo, &home, &ids, placement);
        for (shape, id) in Shape::ALL.iter().zip(&ids) {
            assert_by_id_doors_answer(
                &repo,
                &home,
                Family::Task,
                id,
                &format!("{placement} · {}", shape.label()),
                &host_root,
            );
        }

        if joined {
            // **The named cell**: a bare `mkdir` under a joined milestone, then
            // `jigc task discard <sub>` — refused, with NO record commit.
            let before = git(&repo, &["rev-parse", "HEAD"]);
            let log = git(&repo, &["log", "--oneline"]);
            let out = jigc(&repo, &home, &["task", "discard", &ids[0]]);
            let text = surface(&out);
            assert!(
                !out.status.success(),
                "[{placement}] `jigc task discard` over a residual refuses;\n{text}",
            );
            assert_eq!(
                git(&repo, &["rev-parse", "HEAD"]),
                before,
                "[{placement}] …and commits NOTHING — the false-flip path is closed at \
                 resolution, one layer before the record;\n{text}",
            );
            assert_eq!(
                git(&repo, &["log", "--oneline"]),
                log,
                "[{placement}] `git log` is unchanged;\n{text}",
            );
            assert!(
                repo.join(".jigc").join("tasks").join(&ids[0]).exists(),
                "[{placement}] …and the directory is left exactly as it was;\n{text}",
            );
        }
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 4 — the four uncommitted-pick states × BEHALF_DOORS' acting rows
// ═════════════════════════════════════════════════════════════════════════════

use cli::cli::{ActsOnBehalf, BEHALF_DOORS};
use cli::repo::InProgress;
use support::git_state::{GitState, GitStateRepo};

/// The four states Increment 4 minted — the clean pick, the same pick over a range, the
/// conflicted one, and the conflicted one the user resolved with `git add`.
///
/// **Derived, never listed**: the filter is *the member the fixture declares it produces*
/// (`GitState::in_progress`), so a fifth cell of the same member joins this arm with no
/// edit here, and a cell re-declared onto another member leaves it the same way.
fn uncommitted_pick_states() -> Vec<GitState> {
    GitState::ALL
        .iter()
        .copied()
        .filter(|state| state.in_progress() == Some(InProgress::UncommittedCherryPick))
        .collect()
}

/// The well-formed work-unit id every `<id>` slot is filled with. The posture guard answers
/// before any id resolution, so the id need only be well-formed.
const AXIS_ID: &str = "axis-unit";

/// The cells with **no outstanding conflict**, where a conflict-shaped conclude phrase
/// would be a lie — read off the fixture's own driven expectation (`unmerged == 0`) rather
/// than decided here, so the claim rests on what git actually left in the index.
fn is_clean_cell(state: GitState) -> bool {
    support::git_state::expectation(state).unmerged == 0
}

/// The `MERGE_MSG` of the worktree at `at` — resolved per worktree, because a linked
/// worktree's git dir lives under `.git/worktrees/<name>/`.
fn merge_msg(at: &Path) -> PathBuf {
    let dir = git(at, &["rev-parse", "--absolute-git-dir"]);
    PathBuf::from(dir.trim()).join("MERGE_MSG")
}

/// **Arm 4** — the four uncommitted-pick states × every acting `BEHALF_DOORS` row: the
/// refusal leaves the authored message and the index **byte-unchanged**, and the abandon
/// argv the door printed, run verbatim, leaves the picked changes in the working tree.
#[test]
fn arm4_an_uncommitted_pick_is_refused_and_nothing_of_it_is_consumed() {
    let states = uncommitted_pick_states();
    assert_eq!(
        states.len(),
        4,
        "Increment 4 minted four cells of one member — clean, range, conflicted, resolved",
    );

    let acting: Vec<(&'static [&'static str], &'static [&'static str])> = BEHALF_DOORS
        .iter()
        .filter_map(|row| match &row.acts {
            ActsOnBehalf::CommitsOnBehalf { argv, .. } | ActsOnBehalf::MovesOnBehalf { argv } => {
                Some((row.door, *argv))
            }
            ActsOnBehalf::Neither => None,
        })
        .collect();
    assert_eq!(
        acting.len(),
        12,
        "the acting classes of the total leaf classification — a row added anywhere reddens \
         until someone answers what it acts on",
    );

    for state in states {
        let fixture = GitStateRepo::build(state);
        let repo = fixture.repo();
        let home = fixture.home();
        let label = state.name();

        let msg_path = merge_msg(&repo);
        let message = fs::read(&msg_path).unwrap_or_else(|err| {
            panic!("[{label}] the fixture's own `MERGE_MSG` must exist: {err}")
        });
        let index_before = git(&repo, &["ls-files", "--stage"]);
        let head_before = git(&repo, &["rev-parse", "HEAD"]);
        let mut routes: BTreeSet<String> = BTreeSet::new();

        for (door, argv) in &acting {
            let filled: Vec<String> = argv
                .iter()
                .map(|token| {
                    if *token == WORK_UNIT_ID_SLOT {
                        AXIS_ID.to_owned()
                    } else {
                        (*token).to_owned()
                    }
                })
                .collect();
            let args: Vec<&str> = filled.iter().map(String::as_str).collect();
            let name = format!("jigc {}", door.join(" "));

            let out = jigc(&repo, &home, &args);
            let text = surface(&out);
            assert!(
                !out.status.success(),
                "[{label}] `{name}` must refuse an un-concluded pick;\n{text}",
            );
            assert!(
                text.contains("repo.operation-in-progress"),
                "[{label}] `{name}` refuses under the family's identity;\n{text}",
            );
            assert!(
                text.contains("cherry-pick"),
                "[{label}] …naming the member's own noun;\n{text}",
            );
            if is_clean_cell(state) {
                assert!(
                    !text.contains("once its conflicts are resolved"),
                    "[{label}] `{name}` must not claim conflicts this cell does not have — \
                     the shipped qualifier is false here;\n{text}",
                );
            }

            // **The three damage shapes, all closed at once**: the authored message
            // survives (no swallow, no message-only kill) and the index is untouched (no
            // contamination).
            assert_eq!(
                fs::read(&msg_path).ok().as_deref(),
                Some(message.as_slice()),
                "[{label}] `{name}` left the authored `MERGE_MSG` byte-changed or gone;\n{text}",
            );
            assert_eq!(
                git(&repo, &["ls-files", "--stage"]),
                index_before,
                "[{label}] `{name}` contaminated the index;\n{text}",
            );
            assert_eq!(
                git(&repo, &["rev-parse", "HEAD"]),
                head_before,
                "[{label}] `{name}` concluded the user's pick;\n{text}",
            );

            let route = text
                .split_once("abandon it with `")
                .map(|(_, rest)| {
                    rest.split_once('`')
                        .expect("the abandon command is backtick-delimited")
                        .0
                        .to_owned()
                })
                .unwrap_or_else(|| panic!("[{label}] `{name}` owes an abandoning route;\n{text}"));
            routes.insert(route);
        }

        // One producer, so every door prints the same abandon command.
        assert_eq!(
            routes.len(),
            1,
            "[{label}] every acting door prints ONE abandon route; got {routes:?}",
        );
        let route = routes.into_iter().next().expect("one route");

        // The emitted bytes, split by a real shell and RUN — a route that cannot be
        // followed is a route that does not exist.
        let tokens: Vec<String> = route
            .strip_prefix("git ")
            .unwrap_or_else(|| panic!("[{label}] the abandon route is a `git` command: {route}"))
            .split_whitespace()
            .map(str::to_owned)
            .collect();
        let refs: Vec<&str> = tokens.iter().map(String::as_str).collect();
        let ran = Command::new("git")
            .args(&refs)
            .current_dir(&repo)
            .output()
            .expect("run the emitted abandon command");
        assert!(
            ran.status.success(),
            "[{label}] `{route}` must be runnable: {}",
            String::from_utf8_lossy(&ran.stderr),
        );
        assert!(
            cli::repo::posture(&repo).is_empty(),
            "[{label}] …and it leaves the repository with no operation in progress",
        );
        assert!(
            !git(&repo, &["status", "--porcelain"]).trim().is_empty(),
            "[{label}] …with the PICKED CHANGES still in the working tree — `git reset` \
             unstages, it does not discard",
        );
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 5 — MINT_DOORS ∪ `milestone add-from-spec` × the degenerate titles
// ═════════════════════════════════════════════════════════════════════════════

use engine::state::{MINT_DOORS, Snapshot};

/// The titles that yield **no id at all**. Only the first three read as abuse; the last two
/// are a title in another script and a title of stopwords alone, and both are things a
/// person types on purpose — which is why the refusal's sentence has to be true of all five.
const DEGENERATE: &[&str] = &["", "   ", "!!!", "日本語", "the of a"];

/// The identity the mint class refuses with.
const UNSLUGABLE: &str = "write.unslugable-title";

/// A spec whose one criterion is titled in a script the slug rule yields nothing from — the
/// input `milestone add-from-spec` turns into a sub-task title.
const DEGENERATE_SPEC: &str = "\
# Rate limit

## Goal

Bound per-client request volume.

## Context

Downstream services enforced limits ad hoc.

## Criteria

### 日本語  {#degenerate-criterion}

The criterion whose title yields no id.
";

/// A fixture repo that is set up and standing on a clean tree, so *"the refused door left
/// `git status` clean"* is a statement about the door rather than about the fixture.
fn mint_fixture(tag: &str) -> (TempDir, TempDir) {
    let repo = TempDir::new(&format!("mint-{tag}"));
    let home = TempDir::new(&format!("mint-home-{tag}"));
    init_repo(repo.path());
    jigc_ok(repo.path(), home.path(), &["setup"]);
    git(repo.path(), &["add", "-A"]);
    let out = Command::new("git")
        .args(["commit", "-q", "-m", "the fixture's own bytes"])
        .current_dir(repo.path())
        .output()
        .expect("run git commit");
    let _ = out;
    assert!(
        git(repo.path(), &["status", "--porcelain"])
            .trim()
            .is_empty(),
        "the degenerate cells start from a clean tree",
    );
    (repo, home)
}

/// Every working-area name currently on disk, task areas and milestone areas alike.
fn area_names(repo: &Path) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for (kind, dir) in [
        ("task", repo.join(".jigc").join("tasks")),
        ("milestone", repo.join(".jigc").join("milestones")),
    ] {
        let Ok(entries) = fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                out.push((
                    kind.to_string(),
                    entry.file_name().to_string_lossy().into_owned(),
                ));
            }
        }
    }
    out.sort();
    out
}

/// Drive one door over the **whole** degenerate title set and assert the refusal is *before
/// any write*: the class's identity and the half of its sentence that is about the caller's
/// title, and HEAD, the working-area set and the working tree exactly as they were.
fn refuses_before_any_write(
    repo: &Path,
    home: &Path,
    door: &str,
    argv: &dyn Fn(&str) -> Vec<String>,
) {
    for title in DEGENERATE {
        let head = git(repo, &["rev-parse", "HEAD"]);
        let areas = area_names(repo);
        let owned = argv(title);
        let args: Vec<&str> = owned.iter().map(String::as_str).collect();
        let out = jigc(repo, home, &args);
        let text = surface(&out);

        assert!(
            !out.status.success(),
            "[{door}] must refuse the title {title:?};\n{text}",
        );
        assert!(
            text.contains(UNSLUGABLE),
            "[{door}] refuses {title:?} under `{UNSLUGABLE}`;\n{text}",
        );
        assert!(
            text.contains(
                "ids are built from ASCII letters and digits, so a title in another script, \
                 or of stopwords only, yields none"
            ),
            "[{door}] owes the class's sentence, which is true for a title in ANY script;\n{text}",
        );
        assert_eq!(
            git(repo, &["rev-parse", "HEAD"]),
            head,
            "[{door}] moved HEAD while refusing {title:?} — the refusal precedes every write, \
             the record commit included;\n{text}",
        );
        assert_eq!(
            area_names(repo),
            areas,
            "[{door}] left a working area behind while refusing {title:?};\n{text}",
        );
        assert!(
            git(repo, &["status", "--porcelain"]).trim().is_empty(),
            "[{door}] left the working tree dirty while refusing {title:?};\n{text}",
        );
    }
}

/// **Arm 5** — the mint-door registry, plus the one door no registry row names, crossed with
/// the five degenerate titles.
#[test]
fn arm5_no_door_mints_a_work_unit_at_a_fabricated_identity() {
    let mut prose = 0_usize;
    let mut exempt = 0_usize;

    for door in MINT_DOORS {
        match door.site {
            // ── the three prose rows: the id is slugged from the caller's title ──
            "crates/cli/src/start.rs::mint_in_repo" => {
                prose += 1;
                let (repo, home) = mint_fixture("start");
                refuses_before_any_write(repo.path(), home.path(), door.door, &|title| {
                    vec![
                        "start".into(),
                        "--workflow".into(),
                        "quick-fix".into(),
                        title.to_owned(),
                    ]
                });
            }
            "crates/cli/src/milestone.rs::run_create" => {
                prose += 1;
                let (repo, home) = mint_fixture("create");
                refuses_before_any_write(repo.path(), home.path(), door.door, &|title| {
                    vec!["milestone".into(), "create".into(), title.to_owned()]
                });
            }
            "crates/engine/src/milestone.rs::add_task" => {
                prose += 1;
                let (repo, home) = mint_fixture("add-task");
                jigc_ok(
                    repo.path(),
                    home.path(),
                    &["milestone", "create", "Rate limit"],
                );
                git(repo.path(), &["add", "-A"]);
                refuses_before_any_write(repo.path(), home.path(), door.door, &|title| {
                    vec![
                        "milestone".into(),
                        "add-task".into(),
                        "rate-limit".into(),
                        title.to_owned(),
                    ]
                });
            }
            // ── the two rows whose id comes from something that is not a title ──
            //
            // They owe no degenerate cell — there is no title to degenerate — so what this
            // arm asks of them instead is that they still mint the id they derive, driven
            // rather than asserted. The reason is stated per row on the `IdSource` mold the
            // registry already uses for `Snapshot`.
            "crates/cli/src/start.rs::mint_migration_in_repo" => {
                exempt += 1;
                // The id is a `blake3` of the repo-relative source path (M44), never prose.
                let (repo, home) = mint_fixture("migrate");
                let legacy = repo.path().join("legacy");
                fs::create_dir_all(&legacy).expect("mk legacy/");
                fs::write(
                    legacy.join("CHANGELOG.md"),
                    "# Change Log\n\n## v1\n\n- a thing\n",
                )
                .expect("write the foreign source");
                git(repo.path(), &["add", "-A"]);
                let out = Command::new("git")
                    .args(["commit", "-q", "-m", "the foreign source"])
                    .current_dir(repo.path())
                    .output()
                    .expect("run git commit");
                assert!(out.status.success(), "the foreign-source commit lands");
                jigc_ok(
                    repo.path(),
                    home.path(),
                    &["migrate", "legacy/CHANGELOG.md", "--as", "changelog"],
                );
                let minted = area_names(repo.path());
                assert_eq!(
                    minted.len(),
                    1,
                    "[{}] the row mints exactly one area from a path, not a title",
                    door.door,
                );
                assert!(
                    engine::slug::is_slug(&minted[0].1),
                    "[{}] …and its id is well-formed, so every by-id door accepts it",
                    door.door,
                );
            }
            "crates/engine/src/milestone.rs::reseed_sub_task_areas" => {
                exempt += 1;
                // The id is the one the **committed record** already names: the re-seed
                // rebuilds an area, it does not name a work unit. It is also the registry's
                // second `Snapshot::Exempt` row, for a related reason.
                assert!(
                    matches!(door.snapshot, Snapshot::Exempt(_)),
                    "[{}] the re-seed writes no staged snapshot either",
                    door.door,
                );
                let (repo, home) = mint_fixture("reseed");
                jigc_ok(
                    repo.path(),
                    home.path(),
                    &["milestone", "create", "Rate limit"],
                );
                let sub = add_sub_task(repo.path(), home.path(), "rate-limit", "Warm the cache");
                // The fresh-clone shape: the gitignored workbench areas are gone and the
                // committed record is all that is left.
                fs::remove_dir_all(repo.path().join(".jigc").join("tasks"))
                    .expect("empty the task areas");
                jigc_ok(
                    repo.path(),
                    home.path(),
                    &["milestone", "add-task", "rate-limit", "Shard the index"],
                );
                assert!(
                    task_area(repo.path(), &sub).join("base.json").is_file(),
                    "[{}] the re-seed rebuilds the RECORDED id's area, pin and all",
                    door.door,
                );
            }
            other => panic!(
                "`{other}` is a mint door with no answer about a degenerate title — a sixth \
                 row owes this arm one, exactly as it already owes `Snapshot` a value"
            ),
        }
    }
    assert_eq!(prose, 3, "three prose-titled mint doors");
    assert_eq!(exempt, 2, "two rows derive their id from something else");
    assert_eq!(
        prose + exempt,
        MINT_DOORS.len(),
        "every registry row answered"
    );

    // ── the derivation: `milestone add-from-spec` reaches the same `add_task` ──
    let (repo, home) = mint_fixture("from-spec");
    let specs = repo.path().join("docs").join("specs");
    fs::create_dir_all(&specs).expect("mk docs/specs/");
    fs::write(specs.join("rate-limit.md"), DEGENERATE_SPEC).expect("write the spec");
    git(repo.path(), &["add", "-A"]);
    let out = Command::new("git")
        .args(["commit", "-q", "-m", "add the spec"])
        .current_dir(repo.path())
        .output()
        .expect("run git commit");
    assert!(out.status.success(), "the spec commit lands");
    jigc_ok(
        repo.path(),
        home.path(),
        &["milestone", "create", "Rate limit"],
    );

    let head = git(repo.path(), &["rev-parse", "HEAD"]);
    let areas = area_names(repo.path());
    let refused = jigc(
        repo.path(),
        home.path(),
        &[
            "milestone",
            "add-from-spec",
            "rate-limit",
            "spec:rate-limit",
        ],
    );
    let text = surface(&refused);
    assert!(
        !refused.status.success(),
        "`jigc milestone add-from-spec` must refuse a criterion whose title yields no id;\n{text}",
    );
    assert!(
        text.contains(UNSLUGABLE),
        "…under the same identity as the registry's rows — one site, two doors;\n{text}",
    );
    assert_eq!(
        git(repo.path(), &["rev-parse", "HEAD"]),
        head,
        "…before any record commit;\n{text}",
    );
    assert_eq!(
        area_names(repo.path()),
        areas,
        "…and with no sub-task area left behind;\n{text}",
    );

    // ── §12's boundary cell: outside a repository, the not-in-repo answer still wins ──
    let outside = TempDir::new("no-repo");
    let home = TempDir::new("no-repo-home");
    let out = jigc(outside.path(), home.path(), &["milestone", "create", ""]);
    let text = surface(&out);
    assert!(
        !out.status.success(),
        "outside a repository the door refuses;\n{text}"
    );
    assert!(
        !text.contains(UNSLUGABLE),
        "…with the ONE not-in-repo answer, not the new guard's — jigc outside a git \
         repository answers once (M49);\n{text}",
    );
    assert!(text.contains("git repository"), "…and says so;\n{text}",);
}
