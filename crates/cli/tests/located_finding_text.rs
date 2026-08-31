//! M49 Increment 8 / T4 — **a located finding says where, on every surface that renders one
//! as text** (`design/validation.md` → the route exemption; `design/surface-contract.md` →
//! the surface style guide).
//!
//! The `conformance.*` route exemption rests on one sentence: *"the located message **is**
//! the repair"* — fix the named line, no CLI verb repairs a hand-broken byte. That sentence
//! was **false on the agent-text surface**: the JSON envelope carried
//! `location: {address: "changelog:changelog#…/bogusfield", line: 15}` while the text an
//! agent reads carried neither, because `render::finding_line` never looked at
//! `finding.location`. An exemption whose rationale is a fact the surface withholds is a
//! law-1 lie about the exemption, not merely a missing convenience.
//!
//! Three arms, and the third is the one that makes the fix a *class* fix:
//!
//! 1. the repro through the real binary — the agent text names the finding's **own**
//!    `location.address` and line, cross-read from the same run's `--format json` so the
//!    assertion cannot pass over a reconstruction;
//! 2. a **location-less** finding (`setup.repo-root`, `location: null` by construction)
//!    renders exactly as it did at HEAD — the new line appears where there is a locus and
//!    nowhere else;
//! 3. the **source-derived** sweep: every production read of a `Finding`'s `message` in
//!    `crates/cli/src` — the act that turns a finding into text — carries a verdict and a
//!    reason, and a `Carries` verdict is checked against the source rather than believed.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::support::rust_source;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-located-finding-{tag}-{}-{:?}",
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

/// Run `jigc <args>` against the embedded packs.
fn jigc(cwd: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(cwd)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("run the jigc binary")
}

/// Run `jigc <args>`, asserting exit 0, returning stdout.
fn ok(cwd: &Path, home: &Path, args: &[&str], what: &str) -> String {
    let out = jigc(cwd, home, args);
    assert!(
        out.status.success(),
        "`{what}` must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout).expect("utf-8")
}

/// A real git repo with one commit, then `jigc setup`.
fn setup_repo(repo: &Path, home: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write README");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    ok(repo, home, &["setup"], "jigc setup");
}

// ---------------------------------------------------------------------------------
// Arm 1 — the repro: the agent text names the finding's own address and line
// ---------------------------------------------------------------------------------

/// A committed `CHANGELOG.md` carrying a hand-written undeclared field key — the shape a
/// human produces editing the file in an editor, and the shape the exemption's *"fix the
/// named line"* is written for.
const BROKEN_CHANGELOG: &str = "\
---
schema-version: 2
---

# Changelog

## Releases

### 1.0.0  {#1-0-0}

<!-- fields -->
- date: 2026-01-01
- bogusfield: nope

Initial release.
";

/// The store sweep's agent text names the address **and** the line the JSON carries for the
/// same finding — read from the same corpus, never rebuilt in the test.
#[test]
fn a_located_finding_names_its_own_address_and_line_in_agent_text() {
    let repo = TempDir::new("located");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    fs::write(repo.path().join("CHANGELOG.md"), BROKEN_CHANGELOG).expect("write CHANGELOG.md");
    git(repo.path(), &["add", "CHANGELOG.md"]);
    git(repo.path(), &["commit", "-q", "-m", "changelog"]);

    let json_out = ok(
        repo.path(),
        home.path(),
        &["validate", "--format", "json"],
        "jigc validate --format json",
    );
    let envelope: serde_json::Value = serde_json::from_str(&json_out).expect("a JSON envelope");
    let finding = envelope["findings"]
        .as_array()
        .expect("a findings array")
        .iter()
        .find(|f| {
            f["message"]
                .as_str()
                .is_some_and(|m| m.contains("bogusfield"))
        })
        .cloned()
        .unwrap_or_else(|| panic!("the hand-broken field must be reported; got:\n{json_out}"));
    let address = finding["location"]["address"]
        .as_str()
        .expect("the finding carries an addressed location")
        .to_owned();
    let line = finding["location"]["line"]
        .as_u64()
        .expect("the finding carries a source line");

    let agent = ok(repo.path(), home.path(), &["validate"], "jigc validate");
    assert!(
        agent.contains(&address),
        "the agent text must name the finding's own `location.address` — the exemption in \
         `design/validation.md` grants `conformance.*` a route-less pass because *the located \
         message is the repair*, which is false while the located half reaches only the JSON \
         envelope.\nexpected address: {address}\nagent text:\n{agent}",
    );
    let located = agent
        .lines()
        .find(|l| l.contains(&address))
        .expect("the line naming the address");
    assert!(
        located.contains(&format!("line {line}")),
        "the address and the line are one locus and must render together — a reader told \
         *which* leaf but not *where* still has to search the file.\nexpected line: {line}\n\
         got: {located}",
    );
}

// ---------------------------------------------------------------------------------
// Arm 2 — a location-less finding renders exactly as at HEAD
// ---------------------------------------------------------------------------------

/// `jigc setup` outside a git repository blocks with `setup.repo-root` — a **declared
/// singleton** minted through `Finding::block`, so `location: null` by construction. Its
/// agent text is the message line plus its route line, and nothing else: the locus line
/// appears where there is a locus and nowhere else.
#[test]
fn a_location_less_finding_renders_exactly_as_before() {
    let outside = TempDir::new("outside");
    let home = TempDir::new("home");

    // A blocking `setup` prints its finding envelope on **stderr** (`cli.rs` → the setup
    // dispatch), the stream discipline every reject surface follows.
    let json_out = jigc(outside.path(), home.path(), &["setup", "--format", "json"]);
    let finding: serde_json::Value =
        serde_json::from_slice(&json_out.stderr).expect("a finding envelope");
    assert!(
        finding["location"].is_null(),
        "the arm needs a finding with NO location; got:\n{finding:#?}",
    );

    let text = jigc(outside.path(), home.path(), &["setup"]);
    let agent = String::from_utf8(text.stderr).expect("utf-8");
    let expected = format!(
        "blocking · {} — {}\n  route: {}\n",
        finding["code"].as_str().expect("a code"),
        finding["message"].as_str().expect("a message"),
        finding["route"].as_str().expect("a route"),
    );
    assert!(
        agent.starts_with(&expected),
        "a location-less finding must render exactly as it did before — no empty locus line, \
         no placeholder coordinate.\nexpected prefix:\n{expected}\ngot:\n{agent}",
    );
    assert!(
        !agent.contains("  at: "),
        "no locus line may be printed for a finding that carries none;\ngot:\n{agent}",
    );
}

// ---------------------------------------------------------------------------------
// Arm 3 — the source-derived sweep of every text render of a `Finding`
// ---------------------------------------------------------------------------------

/// What a site does with the `Finding` message it reads.
#[derive(Clone, Copy, PartialEq)]
enum Verdict {
    /// Renders the finding as text **and** reads its location to say where — checked
    /// against the source: the enclosing function must reach the shared locus renderer.
    Carries,
    /// Reads a message for something other than rendering the finding to a reader. The
    /// reason states what, and is the site's disposition.
    NotARender,
}

/// **Every production read of a `Finding`'s `message` in `crates/cli/src`** — the act that
/// turns a finding into text — with a verdict and a reason per member, the
/// `is_declared_singleton` / `ROUTE_PLACEHOLDERS` shape.
///
/// Keyed on `(file, enclosing fn)`: a new render site lands in no row and reddens here,
/// naming itself, rather than shipping a finding whose locus reaches only the JSON.
const MESSAGE_SITES: &[(&str, &str, Verdict, &str)] = &[
    (
        "config.rs",
        "finding_to_err",
        Verdict::Carries,
        "the cascade's operational-error funnel",
    ),
    (
        "describe.rs",
        "finding_to_err",
        Verdict::Carries,
        "the introspection funnel",
    ),
    (
        "doc.rs",
        "finding_to_err",
        Verdict::Carries,
        "the write-verb funnel — every `write.*` reject reaches an agent through it",
    ),
    (
        "ingest.rs",
        "adopt_annotations",
        Verdict::NotARender,
        "reads the engine's pinned message shape for its leading count token and emits an \
         adoption annotation; the finding itself is never printed here",
    ),
    (
        "ingest.rs",
        "near_miss_finding",
        Verdict::NotARender,
        "re-derives a Finding from the parse sweep's own — message AND location carried \
         through — which the ingest row then renders through `finding_line`",
    ),
    (
        "migrate.rs",
        "ensure_migratable",
        Verdict::NotARender,
        "widens the shared unknown-doctype block's message with `migrate`'s narrower usable \
         set before handing the finding to `render::finding_error`, which renders it — \
         nothing is printed here",
    ),
    (
        "migrate_corpus.rs",
        "relayed",
        Verdict::Carries,
        "relays the gate's/parser's own words into the refusal message — the diagnostics \
         that say WHERE the buffer broke, so the locus rides with each",
    ),
    (
        "milestone.rs",
        "blocked",
        Verdict::Carries,
        "the blocked milestone finalize's stderr render",
    ),
    (
        "milestone.rs",
        "dispatch_join",
        Verdict::Carries,
        "the blocked join's stderr render",
    ),
    (
        "milestone.rs",
        "finding_to_err",
        Verdict::Carries,
        "the milestone operational-error funnel",
    ),
    (
        "pack.rs",
        "def_load_failure",
        Verdict::Carries,
        "the one funnel all five pack-load fences raise a def-parse failure through",
    ),
    (
        "render.rs",
        "corpus_migration",
        Verdict::Carries,
        "the migration report's blocked / unadopted / unfilled rows — the row head IS the \
         locus, rendered from the finding's own location",
    ),
    (
        "render.rs",
        "finding_error",
        Verdict::Carries,
        "the shared finding→`anyhow` flattening the four operational-funnel doors \
         (`migrate` / `relocate` / `rename` / `task bind`) refuse through",
    ),
    (
        "render.rs",
        "finding_line",
        Verdict::Carries,
        "the house agent-text finding line every findings surface renders through",
    ),
    (
        "start.rs",
        "finding_to_err",
        Verdict::Carries,
        "the front door's operational-error funnel",
    ),
    (
        "task.rs",
        "finding_to_err",
        Verdict::Carries,
        "the task funnel",
    ),
    (
        "task.rs",
        "try_execute_finalize_plan",
        Verdict::NotARender,
        "`plan.message` is the rendered commit message written to git's `-F` file, not a \
         Finding's",
    ),
];

/// The token every `Carries` site must reach — the one renderer of a finding's locus.
const LOCUS_RENDERER: &str = "finding_locus";

/// The `crates/cli` production tree.
fn cli_src() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

/// The source span of the function whose body encloses `at` — the last `fn <name>` before
/// it, brace-matched from its opening `{`, so a `Carries` verdict is checked against what
/// the function actually does rather than against the file it lives in.
fn enclosing_fn_body(code: &str, at: usize) -> &str {
    let start = code[..at].rfind("fn ").expect("an enclosing fn");
    let open = start + code[start..].find('{').expect("a function body");
    let bytes = code.as_bytes();
    let mut depth = 0usize;
    for (i, b) in bytes.iter().enumerate().skip(open) {
        match b {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return &code[open..=i];
                }
            }
            _ => {}
        }
    }
    &code[open..]
}

/// Every production site that reads a `Finding`'s message is disposed — it either renders
/// the locus with it, or says at the site what it is doing instead.
///
/// The sweep is **derived from the source**, not from the task that shipped it: the fix's
/// axis is *every text render of a finding*, and a list of the seven sites the plan named
/// would have missed five (`pack.rs`'s fence family, the two milestone stderr renders and
/// the migration report's three rows).
#[test]
fn every_text_render_of_a_finding_is_disposed() {
    let mut seen: Vec<(String, String)> = Vec::new();
    let mut offenders: Vec<String> = Vec::new();

    for path in rust_source::rust_files(&cli_src()) {
        let body = fs::read_to_string(&path).expect("read a cli source");
        let code = rust_source::code_only(&body);
        let regions = rust_source::cfg_test_regions(&code);
        let file = path
            .file_name()
            .expect("a source file has a name")
            .to_string_lossy()
            .into_owned();

        for (at, _) in code.match_indices(".message") {
            if rust_source::is_test_domain(&path, &regions, at) {
                continue;
            }
            let owner = rust_source::enclosing_fn(&code, at).unwrap_or("<top level>");
            let line = code[..at].lines().count();
            let key = (file.clone(), owner.to_owned());
            if !seen.contains(&key) {
                seen.push(key);
            }
            let Some((_, _, verdict, _)) = MESSAGE_SITES
                .iter()
                .find(|(f, fun, _, _)| *f == file && *fun == owner)
            else {
                offenders.push(format!(
                    "  {file}:{line}: `{owner}` reads a finding message and carries no verdict"
                ));
                continue;
            };
            if *verdict == Verdict::Carries
                && !enclosing_fn_body(&code, at).contains(LOCUS_RENDERER)
            {
                offenders.push(format!(
                    "  {file}:{line}: `{owner}` is declared `Carries` but never reaches \
                     `{LOCUS_RENDERER}`"
                ));
            }
        }
    }

    assert!(
        offenders.is_empty(),
        "every production read of a `Finding`'s message owes a verdict, and a `Carries` \
         verdict owes the source to back it — a finding whose locus reaches only the JSON \
         envelope leaves the `conformance.*` route exemption resting on a fact the text \
         surface withholds (`design/validation.md`).\n{}",
        offenders.join("\n"),
    );

    let mut stale: Vec<String> = MESSAGE_SITES
        .iter()
        .filter(|(f, fun, _, _)| !seen.iter().any(|(file, owner)| file == f && owner == fun))
        .map(|(f, fun, _, _)| format!("  {f}: `{fun}`"))
        .collect();
    stale.sort();
    assert!(
        stale.is_empty(),
        "a row naming a site that no longer exists is a claim about nothing — drop it with \
         the code it described:\n{}",
        stale.join("\n"),
    );
}
