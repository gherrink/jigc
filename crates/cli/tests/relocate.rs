//! **`jigc relocate` — the dedicated suite** (the latent-surface sweep, 2026-08-13).
//!
//! Provenance: [decisions-pending.md](../../../implementation/decisions-pending.md) →
//! *The latent-surface sweep*, whose scope is derived from the **verb registry minus what
//! the trial walks minus what a named standing suite fences** — never from a changed-file
//! table. Against that derivation `relocate` was the charter's headline residue: the
//! `verb_suite_coverage` map named exactly **two** suites for it, and both are incidental
//! (`anyhow_route_spans` drives the frozen-doctype refusal for its *route span*;
//! `flow40_acceptance` arm 3 drives the happy-path move as one step of a larger flow).
//!
//! **What the derivation actually found, stated as a result rather than dressed up.**
//! Checked by content, not by name (pinning.md §5 — *a coverage classification is a claim
//! about the code*):
//!
//! | property | disposition before this suite |
//! |---|---|
//! | happy-path detect + move of a stranded freeze-exempt instance | fenced — `flow40_acceptance::freeze_exempt_relocation_moves_a_stranded_committed_instance` |
//! | a manifest-frozen doctype is refused and routed to `migrate-corpus` | fenced — same arm + `anyhow_route_spans::frozen_doctype_relocate_routes_to_migrate_corpus` |
//! | the **foreign-squatter displacement** at the destination | **unit-tested only** — `relocate.rs`'s in-module `a_foreign_squatter_at_the_destination_moves_into_the_workbench_and_the_managed_lands`, never driven through the real binary |
//! | `relocate` is **not** a committing door | true by construction — it `git mv`s and stages, and calls no commit, so its absence from [`ERROR_CODE_REGISTRY`](../../src/invocation_log.rs) is correct, not a hole |
//!
//! **So this suite pins the one genuine gap: the squatter path through the real binary.**
//! It matters because of what the pre-1.0.0 trial found one door over — `milestone
//! provision` *deletes* a non-registered leftover's uncommitted work at exit 0 under an
//! ordinary success line ([RC-pre-1.0/findings-verification.md](../../../completions/artifacts/RC-pre-1.0/findings-verification.md)
//! → F3). `relocate` touches a user's committed-but-unmanaged file on the same kind of
//! path: it `git rm --cached`s the squatter and `fs::rename`s it into the **gitignored**
//! `.jigc/` workbench. The file survives — but a move into a gitignored tree is
//! indistinguishable from a deletion to anyone who does not read the ack, so **the ack
//! naming it is the whole safety property**, and an in-module unit test cannot fence what
//! the *binary prints*.
//!
//! **The sweep's other two candidates are deliberately not here**, each with its reason:
//! `milestone join` is driven at five call sites across three suites and commits nothing;
//! and the **leftover re-`provision`** is now a *confirmed defect* (F3), so pinning its
//! current behaviour would pin data loss as expected output — it routes to that fix's red
//! test, not to a sweep guard. Recording that is the charter's own instruction: *if the
//! derivation turns up nothing beyond those, that is a result.*

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-relocate-{tag}-{}-{:?}",
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

fn git_init(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
}

fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("run jigc")
}

fn assert_ok(out: &std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} must succeed; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

fn stdout_of(out: &std::process::Output) -> String {
    String::from_utf8(out.stdout.clone())
        .expect("utf-8 stdout")
        .trim()
        .to_string()
}

/// The manifest-less `note` fixture doctype (`location: notes/`) — the genuinely
/// freeze-exempt shape `relocate` serves, mirroring `flow40_acceptance`'s fixture so the
/// two suites agree on what "freeze-exempt" means.
fn write_note_pack(dir: &Path) {
    fs::create_dir_all(dir.join("schemas")).expect("mk fixture pack schemas/");
    fs::write(
        dir.join("schemas").join("note.yaml"),
        "type: note\nlocation: notes/\nid-from: title\nsections:\n  - id: body\n    slot: { hint: \"The note.\" }\n",
    )
    .expect("write the note schema");
}

/// **The squatter displacement, through the real binary.**
///
/// A committed-but-**unmanaged** file sits at the destination a stranded managed instance
/// must move to. `relocate` frees the slot by moving that foreign file into the gitignored
/// `.jigc/` workbench — a move that reads as a deletion to anyone who does not read the
/// output. This asserts all three halves of the safety property: the managed doc lands,
/// the foreign bytes **survive** at their new home, and **the ack names the displacement
/// with both paths**, which is the half no in-module unit test can reach.
#[test]
fn a_foreign_squatter_is_displaced_into_the_workbench_and_the_ack_names_it() {
    let repo = TempDir::new("squatter");
    let home = TempDir::new("home");
    git_init(repo.path());

    let note_pack = repo.path().join(".jigc").join("note-pack");
    write_note_pack(&note_pack);
    fs::write(
        repo.path().join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", note_pack.display()),
    )
    .expect("write packs.yaml naming the note pack");

    // A managed `note` stranded at a legacy home, and a FOREIGN file already committed at
    // the destination the relocation targets.
    // The destination resolves through the cascade's `docs-root` (default `docs/`), so a
    // `location: notes/` doctype homes at `docs/notes/` — never at a bare `notes/`.
    let prior_rel = "docs/legacy-notes/cache-benchmarks.md";
    let dest_rel = "docs/notes/cache-benchmarks.md";
    let managed_body = "# Cache Benchmarks\n\n## Body\n\nA single node caps throughput.\n";
    let foreign_body = "# Someone else's notes\n\nHand-written, never managed by jigc.\n";

    fs::create_dir_all(repo.path().join("docs").join("legacy-notes")).expect("mk prior home");
    fs::write(repo.path().join(prior_rel), managed_body).expect("write the stranded note");
    fs::create_dir_all(repo.path().join("docs").join("notes")).expect("mk destination dir");
    fs::write(repo.path().join(dest_rel), foreign_body).expect("write the foreign squatter");
    git(repo.path(), &["add", prior_rel, dest_rel]);
    git(
        repo.path(),
        &[
            "commit",
            "-q",
            "-m",
            "strand the note; squat its destination",
        ],
    );

    let out = jigc(
        repo.path(),
        home.path(),
        &["relocate", "note", "--from", "docs/legacy-notes/"],
    );
    assert_ok(&out, "`jigc relocate note --from docs/legacy-notes/`");
    let report = stdout_of(&out);

    // (1) The ack names the displacement — BOTH paths and the reason. This is the safety
    //     property: a move into a gitignored tree that nothing narrates is a deletion as
    //     far as the reader is concerned (RC-pre-1.0 → F3, the sibling that does delete).
    assert!(
        report.contains("displaced") && report.contains(dest_rel),
        "the relocation ack must name the displaced squatter and where it went; got:\n{report}"
    );
    assert!(
        report.contains("workbench"),
        "the ack must say the squatter went to the workbench, not merely that it moved; got:\n{report}"
    );

    // (2) The managed doc actually landed at its schema home, carrying its own bytes.
    let landed = fs::read_to_string(repo.path().join(dest_rel)).expect("read the destination");
    assert!(
        landed.contains("A single node caps throughput."),
        "the managed note must land at its schema home; got:\n{landed}"
    );

    // (3) The foreign bytes SURVIVE — displaced, never destroyed. Located by content
    //     rather than by a reconstructed path, so the assertion does not encode the
    //     workbench layout it is not the owner of.
    let workbench = repo.path().join(".jigc");
    let mut found = None;
    for entry in walk(&workbench) {
        if let Ok(body) = fs::read_to_string(&entry)
            && body.contains("Hand-written, never managed by jigc.")
        {
            found = Some(entry);
            break;
        }
    }
    assert!(
        found.is_some(),
        "the displaced foreign file's bytes must survive somewhere under .jigc/; \
         searched {workbench:?}\nack was:\n{report}"
    );

    // (4) Negative control — without a squatter the ack must NOT claim a displacement,
    //     so (1) is the displacement being reported rather than a constant string.
    let repo2 = TempDir::new("no-squatter");
    let home2 = TempDir::new("home2");
    git_init(repo2.path());
    let pack2 = repo2.path().join(".jigc").join("note-pack");
    write_note_pack(&pack2);
    fs::write(
        repo2.path().join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", pack2.display()),
    )
    .expect("write packs.yaml");
    fs::create_dir_all(repo2.path().join("docs").join("legacy-notes")).expect("mk prior home");
    fs::write(repo2.path().join(prior_rel), managed_body).expect("write the stranded note");
    git(repo2.path(), &["add", prior_rel]);
    git(repo2.path(), &["commit", "-q", "-m", "strand the note"]);

    let clean = jigc(
        repo2.path(),
        home2.path(),
        &["relocate", "note", "--from", "docs/legacy-notes/"],
    );
    assert_ok(
        &clean,
        "`jigc relocate` with no squatter at the destination",
    );
    let clean_report = stdout_of(&clean);
    assert!(
        clean_report.contains("0 displaced"),
        "with no squatter the ack must report zero displacements; got:\n{clean_report}"
    );
}

/// Every file under `dir`, recursively. Small by construction — the `.jigc/` tree of a
/// freshly relocated fixture repo.
fn walk(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let Ok(entries) = fs::read_dir(dir) else {
        return out;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            out.extend(walk(&path));
        } else {
            out.push(path);
        }
    }
    out
}

/// **The squatter displacement re-probes the posture immediately before it frees the index
/// slot** (M52 Increment 3 / T4; `settle-record.md` → D2.5 — the last of eleven
/// index-mutating sites; [baseline-posture.md](../../../completions/artifacts/M52/baseline-posture.md)
/// §1.3, §2.10).
///
/// The door adjudicated the posture once, before anything was resolved. This drives the
/// interleaving the door cannot answer: an operation the user starts **after** that verdict
/// and before the act. Through the real binary, and deterministically — a `git` shim on the
/// child's `PATH` opens a real, conflicting `git cherry-pick` on the **first `git ls-files
/// -z`**, which is `orphan::committed_markdown`, the listing `relocate` takes its subject
/// from and the first git call past the door (the door's own probe runs `git ls-files -u`,
/// a different argv, and every marker it reads it reads before that).
///
/// What the refusal must protect is the half a following `move_doc` probe cannot: the
/// squatter's **index entry**. `displace_foreign_squatter` frees it with `git rm --cached`
/// *before* the managed `git mv` runs, so without a probe at that line the user's
/// committed-but-unmanaged file is dropped from the index and renamed into the gitignored
/// workbench — and the run then exits **0**, reporting the doc it could not move as one
/// blocked row, under a repository-wide refusal that is nobody's per-doc fault.
///
/// The clean cell is the sibling above (`…_is_displaced_into_the_workbench_and_the_ack_names_it`),
/// which still displaces; this asserts the refusing one.
#[test]
fn a_cherry_pick_opened_after_the_doors_verdict_refuses_before_the_squatters_slot_is_freed() {
    let repo = TempDir::new("midrun-pick");
    let home = TempDir::new("home-midrun");
    let shim_dir = TempDir::new("shim");
    git_init(repo.path());

    let note_pack = repo.path().join(".jigc").join("note-pack");
    write_note_pack(&note_pack);
    fs::write(
        repo.path().join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", note_pack.display()),
    )
    .expect("write packs.yaml naming the note pack");

    let prior_rel = "docs/legacy-notes/cache-benchmarks.md";
    let dest_rel = "docs/notes/cache-benchmarks.md";
    let managed_body = "# Cache Benchmarks\n\n## Body\n\nA single node caps throughput.\n";
    let foreign_body = "# Someone else's notes\n\nHand-written, never managed by jigc.\n";

    fs::create_dir_all(repo.path().join("docs").join("legacy-notes")).expect("mk prior home");
    fs::write(repo.path().join(prior_rel), managed_body).expect("write the stranded note");
    fs::create_dir_all(repo.path().join("docs").join("notes")).expect("mk destination dir");
    fs::write(repo.path().join(dest_rel), foreign_body).expect("write the foreign squatter");
    git(repo.path(), &["add", prior_rel, dest_rel]);
    git(
        repo.path(),
        &[
            "commit",
            "-q",
            "-m",
            "strand the note; squat its destination",
        ],
    );

    // A commit that conflicts with HEAD on a non-`.md` file, so the shim's cherry-pick
    // stops with `CHERRY_PICK_HEAD` in place and the `.md` listing relocate walks is
    // untouched by the conflict.
    let base = git(repo.path(), &["rev-parse", "HEAD"]).trim().to_string();
    git(repo.path(), &["checkout", "-q", "-b", "side", &base]);
    fs::write(repo.path().join("conflict.txt"), "theirs\n").expect("write theirs");
    git(repo.path(), &["add", "conflict.txt"]);
    git(repo.path(), &["commit", "-q", "-m", "theirs"]);
    let pick = git(repo.path(), &["rev-parse", "HEAD"]).trim().to_string();
    git(repo.path(), &["checkout", "-q", "-"]);
    fs::write(repo.path().join("conflict.txt"), "ours\n").expect("write ours");
    git(repo.path(), &["add", "conflict.txt"]);
    git(repo.path(), &["commit", "-q", "-m", "ours"]);

    // The shim: fire once, on `git ls-files -z`, then hand every call to the real git.
    let real_git = String::from_utf8(
        Command::new("sh")
            .args(["-c", "command -v git"])
            .output()
            .expect("locate git")
            .stdout,
    )
    .expect("utf-8 git path")
    .trim()
    .to_string();
    let stamp = shim_dir.path().join("fired");
    let shim = shim_dir.path().join("git");
    fs::write(
        &shim,
        format!(
            "#!/bin/sh\nif [ \"$1\" = \"ls-files\" ] && [ \"$2\" = \"-z\" ] && \
             [ ! -e {stamp} ]; then\n  : > {stamp}\n  {real_git} -C {repo} cherry-pick {pick} \
             >/dev/null 2>&1\nfi\nexec {real_git} \"$@\"\n",
            stamp = stamp.display(),
            real_git = real_git,
            repo = repo.path().display(),
            pick = pick,
        ),
    )
    .expect("write the git shim");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&shim, fs::Permissions::from_mode(0o755)).expect("chmod the shim");
    }

    let index_before = git(repo.path(), &["ls-files", "--stage", "--", dest_rel]);
    assert!(
        !index_before.trim().is_empty(),
        "the fixture must commit the squatter, so it HAS an index entry to lose",
    );

    let path = format!(
        "{}:{}",
        shim_dir.path().display(),
        std::env::var("PATH").unwrap_or_default()
    );
    let out = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(["relocate", "note", "--from", "docs/legacy-notes/"])
        .current_dir(repo.path())
        .env("HOME", home.path())
        .env("PATH", &path)
        .env_remove("JIGC_PACK_DIR")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("run jigc");
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();

    assert!(
        stamp.exists() && repo.path().join(".git").join("CHERRY_PICK_HEAD").exists(),
        "the instrument must have opened a real cherry-pick mid-run; \
         stamp={}\nstdout:\n{stdout}\nstderr:\n{stderr}",
        stamp.exists(),
    );
    // The squatter is untouched — its bytes at the destination AND its index entry.
    assert_eq!(
        fs::read_to_string(repo.path().join(dest_rel)).unwrap_or_default(),
        foreign_body,
        "a refused displacement must leave the squatter's bytes at the destination — they \
         were moved out from under an operation jigc did not conclude;\nstdout:\n{stdout}",
    );
    assert_eq!(
        git(repo.path(), &["ls-files", "--stage", "--", dest_rel]),
        index_before,
        "a refused displacement must leave the squatter's index entry byte-identical",
    );
    assert!(
        walk(&repo.path().join(".jigc").join("displaced")).is_empty(),
        "nothing may be parked in the workbench by a refused run",
    );
    assert!(
        repo.path().join(prior_rel).exists(),
        "the stranded doc must stay where it was",
    );
    assert_eq!(
        out.status.code(),
        Some(1),
        "a mover that meets an un-concluded operation must refuse the run, not report it as \
         a per-doc block at exit 0;\nstdout:\n{stdout}\nstderr:\n{stderr}",
    );
    assert!(
        stderr.contains("repo.operation-in-progress"),
        "the refusal must carry the family's identity; stderr:\n{stderr}",
    );
    assert!(
        stderr.contains("cherry-pick"),
        "the refusal must name the operation and the git command that concludes it; \
         stderr:\n{stderr}",
    );
}
