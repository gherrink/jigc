//! **The M48 rc.11-wave done-picture acceptance suite** — the *pre-1.0 reliability +
//! discoverability wave*, driven end-to-end through the **real `jigc` binary**
//! (`design/worked-examples.md` → flow 48; roadmap → Milestone 48, Increment 12).
//!
//! **The claim the wave proves: the discoverability lens that landed six consecutive
//! trials is a *mechanism* defect, and the mechanism is countable and fenceable** —
//! riding with it, as the gate rather than the point: no shipped door destroys
//! uncommitted work at exit 0, no write path acks success over a silent no-op, and the
//! pre-1.0 additive-key window closes. Increments 1–11 shipped each fix with its own
//! axis suite; this suite is the **composite acceptance** that ties the wave into six
//! done-picture arms over the real binary — **each arm enumerating its axis from a
//! code-side registry or from the class's defining case-set**, never pinning the single
//! repro the trial reported. Where the wave found no registry it **minted** one, and the
//! arms below consume the mints rather than re-typing their contents.
//!
//! The six arms — eight `#[test]`s over the real binary, since arms 1 and 6 each carry two
//! cells that need their own fixture world:
//!
//!   (1) **No destroying door takes work it cannot prove is junk** — the axis is the
//!       code-side [`cli::milestone::DESTROYING_DOORS`] × [`cli::milestone::LEFTOVER_VERDICTS`]
//!       matrix (3 × 3), plus `uninstall`'s authored-prose cell: every cell refuses with
//!       **that door's** blocking code, leaves the planted bytes byte-intact, and names
//!       `--force` — which is then driven and is the only way past (Inc 1).
//!
//!   (2) **An authoring step cannot ship without naming the read-back, and the read it
//!       names runs** — the owe-set is re-derived from **both** loaded packs
//!       ([`cli::doc::doc_write_verbs`] ∪ the `{{schema:<T>}}` arm) and compared against
//!       the [`cli::pack::STAGED_READ_BACK_CODE`] declarers; the composed
//!       `jigc doc show … --task <id>` line is then **lifted verbatim out of a real
//!       composition and run**, serving the staged write the committed store does not
//!       carry (Inc 3).
//!
//!   (3) **No minting verb acks success over a title it dropped or diverted** — the axis
//!       is the **doctype census** re-derived from the loaded schema registry (fixed
//!       identity · slug identity · transient sink), driven at **both** minting verbs,
//!       with the reject's emitted route run verbatim to a **landed in-task title
//!       change** (Inc 2).
//!
//!   (4) **One file answers one code and one route at every door, and the install commit
//!       carries what it claims** — the class axis is the discriminator's own three
//!       cells (foreign · managed below-or-unstamped · managed at-version) across the
//!       **store sweep** and the **task gate**; riding with it, `jigc setup` commits the
//!       `pre-commit` hook **iff** the hook is a working-tree file, over both producible
//!       hooks-dir shapes (Inc 4 + Inc 5).
//!
//!   (5) **A read intent lands on a read verb, and every write ack and exit flip states
//!       its fact** — three code-side registries in one arm:
//!       [`cli::cli::CURATED_SIBLING_TIPS`] and [`cli::cli::PARENT_READ_ANSWERS`] ×
//!       [`cli::cli::READ_INTENT_GUESSES`] (every command span in the printed tip is a
//!       [`cli::cli::VerbKind::Read`] verb), [`cli::render::ConfigAck::ALL`] driven
//!       **through the binary** at all six authoring verbs, and
//!       [`cli::render::STORE_EXIT_FLIPS`] — whose members are produced **live** and
//!       checked against the `.jigc/AGENT.md` preload clause that promises them
//!       (Inc 6 + Inc 7).
//!
//!   (6) **No door frames an empty commit as a rejection, and the guide artifact stays
//!       jigc's** — the axis is [`cli::invocation_log::COMMITTING_DOORS`] driven into the
//!       state where its commit records nothing, plus the adapter artifact's own
//!       ownership case-set ([`cli::setup::GuideOwnership`], matched exhaustively so a
//!       fourth verdict cannot compile without a cell) (Inc 8 + Inc 10).
//!
//! **Increment 11 deliberately gets no arm.** Its deliverable is a **CI build fence** over
//! the two schema manifests — no verb, no finding, no route, nothing a walk through the
//! binary can reach — and its live arm is `#[ignore]`d by design, invoked by the named CI
//! step (`crates/cli/tests/manifest_freeze_fence.rs`). An acceptance arm here could only
//! re-run that suite's comparator, which is a receipt for a test rather than a
//! done-picture, so the wave records the absence instead of manufacturing a walk.
//!
//! **The declared proof split.** Each arm proves the wave's claim at the *done-picture*
//! altitude; the per-fix mechanism clauses stay with the dedicated axis suites and are
//! not re-proven here: the leftover classifier's per-verdict fixtures and its negative
//! controls are `provision_leftover_guard.rs`'s and `uninstall_worktree_guard.rs`'s; the
//! read-back fence's applied-mutation redness is `read_back_fence.rs`'s; the title
//! guard's incumbent-bytes × supplied-title input axis is `write_title_divergence.rs`'s
//! and the in-task re-slug's derived-referrer sweep `doc_rename_in_task.rs`'s; the
//! four-stamp route axis is `foreign_at_both_doors.rs`'s and the four-shape hooks-dir
//! axis `setup.rs`'s; the parity fence over every `--format json` leaf is
//! `text_json_parity_axis.rs`'s; the per-door empty-commit diagnoses are
//! `commit_rejected_axis.rs`'s and the guide artifact's install/upgrade/uninstall
//! life-cycle `adapter_artifact.rs`'s. One member of arm 5's exit-flip axis
//! (`probe-unreliable`) is produced live by `store_sweep_acceptance.rs` (c) rather than
//! here — it needs a crashing probe over an anchored store — and the arm classifies it
//! **explicitly**, so an unclassified member is a hard panic, never a silent skip.
//!
//! **Red on `1.0.0-rc.10`**, arm by arm: `milestone provision` deleted a leftover's
//! staged, unstaged *and* untracked work at exit 0 and `uninstall` destroyed authored
//! prose at exit 0; exactly one shipped step file named `doc show` at all, and not as a
//! read-back (the denominator lives in one home — `implementation/decisions-pending.md`);
//! a re-`create`/`author` under a corrected title acked success and committed two ADRs;
//! one foreign file answered two codes at two doors and the managed cell's route told a
//! stamped doc to adopt itself; `jigc doc read` was answered with `'create', 'rename'` —
//! two writes — while six `config` writes said nothing about being uncommitted; an
//! idempotent `rename` dressed git's empty-commit refusal in the survivable-rejection
//! frame; and no adopter had a version-matched path to the guides at all.
//!
//! Isolation: the arms that need a built corpus ride the shared [`support::trial_corpus`]
//! substrate (process-unique tempdir, per-repo git identity, `$HOME` repointed,
//! `JIGC_PACK_DIR` scrubbed); the arms that need **worktrees** build their repos fresh —
//! `TrialCorpus::copy_state` refuses a worktree-bearing corpus by design
//! ([pinning.md](../../../implementation/pinning.md) §4: a copied `.git/worktrees/*/gitdir`
//! holds absolute paths back into the source), so arm 1's nine cells and arm 6's fan-out
//! doors mint their fixtures per cell. Cost accepted at planning.

use crate::support;

use clap::CommandFactory;
use cli::cli::{
    CURATED_SIBLING_TIPS, Cli, PARENT_READ_ANSWERS, READ_INTENT_GUESSES, VerbKind, verb_kind,
};
use cli::milestone::{DESTROYING_DOORS, DestroyingDoor, LEFTOVER_VERDICTS, LeftoverVerdict};
use cli::pack::{CompositePack, EmbeddedPack};
use cli::render::{ConfigAck, STORE_EXIT_FLIPS};
use cli::setup::GuideOwnership;
use engine::packsource::{PackResourceKind, PackSource};
use engine::schema::Schema;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use support::trial_corpus::{State, TrialCorpus};

// ═════════════════════════════════════════════════════════════════════════════
// Shared helpers
// ═════════════════════════════════════════════════════════════════════════════

/// A throwaway directory that removes itself on drop — for the arms that build their
/// repos outside a [`TrialCorpus`] (every worktree-bearing cell, the hooks-dir shapes).
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-flow48-{tag}-{}-{:?}",
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
        // Linked worktrees inside the tree are ordinary directories to `remove_dir_all`;
        // a moved-away source is simply already gone.
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// Stdout of an invocation as UTF-8.
fn stdout_of(out: &Output) -> String {
    String::from_utf8_lossy(&out.stdout).into_owned()
}

/// Stderr of an invocation as UTF-8.
fn stderr_of(out: &Output) -> String {
    String::from_utf8_lossy(&out.stderr).into_owned()
}

/// Everything the run printed, both streams — the reader's view of one invocation.
fn printed(out: &Output) -> String {
    format!("{}{}", stdout_of(out), stderr_of(out))
}

fn owned(args: &[&str]) -> Vec<String> {
    args.iter().map(|arg| (*arg).to_string()).collect()
}

/// Run `git <args>` in `cwd`, asserting success, returning stdout.
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
    String::from_utf8_lossy(&out.stdout).into_owned()
}

/// Initialize a real git repo with one commit (every mint reads HEAD).
fn init_repo(root: &Path) {
    git_ok(root, &["init", "-q"]);
    git_ok(root, &["config", "user.email", "test@example.com"]);
    git_ok(root, &["config", "user.name", "Test"]);
    git_ok(root, &["config", "commit.gpgsign", "false"]);
    fs::write(root.join("README.md"), "hello\n").expect("write README");
    git_ok(root, &["add", "."]);
    git_ok(root, &["commit", "-q", "-m", "initial"]);
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, the embedded pack-set.
fn run_jigc(repo: &Path, home: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("spawn the jigc binary")
}

/// Run `jigc <args>` and assert it exited 0, returning stdout.
fn run_jigc_ok(repo: &Path, home: &Path, args: &[&str], what: &str) -> String {
    let out = run_jigc(repo, home, args);
    assert!(
        out.status.success(),
        "{what} must exit 0; stdout:\n{}\nstderr:\n{}",
        stdout_of(&out),
        stderr_of(&out),
    );
    stdout_of(&out)
}

/// The task id a mint printed, read off the binary's `task minted: <id>` line — never
/// reconstructed from the intent (the slug rule is the binary's).
fn minted_task(stdout: &str) -> String {
    stdout
        .lines()
        .find_map(|line| line.strip_prefix("task minted: "))
        .unwrap_or_else(|| panic!("a mint must print `task minted: <id>`; got:\n{stdout}"))
        .trim()
        .to_string()
}

/// The production pack composition, built the **CWD-free** way (`registry_seam.rs`'s
/// idiom): `[dev ▸ methodology]`, dev highest-precedence.
fn composite() -> CompositePack {
    CompositePack::new(vec![
        Box::new(EmbeddedPack::new()),
        Box::new(EmbeddedPack::methodology()),
    ])
}

/// Every shipped doctype, keyed by id — the loaded schema registry both the census arm
/// and the write arms enumerate.
fn shipped_schemas() -> BTreeMap<String, Schema> {
    let pack = composite();
    let mut out = BTreeMap::new();
    for id in pack.list(PackResourceKind::Schemas) {
        let bytes = pack
            .read(PackResourceKind::Schemas, &id)
            .expect("a listed schema reads back");
        let schema = cli::pack::load_pack_schema(&pack, &bytes).expect("a shipped schema parses");
        out.insert(schema.ty.clone(), schema);
    }
    out
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 1 — the destroying-door × leftover-verdict matrix
// ═════════════════════════════════════════════════════════════════════════════

/// The bytes planted in every leftover — a refusal must leave them exactly this.
const PRECIOUS: &str = "precious, uncommitted, in no object DB\n";

/// One prepared cell: a repo standing in the shape that produces one
/// [`LeftoverVerdict`], with [`PRECIOUS`] planted at the `area-low` worktree path.
struct LeftoverCell {
    repo: PathBuf,
    home: PathBuf,
    leftover: PathBuf,
    planted: PathBuf,
    /// The temp roots this cell owns — dropped with it, never before.
    _keep: Vec<TempDir>,
}

/// Mint `milestone:cache-rework` with two sub-tasks — id-sorted `[area-low, area-zed]`,
/// so the leftover planted at `area-low` is the first path every door reaches.
fn mint_milestone(repo: &Path, home: &Path) {
    run_jigc_ok(
        repo,
        home,
        &["milestone", "create", "Cache rework"],
        "`jigc milestone create`",
    );
    for intent in ["Area zed", "Area low"] {
        run_jigc_ok(
            repo,
            home,
            &["milestone", "add-task", "cache-rework", intent],
            "`jigc milestone add-task`",
        );
    }
}

/// `cp -R <from> <to>` — the ordinary way a corpus copy is made (the trigger the charter
/// did not know about), reproduced verbatim rather than simulated.
fn copy_repo(from: &Path, to: &Path) {
    let out = Command::new("cp")
        .arg("-R")
        .arg(from)
        .arg(to)
        .output()
        .expect("run cp");
    assert!(
        out.status.success(),
        "cp -R {from:?} {to:?} failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
}

/// The `area-low` worktree path of `repo`, with [`PRECIOUS`] planted inside it.
fn plant_precious(repo: &Path) -> (PathBuf, PathBuf) {
    let leftover = repo.join(".jigc").join("worktrees").join("area-low");
    fs::create_dir_all(&leftover).expect("mk the leftover dir");
    let planted = leftover.join("precious.txt");
    fs::write(&planted, PRECIOUS).expect("plant precious.txt");
    (leftover, planted)
}

/// Build `count` ready cells in the real shape that yields `verdict` — one per door, so
/// the matrix's cells never share a repo.
///
/// The two copy-shaped verdicts build **one** provisioned source and `cp -R` it per cell
/// (a copy's worktree admin record names the *source's* path, so nothing under the copy's
/// own `.jigc/worktrees/` is registered there); [`LeftoverVerdict::Unverifiable`] then
/// moves that source away once, after all its copies exist, so `rev-parse` inside each
/// copy's worktree exits non-zero and git can say nothing at all.
fn plant_cells(verdict: LeftoverVerdict, count: usize) -> Vec<LeftoverCell> {
    let home = TempDir::new("home");
    let mut cells = Vec::new();
    match verdict {
        LeftoverVerdict::NoOwnLinkage => {
            // A plain directory with no `.git` of its own: git walks up and answers for
            // the enclosing repo, so nothing there vouches for these bytes.
            for index in 0..count {
                let root = TempDir::new(&format!("no-linkage-{index}"));
                init_repo(root.path());
                mint_milestone(root.path(), home.path());
                let (leftover, planted) = plant_precious(root.path());
                cells.push(LeftoverCell {
                    repo: root.path().to_path_buf(),
                    home: home.path().to_path_buf(),
                    leftover,
                    planted,
                    _keep: vec![root],
                });
            }
        }
        LeftoverVerdict::OwnWorktree | LeftoverVerdict::Unverifiable => {
            let source = TempDir::new("source");
            init_repo(source.path());
            mint_milestone(source.path(), home.path());
            run_jigc_ok(
                source.path(),
                home.path(),
                &["milestone", "provision", "cache-rework"],
                "the source's first `jigc milestone provision`",
            );
            let copies = TempDir::new("copies");
            for index in 0..count {
                let repo = copies.path().join(format!("copy-{index}"));
                copy_repo(source.path(), &repo);
                let (leftover, planted) = plant_precious(&repo);
                cells.push(LeftoverCell {
                    repo,
                    home: home.path().to_path_buf(),
                    leftover,
                    planted,
                    _keep: Vec::new(),
                });
            }
            let mut keep = vec![copies, home];
            if verdict == LeftoverVerdict::Unverifiable {
                let attic = TempDir::new("attic");
                fs::rename(source.path(), attic.path().join("moved-source"))
                    .expect("move the source repo away");
                keep.push(attic);
            }
            keep.push(source);
            // The shared roots ride the first cell, which outlives the loop below.
            if let Some(first) = cells.first_mut() {
                first._keep.append(&mut keep);
            }
            return cells;
        }
    }
    if let Some(first) = cells.first_mut() {
        first._keep.push(home);
    }
    cells
}

/// The argv that drives one destroying door at the planted milestone, and what must be
/// true of the repo once `--force` has been given — the door's **own** post-condition,
/// since the three teardowns do genuinely different things with consent (`provision`
/// clears and re-provisions the path, `uninstall` removes `.jigc/` whole, and `discard`
/// removes the worktrees this repo **registered** and leaves every other path on disk).
fn door_drive(door: &DestroyingDoor) -> (Vec<String>, fn(&LeftoverCell)) {
    match door.verb {
        "jigc milestone provision" => {
            (owned(&["milestone", "provision", "cache-rework"]), |cell| {
                assert!(
                    cell.leftover.join(".git").is_file(),
                    "[provision --force] a fresh linked worktree must stand at the cleared path",
                );
                assert!(
                    !cell.planted.exists(),
                    "[provision --force] `--force` is the consent to delete the leftover",
                );
            })
        }
        "jigc milestone discard" => (owned(&["milestone", "discard", "cache-rework"]), |cell| {
            // The abandon is committed, so what changed is the **record**, not the disk: an
            // *unregistered* path is deliberately left standing (`remove_worktrees`'
            // contract), which is exactly why the refusal's abandon arm is conditional on
            // registration. So the post-condition is asked of the milestone itself: the
            // abandon settled it, so no live milestone answers to that id any more.
            let again = run_jigc(
                &cell.repo,
                &cell.home,
                &["milestone", "list-tasks", "cache-rework"],
            );
            assert!(
                !again.status.success(),
                "[discard --force] the abandon must have settled the milestone — no live \
                 milestone answers to `cache-rework`; got:\n{}",
                printed(&again),
            );
        }),
        "jigc uninstall" => (owned(&["uninstall"]), |cell| {
            assert!(
                !cell.repo.join(".jigc").exists(),
                "[uninstall --force] the teardown removes `.jigc/` whole",
            );
        }),
        other => panic!(
            "`{other}` is a destroying door with no fixture in this flow — the axis is the \
             code-side `DESTROYING_DOORS` table, so a door added there owes its cells here",
        ),
    }
}

/// **Arm 1.** No shipped door destroys uncommitted work at exit 0 — over the whole
/// **door × leftover-verdict** matrix, plus the authored-prose cell `uninstall` owns
/// alone.
///
/// The axis is two code-side tables minted beside the classifier they describe
/// ([`DESTROYING_DOORS`] × [`LEFTOVER_VERDICTS`]): a fourth verdict cannot be added
/// without this arm failing to compile, and a fourth destroying door is a hard panic
/// here rather than a silent gap. Per cell, through the real binary: the door **refuses**
/// non-zero carrying *its own* blocking code, names the path and what is in it, routes at
/// the consent flag, and leaves the planted bytes **byte-intact** — then `--force` is
/// driven and the door proceeds, so consent is proven to be the only way past rather than
/// assumed.
///
/// Red on rc.10: `milestone provision` deleted a non-registered leftover's staged,
/// unstaged and untracked work at **exit 0** — the registered set is blind to a `cp -R`
/// of the repo, which is how every trial corpus is made — and `uninstall` destroyed a
/// task's authored prose on the same footing.
#[test]
fn no_destroying_door_takes_work_it_cannot_prove_is_junk() {
    assert_eq!(
        DESTROYING_DOORS.len(),
        3,
        "the axis is the code-side destroying-door table",
    );
    let codes: BTreeSet<&str> = DESTROYING_DOORS.iter().map(|door| door.code).collect();
    assert_eq!(
        codes.len(),
        DESTROYING_DOORS.len(),
        "one blocking code per door — a shared identity makes a refusal unattributable",
    );

    for verdict in LEFTOVER_VERDICTS {
        let mut cells = plant_cells(verdict, DESTROYING_DOORS.len());
        for (door, cell) in DESTROYING_DOORS.iter().zip(cells.iter_mut()) {
            let verb = door.verb;
            let (driven, forced_postcondition) = door_drive(door);
            let argv: Vec<&str> = driven.iter().map(String::as_str).collect();

            let refused = run_jigc(&cell.repo, &cell.home, &argv);
            let text = printed(&refused);
            assert!(
                !refused.status.success(),
                "[{verb} · {verdict:?}] the door must REFUSE a leftover it cannot prove is \
                 junk; got:\n{text}",
            );
            assert!(
                text.contains(door.code),
                "[{verb} · {verdict:?}] the refusal must carry THIS door's code `{}`; \
                 got:\n{text}",
                door.code,
            );
            assert!(
                text.contains("precious.txt"),
                "[{verb} · {verdict:?}] the refusal must name what would be destroyed; \
                 got:\n{text}",
            );
            assert!(
                text.contains("--force"),
                "[{verb} · {verdict:?}] the refusal must route at the consent flag; \
                 got:\n{text}",
            );
            assert_eq!(
                fs::read_to_string(&cell.planted).expect("the planted file survives the refusal"),
                PRECIOUS,
                "[{verb} · {verdict:?}] the planted bytes must survive byte-intact",
            );

            // Consent, driven — the only way past.
            let mut forced = driven.clone();
            forced.push("--force".to_string());
            let argv: Vec<&str> = forced.iter().map(String::as_str).collect();
            let out = run_jigc(&cell.repo, &cell.home, &argv);
            assert!(
                out.status.success(),
                "[{verb} · {verdict:?}] `--force` is the operator's consent — the door must \
                 proceed at exit 0; got:\n{}",
                printed(&out),
            );
            forced_postcondition(cell);
        }
    }
}

/// **Arm 1's second cell** — the authored prose only `uninstall` can reach: bytes in no
/// object DB at all, under `.jigc/tasks/<id>/docs/`, which the teardown used to take at
/// exit 0.
#[test]
fn uninstall_refuses_the_authored_prose_that_lives_nowhere_else() {
    let repo = TempDir::new("prose-repo");
    let home = TempDir::new("prose-home");
    init_repo(repo.path());
    run_jigc_ok(repo.path(), home.path(), &["setup"], "`jigc setup`");
    let task = minted_task(&run_jigc_ok(
        repo.path(),
        home.path(),
        &["start", "--workflow", "record-decision", "settle-the-cache"],
        "`jigc start`",
    ));
    let created = run_jigc_ok(
        repo.path(),
        home.path(),
        &[
            "doc",
            "create",
            "adr",
            "--title",
            "Cache strategy",
            "--task",
            &task,
        ],
        "`jigc doc create adr`",
    );
    let address = created.trim().to_string();
    let staged = repo
        .path()
        .join(".jigc")
        .join("tasks")
        .join(&task)
        .join("docs")
        .join(format!("{address}.md"));
    assert!(
        staged.is_file(),
        "the staged prose must exist at {staged:?} for the guard to have a subject",
    );
    let before = fs::read(&staged).expect("read the staged doc");

    let refused = run_jigc(repo.path(), home.path(), &["uninstall"]);
    let text = printed(&refused);
    assert!(
        !refused.status.success(),
        "`jigc uninstall` must refuse while a task's staged prose lives only here; got:\n{text}",
    );
    assert!(
        text.contains("uninstall.staged-prose"),
        "the prose refusal takes its own identity — the worktree code names the wrong \
         subject; got:\n{text}",
    );
    assert!(
        text.contains("--force"),
        "the refusal must route at the consent flag; got:\n{text}",
    );
    assert_eq!(
        fs::read(&staged).expect("the staged prose survives the refusal"),
        before,
        "the authored prose must survive byte-intact",
    );

    let forced = run_jigc(repo.path(), home.path(), &["uninstall", "--force"]);
    assert!(
        forced.status.success(),
        "`--force` is the consent to delete — the teardown must proceed; got:\n{}",
        printed(&forced),
    );
    assert!(
        !repo.path().join(".jigc").exists(),
        "the forced teardown removes `.jigc/` whole",
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 2 — the read-back: every write-soliciting step names it, and the read runs
// ═════════════════════════════════════════════════════════════════════════════

/// One loaded pack's steps, as `(id, def)` pairs sorted by id — read from the **loaded
/// registry** (the composition the binary itself resolves), never off a directory walk.
fn loaded_steps(pack: &dyn PackSource) -> Vec<(String, engine::compose::StepDef)> {
    let mut ids = pack.list(PackResourceKind::Steps);
    ids.sort();
    ids.into_iter()
        .map(|id| {
            let bytes = pack
                .read(PackResourceKind::Steps, &id)
                .expect("a listed step reads back");
            let def = engine::compose::load_step_def(id.as_str(), &bytes)
                .expect("a shipped step's front-matter parses");
            (id.as_str().to_string(), def)
        })
        .collect()
}

/// The pack's catalog ids whose command-ref is a `jigc doc <write-verb>` call — the
/// write half of the read-back derivation. The write-verb partition is the **production**
/// one ([`cli::doc::doc_write_verbs`]: the clap `doc` leaf set minus the declared read
/// verbs), so a new `doc` write verb widens this set without a second hand list.
fn doc_write_command_ids(pack: &dyn PackSource) -> BTreeSet<String> {
    let bytes = pack
        .read(
            PackResourceKind::Config,
            &engine::packsource::ResourceId::from("commands"),
        )
        .expect("a shipped pack carries a command catalog");
    let catalog = engine::compose::load_command_catalog(&bytes).expect("the catalog parses");
    let write_verbs = cli::doc::doc_write_verbs();
    catalog
        .commands
        .iter()
        .filter(|(_, command)| {
            if command.command != "jigc" {
                return false;
            }
            let mut literals = command.args.iter().filter_map(|arg| match arg {
                engine::compose::CommandArg::Literal { literal } => Some(literal.as_str()),
                _ => None,
            });
            literals.next() == Some("doc")
                && literals
                    .next()
                    .is_some_and(|verb| write_verbs.iter().any(|write| write == verb))
        })
        .map(|(id, _)| id.clone())
        .collect()
}

/// Every lone-line `{{ cli.<id> }}` ref of a step body — the compose seam's own class
/// rule (a `{{cli.…}}` renders only as a whole line).
fn cli_refs(body: &str) -> Vec<String> {
    body.lines()
        .filter_map(|line| {
            let inner = line.trim().strip_prefix("{{")?.strip_suffix("}}")?.trim();
            let id = inner.strip_prefix("cli.")?.trim();
            (!id.is_empty() && !id.contains(char::is_whitespace)).then(|| id.to_owned())
        })
        .collect()
}

/// Whether a step body carries a `{{schema:<T>}}` authoring-payload ref — the
/// derivation's second signal (a migrate author template solicits its whole write
/// through the projection and carries no `{{cli.<id>}}` ref at all).
fn has_schema_ref(body: &str) -> bool {
    let mut rest = body;
    while let Some(open) = rest.find("{{") {
        rest = &rest[open + 2..];
        let Some(close) = rest.find("}}") else { break };
        if rest[..close].trim().starts_with("schema:") {
            return true;
        }
        rest = &rest[close + 2..];
    }
    false
}

/// The pack's **owe-set** (steps that solicit a managed-doc write) and its
/// **declarer set** (steps declaring [`cli::pack::STAGED_READ_BACK_CODE`]).
fn owe_and_declarers(pack: &dyn PackSource) -> (BTreeSet<String>, BTreeSet<String>) {
    let writes = doc_write_command_ids(pack);
    let mut owed = BTreeSet::new();
    let mut declared = BTreeSet::new();
    for (id, def) in loaded_steps(pack) {
        if cli_refs(&def.body).iter().any(|r| writes.contains(r)) || has_schema_ref(&def.body) {
            owed.insert(id.clone());
        }
        if def
            .states_constraints
            .iter()
            .any(|code| code == cli::pack::STAGED_READ_BACK_CODE)
        {
            declared.insert(id);
        }
    }
    (owed, declared)
}

/// **Arm 2.** An authoring step cannot ship without naming the read-back — and the read
/// it names **runs**, serving the staged write the committed store does not carry.
///
/// The mechanism behind six consecutive trials was that no soliciting surface named
/// `jigc doc show … --task <id>`, so this arm enumerates the owe-set the way the fence
/// does — **re-derived from both loaded packs** (`{{cli.<id>}}` resolving to a
/// [`cli::doc::doc_write_verbs`] catalog entry ∪ `{{schema:<T>}}`), never hand-listed —
/// and then walks the done picture: every composed workflow of either pack that includes
/// a declaring step prints the read-back line, and in a real task the line is **lifted
/// verbatim out of the composition and executed**, against a write the committed store
/// cannot answer.
///
/// The fence's own redness under applied mutation is `read_back_fence.rs`'s; this arm
/// proves the shipped composition tells the agent, and that what it tells them works.
///
/// Red on rc.10: exactly one shipped step file named `doc show` at all, and it was a
/// committed-doc id lookup rather than a read-back of in-flight work.
#[test]
fn every_write_soliciting_step_names_the_read_back_and_the_read_runs() {
    // ── the derivation, over both loaded packs ──
    let packs: Vec<(&str, Box<dyn PackSource>)> = vec![
        ("dev", Box::new(EmbeddedPack::new())),
        ("methodology", Box::new(EmbeddedPack::methodology())),
    ];
    let mut declaring_steps: BTreeSet<String> = BTreeSet::new();
    for (name, pack) in &packs {
        let (owed, declared) = owe_and_declarers(pack.as_ref());
        assert!(
            !owed.is_empty(),
            "the `{name}` pack must solicit at least one managed-doc write for the axis to \
             sweep",
        );
        assert_eq!(
            owed,
            declared,
            "every `{name}` step that solicits a managed-doc write must declare `{}` — and \
             only those",
            cli::pack::STAGED_READ_BACK_CODE,
        );
        for (id, def) in loaded_steps(pack.as_ref()) {
            if declared.contains(&id) {
                assert!(
                    def.body.contains("jigc doc show") && def.body.contains("--task"),
                    "the `{name}` step `{id}` declares the read-back but its prose never \
                     names the addressed staged read; got:\n{}",
                    def.body,
                );
                declaring_steps.insert(id);
            }
        }
    }
    assert!(
        declaring_steps.len() >= 25,
        "the owe-set spans both packs' authoring surface; got {} steps",
        declaring_steps.len(),
    );

    // ── the composed surface: every workflow including a declarer prints the line ──
    let corpus = TrialCorpus::build(State::Fresh);
    let mut composed_with_declarer = 0usize;
    for (name, pack) in &packs {
        for id in pack.list(PackResourceKind::Workflows) {
            let bytes = pack
                .read(PackResourceKind::Workflows, &id)
                .expect("a listed workflow reads back");
            let def =
                engine::compose::load_workflow_def(&bytes).expect("a shipped workflow parses");
            let includes_declarer = def
                .includes
                .iter()
                .any(|step| declaring_steps.contains(step.as_str()));
            if !includes_declarer {
                continue;
            }
            composed_with_declarer += 1;
            let preview = corpus.jigc(&["workflow", id.as_str(), "--preview"]);
            assert!(
                preview.status.success(),
                "[{name}:{}] the preview must compose; got:\n{}",
                id.as_str(),
                printed(&preview),
            );
            let text = stdout_of(&preview);
            assert!(
                text.contains("jigc doc show") && text.contains("--task"),
                "[{name}:{}] a composition including a write-soliciting step must print the \
                 addressed staged read; got:\n{text}",
                id.as_str(),
            );
        }
    }
    assert!(
        composed_with_declarer >= 5,
        "the composed surface must reach several declaring workflows; got \
         {composed_with_declarer}",
    );

    // ── the done picture: the composed line, lifted verbatim and RUN ──
    let task = corpus.start_workflow("record-decision", "settle-the-cache-strategy");
    let resumed = corpus.jigc_ok(&["start", "--task", &task]);
    let line = resumed
        .lines()
        .find(|line| line.trim_start().starts_with("jigc doc show adr:"))
        .unwrap_or_else(|| {
            panic!("the composed `record-decision` must name the staged read; got:\n{resumed}")
        })
        .trim()
        .to_string();
    assert!(
        line.contains(&format!("--task {task}")),
        "the composed line must carry THIS task's id, resolved — never `{{{{task.id}}}}`; \
         got: {line}",
    );

    let address = corpus
        .jigc_ok(&[
            "doc",
            "create",
            "adr",
            "--title",
            "Cache strategy",
            "--task",
            &task,
        ])
        .trim()
        .to_string();
    corpus.jigc_stdin_ok(
        &[
            "doc",
            "set-slot",
            &format!("{address}#decision"),
            "--from-file",
            "-",
            "--task",
            &task,
        ],
        "Keep sessions in one node.\n",
    );

    // Everything but the author-supplied `<slug>` placeholder is the emitted bytes: the
    // slug is the agent's to choose, and `create` printed the identity it minted.
    let slug = address
        .split_once(':')
        .expect("the emitted address is `<type>:<slug>`")
        .1;
    let driven = line.replace("adr:<slug>", &format!("adr:{slug}"));
    let argv: Vec<&str> = driven.split_whitespace().skip(1).collect();
    let served = corpus.jigc(&argv);
    assert!(
        served.status.success(),
        "the composed read-back `{driven}` must run; got:\n{}",
        printed(&served),
    );
    assert!(
        stdout_of(&served).contains("Keep sessions in one node."),
        "the read-back must serve the STAGED write; got:\n{}",
        stdout_of(&served),
    );

    // And the claim that makes it worth naming: the committed store cannot answer it.
    let committed = corpus.jigc(&["doc", "show", &format!("adr:{slug}")]);
    assert!(
        !committed.status.success(),
        "the committed store must not carry the staged write — that is why the step names \
         the `--task` read; got:\n{}",
        printed(&committed),
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 3 — the doctype census × both minting verbs
// ═════════════════════════════════════════════════════════════════════════════

/// Which identity cell a doctype falls in — the three code-side predicates the write
/// path branches on, in the order it evaluates them (`doc_rename_in_task.rs`'s census,
/// re-derived here from the same loaded registry).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum IdentityCell {
    /// `Schema::fixed_title().is_some()` — the `singleton:` set: the `# H1` is the
    /// schema's, so an author-supplied title can only be dropped.
    Fixed,
    /// `location.is_some()` — the slug-identity set: the title mints the slug.
    Slug,
    /// Neither — a transient sink whose slug is the task id (`commit:<task-id>`).
    Transient,
}

/// The whole loaded doctype registry, partitioned into its identity cells.
fn identity_census() -> BTreeMap<String, (Schema, IdentityCell)> {
    shipped_schemas()
        .into_iter()
        .map(|(ty, schema)| {
            let cell = if schema.fixed_title().is_some() {
                IdentityCell::Fixed
            } else if schema.location.is_some() {
                IdentityCell::Slug
            } else {
                IdentityCell::Transient
            };
            (ty, (schema, cell))
        })
        .collect()
}

/// A workflow whose create-gate admits `doctype`, read from the loaded workflow registry
/// — a directly-startable one is preferred, since a `migrate-*` workflow is minted by
/// `jigc migrate <path>` rather than by `jigc start`.
fn admitting_workflow(doctype: &str) -> String {
    let pack = composite();
    let mut admitting: Vec<String> = pack
        .list(PackResourceKind::Workflows)
        .iter()
        .filter_map(|id| {
            let bytes = pack.read(PackResourceKind::Workflows, id).ok()?;
            let def = engine::compose::load_workflow_def(&bytes).ok()?;
            def.allows_create
                .iter()
                .any(|entry| entry.doc_type == doctype)
                .then(|| id.as_str().to_string())
        })
        .collect();
    admitting.sort_by_key(|id| (id.starts_with("migrate-"), id.clone()));
    admitting
        .into_iter()
        .next()
        .unwrap_or_else(|| panic!("no shipped workflow admits `{doctype}`"))
}

/// The first blocking finding of a `--format json` reject, parsed off stderr.
fn blocking_finding(out: &Output, what: &str) -> Value {
    let stderr = stderr_of(out);
    let report: Value = serde_json::from_str(stderr.trim())
        .unwrap_or_else(|e| panic!("`{what}`: stderr must be one JSON report ({e}):\n{stderr}"));
    report["findings"][0].clone()
}

/// The leading backticked command of a route string (`` `<cmd>`<tail> ``).
fn backticked(route: &str, what: &str) -> String {
    route
        .strip_prefix('`')
        .and_then(|rest| rest.split('`').next())
        .unwrap_or_else(|| panic!("`{what}`: the route carries a backticked command; got: {route}"))
        .to_string()
}

/// Run a `--format json` invocation against a corpus.
fn json_run(corpus: &TrialCorpus, args: &[&str], stdin: Option<&str>) -> Output {
    let mut argv = vec!["--format", "json"];
    argv.extend_from_slice(args);
    match stdin {
        Some(payload) => corpus.jigc_stdin(&argv, payload),
        None => corpus.jigc(&argv),
    }
}

/// The `adr` authoring payload the arm feeds `doc author`, titled `title`.
fn adr_payload(title: &str) -> String {
    format!(
        "title: {title}\n\
         sections:\n\
         \x20 - id: context\n\
         \x20   set:\n\
         \x20     context: |-\n\
         \x20       <<Sessions must survive a node restart.>>\n\
         \x20 - id: decision\n\
         \x20   set:\n\
         \x20     decision: |-\n\
         \x20       <<Keep sessions in one node.>>\n\
         \x20 - id: consequences\n\
         \x20   set:\n\
         \x20     consequences: |-\n\
         \x20       <<A cold node loses its sessions.>>\n"
    )
}

/// **Arm 3.** No minting verb acks success over a title it dropped or diverted — and the
/// correction it refuses has somewhere to go, driven to a **landed** in-task title change.
///
/// The axis is the **doctype census**, re-derived from the loaded schema registry rather
/// than hand-listed: every shipped doctype falls in exactly one identity cell, all three
/// cells are populated, and each is driven at the verbs that can mint it. The
/// fixed-identity set is swept at `doc create` over the whole census (a singleton added
/// to either pack joins the sweep); the slug-identity cell is driven at **both** minting
/// verbs, in both divergence shapes — the same-identity drop (`write.title-ignored`) and
/// the different-identity mint (`write.identity-change`) — with the emitted route lifted
/// out of the JSON finding, **run verbatim**, and the resulting title read back off the
/// staged doc.
///
/// Red on rc.10: a re-`create`/`author` under a corrected title acked success at exit 0
/// and committed **two** ADRs — the correction became a second document nobody authored.
#[test]
fn no_minting_verb_acks_a_title_it_dropped_or_diverted() {
    // ── the census partitions the whole registry ──
    let census = identity_census();
    let pack = composite();
    let registry: BTreeSet<String> = pack
        .list(PackResourceKind::Schemas)
        .iter()
        .map(|id| id.as_str().to_string())
        .collect();
    assert_eq!(
        census.keys().cloned().collect::<BTreeSet<String>>(),
        registry,
        "every listed doctype must land in exactly one identity cell",
    );
    for cell in [
        IdentityCell::Fixed,
        IdentityCell::Slug,
        IdentityCell::Transient,
    ] {
        assert!(
            census.values().any(|(_, c)| *c == cell),
            "the {cell:?} cell must be populated — an empty cell means its sweep proves \
             nothing",
        );
    }

    // ── the fixed-identity cell, over the whole census: a dropped title is refused ──
    let base = TrialCorpus::build(State::Fresh);
    let fixed: Vec<(String, String)> = census
        .iter()
        .filter(|(_, (_, cell))| *cell == IdentityCell::Fixed)
        .map(|(ty, (schema, _))| {
            (
                ty.clone(),
                schema.fixed_title().expect("a fixed cell has a title"),
            )
        })
        .collect();
    for (ty, title) in &fixed {
        let corpus = base.copy_state();
        let task = corpus.start_workflow(&admitting_workflow(ty), "poke the singleton title");
        let divergent = format!("{title} of ours");
        let out = json_run(
            &corpus,
            &["doc", "create", ty, "--title", &divergent, "--task", &task],
            None,
        );
        assert!(
            !out.status.success(),
            "[{ty}] a title the doc will never carry must block, not ack; got:\n{}",
            printed(&out),
        );
        let finding = blocking_finding(&out, ty);
        assert_eq!(
            finding["code"],
            "write.title-ignored",
            "[{ty}] the refusal names the dropped title, not an identity change; got:\n{}",
            stderr_of(&out),
        );
        // The complement, in the same loop: the guard refuses the DROPPED title, never
        // the doctype — the title the doc will actually carry still lands.
        assert!(
            corpus
                .jigc(&["doc", "create", ty, "--title", title, "--task", &task])
                .status
                .success(),
            "[{ty}] the title the doc WILL carry must still land",
        );
    }

    // ── the transient cell: a sink whose slug is the task id is mintable by no verb ──
    let sinks = base.copy_state();
    let sink_task = sinks.start_workflow("single-task", "poke the transient sink");
    for (ty, _) in census
        .iter()
        .filter(|(_, (_, cell))| *cell == IdentityCell::Transient)
    {
        let out = sinks.jigc(&[
            "doc",
            "create",
            ty,
            "--title",
            "A title of ours",
            "--task",
            &sink_task,
        ]);
        assert!(
            !out.status.success(),
            "[{ty}] a transient sink's identity is the task's — no author-supplied title \
             may mint one; got:\n{}",
            printed(&out),
        );
    }

    // ── the slug-identity cell: both minting verbs, both divergence shapes ──
    let corpus = base.copy_state();
    let task = corpus.start_workflow("record-decision", "settle-the-cache-store");
    let address = corpus
        .jigc_ok(&[
            "doc",
            "create",
            "adr",
            "--title",
            "Adopt Redis",
            "--task",
            &task,
        ])
        .trim()
        .to_string();
    let staged_dir = corpus.repo().join(".jigc").join("tasks").join(&task);
    let docs_before = staged_docs(&staged_dir);

    // (a) the same-identity drop, at BOTH minting verbs.
    for (verb, out) in [
        (
            "doc create",
            json_run(
                &corpus,
                &[
                    "doc",
                    "create",
                    "adr",
                    "--title",
                    "Adopt Redis!",
                    "--task",
                    &task,
                ],
                None,
            ),
        ),
        (
            "doc author",
            json_run(
                &corpus,
                &["doc", "author", "adr", "--from-file", "-", "--task", &task],
                Some(&adr_payload("Adopt Redis!")),
            ),
        ),
    ] {
        assert!(
            !out.status.success(),
            "[{verb}] a title that slugs to the same identity but will never be written \
             must block; got:\n{}",
            printed(&out),
        );
        assert_eq!(
            blocking_finding(&out, verb)["code"],
            "write.title-ignored",
            "[{verb}] the same-identity drop is not an identity change; got:\n{}",
            stderr_of(&out),
        );
    }

    // (b) the different-identity mint, at BOTH minting verbs — the cell that committed
    //     two ADRs.
    let mut routes = Vec::new();
    for (verb, out) in [
        (
            "doc create",
            json_run(
                &corpus,
                &[
                    "doc",
                    "create",
                    "adr",
                    "--title",
                    "Adopt Valkey",
                    "--task",
                    &task,
                ],
                None,
            ),
        ),
        (
            "doc author",
            json_run(
                &corpus,
                &["doc", "author", "adr", "--from-file", "-", "--task", &task],
                Some(&adr_payload("Adopt Valkey")),
            ),
        ),
    ] {
        assert!(
            !out.status.success(),
            "[{verb}] a title that mints a DIFFERENT identity must block; got:\n{}",
            printed(&out),
        );
        let finding = blocking_finding(&out, verb);
        assert_eq!(
            finding["code"],
            "write.identity-change",
            "[{verb}] a diverted title is an identity change; got:\n{}",
            stderr_of(&out),
        );
        routes.push(
            finding["route"]
                .as_str()
                .unwrap_or_else(|| panic!("[{verb}] a blocking finding carries a route"))
                .to_string(),
        );
    }

    // Nothing was minted and nothing was written by any of the four rejects.
    assert_eq!(
        staged_docs(&staged_dir),
        docs_before,
        "a refused write persists nothing — no second doc, no rewritten bytes",
    );

    // (c) the route, lifted out of the emitted finding and RUN — the correction lands.
    let printed_route = backticked(&routes[0], "the identity-change route");
    let argv = support::shell_words(&printed_route, &corpus.repo(), &corpus.home());
    assert_eq!(
        argv.first().map(String::as_str),
        Some("jigc"),
        "the route must be a runnable `jigc …` command; got `{printed_route}`",
    );
    let borrowed: Vec<&str> = argv[1..].iter().map(String::as_str).collect();
    let landed = corpus.jigc(&borrowed);
    assert!(
        landed.status.success(),
        "the emitted route `{printed_route}` must land the correction; got:\n{}",
        printed(&landed),
    );
    let shown = corpus.jigc_ok(&["doc", "show", "adr:adopt-valkey", "--task", &task]);
    assert!(
        shown.contains("# Adopt Valkey"),
        "the corrected title must be the staged doc's own H1; got:\n{shown}",
    );
    assert!(
        !corpus
            .jigc(&["doc", "show", &address, "--task", &task])
            .status
            .success(),
        "the correction MOVED the doc — the old identity must be gone, not duplicated",
    );
    let after = staged_docs(&staged_dir);
    assert_eq!(
        after.len(),
        docs_before.len(),
        "the correction must leave the staged doc COUNT unchanged; before {docs_before:?}, \
         after {after:?}",
    );
}

/// The task working area's staged doc set, as `(file name, bytes)` — the byte-level
/// witness that a refused write persisted nothing.
fn staged_docs(task_dir: &Path) -> BTreeMap<String, Vec<u8>> {
    let docs = task_dir.join("docs");
    let mut out = BTreeMap::new();
    let Ok(entries) = fs::read_dir(&docs) else {
        return out;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "md") {
            out.insert(
                entry.file_name().to_string_lossy().into_owned(),
                fs::read(&path).expect("read a staged doc"),
            );
        }
    }
    out
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 4 — one file, one code, one route at every door · and the install commit
// ═════════════════════════════════════════════════════════════════════════════

/// The **foreign** cell: a freeform file committed in git at a managed home, which jigc
/// never wrote — the adoption case.
const FOREIGN_PATH: &str = "docs/decisions/notes.md";
const FOREIGN_BODY: &str = "# scratch notes\n\nrandom thoughts, not an ADR\n";

/// The **managed at-version** cell: a v2-stamped ADR whose `## Consequences` is gone, so
/// it fails to parse under the shape this binary knows. Nothing to migrate — the damage is
/// out-of-band, on jigc's own doc.
const AT_VERSION_PATH: &str = "docs/decisions/cache-sessions-in-memory.md";
const AT_VERSION_BODY: &str = "\
---
status: accepted
date: 2026-06-25
schema-version: 2
---

# Cache sessions in memory

## Context

Session lookups must stay sub-millisecond.

## Options

A distributed cache was weighed and rejected on latency.

## Decision

Keep sessions in a single in-memory node.
";

/// The **managed below-version** cell: the shipped prior (v1) shape, stamped 1 while the
/// `adr` manifest is at 2 — the commonest stale doc in a real corpus, and the one whose
/// hand-repair would be repairing what `migrate-corpus` must rewrite.
const BELOW_VERSION_PATH: &str = "docs/decisions/queue-writes-behind-a-buffer.md";
const BELOW_VERSION_BODY: &str = "\
---
status: accepted
date: 2026-06-25
schema-version: 1
---

# Queue writes behind a buffer

## Context

Write bursts overwhelm the primary.

## Decision

Buffer writes and drain them on a timer.

## Consequences

A crash loses the un-drained tail.
";

/// The discriminator's own cells — the class axis this arm iterates, matched exhaustively
/// so a fourth adjudication cannot be added without a cell here.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FileClass {
    /// Committed at a managed home, never written by jigc.
    Foreign,
    /// jigc's own doc, at the version this binary knows, not conforming.
    ManagedAtVersion,
    /// jigc's own doc, below the manifest version (or unstamped).
    ManagedBelowVersion,
}

impl FileClass {
    fn path(self) -> &'static str {
        match self {
            FileClass::Foreign => FOREIGN_PATH,
            FileClass::ManagedAtVersion => AT_VERSION_PATH,
            FileClass::ManagedBelowVersion => BELOW_VERSION_PATH,
        }
    }

    fn body(self) -> &'static str {
        match self {
            FileClass::Foreign => FOREIGN_BODY,
            FileClass::ManagedAtVersion => AT_VERSION_BODY,
            FileClass::ManagedBelowVersion => BELOW_VERSION_BODY,
        }
    }

    /// The code this class answers with at the **task-scope** door — the reconciler's own
    /// adjudication of a committed file, and the seam M42's sweep never reached.
    fn code(self) -> &'static str {
        match self {
            FileClass::Foreign => UNADOPTED_CODE,
            FileClass::ManagedAtVersion | FileClass::ManagedBelowVersion => CONFORMANCE_BLOCK_CODE,
        }
    }

    /// A phrase the route must carry, and one it must not — the split the wave settled:
    /// adoption advice only where adoption is the answer, migration only where a
    /// migration exists, hand-repair only where the file really is the operator's.
    fn route_says(self) -> (&'static str, &'static str) {
        match self {
            FileClass::Foreign => ("jigc ingest", "yours to hand-edit"),
            FileClass::ManagedAtVersion => ("yours to hand-edit", "jigc ingest"),
            FileClass::ManagedBelowVersion => ("run the corpus migration", "jigc ingest"),
        }
    }
}

/// The three classes, as the arm iterates them.
const FILE_CLASSES: [FileClass; 3] = [
    FileClass::Foreign,
    FileClass::ManagedAtVersion,
    FileClass::ManagedBelowVersion,
];

/// The adoption advisory a **foreign** squatter draws — the one code the discriminator
/// may hand a file jigc never wrote.
const UNADOPTED_CODE: &str = "schema-conformance.unadopted-instance";

/// The reconciler's block over a **managed** committed doc that does not conform.
const CONFORMANCE_BLOCK_CODE: &str = "reconciliation.conformance-block";

/// All findings of a `--format json` envelope (stdout when the run succeeded, stderr when
/// it was refused).
fn envelope_findings(out: &Output, what: &str) -> Vec<Value> {
    let stdout = stdout_of(out);
    let body = if stdout.trim().is_empty() {
        stderr_of(out)
    } else {
        stdout
    };
    let value: Value = serde_json::from_str(body.trim())
        .unwrap_or_else(|e| panic!("`{what}` must emit one JSON document ({e}):\n{body}"));
    value["findings"].as_array().cloned().unwrap_or_default()
}

/// The findings at `(code, target)` — the wave's stable, discriminating finding key.
fn at_key<'a>(findings: &'a [Value], code: &str, target: &str) -> Vec<&'a Value> {
    findings
        .iter()
        .filter(|f| {
            f["key"]["code"].as_str() == Some(code) && f["key"]["target"].as_str() == Some(target)
        })
        .collect()
}

/// The single finding at `(code, target)`, or a panic naming what was there instead.
fn one_at<'a>(findings: &'a [Value], code: &str, target: &str, door: &str) -> &'a Value {
    let hits = at_key(findings, code, target);
    assert_eq!(
        hits.len(),
        1,
        "[{door}] exactly one `{code}` at `{target}` — one file answers one code; \
         got:\n{findings:#?}",
    );
    hits[0]
}

/// **Arm 4.** One file answers **one** code and **one** route at every door — and the
/// install commit carries exactly what `setup` says it installed.
///
/// The class axis is the managed-vs-foreign discriminator's own three cells, matched
/// exhaustively: a **foreign** squatter converges on the adoption advisory with a
/// **byte-identical** route at the store sweep and at the task gate; a **managed
/// at-version** doc keeps the conformance block and is told the one thing that is true of
/// it (*this file is yours to hand-repair*); a **managed below-version** doc is routed at
/// the corpus migration rather than at adoption. All three stand in **one** repo, so the
/// split is proven per *file*, never per report.
///
/// Riding with it, the wave's other install-side truth: `jigc setup` commits the
/// `pre-commit` hook **iff** the hook is a working-tree file — driven over both producible
/// hooks-dir shapes (the default `.git/hooks`, where it is not committable, and an
/// in-worktree `core.hooksPath`, where it is).
///
/// Red on rc.10: the task-scope door graded every non-conformant committed file
/// `reconciliation.conformance-block`, foreign and managed alike, and the advisory told a
/// stamped managed doc to ingest or migrate *itself*; and the install commit's pathspec was
/// a hardcoded seven-path list that could not see an in-repo `core.hooksPath` hook at all.
#[test]
fn one_file_answers_one_code_at_every_door_and_the_install_commit_carries_its_hook() {
    let repo = TempDir::new("classes-repo");
    let home = TempDir::new("classes-home");
    init_repo(repo.path());
    run_jigc_ok(repo.path(), home.path(), &["setup"], "`jigc setup`");

    fs::create_dir_all(repo.path().join("docs").join("decisions")).expect("mk docs/decisions/");
    for class in FILE_CLASSES {
        fs::write(repo.path().join(class.path()), class.body()).expect("write the class fixture");
    }
    git_ok(repo.path(), &["add", "docs"]);
    git_ok(repo.path(), &["commit", "-q", "-m", "seed all three cells"]);

    // The two doors, one run each — every class is adjudicated in both reports.
    let store = run_jigc(repo.path(), home.path(), &["validate", "--format", "json"]);
    let store_findings = envelope_findings(&store, "jigc validate");
    let task = minted_task(&run_jigc_ok(
        repo.path(),
        home.path(),
        &["start", "--workflow", "single-task", "check-the-store"],
        "`jigc start`",
    ));
    let gate = run_jigc(
        repo.path(),
        home.path(),
        &["task", "validate", &task, "--format", "json"],
    );
    let gate_findings = envelope_findings(&gate, "jigc task validate");

    for class in FILE_CLASSES {
        let path = class.path();
        let code = class.code();
        let (must_say, must_not_say) = class.route_says();

        // The task-scope door: the class's own code, carrying the route that is true of
        // *this file* — the seam M42's managed-vs-foreign sweep reached in the store
        // family and never here.
        let gate_finding = one_at(&gate_findings, code, path, "task gate");
        let gate_route = gate_finding["route"]
            .as_str()
            .unwrap_or_else(|| panic!("[{class:?}] the task finding carries a route"));
        assert!(
            gate_route.contains(must_say) && !gate_route.contains(must_not_say),
            "[{class:?}] the route must say `{must_say}` and must not say \
             `{must_not_say}`; got:\n{gate_route}",
        );
        // One file, one code: no path answers another class's task-scope code.
        for other in FILE_CLASSES.iter().filter(|c| c.code() != code) {
            assert!(
                at_key(&gate_findings, other.code(), path).is_empty(),
                "[{class:?} · task gate] one file, one code — `{path}` must not also \
                 answer `{}`; got:\n{gate_findings:#?}",
                other.code(),
            );
        }

        // The store door, per class. The two doors adjudicate different questions — the
        // reconciler asks *"did this committed file drift from what jigc wrote?"*, the
        // store sweep runs the content families over the store — so the claim held across
        // them is the **discriminator's**: the foreign file draws the same advisory with a
        // **byte-identical** route at both, and a managed doc is never called foreign at
        // either.
        match class {
            FileClass::Foreign => {
                let store_route = one_at(&store_findings, code, path, "store sweep")["route"]
                    .as_str()
                    .unwrap_or_else(|| panic!("[{class:?}] the store finding carries a route"))
                    .to_string();
                assert_eq!(
                    store_route, gate_route,
                    "[{class:?}] the two doors must serve ONE route, byte for byte",
                );
            }
            FileClass::ManagedAtVersion => {
                assert!(
                    at_key(&store_findings, UNADOPTED_CODE, path).is_empty(),
                    "[{class:?}] a STAMPED managed doc is never adjudicated foreign; \
                     got:\n{store_findings:#?}",
                );
                assert!(
                    store_findings.iter().any(|f| {
                        f["key"]["code"]
                            .as_str()
                            .is_some_and(|code| code.starts_with("conformance."))
                            && f["key"]["target"]
                                .as_str()
                                .is_some_and(|t| t.starts_with("adr:cache-sessions-in-memory"))
                    }),
                    "[{class:?}] the store sweep still reports the real break on jigc's own \
                     doc; got:\n{store_findings:#?}",
                );
            }
            FileClass::ManagedBelowVersion => {
                assert!(
                    at_key(&store_findings, UNADOPTED_CODE, path).is_empty(),
                    "[{class:?}] a doc that parses against a shape jigc once shipped is \
                     managed, never foreign; got:\n{store_findings:#?}",
                );
                let stale = one_at(
                    &store_findings,
                    "schema-conformance.schema-version-current",
                    "adr:queue-writes-behind-a-buffer",
                    "store sweep",
                );
                assert_eq!(
                    stale["severity"].as_str(),
                    Some("blocking"),
                    "[{class:?}] an unmigrated managed corpus flips the store sweep's exit; \
                     got:\n{stale:#?}",
                );
                assert!(
                    !store.status.success(),
                    "[{class:?}] …and the run itself exits non-zero over it; got:\n{}",
                    printed(&store),
                );
            }
        }
    }

    // ── the install commit carries the hook iff the hook is a working-tree file ──
    for (shape, hooks_path) in [
        ("default .git/hooks", None),
        ("core.hooksPath", Some("my-hooks")),
    ] {
        let repo = TempDir::new("hooks-repo");
        let home = TempDir::new("hooks-home");
        init_repo(repo.path());
        if let Some(dir) = hooks_path {
            git_ok(repo.path(), &["config", "core.hooksPath", dir]);
        }
        let summary = run_jigc_ok(repo.path(), home.path(), &["setup"], "`jigc setup`");
        let hook = match hooks_path {
            Some(dir) => repo.path().join(dir).join("pre-commit"),
            None => repo.path().join(".git").join("hooks").join("pre-commit"),
        };
        assert!(
            hook.is_file(),
            "[{shape}] the install must write the hook git resolved, at {hook:?}; \
             summary:\n{summary}",
        );
        let committed = git_ok(repo.path(), &["show", "--name-only", "--format=", "HEAD"]);
        let carries_hook = committed
            .lines()
            .any(|line| line.trim() == format!("{}/pre-commit", hooks_path.unwrap_or_default()));
        assert_eq!(
            carries_hook,
            hooks_path.is_some(),
            "[{shape}] the install commit must carry the hook exactly when the hook is a \
             working-tree file; commit carried:\n{committed}",
        );
        // Nothing from inside git's own control dir is ever staged, on either shape.
        assert!(
            !committed.lines().any(|line| line.starts_with(".git/")),
            "[{shape}] no path inside `.git/` may ride the install commit; carried:\n{committed}",
        );
        // And no hook is left untracked in the working tree.
        let untracked = git_ok(repo.path(), &["status", "--porcelain"]);
        assert!(
            !untracked.contains("pre-commit"),
            "[{shape}] the install must leave no untracked hook behind; status:\n{untracked}",
        );
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 5 — read intents, cascade acks, and the store sweep's exit flips
// ═════════════════════════════════════════════════════════════════════════════

/// Every backticked `jigc …` command span of a printed tip, as argv paths with flags and
/// `<placeholder>` operands dropped — the verb path [`verb_kind`] classifies.
fn jigc_spans(tip: &str) -> Vec<Vec<String>> {
    let mut out = Vec::new();
    let mut rest = tip;
    while let Some(open) = rest.find('`') {
        rest = &rest[open + 1..];
        let Some(close) = rest.find('`') else { break };
        let span = &rest[..close];
        rest = &rest[close + 1..];
        let mut tokens = span.split_whitespace();
        if tokens.next() != Some("jigc") {
            continue;
        }
        let path: Vec<String> = tokens
            .take_while(|token| !token.starts_with('<') && !token.starts_with('-'))
            .map(str::to_string)
            .collect();
        if !path.is_empty() {
            out.push(path);
        }
    }
    out
}

/// The subcommand names a `tip: … 'a', 'b'` did-you-mean line quotes.
fn suggested_names(tip: &str) -> Vec<String> {
    tip.split('\'')
        .skip(1)
        .step_by(2)
        .map(str::to_string)
        .collect()
}

/// Whether `parent` has a real subcommand named `name` in the clap tree — so a read-intent
/// token that is a genuine verb under that parent (`jigc doc list`) is driven as itself
/// rather than as a miss.
fn parent_has_subcommand(parent: &[&str], name: &str) -> bool {
    let mut cmd = Cli::command();
    for token in parent {
        let Some(sub) = cmd.find_subcommand(token).cloned() else {
            return false;
        };
        cmd = sub;
    }
    cmd.find_subcommand(name).is_some()
}

/// Build a bare repo + `$HOME` for a one-shot `jigc config <verb>` drive.
fn config_repo(tag: &str) -> (TempDir, TempDir) {
    let repo = TempDir::new(tag);
    let home = TempDir::new(&format!("{tag}-home"));
    init_repo(repo.path());
    fs::create_dir_all(repo.path().join(".jigc").join("config"))
        .expect("create the project cascade layer");
    (repo, home)
}

/// The argv that drives one [`ConfigAck`] arm to a real success, and any stdin it needs —
/// matched on the arm's own verb, so a seventh authoring verb is a hard panic here.
fn config_drive(verb: &str, repo: &Path) -> (Vec<String>, Option<&'static str>) {
    match verb {
        "set" => (owned(&["config", "set", "docs-root", "docs"]), None),
        "insert-step" => {
            fs::write(
                repo.join("extra.yaml"),
                "Run the extra project step before finalizing.\n",
            )
            .expect("write the source step");
            (
                owned(&[
                    "config",
                    "insert-step",
                    "--workflow",
                    "single-task",
                    "--after",
                    "implement",
                    "./extra.yaml",
                ]),
                None,
            )
        }
        "replace-step" => {
            fs::write(
                repo.join("project-implement.yaml"),
                "Implement it the house way.\n",
            )
            .expect("write the replacement step");
            (
                owned(&[
                    "config",
                    "replace-step",
                    "workflow:single-task#implement",
                    "./project-implement.yaml",
                ]),
                None,
            )
        }
        "remove-step" => (
            owned(&["config", "remove-step", "workflow:single-task#implement"]),
            None,
        ),
        "fill" => (
            owned(&[
                "config",
                "fill",
                "step:implement#extra-guidance",
                "--from-file",
                "-",
            ]),
            Some("Confirm a changelog entry exists before finalizing.\n"),
        ),
        "fork" => (
            owned(&["config", "fork", "workflow:single-task#implement"]),
            None,
        ),
        other => panic!(
            "`config {other}` is a cascade-authoring verb with no drive in this flow — the \
             axis is `ConfigAck::ALL`, so a seventh verb owes its cell here",
        ),
    }
}

/// Run `jigc <args>` with `cwd`/`$HOME` set, feeding `stdin`.
fn run_jigc_stdin(repo: &Path, home: &Path, args: &[&str], stdin: &str) -> Output {
    use std::io::Write;
    use std::process::Stdio;
    let mut child = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn the jigc binary");
    child
        .stdin
        .take()
        .expect("stdin is piped")
        .write_all(stdin.as_bytes())
        .expect("write stdin");
    child.wait_with_output().expect("collect the jigc output")
}

/// A conformant `adr` body, optionally stamped — the substrate the exit-flip fixtures
/// bend into each condition.
fn adr_body(title: &str, stamp: Option<u32>) -> String {
    let stamp_line = match stamp {
        Some(version) => format!("schema-version: {version}\n"),
        None => String::new(),
    };
    format!(
        "---\nstatus: accepted\ndate: 2026-08-15\n{stamp_line}---\n\n# {title}\n\n## Context\n\n\
         Session lookups must stay sub-millisecond.\n\n## Options\n\nA distributed cache was \
         weighed and rejected on latency.\n\n## Decision\n\nKeep sessions in one node.\n\n\
         ## Consequences\n\nA cold node loses its sessions.\n"
    )
}

/// How this flow reaches one [`STORE_EXIT_FLIPS`] member: either a fixture driven here, or
/// a member whose live production is declared to another suite (with its reason).
enum FlipCell {
    /// A corpus this arm builds, which must make `jigc validate` exit non-zero.
    Live(fn() -> TrialCorpus),
    /// Produced live elsewhere — the suite that does it (checked to exist, so the citation
    /// cannot rot into a file nobody has) and why this walk does not build it, so the split
    /// is a recorded decision rather than a gap.
    ProvenElsewhere {
        suite: &'static str,
        why: &'static str,
    },
}

/// A corpus whose committed store carries a doc stamped **below** its doctype's manifest
/// version — the unmigrated corpus.
fn corpus_below_version() -> TrialCorpus {
    let corpus = TrialCorpus::build(State::Fresh);
    let decisions = corpus.repo().join("docs").join("decisions");
    fs::create_dir_all(&decisions).expect("mk docs/decisions/");
    fs::write(
        decisions.join("cache-strategy.md"),
        adr_body("Cache strategy", Some(1)),
    )
    .expect("write the below-version adr");
    corpus.git(&["add", "."]);
    corpus.git(&["commit", "-q", "-m", "a stale-stamped adr"]);
    corpus
}

/// A corpus carrying a doc stamped **above** this build's schema-version — the condition
/// the pre-1.0 trial planted, and the one the preload names by phrase.
fn corpus_ahead() -> TrialCorpus {
    let corpus = TrialCorpus::build(State::Fresh);
    let decisions = corpus.repo().join("docs").join("decisions");
    fs::create_dir_all(&decisions).expect("mk docs/decisions/");
    fs::write(
        decisions.join("shard-the-index.md"),
        adr_body("Shard the index", Some(99)),
    )
    .expect("write the ahead-stamped adr");
    corpus.git(&["add", "."]);
    corpus.git(&["commit", "-q", "-m", "an ahead-stamped adr"]);
    corpus
}

/// A corpus whose baselined managed doc was moved **out of band** with `git mv` — the
/// member whose sweep *worked*, and which the preload's class may therefore not describe
/// as an untrustworthy sweep.
fn corpus_oob_rename() -> TrialCorpus {
    let corpus = TrialCorpus::build(State::Fresh);
    let task = corpus.start_workflow("record-decision", "settle-the-cache-strategy");
    let address = corpus
        .jigc_ok(&[
            "doc",
            "create",
            "adr",
            "--title",
            "Cache strategy",
            "--task",
            &task,
        ])
        .trim()
        .to_string();
    for (section, prose) in [
        ("context", "Session lookups must stay sub-millisecond."),
        ("decision", "Keep sessions in one node."),
        ("consequences", "A cold node loses its sessions."),
    ] {
        corpus.jigc_stdin_ok(
            &[
                "doc",
                "set-slot",
                &format!("{address}#{section}"),
                "--from-file",
                "-",
                "--task",
                &task,
            ],
            &format!("{prose}\n"),
        );
    }
    corpus.finalize(&task, "adr", "record the cache decision", false);
    // The human-in-git channel: a bare `git mv`, staged. The shipped pre-commit hook
    // refuses to *commit* it (that is the hook's job), so the sweep meets the move
    // exactly where an operator would — staged, un-committed, and detected.
    corpus.git(&[
        "mv",
        "docs/decisions/cache-strategy.md",
        "docs/decisions/session-cache.md",
    ]);
    corpus
}

/// **Arm 5.** A read intent lands on a read verb, every cascade write says it is
/// uncommitted, and every exit-flipping condition delivers what the preload promises.
///
/// Three code-side registries, driven through the real binary:
///
///   * [`CURATED_SIBLING_TIPS`] — each `(parent, guess)` row prints **its** tip instead of
///     clap's did-you-mean, which could steer a `task discard-write` guesser at `discard`
///     and destroy the whole task;
///   * [`PARENT_READ_ANSWERS`] × [`READ_INTENT_GUESSES`] — the axis, not the curated pair:
///     every read-shaped guess under every parent node is answered with **read verbs
///     only**, classified by the production [`verb_kind`] rather than by a list of names,
///     so a write verb can never reach the tip slot;
///   * [`ConfigAck::ALL`] — all six cascade-authoring verbs driven to a real success:
///     each states the write is uncommitted **and** carries `committed: false` beside its
///     declared `op` on the wire;
///   * [`STORE_EXIT_FLIPS`] — each member produced **live** and checked against the
///     `.jigc/AGENT.md` clause that promises it: the sweep exits non-zero and its closing
///     line names *which* condition fired, and while any member is a sweep that **worked**
///     the preload may not state the class as an untrustworthy sweep.
///
/// Red on rc.10: `jigc doc read` was answered with `'create', 'rename'` — a read intent
/// steered at two writes — `config show` drew nothing at all, and the six cascade writes
/// said nothing about landing uncommitted in a committed layer.
#[test]
fn a_read_intent_lands_on_a_read_verb_and_every_ack_and_flip_states_its_fact() {
    let bare = TempDir::new("bare");
    let bare_home = TempDir::new("bare-home");

    // ── the curated rows ──
    for row in CURATED_SIBLING_TIPS {
        let out = run_jigc(bare.path(), bare_home.path(), &[row.parent, row.guess]);
        let text = printed(&out);
        assert!(
            !out.status.success(),
            "[{} {}] a guess at a verb that does not exist stays a usage error; got:\n{text}",
            row.parent,
            row.guess,
        );
        let tip = (row.tip)();
        assert!(
            text.contains(&tip),
            "[{} {}] jigc's own block must carry THIS row's tip; got:\n{text}\nwanted:\n{tip}",
            row.parent,
            row.guess,
        );
        assert!(
            !text.contains("similar subcommand"),
            "[{} {}] a curated row replaces clap's did-you-mean, never prints below it; \
             got:\n{text}",
            row.parent,
            row.guess,
        );
    }

    // ── the read-intent axis: every parent node × every read-shaped guess ──
    let mut answered = 0usize;
    for answer in PARENT_READ_ANSWERS {
        for guess in READ_INTENT_GUESSES {
            if parent_has_subcommand(answer.parent, guess) {
                continue; // a real verb under that parent is not a miss at all
            }
            let mut argv: Vec<&str> = answer.parent.to_vec();
            argv.push(guess);
            let out = run_jigc(bare.path(), bare_home.path(), &argv);
            let text = printed(&out);
            let where_ = format!("jigc {}", argv.join(" "));
            assert!(
                !out.status.success(),
                "[{where_}] an unknown subcommand stays a usage error; got:\n{text}",
            );
            let tip = text
                .lines()
                .find(|line| line.trim_start().starts_with("tip:"))
                .unwrap_or_else(|| {
                    panic!("[{where_}] a read-shaped miss must earn a tip; got:\n{text}")
                });
            answered += 1;
            if tip.contains("similar subcommand") {
                // clap's own wording, kept — but only over siblings that READ.
                for name in suggested_names(tip) {
                    let mut path: Vec<String> =
                        answer.parent.iter().map(|p| (*p).to_string()).collect();
                    path.push(name.clone());
                    assert_eq!(
                        verb_kind(&path),
                        Some(VerbKind::Read),
                        "[{where_}] a read intent may only be offered read verbs; `{name}` \
                         is not one; got:\n{tip}",
                    );
                }
            } else {
                let spans = jigc_spans(tip);
                assert!(
                    !spans.is_empty(),
                    "[{where_}] the parent's read answer must name runnable verbs; got:\n{tip}",
                );
                for path in spans {
                    assert_eq!(
                        verb_kind(&path),
                        Some(VerbKind::Read),
                        "[{where_}] every verb offered to a read intent must READ; \
                         `{path:?}` does not; got:\n{tip}",
                    );
                }
            }
        }
    }
    assert!(
        answered >= 50,
        "the read-intent axis must span the whole lexicon × the parent nodes; got {answered}",
    );

    // ── the cascade-authoring acks: all six verbs, both surfaces ──
    for arm in ConfigAck::ALL {
        let (repo, home) = config_repo(&format!("cfg-{}", arm.verb));
        let (driven, stdin) = config_drive(arm.verb, repo.path());
        let argv: Vec<&str> = driven.iter().map(String::as_str).collect();
        let text_out = match stdin {
            Some(payload) => run_jigc_stdin(repo.path(), home.path(), &argv, payload),
            None => run_jigc(repo.path(), home.path(), &argv),
        };
        assert!(
            text_out.status.success(),
            "[config {}] the drive must land; got:\n{}",
            arm.verb,
            printed(&text_out),
        );
        let text = printed(&text_out);
        for fact in [
            ".jigc/config/",
            "uncommitted",
            "commit it with your next commit",
        ] {
            assert!(
                text.contains(fact),
                "[config {}] the ack must state `{fact}`; got:\n{text}",
                arm.verb,
            );
        }

        // The wire half, in a fresh repo (each verb's delta lands once).
        let (repo, home) = config_repo(&format!("cfg-json-{}", arm.verb));
        let (driven, stdin) = config_drive(arm.verb, repo.path());
        let mut argv: Vec<&str> = vec!["--format", "json"];
        argv.extend(driven.iter().map(String::as_str));
        let json_out = match stdin {
            Some(payload) => run_jigc_stdin(repo.path(), home.path(), &argv, payload),
            None => run_jigc(repo.path(), home.path(), &argv),
        };
        assert!(
            json_out.status.success(),
            "[config {} --format json] the drive must land; got:\n{}",
            arm.verb,
            printed(&json_out),
        );
        let envelope: Value = serde_json::from_str(stdout_of(&json_out).trim())
            .unwrap_or_else(|e| panic!("[config {}] one JSON document ({e})", arm.verb));
        assert_eq!(
            envelope["op"].as_str(),
            Some(arm.op),
            "[config {}] the envelope names this arm's declared op; got:\n{envelope:#}",
            arm.verb,
        );
        assert_eq!(
            envelope["committed"].as_bool(),
            Some(false),
            "[config {}] the text's fact must be on the wire too — the parity rule; \
             got:\n{envelope:#}",
            arm.verb,
        );
    }

    // ── the exit-flip axis, live, against the preload that promises it ──
    let preload_corpus = TrialCorpus::build(State::Fresh);
    let agent_md = fs::read_to_string(preload_corpus.repo().join(".jigc").join("AGENT.md"))
        .expect("`jigc setup` writes the preloaded AGENT.md");
    assert!(
        agent_md.contains("store-scope `jigc validate` is report-only")
            && agent_md.contains("unless one of a few conditions flips that exit"),
        "the preload must state the report-only stance and its exception; got:\n{agent_md}",
    );
    if STORE_EXIT_FLIPS
        .iter()
        .any(|flip| !flip.sweep_untrustworthy)
    {
        for claim in [
            "could not be trusted",
            "cannot be trusted",
            "not trustworthy",
        ] {
            assert!(
                !agent_md.contains(claim),
                "a member of the axis is a sweep that WORKED, so the class may not be stated \
                 as {claim:?}; got:\n{agent_md}",
            );
        }
    }

    for flip in STORE_EXIT_FLIPS {
        let cell = match flip.id {
            "probe-unreliable" => FlipCell::ProvenElsewhere {
                suite: "store_sweep_acceptance.rs",
                why: "it needs a crashing probe over a code-anchored store, which this \
                      walk does not build",
            },
            "oob-rename" => FlipCell::Live(corpus_oob_rename),
            "unmigrated-corpus" => FlipCell::Live(corpus_below_version),
            "ahead-corpus" => FlipCell::Live(corpus_ahead),
            other => panic!(
                "`{other}` is an exit-flipping condition with no cell in this flow — the axis \
                 is `STORE_EXIT_FLIPS`, so a fifth condition owes a decision here",
            ),
        };
        let build = match cell {
            FlipCell::Live(build) => build,
            FlipCell::ProvenElsewhere { suite, why } => {
                let path = Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("tests")
                    .join(suite);
                assert!(
                    path.is_file(),
                    "[{}] its live production is declared to `{suite}` ({why}) — a citation \
                     to a suite that does not exist is not a proof split, it is a gap",
                    flip.id,
                );
                continue;
            }
        };
        let corpus = build();
        let swept = corpus.jigc(&["validate"]);
        let text = printed(&swept);
        assert!(
            !swept.status.success(),
            "[{}] this condition must flip the store sweep's exit; got:\n{text}",
            flip.id,
        );
        assert!(
            text.contains(flip.cause),
            "[{}] the closing line must name WHICH condition fired ({:?}) — what the preload \
             sends the agent there for; got:\n{text}",
            flip.id,
            flip.cause,
        );
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 6 — the empty-commit outcome, and the adapter's owned artifact
// ═════════════════════════════════════════════════════════════════════════════

/// The **frame's assertion** — the clause claiming a rejection happened. Its appearance
/// where nobody rejected anything is the defect this arm exists to catch, so it is matched
/// as the frame emits it.
const REJECTION_ASSERTION: &str = "was rejected (no commit was made)";

/// git's **own** empty-commit prose. It arrives on the seam as the same non-zero exit a
/// hook rejection does, so a door that relays it has mistaken git's refusal to record
/// nothing for someone rejecting the run.
const GIT_EMPTY_COMMIT_PROSE: &str = "nothing to commit";

/// A conformant, v2-stamped `adr` committed by hand at its canonical home — the substrate
/// the `rename` and `migrate-corpus` cells stand on.
fn commit_adr(repo: &Path, slug: &str, title: &str) {
    let dir = repo.join("docs").join("decisions");
    fs::create_dir_all(&dir).expect("mk docs/decisions/");
    fs::write(dir.join(format!("{slug}.md")), adr_body(title, Some(2))).expect("write the adr");
    git_ok(repo, &["add", "."]);
    git_ok(repo, &["commit", "-q", "-m", "seed an adr"]);
}

/// The project scalar layer with the invocation log on — the door's error identity is read
/// back off the log, not off the printed text.
fn logging_project(repo: &Path, squash: Option<&str>) {
    let mut manifest = String::from("scalar:\n  invocation-log: true\n");
    if let Some(value) = squash {
        manifest.push_str(&format!("  finalize.fan-out.squash: {value}\n"));
    }
    let dir = repo.join(".jigc").join("config");
    fs::create_dir_all(&dir).expect("mk the project config dir");
    fs::write(dir.join("manifest.yaml"), manifest).expect("write the project scalar layer");
}

/// A committed 2-criteria `spec` — the `add-from-spec` seed substrate.
const TWO_CRITERIA_SPEC: &str = "\
# Rate limit

## Goal

Bound per-client request volume.

## Context

Downstream services enforced limits ad hoc.

## Criteria

### Rejects the 101st request  {#rejects-burst}

The gateway rejects the 101st request in a rolling 60s window.

### Admits within the window  {#admits-within}

Requests under the cap are admitted unchanged.
";

/// One door's **empty-commit cell**: the repo it stands in and the argv that drives the
/// door into the state where the commit it would make records nothing.
struct EmptyCase {
    repo: TempDir,
    home: TempDir,
    driven: Vec<String>,
    /// Whether this door **acks the no-op** (exit 0) rather than refusing earlier with its
    /// own true diagnosis — declared per cell, so a cell that silently stopped reaching the
    /// emptiness (and passed the absence assertions vacuously) reddens.
    acks_no_op: bool,
    /// A substring of the door's **own** diagnosis — the witness that the fixture really
    /// reached the state where its commit would record nothing. Which words each door
    /// chooses is `commit_rejected_axis.rs`'s assertion; here it is the non-vacuity proof.
    diagnosis: &'static str,
}

/// Build one door's empty-commit fixture. **No hook is installed anywhere** — the whole
/// point is that nothing rejects these runs — and every state is reached the way an
/// operator reaches it: the repeated call, or the change that changes nothing.
fn empty_case(verb: &str) -> EmptyCase {
    let repo = TempDir::new("empty");
    let home = TempDir::new("empty-home");
    init_repo(repo.path());
    run_jigc_ok(repo.path(), home.path(), &["setup"], "`jigc setup`");
    let jigc = |args: &[&str], what: &str| run_jigc_ok(repo.path(), home.path(), args, what);
    let (driven, acks_no_op, diagnosis) = match verb {
        // Nothing staged over a clean tree: the task produces no diff at all.
        "jigc task finalize" => {
            logging_project(repo.path(), None);
            let task = minted_task(&jigc(
                &["start", "--workflow", "single-task", "produce-no-diff"],
                "`jigc start`",
            ));
            // The commit doc is authored clean, so the finalize reaches the **commit**
            // phase and meets the emptiness there rather than stopping at a content row —
            // and the tree is settled first, or the fixture's own scalar layer would be
            // the diff that makes the commit non-empty.
            settle_tree(repo.path());
            fill_commit_doc(repo.path(), home.path(), &task);
            (
                owned(&["task", "finalize", &task]),
                false,
                "finalize.empty-commit",
            )
        }
        // A milestone whose sub-task contributed neither a merged doc nor staged code.
        "jigc milestone finalize (squash: true)" | "jigc milestone finalize (squash: false)" => {
            let squash = if verb.ends_with("true)") {
                "true"
            } else {
                "false"
            };
            logging_project(repo.path(), Some(squash));
            jigc(
                &["milestone", "create", "Cache rework"],
                "`milestone create`",
            );
            jigc(
                &["milestone", "add-task", "cache-rework", "Area low"],
                "`milestone add-task`",
            );
            (
                owned(&["milestone", "finalize", "cache-rework"]),
                false,
                "would land no work",
            )
        }
        // The idempotent rename: a `--to` that slugs to the doc's own id AND matches the
        // `# H1` it already carries, so the rewrite writes the same bytes back.
        "jigc rename" => {
            logging_project(repo.path(), None);
            commit_adr(repo.path(), "alpha-decision", "Alpha decision");
            // An unrelated untracked file rides along, because git's empty-commit refusal
            // *lists* it — an unguarded door names a file with nothing to do with the run.
            fs::write(repo.path().join("scratch.txt"), "unrelated\n").expect("write scratch.txt");
            (
                owned(&["rename", "adr:alpha-decision", "--to", "Alpha decision"]),
                true,
                "no-op:",
            )
        }
        // A corpus already at the current schema-version: the migration rewrites nothing.
        "jigc migrate-corpus" => {
            logging_project(repo.path(), None);
            commit_adr(repo.path(), "alpha-decision", "Alpha decision");
            (owned(&["migrate-corpus"]), true, "already current")
        }
        // The three record-only doors reach emptiness only through the repeated call.
        "jigc milestone create" => {
            logging_project(repo.path(), None);
            jigc(
                &["milestone", "create", "Cache rework"],
                "`milestone create`",
            );
            (
                owned(&["milestone", "create", "Cache rework"]),
                false,
                "milestone.record-exists",
            )
        }
        "jigc milestone add-task" => {
            logging_project(repo.path(), None);
            jigc(
                &["milestone", "create", "Cache rework"],
                "`milestone create`",
            );
            jigc(
                &["milestone", "add-task", "cache-rework", "Area low"],
                "`milestone add-task`",
            );
            (
                owned(&["milestone", "add-task", "cache-rework", "Area low"]),
                false,
                "milestone.sub-task-collision",
            )
        }
        "jigc milestone add-from-spec" => {
            logging_project(repo.path(), None);
            let specs = repo.path().join("docs").join("specs");
            fs::create_dir_all(&specs).expect("mk docs/specs/");
            fs::write(specs.join("rate-limit.md"), TWO_CRITERIA_SPEC).expect("write the spec");
            git_ok(repo.path(), &["add", "."]);
            git_ok(repo.path(), &["commit", "-q", "-m", "add spec"]);
            jigc(&["milestone", "create", "Rate limit"], "`milestone create`");
            jigc(
                &[
                    "milestone",
                    "add-from-spec",
                    "rate-limit",
                    "spec:rate-limit",
                ],
                "`milestone add-from-spec`",
            );
            (
                owned(&[
                    "milestone",
                    "add-from-spec",
                    "rate-limit",
                    "spec:rate-limit",
                ]),
                true,
                "seeded 0 sub-task(s)",
            )
        }
        "jigc milestone discard" => {
            logging_project(repo.path(), None);
            jigc(
                &["milestone", "create", "Cache rework"],
                "`milestone create`",
            );
            jigc(
                &["milestone", "discard", "cache-rework"],
                "`milestone discard`",
            );
            (
                owned(&["milestone", "discard", "cache-rework"]),
                false,
                "milestone.terminal",
            )
        }
        other => panic!(
            "`{other}` is a committing door with no empty-commit fixture in this flow — the \
             axis is the code-side `COMMITTING_DOORS` table, so a door added there owes its \
             cell here",
        ),
    };
    EmptyCase {
        repo,
        home,
        driven,
        acks_no_op,
        diagnosis,
    }
}

/// Commit everything outstanding, so the cell's tree is clean and the door's own commit is
/// the only one that could record anything. An already-clean tree is left alone — git
/// refuses the empty commit that would otherwise be attempted here, which is the very
/// refusal this arm is about.
fn settle_tree(repo: &Path) {
    git_ok(repo, &["add", "-A"]);
    if !git_ok(repo, &["status", "--porcelain"]).trim().is_empty() {
        git_ok(repo, &["commit", "-q", "-m", "settle the fixture"]);
    }
    assert!(
        git_ok(repo, &["status", "--porcelain"]).trim().is_empty(),
        "the empty-commit fixture must start from a clean tree",
    );
}

/// Author a task's `commit` doc clean, so a finalize reaches its commit phase instead of
/// stopping at a content row.
fn fill_commit_doc(repo: &Path, home: &Path, task: &str) {
    run_jigc_ok(
        repo,
        home,
        &[
            "doc",
            "set-field",
            &format!("commit:{task}#type"),
            "--value",
            "feat",
            "--task",
            task,
        ],
        "`doc set-field type`",
    );
    run_jigc_ok(
        repo,
        home,
        &[
            "doc",
            "set-field",
            &format!("commit:{task}#scope"),
            "--value",
            "cache",
            "--task",
            task,
        ],
        "`doc set-field scope`",
    );
    for (slot, prose) in [
        ("summary", "produce no diff"),
        ("body", "Driven by the flow-48 acceptance suite."),
    ] {
        run_jigc_stdin(
            repo,
            home,
            &[
                "doc",
                "set-slot",
                &format!("commit:{task}#{slot}"),
                "--from-file",
                "-",
                "--task",
                task,
            ],
            &format!("{prose}\n"),
        );
    }
}

/// The parsed JSONL invocation-log records of a repo.
fn log_records(repo: &Path) -> Vec<Value> {
    let path = repo.join(".jigc").join("logs").join("invocations.jsonl");
    match fs::read_to_string(&path) {
        Ok(body) => body
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| serde_json::from_str(line).expect("each log line is valid JSON"))
            .collect(),
        Err(_) => Vec::new(),
    }
}

/// **Arm 6, first half.** No committing door dresses an **empty commit** as a rejection.
///
/// `git_commit_capture` types *any* non-zero `git commit` exit as a rejection, and git
/// refuses a commit that would record nothing with exactly that shape — so a door that
/// commits unconditionally asserts a rejection nobody made, routes to a re-run that can
/// only fail identically, and logs a `*.commit-rejected` identity. The axis is the same
/// [`COMMITTING_DOORS`] table the rejecting sweep iterates, driven here into the
/// **complementary** cell with no hook installed anywhere: per door, `HEAD` is untouched,
/// the frame's assertion is absent, git's own empty-commit prose is never relayed, and no
/// `*.commit-rejected` identity reaches the invocation log.
///
/// Which diagnosis each door gives instead — the no-op ack, the identity guard, the
/// zero-contribution refusal — is `commit_rejected_axis.rs`'s per-door assertion; this arm
/// holds the **class** property over the whole table.
#[test]
fn no_committing_door_frames_an_empty_commit_as_a_rejection() {
    for door in cli::invocation_log::COMMITTING_DOORS {
        let verb = door.verb;
        let case = empty_case(verb);
        let repo = case.repo.path();
        let home = case.home.path();
        let argv: Vec<&str> = case.driven.iter().map(String::as_str).collect();

        let head_before = git_ok(repo, &["rev-parse", "HEAD"]);
        let out = run_jigc(repo, home, &argv);
        let text = printed(&out);

        assert_eq!(
            out.status.success(),
            case.acks_no_op,
            "[{verb}] the cell must reach the door's declared empty-commit outcome; \
             got:\n{text}",
        );
        assert!(
            text.contains(case.diagnosis),
            "[{verb}] the door must give its OWN diagnosis (`{}`) — the witness that this \
             fixture reached the state where its commit records nothing; got:\n{text}",
            case.diagnosis,
        );
        assert_eq!(
            git_ok(repo, &["rev-parse", "HEAD"]),
            head_before,
            "[{verb}] a commit that would record nothing must leave HEAD untouched",
        );
        assert!(
            !text.contains(REJECTION_ASSERTION),
            "[{verb}] nobody rejected this run — the frame's assertion must not appear; \
             got:\n{text}",
        );
        assert!(
            !text.contains(GIT_EMPTY_COMMIT_PROSE),
            "[{verb}] git's own empty-commit prose must never be relayed as the cause; \
             got:\n{text}",
        );
        assert!(
            !text.contains(door.error_code),
            "[{verb}] the rejection identity `{}` belongs to a run something rejected; \
             got:\n{text}",
            door.error_code,
        );
        for record in log_records(repo) {
            assert_ne!(
                record["error_code"].as_str(),
                Some(door.error_code),
                "[{verb}] no `*.commit-rejected` identity may reach the log over an empty \
                 commit; got {record}",
            );
        }
    }
}

/// The Claude Code profile's declared guide target — the path the arm reads, stated once
/// as the fixture's premise (the profile is the authority; a change there reddens this).
const GUIDE_PATH: &str = ".claude/skills/jigc/SKILL.md";

/// The guide artifact's text at `repo`.
fn guide_text(repo: &Path) -> String {
    fs::read_to_string(repo.join(GUIDE_PATH)).expect("the guide artifact is installed")
}

/// **Arm 6, second half.** The adapter's owned artifact: jigc replaces what jigc wrote,
/// and refuses to clobber what the user wrote.
///
/// The axis is the ownership question's own case-set ([`GuideOwnership`]), matched
/// **exhaustively** so a fourth verdict cannot be added without a cell here:
///
///   * [`GuideOwnership::Absent`] — the ordinary first install writes the profile-declared
///     path, stamped with the binary version and its body's own `blake3`;
///   * [`GuideOwnership::Owned`] — a copy jigc wrote is **replaced** on the next `setup`
///     (proven through the stamp: an older version is re-stamped, and an unchanged install
///     is byte-identical);
///   * [`GuideOwnership::UserModified`] — a copy the user edited is left **byte-identical**
///     and reported as an advisory with a route, never blocking the install.
///
/// Red on rc.10: no adopter had a version-matched path to the guides at all — the trial had
/// to seed them into a sibling directory by hand.
#[test]
fn the_guide_artifact_is_installed_replaced_and_never_clobbered() {
    for ownership in [
        GuideOwnership::Absent,
        GuideOwnership::Owned,
        GuideOwnership::UserModified,
    ] {
        let repo = TempDir::new("guide");
        let home = TempDir::new("guide-home");
        init_repo(repo.path());
        run_jigc_ok(
            repo.path(),
            home.path(),
            &["setup"],
            "the first `jigc setup`",
        );
        let installed = guide_text(repo.path());
        assert!(
            installed.contains("jigc-version:") && installed.contains("jigc-body-blake3:"),
            "the artifact must be self-describing — version-stamped and body-hashed; \
             got:\n{installed}",
        );

        match ownership {
            GuideOwnership::Absent => {
                // The install commit names it, so what `setup` lists as installed is what
                // its commit carries.
                let committed = git_ok(repo.path(), &["show", "--name-only", "--format=", "HEAD"]);
                assert!(
                    committed.lines().any(|line| line.trim() == GUIDE_PATH),
                    "the install commit must carry the guide artifact; carried:\n{committed}",
                );
            }
            GuideOwnership::Owned => {
                // Still jigc's bytes, stamped at an older version: replaced and re-stamped.
                let downgraded = installed.replacen("jigc-version:", "jigc-version: 0.0.1 #", 1);
                fs::write(repo.path().join(GUIDE_PATH), &downgraded).expect("downgrade the stamp");
                run_jigc_ok(
                    repo.path(),
                    home.path(),
                    &["setup"],
                    "the replacing `jigc setup`",
                );
                assert_eq!(
                    guide_text(repo.path()),
                    installed,
                    "a copy jigc owns is replaced with this build's artifact, byte for byte",
                );
            }
            GuideOwnership::UserModified => {
                let mine = format!("{installed}\n<!-- my own note -->\n");
                fs::write(repo.path().join(GUIDE_PATH), &mine).expect("edit the artifact");
                let out = run_jigc(repo.path(), home.path(), &["setup"]);
                let text = printed(&out);
                assert!(
                    out.status.success(),
                    "an edited guide is an advisory, never a blocked install; got:\n{text}",
                );
                assert_eq!(
                    guide_text(repo.path()),
                    mine,
                    "the user's bytes must survive byte-identical",
                );
                assert!(
                    text.contains(cli::setup::GUIDE_MODIFIED_CODE),
                    "the install must SAY it left the artifact alone; got:\n{text}",
                );
            }
        }
    }
}
