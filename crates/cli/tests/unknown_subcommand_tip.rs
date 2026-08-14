//! M43 Inc 1 T6 — unknown-subcommand semantic guesses get the **honest sibling tip**,
//! never a silent alias (`DECISIONS.md` 2026-07-16 M43 Settle, cross-cutting; law 2 —
//! nothing hides; trial provenance: A1 papercut, log rec 254).
//!
//! Two guessed verbs agents actually reached for are curated (`crates/cli/src/cli.rs`
//! → `unknown_subcommand_tip`):
//!
//! - `jigc task discard-write` — the ghost verb the reconciliation route used to name
//!   (repaired at T4). clap's own did-you-mean steers to `discard`, which destroys the
//!   WHOLE task, so the tip must state that effect out loud.
//! - `jigc task status` — the tip names what the real siblings actually *do*
//!   (`task list` enumerates; `task validate <id>` previews part of the gate), not a bare
//!   did-you-mean.
//!
//! Each test asserts the **emitted stderr bytes**: clap's own error + usage output is
//! preserved (the guess is still a usage error — never a silent alias, so exit stays 2
//! and nothing dispatches), the honest tip follows it, and a placeholder-free command
//! span extracted from the emitted bytes **runs verbatim** (the emitted bytes are the
//! contract; placeholder-carrying spans are parse-fenced at construction by the T2
//! route fence, live in this debug-build binary). An unknown guess **outside** the
//! curated map stays inert: clap's error unchanged, no tip, exit 2 — never an error in
//! the tip machinery.
//!
//! **M48 Inc 6 T2 — jigc renders the unknown-subcommand block itself** (`DECISIONS.md`
//! 2026-08-13 the Settle, F8 parts 2–3). Letting clap render it printed clap's own
//! `tip: a similar subcommand exists: 'discard'` **above** the curated tip warning
//! against exactly that verb — a law-1 lie on the surface built to remove one. So for
//! this one error kind jigc composes the block (error line · tip slot · usage ·
//! try-help) from the error's own context, and the tip slot carries the curated tip
//! where a curated row exists. **Every other clap error kind keeps clap's own render**,
//! did-you-mean included, and an **uncurated** guess keeps clap's suggestion verbatim.
//! The suppression arm iterates [`CURATED_SIBLING_TIPS`] itself — the table is the axis,
//! never a list re-typed here (the receipt-outlives-contract failure the M48 razor
//! guards against).
//!
//! **M48 Inc 6 T3 — the read-shaped rows** (`DECISIONS.md` 2026-08-13 the Settle, F8
//! part 2): a **read** intent must never land on a **write** verb. Three misses agents
//! reached for are curated — `doc read` / `doc get` → `jigc doc show`, `config show` →
//! `jigc config get` / `jigc config list` — because clap answered them with a write verb
//! or with nothing at all: `doc read` drew `tip: some similar subcommands exist:
//! 'create', 'rename'` (a read intent steered at two writes — law 1), while `doc get` and
//! `config show` were **silent misses**, clap finding no near sibling and the surface
//! saying nothing about the read rung that does exist.
//!
//! No external test crates: the binary path comes from `CARGO_BIN_EXE_jigc`, the temp
//! repo is a real `git init`, and a self-cleaning `TempDir` keeps the test off the
//! dev's repo.

use cli::cli::CURATED_SIBLING_TIPS;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// clap's two did-you-mean openings — the lines jigc's own render drops for a curated
/// row (and keeps verbatim for every uncurated one).
const CLAP_DID_YOU_MEAN: [&str; 2] = [
    "tip: a similar subcommand exists",
    "tip: some similar subcommands exist",
];

/// The read-shaped misses of M48 Inc 6 T3: `(parent, guess, the read span the tip must
/// name)`. Every row is a **read** intent — the question "show me what is there" — so
/// every row's tip must answer with a read verb and only a read verb.
const READ_SHAPED_MISSES: [(&str, &str, &str); 3] = [
    ("doc", "read", "jigc doc show <address>"),
    ("doc", "get", "jigc doc show <address>"),
    ("config", "show", "jigc config get <key>"),
];

/// The `doc` / `config` **write** verbs. None may appear anywhere in a read-shaped
/// miss's emitted stderr — not in the tip, not in a suggestion, not in the usage block.
const WRITE_VERB_TOKENS: [&str; 8] = [
    "create",
    "rename",
    "set",
    "insert-step",
    "replace-step",
    "remove-step",
    "fill",
    "fork",
];

/// Split emitted text into command-ish tokens: runs of `[A-Za-z0-9-]`, so a hyphenated
/// verb (`insert-step`) stays one token and ordinary prose (`creates`, `subset`) cannot
/// masquerade as one. A token counts as naming a write verb when it **is** the verb or
/// carries it as a hyphenated head (`set-field`, `set-slot`).
fn names_a_write_verb(text: &str) -> Option<String> {
    text.split(|c: char| !c.is_ascii_alphanumeric() && c != '-')
        .filter(|token| !token.is_empty())
        .find(|token| {
            WRITE_VERB_TOKENS
                .iter()
                .any(|verb| *token == *verb || token.starts_with(&format!("{verb}-")))
        })
        .map(str::to_owned)
}

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-siblingtip-{tag}-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
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

/// Run a `git` command in `repo`, asserting success.
fn git(repo: &Path, args: &[&str]) {
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
}

/// Initialize a real git repo with one commit + the `.jigc/config/` project layer.
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, capturing output.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

/// Assert the guess stayed a genuine clap usage error — exit 2, clap's own
/// `unrecognized subcommand` + usage output preserved on stderr (never a silent
/// alias: nothing dispatched) — and return the stderr text.
fn assert_usage_error_preserved(out: &std::process::Output, guess: &str) -> String {
    assert_eq!(
        out.status.code(),
        Some(2),
        "an unknown subcommand stays a clap usage error (exit 2); stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let stderr = String::from_utf8(out.stderr.clone()).expect("utf-8 stderr");
    assert!(
        stderr.contains(&format!("error: unrecognized subcommand '{guess}'")),
        "clap's own error line is preserved; got:\n{stderr}",
    );
    assert!(
        stderr.contains("Usage: jigc task"),
        "clap's own usage output is preserved; got:\n{stderr}",
    );
    stderr
}

/// Extract the first backticked `jigc …` span from emitted text and run it verbatim,
/// asserting exit 0 — the emitted bytes are the contract, so the command an agent
/// would copy must actually run.
fn run_first_emitted_span(text: &str, repo: &Path, home: &Path) {
    let (_, tail) = text
        .split_once('`')
        .unwrap_or_else(|| panic!("no backticked span in:\n{text}"));
    let (span, _) = tail
        .split_once('`')
        .unwrap_or_else(|| panic!("unterminated backticked span in:\n{text}"));
    let argv: Vec<&str> = span.split_whitespace().collect();
    assert_eq!(
        argv.first(),
        Some(&"jigc"),
        "the emitted span must lead with the binary name; got `{span}`",
    );
    let run = jigc(repo, home, &argv[1..]);
    assert_eq!(
        run.status.code(),
        Some(0),
        "the emitted span `{span}` must run verbatim; stderr:\n{}",
        String::from_utf8_lossy(&run.stderr),
    );
}

/// `jigc task discard-write <path>` — the ghost-verb guess. clap's did-you-mean points
/// at `discard`; the honest tip must say what `task discard` actually DOES (abandons
/// the whole task), so the guesser is not steered into destroying it.
#[test]
fn task_discard_write_guess_gets_the_whole_task_effect_tip() {
    let repo = TempDir::new("discard-write");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let out = jigc(
        repo.path(),
        home.path(),
        &["task", "discard-write", "docs/adr.md"],
    );
    let stderr = assert_usage_error_preserved(&out, "discard-write");

    let tip_at = stderr
        .find("tip: no per-write discard exists")
        .unwrap_or_else(|| panic!("the honest tip is missing from:\n{stderr}"));
    let tip = &stderr[tip_at..];
    assert!(
        tip.contains("`jigc task discard <task-id>`"),
        "the tip names the real sibling; got:\n{tip}",
    );
    assert!(
        tip.contains("abandons the WHOLE task"),
        "the tip states the sibling's effect, not a bare did-you-mean; got:\n{tip}",
    );
    assert!(
        tip.contains("revert that file on disk"),
        "the tip names the per-write alternative (the on-disk revert); got:\n{tip}",
    );
}

/// `jigc task status` — the tip names what each real sibling does: `task list`
/// enumerates the active tasks, `task validate <id>` previews **part** of the gate.
/// The scoping is M47 Inc 4 T4's (law 1): the preview covers this task's content
/// findings, the carryover gate and the staging-independent `owner-artifact` causes,
/// so the tip says *part of* rather than re-asserting the retired flat promise. The
/// placeholder-free span (`jigc task list`) is extracted from the emitted bytes and
/// run verbatim.
#[test]
fn task_status_guess_gets_the_real_sibling_effects_and_the_span_runs() {
    let repo = TempDir::new("status");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let out = jigc(repo.path(), home.path(), &["task", "status"]);
    let stderr = assert_usage_error_preserved(&out, "status");

    let tip_at = stderr
        .find("tip: ")
        .unwrap_or_else(|| panic!("the honest tip is missing from:\n{stderr}"));
    let tip = &stderr[tip_at..];
    assert!(
        tip.contains("`jigc task list` enumerates the active tasks"),
        "the tip states what `task list` does; got:\n{tip}",
    );
    assert!(
        tip.contains("`jigc task validate <task-id>` previews part of the finalize gate"),
        "the tip states what `task validate` does; got:\n{tip}",
    );
    for covered in ["content findings", "carryover", "owner-artifact"] {
        assert!(
            tip.contains(covered),
            "the tip names `{covered}` as covered by the preview; got:\n{tip}",
        );
    }

    // The emitted `jigc task list` span runs verbatim (exit 0) in a real repo.
    run_first_emitted_span(tip, repo.path(), home.path());
}

/// **A read intent never lands on a write verb** (M48 Inc 6 T3): per row of
/// [`READ_SHAPED_MISSES`], the emitted stderr is a usage error (exit 2) that names the
/// guess, carries a tip naming the **read** verb, and holds **no** write-verb token
/// anywhere — clap's own answers were `'create', 'rename'` for `doc read` and silence
/// for `doc get` / `config show`. The tip's first backticked span is extracted from the
/// emitted bytes and run verbatim, so the read the surface points at is one that works.
#[test]
fn a_read_shaped_miss_routes_to_a_read_verb_and_names_no_write_verb() {
    let repo = TempDir::new("read-shaped");
    let home = TempDir::new("home");
    init_repo(repo.path());

    for (parent, guess, read_span) in READ_SHAPED_MISSES {
        let out = jigc(repo.path(), home.path(), &[parent, guess]);
        assert_eq!(
            out.status.code(),
            Some(2),
            "`jigc {parent} {guess}` stays a usage error (exit 2); stdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        );
        let stderr = String::from_utf8(out.stderr.clone()).expect("utf-8 stderr");
        assert!(
            stderr.contains(&format!("error: unrecognized subcommand '{guess}'")),
            "the error line names the guess; got:\n{stderr}",
        );

        let tip_at = stderr.find("tip: ").unwrap_or_else(|| {
            panic!(
                "`jigc {parent} {guess}` is a read-shaped miss and must carry a tip; got:\n{stderr}"
            )
        });
        let tip = &stderr[tip_at..];
        assert!(
            tip.contains(&format!("`{read_span}`")),
            "the tip names the read verb `{read_span}`; got:\n{tip}",
        );

        if let Some(token) = names_a_write_verb(&stderr) {
            panic!(
                "`jigc {parent} {guess}` is a read intent and must name no write verb, \
                 but its emitted stderr carries `{token}`:\n{stderr}"
            );
        }

        // The emitted span runs verbatim (exit 0) in a real repo — the read the tip
        // points at is the contract, not a description of one.
        run_first_emitted_span(tip, repo.path(), home.path());
    }
}

/// **The table is the axis** (M48 Inc 6 T2): for EVERY row of `CURATED_SIBLING_TIPS`,
/// iterated from the table rather than re-typed here, the emitted block carries the
/// row's own tip and **neither** of clap's did-you-mean openings — jigc renders the
/// block, so the suggestion that contradicts the curated tip is gone rather than
/// printed above it. The rest of clap's block is unchanged (error line, the parent's
/// usage, exit 2).
#[test]
fn every_curated_row_renders_without_claps_contradicting_suggestion() {
    let repo = TempDir::new("curated-render");
    let home = TempDir::new("home");
    init_repo(repo.path());

    assert!(
        !CURATED_SIBLING_TIPS.is_empty(),
        "the curated table must carry rows, or this arm asserts nothing",
    );
    for row in CURATED_SIBLING_TIPS {
        let out = jigc(repo.path(), home.path(), &[row.parent, row.guess]);
        assert_eq!(
            out.status.code(),
            Some(2),
            "`jigc {} {}` stays a usage error (exit 2)",
            row.parent,
            row.guess,
        );
        let stderr = String::from_utf8(out.stderr.clone()).expect("utf-8 stderr");
        assert!(
            stderr.contains(&format!("error: unrecognized subcommand '{}'", row.guess)),
            "the error line names the guess; got:\n{stderr}",
        );
        assert!(
            stderr.contains(&format!("Usage: jigc {}", row.parent)),
            "the parent's usage output is preserved; got:\n{stderr}",
        );
        assert!(
            stderr.contains(&(row.tip)()),
            "the row's own curated tip is emitted verbatim; got:\n{stderr}",
        );
        for lie in CLAP_DID_YOU_MEAN {
            assert!(
                !stderr.contains(lie),
                "`jigc {} {}` must not print clap's `{lie}` beside its curated tip; got:\n{stderr}",
                row.parent,
                row.guess,
            );
        }
    }
}

/// The omitting context for the render takeover: an **uncurated** guess keeps clap's
/// did-you-mean verbatim — the takeover drops the suggestion only where jigc has a
/// curated tip that would contradict it, never globally.
#[test]
fn an_uncurated_guess_keeps_claps_did_you_mean() {
    let repo = TempDir::new("uncurated-suggestion");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let out = jigc(repo.path(), home.path(), &["valdiate"]);
    assert_eq!(
        out.status.code(),
        Some(2),
        "an unknown top-level subcommand stays a usage error (exit 2)",
    );
    let stderr = String::from_utf8(out.stderr.clone()).expect("utf-8 stderr");
    assert!(
        stderr.contains("error: unrecognized subcommand 'valdiate'"),
        "clap's own error line is preserved; got:\n{stderr}",
    );
    assert!(
        stderr.contains("tip: a similar subcommand exists: 'validate'"),
        "an ordinary typo keeps did-you-mean; got:\n{stderr}",
    );
    assert!(
        stderr.contains("Usage: jigc"),
        "the usage output is preserved; got:\n{stderr}",
    );
}

/// The omitting context: an unknown guess **outside** the curated map stays inert —
/// clap's error and exit 2 unchanged, no curated tip text — never an error in the
/// tip machinery itself.
#[test]
fn an_uncurated_guess_stays_a_plain_clap_error_with_no_tip() {
    let repo = TempDir::new("uncurated");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let out = jigc(repo.path(), home.path(), &["task", "frobnicate"]);
    let stderr = assert_usage_error_preserved(&out, "frobnicate");
    assert!(
        !stderr.contains("no per-write discard exists")
            && !stderr.contains("enumerates the active tasks"),
        "an uncurated guess gets no curated tip; got:\n{stderr}",
    );
    // clap itself finds no near sibling for this guess, so the rendered block carries
    // no tip slot at all — jigc invents neither a tip nor an empty one.
    assert!(
        !stderr.contains("tip:"),
        "an uncurated guess clap cannot suggest for gets no tip line; got:\n{stderr}",
    );
}

/// The parent-scoped context: the curated guess under the WRONG parent (`jigc doc
/// status`) stays inert — the map keys on (parent, guess), so a `doc` guess must not
/// receive the `task`-sibling tip.
#[test]
fn a_curated_guess_under_the_wrong_parent_gets_no_tip() {
    let repo = TempDir::new("wrong-parent");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let out = jigc(repo.path(), home.path(), &["doc", "status"]);
    assert_eq!(
        out.status.code(),
        Some(2),
        "an unknown doc subcommand stays a clap usage error (exit 2)",
    );
    let stderr = String::from_utf8(out.stderr.clone()).expect("utf-8 stderr");
    assert!(
        stderr.contains("error: unrecognized subcommand 'status'"),
        "clap's own error line is preserved; got:\n{stderr}",
    );
    assert!(
        !stderr.contains("enumerates the active tasks"),
        "the task-sibling tip must not fire under `doc`; got:\n{stderr}",
    );
}
