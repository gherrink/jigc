//! M42 Increment 7 / T5 — `jigc milestone discard <id> [--force]`: the milestone family's
//! terminal verb (`design/team-ready-state.md` → `jigc milestone discard <id>` — the three
//! properties; `design/write-commands.md` → Abandoning a milestone). It settles the committed
//! record to the `discarded` terminal (T1's schema-version-2 enum member, through T4's per-item
//! flip), lands ONE record-only commit, and tears the workbench down — the sub-task areas, the
//! registered fan-out worktrees, **and** `.jigc/milestones/<id>/`, which had no reachable remover
//! at all.
//!
//! Seven proofs, driving the REAL binary against throwaway git repos:
//!
//!   (RED-i)   **The dirty-worktree refusal.** A file written into a provisioned sub-task
//!             worktree makes `discard` REFUSE (exit non-zero, naming the worktree and the dirty
//!             path); the record, the worktree, and the file are all intact, and no commit lands.
//!             Red today: teardown's `git worktree remove --force` — safe at *finalize*, where
//!             the commit lands first — destroys the file silently at exit 0 on the abandon path,
//!             where the work is by definition uncommitted (the M31 WIP-safety shape).
//!
//!   (RED-ii)  **`--force` settles + tears down.** Exit 0; the committed record's header and every
//!             in-flight sub-task read `status: discarded`; EXACTLY ONE record-only commit lands
//!             (unrelated staged and untracked WIP in the main checkout untouched); and
//!             `.jigc/milestones/<id>/`, the sub-task areas, and the registered worktrees are all
//!             gone.
//!
//!   (RED-iii) **An unknown milestone id routes and removes nothing** — a live milestone's
//!             workbench and record survive a discard aimed at an id that does not exist.
//!
//!   (RED-iv)  **Dev-only degrades** (the omitting context): with no methodology pack there is no
//!             `milestone-record` schema, so `discard` settles no record and lands NO commit —
//!             and still tears the whole workbench down, exit 0.
//!
//! And the three the M42 completion audit's HIGH added — **the terminal is actually terminal**
//! (`design/team-ready-state.md` → The lifecycle). The teardown was never a *guard*: the workbench
//! reseed (the M39 fresh-clone continuation path) rebuilt `.jigc/milestones/<id>/` from the
//! committed record without ever reading its `status`, so the settled milestone came straight back:
//!
//!   (RED-v)   **A `discarded` milestone refuses every verb** — `provision` no longer
//!             re-provisions worktrees at the abandoned base, `add-task` no longer appends an
//!             **active** sub-task to a **discarded** record (both exit 0 before the fix).
//!
//!   (RED-vi)  **A `joined` milestone refuses every verb** — the same hole at the *other*
//!             terminal, closed by the same predicate, not a second special case.
//!
//!   (RED-vii) **`create` refuses an id a committed record already owns** — the mint's collision
//!             check reads the *workbench*, which a terminal op has just removed, so re-creating a
//!             settled milestone's title **overwrote its committed record** at exit 0.
//!
//! And the one M48 Increment 1 adds — **the guard's subject is the path, not the registered set**
//! (`DECISIONS.md` 2026-08-13 → the Settle, F3):
//!
//!   (RED-viii) **A non-registered, non-empty sub-task worktree path refuses too.** RED-i's guard
//!              only ever looked at worktrees *registered here*, which is structurally blind to
//!              the ordinary trigger (a `cp -R` or `mv` of the repo registers the copy's
//!              worktrees at the **source's** path), so the abandon settled the record and tore
//!              the workbench down at exit 0 over content nothing could vouch for. `--force`
//!              still settles and tears down exactly as before — and leaves the non-registered
//!              path **orphaned on disk**, which is what the teardown has always done and what
//!              the refusal must therefore say.

use cli::milestone::DISCARD_DOOR;

/// [`DISCARD_DOOR`]'s blocking refusal code. Since M46 Inc 2 the door table's `code` is the
/// axis's **refuse-vs-narrate discriminator** (`jigc milestone finalize` joined it and
/// carries no refusal), so a refusing door's code is read through its `Some`.
fn discard_code() -> &'static str {
    DISCARD_DOOR
        .code
        .expect("`jigc milestone discard` is a refusing door")
}
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-milestone-discard-{tag}-{}-{:?}",
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
        // The fan-out worktrees are ordinary directories under `.jigc/worktrees/` — a plain
        // recursive remove clears them along with the repo.
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// The milestone under test and its three sub-tasks — all **active**.
///
/// The retired fixture built a *partially-joined* milestone by running `finalize` and then
/// `add-task`. That sequence is **not a legitimate flow**: `finalize` joins the whole milestone
/// and `joined` is a **terminal** — the add-task only worked because the workbench reseed never
/// consulted the record's status (the M42 completion-audit HIGH, fixed here). A `joined` item
/// under a live header is therefore not reachable through the verbs at all, so `discard`'s
/// per-item rule (*a genuinely joined sub-task stays `joined`*) is proven where it is reachable:
/// the engine unit tests over a directly-constructed record
/// (`engine::milestone::tests::discard_flips_non_joined_items_and_header_leaving_a_joined_item_byte_identical`).
const MILESTONE: &str = "cache-rework";
const SUBS: [&str; 3] = [
    "warm-the-read-cache",
    "evict-cold-entries",
    "purge-stale-keys",
];
/// The sub-task whose worktree carries the abandon path's uncommitted WIP.
const ACTIVE_SUB: &str = "purge-stale-keys";
/// The bytes planted in the non-registered leftover — a refusal must leave them exactly this.
const PRECIOUS: &str = "precious, uncommitted, in no object DB\n";

/// Run a `git` command in `repo`, asserting success, returning trimmed stdout.
fn git(repo: &Path, args: &[&str]) -> String {
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
    String::from_utf8(out.stdout).expect("utf-8 git stdout")
}

/// Initialize a real git repo with one commit (mint reads HEAD via `git rev-parse`).
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "README.md"]);
    git(repo, &["commit", "-q", "-m", "initial"]);
}

/// Write the `[dev ▸ methodology]` compose marker — the exact key `make_pack` reads, so the
/// composed cascade resolves the methodology-pack `milestone-record` doctype (and dev `docs-root`).
fn write_compose_marker(repo: &Path) {
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("mk project config");
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        "compose-embedded-methodology: true\n",
    )
    .expect("write compose marker");
}

/// Run `jigc milestone <args>` with `cwd = repo` and `$HOME = home`, never inheriting a harness
/// `JIGC_PACK_DIR` (the compose-marker path requires it ABSENT, else the env pack supersedes).
fn run_milestone(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .arg("milestone")
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("run the jigc binary")
}

/// Assert a `jigc milestone` invocation exited 0, surfacing stderr on failure.
fn assert_ok(out: &std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
}

/// The number of commits reachable from HEAD.
fn commit_count(repo: &Path) -> u32 {
    git(repo, &["rev-list", "--count", "HEAD"])
        .trim()
        .parse()
        .expect("commit count parses")
}

/// The repo-relative paths touched by a commit.
fn commit_files(repo: &Path, rev: &str) -> Vec<String> {
    git(
        repo,
        &["diff-tree", "--no-commit-id", "--name-only", "-r", rev],
    )
    .lines()
    .map(str::to_string)
    .collect()
}

/// The committed record path (repo-relative) under the composed docs-root.
fn record_rel() -> String {
    format!("docs/milestone-records/{MILESTONE}.md")
}

/// The committed record's bytes.
fn read_record(repo: &Path) -> String {
    fs::read_to_string(repo.join(record_rel())).expect("read the committed milestone record")
}

/// The gitignored workbench paths the teardown must remove.
fn milestone_area(repo: &Path) -> PathBuf {
    repo.join(".jigc").join("milestones").join(MILESTONE)
}

fn subtask_area(repo: &Path, sub: &str) -> PathBuf {
    repo.join(".jigc").join("tasks").join(sub)
}

fn worktree_dir(repo: &Path, sub: &str) -> PathBuf {
    repo.join(".jigc").join("worktrees").join(sub)
}

/// The repo's registered git worktrees (the `worktree <path>` lines of the porcelain listing),
/// **excluding** the main checkout — exactly the fan-out worktrees `provision` registered.
fn registered_fanout_worktrees(repo: &Path) -> Vec<String> {
    git(repo, &["worktree", "list", "--porcelain"])
        .lines()
        .filter_map(|line| line.strip_prefix("worktree "))
        .filter(|path| path.contains(".jigc/worktrees/"))
        .map(str::to_string)
        .collect()
}

/// The `status` leaf of the record's `meta` header — read from the emitted committed bytes
/// (everything ahead of the H1).
fn header_status(body: &str) -> String {
    let end = body.find("\n# ").expect("the record carries an H1");
    body[..end]
        .lines()
        .find_map(|line| line.strip_prefix("status: "))
        .map(str::to_string)
        .unwrap_or_else(|| panic!("the record header carries a `status` field:\n{body}"))
}

/// The `status` leaf recorded for sub-task `sub` — sliced out of the item's own field block in
/// the emitted committed bytes (from its `{#<id>}` anchor to the next item heading).
fn item_status(body: &str, sub: &str) -> String {
    let anchor = format!("{{#{sub}}}");
    let start = body
        .find(&anchor)
        .unwrap_or_else(|| panic!("the record names sub-task `{sub}`:\n{body}"));
    let rest = &body[start..];
    let end = rest[1..]
        .find("\n### ")
        .map(|i| i + 1)
        .unwrap_or(rest.len());
    rest[..end]
        .lines()
        .find_map(|line| line.trim().strip_prefix("- status: "))
        .map(str::to_string)
        .unwrap_or_else(|| {
            panic!(
                "sub-task `{sub}` carries a `status` leaf:\n{}",
                &rest[..end]
            )
        })
}

/// The milestone and its three sub-tasks, minted through the production verbs — **without**
/// `provision`, so nothing under `.jigc/worktrees/` is registered. The un-provisioned half of
/// [`setup_live_milestone`], reused by the non-registered-leftover arm below (which needs exactly
/// this state: a milestone whose sub-task worktree paths this repo has no registration for).
fn mint_milestone(repo: &Path, home: &Path) {
    assert_ok(
        &run_milestone(repo, home, &["create", "Cache rework"]),
        "`jigc milestone create`",
    );
    for (sub, intent) in SUBS.iter().zip([
        "Warm the read cache",
        "Evict cold entries",
        "Purge stale keys",
    ]) {
        assert_ok(
            &run_milestone(repo, home, &["add-task", MILESTONE, intent]),
            &format!("add-task `{sub}`"),
        );
    }
}

/// A **live, mid-flight** milestone, built entirely through the production verbs on a
/// `[dev ▸ methodology]` repo: create → add-task ×3 → provision (one worktree per sub-task). The
/// committed record reads `active` on the header and on every item, and the workbench (the
/// milestone area, the sub-task areas, three registered worktrees) is live — the state the abandon
/// path exists for, and the state every terminal test below settles *from*.
fn setup_live_milestone(repo: &Path, home: &Path) {
    mint_milestone(repo, home);
    assert_ok(
        &run_milestone(repo, home, &["provision", MILESTONE]),
        "`jigc milestone provision`",
    );

    // The pre-discard record: nothing has landed, so the header and every item read `active`.
    let body = read_record(repo);
    assert_eq!(
        header_status(&body),
        "active",
        "pre-discard header:\n{body}"
    );
    for sub in SUBS {
        assert_eq!(
            item_status(&body, sub),
            "active",
            "pre-discard: `{sub}` is in flight:\n{body}",
        );
    }
    // The live workbench: the milestone area, the sub-task areas, three worktrees.
    assert!(milestone_area(repo).is_dir(), "the milestone area is live");
    assert!(
        subtask_area(repo, ACTIVE_SUB).is_dir(),
        "the sub-task working areas are live",
    );
    assert_eq!(
        registered_fanout_worktrees(repo).len(),
        3,
        "provision registered one worktree per sub-task",
    );
}

/// (RED-i) A dirty sub-task worktree REFUSES the discard — the abandon path's WIP-safety
/// property. Nothing is destroyed: the file, the worktree, the record, and the workbench all
/// survive, and no commit lands.
#[test]
fn a_dirty_subtask_worktree_refuses_the_discard() {
    let repo = TempDir::new("dirty");
    let home = TempDir::new("home");
    init_repo(repo.path());
    write_compose_marker(repo.path());
    setup_live_milestone(repo.path(), home.path());

    // Uncommitted work in the fanned sub-agent's worktree — by definition uncommitted on the
    // abandon path (nothing has been committed for it, and nothing will be).
    let scratch = worktree_dir(repo.path(), ACTIVE_SUB).join("scratch.rs");
    fs::write(&scratch, "fn wip() {}\n").expect("write the sub-agent's WIP");

    let before = read_record(repo.path());
    let pre_count = commit_count(repo.path());

    let out = run_milestone(repo.path(), home.path(), &["discard", MILESTONE]);
    assert!(
        !out.status.success(),
        "a dirty sub-task worktree must REFUSE the discard; got exit 0\nstdout:\n{}",
        String::from_utf8_lossy(&out.stdout),
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains(ACTIVE_SUB) && stderr.contains("scratch.rs"),
        "the refusal names the dirty worktree and its path; stderr:\n{stderr}",
    );
    assert!(
        stderr.contains("--force"),
        "the refusal routes to `--force` (the explicit destroy-my-work confirmation); stderr:\n{stderr}",
    );

    // Nothing was destroyed — the file, the worktree, the workbench, the record, the history.
    assert_eq!(
        fs::read_to_string(&scratch).expect("the WIP file survives the refusal"),
        "fn wip() {}\n",
        "the refused discard must not touch the uncommitted work",
    );
    assert!(
        worktree_dir(repo.path(), ACTIVE_SUB).is_dir(),
        "the refused discard leaves the worktree registered and on disk",
    );
    assert_eq!(
        registered_fanout_worktrees(repo.path()).len(),
        3,
        "the refused discard removes no worktree",
    );
    assert!(milestone_area(repo.path()).is_dir(), "the area survives");
    assert!(
        subtask_area(repo.path(), ACTIVE_SUB).is_dir(),
        "the sub-task area survives",
    );
    assert_eq!(
        read_record(repo.path()),
        before,
        "the refused discard leaves the committed record byte-identical",
    );
    assert_eq!(
        commit_count(repo.path()),
        pre_count,
        "the refused discard commits nothing",
    );
}

/// (RED-viii, M48 Inc 1 T3) A **non-registered, non-empty** directory at a sub-task's worktree
/// path refuses the discard too — the guard's subject is the *path*, not the registered set.
///
/// Red before the swap: `provisioned_worktrees` intersects the task list with the *registered*
/// worktrees, so an unregistered leftover was invisible and the discard settled the record and
/// tore the workbench down at exit 0 — abandoning the milestone that was the only remaining
/// handle on those bytes. The ordinary way to reach this state is a `cp -R` or `mv` of the repo
/// (the copy's admin records name the source's paths), reduced here to what the guard actually
/// sees: a path this repo has no registration for, holding content nothing can vouch for.
///
/// **The refusal must not claim a removal the door does not perform.** `remove_worktrees` skips a
/// non-registered path, so `--force` settles the record and tears the workbench down exactly as
/// before and the leftover is left **orphaned on disk** — the composed-doors property, asserted
/// here so the refusal's wording stays checkable rather than decorative.
#[test]
fn a_non_registered_leftover_at_a_subtask_worktree_path_refuses_the_discard() {
    let repo = TempDir::new("leftover");
    let home = TempDir::new("home");
    init_repo(repo.path());
    write_compose_marker(repo.path());
    mint_milestone(repo.path(), home.path());

    let leftover = worktree_dir(repo.path(), ACTIVE_SUB);
    fs::create_dir_all(&leftover).expect("mk the leftover dir");
    let planted = leftover.join("precious.txt");
    fs::write(&planted, PRECIOUS).expect("plant the precious file");
    assert!(
        registered_fanout_worktrees(repo.path()).is_empty(),
        "the fixture's premise: this repo has NO registration for that path",
    );

    let before = read_record(repo.path());
    let pre_count = commit_count(repo.path());

    let out = run_milestone(repo.path(), home.path(), &["discard", MILESTONE]);
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    assert!(
        !out.status.success(),
        "a non-registered leftover holding content must REFUSE the discard; got exit 0\nstdout:\n{}",
        String::from_utf8_lossy(&out.stdout),
    );
    assert!(
        stderr.contains(discard_code()),
        "the refusal carries the door-scoped code `{}`; stderr:\n{stderr}",
        discard_code(),
    );
    assert!(
        stderr.contains(&leftover.display().to_string()),
        "the refusal names the leftover path `{}`; stderr:\n{stderr}",
        leftover.display(),
    );
    assert!(
        stderr.contains("precious.txt"),
        "the refusal names what it found there; stderr:\n{stderr}",
    );
    assert!(
        stderr.contains("--force"),
        "the refusal routes at the consent flag; stderr:\n{stderr}",
    );

    // Nothing moved: the planted bytes, the record, the history, the workbench.
    assert_eq!(
        fs::read_to_string(&planted).expect("the planted file survives the refusal"),
        PRECIOUS,
        "the refused discard leaves the leftover byte-intact",
    );
    assert_eq!(
        read_record(repo.path()),
        before,
        "the refused discard leaves the committed record byte-identical",
    );
    assert_eq!(
        commit_count(repo.path()),
        pre_count,
        "the refused discard commits nothing",
    );
    assert!(milestone_area(repo.path()).is_dir(), "the area survives");
    assert!(
        subtask_area(repo.path(), ACTIVE_SUB).is_dir(),
        "the sub-task area survives",
    );

    // `--force` is the consent: the record settles and the workbench goes, exactly as before.
    let forced = run_milestone(repo.path(), home.path(), &["discard", MILESTONE, "--force"]);
    assert_ok(&forced, "`jigc milestone discard --force`");
    let after = read_record(repo.path());
    assert_eq!(
        header_status(&after),
        "discarded",
        "`--force` settles the record:\n{after}",
    );
    assert_eq!(
        commit_count(repo.path()),
        pre_count + 1,
        "`--force` lands exactly one record-only commit; stderr:\n{}",
        String::from_utf8_lossy(&forced.stderr),
    );
    assert!(
        !milestone_area(repo.path()).exists(),
        "`--force` removes `.jigc/milestones/<id>/`",
    );
    assert!(
        !subtask_area(repo.path(), ACTIVE_SUB).exists(),
        "`--force` removes the sub-task working areas",
    );
    // And the leftover is ORPHANED, not deleted — the teardown never reached a path it has no
    // registration for, before this change or after it. The refusal said so; here it is.
    assert_eq!(
        fs::read_to_string(&planted).expect("the leftover survives the forced discard"),
        PRECIOUS,
        "the teardown leaves a non-registered path on disk — the doors compose",
    );
}

/// (RED-ix, M48 Inc 1) The law-1 half of RED-viii's widening: `jigc milestone discard --help` —
/// the surface a reader consults **before** running a teardown — must state the refusal the door
/// actually performs and what `--force` actually costs.
///
/// Red before this fix: T3 replaced the guard's subject (the registered set ∩ the task list) with
/// the *path* probe, so the door refuses over content that is entirely **committed** — while the
/// long help still promised refusal only "when any sub-task worktree holds uncommitted work", and
/// the flag still promised `--force` "destroy[s] it" over a path the teardown provably **leaves on
/// disk** (RED-viii's own closing assertion). Two law-1 lies, introduced by the widening that made
/// them false (`design/surface-contract.md` → law 1; the *widen a guard's trigger, re-derive its
/// response* rule in `implementation/dev-workflow.md`).
///
/// Asserted against the **emitted bytes** (the real `--help` render through the built binary), not
/// the const the doc comment compiles into, so the pin binds what a reader actually sees — the
/// `uninstall_long_help_states_the_refusal_and_names_its_escape_hatch` idiom for the sibling door.
/// `--help` needs no repo or pack: clap prints it before dispatch.
#[test]
fn discard_long_help_states_the_widened_refusal_and_what_force_really_does() {
    let out = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(["milestone", "discard", "--help"])
        .output()
        .expect("run the jigc binary");
    assert!(
        out.status.success(),
        "`jigc milestone discard --help` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    // clap wraps the long help to the terminal width, so compare on collapsed whitespace.
    let help = String::from_utf8_lossy(&out.stdout)
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");

    for needle in [
        // the refusal, by the code it carries and the paths it probes …
        discard_code(),
        ".jigc/worktrees/",
        // … its real subject: any content the door cannot prove disposable, committed included,
        // and the ordinary way an unregistered path gets there …
        "content nothing can prove is disposable",
        "has not registered as a worktree",
        // … and what `--force` really does, per disposition (RED-viii asserts both halves live).
        "--force",
        "removed with everything uncommitted in it",
        "orphaned on disk",
    ] {
        assert!(
            help.contains(needle),
            "`milestone discard --help` must state the widened refusal and `--force`'s real \
             cost — missing {needle:?}; got:\n{help}",
        );
    }
    // The two falsified promises are the defect itself: a refusal scoped to *uncommitted* work
    // (the door refuses over committed content too), and a `--force` that "destroys" the content
    // it in fact leaves behind on a path this repo never registered.
    for banned in [
        "Refuses when any sub-task worktree holds uncommitted work",
        "Discard even when a sub-task worktree holds uncommitted work",
        "the explicit consent to destroy it",
        "without this, a dirty worktree refuses the abandon",
    ] {
        assert!(
            !help.contains(banned),
            "`milestone discard --help` must not restate the falsified promise {banned:?}; \
             got:\n{help}",
        );
    }
}

/// (RED-ii) `--force` settles the record (the header and every in-flight sub-task to `discarded`)
/// in EXACTLY ONE record-only commit — unrelated staged and untracked WIP untouched — and tears
/// the whole workbench down.
#[test]
fn force_settles_the_record_and_tears_the_workbench_down() {
    let repo = TempDir::new("force");
    let home = TempDir::new("home");
    init_repo(repo.path());
    write_compose_marker(repo.path());
    setup_live_milestone(repo.path(), home.path());

    // The sub-agent's uncommitted work — `--force` is the explicit consent to destroy it.
    fs::write(
        worktree_dir(repo.path(), ACTIVE_SUB).join("scratch.rs"),
        "fn wip() {}\n",
    )
    .expect("write the sub-agent's WIP");
    // Unrelated in-flight WIP in the main checkout: one staged, one untracked. Neither may ride
    // the record commit (the M30/M31 path-scoped staging discipline).
    fs::write(repo.path().join("staged.rs"), "fn staged() {}\n").expect("write the staged WIP");
    git(repo.path(), &["add", "staged.rs"]);
    fs::write(repo.path().join("untracked.md"), "notes\n").expect("write the untracked WIP");

    let pre_count = commit_count(repo.path());

    let out = run_milestone(repo.path(), home.path(), &["discard", MILESTONE, "--force"]);
    assert_ok(&out, "`jigc milestone discard --force`");

    // Exactly ONE commit landed, and it touched ONLY the record.
    assert_eq!(
        commit_count(repo.path()),
        pre_count + 1,
        "discard lands exactly one record-only commit; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    assert_eq!(
        commit_files(repo.path(), "HEAD"),
        vec![record_rel()],
        "the discard commit records ONLY the milestone record",
    );

    // The settled record — on disk and in the commit, the same bytes.
    let after = read_record(repo.path());
    assert_eq!(
        after,
        git(repo.path(), &["show", &format!("HEAD:{}", record_rel())]),
        "the on-disk record matches the bytes the discard committed",
    );
    assert_eq!(
        header_status(&after),
        "discarded",
        "the header settles to the abandon terminal:\n{after}",
    );
    for sub in SUBS {
        assert_eq!(
            item_status(&after, sub),
            "discarded",
            "every never-joined sub-task settles to discarded:\n{after}",
        );
    }
    assert!(
        !after.contains("status: active"),
        "no recorded status survives un-settled:\n{after}",
    );

    // The unrelated WIP is untouched: the staged file is still staged (never committed), the
    // untracked file still untracked on disk.
    assert_eq!(
        git(repo.path(), &["diff", "--cached", "--name-only"]).trim(),
        "staged.rs",
        "the pre-staged unrelated file stays staged and out of the record commit",
    );
    assert!(
        repo.path().join("untracked.md").is_file(),
        "the untracked WIP survives the discard",
    );

    // The workbench is gone: the milestone area (which had NO reachable remover), the sub-task
    // areas, and every registered fan-out worktree.
    assert!(
        !milestone_area(repo.path()).exists(),
        "discard removes `.jigc/milestones/<id>/`",
    );
    assert!(
        !subtask_area(repo.path(), ACTIVE_SUB).exists(),
        "discard removes the sub-task working areas",
    );
    assert!(
        registered_fanout_worktrees(repo.path()).is_empty(),
        "discard removes every registered fan-out worktree; still registered: {:?}",
        registered_fanout_worktrees(repo.path()),
    );
    for sub in SUBS {
        assert!(
            !worktree_dir(repo.path(), sub).exists(),
            "the `{sub}` worktree checkout is gone from disk",
        );
    }
}

/// (RED-iii) An unknown milestone id routes a block and removes nothing — the live milestone's
/// record, workbench, and worktrees all survive.
#[test]
fn an_unknown_milestone_id_routes_and_removes_nothing() {
    let repo = TempDir::new("unknown");
    let home = TempDir::new("home");
    init_repo(repo.path());
    write_compose_marker(repo.path());
    setup_live_milestone(repo.path(), home.path());

    let before = read_record(repo.path());
    let pre_count = commit_count(repo.path());

    let out = run_milestone(repo.path(), home.path(), &["discard", "no-such-milestone"]);
    assert!(
        !out.status.success(),
        "an unknown milestone id must block; got exit 0",
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("no-such-milestone") && stderr.contains("route:"),
        "the block names the id and carries a route; stderr:\n{stderr}",
    );

    // Nothing was removed or committed.
    assert_eq!(read_record(repo.path()), before, "the record is untouched");
    assert_eq!(commit_count(repo.path()), pre_count, "nothing committed");
    assert!(milestone_area(repo.path()).is_dir(), "the area survives");
    assert!(
        subtask_area(repo.path(), ACTIVE_SUB).is_dir(),
        "the sub-task area survives",
    );
    assert_eq!(
        registered_fanout_worktrees(repo.path()).len(),
        3,
        "no worktree was torn down",
    );
}

/// (RED-iv) **The omitting context** — dev-only (no methodology pack, so no `milestone-record`
/// schema resolves): `discard` settles no record and lands NO commit, and still tears the whole
/// workbench down at exit 0. The record arms degrade, they never error.
#[test]
fn dev_only_discard_tears_down_the_workbench_with_no_record_commit() {
    let repo = TempDir::new("devonly");
    let home = TempDir::new("home");
    init_repo(repo.path());
    // No compose marker — the dev pack alone, which ships no `milestone-record` doctype.

    assert_ok(
        &run_milestone(repo.path(), home.path(), &["create", "Cache rework"]),
        "dev-only `jigc milestone create`",
    );
    assert_ok(
        &run_milestone(
            repo.path(),
            home.path(),
            &["add-task", MILESTONE, "Purge stale keys"],
        ),
        "dev-only add-task",
    );
    assert_ok(
        &run_milestone(repo.path(), home.path(), &["provision", MILESTONE]),
        "dev-only provision",
    );
    let pre_count = commit_count(repo.path());

    let out = run_milestone(repo.path(), home.path(), &["discard", MILESTONE]);
    assert_ok(&out, "dev-only `jigc milestone discard`");

    // No record exists, so none is settled and nothing is committed.
    assert!(
        !repo.path().join("docs").join("milestone-records").exists(),
        "dev-only resolves no `milestone-record` doctype — no record home is created",
    );
    assert_eq!(
        commit_count(repo.path()),
        pre_count,
        "dev-only discard lands no record commit",
    );
    // The workbench is torn down all the same.
    assert!(
        !milestone_area(repo.path()).exists(),
        "dev-only discard removes `.jigc/milestones/<id>/`",
    );
    assert!(
        !subtask_area(repo.path(), ACTIVE_SUB).exists(),
        "dev-only discard removes the sub-task working area",
    );
    assert!(
        registered_fanout_worktrees(repo.path()).is_empty(),
        "dev-only discard removes the registered fan-out worktree",
    );
}

/// **The terminal predicate, through the binary** (`design/team-ready-state.md` → The lifecycle —
/// a terminal record has no workbench; M42 completion-audit HIGH). Run every milestone verb that
/// reads through the workbench cache against a **settled** milestone: each must REFUSE — naming
/// the terminal and routing to the committed record's read surface — resurrect **no** workbench,
/// register **no** worktree, leave the record **byte-identical**, and land **no** commit.
///
/// `add-from-spec` is aimed at a **nonexistent** spec on purpose: the terminal refusal must fire
/// **before** the verb's own guards, so the operator is told the milestone is settled rather than
/// being sent off to fix a spec address that was never the problem.
fn assert_every_verb_refuses(repo: &Path, home: &Path, terminal: &str) {
    let before = read_record(repo);
    let pre_count = commit_count(repo);

    for args in [
        vec!["add-task", MILESTONE, "Another thing"],
        vec!["add-from-spec", MILESTONE, "spec:no-such-spec"],
        vec!["list-tasks", MILESTONE],
        vec!["provision", MILESTONE],
        vec!["execute", MILESTONE],
        vec!["join", MILESTONE],
        vec!["finalize", MILESTONE],
        vec!["discard", MILESTONE, "--force"],
    ] {
        let what = format!("`jigc milestone {}`", args.join(" "));
        let out = run_milestone(repo, home, &args);
        assert!(
            !out.status.success(),
            "{what} on a `{terminal}` milestone must REFUSE; got exit 0\nstdout:\n{}",
            String::from_utf8_lossy(&out.stdout),
        );
        let stderr = String::from_utf8_lossy(&out.stderr);
        assert!(
            stderr.contains(MILESTONE) && stderr.contains(terminal),
            "{what}: the refusal names the milestone and its terminal; stderr:\n{stderr}",
        );
        assert!(
            stderr.contains(&format!("jigc doc show milestone-record:{MILESTONE}")),
            "{what}: the refusal routes to the committed record's read surface; stderr:\n{stderr}",
        );

        // The workbench stays torn down — the reseed is what resurrected it.
        assert!(
            !milestone_area(repo).exists(),
            "{what}: the settled milestone's workbench must NOT be rebuilt",
        );
        assert!(
            registered_fanout_worktrees(repo).is_empty(),
            "{what}: no worktree may be re-provisioned at a settled milestone's base; registered: {:?}",
            registered_fanout_worktrees(repo),
        );
        assert_eq!(
            read_record(repo),
            before,
            "{what}: the settled record is byte-identical after the refusal",
        );
        assert_eq!(
            commit_count(repo),
            pre_count,
            "{what}: a refused verb commits nothing",
        );
    }
}

/// (RED-v) **`discarded` is actually terminal.** After the abandon, every milestone verb refuses:
/// the record cannot gain an `active` sub-task, and `provision` cannot re-provision the worktrees
/// the teardown removed.
///
/// Red before the fix: the teardown was **not a guard**. `reseed_cache` rebuilt
/// `.jigc/milestones/<id>/` from the committed record whenever the cache was absent — the M39
/// fresh-clone continuation path — and never consulted the record's `status`. So `provision`
/// re-provisioned worktrees at the abandoned base (exit 0) and `add-task` appended an **active**
/// sub-task to a **discarded** record (exit 0): the lying committed record was back, and the
/// worktree hazard the teardown was added for was reopened.
#[test]
fn a_discarded_milestone_refuses_every_verb() {
    let repo = TempDir::new("terminal-discarded");
    let home = TempDir::new("home");
    init_repo(repo.path());
    write_compose_marker(repo.path());
    setup_live_milestone(repo.path(), home.path());

    assert_ok(
        &run_milestone(repo.path(), home.path(), &["discard", MILESTONE, "--force"]),
        "`jigc milestone discard --force`",
    );
    assert_eq!(
        header_status(&read_record(repo.path())),
        "discarded",
        "precondition: the record settled to the abandon terminal",
    );

    assert_every_verb_refuses(repo.path(), home.path(), "discarded");
}

/// (RED-vi) **`joined` is terminal too — the same hole, not a second special case.** After
/// `milestone finalize` lands the work and removes the workbench, every milestone verb refuses:
/// nothing re-provisions worktrees at the landed base, and no `active` sub-task is appended to a
/// record that says the milestone joined.
///
/// Red before the fix: identical to the `discarded` sibling — `reseed_cache` rebuilt the cache
/// from the **joined** record. `add-task` after a finalize was so thoroughly accepted that Inc 7's
/// own fixture (`setup_partially_joined_milestone`, now retired) *used* it to mint its
/// "partially-joined" record. Both terminals are fixed by **one** predicate — a record in a
/// terminal state does not re-seed a workbench.
#[test]
fn a_joined_milestone_refuses_every_verb() {
    let repo = TempDir::new("terminal-joined");
    let home = TempDir::new("home");
    init_repo(repo.path());
    write_compose_marker(repo.path());
    setup_live_milestone(repo.path(), home.path());

    // One sub-agent stages real code in its worktree — the work that makes this a genuine join.
    // Without it the boundary would land nothing but its own record flip, which `finalize`
    // refuses (M47 Inc 3; `design/finalize.md` → The zero-contribution refusal), and the
    // `joined` terminal this arm needs would never be reached.
    let wt = worktree_dir(repo.path(), ACTIVE_SUB);
    fs::write(wt.join("landed.rs"), "pub fn landed() {}\n").expect("write worktree code");
    git(&wt, &["add", "landed.rs"]);

    assert_ok(
        &run_milestone(repo.path(), home.path(), &["finalize", MILESTONE]),
        "`jigc milestone finalize` (the sub-tasks genuinely join)",
    );
    assert_eq!(
        header_status(&read_record(repo.path())),
        "joined",
        "precondition: the record settled to the landed terminal",
    );
    assert!(
        !milestone_area(repo.path()).exists(),
        "precondition: a landed finalize removes the milestone area",
    );

    assert_every_verb_refuses(repo.path(), home.path(), "joined");
}

/// (RED-vii) **The identity door.** `milestone create` mints against the **workbench** collision
/// check (`.jigc/milestones/<id>/` exists) — which a terminal op has just removed — so re-creating
/// a settled milestone's title **overwrote its committed record** with a fresh `active` one,
/// destroying a settled record (a *landed* one, in the `joined` case) with no warning, exit 0. The
/// committed record — not the disposable cache — owns a milestone's identity, so `create` refuses
/// when a record already owns the id.
#[test]
fn create_refuses_when_a_committed_record_already_owns_the_id() {
    let repo = TempDir::new("terminal-create");
    let home = TempDir::new("home");
    init_repo(repo.path());
    write_compose_marker(repo.path());
    setup_live_milestone(repo.path(), home.path());

    assert_ok(
        &run_milestone(repo.path(), home.path(), &["discard", MILESTONE, "--force"]),
        "`jigc milestone discard --force`",
    );
    let settled = read_record(repo.path());
    let pre_count = commit_count(repo.path());

    let out = run_milestone(repo.path(), home.path(), &["create", "Cache rework"]);
    assert!(
        !out.status.success(),
        "re-creating a settled milestone's id must REFUSE; got exit 0\nstdout:\n{}",
        String::from_utf8_lossy(&out.stdout),
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains(MILESTONE) && stderr.contains("discarded"),
        "the refusal names the id and the record's status; stderr:\n{stderr}",
    );
    assert_eq!(
        read_record(repo.path()),
        settled,
        "the settled record is NOT overwritten by a fresh mint",
    );
    assert_eq!(
        commit_count(repo.path()),
        pre_count,
        "the refused create commits nothing",
    );
    assert!(
        !milestone_area(repo.path()).exists(),
        "the refused create mints no workbench",
    );
}
