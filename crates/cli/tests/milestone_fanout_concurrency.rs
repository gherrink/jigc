//! **The concurrent N-process fan-out → join determinism pin** — the standing form of
//! the fact every wave re-established by hand (the confidence-audit back-sweep's c1,
//! the one genuine coverage hole the M39–M44 triage found): N *concurrent* OS
//! processes driving the real binary through provisioned worktree sub-tasks
//! (compose → doc writes → staged code, × N, divergent completion orders) join into
//! one deterministic, by-task-id, byte-stable commit with no cross-worktree
//! contamination.
//!
//! **Provenance (pinning.md §3 — the fact was hand-established three times, held by
//! nothing):** the M39 e2e auditor's N-process binary sim + verbatim spawn-template
//! (`completions/artifacts/M39/VERDICT.md` → Honest bounds), re-proven one-off by the
//! M43 e2e audit (`completions/artifacts/M43/VERDICT.md` → the e2e paragraph: "the
//! N-process fan-out sim + verbatim spawn-template execution + by-task-id join
//! determinism hold"), live-confirmed at the project-alpha-3.0 trial (5 parallel
//! worktrees, 2026-07-22, `completions/artifacts/RC-alpha3/trial-record.md`).
//! Standing coverage was sequential-only (`flow9_milestone_join.rs` varies the
//! *recorded feed order* in one process; nothing spawned concurrent jigc processes
//! against one milestone). The invariant under pin is architectural: "the CLI merges
//! at a join ordered **by task ID, not completion order**" (CLAUDE.md → Concurrency
//! is one bounded primitive).
//!
//! **The concurrency mechanism shipped, stated honestly:** each fixture spawns the
//! three sub-agent child processes (a `sh` script driving the built binary exactly as
//! the shipped spawn template directs — `cd .jigc/worktrees/<sub>` then `jigc …`)
//! and holds them at a barrier until all three are up; the **compose phase then runs
//! genuinely simultaneously** (three live `jigc` processes sharing one repo's
//! `.jigc/`). The **write/completion phase is explicitly sequenced** by per-child go
//! tokens released in the arm's chosen order — run A completes low→mid→zed, run B
//! zed→low→mid — so the divergent completion orders are deterministic, not sampled.
//! True write-phase simultaneity cannot reproduce a *specific* completion order (the
//! thing the order-invariance assertion needs two of), so the pin combines genuine
//! N-process simultaneity (compose) with explicitly-sequenced divergent completion
//! orders (writes) — the charter's sanctioned form.
//!
//! **Fixture discipline:** every fixture is built **fresh through the binary** —
//! never copied — because a copied fixture carrying provisioned worktrees reads and
//! writes the *source* fixture through the absolute paths in `.git/worktrees/*/gitdir`
//! (pinning.md §4's worktree caveat). Git commit dates are pinned via
//! `GIT_AUTHOR_DATE`/`GIT_COMMITTER_DATE` so the three repos' pre-milestone history
//! (and therefore the `base:` sha the committed milestone record embeds) is
//! byte-identical, making whole-tree-hash comparison across repos meaningful.
//!
//! The fixture forces genuine overlap: `area-low` and `area-zed` both create
//! `adr:cache-strategy` (distinguishable bodies — "eager" vs "lazy"), so the join's
//! suffix-by-task-id decision is observable in the committed tree. A join keyed on
//! completion order (or staged-file mtime, its on-disk proxy) would hand the bare
//! slug to whichever collider completed first and diverge run B from run A —
//! verified red under exactly that local mutation of `materialize`'s group sort
//! (mutate → catch → restore; never committed).

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Child, Command};
use std::time::{Duration, Instant};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-fanout-conc-{tag}-{}-{:?}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
        );
        path.push(unique);
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

/// The pinned commit date every git-writing process runs under, so the three
/// fixtures' pre-milestone commits (initial + setup + record commits) hash
/// identically and the milestone record's embedded `base:` sha matches across repos.
const PINNED_DATE: &str = "2026-01-01T12:00:00 +0000";

/// The three sub-task ids, in canonical id order.
const SUBS: [&str; 3] = ["area-low", "area-mid", "area-zed"];

/// Per-sub-task authoring inputs: (title, slug, flavor). `area-low` and `area-zed`
/// deliberately collide on `adr:cache-strategy` with distinguishable flavors.
fn sub_inputs(sub: &str) -> (&'static str, &'static str, &'static str) {
    match sub {
        "area-low" => ("Cache strategy", "cache-strategy", "eager"),
        "area-mid" => ("Eviction policy", "eviction-policy", "mid"),
        "area-zed" => ("Cache strategy", "cache-strategy", "lazy"),
        other => panic!("unknown sub-task {other}"),
    }
}

/// Run `git <args>` in `repo` under the pinned dates and the fixture `home`,
/// asserting success. `$HOME` is redirected so the developer's global git config
/// (e.g. `commit.gpgsign` — whose signature embeds its own wall-clock timestamp and
/// would diverge otherwise-identical commits) can never reach the fixture repos.
fn git(repo: &Path, home: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("GIT_AUTHOR_DATE", PINNED_DATE)
        .env("GIT_COMMITTER_DATE", PINNED_DATE)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).expect("utf-8 git stdout")
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, pinned dates, asserting exit 0.
fn jigc_ok(repo: &Path, home: &Path, args: &[&str]) {
    let out = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("GIT_AUTHOR_DATE", PINNED_DATE)
        .env("GIT_COMMITTER_DATE", PINNED_DATE)
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("run the jigc binary");
    assert!(
        out.status.success(),
        "`jigc {args:?}` must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// The sub-agent child script — the real per-worktree sequence the shipped spawn
/// template directs, driven through the built binary (`jigc` resolved off `$PATH`,
/// which the parent prepends the built binary's directory to). Two barriers:
/// `compose-go` (shared — released once all three children are up, so the three
/// compose invocations run genuinely simultaneously) and `go-<sub>` (per-child — the
/// parent releases these in the arm's chosen completion order). Bounded waits so a
/// protocol bug fails loudly (exit 9) instead of hanging the suite.
const CHILD_SCRIPT: &str = r#"
set -e
cd "$WT"
touch "$SYNC/up-$SUB"
i=0; until [ -e "$SYNC/compose-go" ]; do i=$((i+1)); [ $i -gt 12000 ] && exit 9; sleep 0.005; done
jigc workflow single-task --task "$SUB" > /dev/null
touch "$SYNC/composed-$SUB"
i=0; until [ -e "$SYNC/go-$SUB" ]; do i=$((i+1)); [ $i -gt 12000 ] && exit 9; sleep 0.005; done
jigc doc create adr --title "$TITLE" --task "$SUB" > /dev/null
printf 'Context (%s).\n' "$FLAVOR" | jigc doc set-slot "adr:$SLUG#context" --from-file - --task "$SUB" > /dev/null
printf 'Decision (%s).\n' "$FLAVOR" | jigc doc set-slot "adr:$SLUG#decision" --from-file - --task "$SUB" > /dev/null
printf 'Consequences (%s).\n' "$FLAVOR" | jigc doc set-slot "adr:$SLUG#consequences" --from-file - --task "$SUB" > /dev/null
mkdir -p src
printf 'pub fn work() {}\n' > "src/$SUB.rs"
git add "src/$SUB.rs"
"#;

/// Spawn the sub-agent child process for `sub` (cwd = its provisioned worktree, the
/// built binary's dir prepended to `$PATH`, pinned dates, scrubbed pack dir).
fn spawn_child(repo: &Path, home: &Path, sync: &Path, sub: &str) -> Child {
    let (title, slug, flavor) = sub_inputs(sub);
    let bin_dir = Path::new(env!("CARGO_BIN_EXE_jigc"))
        .parent()
        .expect("the built jigc binary has a parent dir");
    let path = match std::env::var_os("PATH") {
        Some(existing) => {
            let mut dirs = vec![bin_dir.to_path_buf()];
            dirs.extend(std::env::split_paths(&existing));
            std::env::join_paths(dirs).expect("join PATH")
        }
        None => bin_dir.as_os_str().to_owned(),
    };
    Command::new("sh")
        .arg("-c")
        .arg(CHILD_SCRIPT)
        .env("WT", repo.join(".jigc").join("worktrees").join(sub))
        .env("SYNC", sync)
        .env("SUB", sub)
        .env("TITLE", title)
        .env("SLUG", slug)
        .env("FLAVOR", flavor)
        .env("HOME", home)
        .env("PATH", path)
        .env("GIT_AUTHOR_DATE", PINNED_DATE)
        .env("GIT_COMMITTER_DATE", PINNED_DATE)
        .env_remove("JIGC_PACK_DIR")
        .spawn()
        .expect("spawn the sub-agent child process")
}

/// Poll until `path` exists (5 ms period, 60 s bound) — the parent half of the
/// file-token barrier protocol.
fn wait_for(path: &Path, what: &str) {
    let deadline = Instant::now() + Duration::from_secs(60);
    while !path.exists() {
        assert!(
            Instant::now() < deadline,
            "timed out waiting for {what} ({})",
            path.display(),
        );
        std::thread::sleep(Duration::from_millis(5));
    }
}

/// Reap `child`, asserting it exited 0 (a `9` is a barrier-protocol timeout).
fn reap(mut child: Child, sub: &str) {
    let status = child.wait().expect("wait for the sub-agent child");
    assert!(
        status.success(),
        "the {sub} sub-agent child must exit 0; got {status:?}",
    );
}

/// One landed fixture's committed observables.
struct Landed {
    /// The tempdir keeping the repo alive for post-hoc reads.
    dir: TempDir,
    /// HEAD's full commit message (`git log -1 --format=%B`).
    message: String,
    /// HEAD's committed tree hash (`git rev-parse HEAD^{tree}`).
    tree: String,
    /// The finalize commit's own changed-path set (`git show --name-only`).
    committed_paths: BTreeSet<String>,
}

impl Landed {
    fn repo(&self) -> PathBuf {
        self.dir.path().join("repo")
    }
}

/// Build the fan-out fixture **fresh through the binary** (never copied —
/// pinning.md §4's worktree caveat), run the three sub-agent children, and finalize.
///
/// `concurrent = true`: all three children are spawned first, barriered until all
/// are up, composed simultaneously, then completed one at a time in `order`.
/// `concurrent = false` (the serial baseline): each child runs start-to-finish alone,
/// in `order`, with its tokens pre-granted — zero overlap anywhere.
fn build_and_finalize(tag: &str, order: [&str; 3], concurrent: bool) -> Landed {
    assert_eq!(
        order.iter().collect::<BTreeSet<_>>(),
        SUBS.iter().collect::<BTreeSet<_>>(),
        "the completion order must be a permutation of the sub-task id set",
    );

    let dir = TempDir::new(tag);
    let repo = dir.path().join("repo");
    let home = dir.path().join("home");
    let sync = dir.path().join("sync");
    for d in [&repo, &home, &sync] {
        fs::create_dir_all(d).expect("create fixture dir");
    }

    // The base repo + jigc workspace, all commits under the pinned dates.
    git(&repo, &home, &["init", "-q"]);
    git(&repo, &home, &["config", "user.email", "test@example.com"]);
    git(&repo, &home, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write README");
    git(&repo, &home, &["add", "."]);
    git(&repo, &home, &["commit", "-q", "-m", "initial"]);
    jigc_ok(&repo, &home, &["setup"]);

    // The milestone: create → 3 × add-task (id order) → provision the worktrees.
    jigc_ok(&repo, &home, &["milestone", "create", "Cache rework"]);
    for intent in ["Area low", "Area mid", "Area zed"] {
        jigc_ok(
            &repo,
            &home,
            &[
                "milestone",
                "add-task",
                "cache-rework",
                intent,
                "--workflow",
                "single-task",
            ],
        );
    }
    jigc_ok(&repo, &home, &["milestone", "provision", "cache-rework"]);

    if concurrent {
        // Spawn all three; hold them until all are up (three live processes at
        // once), release the compose phase simultaneously, then complete them one
        // at a time in the arm's order.
        let mut children: Vec<(&str, Child)> = SUBS
            .iter()
            .map(|sub| (*sub, spawn_child(&repo, &home, &sync, sub)))
            .collect();
        for sub in SUBS {
            wait_for(&sync.join(format!("up-{sub}")), "child up");
        }
        fs::write(sync.join("compose-go"), b"").expect("release the compose barrier");
        for sub in SUBS {
            wait_for(&sync.join(format!("composed-{sub}")), "concurrent compose");
        }
        for sub in order {
            fs::write(sync.join(format!("go-{sub}")), b"").expect("release the write barrier");
            let pos = children
                .iter()
                .position(|(id, _)| *id == sub)
                .expect("child spawned");
            let (_, child) = children.remove(pos);
            reap(child, sub);
        }
    } else {
        // The serial baseline: tokens pre-granted, one child at a time, zero overlap.
        fs::write(sync.join("compose-go"), b"").expect("pre-grant the compose barrier");
        for sub in order {
            fs::write(sync.join(format!("go-{sub}")), b"").expect("pre-grant the write barrier");
            let child = spawn_child(&repo, &home, &sync, sub);
            reap(child, sub);
        }
    }

    jigc_ok(&repo, &home, &["milestone", "finalize", "cache-rework"]);

    let message = git(&repo, &home, &["log", "-1", "--format=%B"]);
    let tree = git(&repo, &home, &["rev-parse", "HEAD^{tree}"])
        .trim()
        .to_string();
    let committed_paths = git(&repo, &home, &["show", "--name-only", "--format=", "HEAD"])
        .lines()
        .filter(|l| !l.is_empty())
        .map(str::to_owned)
        .collect();
    Landed {
        dir,
        message,
        tree,
        committed_paths,
    }
}

/// **The pin.** Two genuinely-concurrent runs under divergent explicit completion
/// orders (A: low→mid→zed, B: zed→low→mid) and one serial baseline land
/// byte-identical committed state — message and whole tree — with the sub-tasks
/// ordered by task id, the collision suffix assigned by task id (never completion
/// order / mtime), and no write escaping its keyed area.
#[test]
fn concurrent_fanout_join_is_byte_identical_across_completion_orders_and_serial() {
    let order_a = ["area-low", "area-mid", "area-zed"];
    let order_b = ["area-zed", "area-low", "area-mid"];
    assert_ne!(order_a, order_b, "the completion orders must diverge");

    let run_a = build_and_finalize("order-a", order_a, true);
    let run_b = build_and_finalize("order-b", order_b, true);
    let serial = build_and_finalize("serial", order_a, false);

    // (a) The joined commit is byte-identical across divergent completion orders and
    // identical to the serial baseline — message and whole committed tree.
    assert_eq!(
        run_a.message, run_b.message,
        "the committed message must be byte-identical under divergent completion \
         orders;\nA:\n{}\nB:\n{}",
        run_a.message, run_b.message,
    );
    assert_eq!(
        run_a.message, serial.message,
        "the concurrent run's message must equal the serial baseline's;\n\
         concurrent:\n{}\nserial:\n{}",
        run_a.message, serial.message,
    );
    assert_eq!(
        run_a.tree, run_b.tree,
        "the committed tree must be byte-identical under divergent completion orders \
         ({} vs {})",
        run_a.tree, run_b.tree,
    );
    assert_eq!(
        run_a.tree, serial.tree,
        "the concurrent run's tree must equal the serial baseline's ({} vs {})",
        run_a.tree, serial.tree,
    );

    // (b) The join orders by task id, not completion order: the synthesized message
    // lists the sub-tasks in id order — including in run B, where the completion
    // order was zed→low→mid.
    let positions: Vec<usize> =
        SUBS.iter()
            .map(|sub| {
                run_b.message.find(&format!("- {sub}")).unwrap_or_else(|| {
                    panic!("the message must list {sub}; got:\n{}", run_b.message)
                })
            })
            .collect();
    assert!(
        positions.windows(2).all(|w| w[0] < w[1]),
        "the message must list sub-tasks in task-id order regardless of completion \
         order; got:\n{}",
        run_b.message,
    );

    // (b, the sharp half) The collision suffix is assigned by task id: the lower
    // collider (`area-low`, "eager") keeps the bare slug even in run B, where it
    // completed AFTER `area-zed` — a completion-order/mtime-keyed suffix would land
    // "lazy" at the bare slug there. Read off run B's committed tree.
    let decisions = run_b.repo().join("docs").join("decisions");
    let bare = fs::read_to_string(decisions.join("cache-strategy.md")).expect("read bare");
    assert!(
        bare.contains("Context (eager)."),
        "the bare slug must carry the lower task id's body (eager) even when it \
         completed last; got:\n{bare}",
    );
    let suffixed = fs::read_to_string(decisions.join("cache-strategy-2.md")).expect("read -2");
    assert!(
        suffixed.contains("Context (lazy)."),
        "the `-2` suffix must carry the higher task id's body (lazy); got:\n{suffixed}",
    );

    // (c) No worktree write escapes its keyed area: the finalize commit's changed
    // set is exactly the three promoted docs + the flipped milestone record + the
    // three per-worktree code files — nothing more, nothing missing.
    let expected: BTreeSet<String> = [
        "docs/decisions/cache-strategy.md",
        "docs/decisions/cache-strategy-2.md",
        "docs/decisions/eviction-policy.md",
        "docs/milestone-records/cache-rework.md",
        "src/area-low.rs",
        "src/area-mid.rs",
        "src/area-zed.rs",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect();
    for landed in [&run_a, &run_b, &serial] {
        assert_eq!(
            landed.committed_paths, expected,
            "the finalize commit must change exactly the joined set — an extra path \
             is an escaped write, a missing one a dropped area",
        );
    }
}
