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
//! **M48 Inc 6 T3 — a read intent is never answered with a write verb** (`DECISIONS.md`
//! 2026-08-13 the Settle, F8 part 2). Three misses agents reached for are curated —
//! `doc read` / `doc get` → `jigc doc show`, `config show` → `jigc config get` — because
//! clap answered them with a write verb or with nothing at all: `doc read` drew `tip:
//! some similar subcommands exist: 'create', 'rename'` (a read intent steered at two
//! writes — law 1), while `doc get` and `config show` were **silent misses**.
//!
//! **The claim, though, is universal**, so the curated rows cannot be its acceptance: at
//! the increment's first HEAD `jigc read` still drew `'relocate', 'rename'` and `jigc doc
//! cat` still drew `'create'` — the identical law-1 lie, one uncurated `(parent, guess)`
//! away. So the fence below iterates the **read-intent axis** instead: every parent node
//! the clap tree enumerates (the root included) × every token of
//! [`READ_INTENT_GUESSES`](cli::cli::READ_INTENT_GUESSES) that names no real child there,
//! with every verb reference in the emitted bytes — command spans **and** clap's quoted
//! did-you-mean names alike — classified against
//! [`VERB_KINDS`](cli::cli::VERB_KINDS), the code-side read/write table the rule itself
//! reads. A verb added anywhere joins both sides the day it lands.
//!
//! No external test crates: the binary path comes from `CARGO_BIN_EXE_jigc`, the temp
//! repo is a real `git init`, and a self-cleaning `TempDir` keeps the test off the
//! dev's repo.

use clap::CommandFactory;
use cli::cli::{CURATED_SIBLING_TIPS, Cli, READ_INTENT_GUESSES, VerbKind, verb_kind};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// clap's two did-you-mean openings — the lines jigc's own render drops for a curated
/// row (and keeps verbatim for every uncurated one).
const CLAP_DID_YOU_MEAN: [&str; 2] = [
    "tip: a similar subcommand exists",
    "tip: some similar subcommands exist",
];

/// Every **parent** node's argv path in the clap tree, the root (`vec![]`) first — the
/// enumeration seam the read-intent axis is built over, so a new verb group joins the
/// sweep the day it lands. `help` is clap's builtin, not a jigc verb.
fn parent_paths() -> Vec<Vec<String>> {
    fn walk(cmd: &clap::Command, prefix: Vec<String>, out: &mut Vec<Vec<String>>) {
        let subs: Vec<&clap::Command> = cmd
            .get_subcommands()
            .filter(|sub| sub.get_name() != "help")
            .collect();
        if subs.is_empty() {
            return;
        }
        out.push(prefix.clone());
        for sub in subs {
            let mut child = prefix.clone();
            child.push(sub.get_name().to_string());
            walk(sub, child, out);
        }
    }
    let mut out = Vec::new();
    walk(&Cli::command(), Vec::new(), &mut out);
    out
}

/// The direct child names of the node at `path`.
fn children_of(path: &[String]) -> Vec<String> {
    let mut cmd = Cli::command();
    for token in path {
        cmd = cmd
            .find_subcommand(token)
            .unwrap_or_else(|| panic!("`{token}` is a node of the clap tree"))
            .clone();
    }
    cmd.get_subcommands()
        .map(|sub| sub.get_name().to_string())
        .collect()
}

/// **The read-intent axis**: every `(parent node, read-shaped guess)` pair the surface
/// can be asked — every parent the clap tree enumerates × every token of
/// `READ_INTENT_GUESSES` that names no real child there (a real child dispatches and is
/// no miss at all).
fn read_intent_misses() -> Vec<(Vec<String>, String)> {
    let mut out = Vec::new();
    for parent in parent_paths() {
        let children = children_of(&parent);
        for guess in READ_INTENT_GUESSES {
            if children.iter().any(|child| child == guess) {
                continue;
            }
            out.push((parent.clone(), (*guess).to_string()));
        }
    }
    out
}

/// Trim a token of the punctuation emitted text wraps command names in — backticks,
/// clap's single quotes, and trailing sentence marks.
fn unwrap_token(token: &str) -> &str {
    token.trim_matches(|c: char| !c.is_ascii_alphanumeric() && c != '-')
}

/// Every **verb reference** the emitted `text` makes, as `(argv path, kind)` — the two
/// shapes a block can name a verb in:
///
/// * a command **span** (`` `jigc doc show <address>` ``): the tokens after `jigc` are
///   walked down the clap tree for as far as they match, and the walked path is
///   classified when it lands on a leaf;
/// * a **quoted name** (clap's did-you-mean `'create'`, and the error line's own echo of
///   the guess): resolved as a child of `parent`, and classified when that is a leaf.
///
/// Anything that resolves to no leaf verb — the usage line's `jigc doc [OPTIONS]`, the
/// guess itself — is not a verb reference and is skipped.
fn verb_references(text: &str, parent: &[String]) -> Vec<(Vec<String>, VerbKind)> {
    let tokens: Vec<&str> = text.split_whitespace().collect();
    let mut out = Vec::new();
    for (at, token) in tokens.iter().enumerate() {
        // A quoted name, resolved under the parent the guess was typed at.
        if token.contains('\'') {
            let mut path = parent.to_vec();
            path.push(unwrap_token(token).to_string());
            if let Some(kind) = verb_kind(&path) {
                out.push((path, kind));
            }
        }
        // A command span leading with the binary name.
        if unwrap_token(token) != "jigc" {
            continue;
        }
        let mut cmd = Cli::command();
        let mut path: Vec<String> = Vec::new();
        for next in &tokens[at + 1..] {
            let name = unwrap_token(next);
            match cmd.find_subcommand(name) {
                Some(sub) => {
                    path.push(name.to_string());
                    cmd = sub.clone();
                }
                None => break,
            }
        }
        if let Some(kind) = verb_kind(&path) {
            out.push((path, kind));
        }
    }
    out
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

/// Run the first **placeholder-free** backticked `jigc …` span in `text` verbatim,
/// asserting exit 0. A tip whose spans all carry placeholders (`<milestone-id>`) has
/// nothing runnable as emitted and is left to the construction-time route fence.
fn run_first_placeholder_free_span(text: &str, repo: &Path, home: &Path) {
    for span in text.split('`').skip(1).step_by(2) {
        if span.contains('<') || !span.starts_with("jigc ") {
            continue;
        }
        let argv: Vec<&str> = span.split_whitespace().skip(1).collect();
        let run = jigc(repo, home, &argv);
        assert_eq!(
            run.status.code(),
            Some(0),
            "the emitted span `{span}` must run verbatim; stderr:\n{}",
            String::from_utf8_lossy(&run.stderr),
        );
        return;
    }
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
        tip.contains("`jigc task discard <task-id> --force`"),
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

/// **A read intent never lands on a write verb — over the whole axis** (M48 Inc 6 T3,
/// widened on the increment's validation). For every `(parent, read-shaped guess)` pair
/// [`read_intent_misses`] enumerates from the clap tree — `jigc read` and `jigc doc cat`
/// among them, the two the curated `(parent, guess)` rows left answered with
/// `'relocate', 'rename'` and `'create'` — the emitted stderr is a usage error (exit 2)
/// naming the guess, and **every verb reference in it classifies
/// [`VerbKind::Read`]**: no command span, no quoted did-you-mean name, resolves to a
/// verb that acts. At least one read verb is named, so the miss routes somewhere rather
/// than merely withholding the lie.
///
/// Once per parent the tip's first backticked span is extracted from the emitted bytes
/// and run verbatim, so the read the surface points at is one that works.
#[test]
fn no_read_intent_is_answered_with_a_write_verb() {
    let repo = TempDir::new("read-shaped");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let misses = read_intent_misses();
    assert!(
        misses.len() >= 3 * READ_INTENT_GUESSES.len(),
        "the axis must span every parent node of the clap tree; got {} pairs",
        misses.len(),
    );
    let mut span_run_for: Vec<Vec<String>> = Vec::new();
    for (parent, guess) in misses {
        let mut argv: Vec<&str> = parent.iter().map(String::as_str).collect();
        argv.push(&guess);
        let typed = format!("jigc {}", argv.join(" "));
        let out = jigc(repo.path(), home.path(), &argv);
        assert_eq!(
            out.status.code(),
            Some(2),
            "`{typed}` stays a usage error (exit 2); stdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        );
        let stderr = String::from_utf8(out.stderr.clone()).expect("utf-8 stderr");
        assert!(
            stderr.contains(&format!("error: unrecognized subcommand '{guess}'")),
            "the error line names the guess; got:\n{stderr}",
        );

        let named = verb_references(&stderr, &parent);
        if let Some((path, _)) = named.iter().find(|(_, kind)| *kind == VerbKind::Write) {
            panic!(
                "`{typed}` is a read intent and must name no write verb, but its emitted \
                 stderr names `jigc {}`:\n{stderr}",
                path.join(" "),
            );
        }
        assert!(
            named.iter().any(|(_, kind)| *kind == VerbKind::Read),
            "`{typed}` must route to a read verb, not merely withhold the wrong one; \
             got:\n{stderr}",
        );

        if !span_run_for.contains(&parent) {
            span_run_for.push(parent.clone());
            let tip_at = stderr.find("tip: ").unwrap_or_else(|| {
                panic!("`{typed}` names a read verb inside a tip; got:\n{stderr}")
            });
            // The emitted span runs verbatim (exit 0) in a real repo — the read the tip
            // points at is the contract, not a description of one. A placeholder-carrying
            // span cannot be run as emitted; those are parse-fenced at construction by
            // the T2 route fence, live in this debug build.
            run_first_placeholder_free_span(&stderr[tip_at..], repo.path(), home.path());
        }
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

/// The parent-scoped context: the curated `task status` row does not fire under the
/// WRONG parent (`jigc doc status`) — the map keys on (parent, guess). What that miss
/// gets instead is the **`doc` parent's** read answer, because `status` is a read intent
/// wherever it is typed; the one thing it must never get is another parent's siblings.
#[test]
fn a_curated_guess_under_the_wrong_parent_gets_its_own_parents_answer() {
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
    assert!(
        stderr.contains("`jigc doc show <address>`"),
        "the read-shaped miss still earns its own parent's read answer; got:\n{stderr}",
    );
}
