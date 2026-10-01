//! **The M46 reconciliation-wave done-picture acceptance suite** — the wave driven
//! end-to-end through the **real `jigc` binary** (`design/worked-examples.md` → flow 49;
//! roadmap → Milestone 46, Increment 10).
//!
//! **The claim the wave proves is one claim: every fix reconciles a contradiction the
//! codebase already carries — a rule stated in one place and violated in another.** The
//! razor that admitted each fix is a three-leg test (a rule **stated** in a locked
//! artifact · **violated** at HEAD · **demonstrable** by driving the binary), and it is
//! falsifiable by construction: it refused nine items with citations before the build
//! rather than during it ([razor-ledger.md](../../../completions/artifacts/M46/razor-ledger.md)).
//! Increments 1–9 shipped each fix with its own axis suite; this suite is the
//! **composite acceptance** that ties the wave into six done-picture arms over the real
//! binary — **each arm enumerating its cell set from a code-side registry or from the
//! class's defining case-set**, never pinning the single repro a trial reported.
//!
//! The six arms — **ten `#[test]`s** over the real binary, since four of them carry a
//! second cell that needs its own fixture world:
//!
//!   (1) **The write seam reports no success for a write it discarded** — the cell set is
//!       the record's **complete mutator set** ([`engine::file_state::FileStateRecord`]
//!       exposes exactly two `&mut self` methods, `record` and `forget`, both named in
//!       code here so a rename cannot leave this arm silently narrow), driven by
//!       **concurrent `jigc` processes**, plus the **post-sweep hand-off** — the third
//!       cell, which exists only through the binary, across a real `git commit`, with a
//!       real hook process in the middle. Then the **arm-A/arm-B contrast** that makes
//!       the cost observable: with the baseline present an out-of-band conflict is
//!       detected and blocked at exit 3; with it lost the same drift is silently merged
//!       at exit 0 (Inc 1).
//!
//!       **This is a defining case-set, not a production table, and the chapter says so.**
//!       There is no code-side array of mutation kinds; claiming one would be the law-1
//!       defect this wave exists to remove, one file later ([pinning.md] §5 — classify
//!       from the code, never from the artifact in hand).
//!
//!   (2) **No destroying door destroys bytes it does not name, and none takes a path it
//!       never looked at** — the axis is the code-side [`cli::milestone::DESTROYING_DOORS`]
//!       table read through its own **disposition**
//!       ([`cli::milestone::DestroyingDoor::disposition`], an `Option<code>` until M52
//!       Increment 4 generalized the subject from a worktree-shaped path to a destroyed
//!       one): every door either **refuses** what it cannot prove is disposable, *or*
//!       **names every byte it destroys**, *or* **keeps the bytes and says where they
//!       went**, and silence is the one arm no door may take. Driven over the **`--ignored`
//!       axis** — a path no commit could ever carry, which every door reaches exactly as
//!       hard as tracked bytes. Plus the **path-subject** cell: the milestone boundary
//!       reads the worktree **on disk**, not the registered set (Inc 2).
//!
//!   (3) **A never-adopted foreign file is not the corpus migration's subject, and both
//!       doors say so in the same words** — the subject set is a **derivation**, stated as
//!       one: `engine::validate::is_unadopted_foreign`'s call sites (the store sweep's
//!       fifth family, the `AdoptionInputs::unadopted` seam, the read-side reroute,
//!       `doc list`'s registration state, the task-scope reconciler) **× the two home
//!       kinds** a managed doc can have (a `placement` literal, a `location:` home). The
//!       advisory is asserted **byte-identical** at `jigc validate` and
//!       `jigc migrate-corpus`, and [`cli::render::STORE_EXIT_FLIPS`]' fifth member flips
//!       the sweep's exit so the exclusion is never a false green (Inc 3).
//!
//!   (4) **No route promises a re-run that changes nothing, and every halt names its own
//!       cause** — the cell set is [`engine::transform::HaltReason`]'s three variants,
//!       matched **exhaustively** so a fourth cause cannot compile without deciding what
//!       it says; the two binary-reachable ones are driven here and the third is
//!       classified explicitly rather than silently skipped. Riding with it: a `set:`
//!       field's conforming absence folds to a **byte** no-op and the report names the
//!       fields it left unfilled. And the **pre-guard repair route is run to green from
//!       the repo where the corruption happened**, not only from a fresh clone
//!       (Inc 4 + Inc 5).
//!
//!   (5) **The gate previews what it gates on, and a diagnosis states the comparison it
//!       made** — the axis is [`cli::gate_coverage::GATE_COVERAGE`] filtered to
//!       [`cli::gate_coverage::Tier::Previewed`], every member either driven live at
//!       `jigc task validate` or classified explicitly against the suite that drives it;
//!       the changelog member is additionally shown **item-count-invariant** (a committed
//!       category's `notes` rewritten suppresses the advisory though the item count never
//!       moves); and `doc-code.criterion-maps-to-test` over a closure-framework citation
//!       names the **file-only fallback** instead of asserting an absence its own parsed
//!       bytes contradict (Inc 6 + Inc 7).
//!
//!   (6) **The seven admitted surfaces answer at the door that prints them** — and **there
//!       is no registry here**. The cell set is the razor ledger's §1 admitted set, carried
//!       by hand deliberately: a surface batch has no production table, and minting one
//!       would be scaffolding rather than a fix (Inc 8).
//!
//! **Increment 9 deliberately gets no arm.** Its four commits touch `DECISIONS.md`,
//! `implementation/decisions-pending.md`, `implementation/doctype-map.md` and two new
//! suites — no verb, no finding, no route for a done-picture walk to reach. The wave
//! records the absence rather than manufacturing a walk (the M48 Increment 11 precedent).
//!
//! **The declared proof split.** Each arm proves the wave's claim at the *done-picture*
//! altitude; the per-fix mechanism clauses stay with the dedicated axis suites and are not
//! re-proven here: the merge's forced interleave and its lock's wait budget are
//! `engine::file_state`'s unit axis and `file_state_concurrency.rs`'s, the hand-off's
//! non-vacuity `file_state_merge_hand_off.rs`'s and the contrast's applied mutant
//! `reconciliation_baseline_contrast.rs`'s; the per-verdict leftover fixtures and the
//! `WORKTREE_DOORS × LEFTOVER_VERDICTS` refusal matrix are `provision_leftover_guard.rs`'s,
//! `uninstall_worktree_guard.rs`'s and `flow48_acceptance.rs`'s, the four manifest cells
//! `milestone_path_subject.rs`'s; the foreign axis's two fixture worlds are
//! `migrate_corpus_foreign.rs`'s; the halt-cause variant map is
//! `migrate_corpus_halt_causes.rs`'s and the `set:` locus axis
//! `migrate_corpus_set_fields.rs`'s, the three carriers of the unanchored-heading
//! diagnosis `pre_guard_repair_route.rs`'s; the whole `doc` write-verb axis of the
//! changelog write-touch is `changelog_write_touch.rs`'s, the eight-site coverage fence
//! `gate_coverage_fence.rs`'s, and the probe's wire bytes `doc_code_probe.rs`'s; each
//! admitted surface's own arms live with the suites Increment 8 revised.
//!
//! **Red at the wave's base**, arm by arm — each line below is the behaviour the
//! increment's own `DECISIONS.md` entry recorded as *driven at HEAD* before its fix, not a
//! re-run of this suite against an older binary: a concurrent `save` discarded a sibling
//! writer's per-key delta and reported success, and finalize's phase-7 hand-off resurrected
//! a baseline the pre-commit hook had just retired; `milestone finalize` printed
//! `no worktree provisioned` over a directory holding the sub-agent's work and landed a
//! docs-only commit at exit 0, while two of the four destroying doors named nothing at all;
//! one never-adopted foreign file drew an advisory routed at `jigc ingest` from
//! `jigc validate` and a blocking `migrate-corpus.prose-needed` routed at a re-run that
//! changes nothing from `jigc migrate-corpus`; all three halt causes rendered as
//! *"mints a new **required** prose slot"* and a `set:` field's conforming absence was
//! refused outright; the changelog gate ran on the committing path only and counted
//! repeatable items rather than writes; the closure-framework diagnosis asserted an absence
//! its own parsed bytes contradict; and each of the seven admitted surfaces said something
//! that was false, unfollowable, or self-contradicting at the door that printed it.
//!
//! Isolation: every arm builds its own throwaway repo (a real `git init`, a per-repo git
//! identity, `$HOME` repointed, `JIGC_PACK_DIR` scrubbed unless the cell deliberately
//! supplies one), or rides the shared [`support::trial_corpus`] substrate. The
//! worktree-bearing cells build **fresh per cell** — `TrialCorpus::copy_state` refuses a
//! worktree-bearing corpus by design ([pinning.md] §4: a copied `.git/worktrees/*/gitdir`
//! holds absolute paths back into the source). Cost accepted at planning.
//!
//! [pinning.md]: ../../../implementation/pinning.md

use crate::support;

use crate::support::run_then_parse::stdout_json;
use cli::gate_coverage::{self, GateCoverage, Tier};
use cli::milestone::{DESTROYING_DOORS, DestroyingDoor, Disposition, UNINSTALL_DOOR};
use cli::render::STORE_EXIT_FLIPS;
use engine::file_state::FileStateRecord;
use engine::transform::{HaltReason, TransformError};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use support::trial_corpus::{State, TrialCorpus};

// ═════════════════════════════════════════════════════════════════════════════
// Shared helpers
// ═════════════════════════════════════════════════════════════════════════════

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-flow49-{tag}-{}-{:?}",
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

/// Everything an invocation printed, both streams — the reader's view of one run.
fn printed(out: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    )
}

/// Run `git <args>` in `cwd`, asserting success.
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

/// Run `git <args>` in `cwd`, returning trimmed stdout.
fn git_out(cwd: &Path, args: &[&str]) -> String {
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
    String::from_utf8(out.stdout)
        .expect("utf-8 git stdout")
        .trim()
        .to_string()
}

/// The repo's commit count — the no-commit witness a blocked door owes.
fn commit_count(repo: &Path) -> u32 {
    git_out(repo, &["rev-list", "--count", "HEAD"])
        .parse()
        .expect("a commit count")
}

/// Initialize a real git repo with one commit plus the `.jigc/config/` project layer.
fn init_repo(repo: &Path) {
    git_ok(repo, &["init", "-q"]);
    git_ok(repo, &["config", "user.email", "test@example.com"]);
    git_ok(repo, &["config", "user.name", "Test"]);
    git_ok(repo, &["config", "commit.gpgsign", "false"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write README");
    git_ok(repo, &["add", "."]);
    git_ok(repo, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create the project layer");
}

/// Build the `jigc` invocation for `repo` / `home`, with `JIGC_PACK_DIR` scrubbed (an
/// inherited one would silently swap the pack under the arm).
fn jigc_command(repo: &Path, home: &Path, args: &[&str]) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    command
}

/// Run `jigc <args>` against `repo` with `$HOME = home`.
fn run_jigc(repo: &Path, home: &Path, args: &[&str]) -> Output {
    jigc_command(repo, home, args)
        .output()
        .expect("run the jigc binary")
}

/// Run `jigc <args>`, assert success, return stdout.
fn run_jigc_ok(repo: &Path, home: &Path, args: &[&str], what: &str) -> String {
    let out = run_jigc(repo, home, args);
    assert!(
        out.status.success(),
        "{what} must succeed; got exit {:?}:\n{}",
        out.status.code(),
        printed(&out),
    );
    String::from_utf8(out.stdout).expect("utf-8 jigc stdout")
}

/// Run `jigc <args>` with `stdin` piped.
fn run_jigc_stdin(repo: &Path, home: &Path, args: &[&str], stdin: &[u8]) -> Output {
    let mut child = jigc_command(repo, home, args)
        .stdin(Stdio::piped())
        .spawn()
        .expect("spawn jigc");
    child
        .stdin
        .take()
        .expect("stdin piped")
        .write_all(stdin)
        .expect("write jigc stdin");
    child.wait_with_output().expect("wait for jigc")
}

/// Run `jigc <args>` with `stdin` piped, asserting success.
fn run_jigc_stdin_ok(repo: &Path, home: &Path, args: &[&str], stdin: &[u8], what: &str) -> String {
    let out = run_jigc_stdin(repo, home, args, stdin);
    assert!(
        out.status.success(),
        "{what} must succeed; got exit {:?}:\n{}",
        out.status.code(),
        printed(&out),
    );
    String::from_utf8(out.stdout).expect("utf-8 jigc stdout")
}

/// The task id the binary **printed** it minted — read from the real output, never
/// reconstructed from the intent (the slug rule is the binary's).
fn minted_task(stdout: &str) -> String {
    stdout
        .lines()
        .find_map(|line| line.strip_prefix("task minted: "))
        .unwrap_or_else(|| panic!("the mint must print `task minted: <id>`; got:\n{stdout}"))
        .trim()
        .to_string()
}

/// Fill every author-required field/slot of the provisioned `commit` doc.
fn fill_commit(repo: &Path, home: &Path, task: &str, summary: &str) {
    for (address, value) in [
        (format!("commit:{task}#type"), "feat"),
        (format!("commit:{task}#scope"), "cache"),
    ] {
        run_jigc_ok(
            repo,
            home,
            &["doc", "set-field", &address, "--value", value],
            &format!("set-field {address}"),
        );
    }
    for (address, prose) in [
        (format!("commit:{task}#summary"), summary),
        (format!("commit:{task}#body"), "A cache change."),
    ] {
        run_jigc_stdin_ok(
            repo,
            home,
            &["doc", "set-slot", &address, "--from-file", "-"],
            format!("{prose}\n").as_bytes(),
            &format!("set-slot {address}"),
        );
    }
}

/// The persisted file-state map, as `path → hash`.
fn file_state(repo: &Path) -> BTreeMap<String, String> {
    let path = repo.join(".jigc").join("state").join("file-state.json");
    let bytes = fs::read(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    let value: Value = serde_json::from_slice(&bytes).expect("file-state.json parses");
    value
        .get("hashes")
        .and_then(|hashes| hashes.as_object())
        .expect("file-state.json carries a `hashes` object")
        .iter()
        .map(|(key, hash)| (key.clone(), hash.as_str().unwrap_or_default().to_string()))
        .collect()
}

/// A conformant, current-stamp `adr` body.
fn adr_body(title: &str) -> String {
    format!(
        "---\nstatus: accepted\ndate: 2026-06-25\nschema-version: 2\n---\n\n\
         # {title}\n\n## Context\n\nSession lookups must stay sub-millisecond.\n\n\
         ## Options\n\nA distributed cache was weighed and rejected on latency.\n\n\
         ## Decision\n\nKeep sessions in a single in-memory node.\n\n\
         ## Consequences\n\nA cold node loses its sessions.\n"
    )
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 1 — the write seam reports no success for a write it discarded (Inc 1)
// ═════════════════════════════════════════════════════════════════════════════

/// The committed managed doc whose baseline the `pre-commit` hook retires mid-commit —
/// the subject of the post-sweep hand-off cell. The task below never opens it.
const HAND_OFF_SUBJECT: &str = "docs/decisions/planted-baseline.md";

/// The three plain files a first task commits, so the record carries keys `jigc ingest`
/// can never re-adopt (the candidate walk is `*.md`): two are forgotten concurrently, the
/// third is the untouched control.
const CONCURRENT_KEYS: [&str; 3] = ["alpha.txt", "beta.txt", "gamma.txt"];

/// The freshly committed, never-adopted ADR the concurrent `jigc ingest` **records** —
/// the record-side mutation, in its own process, against a key no `unmanage` touches.
const CONCURRENT_ADOPTED: &str = "docs/decisions/adopt-me.md";

/// Name the record's **complete mutator set** from the code, so a renamed or added
/// `&mut self` method cannot leave this arm silently narrow.
///
/// [`FileStateRecord`] exposes exactly two: [`FileStateRecord::record`] and
/// [`FileStateRecord::forget`]. There is no code-side table of mutation kinds — the honest
/// label is the class's *defining case-set*, and this call is what binds it to the type.
fn the_records_two_mutators_are_named_here() {
    let mut probe = FileStateRecord::new();
    probe.record("docs/decisions/probe.md".to_string(), "0".repeat(64));
    assert!(
        probe.forget("docs/decisions/probe.md"),
        "the case-set is `record` × `forget` — the record's whole mutator surface",
    );
}

/// Install a `pre-commit` hook that runs `jigc unmanage <path>` — a shipped,
/// register-only, git-index-free record writer — so a whole other process retires a
/// baseline while `jigc task finalize` holds its own long-lived copy of the same record.
fn install_unmanage_hook(repo: &Path, subject: &str) {
    let hooks = repo.join(".git").join("hooks");
    fs::create_dir_all(&hooks).expect("mk hooks");
    let hook = hooks.join("pre-commit");
    fs::write(
        &hook,
        format!(
            "#!/bin/sh\n\"{}\" unmanage {subject} || exit 1\nexit 0\n",
            env!("CARGO_BIN_EXE_jigc"),
        ),
    )
    .expect("write the pre-commit hook");
    fs::set_permissions(&hook, fs::Permissions::from_mode(0o755)).expect("chmod hook");
}

/// **Arm 1, cells 1–3.** The record's whole mutator set survives a concurrent sibling
/// writer — driven by **separate `jigc` processes**, not by threads in one — and the
/// **post-sweep hand-off**, the cell that exists only through the binary.
///
/// *Cell A/B (the concurrent processes).* A first task commits three plain files, so the
/// record carries three keys the `*.md` ingest walk can never re-adopt; a fourth,
/// conformant ADR is committed but never adopted. Then three processes are spawned
/// **before any is waited on**: two `jigc unmanage` (the **forget** kind, disjoint keys)
/// and one `jigc ingest` (the **record** kind, adopting the ADR). Every one reports
/// success, and every one's decision is on disk afterwards: before the base-relative
/// merge, whichever process saved last wrote its own loaded map verbatim and silently
/// discarded the others — a write that reported success for a write it had discarded.
///
/// *Cell C (the hand-off).* `task finalize` loads the record in its preflight sweep,
/// carries it **by value** through staging and the `git commit` — during which git runs a
/// `pre-commit` hook, a whole other process writing the same shared file — and only saves
/// it in phase 7. The hook retires a bystander doc's baseline; after the finalize lands,
/// that retirement must **stay**.
#[test]
fn the_write_seam_reports_no_success_for_a_write_it_discarded() {
    the_records_two_mutators_are_named_here();

    // ── Cells A + B — the mutator set, under concurrent processes ──────────────
    let repo = TempDir::new("concurrent");
    let home = TempDir::new("concurrent-home");
    init_repo(repo.path());

    let task = minted_task(&run_jigc_ok(
        repo.path(),
        home.path(),
        &["start", "--workflow", "quick-fix", "seed the baselines"],
        "`jigc start --workflow quick-fix`",
    ));
    for rel in CONCURRENT_KEYS {
        fs::write(repo.path().join(rel), format!("{rel}\n")).expect("write the code file");
        git_ok(repo.path(), &["add", rel]);
    }
    fill_commit(repo.path(), home.path(), &task, "seed the baselines");
    run_jigc_ok(
        repo.path(),
        home.path(),
        &["task", "finalize", &task],
        "`jigc task finalize` (seeding the baselines)",
    );

    let seeded = file_state(repo.path());
    for rel in CONCURRENT_KEYS {
        assert!(
            seeded.contains_key(rel),
            "precondition: the landed finalize must record `{rel}`; record:\n{seeded:?}",
        );
    }

    // The record-side mutation's subject: committed, conformant, and **not yet adopted**,
    // so the concurrent `jigc ingest` genuinely records a new key.
    let decisions = repo.path().join("docs").join("decisions");
    fs::create_dir_all(&decisions).expect("mk docs/decisions/");
    fs::write(decisions.join("adopt-me.md"), adr_body("Adopt me")).expect("write the adr");
    git_ok(repo.path(), &["add", "docs"]);
    git_ok(
        repo.path(),
        &["commit", "-q", "-m", "plant an adr to adopt"],
    );
    assert!(
        !file_state(repo.path()).contains_key(CONCURRENT_ADOPTED),
        "precondition: the planted adr must still be un-adopted",
    );

    // Every process spawned before any is waited on — the concurrency is real, not a
    // sequence dressed up as one.
    let mut children: Vec<(String, Child)> = Vec::new();
    for rel in [CONCURRENT_KEYS[0], CONCURRENT_KEYS[1]] {
        children.push((
            format!("jigc unmanage {rel}"),
            jigc_command(repo.path(), home.path(), &["unmanage", rel])
                .spawn()
                .expect("spawn `jigc unmanage`"),
        ));
    }
    children.push((
        "jigc ingest".to_string(),
        jigc_command(repo.path(), home.path(), &["ingest"])
            .spawn()
            .expect("spawn `jigc ingest`"),
    ));
    for (what, child) in children {
        let out = child
            .wait_with_output()
            .expect("wait for the sibling writer");
        assert!(
            out.status.success(),
            "`{what}` ran concurrently and must report success; got:\n{}",
            printed(&out),
        );
    }

    let after = file_state(repo.path());
    for rel in [CONCURRENT_KEYS[0], CONCURRENT_KEYS[1]] {
        assert!(
            !after.contains_key(rel),
            "the **forget** kind: `jigc unmanage {rel}` reported success, so `{rel}`'s \
             baseline must be gone — a sibling writer's save may not resurrect it; \
             record:\n{after:?}",
        );
    }
    assert!(
        after.contains_key(CONCURRENT_ADOPTED),
        "the **record** kind: the concurrent `jigc ingest` reported success, so its \
         adoption must be on disk — a sibling writer's save may not drop it; \
         record:\n{after:?}",
    );
    assert!(
        after.contains_key(CONCURRENT_KEYS[2]),
        "the control key nobody touched must survive both writers; record:\n{after:?}",
    );

    // ── Cell C — the post-sweep hand-off ───────────────────────────────────────
    let repo = TempDir::new("handoff");
    let home = TempDir::new("handoff-home");
    init_repo(repo.path());

    let decisions = repo.path().join("docs").join("decisions");
    fs::create_dir_all(&decisions).expect("mk docs/decisions/");
    fs::write(
        decisions.join("planted-baseline.md"),
        adr_body("Planted baseline"),
    )
    .expect("write the subject adr");
    git_ok(repo.path(), &["add", "docs"]);
    git_ok(
        repo.path(),
        &["commit", "-q", "-m", "plant the subject adr"],
    );
    run_jigc_ok(
        repo.path(),
        home.path(),
        &["ingest"],
        "`jigc ingest` (baseline the hand-off subject)",
    );
    assert!(
        file_state(repo.path()).contains_key(HAND_OFF_SUBJECT),
        "precondition: the subject must carry a baseline before the finalize starts",
    );

    install_unmanage_hook(repo.path(), HAND_OFF_SUBJECT);

    let task = minted_task(&run_jigc_ok(
        repo.path(),
        home.path(),
        &["start", "--workflow", "quick-fix", "add rate limiter"],
        "`jigc start --workflow quick-fix`",
    ));
    let code = format!("{task}.txt");
    fs::write(repo.path().join(&code), "the code change\n").expect("write the code change");
    git_ok(repo.path(), &["add", &code]);
    fill_commit(repo.path(), home.path(), &task, "add rate limiter");
    let landed = run_jigc_ok(
        repo.path(),
        home.path(),
        &["task", "finalize", &task],
        "`jigc task finalize` (the hand-off)",
    );

    // Non-vacuity: the subject is outside the landed commit and outside the plan's own
    // promoted set, so phase 7 cannot have re-recorded it on its own account. The
    // manifest is finalize's own speech, so the relayed hook stream is cut off first.
    let committed = git_out(
        repo.path(),
        &["diff-tree", "--no-commit-id", "-r", "--name-only", "HEAD"],
    );
    assert!(
        !committed.lines().any(|path| path == HAND_OFF_SUBJECT),
        "the landed commit must not touch `{HAND_OFF_SUBJECT}`; committed:\n{committed}",
    );
    let manifest = landed
        .split("--- hook output ---")
        .next()
        .expect("stdout has a first segment");
    assert!(
        !manifest.contains(HAND_OFF_SUBJECT),
        "the finalize must not promote or name the bystander doc; manifest:\n{manifest}",
    );

    let after = file_state(repo.path());
    assert!(
        after.contains_key(&code),
        "phase 7 must have advanced the record for the task's own committed code file — \
         otherwise the cell passes on a save that never happened; record:\n{after:?}",
    );
    assert!(
        !after.contains_key(HAND_OFF_SUBJECT),
        "the hook retired `{HAND_OFF_SUBJECT}`'s baseline mid-commit; the record finalize \
         had been holding since its preflight must merge that retirement, never \
         resurrect the key; record:\n{after:?}",
    );
}

/// The prose the *human* writes out of band, straight into the committed file.
const HUMAN_PROSE: &str = "A cold node loses its sessions; clients re-authenticate.";

/// The prose the *task* writes through the CLI, into a different slot of the same doc.
const TASK_PROSE: &str = "A single in-memory node keeps lookups fast, for now.";

/// The committed ADR both arms of the contrast drift.
const CONTRAST_ADR: &str = "docs/decisions/single-node-cache.md";

/// What one arm of the contrast observed.
struct ContrastOutcome {
    exit: i32,
    codes: Vec<String>,
    commits_before: u32,
    commits_after: u32,
    committed_adr: String,
    baseline_at_finalize: bool,
}

/// Drive one arm of the contrast. **One branch** — whether the committed ADR's baseline
/// is present when `jigc task finalize` runs — so the difference the pair asserts cannot
/// come from anywhere else.
fn drive_contrast(baseline_present: bool) -> ContrastOutcome {
    let repo = TempDir::new(if baseline_present { "armA" } else { "armB" });
    let home = TempDir::new("contrast-home");
    let repo = repo.path();
    let home = home.path();
    init_repo(repo);

    // Task 0 — author + finalize the ADR. Its landed finalize posts the committed
    // baseline, which is the precondition arm B removes.
    let task0 = minted_task(&run_jigc_ok(
        repo,
        home,
        &[
            "start",
            "--workflow",
            "single-task",
            "cache sessions in a single in-memory node",
        ],
        "`jigc start` (task 0)",
    ));
    run_jigc_ok(
        repo,
        home,
        &["doc", "create", "adr", "--title", "Single-node cache"],
        "`jigc doc create adr`",
    );
    for (slot, prose) in [
        ("context", "Session lookups must stay sub-millisecond."),
        ("decision", "A single in-memory node keeps lookups fast."),
        ("consequences", "A cold node loses its sessions."),
    ] {
        run_jigc_stdin_ok(
            repo,
            home,
            &[
                "doc",
                "set-slot",
                &format!("adr:single-node-cache#{slot}"),
                "--from-file",
                "-",
            ],
            format!("{prose}\n").as_bytes(),
            &format!("set-slot #{slot}"),
        );
    }
    fill_commit(repo, home, &task0, "record the cache decision");
    run_jigc_ok(
        repo,
        home,
        &["task", "finalize", &task0],
        "`jigc task finalize` (task 0)",
    );

    // The human channel: an out-of-band, conformant, prose-only edit, committed in git
    // directly and **before** the warm task mints (so its base is this HEAD and a
    // base-mismatch cannot pre-empt the reconcile gate).
    let path = repo.join(CONTRAST_ADR);
    let body = fs::read_to_string(&path).expect("read the committed ADR");
    let edited = body.replacen("A cold node loses its sessions.", HUMAN_PROSE, 1);
    assert_ne!(body, edited, "the out-of-band edit must change the ADR");
    fs::write(&path, edited).expect("apply the out-of-band edit");
    git_ok(repo, &["add", CONTRAST_ADR]);
    git_ok(repo, &["commit", "-q", "-m", "tighten the consequences"]);

    // The warm task touches the SAME doc through the CLI, so both sides have moved.
    let warm = minted_task(&run_jigc_ok(
        repo,
        home,
        &[
            "start",
            "--workflow",
            "single-task",
            "revise the cache decision",
        ],
        "`jigc start` (warm task)",
    ));
    run_jigc_stdin_ok(
        repo,
        home,
        &[
            "doc",
            "set-slot",
            "adr:single-node-cache#decision",
            "--from-file",
            "-",
        ],
        format!("{TASK_PROSE}\n").as_bytes(),
        "set-slot #decision (warm task)",
    );
    fill_commit(repo, home, &warm, "revise the decision");

    // The one branch — arm B loses the baseline through the engine API that owns the
    // record, which is the shape a forgetting writer (or a discarded concurrent save)
    // really produces.
    if !baseline_present {
        let jigc_root = repo.join(".jigc");
        let mut record = FileStateRecord::load(&jigc_root).expect("load the record");
        assert!(
            record.forget(CONTRAST_ADR),
            "arm B's drop must remove a key that was really there",
        );
        record.save(&jigc_root).expect("persist the lost baseline");
    }

    let baseline_at_finalize = FileStateRecord::load(&repo.join(".jigc"))
        .expect("load the record")
        .get(CONTRAST_ADR)
        .is_some();
    let commits_before = commit_count(repo);
    let out = run_jigc(repo, home, &["task", "finalize", &warm, "--format", "json"]);
    let envelope: Value = stdout_json(&out, &[0, 3], "`jigc task finalize --format json`");
    let exit = out.status.code().expect("finalize exits, never signalled");
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    let codes = envelope["findings"]
        .as_array()
        .unwrap_or_else(|| panic!("the envelope carries a `findings` array; got:\n{stdout}"))
        .iter()
        .map(|finding| finding["code"].as_str().unwrap_or_default().to_string())
        .collect();

    ContrastOutcome {
        exit,
        codes,
        commits_before,
        commits_after: commit_count(repo),
        committed_adr: fs::read_to_string(repo.join(CONTRAST_ADR)).expect("read the ADR"),
        baseline_at_finalize,
    }
}

/// **Arm 1, the contrast** — what a recorded baseline is *worth*, driven as one pair
/// rather than described.
///
/// `CLAUDE.md`'s invariant says out-of-band edits are *"detected and routed … never
/// silently merged"*. That is not a property of the reconciliation state machine alone:
/// it is a property of the state machine **plus a recorded baseline**. Identical repo,
/// identical committed ADR, identical human edit, identical in-task write — the only
/// difference is whether the ADR's key is in `.jigc/state/file-state.json` when finalize
/// runs. With it: `reconciliation.conflict-block`, exit 3, no commit. Without it: exit 0
/// and a commit carrying **both** sides' prose. So a write that reported success for a
/// delta it discarded did not degrade the guarantee — it switched it off.
#[test]
fn a_lost_baseline_switches_off_the_never_silently_merged_guarantee() {
    let arm_a = drive_contrast(true);
    let arm_b = drive_contrast(false);

    assert!(
        arm_a.baseline_at_finalize && !arm_b.baseline_at_finalize,
        "the fixture's own witness: the arms must diverge exactly where they claim to",
    );

    assert_eq!(
        arm_a.exit,
        i32::from(cli::task::EXIT_VALIDATION_BLOCKED),
        "arm A blocks at the validation exit code; findings were {:?}",
        arm_a.codes,
    );
    assert!(
        arm_a
            .codes
            .iter()
            .any(|code| code == "reconciliation.conflict-block"),
        "arm A's block is the conflict; got {:?}",
        arm_a.codes,
    );
    assert_eq!(
        arm_a.commits_before, arm_a.commits_after,
        "a conflict-block creates no commit",
    );
    assert!(
        arm_a.committed_adr.contains(HUMAN_PROSE) && !arm_a.committed_adr.contains(TASK_PROSE),
        "arm A promotes nothing and leaves the human's bytes alone; got:\n{}",
        arm_a.committed_adr,
    );

    assert_eq!(
        arm_b.exit,
        i32::from(cli::task::EXIT_SUCCESS),
        "arm B succeeds — nothing blocks; findings were {:?}",
        arm_b.codes,
    );
    assert!(
        arm_b
            .codes
            .iter()
            .any(|code| code == "file-state.baseline-adopt"),
        "arm B takes the UNKNOWN baseline-adopt arm instead; got {:?}",
        arm_b.codes,
    );
    assert_eq!(
        arm_b.commits_after,
        arm_b.commits_before + 1,
        "arm B lands a commit",
    );
    assert!(
        arm_b.committed_adr.contains(HUMAN_PROSE) && arm_b.committed_adr.contains(TASK_PROSE),
        "arm B silently merges both sides at exit 0 — the invariant, switched off; got:\n{}",
        arm_b.committed_adr,
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 2 — every destroying door answers for what it removes (Inc 2)
// ═════════════════════════════════════════════════════════════════════════════

/// The `??` plant — a path git merely does not track. It is also what the refusing doors
/// refuse on, so one fixture witnesses both arms of the rule.
const PLANTED_UNTRACKED: &str = "scratch.rs";

/// The `!!` plant — a path git **ignores**. No commit could ever carry it, and every
/// destroying door deletes it exactly as hard as tracked bytes.
const PLANTED_IGNORED: &str = "secrets.env";

/// `cp -R <from> <to>` — the ordinary way a corpus copy is made (how every RC trial corpus
/// is made), reproduced verbatim rather than simulated.
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

/// [`init_repo`] plus a **committed** ignore rule, so every fan-out worktree's checkout
/// inherits it — the `!!` plant needs a rule that is in the tree.
fn init_repo_ignoring(repo: &Path) {
    init_repo(repo);
    fs::write(repo.join(".gitignore"), format!("{PLANTED_IGNORED}\n")).expect("write .gitignore");
    git_ok(repo, &["add", ".gitignore"]);
    git_ok(repo, &["commit", "-q", "-m", "ignore the secret"]);
}

/// Mint `milestone:cache-rework` with one sub-task per intent.
fn mint_milestone(repo: &Path, home: &Path, intents: &[&str]) {
    run_jigc_ok(
        repo,
        home,
        &["milestone", "create", "Cache rework"],
        "`jigc milestone create`",
    );
    for intent in intents {
        run_jigc_ok(
            repo,
            home,
            &["milestone", "add-task", "cache-rework", intent],
            "`jigc milestone add-task`",
        );
    }
}

/// Stage a sub-task's authored `commit:<sub>` doc + its provenance bit — the sub-task
/// contribution a boundary promotes, and the authored prose a teardown destroys.
fn stage_subtask_commit(repo: &Path, sub: &str) {
    let address = format!("commit:{sub}");
    let docs = repo.join(".jigc").join("tasks").join(sub).join("docs");
    fs::create_dir_all(&docs).expect("mk the sub-task docs area");
    fs::write(
        docs.join(format!("{address}.md")),
        format!(
            "---\ntype: feat\n---\n\n# {sub}\n\n## Summary\n\nrework {sub}\n\n## Body\n\n\n\n## \
             Trailers\n"
        ),
    )
    .expect("write the staged commit doc");
    let manifest = docs.join("provenance.json");
    let mut record: Value = match fs::read_to_string(&manifest) {
        Ok(text) => serde_json::from_str(&text).expect("the provenance manifest parses"),
        Err(_) => serde_json::json!({ "docs": {} }),
    };
    record["docs"][&address] = Value::String("created".to_string());
    fs::write(
        &manifest,
        serde_json::to_string_pretty(&record).expect("serialize the provenance manifest"),
    )
    .expect("write the provenance manifest");
}

/// Write + `git add` a file **inside** a provisioned fan-out worktree.
fn stage_in_worktree(repo: &Path, sub: &str, rel: &str, body: &str) {
    let worktree = repo.join(".jigc").join("worktrees").join(sub);
    let path = worktree.join(rel);
    fs::create_dir_all(path.parent().expect("the plant has a parent")).expect("mk the parent");
    fs::write(&path, body).expect("write the worktree file");
    git_ok(&worktree, &["add", rel]);
}

/// Where a door's plants go — the path that door is about to remove, which is **not** the
/// same shape for every member since M52 Increment 4 generalized the table's subject from a
/// worktree to a destroyed path.
#[derive(Clone, Copy)]
enum PlantAt {
    /// `area-zed`'s fan-out **worktree** — the subject of the three doors that stand over
    /// `.jigc/worktrees/`.
    Worktree,
    /// `area-zed`'s **working area** under `.jigc/tasks/` — the subject the milestone
    /// boundary displaces (`settle-record.md` §18), which no worktree contains.
    SubTaskArea,
}

/// Put **both** plants at `area-zed`'s `at` and return the path holding them.
fn plant_both(repo: &Path, at: PlantAt) -> PathBuf {
    let doomed = match at {
        PlantAt::Worktree => repo.join(".jigc").join("worktrees").join("area-zed"),
        PlantAt::SubTaskArea => repo.join(".jigc").join("tasks").join("area-zed"),
    };
    plant_into(&doomed);
    doomed
}

/// Write both plants into `dir` — one path git merely does not track, one it ignores.
fn plant_into(dir: &Path) {
    for rel in [PLANTED_UNTRACKED, PLANTED_IGNORED] {
        let path = dir.join(rel);
        fs::create_dir_all(path.parent().expect("the plant has a parent")).expect("mk the parent");
        fs::write(&path, PLANTED_BYTES).expect("write the plant");
    }
}

/// What both plants hold — read back byte-for-byte wherever a door was supposed to keep
/// them.
const PLANTED_BYTES: &str = "sub-agent WIP\n";

/// One prepared cell: a repo standing in the shape in which `door` actually destroys, with
/// both plants at the path it is about to take.
struct DoorCell {
    repo: PathBuf,
    home: PathBuf,
    /// The path the door removes, holding both plants — they must be gone from here once it
    /// ran, or the cell proved an answer over a destruction that never happened.
    doomed: PathBuf,
    /// Where a [`Disposition::Displace`] member must have parked them, relative path
    /// preserved (`.jigc/displaced/<unit-id>/`). `None` for a door that destroys.
    kept: Option<PathBuf>,
    /// The argv that drives this door to its removal — **without** any consent, which the
    /// refusing arm appends from the door's own [`DestroyingDoor::consent`].
    argv: Vec<String>,
    _keep: Vec<TempDir>,
    _corpus: Option<TrialCorpus>,
}

/// Build the fixture `door` needs to reach its removal, and the argv that drives it there.
///
/// **The fixture is keyed on the door, not on the disposition**, because what a door
/// destroys and how it answers for it are two different facts: the three worktree doors and
/// the milestone boundary all stand in the fan-out world, but the boundary's *displaceable*
/// subject is a sub-task's working area, and the two `jigc task` doors stand over a single
/// task's area with no milestone in sight at all.
fn door_cell(door: &DestroyingDoor) -> DoorCell {
    match door.verb {
        "jigc milestone provision" => fanout_cell(
            PlantAt::Worktree,
            &["milestone", "provision", "cache-rework"],
        ),
        "jigc milestone discard" => {
            fanout_cell(PlantAt::Worktree, &["milestone", "discard", "cache-rework"])
        }
        "jigc uninstall" => fanout_cell(PlantAt::Worktree, &["uninstall"]),
        "jigc milestone finalize" => fanout_cell(
            PlantAt::SubTaskArea,
            &["milestone", "finalize", "cache-rework"],
        ),
        "jigc task discard" => task_area_cell(false),
        "jigc task finalize" => task_area_cell(true),
        other => panic!(
            "`{other}` is a member of the code-side `DESTROYING_DOORS` table with no cell in \
             this arm — a door added there owes its cell here, and a missing one is a hard \
             panic, never a silent gap",
        ),
    }
}

/// The fan-out world: a provisioned `milestone:cache-rework` with two sub-tasks, both plants
/// at `area-zed`'s `at`.
///
/// Three doors take the plain provisioned repo. `provision` cannot: it **reuses** a
/// registered worktree untouched, so the only shape in which it destroys anything is the
/// one that produced the defect — a `cp -R` of the repo, whose worktrees are registered at
/// the *source's* path and nowhere under the copy's own `.jigc/worktrees/`.
fn fanout_cell(at: PlantAt, argv: &[&str]) -> DoorCell {
    let home = TempDir::new("door-home");
    let home_path = home.path().to_path_buf();
    let source = TempDir::new("door-src");
    init_repo_ignoring(source.path());
    mint_milestone(source.path(), &home_path, &["Area zed", "Area low"]);
    for sub in ["area-low", "area-zed"] {
        stage_subtask_commit(source.path(), sub);
    }
    run_jigc_ok(
        source.path(),
        &home_path,
        &["milestone", "provision", "cache-rework"],
        "`jigc milestone provision`",
    );
    stage_in_worktree(source.path(), "area-low", "src/low.rs", "pub fn low() {}\n");

    let mut keep = vec![source, home];
    let repo = if argv.starts_with(&["milestone", "provision"]) {
        let copies = TempDir::new("door-copy");
        let repo = copies.path().join("copy");
        copy_repo(keep[0].path(), &repo);
        // Exactly one leftover, or the refusal is not attributable to the planted path.
        fs::remove_dir_all(repo.join(".jigc").join("worktrees").join("area-low"))
            .expect("drop the copy's other worktree");
        keep.insert(0, copies);
        repo
    } else {
        keep[0].path().to_path_buf()
    };
    let doomed = plant_both(&repo, at);

    DoorCell {
        // A plant in a **working area** is the one subject a displacing door can move
        // aside; a worktree plant has no such destination — that teardown narrates what it
        // takes, the measured `--ignored` bound of M46.
        kept: matches!(at, PlantAt::SubTaskArea)
            .then(|| repo.join(".jigc").join("displaced").join("area-zed")),
        repo,
        home: home_path,
        doomed,
        argv: argv.iter().map(|arg| (*arg).to_string()).collect(),
        _keep: keep,
        _corpus: None,
    }
}

/// The single-task world: one live task whose working area holds both plants, driven at
/// `jigc task discard` or — when `finalize` — at a `jigc task finalize` that really lands.
///
/// The commit gets a real diff on purpose: a commit-only task with nothing staged blocks at
/// `finalize.empty-commit`, and a cell that never reaches the removal proves nothing about
/// what the removal does.
fn task_area_cell(finalize: bool) -> DoorCell {
    let corpus = TrialCorpus::build(State::Fresh);
    let task = corpus.start_workflow("quick-fix", "Cap the retry budget");
    let repo = corpus.repo();
    let doomed = repo.join(".jigc").join("tasks").join(&task);
    plant_into(&doomed);

    let argv: Vec<String> = if finalize {
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
        corpus.set_slot(
            &format!("commit:{task}#body"),
            &task,
            "Driven by the destroying-door axis.",
        );
        vec!["task".into(), "finalize".into(), task.clone()]
    } else {
        vec!["task".into(), "discard".into(), task.clone()]
    };

    DoorCell {
        repo: repo.clone(),
        home: corpus.home(),
        doomed,
        kept: finalize.then(|| repo.join(".jigc").join("displaced").join(&task)),
        argv,
        _keep: Vec::new(),
        _corpus: Some(corpus),
    }
}

/// **Arm 2, cell 1** — the door rule, re-derived at the scope it holds and iterated over
/// the whole code-side table, **per disposition**.
///
/// M46 Increment 2 made [`DESTROYING_DOORS`]' subject **destruction** rather than refusal,
/// so `jigc milestone finalize` — which destroys on the ordinary success path — became a
/// member. M52 Increment 4 generalized the subject again, from a *worktree-shaped path* to
/// a **destroyed path**, which admitted the two doors that take a task's whole working
/// area (`jigc task discard`, `jigc task finalize`), and replaced the refuse-vs-narrate
/// `Option<code>` with [`Disposition`], because a door has **three** honest answers, not
/// two. The rule the table satisfies:
///
/// > every such door **answers for what it removes** — it refuses what it cannot prove is
/// > disposable, **or** it names every byte it destroys, **or** it keeps the bytes and says
/// > where it put them — and **silence is the one arm no door may take**.
///
/// Per member, through the real binary, over its own destroyed path holding one
/// **gitignored** file and one plain untracked file, with the assertions derived from that
/// member's [`Disposition`] rather than from a hand-written cell list:
///
///   * [`Disposition::Refuse`] — it refuses **first**, with one of *its own* blocking
///     identities, naming the untracked plant, routing at **its own** consent, and leaving
///     both plants byte-intact; driven again with that consent it names **both** plants —
///     the gitignored one no commit could ever carry included — and really destroys them.
///   * [`Disposition::Narrate`] — driven once, it names both plants and destroys them. No
///     member holds this disposition today (`settle-record.md` §18 moved the last one);
///     the arm is written because the rule is stated over three answers, and a door that
///     takes bytes it cannot refuse over must land here rather than in silence.
///   * [`Disposition::Displace`] — driven once, it names **where the bytes went**, and they
///     are there: byte-intact under `.jigc/displaced/<unit-id>/` with the relative path
///     preserved, and gone from the area it removed. **No consent cell is enumerated for a
///     displacing door** — `jigc task finalize --force` is a capability the Settle refuses,
///     and a member carries a consent only if it has one.
///
/// **The declared bound rides here rather than being implied:** at the worktree doors the
/// ignored axis is a *narration* axis only. The refusal probe deliberately stays
/// `--porcelain` without `--ignored` (a provisioned worktree that did its job holds
/// `target/`-shaped build output, so refusing on it would fire on the ordinary fan-out
/// **success** path and train `--force` into reflex), so the loss of an ignored path there
/// is made **visible, not prevented**.
#[test]
fn every_destroying_door_answers_for_what_it_removes() {
    assert_eq!(
        DESTROYING_DOORS.len(),
        6,
        "the axis is the code-side destroying-door table, read whole",
    );
    let refusing = DESTROYING_DOORS
        .iter()
        .filter(|door| door.consent().is_some())
        .count();
    assert!(
        refusing > 0 && refusing < DESTROYING_DOORS.len(),
        "the table's subject is destruction, not refusal: it must carry both a refusing \
         member and a member that answers some other way, or this arm proves only half the \
         rule",
    );
    let codes: BTreeSet<&str> = DESTROYING_DOORS
        .iter()
        .flat_map(|door| door.codes)
        .copied()
        .collect();
    assert_eq!(
        codes.len(),
        DESTROYING_DOORS
            .iter()
            .map(|door| door.codes.len())
            .sum::<usize>(),
        "no blocking identity is shared between two doors — a shared code makes a refusal \
         unattributable to the door that raised it",
    );

    for door in DESTROYING_DOORS {
        let verb = door.verb;
        let cell = door_cell(door);
        let base = cell.argv.clone();

        match door.disposition {
            // (1) Refuse — what it cannot prove is disposable, it does not take, and the
            //     consent it names is the only way past.
            Disposition::Refuse { consent } => {
                let argv: Vec<&str> = base.iter().map(String::as_str).collect();
                let refused = run_jigc(&cell.repo, &cell.home, &argv);
                let text = printed(&refused);
                assert!(
                    !refused.status.success(),
                    "[{verb}] a refusing door must REFUSE work it cannot prove is junk; \
                     got:\n{text}",
                );
                assert!(
                    door.codes.iter().any(|code| text.contains(code)),
                    "[{verb}] the refusal must carry one of THIS door's codes {:?} — every \
                     code in the table is door-scoped, so a refusal carrying none of them \
                     is unattributable; got:\n{text}",
                    door.codes,
                );
                assert!(
                    text.contains(PLANTED_UNTRACKED) && text.contains(consent),
                    "[{verb}] the refusal must name what would be destroyed and route at \
                     `{consent}`, its own consent; got:\n{text}",
                );
                for rel in [PLANTED_UNTRACKED, PLANTED_IGNORED] {
                    assert_eq!(
                        fs::read_to_string(cell.doomed.join(rel)).unwrap_or_default(),
                        PLANTED_BYTES,
                        "[{verb}] a refusal takes nothing — `{rel}` must survive byte-intact",
                    );
                }

                let mut driven = base;
                driven.push(consent.to_string());
                let text = drive_to_removal(&cell, verb, &driven);
                if verb == UNINSTALL_DOOR.verb {
                    assert!(
                        text.contains("commit:area-zed"),
                        "[{verb}] the teardown must ALSO name the authored task prose it \
                         destroys — a door that names half of what it takes is the law-1 \
                         half-truth; got:\n{text}",
                    );
                }
                assert_destroyed(&cell, verb);
            }

            // (2) Narrate — what it does take, it names.
            Disposition::Narrate => {
                drive_to_removal(&cell, verb, &base);
                assert_destroyed(&cell, verb);
            }

            // (3) Displace — what it will not take, it keeps, and says where it put it.
            Disposition::Displace => {
                let kept = cell
                    .kept
                    .as_ref()
                    .expect("a displacing door's cell names where the bytes must land");
                let text = drive_to_removal(&cell, verb, &base);
                let unit = kept
                    .file_name()
                    .expect("the displacement home is named for the work unit")
                    .to_string_lossy()
                    .into_owned();
                for rel in [PLANTED_UNTRACKED, PLANTED_IGNORED] {
                    assert!(
                        text.contains(&format!(".jigc/displaced/{unit}/{rel}")),
                        "[{verb}] a displacing door must name WHERE the bytes went, \
                         repo-relative; `{rel}` is missing from:\n{text}",
                    );
                    assert_eq!(
                        fs::read_to_string(kept.join(rel)).unwrap_or_default(),
                        PLANTED_BYTES,
                        "[{verb}] …and they must BE there, byte-intact with the relative \
                         path preserved — a named move nobody made is the law-1 half-truth \
                         in the other direction",
                    );
                }
                assert_destroyed(&cell, verb);
            }
        }
    }
}

/// Drive `argv` at the door and assert it reached its removal at exit 0, naming **both**
/// plants — the shared half of every disposition's answer: the gitignored plant no commit
/// could carry is named exactly as hard as the untracked one. Returns what was printed.
fn drive_to_removal(cell: &DoorCell, verb: &str, argv: &[String]) -> String {
    let args: Vec<&str> = argv.iter().map(String::as_str).collect();
    let out = run_jigc(&cell.repo, &cell.home, &args);
    let text = printed(&out);
    assert!(
        out.status.success(),
        "[{verb}] the door must reach its removal at exit 0; got:\n{text}",
    );
    assert!(
        text.contains(PLANTED_UNTRACKED),
        "[{verb}] the door must name the untracked path it removes; got:\n{text}",
    );
    assert!(
        text.contains(PLANTED_IGNORED),
        "[{verb}] the door must name the **gitignored** path it removes — no commit could \
         ever carry it, and the removal reaches it just as hard; got:\n{text}",
    );
    text
}

/// Non-vacuity, shared by all three arms: the plants really are gone from the path the door
/// removed, so every claim above was about a real removal rather than about nothing.
fn assert_destroyed(cell: &DoorCell, verb: &str) {
    for rel in [PLANTED_UNTRACKED, PLANTED_IGNORED] {
        assert!(
            !cell.doomed.join(rel).exists(),
            "[{verb}] the door must really have removed `{rel}` from the path it took, or \
             this cell proves an answer over nothing",
        );
    }
}

/// **Arm 2, cell 2 — the path subject.** The milestone boundary reads the worktree **on
/// disk**, not the registered set.
///
/// `provisioned_worktrees` used to intersect the milestone's task list with the repo's
/// *registered* worktrees — the same wrong subject M48 retired at the three refusing
/// doors, which survived here. A `cp -R` or `mv` of the whole repo (how every RC trial
/// corpus is made) leaves the copy's worktree admin records naming the **source's** paths,
/// so nothing under the copy's own `.jigc/worktrees/` is registered there: the boundary saw
/// *no worktrees at all* exactly where the sub-agents' live work sat, and landed a
/// docs-only commit at exit 0 while the manifest printed `no worktree provisioned` over a
/// directory holding the work.
///
/// The fixture is that shape verbatim — the premise (the copy registers nothing of its own)
/// is asserted, not assumed — and the claim is read off the **landed commit** and the
/// **emitted manifest**, which are the contract.
#[test]
fn the_milestone_boundary_reads_the_worktree_on_disk_not_the_registered_set() {
    let source_root = TempDir::new("subject-src");
    let source = source_root.path().join("repo");
    fs::create_dir_all(&source).expect("mk the source repo dir");
    init_repo(&source);
    let home = TempDir::new("subject-home");
    mint_milestone(&source, home.path(), &["Area low"]);
    run_jigc_ok(
        &source,
        home.path(),
        &["milestone", "provision", "cache-rework"],
        "`jigc milestone provision` in the source",
    );

    let copies = TempDir::new("subject-copies");
    let copy = copies.path().join("copy");
    copy_repo(&source, &copy);
    stage_in_worktree(&copy, "area-low", "src/low.rs", "pub fn low() {}\n");

    // The premise: the copy registers **nothing** under its own worktrees root.
    let listed = git_out(&copy, &["worktree", "list", "--porcelain"]);
    let own_root = copy
        .canonicalize()
        .expect("canonicalize the copy")
        .join(".jigc")
        .join("worktrees");
    assert!(
        !listed.contains(&own_root.display().to_string()),
        "the fixture's premise is that the copy registers NO worktree of its own; \
         got:\n{listed}",
    );

    let landed = run_jigc(
        &copy,
        home.path(),
        &["milestone", "finalize", "cache-rework"],
    );
    let text = printed(&landed);
    assert!(
        landed.status.success(),
        "the boundary must land over a live-but-unregistered worktree; got:\n{text}",
    );
    let paths = git_out(&copy, &["diff", "--name-only", "HEAD~1", "HEAD"]);
    assert!(
        paths.lines().any(|path| path == "src/low.rs"),
        "the sub-agent's staged code must be IN the landed commit — the boundary reads the \
         path, not the registration; landed:\n{paths}",
    );
    assert!(
        !text.contains("no worktree provisioned"),
        "a live worktree must not be narrated as never provisioned; got:\n{text}",
    );
    assert!(
        text.contains("area-low: 1 code file"),
        "the manifest must credit the sub-task's code; got:\n{text}",
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 3 — a never-adopted foreign file is not the corpus migration's subject (Inc 3)
// ═════════════════════════════════════════════════════════════════════════════

/// Run `jigc <args>` with the **real** `doc-code` probe (no `JIGC_DOC_CODE_PROBE` override).
fn run_jigc_probed(repo: &Path, home: &Path, args: &[&str]) -> Output {
    jigc_command(repo, home, args)
        .env_remove("JIGC_DOC_CODE_PROBE")
        .output()
        .expect("run the jigc binary")
}

/// A **real** Keep-a-Changelog file — the stock brownfield `CHANGELOG.md`. No stamp, and
/// it parses against no shipped `changelog` version: **foreign**, at the `changelog`
/// doctype's **placement** home.
const KEEP_A_CHANGELOG: &str = "\
# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Added

- A new thing.
";

/// An MADR-shaped decision record — the **located** squatter, at the `adr` doctype's
/// `docs/decisions/` home. Foreign: no stamp, and its sections match neither the current
/// (v2) `adr` shape nor the shipped v1 snapshot.
const MADR_DECISION: &str = "\
# 2. Use Postgres

Date: 2026-01-04

## Status

Accepted

## Context and problem

We need a relational store, and we need it before the pilot.

## Chosen option

Postgres, because the team already runs it.
";

/// The **control** — a genuinely managed, **below-version** ADR: the shipped prior (v1)
/// shape, stamped `schema-version: 1` while the `adr` manifest is at 2. It is the corpus
/// migration's actual subject, and the exclusion must not swallow it.
const ADR_STALE_V1: &str = "\
---
status: accepted
date: 2026-06-25
schema-version: 1
---

# Cache sessions in memory

## Context

Session lookups must stay sub-millisecond.

## Decision

Keep sessions in a single in-memory node.

## Consequences

A cold node loses its sessions.
";

/// One cell of the derived subject set: a foreign committed file and the **home kind** its
/// doctype declares.
struct ForeignCell {
    path: &'static str,
    /// `placement: { file: … }` — a literal path at the repo root, `location: None`,
    /// `docs-root` never applied; or `location: <dir>/`, resolved through the cascade.
    home: &'static str,
}

/// The two home kinds a managed doc can have, each carrying a foreign squatter.
///
/// **This is a derivation, and it is stated as one.** There is no code-side table
/// enumerating *"the surfaces that classify a committed file"*; the honest subject set is
/// `engine::validate::is_unadopted_foreign`'s call sites — the store sweep's fifth family,
/// the `AdoptionInputs::unadopted` seam (which `migrate_corpus.rs` reaches the
/// discriminator **through**, rather than by a direct call of its own), the read-side
/// reroute, `doc list`'s registration state and the task-scope reconciler — **× the two
/// home kinds below**. This arm drives the pair of doors that used to disagree; the others
/// carry their own suites.
const FOREIGN_CELLS: &[ForeignCell] = &[
    ForeignCell {
        path: "CHANGELOG.md",
        home: "placement",
    },
    ForeignCell {
        path: "docs/decisions/0002-use-postgres.md",
        home: "located",
    },
];

/// The control doc's committed path.
const CONTROL_DOC: &str = "docs/decisions/cache-sessions-in-memory.md";

/// The advisory both doors must raise, in the same words.
const UNADOPTED_CODE: &str = "schema-conformance.unadopted-instance";

/// The `(message, route)` pair of the single finding coded `code` and addressed at `path`.
fn finding_words(findings: &[Value], code: &str, path: &str, door: &str) -> (String, String) {
    let matches: Vec<&Value> = findings
        .iter()
        .filter(|finding| {
            finding["code"].as_str() == Some(code)
                && finding["location"]["address"].as_str() == Some(path)
        })
        .collect();
    assert_eq!(
        matches.len(),
        1,
        "[{door}] expected exactly one `{code}` addressed at `{path}`; the set was:\n{findings:#?}",
    );
    (
        matches[0]["message"]
            .as_str()
            .expect("a message string")
            .to_string(),
        matches[0]["route"]
            .as_str()
            .expect("a route string")
            .to_string(),
    )
}

/// The `findings[]` array of a `--format json` envelope, from a run that exited `expected`.
fn findings_of(out: &Output, expected: &[i32], what: &str) -> Vec<Value> {
    let envelope: Value = stdout_json(out, expected, what);
    envelope["findings"]
        .as_array()
        .unwrap_or_else(|| panic!("`{what}` carries a `findings` array; got:\n{envelope:#}"))
        .clone()
}

/// **Arm 3** — one file, one code, one route, in the same words at both doors; and the
/// exclusion is never a false green.
///
/// `jigc migrate-corpus` upgrades the **managed** corpus. A never-adopted foreign file
/// squatting at a managed doctype's home is the *adoption* case, and M42 shipped the one
/// discriminator that says so. The store door asked it; this door did not — so the same
/// file drew an advisory routed at `jigc ingest` from `jigc validate` and a **blocking**
/// `migrate-corpus.prose-needed` routed at *"author the prose, then re-run"* from
/// `jigc migrate-corpus`, at exit 1: two surfaces, one file, two stories, and a route that
/// changes nothing however exactly it is followed.
///
/// Per home kind, through the real binary: the foreign file is named by **no**
/// `migrate-corpus.*` finding, sits in **neither** `migrated` nor `already_current` nor
/// `blocked`, and is reported as the store door's own advisory with a **byte-identical**
/// message and route at both doors. The managed below-version control migrates past them —
/// at HEAD it sat `deferred` behind a foreign first-blocker, so the exclusion is what lets
/// it migrate at all. And [`STORE_EXIT_FLIPS`]' fifth member holds the sweep's exit
/// non-zero, so excluding the file from the *migration* never becomes a green over a
/// document jigc has never been handed.
#[test]
fn a_never_adopted_foreign_file_draws_one_advisory_at_both_doors() {
    // The registry read: the exclusion's exit-flipping member is on the code-side axis.
    let squatter = STORE_EXIT_FLIPS
        .iter()
        .find(|flip| flip.id == "foreign-squatter")
        .expect("the foreign squatter is a member of the code-side store-exit-flip axis");
    assert!(
        !squatter.sweep_untrustworthy,
        "the sweep WORKED here — it found the file and named it foreign — so no surface may \
         state this member as an untrustworthy sweep",
    );

    let root = TempDir::new("foreign");
    let repo = root.path().join("repo");
    let home = root.path().join("home");
    fs::create_dir_all(&repo).expect("mk repo");
    fs::create_dir_all(&home).expect("mk home");
    init_repo(&repo);
    run_jigc_ok(&repo, &home, &["setup"], "`jigc setup`");

    let decisions = repo.join("docs").join("decisions");
    fs::create_dir_all(&decisions).expect("mk docs/decisions/");
    fs::write(decisions.join("0002-use-postgres.md"), MADR_DECISION).expect("write the madr");
    fs::write(repo.join("CHANGELOG.md"), KEEP_A_CHANGELOG).expect("write the changelog");
    fs::write(repo.join(CONTROL_DOC), ADR_STALE_V1).expect("write the control adr");
    git_ok(&repo, &["add", "-A"]);
    git_ok(&repo, &["commit", "-q", "-m", "seed the corpus"]);

    // ── Door 1 — the store sweep ───────────────────────────────────────────────
    let swept = run_jigc_probed(&repo, &home, &["validate", "--format", "json"]);
    let store_findings = findings_of(&swept, &[1], "jigc validate --format json");
    assert!(
        !swept.status.success(),
        "the corpus is not green: a below-version managed doc and two never-adopted files \
         at managed homes both hold the sweep's exit",
    );

    // ── Door 2 — the corpus migration ──────────────────────────────────────────
    let migrated = run_jigc_probed(&repo, &home, &["migrate-corpus", "--format", "json"]);
    let report: Value = stdout_json(&migrated, &[0], "`jigc migrate-corpus --format json`");
    // The corpus envelope reports the excluded files under its own `unadopted[]` key — the
    // one producer both doors read, surfaced here rather than folded into a verdict.
    let migrate_findings = report["unadopted"]
        .as_array()
        .unwrap_or_else(|| panic!("the corpus envelope carries `unadopted`; got:\n{report:#}"))
        .clone();

    for cell in FOREIGN_CELLS {
        let path = cell.path;
        let home_kind = cell.home;

        // It is not the migration's subject: no fold verdict names it at all.
        for key in ["migrated", "already_current", "blocked"] {
            let listed = report[key]
                .as_array()
                .unwrap_or_else(|| panic!("the envelope carries `{key}`; got:\n{report:#}"));
            let named = listed.iter().any(|entry| {
                entry.as_str() == Some(path)
                    || entry["location"]["address"].as_str() == Some(path)
                    || entry["path"].as_str() == Some(path)
            });
            assert!(
                !named,
                "[{home_kind}] `{path}` is never-adopted and foreign — it must appear in no \
                 `{key}` verdict; got:\n{report:#}",
            );
        }
        assert!(
            !migrate_findings.iter().any(|finding| {
                finding["code"]
                    .as_str()
                    .is_some_and(|code| code.starts_with("migrate-corpus."))
                    && finding["location"]["address"].as_str() == Some(path)
            }),
            "[{home_kind}] no `migrate-corpus.*` finding may claim `{path}`; \
             got:\n{migrate_findings:#?}",
        );

        // And it IS reported — verbatim, from the one producer both doors read.
        let store = finding_words(&store_findings, UNADOPTED_CODE, path, "jigc validate");
        let migrate = finding_words(
            &migrate_findings,
            UNADOPTED_CODE,
            path,
            "jigc migrate-corpus",
        );
        assert_eq!(
            store, migrate,
            "[{home_kind}] the two doors must say the same words about `{path}` — a second \
             constructor is exactly how they would disagree again",
        );
        assert!(
            migrate.1.contains("jigc ingest"),
            "[{home_kind}] the advisory routes at adoption, never at a migration re-run that \
             changes nothing; route: {}",
            migrate.1,
        );
    }

    // The control: the managed, below-version doc is the migration's actual subject, and it
    // migrates past the squatters that used to block the whole run.
    let migrated_paths: Vec<String> = report["migrated"]
        .as_array()
        .expect("`migrated` is an array")
        .iter()
        .map(|entry| entry.as_str().unwrap_or_default().to_string())
        .collect();
    assert!(
        migrated_paths.iter().any(|path| path == CONTROL_DOC),
        "the managed below-version doc must migrate past the foreign squatters; \
         got: {migrated_paths:?}",
    );
    let landed = fs::read_to_string(repo.join(CONTROL_DOC)).expect("read the control doc");
    assert!(
        landed.contains("schema-version: 2"),
        "the control doc must land at the current stamp; got:\n{landed}",
    );

    // ── The exclusion is never a false green ───────────────────────────────────
    // With the managed corpus now current, the higher-precedence `unmigrated-corpus`
    // condition is cleared and the squatters are all that is left — so this is the run in
    // which the fifth member has to speak for itself. Excluding a foreign file from the
    // *migration* must not make the *sweep* say the corpus is fine.
    let after = run_jigc_probed(&repo, &home, &["validate"]);
    let agent = printed(&after);
    assert!(
        !after.status.success(),
        "a committed file jigc was never handed keeps the sweep non-zero — reporting it at \
         exit 0 was a green over a home jigc has never seen; got:\n{agent}",
    );
    assert!(
        agent.contains(squatter.cause),
        "the closing line must name WHICH condition fired — the registry's own `{}`; \
         got:\n{agent}",
        squatter.cause,
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 4 — no route promises a re-run that changes nothing (Inc 4 + Inc 5)
// ═════════════════════════════════════════════════════════════════════════════

/// The embedded dev pack tree on disk — the faithful source a mutated copy mirrors.
fn embedded_pack_tree() -> PathBuf {
    Path::new(cli::pack_path!(dev)).to_path_buf()
}

/// Recursively copy `src` into `dst` (both directories), creating `dst`.
fn copy_tree(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).expect("create the copy target dir");
    for entry in fs::read_dir(src).expect("read the pack dir") {
        let entry = entry.expect("a dir entry");
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if from.is_dir() {
            copy_tree(&from, &to);
        } else {
            fs::copy(&from, &to).expect("copy a pack file");
        }
    }
}

/// Bump `ty`'s manifest `schema-version` `from → to` in the copied pack at `pack`.
fn bump_manifest(pack: &Path, ty: &str, from: u32, to: u32) {
    let manifest_path = pack.join("config").join("schema-manifest.yaml");
    let manifest = fs::read_to_string(&manifest_path).expect("read the copied manifest");
    let bumped = manifest.replacen(
        &format!("- type: {ty}\n    schema-version: {from}"),
        &format!("- type: {ty}\n    schema-version: {to}"),
        1,
    );
    assert_ne!(
        manifest, bumped,
        "the manifest must carry `{ty}` at schema-version {from}",
    );
    fs::write(&manifest_path, bumped).expect("write the bumped manifest");
}

/// A dev-pack copy in which `adr` is bumped **2 → 3** by an enum-member rename the shipped
/// migration covers with no authored map — the **transform** halt's real shape.
fn pack_renaming_the_adr_status_members(tag: &str) -> TempDir {
    let dir = TempDir::new(tag);
    copy_tree(&embedded_pack_tree(), dir.path());
    let current = fs::read_to_string(dir.path().join("schemas").join("adr.yaml"))
        .expect("read the copied adr.yaml");
    let prior = current.replacen(
        "of: [proposed, accepted, superseded]",
        "of: [proposed, acc, sup]",
        1,
    );
    assert_ne!(
        current, prior,
        "adr.yaml must declare the `status` enum members",
    );
    fs::write(
        dir.path().join("schema-snapshots").join("adr.v2.yaml"),
        prior,
    )
    .expect("write the adr.v2 snapshot");
    bump_manifest(dir.path(), "adr", 2, 3);
    dir
}

/// A dev-pack copy in which `adr` is bumped **2 → 3** by an **added `set:`-derived** header
/// field: the prior snapshot is the shipped `adr.yaml` with its `date` line deleted.
fn pack_adding_a_set_derived_adr_field(tag: &str) -> TempDir {
    let dir = TempDir::new(tag);
    copy_tree(&embedded_pack_tree(), dir.path());
    let current = fs::read_to_string(dir.path().join("schemas").join("adr.yaml"))
        .expect("read the copied adr.yaml");
    let prior = current.replacen("      - { id: date, type: date, set: on-create }\n", "", 1);
    assert_ne!(
        current, prior,
        "adr.yaml must declare a header `date` field with `set: on-create`",
    );
    fs::write(
        dir.path().join("schema-snapshots").join("adr.v2.yaml"),
        prior,
    )
    .expect("write the adr.v2 snapshot");
    bump_manifest(dir.path(), "adr", 2, 3);
    dir
}

/// A **v0** (pre-stamp) ADR whose `## Consequences` heading is present with **no prose** —
/// the gate halt over a break the migration did not create.
const V0_ADR_EMPTY_CONSEQUENCES: &str = "\
---
status: accepted
---

# Alpha decision

## Context

Session lookups must stay sub-millisecond.

## Options

A distributed cache was weighed and rejected on latency.

## Decision

Keep sessions in a single in-memory node.

## Consequences

";

/// A conformant, **v2-stamped** ADR carrying `status: accepted` — the committed value an
/// empty remap table cannot cover.
const V2_ADR: &str = "\
---
status: accepted
date: 2026-06-25
schema-version: 2
---

# Alpha decision

## Context

Session lookups must stay sub-millisecond.

## Options

A distributed cache was weighed and rejected on latency.

## Decision

Keep sessions in a single in-memory node.

## Consequences

A cold node loses its sessions.
";

/// The same ADR with **no `date:` line** — under the v3 fixture schema that field is
/// `set:`-derived, so its absence still conforms and the migration has nothing to write.
const V2_ADR_WITHOUT_DATE: &str = "\
---
status: accepted
schema-version: 2
---

# Alpha decision

## Context

Session lookups must stay sub-millisecond.

## Options

A distributed cache was weighed and rejected on latency.

## Decision

Keep sessions in a single in-memory node.

## Consequences

A cold node loses its sessions.
";

/// Commit `source` at `docs/decisions/alpha.md` — the one-doc corpus every cell migrates.
fn commit_adr(repo: &Path, source: &str) {
    let dir = repo.join("docs").join("decisions");
    fs::create_dir_all(&dir).expect("mk docs/decisions/");
    fs::write(dir.join("alpha.md"), source).expect("write the adr");
    git_ok(repo, &["add", "."]);
    git_ok(repo, &["commit", "-q", "-m", "seed an adr"]);
}

/// Run `jigc migrate-corpus --format json` against a repo, optionally under a mutated pack,
/// and return the sole `blocked[]` member with the run's refusal asserted.
fn sole_blocked(repo: &Path, home: &Path, pack: Option<&Path>) -> Value {
    let mut command = jigc_command(repo, home, &["migrate-corpus", "--format", "json"]);
    if let Some(pack) = pack {
        command.env("JIGC_PACK_DIR", pack);
    }
    let out = command.output().expect("run the jigc binary");
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    assert!(
        !out.status.success(),
        "a corpus with a doc the verb could not migrate is NOT migrated, and the caller \
         hears so; got:\n{stdout}",
    );
    let report: Value = stdout_json(&out, &[1], "`jigc migrate-corpus --format json`");
    let blocked = report["blocked"]
        .as_array()
        .unwrap_or_else(|| panic!("the envelope carries `blocked[]`; got:\n{report:#}"));
    assert_eq!(
        blocked.len(),
        1,
        "exactly one doc halted the run; report:\n{report:#}",
    );
    blocked[0].clone()
}

/// Where each [`HaltReason`] variant is driven — the class's defining case-set, matched
/// **exhaustively** so a fourth cause cannot compile without deciding what it says and
/// where that is shown.
///
/// Two variants are reachable through the real binary from a dev-pack fixture and are
/// driven below. The third is not reachable from any shipped doctype pair — a fold whose
/// output does not parse under the new schema needs a hand-built change list — so it is
/// **classified explicitly** against its engine-seam proof rather than silently skipped,
/// and that proof's existence is asserted.
fn halt_cell(reason: &HaltReason) -> &'static str {
    match reason {
        HaltReason::Gate(_) => "driven here: the pre-existing empty slot",
        HaltReason::Transform(_) => "driven here: the uncovered enum rename",
        HaltReason::Parse(_) => {
            "engine seam: \
             a_fold_whose_output_does_not_parse_under_the_new_schema_halts_as_a_parse_failure"
        }
    }
}

/// **Arm 4, cell 1** — every halt names its own cause, and a conforming absence is a byte
/// no-op.
///
/// The per-doc transaction collapsed **three** distinct causes into one, and the CLI
/// rendered every one of them as `migrate-corpus.prose-needed`: *"`<path>`'s migration
/// mints a new **required** prose slot … author the new required prose, then re-run"*. Two
/// of the three were lies — a doc whose *pre-existing* slot was empty had nothing minted,
/// and an uncovered enum rename needs an authored old→new map in the pack tooling. No prose
/// an author writes into that doc, and no number of re-runs, changes either outcome: the
/// route was a **permanent dead end, followed exactly**.
///
/// And the fold itself refused an absence that already conforms: a `set:`-declared field
/// with no `default:` was `Unsupported`, though `is_author_required` is
/// `default.is_none() && set.is_none()` — so the doc was blocked with the same
/// author-the-prose route for a field no author can write. It now folds to a **byte**
/// no-op, and the report **names the `set:` fields it left unfilled**, because a silent
/// no-op is its own hazard.
#[test]
fn every_halt_names_its_own_cause_and_a_conforming_absence_is_a_byte_no_op() {
    // The case-set, from the code: every variant is classified, distinctly.
    let causes = [
        HaltReason::Gate(Vec::new()),
        HaltReason::Transform(TransformError::Unclassified),
        HaltReason::Parse(Vec::new()),
    ];
    let cells: Vec<&str> = causes.iter().map(halt_cell).collect();
    assert_eq!(
        cells.len(),
        cells
            .iter()
            .collect::<std::collections::BTreeSet<_>>()
            .len(),
        "every halt cause is classified distinctly; got {cells:?}",
    );
    let engine_seam = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("engine")
            .join("src")
            .join("transform.rs"),
    )
    .expect("read the engine transform module");
    assert!(
        engine_seam.contains(
            "a_fold_whose_output_does_not_parse_under_the_new_schema_halts_as_a_parse_failure"
        ),
        "the parse cause's cited engine-seam proof must exist — an unclassified member is a \
         hard panic, never a silent skip",
    );

    // ── The gate halt — over the STOCK pack: an ordinary brownfield state ───────
    let repo = TempDir::new("gate");
    let home = TempDir::new("gate-home");
    init_repo(repo.path());
    commit_adr(repo.path(), V0_ADR_EMPTY_CONSEQUENCES);
    let finding = sole_blocked(repo.path(), home.path(), None);
    assert_eq!(
        finding["code"], "migrate-corpus.prose-needed",
        "a gate halt keeps the contract-pinned waypoint; finding:\n{finding:#}",
    );
    let message = finding["message"].as_str().expect("a message");
    assert!(
        !message.contains("mints a new"),
        "the slot was empty before the run started — the migration minted nothing, and \
         saying otherwise is a claim about the migration that is simply false; \
         message: {message}",
    );
    let route = finding["route"].as_str().expect("a route");
    assert!(
        route.contains("author the new required prose") && route.contains("jigc migrate-corpus"),
        "a gate halt is the one cause an author CAN clear from the doc, so its route is \
         unchanged — and still followable; route: {route}",
    );

    // ── The transform halt — a repair no prose and no re-run reaches ────────────
    let repo = TempDir::new("transform");
    let home = TempDir::new("transform-home");
    let pack = pack_renaming_the_adr_status_members("transform-pack");
    init_repo(repo.path());
    commit_adr(repo.path(), V2_ADR);
    let finding = sole_blocked(repo.path(), home.path(), Some(pack.path()));
    assert_eq!(
        finding["code"], "migrate-corpus.fold-refused",
        "a halt the gate never saw takes the non-gate code; finding:\n{finding:#}",
    );
    let route = finding["route"].as_str().expect("a route");
    assert!(
        route.contains("authored_remap") && !route.contains("prose"),
        "the route names the schema-authoring repair that exists, and orders no prose no \
         author can write; route: {route}",
    );
    assert_eq!(
        fs::read_to_string(repo.path().join("docs").join("decisions").join("alpha.md"))
            .expect("read the adr"),
        V2_ADR,
        "a refused doc keeps its committed bytes",
    );

    // ── The conforming absence — a byte no-op, named in the report ─────────────
    let repo = TempDir::new("setfield");
    let home = TempDir::new("setfield-home");
    let pack = pack_adding_a_set_derived_adr_field("setfield-pack");
    init_repo(repo.path());
    commit_adr(repo.path(), V2_ADR_WITHOUT_DATE);
    // The forecast first (nothing written), so the landing run below is the doc's first.
    let forecast = jigc_command(
        repo.path(),
        home.path(),
        &["migrate-corpus", "--dry-run", "--format", "json"],
    )
    .env("JIGC_PACK_DIR", pack.path())
    .output()
    .expect("run the jigc binary");
    let stdout = String::from_utf8_lossy(&forecast.stdout).into_owned();
    assert!(
        forecast.status.success(),
        "an absence that already conforms must MIGRATE, not block; got:\n{stdout}",
    );
    let report: Value = stdout_json(
        &forecast,
        &[0],
        "`jigc migrate-corpus --dry-run --format json`",
    );
    assert_eq!(
        report["migrated"]
            .as_array()
            .expect("`migrated` is an array")
            .len(),
        1,
        "the doc is the migration's subject and it lands; report:\n{report:#}",
    );

    let landing = jigc_command(repo.path(), home.path(), &["migrate-corpus"])
        .env("JIGC_PACK_DIR", pack.path())
        .output()
        .expect("run the jigc binary");
    let text = printed(&landing);
    assert!(
        landing.status.success(),
        "the landing run must succeed; got:\n{text}",
    );
    assert!(
        text.contains("date"),
        "a silent no-op is its own hazard — the report must NAME the `set:` field it left \
         unfilled; got:\n{text}",
    );
    let landed = fs::read_to_string(repo.path().join("docs").join("decisions").join("alpha.md"))
        .expect("read the migrated adr");
    assert_eq!(
        landed,
        V2_ADR_WITHOUT_DATE.replace("schema-version: 2", "schema-version: 3"),
        "the fold is a BYTE no-op for the `set:` field: the stamp flips and nothing else \
         moves — no invented date",
    );
}

/// The committed managed doc the pre-guard corruption is planted in.
const CORRUPT_SPEC: &str = "docs/specs/rate-limiter.md";

/// The hand-planted heading — at `###`, the depth `spec#criteria` reserves for items, and
/// carrying no `{#id}` anchor.
const CORRUPT_HEADING: &str = "### Rationale for the cap";

/// The prose under the planted heading.
const CORRUPT_PROSE: &str = "The cap keeps the gateway responsive.";

/// The act the conflict route ordered until M46 and the adapter has never sanctioned —
/// `.jigc/AGENT.md`'s routing sentence forbids editing a managed doc directly.
const FORBIDDEN_ACT: &str = "revert the external edit on disk";

/// The demote target the message itself names, lifted out of ``demote it to `####` or
/// deeper`` — never the depth this file thinks is right.
fn demote_depth(diagnosis: &str) -> String {
    let (_, rest) = diagnosis
        .split_once("demote it to `")
        .unwrap_or_else(|| panic!("the diagnosis must name a demote depth; got:\n{diagnosis}"));
    let (depth, _) = rest
        .split_once('`')
        .unwrap_or_else(|| panic!("the demote depth must be backticked; got:\n{diagnosis}"));
    assert!(
        depth.len() > 3 && depth.chars().all(|ch| ch == '#'),
        "the demote depth must be deeper than the reserved `###`; got `{depth}`",
    );
    depth.to_string()
}

/// The one backticked `jigc …` span a blocking route offers, split through a **real
/// shell** so what runs is the bytes an agent would paste.
fn route_argv(text: &str, repo: &Path, home: &Path) -> Vec<String> {
    let route = text
        .lines()
        .find(|line| line.trim_start().starts_with("route:"))
        .unwrap_or_else(|| panic!("the block must carry a route; got:\n{text}"));
    let span = route
        .split('`')
        .find(|piece| piece.starts_with("jigc "))
        .unwrap_or_else(|| panic!("the route must offer a runnable `jigc` span; got: {route}"));
    support::shell_words(span, repo, home)
}

/// **Arm 4, cell 2 — the pre-guard repair route, run to green from the repo where the
/// corruption happened.**
///
/// `conformance.item-heading-unanchored` diagnoses a heading at the depth the schema
/// reserves for item structure with no `{#id}` anchor — the shape a corpus corrupted
/// before M45's write guard shipped carries at rest. The shipped repair chain
/// (`jigc migrate <path> --as <type>` → `doc author` → `finalize --approve`) worked only
/// **from a fresh clone**: in the repo where the corruption happened a file-state baseline
/// exists, so the finalize dies at `reconciliation.conflict-block` — whose route ordered
/// *"revert the external edit on disk"*, the one act the adapter forbids. A route out of
/// nothing.
///
/// The route now carries an exit for the one path it is true of — the migration's own
/// recorded source — and this cell **runs it as printed**: the argv is lifted out of the
/// emitted bytes and split through a real shell, so what is proven is what an agent would
/// paste. The chain then completes from exactly where it stopped, and the store validates
/// clean.
#[test]
fn the_pre_guard_repair_route_runs_from_the_repo_where_the_corruption_happened() {
    let corpus = TrialCorpus::build(State::Fresh);

    // A committed, managed spec — authored through the shipped chain, so the corruption
    // below is an out-of-band edit to bytes the CLI really wrote.
    let task = corpus.start_workflow("plan", "spec the rate limiter");
    let id = corpus
        .jigc_ok(&[
            "doc",
            "create",
            "spec",
            "--title",
            "Rate limiter",
            "--task",
            &task,
        ])
        .trim_end_matches('\n')
        .to_string();
    for (slot, prose) in [
        ("goal", "Cap bursts at the configured rate."),
        ("context", "The gateway has no limiter today."),
    ] {
        corpus.jigc_stdin_ok(
            &[
                "doc",
                "set-slot",
                &format!("{id}#{slot}"),
                "--from-file",
                "-",
                "--task",
                &task,
            ],
            prose,
        );
    }
    let item = corpus
        .jigc_ok(&[
            "doc",
            "add-item",
            &format!("{id}#criteria"),
            "--title",
            "Burst limit",
            "--task",
            &task,
        ])
        .trim_end_matches('\n')
        .to_string();
    corpus.jigc_stdin_ok(
        &[
            "doc",
            "set-slot",
            &format!("{item}/statement"),
            "--from-file",
            "-",
            "--task",
            &task,
        ],
        "A burst beyond the cap is rejected.",
    );
    corpus.finalize(&task, "spec", "spec the rate limiter", false);

    // The pre-guard corruption: a hand edit at a reserved-depth heading, committed.
    let path = corpus.repo().join(CORRUPT_SPEC);
    let body = fs::read_to_string(&path).expect("the committed spec is readable");
    let broken = body.replace(
        "A burst beyond the cap is rejected.\n",
        &format!("A burst beyond the cap is rejected.\n\n{CORRUPT_HEADING}\n\n{CORRUPT_PROSE}\n"),
    );
    assert_ne!(body, broken, "the hand break must land");
    fs::write(&path, broken).expect("write the hand-broken spec");
    corpus.git(&["add", CORRUPT_SPEC]);
    corpus.git(&[
        "commit",
        "-q",
        "-m",
        "hand-edit the spec (pre-guard corruption)",
    ]);

    // The diagnosis, and the demote depth lifted out of the words it printed.
    let swept = printed(&corpus.jigc(&["validate"]));
    assert!(
        swept.contains("conformance.item-heading-unanchored"),
        "the store sweep must raise the unanchored-heading diagnosis; got:\n{swept}",
    );
    assert!(
        !swept.contains("jigc doc add-item"),
        "the diagnosis must not name a verb the corrupted state refuses — running it \
         answers an unrelated `write.wrong-shape`; got:\n{swept}",
    );
    let anchor = format!("`{CORRUPT_HEADING}` sits at");
    let start = swept.find(&anchor).expect("the diagnosis is printed");
    let rest = &swept[start..];
    let diagnosis = rest[..rest.find('\n').unwrap_or(rest.len())]
        .trim_end()
        .to_string();
    let depth = demote_depth(&diagnosis);

    // The operator applies the repair the message named, then re-enters the shipped chain.
    let body = fs::read_to_string(&path).expect("the corrupted spec is readable");
    let demoted_heading = CORRUPT_HEADING.replacen("###", &depth, 1);
    fs::write(&path, body.replace(CORRUPT_HEADING, &demoted_heading))
        .expect("write the demoted heading");
    let minted = corpus.jigc_ok(&["migrate", CORRUPT_SPEC, "--as", "spec"]);
    let repair = minted_task(&minted);
    corpus.jigc_stdin_ok(
        &[
            "doc",
            "author",
            "spec",
            "--from-file",
            "-",
            "--task",
            &repair,
        ],
        &format!(
            "title: \"Rate limiter\"\n\
             sections:\n\
             \x20 - id: goal\n\
             \x20   set:\n\
             \x20     goal: |-\n\
             \x20       <<Cap bursts at the configured rate.>>\n\
             \x20 - id: context\n\
             \x20   set:\n\
             \x20     context: |-\n\
             \x20       <<The gateway has no limiter today.>>\n\
             \x20 - id: criteria\n\
             \x20   items:\n\
             \x20     - title: \"Burst limit\"\n\
             \x20       set:\n\
             \x20         statement: |-\n\
             \x20           <<A burst beyond the cap is rejected.\n\
             \n\
             \x20           {demoted_heading}\n\
             \n\
             \x20           {CORRUPT_PROSE}>>\n"
        ),
    );

    // The block, at the door the operator reaches first — from **this** repo, baseline and
    // all, which is the state the shipped chain could not clear.
    let blocked = corpus.jigc(&["task", "finalize", &repair]);
    let text = printed(&blocked);
    assert_eq!(
        blocked.status.code(),
        Some(3),
        "the migration finalize must block on the conflict; got:\n{text}",
    );
    assert!(
        text.contains("blocking · reconciliation.conflict-block"),
        "the block must be the conflict, not the review hold; got:\n{text}",
    );
    assert!(
        !text.contains(FORBIDDEN_ACT),
        "the route may not order the one act the adapter forbids; got:\n{text}",
    );

    // The exit, lifted out of the printed route and run verbatim.
    let argv = route_argv(&text, &corpus.repo(), &corpus.home());
    assert_eq!(
        argv.first().map(String::as_str),
        Some("jigc"),
        "the route's command span must be a `jigc` argv; got {argv:?}",
    );
    let run: Vec<&str> = argv[1..].iter().map(String::as_str).collect();
    let exit = corpus.jigc(&run);
    assert!(
        exit.status.success(),
        "the printed route must RUN from here — this is the whole repair; `jigc {}` \
         gave:\n{}",
        run.join(" "),
        printed(&exit),
    );

    // The chain completes from exactly where it stopped.
    assert_eq!(
        corpus.jigc(&["task", "finalize", &repair]).status.code(),
        Some(4),
        "the re-run must reach the migration review hold",
    );
    let approved = corpus.jigc(&["task", "finalize", &repair, "--approve"]);
    assert!(
        approved.status.success(),
        "the approved migration must land; got:\n{}",
        printed(&approved),
    );
    let swept = printed(&corpus.jigc(&["validate"]));
    assert!(
        swept.contains("no findings — the committed store validates clean"),
        "the chain must leave the store clean; got:\n{swept}",
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 5 — the gate previews what it gates on, and a diagnosis states its
//          comparison (Inc 6 + Inc 7)
// ═════════════════════════════════════════════════════════════════════════════

/// The changelog advisory's finding code — the member that joined [`Tier::Previewed`].
const CHANGELOG_GATE: &str = "changelog-recording.gate-granted-unused";

/// How one [`Tier::Previewed`] member is reached in this arm: either a finding code driven
/// **live** at `jigc task validate`, or an explicit classification against the suite that
/// drives it — never a silent skip.
enum PreviewCell {
    /// The code the member raises at the preview door in the fixture below.
    Driven(&'static str),
    /// `(suite file, test fn)` — the member's own axis suite, whose existence is asserted.
    Cited(&'static str, &'static str),
}

/// The cell for one previewed member, matched on the table's own stable id. A member with
/// no cell is a **hard panic**: the axis is the code-side table, so a row added there owes
/// an answer here.
fn preview_cell(row: &GateCoverage) -> PreviewCell {
    match row.id {
        "content-findings" => PreviewCell::Driven("schema-conformance.required-slot-present"),
        "carryover" => PreviewCell::Driven("finalize.carried-staged"),
        "changelog-gate" => PreviewCell::Driven(CHANGELOG_GATE),
        // The owned-location causes need a methodology owner-artifact workflow and their
        // own six-cause axis; that suite drives every one of them at this same door.
        "owner-artifact-unstaged" => PreviewCell::Cited(
            "owner_artifact_cause_axis.rs",
            "every_owned_location_cause_previews_except_the_untracked_one",
        ),
        // The one member invoked SEPARATELY at the door rather than inside the sweep
        // (`cli::gate_coverage::Invocation::SeparatelyAtDoor`): it refuses at the
        // operational exit code over a repository state, so it cannot be a finding code
        // in this arm's fixture, whose whole point is a task standing in every driven
        // member's condition at once. Its own suite drives it over the full git-state
        // axis and asserts the three doors byte-identical.
        "posture" => PreviewCell::Cited(
            "validate_previews_posture.rs",
            "validate_refuses_every_posture_finalize_refuses_and_concludes_nothing",
        ),
        other => panic!(
            "`{other}` is a `Tier::Previewed` member of the code-side `GATE_COVERAGE` table \
             with no cell in this arm — a member added there owes one here, and a missing \
             cell is a hard panic, never a silent gap",
        ),
    }
}

/// **Arm 5, cell 1** — `jigc task validate` previews every member it claims to, and the
/// changelog gate keys on the **write**, not on the item count.
///
/// The coverage claim is stated on eight surfaces and every enumeration was hand-written,
/// so a member joining the previewed set joined it eight times or not at all. M46 made
/// [`GATE_COVERAGE`] the single source and fenced each site per token; this arm drives the
/// other half — that the *behaviour* matches the claim, at the door.
///
/// The changelog member is the one that shows why the set had to become a table: the check
/// ran on the **committing** path only, so seven surfaces stated a coverage claim one
/// member short of the truth and none of them knew it. It also stopped counting repeatable
/// **items** and became the write-touch its design already declared — *"it keys on the
/// gate, never on the diff"* — because three ordinary authoring writes record an entry
/// while leaving the item count exactly where it was.
#[test]
fn the_preview_door_covers_every_member_it_claims_and_the_changelog_gate_keys_on_the_write() {
    let previewed: Vec<&GateCoverage> = gate_coverage::members(Tier::Previewed).collect();
    assert!(
        previewed.len() >= 4,
        "the previewed tier must carry the members M46 added to it; got {}",
        previewed.len(),
    );
    assert!(
        previewed.iter().any(|row| row.id == "changelog-gate"),
        "the changelog gate is a member of the previewed tier since M46 Inc 6",
    );

    // ── The fixture: one task standing in every driven member's condition ──────
    let repo = TempDir::new("preview");
    let home = TempDir::new("preview-home");
    init_repo(repo.path());

    // Pre-staged before the mint — the carryover member's condition.
    fs::write(repo.path().join("foreign.txt"), "someone else's work\n").expect("write the plant");
    git_ok(repo.path(), &["add", "foreign.txt"]);

    let task = minted_task(&run_jigc_ok(
        repo.path(),
        home.path(),
        &["start", "--workflow", "single-task", "tune the cache"],
        "`jigc start --workflow single-task`",
    ));
    // A created ADR with its required prose unwritten — the content-findings member's
    // condition. The changelog gate is granted by this workflow and left unused, which is
    // the third member's condition, and needs no act at all.
    run_jigc_ok(
        repo.path(),
        home.path(),
        &["doc", "create", "adr", "--title", "Single-node cache"],
        "`jigc doc create adr`",
    );

    let out = run_jigc(
        repo.path(),
        home.path(),
        &["task", "validate", &task, "--format", "json"],
    );
    let codes: Vec<String> = findings_of(&out, &[3], "jigc task validate --format json")
        .iter()
        .map(|finding| finding["code"].as_str().unwrap_or_default().to_string())
        .collect();

    for row in &previewed {
        match preview_cell(row) {
            PreviewCell::Driven(code) => assert!(
                codes.iter().any(|seen| seen == code),
                "`{}` is a previewed member, so `jigc task validate` must surface it — the \
                 preview door is where the in-task route is still live; expected `{code}`, \
                 got {codes:?}",
                row.id,
            ),
            PreviewCell::Cited(suite, test) => {
                let path = Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("tests")
                    .join(suite);
                let source = fs::read_to_string(&path)
                    .unwrap_or_else(|err| panic!("read the cited suite {path:?}: {err}"));
                assert!(
                    source.contains(test),
                    "`{}` is classified against `{suite}::{test}`, so that test must exist",
                    row.id,
                );
            }
        }
    }

    // The changelog advisory's route must run **at this door** — an in-task arm offered
    // where in-task verbs are dead is the dead end M46 removed.
    let advisory = findings_of(&out, &[3], "jigc task validate --format json")
        .into_iter()
        .find(|finding| finding["code"].as_str() == Some(CHANGELOG_GATE))
        .expect("the changelog advisory is in the preview envelope");
    let route = advisory["route"].as_str().expect("a route string");
    for span in route.split('`').filter(|piece| piece.starts_with("jigc ")) {
        if span.contains('<') {
            continue; // an author-picked placeholder is not claimed copy-runnable
        }
        let argv = support::shell_words(span, repo.path(), home.path());
        let args: Vec<&str> = argv[1..].iter().map(String::as_str).collect();
        let ran = run_jigc(repo.path(), home.path(), &args);
        assert!(
            !printed(&ran).contains("no task `"),
            "`{span}` is dead at the door that printed it — it answers `no task`:\n{}",
            printed(&ran),
        );
    }

    // ── The write-touch cell: item-count-invariant, and it still suppresses ────
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    let task = corpus.start_workflow("single-task", "tune the cache");
    let committed = fs::read_to_string(corpus.repo().join("CHANGELOG.md"))
        .expect("the committed changelog is readable");
    corpus.jigc_stdin_ok(
        &[
            "doc",
            "set-slot",
            "changelog:changelog#unreleased-changes/changed/notes",
            "--from-file",
            "-",
            "--task",
            &task,
        ],
        "- the fixture builder gained shape-class coverage, restated",
    );
    let staged = fs::read_to_string(
        corpus
            .repo()
            .join(".jigc")
            .join("tasks")
            .join(&task)
            .join("docs")
            .join("changelog:changelog.md"),
    )
    .expect("the staged changelog is readable");
    let items = |text: &str| text.lines().filter(|line| line.starts_with("### ")).count();
    assert_eq!(
        items(&committed),
        items(&staged),
        "the cell's premise: this write leaves the repeatable item count exactly where it \
         was — which is why an item-count predicate could not see it",
    );
    assert_ne!(
        committed, staged,
        "the cell's other premise: the write really landed bytes",
    );

    let out = corpus.jigc(&["task", "validate", &task, "--format", "json"]);
    let codes: Vec<String> = findings_of(&out, &[3], "jigc task validate (write-touch)")
        .iter()
        .map(|finding| finding["code"].as_str().unwrap_or_default().to_string())
        .collect();
    assert!(
        !codes.iter().any(|code| code == CHANGELOG_GATE),
        "a task that DID record an entry must draw no granted-but-unused advisory — the \
         gate keys on the write, never on the diff; got {codes:?}",
    );
}

/// The N-5 fixture, verbatim: a vitest file whose only test is registered by a **closure**,
/// so the test's name is present in the file's text and declares no unit. The shape three
/// frameworks share (PHP/Pest, TS/vitest, `node:test`), reported by three parties.
const CLOSURE_TEST_TS: &str = "import { it, expect } from \"vitest\";\n\n\
                               it('rejects a burst beyond the cap', () => {\n\
                               \x20   expect(true).toBe(true);\n\
                               });\n";

/// The closure test file's committed path.
const CLOSURE_TEST_PATH: &str = "tests/rate_limit.test.ts";

/// The test name the criterion cites — present in the file's text, declared nowhere.
const CLOSURE_TEST_NAME: &str = "rejects a burst beyond the cap";

/// **Arm 5, cell 2** — the diagnosis states the comparison the probe made.
///
/// `doc-code.criterion-maps-to-test` told an agent that a name *"is absent from"* a file
/// whose bytes the probe had just parsed and which carries that name on line 3, and then
/// offered three repairs — all of them repairs of a change that never happened, so an
/// agent following the route as written corrupts a correct citation. The seam splits on
/// evidence the probe already holds: the message now states **both sides** of the
/// comparison, and the route leads with the **file-only fallback** the design declares,
/// **with what it does not buy stated** (the file's presence, not that a test exists).
///
/// **The verdict does not move.** A name living only in a string or a comment must never
/// false-resolve, so the cell stays blocking — this is a repair of the sentence and the
/// repairs, never of the check.
#[test]
fn a_closure_registered_test_name_is_told_what_the_probe_compared() {
    // The only corpus state with a real, resolving code anchor: its committed `spec`'s
    // criterion names a `#[test]` fn in tracked source.
    let corpus = TrialCorpus::build(State::Vendored);
    let repo = corpus.repo();
    let home = corpus.home();

    fs::create_dir_all(repo.join("tests")).expect("mk tests/");
    fs::write(repo.join(CLOSURE_TEST_PATH), CLOSURE_TEST_TS).expect("write the closure fixture");

    // The editable channel: re-point the committed criterion at the closure-registered
    // test, and commit both — the state a brownfield repo arrives in.
    let spec_path = repo.join("docs").join("specs").join("padding.md");
    let spec = fs::read_to_string(&spec_path).expect("the committed spec is readable");
    let repointed = spec
        .lines()
        .map(|line| {
            if line.trim_start().starts_with("- maps-to-test:") {
                format!("- maps-to-test: {CLOSURE_TEST_PATH}#{CLOSURE_TEST_NAME}")
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
        + "\n";
    assert_ne!(
        spec, repointed,
        "the fixture must re-point a `maps-to-test` the committed spec really carries",
    );
    fs::write(&spec_path, repointed).expect("write the re-pointed spec");
    corpus.git(&["add", "-A"]);
    corpus.git(&["commit", "-q", "-m", "cite the vitest closure test"]);

    let swept = run_jigc_probed(&repo, &home, &["validate", "--format", "json"]);
    let findings = findings_of(&swept, &[0], "jigc validate --format json");
    let finding = findings
        .iter()
        .find(|finding| finding["code"].as_str() == Some("doc-code.criterion-maps-to-test"))
        .unwrap_or_else(|| panic!("the sweep must raise the non-resolution; got:\n{findings:#?}"));

    assert_eq!(
        finding["severity"].as_str(),
        Some("blocking"),
        "the verdict does not move — a name living only in a string never resolves; \
         finding:\n{finding:#}",
    );
    let message = finding["message"].as_str().expect("a message string");
    assert!(
        message.contains(&format!("the text `{CLOSURE_TEST_NAME}` occurs in"))
            && message.contains("declares nothing there"),
        "the message must state BOTH sides of the comparison the probe made, never an \
         absence its own parsed bytes contradict; got: {message}",
    );
    assert!(
        !message.contains("is absent from"),
        "the retired sentence claimed an absence the indexed blob disproves; got: {message}",
    );
    let route = finding["route"].as_str().expect("a route string");
    assert!(
        route.starts_with(&format!("cite `{CLOSURE_TEST_PATH}` alone")),
        "the route must LEAD with the repair that works in this cell; got: {route}",
    );
    assert!(
        route.contains("buys the file's presence, not that a test exists"),
        "a recommendation of the quietest path owes its bound; got: {route}",
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 6 — the seven admitted surfaces answer at the door that prints them (Inc 8)
// ═════════════════════════════════════════════════════════════════════════════

/// **There is no registry here, and this arm manufactures none.**
///
/// A surface batch has no production table to iterate: its members are seven independently
/// adjudicated rows of the wave's [razor ledger] §1, each admitted at a **cited** rule in a
/// locked artifact and each demonstrated by driving the binary before the build. Minting a
/// code-side `ADMITTED_SURFACES` table would be scaffolding dressed as an axis — a claim
/// that the seven share a mechanism, which is exactly the false-universal shape this wave
/// removes elsewhere. So the cell set is the ledger's admitted set, carried by hand,
/// deliberately, and stated as such in code and in the chapter.
///
/// The ids are the ledger's own: **B1-1**, **B2-1**, **B2-2**, **B2-3**, **B3b-1**,
/// **B3b-2**, **S-2**.
///
/// [razor ledger]: ../../../completions/artifacts/M46/razor-ledger.md
const ADMITTED_SURFACES: [&str; 7] = ["B1-1", "B2-1", "B2-2", "B2-3", "B3b-1", "B3b-2", "S-2"];

/// **Arm 6** — each admitted surface, driven at the door that prints it.
///
/// The rule every row is admitted at is one of two clauses: *a statement about a surface is
/// quantified over what that surface actually serves, and the repair is **scope**, never a
/// behaviour change — name the scope, and name what the members outside it do instead*
/// (`surface-contract.md`:136), or *every affordance that is the designated recovery is
/// named by the surfaces that produce that state* (law 2). Nothing below changes what any
/// door **does**; every assertion is about what it **says**, at the moment it says it.
#[test]
fn the_seven_admitted_surfaces_answer_at_the_door_that_prints_them() {
    let mut driven: Vec<&str> = Vec::new();

    // ── B1-1 · one block, two meanings of "staged" ─────────────────────────────
    // The task door's state-truth clause ended "and your staged changes are still staged"
    // — one word covering two mechanisms. On a docs-only task (its whole contribution an
    // in-task ADR, nothing `git add`-ed) that sentence printed while `git diff --cached`
    // was empty, so a worker taking the word in git's sense went looking for a state git
    // cannot show. The repair is scope: the clause names the two areas separately.
    {
        let repo = TempDir::new("b1-1");
        let home = TempDir::new("b1-1-home");
        init_repo(repo.path());
        let hooks = repo.path().join(".git").join("hooks");
        fs::create_dir_all(&hooks).expect("mk hooks");
        let hook = hooks.join("pre-commit");
        fs::write(&hook, "#!/bin/sh\necho 'policy: refused' >&2\nexit 1\n")
            .expect("write the rejecting hook");
        fs::set_permissions(&hook, fs::Permissions::from_mode(0o755)).expect("chmod hook");

        let task = minted_task(&run_jigc_ok(
            repo.path(),
            home.path(),
            &[
                "start",
                "--workflow",
                "single-task",
                "record the cache decision",
            ],
            "`jigc start`",
        ));
        run_jigc_ok(
            repo.path(),
            home.path(),
            &["doc", "create", "adr", "--title", "Single-node cache"],
            "`jigc doc create adr`",
        );
        for (slot, prose) in [
            ("context", "Session lookups must stay sub-millisecond."),
            ("decision", "Keep sessions in one node."),
            ("consequences", "A cold node loses its sessions."),
        ] {
            run_jigc_stdin_ok(
                repo.path(),
                home.path(),
                &[
                    "doc",
                    "set-slot",
                    &format!("adr:single-node-cache#{slot}"),
                    "--from-file",
                    "-",
                ],
                format!("{prose}\n").as_bytes(),
                "set-slot",
            );
        }
        fill_commit(repo.path(), home.path(), &task, "record the cache decision");

        let staged_paths = git_out(repo.path(), &["diff", "--cached", "--name-only"]);
        assert!(
            staged_paths.is_empty(),
            "the cell's premise: git's index is EMPTY going in, so the frame's word has \
             nothing to point at in git; got:\n{staged_paths}",
        );
        let refused = run_jigc(repo.path(), home.path(), &["task", "finalize", &task]);
        let text = printed(&refused);
        assert!(
            !refused.status.success(),
            "the hook must reject the commit; got:\n{text}",
        );
        assert!(
            text.contains(&format!(".jigc/tasks/{task}/docs/")),
            "the frame must name the area the task's work actually survives in; got:\n{text}",
        );
        assert!(
            text.contains("git's index"),
            "the frame must name git's index as its own, separate area; got:\n{text}",
        );
        assert!(
            !text.contains("your staged changes are still staged"),
            "the one word covering two mechanisms is exactly what B1-1 retired; got:\n{text}",
        );
        driven.push("B1-1");
    }

    // ── B2-1 · a route the gate that printed it forbids ────────────────────────
    // The absent-instance refusal offered `jigc doc create <type>` unconditionally. Under
    // `quick-fix` (`allows-create: []`) that command answers `create.gate-blocked … allowed
    // doctypes: []` — a printed route that hard-rejects from the state that printed it.
    {
        let repo = TempDir::new("b2-1");
        let home = TempDir::new("b2-1-home");
        init_repo(repo.path());
        let task = minted_task(&run_jigc_ok(
            repo.path(),
            home.path(),
            &[
                "start",
                "--workflow",
                "quick-fix",
                "tighten the cache guard",
            ],
            "`jigc start --workflow quick-fix`",
        ));
        let refused = run_jigc_stdin(
            repo.path(),
            home.path(),
            &[
                "doc",
                "set-slot",
                "adr:absent-decision#context",
                "--from-file",
                "-",
                "--task",
                &task,
            ],
            b"prose\n",
        );
        let text = printed(&refused);
        assert!(
            !refused.status.success(),
            "an absent instance is refused; got:\n{text}",
        );
        assert!(
            !text.contains("jigc doc create adr"),
            "this task's gate forbids `adr`, so the refusal may not offer a create that \
             hard-rejects from here; got:\n{text}",
        );
        driven.push("B2-1");
    }

    // ── B2-2 · a route that contradicts the record it leaves standing ──────────
    // The blanket base-pin refusal offered `jigc task discard <sub>`, which exits 0 and
    // removes the sub-task's working area while the committed record still reads
    // `status: active` and `milestone list-tasks` still lists it. The sub-task arm now
    // names the isolation mechanism the message never named.
    {
        let repo = TempDir::new("b2-2");
        let home = TempDir::new("b2-2-home");
        init_repo(repo.path());
        run_jigc_ok(
            repo.path(),
            home.path(),
            &["milestone", "create", "Rework"],
            "`jigc milestone create`",
        );
        run_jigc_ok(
            repo.path(),
            home.path(),
            &[
                "milestone",
                "add-task",
                "rework",
                "Do the thing",
                "--workflow",
                "single-task",
            ],
            "`jigc milestone add-task`",
        );
        fs::write(repo.path().join("unrelated.md"), "unrelated\n").expect("write the mover");
        git_ok(repo.path(), &["add", "unrelated.md"]);
        git_ok(repo.path(), &["commit", "-q", "-m", "unrelated"]);

        let blocked = run_jigc(
            repo.path(),
            home.path(),
            &["start", "--task", "do-the-thing"],
        );
        let text = printed(&blocked);
        assert!(
            !blocked.status.success(),
            "the sub-task read door keeps the blanket base-pin refusal; got:\n{text}",
        );
        assert!(
            text.contains(".jigc/worktrees/do-the-thing"),
            "the refusal must name the worktree the sub-task's work actually happens in; \
             got:\n{text}",
        );
        assert!(
            !text.contains("jigc task discard do-the-thing"),
            "it may not route at a discard that exits 0 while the committed record still \
             names the sub-task; got:\n{text}",
        );
        driven.push("B2-2");
    }

    // ── B2-3 · a contradiction inside one help output ──────────────────────────
    // The `about` enumerated only the join's doc bodies ("and commit them") while
    // `--carry-staged`, three lines below in the SAME output, stated the aggregate commit
    // is built from the sub-task worktrees. The boundary lands both halves.
    {
        let repo = TempDir::new("b2-3");
        let home = TempDir::new("b2-3-home");
        init_repo(repo.path());
        let help = run_jigc_ok(
            repo.path(),
            home.path(),
            &["milestone", "finalize", "--help"],
            "`jigc milestone finalize --help`",
        );
        let about = help
            .split("Usage:")
            .next()
            .expect("the help opens with its about")
            .to_string();
        for half in ["doc bodies", "code", "sub-task worktree"] {
            assert!(
                about.contains(half),
                "the about must name `{half}` — the boundary commits the join's doc bodies \
                 AND each sub-task worktree's staged code; got:\n{about}",
            );
        }
        driven.push("B2-3");
    }

    // ── B3b-1 · a closed list with no exit ─────────────────────────────────────
    // The router presented the selectable work-workflows and stopped, while `record-change`
    // sits outside that set — correctly hidden, with a reason readable only after you had
    // already found the thing. The repair names the scope and what sits outside it; the
    // member stays hidden.
    {
        let repo = TempDir::new("b3b-1");
        let home = TempDir::new("b3b-1-home");
        init_repo(repo.path());
        let composed = run_jigc_ok(
            repo.path(),
            home.path(),
            &["start", "record a decision about the cache"],
            "`jigc start <intent>` (the router)",
        );
        assert!(
            composed.contains("That catalog is the selectable subset"),
            "the router must name its catalog as a subset; got:\n{composed}",
        );
        assert!(
            composed.contains("jigc describe --workflows"),
            "…and name the fuller read that carries each hidden workflow's reason; \
             got:\n{composed}",
        );
        assert!(
            !composed.contains("record-change"),
            "the fix names the class and the read — it re-lists no suppressed workflow, so \
             `expires: never` keeps meaning what it says; got:\n{composed}",
        );
        // The named read is RUN, never restated.
        let listed = run_jigc_ok(
            repo.path(),
            home.path(),
            &["describe", "--workflows"],
            "`jigc describe --workflows`",
        );
        assert!(
            listed.contains("record-change"),
            "the read the router names must answer with the member outside the catalog; \
             got:\n{listed}",
        );
        driven.push("B3b-1");
    }

    // ── B3b-2 · a read-back over a write the step sanctions leaving off ────────
    // `step:locate-from-spec` printed "Read your write back … the write you just made" over
    // the *spec* address unconditionally, while its own prose sanctions leaving the only
    // spec write in the step off — in which case the printed read blocks `store.not-staged`.
    // The read-back now names the write the step always makes.
    {
        let repo = TempDir::new("b3b-2");
        let home = TempDir::new("b3b-2-home");
        init_repo(repo.path());
        let preview = run_jigc_ok(
            repo.path(),
            home.path(),
            &["workflow", "implement-from-spec", "--preview"],
            "`jigc workflow implement-from-spec --preview`",
        );
        let region = preview
            .split_once("Read your write back")
            .expect("the step still states a staged read-back at all")
            .1;
        let line = region
            .lines()
            .find(|line| line.trim_start().starts_with("jigc doc show "))
            .expect("the read-back prints an addressed staged read");
        assert!(
            line.contains("jigc doc show commit:"),
            "the read-back must name the write this step ALWAYS makes; got: {line}",
        );
        assert!(
            region.contains("ONLY when you set `maps-to-test`"),
            "…and the paragraph after it must name what the member outside that scope does \
             instead; got:\n{region}",
        );
        driven.push("B3b-2");
    }

    // ── S-2 · a foreclosed form whose answer was already on record ─────────────
    // `jigc describe adr` was a bare clap exit 2 while `Commands::Describe`'s own doc three
    // lines away recorded what answers it. Law 2 asks the surface that produces the state
    // to name the designated recovery.
    {
        let repo = TempDir::new("s-2");
        let home = TempDir::new("s-2-home");
        init_repo(repo.path());
        let refused = run_jigc(repo.path(), home.path(), &["describe", "adr"]);
        let text = printed(&refused);
        assert!(
            !refused.status.success(),
            "the single-item lookup stays foreclosed; got:\n{text}",
        );
        assert!(
            text.contains("tip:"),
            "the refusal must land its answer in the tip slot; got:\n{text}",
        );
        assert!(
            text.contains("jigc start --explain"),
            "…and name the resolution trace its own definition already records; got:\n{text}",
        );
        driven.push("S-2");
    }

    assert_eq!(
        driven, ADMITTED_SURFACES,
        "every admitted row of the razor ledger's §1 set is driven here, in order — the set \
         is hand-carried on purpose, so nothing but this assertion keeps it whole",
    );
}
