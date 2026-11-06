//! **Which commits of a range has no recorded review read — by commit membership, never by
//! name** ([`dev/review-coverage`](../dev/review-coverage); the plan before the opening's
//! task `U5`, the instrument of its partition `J2-0` —
//! [detail.md](../completions/artifacts/M55/stabilization-build/pre-opening-plan/detail.md);
//! the human's ruling F-B → A, [DECISIONS.md](../DECISIONS.md) → *2026-10-29 — The three
//! forks of plan v2's review*).
//!
//! Every arm but the last builds one throwaway repository whose history has every shape the
//! rule has to tell apart, and drives the committed script over it with a registry of its
//! own:
//!
//! - **(a)** the listing: a commit no review covers is listed under each file it touched,
//!   with the lines it added; a commit inside a registered range whose paths hold the file
//!   is not — whole bytes, every file, the summary line included.
//! - **(b)** a review scoped by path covers only the files its paths hold: the other file
//!   the same commit touched stays listed.
//! - **(c)** overlapping reviews: a commit two ranges hold is covered by both, and
//!   `--covered` names both, sorted.
//! - **(d)** a merge whose tree is the automatic re-merge of its parents adds nothing of its
//!   own: it is never listed, and its parents' commits are listed on their own merits.
//! - **(e)** a merge whose tree differs — a conflict resolved by hand — is listed by the
//!   files where it differs and no other, until a registered review covers it.
//! - **(f)** the paths of a listing narrow it to the files they hold.
//! - **(g)** the order of the registry's rows never reaches the output: the registry read
//!   forwards and reversed gives byte-identical listings.
//! - **(h)** `--check`: every commit a row names is in `HEAD`'s history and every heading
//!   stands in `DECISIONS.md` exactly once — each broken in turn, each named.
//! - **(i)** a malformed registry is refused before anything is listed: an abbreviated sha,
//!   a missing field, a duplicate review, a commit the repository does not have.
//! - **(j)** the committed registry passes `--check` in this repository.
//!
//! And from the review of `c668e4fd` (U5's review — `DECISIONS.md` → the U5 entry):
//!
//! - **(k)** `--check` refuses a range that claims what its review can never have read: a
//!   base off its tip's line, a range across a merge, a range whose base is a merge — and
//!   names every commit of a row not in `HEAD`'s history, not the first alone; a listing
//!   over such a registry lists nothing.
//! - **(l)** a row's paths are repository-relative and literal wherever the tool runs: a
//!   subdirectory changes nothing, a path is no prefix of a longer name, a `*` is no glob.
//! - **(m)** a merge resolved by taking one side is judged against the re-merge, never
//!   against its first parent; a commit that only removes lines is listed; an octopus
//!   merge is refused.
//! - **(n)** what used to crash is refused or read: a registry that is not UTF-8, a
//!   repository whose log shows signatures, a merge of unrelated histories.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const SCRIPT: &str = "dev/review-coverage";
const HEADER: &str = "review\trange\tpaths\theading\n";

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crates/cli has a repo root two levels up")
        .to_path_buf()
}

/// A throwaway directory that removes itself on drop.
struct Scratch(PathBuf);

impl Scratch {
    fn new(tag: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "jigc-review-coverage-{tag}-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        ));
        fs::create_dir_all(&path).expect("create the scratch directory");
        Scratch(path)
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// A command with this machine's git configuration masked.
fn hermetic(program: &Path, cwd: &Path) -> Command {
    let mut c = Command::new(program);
    c.current_dir(cwd)
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("HOME", cwd)
        .env("GIT_AUTHOR_NAME", "fixture")
        .env("GIT_AUTHOR_EMAIL", "fixture@example.invalid")
        .env("GIT_COMMITTER_NAME", "fixture")
        .env("GIT_COMMITTER_EMAIL", "fixture@example.invalid");
    c
}

fn git(repo: &Path, args: &[&str]) -> String {
    let out = hermetic(Path::new("git"), repo)
        .args(args)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).expect("utf-8 git stdout")
}

fn write(repo: &Path, path: &str, text: &str) {
    let p = repo.join(path);
    fs::create_dir_all(p.parent().expect("a parent")).expect("mkdir");
    fs::write(p, text).expect("write a fixture file");
}

/// One commit at a fixed date, so the history and its order are the same on every run.
fn commit(repo: &Path, n: u32, subject: &str) -> String {
    let date = format!("2026-01-01T00:{n:02}:00Z");
    let out = hermetic(Path::new("git"), repo)
        .env("GIT_AUTHOR_DATE", &date)
        .env("GIT_COMMITTER_DATE", &date)
        .args(["commit", "-qm", subject])
        .output()
        .expect("run git commit");
    assert!(
        out.status.success(),
        "commit {subject}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    git(repo, &["rev-parse", "HEAD"]).trim().to_string()
}

fn merge(repo: &Path, n: u32, branch: &str, subject: &str) -> std::process::ExitStatus {
    let date = format!("2026-01-01T00:{n:02}:00Z");
    hermetic(Path::new("git"), repo)
        .env("GIT_AUTHOR_DATE", &date)
        .env("GIT_COMMITTER_DATE", &date)
        .args(["merge", "-q", "--no-ff", "-m", subject, branch])
        .output()
        .expect("run git merge")
        .status
}

const DECISIONS: &str = "# Decisions\n\n\
## 2026-01-02 — The scoped review\n\nBody.\n\n\
## 2026-01-02 — The wide review\n\nBody.\n\n\
## 2026-01-02 — The overlapping review\n\nBody.\n\n\
## 2026-01-03 — The merge's review\n\nBody.\n";

/// The fixture's history, every shape the rule tells apart:
///
/// ```text
/// c0 root ── c1 (a.txt +1, b.txt +2) ── c2 (a.txt +1) ─┬─ c3 (d.txt +1) ─ m1 ─ c4 (a.txt) ─ m2
///                                                      └─ s1 (c.txt +3) ─┘    └ x1 (a.txt) ┘
/// ```
///
/// `m1` merges `side` cleanly; `m2` merges `x` over a conflict in `a.txt`, resolved by hand.
struct Fixture {
    _scratch: Scratch,
    repo: PathBuf,
    registry: PathBuf,
    c0: String,
    c1: String,
    c2: String,
    c3: String,
    s1: String,
    m1: String,
    c4: String,
    x1: String,
    m2: String,
}

fn fixture(tag: &str) -> Fixture {
    let scratch = Scratch::new(tag);
    let repo = scratch.0.join("repo");
    fs::create_dir_all(&repo).expect("mkdir repo");
    let r = repo.as_path();
    git(r, &["init", "-q", "-b", "main"]);
    write(r, "a.txt", "one\n");
    write(r, "b.txt", "b\n");
    write(r, "DECISIONS.md", DECISIONS);
    git(r, &["add", "-A"]);
    let c0 = commit(r, 0, "root");
    write(r, "a.txt", "one\ntwo\n");
    write(r, "b.txt", "b\nb2\nb3\n");
    git(r, &["add", "-A"]);
    let c1 = commit(r, 1, "c1 touches a and b");
    write(r, "a.txt", "one\ntwo\nthree\n");
    git(r, &["add", "-A"]);
    let c2 = commit(r, 2, "c2 touches a");
    git(r, &["switch", "-q", "-c", "side"]);
    write(r, "c.txt", "c1\nc2\nc3\n");
    git(r, &["add", "-A"]);
    let s1 = commit(r, 3, "s1 adds c");
    git(r, &["switch", "-q", "main"]);
    write(r, "d.txt", "d\n");
    git(r, &["add", "-A"]);
    let c3 = commit(r, 4, "c3 adds d");
    assert!(merge(r, 5, "side", "m1 merges side cleanly").success());
    let m1 = git(r, &["rev-parse", "HEAD"]).trim().to_string();
    git(r, &["switch", "-q", "-c", "x"]);
    write(r, "a.txt", "x\ntwo\nthree\n");
    git(r, &["add", "-A"]);
    let x1 = commit(r, 6, "x1 rewrites a's first line");
    git(r, &["switch", "-q", "main"]);
    write(r, "a.txt", "y\ntwo\nthree\n");
    git(r, &["add", "-A"]);
    let c4 = commit(r, 7, "c4 rewrites a's first line too");
    assert!(
        !merge(r, 8, "x", "m2").success(),
        "the fixture's second merge must stop on its conflict"
    );
    write(r, "a.txt", "z\ntwo\nthree\n");
    git(r, &["add", "-A"]);
    let m2 = commit(r, 8, "m2 merges x over a conflict");
    let registry = scratch.0.join("registry.tsv");
    Fixture {
        _scratch: scratch,
        repo,
        registry,
        c0,
        c1,
        c2,
        c3,
        s1,
        m1,
        c4,
        x1,
        m2,
    }
}

impl Fixture {
    /// The fixture's three reviews: `scoped` reads `a.txt` of `c1`; `wide` reads all of
    /// `c2`; `overlap` reads `a.txt` of `c1` and `c2`.
    fn rows(&self) -> Vec<String> {
        vec![
            format!(
                "scoped\t{}\ta.txt\t2026-01-02 — The scoped review\n",
                self.c1
            ),
            format!(
                "wide\t{}..{}\t.\t2026-01-02 — The wide review\n",
                self.c1, self.c2
            ),
            format!(
                "overlap\t{}^..{}\ta.txt\t2026-01-02 — The overlapping review\n",
                self.c1, self.c2
            ),
        ]
    }

    fn registry(&self, rows: &[String]) {
        let mut text = String::from("# The fixture's reviews.\n");
        text.push_str(HEADER);
        for row in rows {
            text.push_str(row);
        }
        fs::write(&self.registry, text).expect("write the registry");
    }

    fn run(&self, args: &[&str]) -> Output {
        hermetic(&repo_root().join(SCRIPT), &self.repo)
            .arg("--registry")
            .arg(&self.registry)
            .args(args)
            .output()
            .expect("run dev/review-coverage")
    }

    fn listing(&self, args: &[&str]) -> String {
        let out = self.run(args);
        assert_eq!(
            out.status.code(),
            Some(0),
            "a listing exits 0; stderr: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        String::from_utf8(out.stdout).expect("utf-8 listing")
    }

    fn whole(&self) -> String {
        format!("{}..{}", self.c0, self.m2)
    }
}

fn short(sha: &str) -> &str {
    &sha[..12]
}

fn entry(sha: &str, added: u32, removed: u32, state: &str, subject: &str) -> String {
    format!(
        "\t{}\t+{added}\t-{removed}\t{state}\t{subject}\n",
        short(sha)
    )
}

#[test]
fn a_the_listing_names_every_uncovered_commit_under_each_file_it_touched() {
    let f = fixture("a");
    f.registry(&f.rows());
    let got = f.listing(&[&f.whole()]);
    let want = [
        "a.txt\n".to_string(),
        entry(&f.c4, 1, 1, "uncovered", "c4 rewrites a's first line too"),
        entry(&f.x1, 1, 1, "uncovered", "x1 rewrites a's first line"),
        entry(&f.m2, 1, 5, "uncovered-merge", "m2 merges x over a conflict"),
        "b.txt\n".to_string(),
        entry(&f.c1, 2, 0, "uncovered", "c1 touches a and b"),
        "c.txt\n".to_string(),
        entry(&f.s1, 3, 0, "uncovered", "s1 adds c"),
        "d.txt\n".to_string(),
        entry(&f.c3, 1, 0, "uncovered", "c3 adds d"),
        "review-coverage: 6 uncovered entries on 4 files; 1 clean merge covered through its parents\n"
            .to_string(),
    ]
    .concat();
    assert_eq!(got, want);
}

#[test]
fn b_a_review_scoped_by_path_covers_only_the_files_its_paths_hold() {
    let f = fixture("b");
    f.registry(&f.rows()[..1]);
    let got = f.listing(&[&format!("{}^..{}", f.c1, f.c1)]);
    assert_eq!(
        got,
        [
            "b.txt\n".to_string(),
            entry(&f.c1, 2, 0, "uncovered", "c1 touches a and b"),
            "review-coverage: 1 uncovered entry on 1 file; 0 clean merges covered through their parents\n".to_string(),
        ]
        .concat(),
        "`scoped` reads a.txt of c1: a.txt is covered, b.txt is not"
    );
}

#[test]
fn c_overlapping_reviews_each_cover_and_are_named_sorted() {
    let f = fixture("c");
    f.registry(&f.rows());
    let got = f.listing(&["--covered", &format!("{}..{}", f.c0, f.c2)]);
    assert_eq!(
        got,
        [
            "a.txt\n".to_string(),
            entry(&f.c1, 1, 0, "covered:overlap,scoped", "c1 touches a and b"),
            entry(&f.c2, 1, 0, "covered:overlap,wide", "c2 touches a"),
            "b.txt\n".to_string(),
            entry(&f.c1, 2, 0, "uncovered", "c1 touches a and b"),
            "review-coverage: 1 uncovered entry on 1 file; 0 clean merges covered through their parents\n".to_string(),
        ]
        .concat()
    );
}

#[test]
fn d_a_clean_merge_is_covered_through_its_parents_and_never_listed() {
    let f = fixture("d");
    f.registry(&f.rows());
    let range = format!("{}..{}", f.c2, f.m1);
    let got = f.listing(&["--covered", &range]);
    assert!(
        !got.contains(short(&f.m1)),
        "m1's tree is the re-merge of its parents, so it adds nothing to list:\n{got}"
    );
    assert_eq!(
        got,
        [
            "c.txt\n".to_string(),
            entry(&f.s1, 3, 0, "uncovered", "s1 adds c"),
            "d.txt\n".to_string(),
            entry(&f.c3, 1, 0, "uncovered", "c3 adds d"),
            "review-coverage: 2 uncovered entries on 2 files; 1 clean merge covered through its parents\n".to_string(),
        ]
        .concat(),
        "the parents' commits are listed on their own merits"
    );
}

#[test]
fn e_a_merge_with_a_resolved_conflict_is_listed_by_the_files_where_it_differs() {
    let f = fixture("e");
    f.registry(&f.rows());
    let range = format!("{}^!", f.m2);
    let got = f.listing(&[&range]);
    assert_eq!(
        got,
        [
            "a.txt\n".to_string(),
            entry(&f.m2, 1, 5, "uncovered-merge", "m2 merges x over a conflict"),
            "review-coverage: 1 uncovered entry on 1 file; 0 clean merges covered through their parents\n".to_string(),
        ]
        .concat(),
        "m2 differs from the re-merge of its parents in a.txt alone"
    );
    let mut rows = f.rows();
    rows.push(format!(
        "merge\t{}\ta.txt\t2026-01-03 — The merge's review\n",
        f.m2
    ));
    f.registry(&rows);
    assert_eq!(
        f.listing(&["--covered", &range]),
        [
            "a.txt\n".to_string(),
            entry(&f.m2, 1, 5, "covered:merge", "m2 merges x over a conflict"),
            "review-coverage: 0 uncovered entries on 0 files; 0 clean merges covered through their parents\n".to_string(),
        ]
        .concat(),
        "a registered review of the merge covers it"
    );
}

#[test]
fn f_the_listings_paths_narrow_it_to_the_files_they_hold() {
    let f = fixture("f");
    f.registry(&f.rows());
    let got = f.listing(&[&f.whole(), "c.txt", "d.txt"]);
    assert_eq!(
        got,
        [
            "c.txt\n".to_string(),
            entry(&f.s1, 3, 0, "uncovered", "s1 adds c"),
            "d.txt\n".to_string(),
            entry(&f.c3, 1, 0, "uncovered", "c3 adds d"),
            "review-coverage: 2 uncovered entries on 2 files; 1 clean merge covered through its parents\n".to_string(),
        ]
        .concat()
    );
}

#[test]
fn g_the_registrys_row_order_never_reaches_the_output() {
    let f = fixture("g");
    let mut rows = f.rows();
    rows.push(format!(
        "merge\t{}\ta.txt\t2026-01-03 — The merge's review\n",
        f.m2
    ));
    f.registry(&rows);
    let forwards = f.listing(&["--covered", &f.whole()]);
    rows.reverse();
    f.registry(&rows);
    let reversed = f.listing(&["--covered", &f.whole()]);
    assert_eq!(forwards, reversed);
    assert!(
        forwards.contains("covered:overlap,scoped") && forwards.contains("covered:overlap,wide"),
        "the fixture's overlap must reach the output for the comparison to mean anything:\n{forwards}"
    );
}

#[test]
fn h_check_holds_every_row_to_heads_history_and_to_one_heading() {
    let f = fixture("h");
    f.registry(&f.rows());
    let out = f.run(&["--check"]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "a sound registry passes; stdout: {} stderr: {}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    // A heading the log does not hold, and one it holds twice.
    let mut rows = f.rows();
    rows[0] = rows[0].replace("The scoped review", "A review nobody recorded");
    f.registry(&rows);
    let out = f.run(&["--check"]);
    assert_eq!(out.status.code(), Some(1));
    assert_eq!(
        String::from_utf8_lossy(&out.stdout),
        "check: scoped: no heading `2026-01-02 — A review nobody recorded` in DECISIONS.md\n"
    );
    write(
        &f.repo,
        "DECISIONS.md",
        &format!("{DECISIONS}\n## 2026-01-02 — The wide review\n\nAgain.\n"),
    );
    f.registry(&f.rows());
    let out = f.run(&["--check"]);
    assert_eq!(out.status.code(), Some(1));
    assert_eq!(
        String::from_utf8_lossy(&out.stdout),
        "check: wide: the heading `2026-01-02 — The wide review` stands 2 times in DECISIONS.md\n"
    );
    write(&f.repo, "DECISIONS.md", DECISIONS);

    // A commit the repository has, and HEAD's history does not.
    git(&f.repo, &["switch", "-q", "-c", "elsewhere", &f.c2]);
    write(&f.repo, "e.txt", "e\n");
    git(&f.repo, &["add", "-A"]);
    let off = commit(&f.repo, 9, "a commit off main");
    git(&f.repo, &["switch", "-q", "main"]);
    let mut rows = f.rows();
    rows.push(format!("off\t{off}\t.\t2026-01-03 — The merge's review\n"));
    f.registry(&rows);
    let out = f.run(&["--check"]);
    assert_eq!(out.status.code(), Some(1));
    assert_eq!(
        String::from_utf8_lossy(&out.stdout),
        format!("check: off: {off} is not in HEAD's history\n")
    );
}

#[test]
fn i_a_malformed_registry_is_refused_before_anything_is_listed() {
    let f = fixture("i");
    let cases: Vec<(String, &str)> = vec![
        (
            format!("short\t{}\t.\tH\n", short(&f.c1)),
            "a range item is a full 40-hex sha, `A..B` or `A^..B`",
        ),
        (format!("missing\t{}\t.\n", f.c1), "four fields"),
        (
            format!("dup\t{}\t.\tH\ndup\t{}\t.\tH\n", f.c1, f.c2),
            "review `dup` is registered twice",
        ),
        (
            format!("absent\t{}\t.\tH\n", "0".repeat(40)),
            "not a commit of this repository",
        ),
    ];
    for (rows, said) in cases {
        f.registry(std::slice::from_ref(&rows));
        let out = f.run(&[&f.whole()]);
        let stderr = String::from_utf8_lossy(&out.stderr);
        assert_eq!(out.status.code(), Some(2), "refused: {rows}");
        assert!(stderr.contains(said), "names why ({said}): {stderr}");
        assert!(out.stdout.is_empty(), "nothing is listed: {rows}");
    }
}

#[test]
fn j_the_committed_registry_passes_check_in_this_repository() {
    let root = repo_root();
    let out = Command::new(root.join(SCRIPT))
        .current_dir(&root)
        .arg("--check")
        .output()
        .expect("run dev/review-coverage --check");
    assert_eq!(
        out.status.code(),
        Some(0),
        "the committed registry: stdout: {} stderr: {}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
}

/// A commit of `files` (each a path and its text, `None` to remove it) on what is checked out
/// — those paths alone: the tool's own runs leave a Python cache under the fixture's `HOME`.
fn change(repo: &Path, n: u32, files: &[(&str, Option<&str>)], subject: &str) -> String {
    for (path, text) in files {
        match text {
            Some(text) => {
                write(repo, path, text);
                git(repo, &["add", "--", path]);
            }
            None => {
                git(repo, &["rm", "-q", path]);
            }
        }
    }
    commit(repo, n, subject)
}

#[test]
fn k_check_refuses_a_range_that_claims_what_its_review_never_read() {
    let f = fixture("k");
    let heading = "2026-01-03 — The merge's review";
    for (range, said) in [
        // A base on the side the merge brought in: c3, m1 and c4, for a review of c4.
        (
            format!("{}..{}", f.s1, f.c4),
            vec![format!("{}..{}: holds 1 merge", f.s1, f.c4)],
        ),
        // A base whose child is a merge: the merge's side comes along.
        (
            format!("{}^..{}", f.m1, f.c4),
            vec![format!("{}^..{}: holds 1 merge", f.m1, f.c4)],
        ),
        // A linear-looking range across a merge.
        (
            format!("{}..{}", f.c3, f.c4),
            vec![format!("{}..{}: holds 1 merge", f.c3, f.c4)],
        ),
        // A base off the tip's line: x1 was not merged when c4 was made.
        (
            format!("{}..{}", f.x1, f.c4),
            vec![format!(
                "{}..{}: its base is not an ancestor of its tip",
                f.x1, f.c4
            )],
        ),
    ] {
        f.registry(&[format!("wide\t{range}\t.\t{heading}\n")]);
        let out = f.run(&["--check"]);
        let stdout = String::from_utf8_lossy(&out.stdout);
        assert_eq!(out.status.code(), Some(1), "{range}: refused.\n{stdout}");
        for want in &said {
            assert!(
                stdout
                    .lines()
                    .any(|l| l.starts_with(&format!("check: wide: {want}"))),
                "{range}: names `{want}`.\n{stdout}"
            );
        }
        // And a listing over it lists nothing (L1).
        let listed = f.run(&[&f.whole()]);
        assert_eq!(listed.status.code(), Some(2), "{range}: no listing");
        assert!(listed.stdout.is_empty(), "{range}: nothing is listed");
        assert!(
            String::from_utf8_lossy(&listed.stderr).contains("fails --check"),
            "{range}: {}",
            String::from_utf8_lossy(&listed.stderr)
        );
    }

    // Every commit of a row is held to HEAD's history, not one of them alone.
    git(&f.repo, &["switch", "-q", "-c", "elsewhere", &f.c2]);
    write(&f.repo, "e.txt", "e\n");
    git(&f.repo, &["add", "-A"]);
    let off = commit(&f.repo, 9, "a commit off main");
    git(&f.repo, &["switch", "-q", "main"]);
    f.registry(&[format!("part\t{} {}..{off}\t.\t{heading}\n", f.c1, f.c2)]);
    let out = f.run(&["--check"]);
    assert_eq!(out.status.code(), Some(1));
    assert_eq!(
        String::from_utf8_lossy(&out.stdout),
        format!("check: part: {off} is not in HEAD's history\n")
    );

    // A sound registry passes, with each row's commit count.
    f.registry(&f.rows());
    let out = f.run(&["--check"]);
    assert_eq!(out.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&out.stdout);
    for line in [
        "check: scoped: 1 commit\n",
        "check: wide: 1 commit\n",
        "check: overlap: 2 commits\n",
    ] {
        assert!(stdout.contains(line), "`{line}`: {stdout}");
    }
}

#[test]
fn l_a_rows_paths_are_repository_relative_and_literal_wherever_it_runs() {
    let f = fixture("l");
    let cn = change(
        &f.repo,
        9,
        &[
            ("dev/x", Some("x\n")),
            ("sub/dev/x", Some("x\n")),
            ("tools/review-coverage", Some("t\n")),
            ("e.txt", Some("e\n")),
        ],
        "cn touches four paths",
    );
    f.registry(&[format!(
        "paths\t{cn}\tdev tools/review *.txt\t2026-01-03 — The merge's review\n"
    )]);
    let range = format!("{cn}^!");
    let want = [
        "e.txt\n".to_string(),
        entry(&cn, 1, 0, "uncovered", "cn touches four paths"),
        "sub/dev/x\n".to_string(),
        entry(&cn, 1, 0, "uncovered", "cn touches four paths"),
        "tools/review-coverage\n".to_string(),
        entry(&cn, 1, 0, "uncovered", "cn touches four paths"),
        "review-coverage: 3 uncovered entries on 3 files; 0 clean merges covered through their parents\n"
            .to_string(),
    ]
    .concat();
    assert_eq!(f.listing(&[&range]), want, "from the top");
    let out = hermetic(&repo_root().join(SCRIPT), &f.repo.join("sub"))
        .arg("--registry")
        .arg(&f.registry)
        .arg(&range)
        .output()
        .expect("run dev/review-coverage from a subdirectory");
    assert_eq!(
        String::from_utf8_lossy(&out.stdout),
        want,
        "from a subdirectory: the same. stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn m_a_merge_taking_one_side_a_removal_and_an_octopus() {
    let f = fixture("m");
    let r = f.repo.as_path();
    git(r, &["switch", "-q", "-c", "y"]);
    change(r, 9, &[("a.txt", Some("p\ntwo\nthree\n"))], "y1 rewrites a");
    git(r, &["switch", "-q", "main"]);
    change(
        r,
        10,
        &[("a.txt", Some("q\ntwo\nthree\n"))],
        "c5 rewrites a",
    );
    assert!(!merge(r, 11, "y", "m3").success(), "a conflict");
    git(r, &["checkout", "--ours", "a.txt"]);
    git(r, &["add", "a.txt"]);
    let m3 = commit(r, 11, "m3 takes ours");
    let y1 = git(r, &["rev-parse", "y"]).trim().to_string();
    let gone = change(r, 12, &[("d.txt", None)], "c6 removes d");
    f.registry(&f.rows());
    assert_eq!(
        f.listing(&[&format!("{m3}^..{gone}")]),
        [
            "a.txt\n".to_string(),
            entry(&y1, 1, 1, "uncovered", "y1 rewrites a"),
            entry(&m3, 0, 4, "uncovered-merge", "m3 takes ours"),
            "d.txt\n".to_string(),
            entry(&gone, 0, 1, "uncovered", "c6 removes d"),
            "review-coverage: 3 uncovered entries on 2 files; 0 clean merges covered through their parents\n"
                .to_string(),
        ]
        .concat(),
        "m3 differs from the re-merge where it dropped the conflict; c6 removes a line"
    );

    // An octopus merge has no re-merge, and is refused.
    for (branch, file, n) in [("o1", "o1.txt", 13), ("o2", "o2.txt", 14)] {
        git(r, &["switch", "-q", "-c", branch, &gone]);
        change(r, n, &[(file, Some("o\n"))], branch);
    }
    git(r, &["switch", "-q", "main"]);
    let date = "2026-01-01T00:15:00Z";
    let out = hermetic(Path::new("git"), r)
        .env("GIT_AUTHOR_DATE", date)
        .env("GIT_COMMITTER_DATE", date)
        .args(["merge", "-q", "--no-ff", "-m", "octopus", "o1", "o2"])
        .output()
        .expect("run git merge");
    assert!(out.status.success(), "an octopus: {out:?}");
    let out = f.run(&["HEAD^!"]);
    assert_eq!(out.status.code(), Some(2));
    assert!(
        String::from_utf8_lossy(&out.stderr).contains("merges 3 parents"),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(out.stdout.is_empty());
}

#[test]
fn n_what_used_to_crash_is_refused_or_read() {
    let f = fixture("n");
    // A registry that is not UTF-8: refused, by its byte.
    let mut text = HEADER.as_bytes().to_vec();
    text.extend_from_slice(b"bad\t");
    text.extend_from_slice(f.c1.as_bytes());
    text.extend_from_slice(b"\t.\t\xff\n");
    fs::write(&f.registry, text).expect("write the registry");
    let out = f.run(&[&f.whole()]);
    assert_eq!(out.status.code(), Some(2));
    assert!(
        String::from_utf8_lossy(&out.stderr).contains("is not UTF-8"),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );

    // A repository whose log shows signatures: a signed commit's verdict is no line of the
    // log the tool splits.
    let key = f.repo.parent().expect("the scratch").join("key");
    let made = Command::new("ssh-keygen")
        .args(["-q", "-t", "ed25519", "-N", "", "-f"])
        .arg(&key)
        .output()
        .expect("run ssh-keygen");
    assert!(made.status.success(), "a signing key: {made:?}");
    write(&f.repo, "s.txt", "s\n");
    git(&f.repo, &["add", "s.txt"]);
    let date = "2026-01-01T00:20:00Z";
    let out = hermetic(Path::new("git"), &f.repo)
        .env("GIT_AUTHOR_DATE", date)
        .env("GIT_COMMITTER_DATE", date)
        .args(["-c", "gpg.format=ssh", "-c"])
        .arg(format!("user.signingkey={}", key.display()))
        .args(["commit", "-q", "-S", "-m", "signed"])
        .output()
        .expect("run git commit -S");
    assert!(out.status.success(), "a signed commit: {out:?}");
    let signed = git(&f.repo, &["rev-parse", "HEAD"]).trim().to_string();
    git(&f.repo, &["config", "log.showSignature", "true"]);
    f.registry(&f.rows());
    assert_eq!(
        f.listing(&[&format!("{signed}^!")]),
        [
            "s.txt\n".to_string(),
            entry(&signed, 1, 0, "uncovered", "signed"),
            "review-coverage: 1 uncovered entry on 1 file; 0 clean merges covered through their parents\n"
                .to_string(),
        ]
        .concat()
    );
    git(&f.repo, &["config", "--unset", "log.showSignature"]);

    // A merge of unrelated histories: judged against its re-merge like any other.
    git(&f.repo, &["switch", "-q", "--orphan", "other"]);
    change(
        &f.repo,
        21,
        &[("u.txt", Some("u\n"))],
        "u1 starts another history",
    );
    git(&f.repo, &["switch", "-q", "-f", "main"]);
    let date = "2026-01-01T00:22:00Z";
    let out = hermetic(Path::new("git"), &f.repo)
        .env("GIT_AUTHOR_DATE", date)
        .env("GIT_COMMITTER_DATE", date)
        .args([
            "merge",
            "-q",
            "--no-ff",
            "--allow-unrelated-histories",
            "-m",
            "mu",
            "other",
        ])
        .output()
        .expect("run git merge");
    assert!(
        out.status.success(),
        "a merge of unrelated histories: {out:?}"
    );
    assert_eq!(
        f.listing(&["HEAD^!"]),
        "review-coverage: 0 uncovered entries on 0 files; 1 clean merge covered through its parents\n",
        "the merge is its re-merge: clean"
    );
}
