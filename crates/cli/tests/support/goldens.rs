//! The **compose-golden harness** — capture, normalize, compare
//! ([pinning.md](../../../../implementation/pinning.md) §1).
//!
//! **Claim the goldens pin:** the composed surface — everything `start` /
//! `workflow --preview` / `describe` / `doc schema` print — changes only when
//! someone *means* it to, and a pack edit's blast radius is a reviewable diff rather
//! than an invisible propagation. Composition is deterministic by core invariant
//! (same resolved cascade in → same workflow out), so the whole surface is
//! snapshottable.
//!
//! **Goldens are for *noticing*, not forbidding.** A red golden means "you changed a
//! printed surface without looking at what else changed": look, then regenerate. A
//! golden is never hand-edited — that is the one way to make it lie.
//!
//! Three decisions this module implements, each load-bearing:
//!
//!   * **A capture is the whole invocation** — stdout, stderr **and** the exit code.
//!     Not stdout alone: the four `creates-task: false` workflows exit **1** with
//!     **empty stdout** and carry their entire refusal on stderr, so a stdout-only
//!     golden would snapshot empty files forever and pin nothing.
//!   * **`<REPO>` and `<jigc-version>` are the only normalizations.** Verified
//!     empirically at rc.8 across two independently created repos: every swept surface
//!     was byte-identical *before* any normalization, and invariant under TZ, locale, git
//!     identity and branch name — with one qualifier, that bare `jigc start` embeds the
//!     absolute project-config path. That is what `<REPO>` is for, and all it is for.
//!     The second (M54) is the running `jigc` version, exactly as `CARGO_PKG_VERSION`
//!     spells it ([`VERSION_TOKEN`]) and **only where it is printed as a pack version** —
//!     the `<pack-id>/<version>` tokens of the `Pack:` header: the release pipeline bumps
//!     it, and a bump must move no golden, while the same string in a pack's prose is
//!     content and stays verbatim. Anything else that varies (a stamped date reaching a golden through a
//!     header-including doc-slice, say) is a **finding** — reproducibility of structure is
//!     the product claim — never something to quietly normalize away.
//!   * **Regen is refused under CI** ([`update_mode`]) — the insta convention, so a
//!     regen can never green CI.
//!
//! The production suite composes the two halves this module exposes:
//!
//! ```ignore
//! GoldenSuite::new(
//!     Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/goldens"),
//!     update_mode(
//!         std::env::var("UPDATE_GOLDENS").ok().as_deref(),
//!         std::env::var("CI").ok().as_deref(),
//!     ),
//! )
//! ```
//!
//! The root is **always passed in**: there is no repo-resolving default, so a suite
//! that means to write into a tempdir cannot reach the real golden tree by omission.

use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Output;

/// The token every absolute repo path normalizes to — the *only* normalization.
pub const REPO_TOKEN: &str = "<REPO>";

/// The token the running `jigc` version normalizes to — the second normalization, and
/// the only other one (M54 Increment 5 / T3; `implementation/release.md` → Versioning,
/// *A bump moves no golden*). **Exactly the string equal to `CARGO_PKG_VERSION`** — this
/// package's, which is `jigc`'s, since every suite is a test target of the `jigc` package
/// — and never a version-shaped pattern: a surface that regresses to `0.1.0`, or to the
/// engine's own `0.1.0-rc.1`, keeps its bytes and diffs. **And only inside a pack-version
/// token** (`<pack-id>/<version>`, see [`normalize_version`]): the same string anywhere
/// else — the dev pack's "e.g. `1.0.0` mints `1-0-0`" at the 1.0.0 release — is content and
/// keeps its bytes. What the token absorbs is only the release pipeline's bump, which
/// changes the printed pack version and nothing else a golden pins.
pub const VERSION_TOKEN: &str = "<jigc-version>";

/// The regen route, quoted verbatim in every failure so a red golden always carries
/// its own remedy (the surface contract's route floor, applied to our own tests).
///
/// A suite is a **module inside its group target** since the M47 test-target
/// consolidation, so `--test <suite>` no longer names anything — the route addresses
/// the suite by module path instead. Golden writes are per-member files and never
/// contend ([pinning.md](../../../../implementation/pinning.md) → Golden layout), so
/// the nextest form — which runs each test in its own process — regenerates safely too.
const REGEN_ROUTE: &str = "UPDATE_GOLDENS=1 cargo test -p jigc <suite>::";

/// One captured invocation, rendered to the exact bytes a golden holds: the exit
/// code, then stdout, then stderr, with absolute repo paths normalized to
/// [`REPO_TOKEN`].
pub struct Capture {
    text: String,
}

impl Capture {
    /// Capture `out`, normalizing paths under `repo`.
    ///
    /// A stream that is non-empty and lacks a trailing newline is marked, so
    /// `"foo"` and `"foo\n"` cannot render to the same golden — a byte golden that
    /// folded that difference away would not be pinning bytes.
    pub fn of(out: &Output, repo: &Path) -> Self {
        let mut text = String::new();
        let code = match out.status.code() {
            Some(code) => code.to_string(),
            // A signal-killed child has no code; it must not render as any exit
            // status a golden could also legitimately hold.
            None => "<signal>".to_string(),
        };
        let _ = writeln!(text, "exit: {code}");
        push_stream(&mut text, "stdout", &String::from_utf8_lossy(&out.stdout));
        push_stream(&mut text, "stderr", &String::from_utf8_lossy(&out.stderr));
        Capture {
            text: normalize(&text, repo),
        }
    }

    /// Capture a `--format json` invocation: [`Capture::of`] plus the **parse gate**
    /// (confidence-audit minor item 5) — stdout must parse as exactly **one** JSON
    /// document, checked at capture time so it holds in check *and* regen mode alike.
    /// A byte golden alone notices drift but forbids nothing: a regen would silently
    /// accept polluted bytes (hook chatter, a stray diagnostic) as the new expected
    /// output, so the purity claim the goldens ride on is asserted before any golden
    /// is read or written.
    pub fn of_json(out: &Output, repo: &Path) -> Self {
        let stdout = String::from_utf8_lossy(&out.stdout);
        if let Err(err) = serde_json::from_str::<serde_json::Value>(&stdout) {
            panic!(
                "a `--format json` capture's stdout must parse as exactly one JSON \
                 document ({err}) — polluted bytes must never become a golden; got:\n{stdout}",
            );
        }
        Capture::of(out, repo)
    }

    /// Capture a **rendered file** (the AGENT.md bootstrap render), normalizing
    /// paths under `repo` — the same single normalization as [`Capture::of`].
    ///
    /// A file render has no exit code and no streams, so the golden is the file's
    /// bytes alone; the invocation framing would be a fiction here, not a capture.
    pub fn of_file(body: &str, repo: &Path) -> Self {
        Capture {
            text: normalize(body, repo),
        }
    }

    /// The rendered, normalized bytes.
    pub fn text(&self) -> &str {
        &self.text
    }
}

fn push_stream(text: &mut String, name: &str, body: &str) {
    let _ = writeln!(text, "--- {name} ---");
    text.push_str(body);
    if !body.is_empty() && !body.ends_with('\n') {
        let _ = writeln!(text, "\n\\ no trailing newline on {name}");
    }
}

/// Replace absolute paths under `repo` with [`REPO_TOKEN`] — both the path as given
/// and its canonical form, since a tempdir root can be reached through a symlink —
/// then the running `jigc` version, inside a pack-version token only, with
/// [`VERSION_TOKEN`] ([`normalize_version`]).
///
/// **Longest form first.** On macOS the canonical form of a `$TMPDIR` root is the
/// *given* one prefixed with `/private`, so replacing the given form first would
/// eat its tail out of the canonical one and leave `/private<REPO>` behind — a
/// machine-dependent golden diff that has nothing to do with the surface.
fn normalize(text: &str, repo: &Path) -> String {
    let given = repo.display().to_string();
    let mut forms = vec![given.clone()];
    if let Ok(canonical) = repo.canonicalize() {
        let canonical = canonical.display().to_string();
        if canonical != given {
            forms.push(canonical);
        }
    }
    forms.sort_by_key(|form| std::cmp::Reverse(form.len()));
    let mut out = text.to_string();
    for form in forms {
        out = out.replace(&form, REPO_TOKEN);
    }
    normalize_version(&out, env!("CARGO_PKG_VERSION"))
}

/// Replace `version` with [`VERSION_TOKEN`] **only inside a pack-version token** —
/// `<pack-id>/<version>`, the segment the `Pack:` header prints per composed pack
/// (`dev/<v> | methodology/<v>`), which is where the running version reaches a surface.
///
/// **Anchored, never a substring replace.** A pack's own prose can name a version —
/// the dev pack's `author-change` step ships "e.g. `1.0.0` mints `1-0-0`" — and a
/// whole-capture replace rewrote that example the day the running version became
/// `1.0.0`, moving twelve goldens on the one bump the rule exists to absorb (M54
/// Increment 5). An occurrence normalizes only when all three hold:
///
///   * it is immediately preceded by `/`;
///   * the `/` is preceded by a non-empty pack id (`[a-z0-9-]+`) that itself starts the
///     text or follows a character that cannot continue a path or identifier — so a
///     path segment `a/dev/<v>` is not a pack token;
///   * it is not followed by a version continuation — an ASCII alphanumeric, or a
///     `.`/`-`/`+` that is itself followed by one — so `dev/<v>.9` or
///     `dev/<v>-rc.1` is a *different* version and keeps its bytes.
fn normalize_version(text: &str, version: &str) -> String {
    let bytes = text.as_bytes();
    let is_id = |b: u8| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-';
    let continues_path =
        |b: u8| b.is_ascii_alphanumeric() || matches!(b, b'_' | b'.' | b'/' | b'-');
    let is_token = |at: usize| {
        let Some(slash) = at.checked_sub(1) else {
            return false;
        };
        if bytes[slash] != b'/' {
            return false;
        }
        let id_start = bytes[..slash]
            .iter()
            .rposition(|&b| !is_id(b))
            .map_or(0, |i| i + 1);
        if id_start == slash || (id_start > 0 && continues_path(bytes[id_start - 1])) {
            return false;
        }
        let end = at + version.len();
        match (bytes.get(end), bytes.get(end + 1)) {
            (Some(b), _) if b.is_ascii_alphanumeric() => false,
            (Some(b'.' | b'-' | b'+'), Some(next)) if next.is_ascii_alphanumeric() => false,
            _ => true,
        }
    };
    let mut out = String::with_capacity(text.len());
    let mut copied = 0;
    for (at, _) in text.match_indices(version) {
        if is_token(at) {
            out.push_str(&text[copied..at]);
            out.push_str(VERSION_TOKEN);
            copied = at + version.len();
        }
    }
    out.push_str(&text[copied..]);
    out
}

/// One golden's identity: `<root>/compose/<pack>/<surface>--<member>--<state>.txt`.
///
/// Per-member files, so diffs are per-surface and regen writes never contend.
pub struct GoldenKey<'a> {
    /// The pack the member comes from (`dev`, `methodology`).
    pub pack: &'a str,
    /// The swept surface (`start`, `workflow-preview`, `describe`, `doc-schema`, …).
    pub surface: &'a str,
    /// The member within that surface — a workflow id, a doctype, or the surface's
    /// own name when it has no members.
    pub member: &'a str,
    /// The fixture state the capture was taken in
    /// (`support::trial_corpus::State::name`).
    pub state: &'a str,
}

/// A golden set rooted at one directory, in either check or regen mode.
pub struct GoldenSuite {
    root: PathBuf,
    update: bool,
}

impl GoldenSuite {
    /// A suite over `root`. `update` regenerates instead of comparing — derive it
    /// from [`update_mode`], never from a bare env read, or the CI refusal is
    /// bypassed.
    pub fn new(root: impl Into<PathBuf>, update: bool) -> Self {
        GoldenSuite {
            root: root.into(),
            update,
        }
    }

    /// Where `key`'s golden lives.
    pub fn path_of(&self, key: &GoldenKey) -> PathBuf {
        self.root.join("compose").join(key.pack).join(format!(
            "{}--{}--{}.txt",
            key.surface, key.member, key.state
        ))
    }

    /// Compare `capture` against `key`'s golden — or write it, in regen mode.
    ///
    /// Panics on a mismatch **and on a missing golden**: a surface with no golden
    /// pins nothing, and treating "no file" as "nothing to compare" would green a
    /// whole sweep while asserting nothing.
    pub fn check(&self, key: &GoldenKey, capture: &Capture) {
        let path = self.path_of(key);
        if self.update {
            let dir = path.parent().expect("a golden path has a parent");
            fs::create_dir_all(dir)
                .unwrap_or_else(|e| panic!("create the golden dir {}: {e}", dir.display()));
            fs::write(&path, capture.text())
                .unwrap_or_else(|e| panic!("write the golden {}: {e}", path.display()));
            return;
        }
        let expected = fs::read_to_string(&path).unwrap_or_else(|e| {
            panic!(
                "no golden at {}: {e}\n\
                 A surface with no golden pins nothing — this is a failure, not a pass.\n\
                 Regenerate with: {REGEN_ROUTE}",
                path.display(),
            )
        });
        if expected != capture.text() {
            panic!("{}", mismatch_report(&path, &expected, capture.text()));
        }
    }
}

/// The failure a stale golden prints: what was read, where the two sides part, and
/// the route back to green.
fn mismatch_report(path: &Path, expected: &str, actual: &str) -> String {
    let expected_lines: Vec<&str> = expected.lines().collect();
    let actual_lines: Vec<&str> = actual.lines().collect();
    let at = (0..expected_lines.len().max(actual_lines.len()))
        .find(|i| expected_lines.get(*i) != actual_lines.get(*i))
        .expect("the two sides differ, so some line differs");

    let mut report = String::new();
    let _ = writeln!(report, "golden mismatch: {}", path.display());
    let _ = writeln!(
        report,
        "first differing line: {} (golden has {} lines, capture has {})",
        at + 1,
        expected_lines.len(),
        actual_lines.len(),
    );
    // `at` is the FIRST differing index, so it is at most `expected_lines.len()`
    // (equal exactly when the golden is a strict prefix of the capture) — the window
    // is in range without a clamp.
    for line in expected_lines[at.saturating_sub(3)..at].iter() {
        let _ = writeln!(report, "  {line}");
    }
    let _ = writeln!(report, "- golden:  {}", show(expected_lines.get(at)));
    let _ = writeln!(report, "+ capture: {}", show(actual_lines.get(at)));
    let _ = writeln!(
        report,
        "\nA red golden means a printed surface moved. Look at the diff, then \
         regenerate — never hand-edit a golden.\nRegenerate with: {REGEN_ROUTE}",
    );
    report
}

fn show(line: Option<&&str>) -> String {
    match line {
        Some(line) => (*line).to_string(),
        None => "<end of output>".to_string(),
    }
}

/// Whether a run regenerates, from the two environment variables that decide it.
///
/// **A regen request under CI panics** rather than quietly checking or quietly
/// regenerating: a regenerated golden asserts nothing, so a CI run that accepted one
/// would report green over an unreviewed surface change. Either variable counts as
/// set when it is present and non-empty.
pub fn update_mode(update: Option<&str>, ci: Option<&str>) -> bool {
    let set = |v: Option<&str>| v.is_some_and(|v| !v.is_empty());
    if set(update) && set(ci) {
        panic!(
            "refusing to regenerate goldens under CI: UPDATE_GOLDENS is set and so is \
             CI. A regenerated golden pins nothing, so a CI run must never write one — \
             regenerate locally ({REGEN_ROUTE}) and commit the diff for review.",
        );
    }
    set(update)
}
