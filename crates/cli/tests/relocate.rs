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

/// What already stands in the parking home under the squatter's basename.
#[derive(Clone, Copy, Debug)]
enum Parked {
    /// A file an earlier displacement left there — the only copy of its bytes.
    EarlierFile,
    /// The same, and a second one already beside it under the first free name.
    TwoEarlierFiles,
    /// A dangling symbolic link: a name in the way that `exists()` reads as absent.
    DanglingLink,
}

/// **A parked squatter never replaces a file already parked under its name** (the rc.24 fix
/// pass's completion audit, CPL-4).
///
/// The workbench is where a displaced file's **only** copy lives — it is gitignored and in
/// no commit — and this arm parked by basename: driven on `1.0.0-rc.24` and on the tree
/// before this cell, with `.jigc/displaced/<name>` left by an earlier displacement, the next
/// `jigc relocate` printed `displaced … → .jigc/displaced/<name>` at exit 0 and the earlier
/// file was in no file anywhere. The parking home's other producer already had the rule
/// (`<name>.<n>`, the first free); this one now uses it.
///
/// Every cell asserts on the bytes: the earlier occupants are byte-identical where they
/// were, and the path the ack **names** is where the new squatter's bytes are.
#[test]
fn a_parked_squatter_never_replaces_a_file_already_parked_under_its_name() {
    for parked in [
        Parked::EarlierFile,
        Parked::TwoEarlierFiles,
        Parked::DanglingLink,
    ] {
        let cell = format!("{parked:?}");
        let repo = TempDir::new("park-collision");
        let home = TempDir::new("home");
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
        fs::create_dir_all(repo.path().join("docs").join("legacy-notes")).expect("mk prior home");
        fs::write(
            repo.path().join(prior_rel),
            "# Cache Benchmarks\n\n## Body\n\nA single node caps throughput.\n",
        )
        .expect("write the stranded note");
        git(repo.path(), &["add", prior_rel]);
        git(repo.path(), &["commit", "-q", "-m", "strand the note"]);
        // The squatter is untracked: nothing in git holds its bytes either.
        fs::create_dir_all(repo.path().join("docs").join("notes")).expect("mk destination dir");
        fs::write(repo.path().join(dest_rel), "NEW-SQUATTER-MARKER\n").expect("write squatter");

        let workbench = repo.path().join(".jigc").join("displaced");
        fs::create_dir_all(&workbench).expect("mk the parking home");
        let first = workbench.join("cache-benchmarks.md");
        let second = workbench.join("cache-benchmarks.md.2");
        match parked {
            Parked::EarlierFile => {
                fs::write(&first, "EARLIER-PARKED-MARKER\n").expect("park an earlier file");
            }
            Parked::TwoEarlierFiles => {
                fs::write(&first, "EARLIER-PARKED-MARKER\n").expect("park an earlier file");
                fs::write(&second, "SECOND-PARKED-MARKER\n").expect("park a second one");
            }
            Parked::DanglingLink => {
                #[cfg(unix)]
                std::os::unix::fs::symlink("nowhere-at-all.md", &first).expect("plant a link");
                #[cfg(not(unix))]
                fs::write(&first, "EARLIER-PARKED-MARKER\n").expect("park an earlier file");
            }
        }

        let out = jigc(
            repo.path(),
            home.path(),
            &["relocate", "note", "--from", "docs/legacy-notes/"],
        );
        assert_ok(
            &out,
            &format!("{cell}: `jigc relocate note --from docs/legacy-notes/`"),
        );
        let report = stdout_of(&out);

        // The earlier occupants are exactly where and what they were.
        match parked {
            Parked::EarlierFile | Parked::TwoEarlierFiles => {
                assert_eq!(
                    fs::read_to_string(&first).ok().as_deref(),
                    Some("EARLIER-PARKED-MARKER\n"),
                    "{cell}: the earlier parked file must be byte-identical; ack:\n{report}",
                );
            }
            Parked::DanglingLink => {
                #[cfg(unix)]
                assert_eq!(
                    fs::read_link(&first).ok(),
                    Some(PathBuf::from("nowhere-at-all.md")),
                    "{cell}: the link already parked there must be untouched; ack:\n{report}",
                );
            }
        }
        if matches!(parked, Parked::TwoEarlierFiles) {
            assert_eq!(
                fs::read_to_string(&second).ok().as_deref(),
                Some("SECOND-PARKED-MARKER\n"),
                "{cell}: …and so must the second; ack:\n{report}",
            );
        }

        // The ack names where the new squatter went, and that is where its bytes are.
        let named = report
            .split_whitespace()
            .find(|word| word.starts_with(".jigc/displaced/"))
            .unwrap_or_else(|| panic!("{cell}: the ack must name the parked path; got:\n{report}"));
        assert_eq!(
            fs::read_to_string(repo.path().join(named)).ok().as_deref(),
            Some("NEW-SQUATTER-MARKER\n"),
            "{cell}: the path the ack names (`{named}`) must hold the squatter's bytes; \
             ack:\n{report}",
        );
        let expected = match parked {
            Parked::EarlierFile | Parked::DanglingLink => ".jigc/displaced/cache-benchmarks.md.2",
            Parked::TwoEarlierFiles => ".jigc/displaced/cache-benchmarks.md.3",
        };
        assert_eq!(
            named, expected,
            "{cell}: the first free name beside the taken one"
        );

        // …and the managed doc landed.
        assert!(
            fs::read_to_string(repo.path().join(dest_rel))
                .is_ok_and(|landed| landed.contains("A single node caps throughput.")),
            "{cell}: the managed note must land at its schema home",
        );
    }
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

/// The `digest` fixture doctype — **freeze-exempt and `placement:`-homed**, at a literal
/// file whose own name is not a slug. It is the *omitting context* for the destination
/// gate below: a fixed-identity doctype's slug is the type id, so the filename supplies
/// nothing to the identity and the gate is vacuous there by construction.
fn write_digest_pack(dir: &Path) {
    fs::create_dir_all(dir.join("schemas")).expect("mk fixture pack schemas/");
    fs::write(
        dir.join("schemas").join("digest.yaml"),
        "type: digest\nsingleton: true\nplacement: { file: \"Digest Of Notes.md\" }\n\
         id-from: title\nsections:\n  - id: body\n    slot: { hint: \"The digest.\" }\n",
    )
    .expect("write the digest schema");
}

/// Write the manifest-less `note` pack into `repo` and point the project layer at it.
fn install_note_pack(repo: &Path, pack_rel: &str) {
    let pack = repo.join(".jigc").join(pack_rel);
    write_note_pack(&pack);
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", pack.display()),
    )
    .expect("write packs.yaml naming the fixture pack");
}

/// **`relocate` asks the predicate `ingest` asks, about the destination** (M52 Increment 8
/// / T6; `design/validation.md` → the `ingest.unaddressable-identity` row).
///
/// The move is **byte-faithful**, so a `location:`-homed relocation carries the source's
/// own filename into the doctype's home — which means the destination's identity is
/// decided by a name the caller never typed on the command line. Driven at HEAD the door
/// asked nothing: three stranded docs moved, and `jigc doc list` then named
/// `note:My Note` and `note:UPPER_CASE` **managed** while `jigc doc show` refused both
/// `store.malformed-slug` — routing the reader back at `jigc doc list`, the surface that
/// had just claimed them. That is `ingest.unaddressable-identity`'s exact class, at the
/// other door that puts a file at a managed home.
///
/// The arm asserts the whole shape, not only the refusal: the addressable sibling in the
/// **same run** still moves (a triage door classifies the rest — the door's exit is
/// unchanged), neither refused file is moved on disk or in the index, and the store
/// surface afterwards names no managed identity the read surface refuses.
#[test]
fn an_unaddressable_destination_is_refused_and_the_addressable_sibling_still_moves() {
    let repo = TempDir::new("unaddressable-dest");
    let home = TempDir::new("home");
    git_init(repo.path());
    install_note_pack(repo.path(), "note-pack");

    // Three committed docs stranded at a legacy home. Two carry names that no
    // `<type>:<slug>` address reaches; one is a well-formed doc id.
    for (rel, body) in [
        ("notes/My Note.md", "# My Note\n\n## Body\n\nprose\n"),
        ("notes/UPPER_CASE.md", "# Upper Case\n\n## Body\n\nprose\n"),
        ("notes/good-note.md", "# Good Note\n\n## Body\n\nprose\n"),
    ] {
        fs::create_dir_all(repo.path().join("notes")).expect("mk prior home");
        fs::write(repo.path().join(rel), body).expect("write the stranded doc");
    }
    git(repo.path(), &["add", "-A"]);
    git(repo.path(), &["commit", "-q", "-m", "strand three notes"]);

    let out = jigc(
        repo.path(),
        home.path(),
        &["relocate", "note", "--from", "notes"],
    );
    let report = stdout_of(&out);
    // The door's EXIT is not changed: `jigc ingest`'s triage divergence is the mold — a
    // blocking finding inside a run that classifies everything else.
    assert_eq!(
        out.status.code(),
        Some(0),
        "the destination gate binds the MOVE, not the run — the door stays a triage door; \
         stdout:\n{report}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );

    // (1) Both refusals carry `ingest.unaddressable-identity`'s code and route.
    for rel in ["notes/My Note.md", "notes/UPPER_CASE.md"] {
        assert!(
            report.contains(rel),
            "the report must name the doc it refused to move; got:\n{report}"
        );
        assert!(
            report.contains("ingest.unaddressable-identity"),
            "the refusal must carry the predicate's own code; got:\n{report}"
        );
        assert!(
            repo.path().join(rel).exists(),
            "a refused doc must stay where it was — {rel} moved anyway;\n{report}"
        );
    }
    assert_eq!(
        report.matches("ingest.unaddressable-identity").count(),
        2,
        "one refusal per refused doc; got:\n{report}"
    );
    assert!(
        report.contains(&format!(
            "git -C {} mv",
            // The door canonicalizes its root, and on macOS the fixture's `/var/…` is a
            // symlink to `/private/var/…` — so the comparison canonicalizes too rather than
            // asserting the spelling the test happens to hold.
            repo.path()
                .canonicalize()
                .unwrap_or_else(|_| repo.path().to_path_buf())
                .display()
        )),
        "the route must name the act that resolves the state, aimed at the checkout it runs \
         in; got:\n{report}"
    );

    // (2) The addressable sibling in the SAME run still moves.
    assert!(
        repo.path().join("docs/notes/good-note.md").exists(),
        "the addressable sibling must still relocate; got:\n{report}"
    );

    // (3) The index carries no rename for the refused pair.
    let status = git(repo.path(), &["status", "--porcelain"]);
    for name in ["My Note.md", "UPPER_CASE.md"] {
        assert!(
            !status.contains(&format!("docs/notes/{name}")),
            "the refused pair must produce no staged rename; git status:\n{status}"
        );
    }
    assert!(
        status.contains("docs/notes/good-note.md"),
        "the sibling's move must be staged; git status:\n{status}"
    );

    // (4) The store surface names no managed identity the read surface refuses — the
    //     loop this gate closes.
    let listed = jigc(repo.path(), home.path(), &["doc", "list"]);
    assert_ok(&listed, "`jigc doc list`");
    let listing = stdout_of(&listed);
    for line in listing.lines() {
        if !line.ends_with("managed") {
            continue;
        }
        let Some(id) = line.split_whitespace().next() else {
            continue;
        };
        if !id.contains(':') {
            continue;
        }
        let shown = jigc(repo.path(), home.path(), &["doc", "show", id]);
        assert!(
            shown.status.success(),
            "`jigc doc list` named `{id}` managed but `jigc doc show` refuses it — the \
             loop the destination gate closes;\nlisting:\n{listing}\nstderr:\n{}",
            String::from_utf8_lossy(&shown.stderr),
        );
    }

    // (5) The code reaches the machine surface too, not only the text one.
    let json = jigc(
        repo.path(),
        home.path(),
        &["--format", "json", "relocate", "note", "--from", "notes"],
    );
    let json_report = stdout_of(&json);
    assert!(
        json_report.contains("ingest.unaddressable-identity"),
        "the refusal must reach `--format json`; got:\n{json_report}"
    );

    // (6) The EMITTED route is run verbatim — the command string an agent would copy,
    //     lifted out of the report rather than rebuilt here, through a real shell. It is
    //     the whole claim that this refusal is not a dead end: after it, the doc is at the
    //     `note` home under an identity `jigc doc show` resolves.
    let emitted = backticked_move_route(&report)
        .unwrap_or_else(|| panic!("the refusal must emit a runnable `git mv`; got:\n{report}"));
    let ran = Command::new("sh")
        .arg("-c")
        .arg(&emitted)
        .current_dir(repo.path())
        .output()
        .expect("run the emitted route");
    assert!(
        ran.status.success(),
        "the emitted route must run verbatim: `{emitted}`\nstderr:\n{}",
        String::from_utf8_lossy(&ran.stderr),
    );
    let adopted = jigc(repo.path(), home.path(), &["ingest"]);
    assert_ok(&adopted, "`jigc ingest` after the emitted route");
    let shown = jigc(repo.path(), home.path(), &["doc", "show", "note:my-note"]);
    assert!(
        shown.status.success(),
        "after the route the doc must be addressable; ingest said:\n{}\nstderr:\n{}",
        stdout_of(&adopted),
        String::from_utf8_lossy(&shown.stderr),
    );
}

/// The first backtick-delimited span in `report` that **contains** a `git mv` — the
/// emitted command an agent would copy, lifted out of the printed bytes rather than
/// reconstructed, so the arms above run what the binary actually said. It matches on
/// containment, not on the span's first word, because the repair is whatever the door had
/// to emit for the line to run verbatim — which at the relocate door is a `mkdir -p` of the
/// home the move lands in, ahead of the `git mv` itself.
fn backticked_move_route(report: &str) -> Option<String> {
    report
        .split('`')
        // The span is aimed since M53 (the cwd census, C1-10): `mkdir -p <abs dir> && git -C
        // <abs> mv <rel> <rel>`. Both halves name an absolute, because `mkdir` has no `-C`
        // and the pair has to run from one cwd or the other, not one each.
        .find(|span| span.contains(" mv "))
        .map(str::to_string)
}

/// **The refused row's route runs when the refused set is the whole set** (M52 Increment 8
/// / T6 — the completion the arm above did not reach).
///
/// The arm above drives the **mixed** topology: an addressable sibling relocates first and
/// creates the `note` home, so by the time its emitted `git mv` runs the destination
/// directory is already there. That is a property of the fixture, not of the product — and
/// the dominant real shape is the opposite one, because `jigc relocate` exists precisely
/// when a doctype's home has MOVED, so the new home typically holds nothing yet and a run
/// whose every stranded doc is refused moves nothing into it. Driven at HEAD in that cell
/// the emitted route died: `fatal: renaming 'notes/My Note.md' failed: No such file or
/// directory`, exit **128** — the followed-exactly-and-nothing-happens dead end M46's PT-1
/// and this wave's charter both name as the defect.
///
/// The axis is the refused set's **topology** — {some addressable sibling relocates, none
/// does} — and this is the cell the suite had no row for.
#[test]
fn the_emitted_route_runs_verbatim_when_every_stranded_doc_is_refused() {
    let repo = TempDir::new("all-refused-dest");
    let home = TempDir::new("home");
    git_init(repo.path());
    install_note_pack(repo.path(), "note-pack");

    // ONE stranded doc, and its name is not a doc id — so every row of the sweep is
    // refused and nothing in the run creates the `note` home.
    fs::create_dir_all(repo.path().join("notes")).expect("mk prior home");
    fs::write(
        repo.path().join("notes/My Note.md"),
        "# My Note\n\n## Body\n\nprose\n",
    )
    .expect("write the stranded doc");
    git(repo.path(), &["add", "-A"]);
    git(repo.path(), &["commit", "-q", "-m", "strand one note"]);

    let out = jigc(
        repo.path(),
        home.path(),
        &["relocate", "note", "--from", "notes"],
    );
    let report = stdout_of(&out);
    assert!(
        report.contains("ingest.unaddressable-identity"),
        "the only stranded doc must be refused; got:\n{report}"
    );
    assert!(
        !repo.path().join("docs/notes").exists(),
        "the premise of the cell: an all-refused run moves nothing, so nothing creates \
         the home — got a `docs/notes` on disk;\n{report}"
    );

    // The EMITTED route, run verbatim through a real shell from the repo root. This is the
    // whole claim that a refusal is not a dead end.
    let emitted = backticked_move_route(&report)
        .unwrap_or_else(|| panic!("the refusal must emit a runnable repair; got:\n{report}"));
    let ran = Command::new("sh")
        .arg("-c")
        .arg(&emitted)
        .current_dir(repo.path())
        .output()
        .expect("run the emitted route");
    assert!(
        ran.status.success(),
        "the emitted route must run verbatim with no addressable sibling to have created \
         the home: `{emitted}`\nexit: {:?}\nstderr:\n{}",
        ran.status.code(),
        String::from_utf8_lossy(&ran.stderr),
    );

    // …and after it the doc is at the home under an identity the read surface resolves.
    let adopted = jigc(repo.path(), home.path(), &["ingest"]);
    assert_ok(&adopted, "`jigc ingest` after the emitted route");
    let shown = jigc(repo.path(), home.path(), &["doc", "show", "note:my-note"]);
    assert!(
        shown.status.success(),
        "after the route the doc must be addressable; ingest said:\n{}\nstderr:\n{}",
        stdout_of(&adopted),
        String::from_utf8_lossy(&shown.stderr),
    );
}

/// **The omitting context: a `placement:` destination is never gated** (M52 Increment 8 /
/// T6).
///
/// A fixed-identity doctype's slug is the type id, so the filename at its one home
/// supplies nothing to the identity — `Digest Of Notes.md` reads as `digest:digest` and
/// always has. The identical name that blocks a `location:` relocation above must
/// therefore relocate here untouched: the gate is inert in the context that omits its
/// subject, never an error.
#[test]
fn a_placement_destination_relocates_even_when_its_file_name_is_not_a_slug() {
    let repo = TempDir::new("placement-dest");
    let home = TempDir::new("home");
    git_init(repo.path());
    let pack = repo.path().join(".jigc").join("digest-pack");
    write_digest_pack(&pack);
    fs::write(
        repo.path().join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", pack.display()),
    )
    .expect("write packs.yaml naming the digest pack");

    let prior_rel = "legacy/Digest Of Notes.md";
    fs::create_dir_all(repo.path().join("legacy")).expect("mk prior home");
    fs::write(
        repo.path().join(prior_rel),
        "# Digest Of Notes\n\n## Body\n\nprose\n",
    )
    .expect("write the stranded digest");
    git(repo.path(), &["add", "-A"]);
    git(repo.path(), &["commit", "-q", "-m", "strand the digest"]);

    let out = jigc(
        repo.path(),
        home.path(),
        &["relocate", "digest", "--from", prior_rel],
    );
    assert_ok(&out, "`jigc relocate digest --from <prior placement home>`");
    let report = stdout_of(&out);
    assert!(
        !report.contains("ingest.unaddressable-identity"),
        "a fixed-identity home's file name supplies no identity — the gate must be inert \
         here; got:\n{report}"
    );
    assert!(
        repo.path().join("Digest Of Notes.md").exists(),
        "the placement instance must land at its one home; got:\n{report}"
    );
}

// ── the refusal axis (M52 completion audit, fix 5) ──────────────────────────────────

/// The fixture pack for the axis arm: the freeze-exempt `note` doctype plus the two shapes
/// only a manufactured pack can reach — a **transient** doctype (no `location:`, no
/// `placement:`), and one whose declared home is a path git **cannot record**.
///
/// Both are genuinely unreachable on the shipped packs, which is the whole reason the axis
/// arm manufactures them: every shipped doctype is manifest-governed, so the freeze-exempt
/// path refuses them one gate earlier, and no shipped schema declares a home under `.git/`.
fn write_axis_pack(dir: &Path) {
    write_note_pack(dir);
    fs::write(
        dir.join("schemas").join("memo.yaml"),
        "type: memo\nid-from: title\nsections:\n  - id: body\n    slot: { hint: \"The memo.\" }\n",
    )
    .expect("write the transient memo schema");
    fs::write(
        dir.join("schemas").join("vault.yaml"),
        "type: vault\nlocation: .git/vault/\nid-from: title\nsections:\n  - id: body\n    slot: { hint: \"The vault note.\" }\n",
    )
    .expect("write the untrackable-home vault schema");
}

/// **Every refusal `jigc relocate` raises names itself and routes** — the route floor
/// (`design/surface-contract.md` → law 2) over this door's whole refusal surface, driven
/// through the real binary.
///
/// The set it iterates is the **code-side registry**
/// [`cli::relocate::RelocateRefusal::ALL`], not the six the audit's finding reported: at
/// `26d021de` six of this door's nine states reached the wire as bare `anyhow` strings
/// (`relocate --from ''`, `--from /`, the frozen doctype, the transient doctype, the
/// occupied destination, the untrackable destination) while three already carried a code —
/// and a fix that took the reported six would have left the registry a list of what one
/// commit touched rather than a statement about the door. The three that were already right
/// are the arm's **controls**: they are driven here too, so the axis is total and a member
/// added later has to be driven or the ⇔ fence below reds.
///
/// Two surfaces, because the door has two: an argument or doctype refusal is a fact about
/// the **run** and lands on stderr at exit 1, while a per-document refusal is a fact about
/// **one candidate** and lands as a `blocked` row inside the report — a triage door
/// classifies the rest, which is what a sweep is for (`design/validation.md`, the
/// `ingest.unaddressable-identity` row's exit clause). Both must carry the code and the
/// route; neither may reach the wire as a bare sentence.
#[test]
fn every_relocate_refusal_carries_an_identity_and_a_route() {
    use cli::relocate::RelocateRefusal as R;

    let repo = TempDir::new("refusal-axis");
    let home = TempDir::new("home");
    git_init(repo.path());
    let pack = repo.path().join(".jigc").join("axis-pack");
    write_axis_pack(&pack);
    fs::write(
        repo.path().join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", pack.display()),
    )
    .expect("write packs.yaml naming the axis pack");

    // One stranded `note` at a legacy home — the subject every per-document cell moves.
    fs::create_dir_all(repo.path().join("legacy")).expect("mk prior home");
    fs::write(
        repo.path().join("legacy/stranded.md"),
        "# Stranded\n\n## Body\n\nprose\n",
    )
    .expect("write the stranded note");
    // …and a committed instance already sitting at the `note` home, baselined, so the
    // occupied-destination cell meets a **managed** squatter rather than a foreign one.
    fs::create_dir_all(repo.path().join("docs/notes")).expect("mk note home");
    fs::write(
        repo.path().join("docs/notes/stranded.md"),
        "# Stranded\n\n## Body\n\nother prose\n",
    )
    .expect("write the occupying note");
    git(repo.path(), &["add", "-A"]);
    git(repo.path(), &["commit", "-q", "-m", "strand a note"]);
    assert_ok(
        &jigc(repo.path(), home.path(), &["ingest"]),
        "`jigc ingest` baselines the occupying instance so the destination reads *managed*",
    );

    /// Where a refusal lands: `Run` is stderr at exit 1, `Row` is a `blocked` row in the
    /// report the door prints on stdout.
    #[derive(Clone, Copy, PartialEq)]
    enum Surface {
        Run,
        Row,
    }

    let cells: &[(R, Surface, &[&str])] = &[
        // The three controls — already coded before this fix.
        (
            R::UnknownDoctype,
            Surface::Run,
            &["relocate", "nosuch", "--from", "legacy"],
        ),
        (
            R::WorkbenchPriorHome,
            Surface::Run,
            &["relocate", "note", "--from", ".jigc"],
        ),
        (
            R::InstalledPriorHome,
            Surface::Run,
            &["relocate", "note", "--from", ".claude"],
        ),
        // The six the finding named.
        (
            R::PriorHomeMissing,
            Surface::Run,
            &["relocate", "note", "--from", ""],
        ),
        (
            R::PriorHomeUnusable,
            Surface::Run,
            &["relocate", "note", "--from", "/"],
        ),
        (
            R::FrozenDoctype,
            Surface::Run,
            &["relocate", "adr", "--from", "legacy"],
        ),
        (
            R::TransientDoctype,
            Surface::Run,
            &["relocate", "memo", "--from", "legacy"],
        ),
        (
            R::UntrackableDestination,
            Surface::Row,
            &["relocate", "vault", "--from", "legacy"],
        ),
        (
            R::OccupiedDestination,
            Surface::Row,
            &["relocate", "note", "--from", "legacy"],
        ),
        // The tenth, shipped at Increment 8 / T6 and fenced in full by
        // `an_unaddressable_destination_is_refused_and_the_addressable_sibling_still_moves`
        // above; driven here for the axis's own two properties (code + route), over a
        // source whose file name no `<type>:<slug>` address reaches.
        (
            R::UnaddressableDestination,
            Surface::Row,
            &["relocate", "note", "--from", "unaddressable"],
        ),
        // The eleventh (the rc.24 fix pass): a stranded entry that is a **link**. `git mv`
        // would carry it to the doctype's home as the link it is, so the move primitive
        // refuses it under the code every door raises for a doc's home in that state. The
        // nothing-moved and route-follow halves are `store_door_home_shape.rs`'s and
        // `cli::relocate`'s own unit test; this is the member's seat on the axis.
        (
            R::ForeignSource,
            Surface::Row,
            &["relocate", "note", "--from", "linked"],
        ),
    ];

    // The unaddressable cell's own subject — a stranded doc whose name is not a doc id.
    fs::create_dir_all(repo.path().join("unaddressable")).expect("mk unaddressable home");
    fs::write(
        repo.path().join("unaddressable/My Note.md"),
        "# My Note\n\n## Body\n\nprose\n",
    )
    .expect("write the unaddressable-named note");
    // The foreign-source cell's own subject — a committed link under a prior home, to a
    // file that is no doc's home.
    fs::create_dir_all(repo.path().join("linked")).expect("mk the linked prior home");
    fs::create_dir_all(repo.path().join("targets")).expect("mk the link's target dir");
    fs::write(
        repo.path().join("targets/real.txt"),
        "# Pointer\n\n## Body\n\nprose\n",
    )
    .expect("write the link's target");
    std::os::unix::fs::symlink("../targets/real.txt", repo.path().join("linked/pointer.md"))
        .expect("strand a link");
    git(repo.path(), &["add", "-A"]);
    git(
        repo.path(),
        &["commit", "-q", "-m", "strand an unaddressable note"],
    );

    for (refusal, surface, argv) in cells {
        let out = jigc(repo.path(), home.path(), argv);
        let (channel, text) = match surface {
            Surface::Run => ("stderr", String::from_utf8_lossy(&out.stderr).into_owned()),
            Surface::Row => ("stdout", String::from_utf8_lossy(&out.stdout).into_owned()),
        };
        let code = refusal.code();
        assert!(
            text.contains(&format!("blocking · {code}")),
            "{argv:?}: the refusal must name itself `blocking · {code}` on {channel}; got:\n{text}",
        );
        assert!(
            text.contains("route: "),
            "{argv:?} ({code}): every blocking finding carries a route (law 2); got:\n{text}",
        );
        if *surface == Surface::Run {
            assert_eq!(
                out.status.code(),
                Some(1),
                "{argv:?} ({code}): a run-level refusal exits 1; got:\n{text}",
            );
            // …and the machine arm carries the same identity: `--format json` may never
            // answer a refusal with a sentence that has no code in it.
            let mut json_argv: Vec<&str> = argv.to_vec();
            json_argv.extend(["--format", "json"]);
            let json_out = jigc(repo.path(), home.path(), &json_argv);
            let json = String::from_utf8_lossy(&json_out.stderr).into_owned();
            assert!(
                json.contains(code),
                "{json_argv:?}: the json arm carries the code too; got:\n{json}",
            );
        }
    }

    // **The ⇔ fence**: every registry member is driven above, and every cell names a member.
    let driven: Vec<R> = cells.iter().map(|(r, _, _)| *r).collect();
    for member in R::ALL {
        assert!(
            driven.contains(member),
            "{member:?} is a member of `RelocateRefusal::ALL` that no cell drives — a \
             refusal nothing drives is a route-floor claim nothing checks",
        );
    }
    assert_eq!(
        driven.len(),
        R::ALL.len(),
        "the cell table and the registry are the same set, member for member",
    );
}
