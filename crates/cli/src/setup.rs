//! `jigc setup` — the adapter install handler.
//!
//! Orchestrates the MVP adapter install (`design/assistant-adapter.md` →
//! Generated, minimal, regenerated; `DECISIONS.md` 2026-05-31 → adapter install
//! reworked) against the located repo root, using the embedded Claude Code
//! profile:
//!   1. write the canonical bootstrap sentence to the managed `.jigc/AGENT.md`
//!      and inject a bare `@.jigc/AGENT.md` import into `CLAUDE.md` (the
//!      reference floor — no marker-fenced block);
//!   2. initialize the project cascade layer (`.jigc/config/.gitkeep` +
//!      `.jigc/.gitignore`), so the project resolves as *set up*;
//!   3. merge the `Bash(jigc:*)` **allowlist** into `.claude/settings.json` (the
//!      path-of-least-resistance the bootstrap depends on);
//!   4. install the `SessionStart` **hook** running `jigc start` into the same
//!      settings file (the primary bootstrap injection — advertise+demonstrate at
//!      session start);
//!   5. install the assistant-neutral warn-only git `pre-commit` **hook** (the
//!      auto-firing doc↔code drift backstop, pinned to this `jigc`'s own absolute
//!      path) into the repo's real hooks dir (`design/assistant-adapter.md` →
//!      neutral install; regenerated each `setup`).
//!
//! `jigc setup` is the install the unset-project orientation routes the agent to
//! (`crate::orient` → `OrientationView::unset_project`; `design/bootstrap.md` →
//! Orientation output examples). The `Resume` hook + the fan-out spawn binding are
//! post-MVP and out of scope here.
//!
//! Outcome is reported through the settled **finding** envelope (`DECISIONS.md`
//! 2026-05-31 → block-payload = a blocking-severity finding carrying a route):
//! success yields a plain summary; a write failure yields a single blocking
//! `setup.*` finding whose `route` directs the human's next action, and the
//! dispatcher maps it to a non-zero exit.

use crate::adapter::{self, AdapterProfile};
use crate::locate;
use engine::finding::{Finding, Severity};
use std::path::{Path, PathBuf};

/// The committed **binary-provenance stamp** file, repo-relative (`design/storage.md` →
/// Store provenance): a one-line record of which `jigc` build wrote/refreshed this store,
/// so an adopter on a divergent binary is told via a `store-version.binary-mismatch`
/// advisory (never a gate). It is **committed** — the cross-machine claim requires the
/// record travel with the repo — and lives under `.jigc/`, deliberately NOT the gitignored
/// `.jigc/state/` (which never reaches a clone) and NOT a `packs.yaml` field (a closed
/// surface that rejects unknown keys).
pub const VERSION_STAMP_PATH: &str = ".jigc/version";

/// The stamp line's `key:` prefix — the one-line on-disk format is `jigc-version: <semver>`.
const VERSION_STAMP_KEY: &str = "jigc-version:";

/// The stamp body for the running build: `CARGO_PKG_VERSION` at write time
/// (`design/storage.md` → the value is the engine's `CARGO_PKG_VERSION`; the whole
/// workspace shares one version via `version.workspace = true`).
fn version_stamp_body() -> String {
    format!("{VERSION_STAMP_KEY} {}\n", env!("CARGO_PKG_VERSION"))
}

// ──────────────────── the adapter's owned guide artifact (M48 Inc 10) ────────────────────

/// The guide artifact's second stamp line: the `blake3` of the artifact's **own body**
/// (everything after the front matter), so the file records what jigc wrote and a later
/// read can tell jigc's own bytes from a hand-edited copy without keeping a side record.
/// Reserved against the profile ([`adapter::GUIDE_RESERVED_KEYS`]).
const GUIDE_HASH_KEY: &str = "jigc-body-blake3:";

/// The shipped guides, embedded at compile time from the **repo's own** copies — the
/// single home for this content (the `include_dir!` of `packs/methodology/` from the
/// workspace root is the same move). A second authored copy in `crates/cli/` would drift
/// behind them the first time either is edited.
const QUICKSTART_GUIDE: &str = include_str!("../../../QUICKSTART.md");
const MIGRATING_GUIDE: &str = include_str!("../../../MIGRATING.md");

/// The generated paragraph the artifact opens with — the ownership statement (this file is
/// jigc's, refreshed by `setup`), the version it was written from, and the two facts an
/// adopter needs to read the rest honestly: the guides ship concatenated, and their
/// cross-references to jigc's *project* docs are named without links because those files
/// live in the jigc repository, not in the reader's.
fn guide_preamble() -> String {
    format!(
        "`jigc setup` wrote this file from jigc {} and owns it: re-run `jigc setup` after \
         upgrading the binary to refresh it.\n\nIt carries the two guides that ship with that \
         binary, one after the other — the quickstart loop, then the migration field notes. A \
         cross-reference to `QUICKSTART.md` or `MIGRATING.md` means the matching part of this \
         file; every other jigc document named below lives in the jigc project's own \
         repository, not in this one, which is why none of them are links here.\n",
        env!("CARGO_PKG_VERSION"),
    )
}

/// The artifact's **body**: the preamble followed by both shipped guides, every in-repo
/// relative link resolved away ([`unlink_in_repo_links`]).
fn guide_body() -> String {
    format!(
        "{}\n{}\n{}",
        guide_preamble(),
        unlink_in_repo_links(QUICKSTART_GUIDE).trim_end(),
        unlink_in_repo_links(MIGRATING_GUIDE).trim_end(),
    )
}

/// Rewrite `[label](target)` to a bare `label` for every **in-repo relative** target,
/// leaving absolute URLs and in-document anchors as links.
///
/// The guides' relative links point at files of the *jigc* repository (`design/storage.md`,
/// `implementation/roadmap.md`, the archived migration method); from an adopter's tree every
/// one of them dangles, and a guide that strands its reader on a followed link is a law-1
/// defect authored in the same motion that fixes one. The label is kept — it already names
/// the document — so nothing the sentence needs is lost.
///
/// Deliberately lexical and conservative: an unterminated or multi-line construct is left
/// exactly as written rather than guessed at.
fn unlink_in_repo_links(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(open) = rest.find('[') {
        out.push_str(&rest[..open]);
        let after = &rest[open + 1..];
        // Any shape below that is not a plain inline link emits the bracket and resumes
        // scanning right after it, so no byte of the source is ever dropped.
        rest = after;
        let Some(close) = after.find("](") else {
            out.push('[');
            continue;
        };
        let label = &after[..close];
        let tail = &after[close + 2..];
        let Some(end) = tail.find(')') else {
            out.push('[');
            continue;
        };
        let target = &tail[..end];
        if label.contains('[') || label.contains('\n') || target.contains('\n') {
            out.push('[');
            continue;
        }
        if target.contains("://") || target.starts_with('#') {
            out.push('[');
            out.push_str(label);
            out.push_str("](");
            out.push_str(target);
            out.push(')');
        } else {
            out.push_str(label);
        }
        rest = &tail[end + 1..];
    }
    out.push_str(rest);
    out
}

/// The full artifact for `guide`: the profile's own front-matter keys, then jigc's two
/// stamp lines, then the body — the front matter closed by `---` and one blank line.
///
/// The stamp is computed over the body **as assembled**, never over a pre-image, so the
/// recorded hash is always this file's own. Front-matter values are emitted as JSON
/// strings, which are valid single-line YAML scalars — deterministic bytes for any value a
/// profile can declare, with no emitter line-wrapping to reason about.
pub fn guide_artifact(guide: &adapter::GuideTarget) -> String {
    let body = guide_body();
    let hash = engine::file_state::hash_bytes(body.as_bytes());
    let mut front = String::new();
    for (key, value) in &guide.front_matter {
        let value = serde_json::to_string(value).unwrap_or_else(|_| format!("{value:?}"));
        front.push_str(&format!("{key}: {value}\n"));
    }
    format!(
        "---\n{front}{VERSION_STAMP_KEY} {}\n{GUIDE_HASH_KEY} {hash}\n---\n\n{body}",
        env!("CARGO_PKG_VERSION"),
    )
}

/// Whether the file sitting at the guide target is **still jigc's own** — the question that
/// makes "replace what jigc wrote, never what the user wrote" decidable from the file alone
/// (`design/assistant-adapter.md` → The adapter's owned artifacts).
///
/// The artifact keeps no side record: it carries the digest of its own body, so a later run
/// can recompute that digest and compare. Anything that does not match — a body edited under
/// jigc's header, a file with no front matter at all, a front matter without jigc's stamp —
/// is **not** jigc's, and the one safe action over it is to leave it alone and say so.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GuideOwnership {
    /// Nothing at the declared path — the ordinary first install.
    Absent,
    /// jigc's own bytes: the recorded `jigc-body-blake3:` **is** this body's digest, so
    /// rewriting the file destroys nothing a human authored.
    Owned,
    /// Not jigc's (any more). Replacing it would clobber the user's own edits. This is the
    /// **fail-closed** verdict, so it also covers a file present at the path that cannot be
    /// read back as jigc's text at all — bytes that are not UTF-8, or a read that errors for
    /// any reason other than the file being absent.
    UserModified,
}

/// The [`GuideOwnership`] of the artifact at `<repo_root>/<guide.file>`.
///
/// A read-only probe: it opens the file and computes a hash, and writes nothing — which is
/// what lets the read-only `jigc upgrade` door consult it (`design/overrides.md` → The
/// `jigc upgrade` command: report-and-route only).
pub fn guide_ownership(repo_root: &Path, guide: &adapter::GuideTarget) -> GuideOwnership {
    // **Only a genuine `NotFound` is `Absent`** — every other outcome is the user's.
    // `Absent` is one of the two verdicts the write gate treats as "overwrite it", so
    // folding a *present but unreadable* file into it fails **open**: an ordinary readable
    // file whose bytes are not UTF-8 (a hand-written skill saved Latin-1) would be read as
    // absent and silently clobbered at exit 0. The check is the inverse of the generator's
    // assembly and fails **closed**: anything it cannot prove is jigc's own is the user's.
    let bytes = match std::fs::read(repo_root.join(&guide.file)) {
        Ok(bytes) => bytes,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return GuideOwnership::Absent,
        // Present but unreadable (permissions, a directory at the path, an I/O fault): jigc
        // cannot prove these bytes are its own, so it does not get to replace them.
        Err(_) => return GuideOwnership::UserModified,
    };
    let Ok(text) = String::from_utf8(bytes) else {
        // jigc only ever writes UTF-8, so bytes that do not decode were not written by jigc.
        return GuideOwnership::UserModified;
    };
    match recorded_body_digest(&text) {
        Some((recorded, body)) if recorded == engine::file_state::hash_bytes(body.as_bytes()) => {
            GuideOwnership::Owned
        }
        _ => GuideOwnership::UserModified,
    }
}

/// An artifact's recorded body digest and the body it claims to describe, or `None` when the
/// file carries no jigc-shaped front matter at all — the inverse of [`guide_artifact`]'s
/// assembly, kept lexical so a hand-mangled header reads as *not jigc's* rather than as an
/// error.
fn recorded_body_digest(text: &str) -> Option<(&str, &str)> {
    let rest = text.strip_prefix("---\n")?;
    let close = rest.find("\n---\n\n")?;
    let front = &rest[..close + 1];
    let body = &rest[close + "\n---\n\n".len()..];
    let recorded = front
        .lines()
        .find_map(|line| line.trim().strip_prefix(GUIDE_HASH_KEY))?
        .trim();
    Some((recorded, body))
}

/// The finding code both doors raise over a user-modified artifact. **Un-keyed** — not a
/// `CHECK_INVENTORY` row — so the engine's severity post-pass leaves it advisory and it
/// gates nothing, the shipped `store-version.binary-mismatch` mold.
pub const GUIDE_MODIFIED_CODE: &str = "adapter-guide.user-modified";

/// The **advisory** a user-modified guide artifact raises, at `jigc setup` and at
/// `jigc upgrade` alike (`completions/artifacts/M48/settle-record.md` → the check-scope pin:
/// advisory + route, never blocking).
///
/// Advisory by decision, not by omission: blocking an install because one guide file was
/// edited would be hostile, and an install that stops there is worse than one that leaves
/// the file alone and names it. The route is the other half — a detector with no way back is
/// the dead end the route floor exists to forbid — and it states **both** admissible
/// answers, because keeping the edited copy is a legitimate choice, not a defect to repair.
///
/// **Declared deviation from principle #5** (*every customization is a recorded delta
/// against a known base version, never an untracked fork*): this is an untracked-fork
/// *detector* with no delta. Recorded with its own trigger — a **second** adapter-owned
/// artifact — in `implementation/decisions-pending.md` → *No firm trigger yet*.
pub fn guide_modified_finding(path: &str) -> Finding {
    let running = env!("CARGO_PKG_VERSION");
    Finding::graded(
        Severity::Advisory,
        GUIDE_MODIFIED_CODE,
        format!(
            "`{path}` no longer carries the bytes jigc wrote, so jigc left it untouched \
             rather than clobber your edits — it is no longer version-matched to jigc {running}"
        ),
        None,
        Some(
            format!(
                "keep your copy and jigc will keep leaving it alone, or delete `{path}` and \
                 re-run `jigc setup` to reinstall jigc's own copy stamped at {running}"
            )
            .into(),
        ),
    )
}

/// The **advisory** the teardown raises over a user-modified artifact it left standing —
/// [`guide_modified_finding`]'s sibling at the closing door, and the same detector answering
/// the same question, so it carries the same code. Only the register differs: `setup` refused
/// to *overwrite* the file, `uninstall` refuses to *delete* it, so they route at different
/// exits (`design/project-setup.md` → Flow 2 hardening → Teardown / cleanup (G5)).
///
/// Advisory, never blocking, for the same reason and one stronger: a teardown that stops
/// because one file was edited leaves the install half-removed, and the state it is reporting
/// is not damage — it is a file that is now the user's. The route states both admissible
/// answers, the second being `uninstall`'s **already-shipped** consent hatch rather than a new
/// one: `--force` deletes it, so the deletion is asked for rather than assumed.
pub fn guide_kept_finding(path: &str) -> Finding {
    Finding::graded(
        Severity::Advisory,
        GUIDE_MODIFIED_CODE,
        format!(
            "`{path}` no longer carries the bytes jigc wrote, so the teardown left it in place \
             rather than delete your edits — it is yours now, not part of jigc's install"
        ),
        None,
        Some(
            format!(
                "keep it, or delete `{path}` yourself — or re-run `jigc uninstall --force` to \
                 remove it with the rest of the install"
            )
            .into(),
        ),
    )
}

/// Write the guide artifact to `<repo_root>/<guide.file>`, creating its parent dirs.
///
/// Rewritten **whole** on every `setup`, on the `.jigc/AGENT.md` mold: the file is wholly
/// CLI-owned, so it needs no in-file idempotency markers and a re-run over the same binary
/// is byte-identical. A copy stamped at an older version is therefore *replaced* — which is
/// what "regenerated on upgrade" means for an artifact only `setup` writes
/// (`design/assistant-adapter.md` → Generated, minimal, regenerated). The caller gates this
/// on [`guide_ownership`]: only [`GuideOwnership::Owned`] and `Absent` reach here.
fn write_guide_artifact(repo_root: &Path, guide: &adapter::GuideTarget) -> std::io::Result<()> {
    let target = repo_root.join(&guide.file);
    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&target, guide_artifact(guide))
}

/// Write the binary-provenance stamp under `<repo_root>/.jigc/version` (creating `.jigc/`
/// if absent). `jigc setup` writes it and store-writing ops (`finalize`) refresh it — a
/// same-build refresh writes identical bytes, so it is a no-op in the commit.
pub fn write_version_stamp(repo_root: &Path) -> std::io::Result<()> {
    let path = repo_root.join(VERSION_STAMP_PATH);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, version_stamp_body())
}

/// Read the recorded stamp version from `<repo_root>/.jigc/version`, or `None` when the
/// file is **absent** (a pre-M36 store — never false-flagged) or carries no parseable
/// `jigc-version:` line.
fn read_version_stamp(repo_root: &Path) -> Option<String> {
    let text = std::fs::read_to_string(repo_root.join(VERSION_STAMP_PATH)).ok()?;
    text.lines()
        .find_map(|line| line.trim().strip_prefix(VERSION_STAMP_KEY))
        .map(|rest| rest.trim().to_string())
        .filter(|version| !version.is_empty())
}

/// The store-scope `store-version.binary-mismatch` advisory for the store at `jigc_home`,
/// or `None` when the recorded stamp **matches** the running build or is **absent** (a
/// pre-M36 store — never false-flagged). Report-only + **un-keyed**: `(store-version,
/// binary-mismatch)` is not a `CHECK_INVENTORY` row, so the severity post-pass leaves its
/// advisory severity untouched and it never gates a transaction (`design/validation.md` →
/// un-keyed findings; `design/storage.md` → Store provenance — the check). Its `target` is
/// **`null`, and correct** — the store is a singleton, the one declared exception
/// (`design/command-output-contract.md` → the declared non-unique exceptions). A version delta
/// is a heads-up that two builds may resolve the cascade differently, not corruption.
///
/// **Schema-version-aware since M42** (`design/storage.md` → Store provenance — the check).
/// `corpus_stale` says whether the same sweep found the committed corpus stale (a
/// `schema-conformance.schema-version-current` break — the machine handle, **never** a
/// route-string match). It splits the route in two, because the un-split route was a **false
/// all-clear**: it named only *"re-run `jigc setup`"*, and `jigc setup` **re-stamps
/// `.jigc/version`** — self-clearing this very advisory while the corpus stayed stale, so the
/// next `jigc validate` looked like progress.
///   - **stale corpus** → the route names **`jigc migrate-corpus` first** (the verb that
///     actually upgrades the docs), and the re-stamp only after it.
///   - **current corpus** (the divergent-binary-only case) → the plain align-or-re-stamp route
///     stands; naming `migrate-corpus` here would command a verb with nothing to do.
pub fn binary_mismatch_finding(jigc_home: &Path, corpus_stale: bool) -> Option<Finding> {
    let recorded = read_version_stamp(jigc_home)?;
    let running = env!("CARGO_PKG_VERSION");
    if recorded == running {
        return None;
    }
    let provenance = format!("store last written by jigc {recorded}; you are running {running}");
    let (message, route) = if corpus_stale {
        (
            format!(
                "{provenance} — and this store's committed docs are stale against {running}'s \
                 schemas: re-stamping alone would clear this advisory while the corpus stayed stale"
            ),
            format!(
                "run `jigc migrate-corpus` to upgrade the committed docs, then re-run \
                 `jigc setup` to re-stamp the store at {running} (or align the running jigc \
                 back to {recorded})"
            ),
        )
    } else {
        (
            format!("{provenance} — align versions or re-run `jigc setup`"),
            format!(
                "align the running jigc to {recorded}, or re-run `jigc setup` to re-stamp \
                 the store at {running}"
            ),
        )
    };
    Some(Finding::graded(
        Severity::Advisory,
        "store-version.binary-mismatch",
        message,
        None,
        Some(route.into()),
    ))
}

/// The sentinel marker the generated `pre-commit` hook carries on its first body
/// line — the idempotency handle the neutral install (T2) keys on to find, replace,
/// or wrap the jigc-owned block on every `setup` (`design/assistant-adapter.md` →
/// neutral install: "a sentinel-marked block, re-runnable, regenerated on every
/// `setup`"). It must be a stable, unique string; the install path matches on it
/// verbatim, so it lives here beside the body it marks.
pub const PRECOMMIT_SENTINEL: &str =
    "# jigc-managed pre-commit hook (doc<->code backstop) — regenerated by `jigc setup`";

/// Render the assistant-neutral `pre-commit` hook script body for a `jigc` resolved
/// to `jigc_path` — the load-bearing **output-discipline** contract (M19 review B2,
/// `design/assistant-adapter.md` → neutral install; `design/validation.md` →
/// Auto-firing the sweep).
///
/// The hook runs `<jigc_path> validate --format json` (the store-scope doc↔code
/// sweep) and **warns the committer only on real doc↔code drift**, always exiting
/// `0` — it is **warn-only**, it never rejects a commit. The discipline keys on the
/// **content findings**, never on the exit code (the exit code is wrong-way-round —
/// `jigc validate` exits 0 on *found drift* but non-zero on not-a-project /
/// probe-missing / probe-integrity). Concretely, it surfaces a warning **iff** the
/// JSON report on stdout names `doc-code` in its top-level **`blocking_probes`**
/// array; on a clean store, a store whose only `doc-code` finding is *advisory*, a
/// not-a-jigc-project, a probe-missing run, any `pack-probe-integrity` meta-finding,
/// or a `jigc` that is absent / fails to run, it **prints nothing and exits 0**.
///
/// **The match keys on severity, via the array** (M47, `design/command-output-contract.md`
/// → The store sweep's envelope; DECISIONS.md → 2026-07-26 M47, Decision 7 re-settled).
/// Until M47 it grepped the report for a `"probe": "doc-code"` finding at *any* severity,
/// which warned forever over a store whose only `doc-code` finding is the advisory
/// stale-heading guard — permanent false drift for a project the probe cannot fully check.
/// A finding-object grep also has to bind a key to a member **across nested `{}`**, which no
/// line-oriented pattern can do soundly. `blocking_probes` is a **flat array of plain
/// strings**, so the match is bounded by the array's **own `]`** (`[^]]*`) and structurally
/// cannot reach into `findings`. Two mechanics are load-bearing:
///   - the report is pretty-printed, so the array spans several lines — the pattern is fed a
///     **newline-collapsed** copy (`tr -d '\n'`), without which it matches nothing;
///   - that collapse is **pipeline-local**: `$report` itself keeps its newlines, because
///     [`PRECOMMIT_RENAME_BLOCK`] pairs one `git mv` route **per line**.
///
/// The whitespace classes are POSIX (`[[:space:]]`, never the GNU-only `\s`) so the pattern
/// holds under BSD/macOS `grep -E` as well.
///
/// The **absolute** installing-`jigc` path is embedded (the PATH-vs-absolute hazard:
/// a hook calling bare `jigc` would run whatever is on PATH at commit time, possibly
/// a stale binary, and would also break doc-code sibling-probe resolution; the
/// absolute path pins both). A pure CLI-side renderer — the engine stays
/// filesystem-/shell-free. Disk placement + idempotency land in T2.
pub fn precommit_hook_body(jigc_path: &Path) -> String {
    // The committed-in jigc path. `display()` is the install-time, single-machine
    // path (a non-UTF-8 path would render lossily, but a git hook on such a path is
    // not a target we support); quoted in the script so a path with spaces survives.
    let jigc = jigc_path.display();
    format!(
        "#!/bin/sh\n\
         {PRECOMMIT_SENTINEL}\n\
         #\n\
         # Warn-only doc<->code drift backstop: runs `jigc validate` over the\n\
         # committed store and prints a warning ONLY when a doc-code check raised a\n\
         # BLOCKING finding. Always exits 0 — it never blocks the commit. Keys on the\n\
         # findings, never on the exit code (the exit code is wrong-way-round).\n\
         \n\
         jigc='{jigc}'\n\
         \n\
         # Run the sweep, capturing stdout only. If jigc is absent or fails to run,\n\
         # `report` is empty and nothing below matches -> silent exit 0.\n\
         report=\"$(\"$jigc\" validate --format json 2>/dev/null)\"\n\
         \n\
         # Warn IFF the report's top-level `blocking_probes` array names `doc-code` —\n\
         # the probes that raised a BLOCKING finding. Keying on SEVERITY, not on mere\n\
         # presence: an advisory-only doc-code finding (the stale-heading guard; a\n\
         # citation the probe cannot check) must not warn on every commit forever. A\n\
         # clean store, a not-a-jigc-project / probe-missing run (an `error` envelope\n\
         # on stderr, nothing matching here on stdout), and any `pack-probe-integrity`\n\
         # meta-finding all fall through to a silent exit 0.\n\
         #\n\
         # The report is pretty-printed, so the array spans several lines: the newline\n\
         # collapse is what lets one ERE bind the key to a member (without it this\n\
         # matches nothing). The collapse is PIPELINE-LOCAL — `$report` itself keeps\n\
         # its newlines, because the rename block below pairs one route per LINE. The\n\
         # match is bounded by the array's OWN `]`, so it cannot reach into `findings`;\n\
         # whitespace is POSIX-classed so it holds on BSD/macOS grep and survives\n\
         # compact or differently-spaced JSON.\n\
         if printf '%s' \"$report\" | tr -d '\\n' | grep -Eq '\"blocking_probes\"[[:space:]]*:[[:space:]]*\\[[^]]*\"doc-code\"'; then\n\
         \techo 'jigc: doc<->code drift detected in committed docs — run `jigc validate` for details (commit not blocked).' >&2\n\
         fi\n\
         {PRECOMMIT_RENAME_BLOCK}\
         \n\
         exit 0\n"
    )
}

/// The M35 Component B **OOB-rename backstop** spliced into [`precommit_hook_body`] (before
/// its trailing `exit 0`) — the one part of the hook that **blocks** a commit
/// (`design/validation.md` → The M19 pre-commit backstop; `design/reconciliation.md` →
/// Rename detection, Component B). It is a literal `&str` (not a `format!` fragment) so its
/// shell `${…}`/`awk {…}` braces stay verbatim — `format!` interpolates this value whole, so
/// its braces are never re-parsed.
///
/// The store-scope sweep (already captured in `$report`) flags every recorded-but-missing
/// managed doc as a `reconciliation.rename` finding whose **strong-signal route** names the
/// pair as `git mv <new> <old>` (the revert direction). But a move landed in a **prior**
/// commit must **not** block an unrelated later commit (the **masking trap**) — so the block
/// fires **iff** BOTH the finding's old and new paths are in **this commit's** staged set
/// (`git diff --cached --name-status --find-renames`, which carries both whether git records
/// the move as one `R old new` line or as `D old` / `A new`). A rename hit **outside** the
/// staged set warns and exits 0 (the guard). The decision keys on the finding **and** the
/// staged set, never on `jigc validate`'s exit code (which is wrong-way-round and is, per
/// M35, exit-flipped on this very finding — independent of this hook).
const PRECOMMIT_RENAME_BLOCK: &str = "\n\
# M35 — block this commit IFF it ITSELF stages an out-of-band managed-doc rename (a\n\
# bare `git mv` committed without `jigc rename`). The sweep above flags every\n\
# recorded-but-missing managed doc as a `reconciliation.rename` finding whose route\n\
# names the pair as `git mv <new> <old>` (the revert direction). A move landed in a\n\
# PRIOR commit must NOT block an unrelated later commit (the masking trap), so block\n\
# ONLY when BOTH the old and new paths are staged in THIS commit. Keys on the finding\n\
# plus the staged set, never on jigc's exit code.\n\
moves=\"$(printf '%s' \"$report\" | grep -o 'git mv [^`]*')\"\n\
if [ -n \"$moves\" ]; then\n\
\t# Every path THIS commit stages, rename-aware: a staged `git mv` shows as `R old new`\n\
\t# under --find-renames; a delete+add as `D old` / `A new`. One path per line.\n\
\tstaged=\"$(git diff --cached --name-status --find-renames 2>/dev/null | cut -f2- | tr '\\t' '\\n')\"\n\
\t# Block iff some `git mv <new> <old>` route has BOTH its paths in the staged set.\n\
\tif { printf '%s\\n' \"$staged\"; echo '---'; printf '%s\\n' \"$moves\"; } | awk '$0 == \"---\" { seen = 1; next } seen == 0 { S[$0] = 1; next } NF >= 4 && ($3 in S) && ($4 in S) { hit = 1 } END { exit hit ? 0 : 1 }'; then\n\
\t\techo 'jigc: out-of-band managed-doc rename staged in this commit — a bare `git mv` bypasses jigc identity tracking; use `jigc rename` instead (commit blocked).' >&2\n\
\t\texit 1\n\
\tfi\n\
\techo 'jigc: an out-of-band managed-doc rename exists in the committed tree — run `jigc validate` for details (not staged in this commit; commit not blocked).' >&2\n\
fi\n";

/// The sentinel that closes the jigc-managed block when it is **wrapped** around a
/// pre-existing foreign `pre-commit` hook. A fresh (jigc-only) hook is exactly the
/// rendered [`precommit_hook_body`] and carries no end marker; a wrapped hook
/// brackets the appended jigc block between [`PRECOMMIT_SENTINEL`] and this line so a
/// re-install can strip-and-regenerate **only** the jigc block, leaving the foreign
/// hook verbatim (non-destructive + idempotent).
const PRECOMMIT_SENTINEL_END: &str = "# jigc-managed pre-commit hook — end";

/// Install the assistant-neutral warn-only `pre-commit` hook for the repo at
/// `repo_root`, pointing it at the absolute `jigc_path` (`design/assistant-adapter.md`
/// → neutral install: sentinel-marked, idempotent, non-destructive; honor
/// `core.hooksPath` + worktrees; regenerated each `setup`).
///
/// Resolves the **real** hooks dir via a single `git rev-parse --git-path hooks`
/// subprocess (honors `core.hooksPath`, the git-worktree `.git`-is-a-file case, and
/// the common-hooks-dir for linked worktrees — verified live, git 2.53), never the
/// naive `.git/hooks` join. Then writes the rendered [`precommit_hook_body`]
/// idempotently and non-destructively:
///   - no existing hook (or one that is *only* a prior jigc block) → the file becomes
///     exactly the freshly rendered body (regenerated each `setup`);
///   - a pre-existing **foreign** hook → its content is preserved **verbatim** and a
///     jigc block (bracketed by [`PRECOMMIT_SENTINEL`]/[`PRECOMMIT_SENTINEL_END`]) is
///     spliced in just after the foreign shebang and **before** the foreign body, so
///     the warn-only backstop runs even when the foreign hook ends in an explicit
///     `exit` (it never blocks — control falls through to the foreign hook); a
///     re-install strips and regenerates only that block, so the result is
///     byte-identical and the sentinel appears exactly once.
///
/// The written file is made owner-executable (a git hook must be executable to fire).
///
/// Returns the hook's **resolved** path, so the caller can name the file it actually
/// wrote instead of the assumed `.git/hooks/pre-commit` literal (D4 — the literal lies
/// under `core.hooksPath` and in a linked worktree, the two cases this resolution
/// exists for).
pub fn install_precommit_hook(repo_root: &Path, jigc_path: &Path) -> std::io::Result<PathBuf> {
    let hooks_dir = resolve_hooks_dir(repo_root)?;
    std::fs::create_dir_all(&hooks_dir)?;
    let hook = hooks_dir.join("pre-commit");

    let rendered = precommit_hook_body(jigc_path);

    let next = match std::fs::read_to_string(&hook) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => rendered,
        Err(e) => return Err(e),
        Ok(existing) => {
            let foreign = strip_managed_block(&existing, &rendered);
            if foreign.trim().is_empty() {
                // Empty, or only a prior jigc block: regenerate the standalone hook.
                rendered
            } else {
                // Preserve the foreign hook verbatim; run the wrapped jigc block
                // *before* it. The block is warn-only and never exits, so the foreign
                // hook still runs (and owns the final exit) — but the backstop is
                // reached even when the foreign hook ends in an explicit `exit`, which
                // a verbatim-append wrap would skip. The foreign hook keeps its own
                // shebang; the jigc block (shebang- and `exit 0`-stripped, bracketed
                // by the start/end sentinels) is spliced in just after it.
                let foreign = foreign.trim_start_matches('\n');
                let (shebang, rest) = match foreign.split_once('\n') {
                    Some((first, rest)) if first.starts_with("#!") => (format!("{first}\n"), rest),
                    _ => (String::new(), foreign),
                };
                // Normalize the separator so a re-install (strip-and-regenerate) is
                // byte-identical: exactly one blank line between the jigc block's
                // end-marker and the foreign body.
                let rest = rest.trim_start_matches('\n');
                let mut out = shebang;
                out.push_str(&wrapped_managed_block(jigc_path));
                out.push('\n');
                out.push_str(rest);
                out
            }
        }
    };

    std::fs::write(&hook, next)?;
    make_executable(&hook)?;
    Ok(hook)
}

/// Render an installed hook path for the success summary: repo-root-relative when the
/// hook lives inside the repo (the common `.git/hooks/pre-commit` case, and a
/// `core.hooksPath` pointing in-repo), absolute otherwise — a `core.hooksPath` outside
/// the repo, or the **common** hooks dir a linked worktree resolves to, neither of
/// which any repo-relative path can name. Both sides are canonicalized before the
/// strip so a symlinked repo root (macOS `/var` → `/private/var`) still reads relative.
fn display_hook_path(repo_root: &Path, hook: &Path) -> String {
    let (root, real) = match (
        std::fs::canonicalize(repo_root),
        std::fs::canonicalize(hook),
    ) {
        (Ok(root), Ok(real)) => (root, real),
        _ => return hook.display().to_string(),
    };
    match real.strip_prefix(&root) {
        Ok(relative) => relative.display().to_string(),
        Err(_) => real.display().to_string(),
    }
}

/// The jigc-managed block for the **wrap** case: the rendered body with its shebang
/// line dropped (the foreign hook owns the shebang) and its trailing `exit 0` dropped
/// (the block is warn-only — it must fall through to the foreign hook that follows it,
/// which owns the final exit), with an end-sentinel appended so the block is
/// self-delimited for strip-and-regenerate. The block runs **before** the foreign hook
/// so the backstop fires even when the foreign hook ends in an explicit `exit`.
fn wrapped_managed_block(jigc_path: &Path) -> String {
    let body = precommit_hook_body(jigc_path);
    // Drop the leading `#!/bin/sh\n` shebang — the wrapped block runs inside the
    // foreign hook's interpreter.
    let without_shebang = body
        .strip_prefix("#!/bin/sh\n")
        .expect("the rendered body always begins with the sh shebang");
    // Drop the trailing `exit 0\n` — the wrapped block must not terminate the script;
    // control falls through to the foreign hook spliced in after it.
    let without_exit = without_shebang
        .strip_suffix("exit 0\n")
        .expect("the rendered body always ends with `exit 0`")
        .trim_end_matches('\n');
    format!("{without_exit}\n{PRECOMMIT_SENTINEL_END}\n")
}

/// Remove the jigc-managed block from a pre-existing hook, returning the foreign
/// remainder. A block counts as jigc-managed in exactly two forms:
///   - a **wrapped** block bracketed by BOTH [`PRECOMMIT_SENTINEL`] (start) and
///     [`PRECOMMIT_SENTINEL_END`] (end) — cut out inclusive of both markers and the
///     trailing newline, the foreign remainder returned;
///   - a **standalone** jigc hook — `content` byte-identical to `rendered` (the freshly
///     rendered standalone body, which carries the start sentinel but no end marker) —
///     wholly ours, so the remainder is empty.
///
/// Anything else is **foreign** and returned unchanged — including a foreign hook that
/// merely *contains* the start-sentinel string on a line but has no matching end marker
/// and is not our rendered body. Treating a start-sentinel-without-end-marker as wholly
/// jigc-managed would overwrite that foreign hook (synthetic data-loss); a complete
/// bracketed block (or the exact standalone body) is the only thing we own.
fn strip_managed_block<'a>(content: &'a str, rendered: &str) -> std::borrow::Cow<'a, str> {
    let Some(start) = content.find(PRECOMMIT_SENTINEL) else {
        return std::borrow::Cow::Borrowed(content);
    };
    match content[start..].find(PRECOMMIT_SENTINEL_END) {
        // Wrapped block: cut from the start-sentinel's line through the end marker.
        Some(rel_end) => {
            // Back up to the beginning of the start-sentinel's line.
            let block_start = content[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
            // Advance past the end-marker line (its trailing newline if present).
            let abs_end = start + rel_end + PRECOMMIT_SENTINEL_END.len();
            let block_end = content[abs_end..]
                .find('\n')
                .map(|i| abs_end + i + 1)
                .unwrap_or(content.len());
            let mut out = String::from(&content[..block_start]);
            out.push_str(&content[block_end..]);
            std::borrow::Cow::Owned(out)
        }
        // No end marker: ours only if the file is byte-identical to a freshly rendered
        // standalone hook. A foreign hook that merely references the start sentinel is
        // preserved verbatim (wrapped, not stripped).
        None if content == rendered => std::borrow::Cow::Borrowed(""),
        None => std::borrow::Cow::Borrowed(content),
    }
}

/// The teardown verdict for an existing `pre-commit` hook — what [`remove_precommit_hook`]
/// does with the file it found.
enum PrecommitTeardown {
    /// No jigc block found (a purely foreign hook, or a foreign hook that merely
    /// references the start sentinel) — leave the file untouched.
    NotOurs,
    /// The file was a standalone jigc hook (wholly ours) — remove it entirely.
    RemoveFile,
    /// A wrapped foreign hook — restore this foreign remainder to disk.
    RestoreForeign(String),
}

/// Classify an existing `pre-commit` hook's `content` for teardown — the jigc-path-free
/// inverse of [`install_precommit_hook`]'s splice (`uninstall` does not know which
/// absolute `jigc` path the hook was installed with, so it keys on the sentinels'
/// **structure**, never on a rendered-body byte match):
///   - a **wrapped** block bracketed by BOTH [`PRECOMMIT_SENTINEL`] and
///     [`PRECOMMIT_SENTINEL_END`] → cut inclusive of both markers and restore the
///     foreign remainder verbatim (empty remainder → remove the file);
///   - a **standalone** jigc hook — the rendered body's fixed prefix
///     `#!/bin/sh\n{PRECOMMIT_SENTINEL}\n` (any installing-`jigc` path) → wholly ours,
///     remove the file;
///   - anything else — including a foreign hook that merely *contains* the start
///     sentinel string but has no matching end marker and is not our standalone shape
///     — is foreign and left untouched (mirrors [`strip_managed_block`]'s data-loss
///     guard on the install side).
fn classify_precommit_for_teardown(content: &str) -> PrecommitTeardown {
    let Some(start) = content.find(PRECOMMIT_SENTINEL) else {
        return PrecommitTeardown::NotOurs;
    };
    // Wrapped block: bracketed by both sentinels — cut it out, restore the foreign body.
    if let Some(rel_end) = content[start..].find(PRECOMMIT_SENTINEL_END) {
        // Back up to the beginning of the start-sentinel's line (keeps a preceding
        // foreign shebang), and advance past the end-marker's own line.
        let block_start = content[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
        let abs_end = start + rel_end + PRECOMMIT_SENTINEL_END.len();
        let after = content[abs_end..]
            .find('\n')
            .map(|i| &content[abs_end + i + 1..])
            .unwrap_or("");
        // Drop the single blank-line separator `install_precommit_hook` inserts between
        // the jigc block's end marker and the foreign body, so the restore matches the
        // foreign hook's normalized pre-wrap bytes.
        let mut foreign = String::from(&content[..block_start]);
        foreign.push_str(after.trim_start_matches('\n'));
        if foreign.trim().is_empty() {
            return PrecommitTeardown::RemoveFile;
        }
        return PrecommitTeardown::RestoreForeign(foreign);
    }
    // No end marker: ours only if the file is a standalone jigc hook — the rendered
    // body's fixed prefix. A foreign hook that merely references the start sentinel
    // elsewhere is preserved verbatim.
    let standalone_prefix = format!("#!/bin/sh\n{PRECOMMIT_SENTINEL}\n");
    if content.starts_with(&standalone_prefix) {
        PrecommitTeardown::RemoveFile
    } else {
        PrecommitTeardown::NotOurs
    }
}

/// Idempotently **remove** the jigc-managed `pre-commit` hook from the repo at
/// `repo_root` — the inverse of [`install_precommit_hook`] for `jigc uninstall`
/// (`design/project-setup.md` → Flow 2 hardening → Teardown, the M36 symmetry fix:
/// "both hooks must come out"). Resolves the **real** hooks dir the same way install
/// does (honoring `core.hooksPath` + worktrees).
///
/// A **standalone** jigc hook is removed entirely; a **wrapped** foreign hook has only
/// the jigc block (bracketed by the start/end sentinels) pruned, restoring the foreign
/// hook; a purely foreign hook is left untouched. **Idempotent + non-destructive:** an
/// absent hook, or a foreign hook, is a clean no-op, so a second `uninstall` is a
/// no-op. See [`classify_precommit_for_teardown`] for the exact detection.
///
/// Returns `Ok(true)` when a jigc-managed hook was actually removed or unwrapped,
/// `Ok(false)` on the no-op (absent or purely foreign hook) — the honest signal the
/// teardown summary reports on.
pub fn remove_precommit_hook(repo_root: &Path) -> std::io::Result<bool> {
    let hooks_dir = resolve_hooks_dir(repo_root)?;
    let hook = hooks_dir.join("pre-commit");

    let existing = match std::fs::read_to_string(&hook) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(false),
        Err(e) => return Err(e),
        Ok(existing) => existing,
    };

    match classify_precommit_for_teardown(&existing) {
        PrecommitTeardown::NotOurs => Ok(false),
        PrecommitTeardown::RemoveFile => match std::fs::remove_file(&hook) {
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
            Err(e) => Err(e),
            Ok(()) => Ok(true),
        },
        // The file already exists and stays executable; `write` preserves its mode.
        PrecommitTeardown::RestoreForeign(foreign) => std::fs::write(&hook, foreign).map(|()| true),
    }
}

/// Resolve the repo's **real** hooks directory through git, honoring
/// `core.hooksPath`, the worktree `.git`-is-a-file case, and the common hooks dir for
/// linked worktrees. A single `git -C <repo_root> rev-parse --path-format=absolute
/// --git-path hooks` subprocess — never the naive `.git/hooks` join.
fn resolve_hooks_dir(repo_root: &Path) -> std::io::Result<PathBuf> {
    let out = std::process::Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .args(["rev-parse", "--path-format=absolute", "--git-path", "hooks"])
        .output()?;
    if !out.status.success() {
        return Err(std::io::Error::other(format!(
            "git could not resolve the hooks dir for `{}`: {}",
            repo_root.display(),
            String::from_utf8_lossy(&out.stderr).trim(),
        )));
    }
    let path = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if path.is_empty() {
        return Err(std::io::Error::other(
            "git returned an empty hooks path".to_string(),
        ));
    }
    Ok(PathBuf::from(path))
}

/// Make `path` owner/group/other-readable and owner-executable (`0o755`) so git will
/// fire it as a hook. Unix-only — git hooks are a Unix-shell mechanism here.
fn make_executable(path: &Path) -> std::io::Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let mut perms = std::fs::metadata(path)?.permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(path, perms)
}

/// The `doc-code` probe executable, embedded into the `jigc` binary so it travels
/// through `cargo install` (which relocates only declared `[[bin]]` targets — the
/// build-script sibling does not travel; `module-layout.md` → Probe distribution,
/// M20). `build.rs` builds the probe and copies it into `OUT_DIR/doc-code`; the
/// `(I)`-pick is that the include path is nameable only after `build.rs` has run,
/// hence the `OUT_DIR` indirection. [`extract_doc_code_probe`] writes these bytes
/// beside the installed `jigc` at `jigc setup` so the production resolution path
/// (`<jigc-bin-dir>/doc-code`) finds a runnable probe with no manual copy.
const DOC_CODE_PROBE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/doc-code"));

/// Extract the embedded [`DOC_CODE_PROBE`] beside the running `jigc` at `bin_dir`,
/// applying the pinned **heal/upgrade** policy (`module-layout.md` → Probe
/// distribution, M20): write when no sibling exists **or** when an existing sibling's
/// bytes differ from the embedded copy. This is the **first machine-global** setup
/// write — every other setup write is repo-local — so its idempotency is judged at the
/// install-tree scope, not per-repo.
///
/// Writes `bin_dir/doc-code` with the exec bit (`0o755`) when the sibling is **absent
/// or byte-different**, and is a **no-op only** when an existing sibling is
/// byte-identical to the embedded copy. Overwriting a byte-different sibling **heals** a
/// corrupt stub (which would otherwise surface as a `pack-probe-integrity` failure) and
/// **upgrades** a stale probe left behind by a prior `jigc` after an upgrade. (In a
/// build tree this also overwrites the `build.rs`-placed sibling with the embedded copy;
/// both are functional probes built from the same source — debug builds simply aren't
/// byte-reproducible — so the swap is harmless.) A read-only target dir surfaces the
/// underlying IO error to the caller, which maps it to one operational `setup.*` finding
/// (never a panic).
fn extract_doc_code_probe(bin_dir: &Path) -> std::io::Result<()> {
    let dest = bin_dir.join("doc-code");
    // No-op only when an existing sibling is byte-identical to the embedded copy.
    // An absent sibling, or one whose bytes differ (a stale post-upgrade probe or a
    // corrupt stub), is (re)written from the embedded copy — heal/upgrade.
    if let Ok(existing) = std::fs::read(&dest)
        && existing == DOC_CODE_PROBE
    {
        return Ok(());
    }
    std::fs::write(&dest, DOC_CODE_PROBE)?;
    make_executable(&dest)
}

/// The sentinel comment that heads the secrets-floor block in a root `.gitignore`
/// ([`SECRETS_GITIGNORE_BLOCK`]) — the idempotency handle [`seed_secrets_gitignore`]
/// keys on to leave an already-seeded file byte-untouched. Its presence anywhere in the
/// file means the floor is already there.
const SECRETS_GITIGNORE_SENTINEL: &str = "# jigc secrets floor — safe defaults; edit freely";

/// The secrets-floor `.gitignore` block `jigc setup` seeds on the **fresh-repo path
/// only** (`design/project-setup.md` → The secrets-floor `.gitignore`). The same secret
/// set the adapter's `deny` floor blocks — one list, two enforcement points — rendered as
/// gitignore patterns. Headed by [`SECRETS_GITIGNORE_SENTINEL`] and terminated by a
/// newline so a create writes a clean, complete block.
const SECRETS_GITIGNORE_BLOCK: &str = "\
# jigc secrets floor — safe defaults; edit freely
.env
.env.*
# keep a committed template
!.env.example
*.pem
*.key
id_rsa
id_rsa.*
id_ed25519
id_ed25519.*
credentials
.npmrc
";

/// Whether `repo_root` is a **fresh** (zero-commit) git repo — the discriminator that
/// gates the secrets-floor `.gitignore` seed to greenfield repos only
/// (`design/project-setup.md` → The secrets-floor `.gitignore`: "seed iff the repo has
/// zero commits"). **Conservative on every unclear signal:** not a git work tree, git
/// unavailable, or an unreadable HEAD state → `false` (do not seed), so the floor is
/// never dropped into an established or ambiguous repo (a missing floor is cheap; a
/// wrongful seed is the scope breach we refuse).
///
/// Zero commits ⇔ HEAD does not resolve to an object (an unborn HEAD). This keys on
/// `git rev-parse --verify --quiet HEAD` (exit 0 with a sha once the first commit exists,
/// non-zero on an unborn HEAD) rather than the design's illustrative `rev-list --count
/// HEAD == 0`: `rev-list --count HEAD` *errors* on the very unborn case we must seed, so
/// it can't be read as "0". The work-tree check runs first so a `rev-parse --verify`
/// failure is unambiguously "unborn HEAD," never "not a repo."
fn is_fresh_repo(repo_root: &Path) -> bool {
    match git_output(repo_root, ["rev-parse", "--is-inside-work-tree"]) {
        Some(out) if out.status.success() => {}
        _ => return false,
    }
    match git_output(repo_root, ["rev-parse", "--verify", "--quiet", "HEAD"]) {
        Some(out) => !out.status.success(),
        None => false,
    }
}

/// Seed the secrets-floor root `.gitignore` on the **fresh-repo path only**
/// ([`is_fresh_repo`]), merge-never-clobber (`design/project-setup.md` → The secrets-floor
/// `.gitignore`). Returns `Ok(true)` when the repo is fresh and its root `.gitignore`
/// now carries the floor (so [`commit_install`] tracks it), `Ok(false)` when the repo is
/// established / the signal is unclear (a clean no-op — an existing project's `.gitignore`
/// is never touched).
///
/// On the fresh path: absent → create with exactly [`SECRETS_GITIGNORE_BLOCK`]; present
/// **without** the sentinel → append the block under it, preserving the human's lines
/// verbatim; present **with** the sentinel → byte-stable no-op. So a re-run is
/// byte-identical.
fn seed_secrets_gitignore(repo_root: &Path) -> std::io::Result<bool> {
    if !is_fresh_repo(repo_root) {
        return Ok(false);
    }
    let path = repo_root.join(".gitignore");
    match std::fs::read_to_string(&path) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            std::fs::write(&path, SECRETS_GITIGNORE_BLOCK)?;
        }
        Err(e) => return Err(e),
        // Already carries the floor — leave every byte untouched (idempotent re-run).
        Ok(existing) if existing.contains(SECRETS_GITIGNORE_SENTINEL) => {}
        // A foreign `.gitignore`: keep its lines verbatim, append the floor under the
        // sentinel, separated by exactly one blank line (byte-stable on re-run).
        Ok(existing) => {
            let mut out = existing;
            if !out.ends_with('\n') {
                out.push('\n');
            }
            out.push('\n');
            out.push_str(SECRETS_GITIGNORE_BLOCK);
            std::fs::write(&path, out)?;
        }
    }
    Ok(true)
}

/// The assistant whose embedded profile MVP `setup` installs. Single-assistant in
/// the MVP (Claude Code); a `--assistant` selector is post-MVP
/// (`design/assistant-adapter.md` → Generated, minimal, regenerated).
pub(crate) const SETUP_ASSISTANT: &str = "claude-code";

/// The result of a `jigc setup` install: the located repo root and the profile's
/// two host targets, so the dispatcher can render a precise success summary.
#[derive(Debug)]
pub struct SetupSummary {
    /// The repo-root-relative always-loaded file the bootstrap reference was
    /// injected into.
    pub line_file: String,
    /// The repo-root-relative settings file the allowlist was merged into.
    pub allowlist_file: String,
    /// The `pre-commit` hook's path as git resolved it ([`display_hook_path`]) —
    /// repo-root-relative inside the repo, absolute when the hooks dir lives outside it
    /// (`core.hooksPath`, or a linked worktree's common hooks dir).
    pub hook_file: String,
    /// Whether the installed `pre-commit` hook rode the **install commit** — the pathspec
    /// [`commit_install`] actually committed from, not a re-reading of the path's shape.
    ///
    /// `false` on the default `.git/hooks` (git cannot track a path inside its own control
    /// dir), on a `core.hooksPath` outside the repo, from a linked worktree (whose hooks
    /// resolve to the main checkout's common dir), on a hooks dir another repo owns, and on
    /// the shapes where git refuses the path anyway (the soft-member drop). `true` for an
    /// in-worktree `core.hooksPath` whose hook git took.
    ///
    /// The summary line is listed under *"setup installed:"* either way — it IS installed,
    /// locally — so what the surface owes the reader is the **consequence** of a `false`
    /// here: the hook is in no commit, so a clone starts without the drift backstop until
    /// `jigc setup` runs there. Carried as the commit's own answer precisely so the
    /// sentence and the commit cannot drift apart (M48, the surface-fundament lens).
    pub hook_committed: bool,
    /// The repo-root-relative path of the adapter's **owned guide artifact** *this run
    /// wrote*, or `None` when the profile declares no guide target (the omitting context —
    /// inert, never an error) **or** when a user-modified copy was found and left alone.
    /// It names what setup installed, never merely where a file sits: the summary line it
    /// feeds says "stamped with this build", which of a user's own copy would be a lie.
    /// M48 Increment 10.
    pub guide_file: Option<String>,
    /// The install's **non-blocking** findings — advisories the run reports without failing
    /// (today: the user-modified guide artifact, [`guide_modified_finding`]). Empty on an
    /// ordinary install. A blocking outcome is not here: it is `install`'s `Err` arm, which
    /// still carries exactly one finding (M48 Increment 10 / T2).
    pub findings: engine::finding::Findings,
    /// The outcome of committing setup's own install files as a dedicated commit
    /// (M26 shakedown — see [`commit_install`]).
    pub install_commit: InstallCommit,
}

/// The conventional message for the dedicated commit `jigc setup` makes of its own
/// install files. M26 shakedown: setup's scaffolding (`.jigc/` config, the adapter
/// host files) must be its own commit, not swept into the user's first `jigc finalize`.
const INSTALL_COMMIT_MESSAGE: &str = "chore(jigc): install jigc workspace config";

/// The outcome of committing `jigc setup`'s own install files ([`commit_install`]).
#[derive(Debug)]
pub enum InstallCommit {
    /// A dedicated install commit was made; carries its short sha.
    Committed(String),
    /// Nothing to commit — a re-run over an unchanged install (a clean idempotent
    /// no-op, never an empty commit).
    Nothing,
    /// No commit was made for a **benign** reason: not a git repo, an unborn HEAD, no
    /// committable install footprint, or git could not be spawned. The install writes
    /// still succeeded and nothing was staged-but-orphaned — the commit is a
    /// convenience here, so it degrades gracefully exactly as the writes do
    /// (`design/assistant-adapter.md` → setup is the CLI writing install artifacts). A
    /// genuine commit **rejection** (e.g. no identity, leaving the staged files
    /// uncommitted) is NOT this — [`commit_install`] surfaces it as an error so
    /// [`install`] can fail loudly rather than print an unqualified success.
    Skipped,
}

/// Run `jigc setup` from `start`: locate the repo root, load the Claude Code
/// profile, and run both injections idempotently.
///
/// `Ok(summary)` on a clean install (including a re-run, which is a byte-identical
/// no-op by the injectors' idempotency). `Err(finding)` is a single blocking
/// `setup.*` finding carrying a route — the dispatcher renders it and exits
/// non-zero. CLI **locates** the repo root; the injectors do the writes (the
/// engine stays presentation-free and filesystem-free).
pub fn run(start: &Path) -> Result<SetupSummary, Finding> {
    // The not-in-a-repository cause answers with the ONE shared text + route every other
    // door gives (M49 Inc 11 T2, `locate::locate_finding`); the `$HOME`-unset cause is a
    // different precondition and keeps its own carry.
    let ctx = locate::locate(start).map_err(|err| {
        locate::locate_finding(
            "setup.repo-root",
            start,
            &err,
            "run `jigc setup` from inside the target git repository",
        )
    })?;

    let profile = adapter::load_profile(SETUP_ASSISTANT).map_err(|err| {
        Finding::block(
            "setup.profile-load",
            format!("cannot load the `{SETUP_ASSISTANT}` adapter profile: {err}"),
            "reinstall jigc — the embedded adapter profile is missing or malformed",
        )
    })?;

    install(&ctx.repo_root, &profile)
}

/// Run both injections against `repo_root` with `profile`, mapping an IO failure
/// to a blocking `setup.*` finding with a route. The testable core of [`run`]
/// (no location step).
fn install(repo_root: &Path, profile: &AdapterProfile) -> Result<SetupSummary, Finding> {
    // 0. Gate the spawn launch template against the decidable install-time rule
    //    *before* any write, so a broken template fails install touching nothing
    //    (`design/assistant-adapter.md` → Bind the spawn mechanism: "A broken
    //    template is an install-time error … the install does not complete"). The
    //    violated clause's pointer rides as the route.
    if let Err(reason) = adapter::validate_spawn_template(&profile.spawn.template) {
        return Err(Finding::block(
            "setup.spawn-template",
            format!(
                "the `{}` adapter profile's spawn launch template is invalid: {reason}",
                profile.assistant
            ),
            reason.to_string(),
        ));
    }
    // 0b. The same gate over the **guide** target, when the profile declares one — a
    //     path jigc is about to write and commit, so its decidable clauses are checked
    //     before any write too (`design/assistant-adapter.md` → The adapter's owned
    //     artifacts). A profile declaring none skips this and installs no guide.
    if let Some(guide) = profile.guide()
        && let Err(reason) = adapter::validate_guide_target(guide)
    {
        return Err(Finding::block(
            "setup.guide-target",
            format!(
                "the `{}` adapter profile's guide target is invalid: {reason}",
                profile.assistant
            ),
            reason.to_string(),
        ));
    }

    let reference = profile.reference().ok_or_else(|| {
        Finding::block(
            "setup.profile-incomplete",
            format!(
                "the `{}` adapter profile declares no inject reference floor",
                profile.assistant
            ),
            "reinstall jigc — the embedded adapter profile is missing its bootstrap reference",
        )
    })?;
    let line_file = reference.file.clone();
    let bootstrap_file = reference.to.clone();

    // 1. Reference floor: write the managed bootstrap file, then point the
    //    always-loaded file at it with a bare import line.
    adapter::write_bootstrap_file(repo_root).map_err(|err| {
        Finding::block(
            "setup.write-bootstrap",
            format!("cannot write the managed bootstrap file `{bootstrap_file}`: {err}"),
            format!("ensure `{bootstrap_file}` is writable, then re-run `jigc setup`"),
        )
    })?;
    adapter::inject_reference(repo_root).map_err(|err| {
        Finding::block(
            "setup.inject-reference",
            format!("cannot inject the bootstrap reference into `{line_file}`: {err}"),
            format!("ensure `{line_file}` is writable, then re-run `jigc setup`"),
        )
    })?;

    // 2. Initialize the project cascade layer, so the project resolves as set up.
    adapter::init_project_layer(repo_root).map_err(|err| {
        Finding::block(
            "setup.init-project-layer",
            format!("cannot initialize the project layer under `.jigc/`: {err}"),
            "ensure `.jigc/` is writable, then re-run `jigc setup`",
        )
    })?;

    // 2b. Write the `compose-embedded-methodology` marker into the project layer's
    //     `packs.yaml` — the marker `pack::read_compose_marker` reads to compose the
    //     embedded methodology pack as `[dev ▸ methodology]`, so a clean `setup` gives
    //     a real project the dev+methodology surface out of the box (M21,
    //     `design/multi-pack.md` → Embedded second pack + setup auto-wiring). A repo-
    //     local, non-destructive parse-mutate-serialize; idempotent (a second setup is
    //     a byte-identical no-op). A project that already declares a `packs:` list keeps
    //     that list — entries, order and content — and gets the marker alongside it: the
    //     loader composes `[listed… ▸ dev ▸ methodology]` (M49 Inc 6, see
    //     [`write_compose_marker`]).
    write_compose_marker(repo_root).map_err(|err| {
        Finding::block(
            "setup.compose-marker",
            format!(
                "cannot write the `compose-embedded-methodology` marker into `.jigc/config/packs.yaml`: {err}"
            ),
            "ensure `.jigc/config/` is writable, then re-run `jigc setup`",
        )
    })?;

    // 2c. Write the committed binary-provenance stamp (`design/storage.md` → Store
    //     provenance): a one-line `.jigc/version` recording which `jigc` build wrote the
    //     store, so an adopter on a divergent binary gets a `store-version.binary-mismatch`
    //     advisory (never a gate). Committed via `install_tracked_paths` so it travels with
    //     the repo; refreshed by store-writing ops (`finalize`).
    write_version_stamp(repo_root).map_err(|err| {
        Finding::block(
            "setup.version-stamp",
            format!("cannot write the binary-provenance stamp `{VERSION_STAMP_PATH}`: {err}"),
            "ensure `.jigc/` is writable, then re-run `jigc setup`",
        )
    })?;

    // 2d. Seed the secrets-floor root `.gitignore` — **fresh-repo (zero-commit) path
    //     only**, merge-never-clobber (`design/project-setup.md` → The secrets-floor
    //     `.gitignore`). On an established repo (>=1 commit) or an unclear signal it is a
    //     clean no-op, so an existing project's `.gitignore` is never touched. When it
    //     seeds, the file is tracked in the install commit ([`install_tracked_paths`]).
    let seeded_gitignore = seed_secrets_gitignore(repo_root).map_err(|err| {
        Finding::block(
            "setup.secrets-gitignore",
            format!("cannot seed the secrets-floor root `.gitignore`: {err}"),
            "ensure the repository root is writable, then re-run `jigc setup`",
        )
    })?;

    // 3. Allowlist `jigc` so the agent runs it without friction.
    let allowlist_file = profile.allowlist.file.clone();
    adapter::inject_allowlist(repo_root, profile).map_err(|err| {
        Finding::block(
            "setup.inject-allowlist",
            format!("cannot merge the allowlist into `{allowlist_file}`: {err}"),
            format!("ensure `{allowlist_file}` is writable, then re-run `jigc setup`"),
        )
    })?;

    // 4. Install the SessionStart hook (the primary bootstrap injection) into the
    //    same settings file. A no-op for a profile that declares no hook.
    adapter::inject_hook(repo_root, profile).map_err(|err| {
        Finding::block(
            "setup.inject-hook",
            format!("cannot install the session hook into `{allowlist_file}`: {err}"),
            format!("ensure `{allowlist_file}` is writable, then re-run `jigc setup`"),
        )
    })?;

    // 4b. Merge the `deny` **safety floor** into the same settings file — the bounded
    //     blocklist against catastrophic/exfil actions (destructive/exfil shell + secret-
    //     file reads on both the `Read` and `Bash` surfaces), merged never clobbered, the
    //     structure-aware twin of the allowlist merge (`design/assistant-adapter.md` → the
    //     `deny` safety floor). Idempotent; a re-run is byte-stable; a profile with no
    //     floor is inert.
    adapter::inject_deny(repo_root, profile).map_err(|err| {
        Finding::block(
            "setup.inject-deny",
            format!("cannot merge the deny safety floor into `{allowlist_file}`: {err}"),
            format!("ensure `{allowlist_file}` is writable, then re-run `jigc setup`"),
        )
    })?;

    // 4c. Write the adapter's own **owned artifact** — jigc's shipped guides at the
    //     profile-declared path, stamped with this build's version and the `blake3` of
    //     their own body (M48 Increment 10; `design/assistant-adapter.md` → The adapter's
    //     owned artifacts). VISION commits the adapter to skill files that just call the
    //     CLI and the install carried none, so an adopter had no version-matched path to
    //     the guides at all. Rewritten whole each `setup` on the `.jigc/AGENT.md` mold, so
    //     a re-run is byte-identical and a copy from an older build is replaced. Inert for
    //     a profile that declares no guide target.
    //
    //     **And it replaces only what jigc wrote** (M48 Increment 10 / T2). Ownership is
    //     decided from the file itself ([`guide_ownership`]) before any write: a copy whose
    //     recorded digest no longer describes its body — or that carries no jigc stamp at
    //     all — is the *user's*, so it is left byte-identical and reported as an advisory
    //     with a route (never blocking: stopping an install over an edited guide file would
    //     be hostile). It is also dropped from `guide_file`, so neither the summary's
    //     installed list nor the install commit's pathspec claims a file this run did not
    //     write — the user's edit stays their business, unstaged.
    let mut findings: Vec<Finding> = Vec::new();
    let guide_file = match profile.guide() {
        Some(guide) if guide_ownership(repo_root, guide) == GuideOwnership::UserModified => {
            findings.push(guide_modified_finding(&guide.file));
            None
        }
        Some(guide) => {
            write_guide_artifact(repo_root, guide).map_err(|err| {
                Finding::block(
                    "setup.write-guide",
                    format!(
                        "cannot write the adapter guide artifact `{}`: {err}",
                        guide.file
                    ),
                    format!(
                        "ensure `{}` is writable, then re-run `jigc setup`",
                        guide.file
                    ),
                )
            })?;
            Some(guide.file.clone())
        }
        None => None,
    };

    // 5. Install the assistant-neutral warn-only `pre-commit` hook (the auto-firing
    //    doc<->code backstop) into the repo's real hooks dir, pinned to the installing
    //    `jigc`'s own absolute path (the stale-binary hazard — `precommit_hook_body`).
    //    Regenerated each `setup`; idempotent + non-destructive.
    let jigc_path = std::env::current_exe().map_err(|err| {
        Finding::block(
            "setup.install-hook",
            format!(
                "cannot resolve the running `jigc` path to embed in the pre-commit hook: {err}"
            ),
            "re-run `jigc setup` (the install resolves its own absolute path)",
        )
    })?;
    // The **resolved** hook path is carried on (not just its display form): the install
    // commit's pathspec is derived from where the hook actually landed
    // ([`install_tracked_paths`]), which the printed value cannot answer.
    let hook_path = install_precommit_hook(repo_root, &jigc_path).map_err(|err| {
        Finding::block(
            "setup.install-hook",
            format!("cannot install the `pre-commit` hook into the repo's hooks dir: {err}"),
            "ensure the repo's git hooks directory is writable, then re-run `jigc setup`",
        )
    })?;
    let hook_file = display_hook_path(repo_root, &hook_path);

    // 6. Extract the embedded `doc-code` probe beside the installed `jigc` (the
    //    production resolution path `<jigc-bin-dir>/doc-code`), so a `cargo
    //    install`-style install gets a runnable probe with no manual copy. The
    //    **first machine-global** setup write; heal/upgrade policy — write if absent
    //    or if the existing sibling's bytes differ from the embedded copy, so a stale
    //    post-upgrade or corrupt sibling self-heals (`module-layout.md` → Probe
    //    distribution, M20).
    let bin_dir = jigc_path.parent().ok_or_else(|| {
        Finding::block(
            "setup.extract-probe",
            "cannot resolve the directory of the running `jigc` to place the `doc-code` probe"
                .to_string(),
            "re-run `jigc setup` from an installed `jigc` (the install resolves its own directory)",
        )
    })?;
    extract_doc_code_probe(bin_dir).map_err(|err| {
        Finding::block(
            "setup.extract-probe",
            format!("cannot write the `doc-code` probe beside `jigc` at `{}`: {err}", bin_dir.display()),
            format!(
                "ensure the directory holding the `jigc` binary (`{}`) is writable, then re-run `jigc setup`",
                bin_dir.display()
            ),
        )
    })?;

    // 7. Commit setup's own install files as a dedicated commit (M26 shakedown), so the
    //    user's first `jigc finalize` doesn't sweep the scaffolding into their first
    //    feature commit. Idempotent; benign skips (no repo / unborn HEAD / git absent)
    //    degrade gracefully — but a genuine *rejection* of either git step (e.g. no git
    //    identity) means the install is in no commit, so it fails loudly with a finding
    //    routed on git's own cause ([`InstallCommitRejection::finding`]) rather than
    //    masquerading as a clean success (mirrors `finalize`'s identical git-identity
    //    failure).
    let InstallCommitOutcome {
        commit: install_commit,
        hook_committed,
    } = commit_install(
        repo_root,
        &line_file,
        &allowlist_file,
        seeded_gitignore,
        &hook_path,
        guide_file.as_deref(),
    )
    .map_err(|rejection| rejection.finding())?;

    // 8. The forecast (M50 Increment 12 / T3, D5): the install is done — now say what the
    //    **next** door will refuse. `setup` is the one door that meets a repo whose project
    //    layer breaks pack-load and says nothing about it, so an adopter installs at exit 0
    //    and then meets a block on their next command with no idea the install had already
    //    seen it. Deliberately **after** every write and the install commit: this reports on
    //    the state the install leaves behind, and it must not be able to change it.
    //
    //    Never `?`-propagated — the declared bound (D5) is that the bootstrap door itself
    //    refuses nothing, so the probe's failure is an advisory on the existing `findings`
    //    key, never this function's `Err` arm.
    if let Err(err) = crate::pack::make_pack() {
        findings.push(pack_load_finding(&err));
    }

    Ok(SetupSummary {
        line_file,
        allowlist_file,
        hook_file,
        hook_committed,
        guide_file,
        findings: findings.into(),
        install_commit,
    })
}

/// The finding code the install's forecast raises. **Un-keyed** — not a `CHECK_INVENTORY`
/// row — so the engine's severity post-pass leaves it advisory, the same mold
/// [`GUIDE_MODIFIED_CODE`] rides.
pub const PACK_LOAD_CODE: &str = "setup.pack-load";

/// The **advisory** `jigc setup` raises when the pack set the repo resolves does not load
/// (M50 Increment 12 / T3; `design/project-setup.md` → What the install says about the corpus
/// it installed into; `design/corpus-migration.md` → The freeze).
///
/// **The subject is the pack LOAD, not the freeze.** The motivating instance is a
/// shape-changing project schema shadow, which the freeze refuses at every layer — but a
/// *malformed* shadow reaches the identical exit-0 silence through the loader, and the block
/// it produces at the next door carries no code and no route at all. So the forecast is keyed
/// on [`crate::pack::make_pack`] failing, whichever fence refused it, and it carries a route
/// of its own rather than relaying one that may not exist.
///
/// Advisory, never blocking, by decision: `setup` installs the adapter, it does not adjudicate
/// the corpus, and refusing to install over a drifted project layer is circular — the install
/// is what puts the tool in reach of repairing it. What the install owes the reader is
/// therefore the **consequence**: the state it just met is one every other door refuses over.
///
/// The cause is the pack-load error's **first line**, which in every shipped arm is the line
/// naming the offending file — relayed rather than re-composed, so the install and the door it
/// forecasts spell that path the same way on one screen (the reason
/// `crate::pack::assert_project_schema_shadows` is disposed `DeclaredAbsolute` in
/// `crates/cli/tests/repo_relative_paths.rs`). The freeze block's own trailing `route:` span is
/// **not** carried: two routes on one finding is the ambiguity the route floor exists to
/// forbid, and this finding's route names where to read that one in full. It says *any*
/// repair route rather than *the* one, because the malformed arm's block carries none —
/// promising a repair the next door does not print would be the law-1 break this task closes,
/// one door further along.
fn pack_load_finding(err: &anyhow::Error) -> Finding {
    let rendered = format!("{err:#}");
    let cause = rendered.lines().next().unwrap_or_default().trim_end();
    Finding::graded(
        Severity::Advisory,
        PACK_LOAD_CODE,
        format!(
            "the install completed, but this repo's pack set does not load, so every `jigc` \
             command that reads, composes or writes will refuse until it does: {cause}"
        ),
        None,
        Some(
            "fix or remove what the message names, then run `jigc validate` — it re-prints \
             that block in full, including any repair route the refusing fence carries, and \
             exits 0 once the pack set loads"
                .into(),
        ),
    )
}

/// The repo-relative install files `jigc setup` itself writes that are meant to be
/// tracked in git — the committable install footprint, enumerated **explicitly** so the
/// install commit never sweeps the user's unrelated working-tree changes (a blanket
/// `git add -A` would). Deliberately excludes: the transient `.jigc/` working area
/// (`tasks/`/`index/`/`state/`, gitignored by setup's own `.jigc/.gitignore`) and the
/// machine-global `doc-code` probe (beside the binary, not in the repo).
///
/// **The `pre-commit` hook is included exactly when it is a committable working-tree
/// file** — the rule, not the `.git/hooks` instance (M48 Increment 5 / F4). The list was
/// written on the premise *"the hook lives outside the worktree, in git's control dir —
/// never a tracked file"*, which is true of `.git/hooks/pre-commit` and **false under an
/// in-repo `core.hooksPath`**: there the hook is an ordinary working-tree file, so the
/// summary listed a file the install commit did not carry and left it untracked. The
/// premise is therefore replaced by [`committable_hook_path`]'s test, which asks git
/// where the hook landed rather than assuming — so the next hooks-path shape is decided
/// by the rule instead of re-opening the hole. The hook **file** is named, never the
/// hooks **dir**: a `core.hooksPath` directory may hold the user's own other hooks, and
/// `git add <dir>` would sweep them.
///
/// `hook` is [`committable_hook_path`]'s already-resolved answer rather than the hook
/// file, so [`commit_install`] holds the one entry it may have to **drop** (the
/// soft-member retry) without asking git the same question twice.
fn install_tracked_paths(
    line_file: &str,
    allowlist_file: &str,
    seeded_gitignore: bool,
    hook: Option<&str>,
    guide: Option<&str>,
) -> Vec<String> {
    let mut paths = vec![
        line_file.to_string(),      // CLAUDE.md (the bootstrap reference host)
        allowlist_file.to_string(), // .claude/settings.json (allowlist + SessionStart hook)
        ".jigc/AGENT.md".to_string(),
        ".jigc/.gitignore".to_string(),
        VERSION_STAMP_PATH.to_string(), // .jigc/version (the binary-provenance stamp)
        ".jigc/config/.gitkeep".to_string(),
        ".jigc/config/packs.yaml".to_string(),
    ];
    // The root `.gitignore` is committed **only** when setup itself seeded it on the
    // fresh-repo path — never an established repo's pre-existing `.gitignore` (which setup
    // did not touch and must not sweep into its install commit).
    if seeded_gitignore {
        paths.push(".gitignore".to_string());
    }
    // The adapter's owned **guide artifact**, when the profile declares one. It is
    // committed for the same reason the bootstrap file is: it must travel with the repo,
    // so a clone gets the guides that match the binary that wrote them (M48 Increment 10).
    if let Some(guide) = guide {
        paths.push(guide.to_string());
    }
    // The `pre-commit` hook, iff git can track it from this working tree.
    if let Some(hook) = hook {
        paths.push(hook.to_string());
    }
    paths
}

/// The installed `pre-commit` hook's repo-root-relative path when it is a **committable
/// working-tree file**, else `None` — the discriminator the install commit's pathspec is
/// derived from (M48 Increment 5 / F4).
///
/// **The rule now lives in one place** ([`crate::trackable::untrackable_reason`]) and is
/// asked here, not restated: under the canonicalized repo root, outside git's own dirs,
/// and owned by *this* repository rather than a submodule or embedded repo. It was written
/// for this door and generalized at M49, when the doc-relocating movers turned out to need
/// the same answer and to be losing committed documents for want of it — two copies of a
/// trackability rule is exactly how one of them ends up wrong.
///
/// Why the install commit needs it: the pathspec was first built on the premise *"the hook
/// lives in git's control dir — never a tracked file"*, which is false under an in-repo
/// `core.hooksPath`, so the summary listed a file the commit did not carry. Asking is what
/// keeps the next hooks-path shape from re-opening the hole.
///
/// The predicate cannot **promise** git will accept the path (a sparse-checkout excluding
/// the hooks dir refuses it while every test says "committable"), which is why
/// [`commit_install`] treats the hook as a **soft** member of the pathspec. This function
/// keeps the pathspec honest; it is not the only thing standing between an unusual hooks
/// dir and a failed install.
///
/// Deliberately *not* keyed on [`display_hook_path`]'s printed value: that renders the
/// default `.git/hooks/pre-commit` **relative** (it strips the canonicalized repo root,
/// and `.git/` is under it), so "the printed path is relative" is not a committability
/// test. Conservative on failure — a hook that does not canonicalize is not added.
fn committable_hook_path(repo_root: &Path, hook_file: &Path) -> Option<String> {
    let root = std::fs::canonicalize(repo_root).ok()?;
    let hook = std::fs::canonicalize(hook_file).ok()?;
    let relative = hook.strip_prefix(&root).ok()?.to_str()?.to_string();
    crate::trackable::untrackable_reason(repo_root, &relative)
        .is_none()
        .then_some(relative)
}

/// A git step of the install commit that **ran and refused** — the loud half of
/// [`commit_install`]'s contract, carrying git's own words and which step spoke them.
///
/// Both halves are here on purpose: a refusal on the **staging** half is as fatal to the
/// install commit as one on the commit half (nothing lands either way), and collapsing it
/// into `Ok(`[`InstallCommit::Skipped`]`)` is exactly how `jigc setup` once listed a full
/// install it had not committed. The variant decides what the finding may claim — a
/// refused `git add` staged nothing, so the "left staged for a re-run" recovery is true
/// only of [`Self::Commit`].
#[derive(Debug)]
enum InstallCommitRejection {
    /// `git add` refused: nothing was staged, no install commit was made.
    Stage(String),
    /// `git commit` refused: the install files are staged, so a re-run commits them.
    Commit(String),
}

impl InstallCommitRejection {
    /// The blocking finding this refusal surfaces as — message and route both derived
    /// from **git's actual rejection**, never from the one cause the route was first
    /// written for (surface-contract law 1: a route that names `user.email` when git
    /// refused a pathspec sends the reader to fix something that is not broken, and the
    /// re-run fails identically).
    fn finding(&self) -> Finding {
        let (step, git, staged) = match self {
            Self::Stage(git) => ("git add", git, false),
            Self::Commit(git) => ("git commit", git, true),
        };
        let message = if staged {
            format!(
                "the jigc install files were written and staged, but `{step}` was rejected \
                 (no install commit was made):\n{git}"
            )
        } else {
            format!(
                "the jigc install files were written, but `{step}` was rejected — nothing was \
                 added to the index and no install commit was made:\n{git}"
            )
        };
        let route = if is_git_identity_rejection(git) {
            "tell git who you are — set `git config user.email \"you@example.com\"` and \
             `git config user.name \"Your Name\"` — then re-run `jigc setup` to commit the \
             staged install files"
                .to_string()
        } else {
            format!(
                "resolve the refusal `{step}` reports above — it names the path or setting git \
                 declined — then re-run `jigc setup`"
            )
        };
        Finding::block("setup.install-commit", message, route)
    }
}

/// Whether git's rejection text is the **missing-identity** one — the single cause the
/// install-commit route was originally hardcoded to, now a test rather than an
/// assumption. Matches git's own wording on both arms it prints it with.
fn is_git_identity_rejection(git_err: &str) -> bool {
    let text = git_err.to_ascii_lowercase();
    text.contains("tell me who you are")
        || text.contains("unable to auto-detect email address")
        || text.contains("empty ident name")
}

/// Commit `jigc setup`'s own install files as a dedicated commit, so they don't land in
/// the user's first `jigc finalize` (M26 shakedown: the first finalize's `git add --all`
/// swept setup's scaffolding into the first feature commit). Stages **only** the files
/// setup itself wrote ([`install_tracked_paths`], filtered to those present and not
/// gitignored) and commits **only those paths** (a pathspec-limited commit), so any
/// unrelated changes the user already staged stay staged and untouched.
///
/// **Idempotent:** a re-run over an unchanged install stages no net change →
/// `Ok(`[`InstallCommit::Nothing`]`)` (no empty commit). On an **unborn HEAD** (a
/// brand-new repo with no commits) this **mints the repo's first commit** with the
/// install footprint rather than skipping (M30 audit finding 1 — setup owns committing
/// its own install regardless of HEAD state, since the first `finalize` now stages only
/// the task's change-set). **Graceful skip** for the benign cases — not-a-git-repo or a
/// git that could not be spawned → `Ok(`[`InstallCommit::Skipped`]`)` (nothing was
/// staged-but-orphaned; the commit is a convenience there). But a genuine **rejection**
/// — git ran and declined, on *either* the staging or the commit step (no
/// `user.email`/`user.name`, a held index lock, a pathspec this index cannot take) —
/// means the install is not in any commit, so it returns
/// `Err(`[`InstallCommitRejection`]`)` for [`install`] to surface as a loud blocking
/// finding rather than a silent skip behind a success banner.
///
/// **The `pre-commit` hook is a *soft* member of the pathspec.** Seven of the eight entries
/// are files setup wrote at paths setup chose; the eighth is the hook, whose home the
/// *user's* `core.hooksPath` chose, and no test [`committable_hook_path`] can run
/// *promises* git will accept it (a registered-but-absent submodule looked committable to
/// the filesystem until the index was asked; a sparse-checkout excluding the hooks dir
/// still refuses one that passes every test). So a `git add` refusal the hook entry is
/// responsible for costs **the hook's membership**, not the whole install commit: drop it
/// and ask git again. Attribution is **behavioural, never a string match on git's
/// message** — if the retry succeeds, the hook was the cause; if it refuses again, it was
/// not, and *that* refusal (over the reduced pathspec, naming what actually still blocks)
/// is the one surfaced. Falling back leaves the already-declared bound — a hook this repo
/// cannot track is installed, reported, and left as the containing repo's file — instead
/// of an install that cannot be completed at all.
///
/// Uses `--no-verify`: the
/// only hook present is the warn-only `pre-commit` setup just installed, and running the
/// doc↔code backstop against this commit is pointless (it carries install artifacts, not
/// managed docs) — and the hook must not self-trigger on the very commit that installs
/// it. (This is setup's install commit, distinct from `finalize`'s never-`--no-verify`
/// commit of managed work, which the user's hooks *are* policy for.)
fn commit_install(
    repo_root: &Path,
    line_file: &str,
    allowlist_file: &str,
    seeded_gitignore: bool,
    hook_file: &Path,
    guide_file: Option<&str>,
) -> Result<InstallCommitOutcome, InstallCommitRejection> {
    // Require a git work tree — but DO mint on an **unborn HEAD** (a brand-new repo with
    // no commits). Setup owns committing its own install footprint regardless of HEAD
    // state (M30 audit finding 1): on a cold-start repo the first `finalize` since M30
    // stages only the task's change-set ([`crate::task::stage_index_honoring`]), so if
    // setup skipped the install here, `CLAUDE.md`/`.claude/settings.json`/`.jigc/AGENT.md`
    // would be left untracked after the first managed commit. On an unborn HEAD the
    // `git add`/`git commit -- <paths>` below mint the repo's first commit (the staged
    // diff is taken against the empty tree). Skip only when there is no git work tree /
    // git is unavailable — the writes still succeeded; the commit is a convenience there.
    match git_output(repo_root, ["rev-parse", "--is-inside-work-tree"]) {
        Some(out) if out.status.success() => {}
        _ => return Ok(InstallCommitOutcome::uncommitted(InstallCommit::Skipped)),
    }

    // Only the files setup itself wrote, and only those present + not gitignored.
    let hook = committable_hook_path(repo_root, hook_file);
    let mut paths: Vec<String> = install_tracked_paths(
        line_file,
        allowlist_file,
        seeded_gitignore,
        hook.as_deref(),
        guide_file,
    )
    .into_iter()
    .filter(|p| repo_root.join(p).exists())
    .filter(|p| !git_path_ignored(repo_root, p))
    .collect();
    if paths.is_empty() {
        return Ok(InstallCommitOutcome::uncommitted(InstallCommit::Skipped));
    }

    // Stage exactly those paths — never a blanket `git add -A`.
    match stage_paths(repo_root, &paths) {
        StageOutcome::Staged => {}
        // git could not be spawned at all — benign skip (the writes still succeeded).
        StageOutcome::GitUnavailable => {
            return Ok(InstallCommitOutcome::uncommitted(InstallCommit::Skipped));
        }
        // git ran and REFUSED to stage. Retry without the hook — the one soft member —
        // before deciding the install commit is lost (see this function's doc comment);
        // an entry that is not there cannot be the cause, so a pathspec that never
        // carried the hook goes straight to the loud finding.
        StageOutcome::Refused(git) => {
            let dropped = hook.as_deref().filter(|h| paths.iter().any(|p| p == h));
            let Some(dropped) = dropped else {
                return Err(InstallCommitRejection::Stage(git));
            };
            paths.retain(|p| p != dropped);
            if paths.is_empty() {
                return Ok(InstallCommitOutcome::uncommitted(InstallCommit::Skipped));
            }
            match stage_paths(repo_root, &paths) {
                StageOutcome::Staged => {}
                StageOutcome::GitUnavailable => {
                    return Ok(InstallCommitOutcome::uncommitted(InstallCommit::Skipped));
                }
                // The hook was not the cause: surface the refusal over the reduced
                // pathspec, which names what actually still blocks.
                StageOutcome::Refused(git) => return Err(InstallCommitRejection::Stage(git)),
            }
        }
    }

    // Nothing staged among our paths (a re-run over an unchanged install) → clean no-op.
    // `git diff --cached --quiet -- <paths>` exits 0 (success) when there is no staged
    // diff for those paths; with no HEAD it diffs against the empty tree, so a first
    // install still reports changes.
    let mut diff: Vec<&str> = vec!["diff", "--cached", "--quiet", "--"];
    diff.extend(paths.iter().map(String::as_str));
    // From here on the pathspec is settled, so the hook's membership in it is the honest
    // answer to *"is the hook in the install commit?"* — including after the soft-member
    // drop above, which is exactly the case where every committability test said yes and
    // git said no.
    let hook_committed = hook
        .as_deref()
        .is_some_and(|h| paths.iter().any(|p| p == h));

    if git_output(repo_root, diff)
        .map(|o| o.status.success())
        .unwrap_or(false)
    {
        // A re-run over an unchanged install: no commit this time, but the hook's place in
        // the install commit is the one an earlier run gave it — the pathspec still says
        // where it stands, which is what the summary reports.
        return Ok(InstallCommitOutcome {
            commit: InstallCommit::Nothing,
            hook_committed,
        });
    }

    // Commit only our paths: a pathspec-limited commit commits exactly those files and
    // leaves the user's other staged changes uncommitted and untouched.
    let mut commit: Vec<&str> = vec!["commit", "--no-verify", "-m", INSTALL_COMMIT_MESSAGE, "--"];
    commit.extend(paths.iter().map(String::as_str));
    match git_output(repo_root, commit) {
        Some(out) if out.status.success() => {}
        // git ran and REJECTED the commit (e.g. no `user.email`/`user.name`). The files
        // are now staged-but-uncommitted — unlike the benign skips above, this must not
        // hide behind a success banner. Surface git's own rejection (its "tell me who you
        // are" guidance) for `install` to turn into a loud blocking finding.
        Some(out) => return Err(InstallCommitRejection::Commit(git_said(&out))),
        // git could not be spawned at all — benign skip (the writes still succeeded).
        None => return Ok(InstallCommitOutcome::uncommitted(InstallCommit::Skipped)),
    }

    // Resolve the short sha of the commit just made, for the success surface.
    let commit = match git_output(repo_root, ["rev-parse", "--short", "HEAD"]) {
        Some(out) if out.status.success() => {
            let sha = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if sha.is_empty() {
                InstallCommit::Skipped
            } else {
                InstallCommit::Committed(sha)
            }
        }
        _ => InstallCommit::Skipped,
    };
    // The commit was made whatever `rev-parse` then said, so the hook's membership stands
    // even when the sha could not be read back.
    Ok(InstallCommitOutcome {
        commit,
        hook_committed,
    })
}

/// What [`commit_install`] did: the commit outcome, and whether the installed `pre-commit`
/// hook was **in the pathspec that commit was made from**.
///
/// The two travel together because the summary must not say one thing while the commit
/// carries another (M48, the surface-fundament lens). The membership is read off the
/// settled pathspec rather than re-derived: [`committable_hook_path`] decides the entry,
/// the present/not-gitignored filters can still drop it, and the soft-member retry drops it
/// on a git refusal no test can predict — so the pathspec is the only place all three
/// answers have already been folded together.
#[derive(Debug)]
struct InstallCommitOutcome {
    /// The outcome of the commit itself.
    commit: InstallCommit,
    /// Whether the `pre-commit` hook rode it. `false` means the hook is installed and
    /// working **locally only** — in no commit, so no clone has it.
    hook_committed: bool,
}

impl InstallCommitOutcome {
    /// An outcome that carried no hook — every path that returns before a pathspec is
    /// settled (no work tree, git unavailable, nothing to commit).
    fn uncommitted(commit: InstallCommit) -> Self {
        Self {
            commit,
            hook_committed: false,
        }
    }
}

/// What one `git add -- <paths>` did, with git-could-not-be-spawned kept distinct from
/// git-ran-and-refused: the first is a benign skip, the second is fatal to the install
/// commit — and the two must not collapse (that collapse is how `jigc setup` once printed
/// a full success banner over an install it had not committed).
enum StageOutcome {
    /// git staged the pathspec.
    Staged,
    /// git could not be spawned at all — the writes still succeeded.
    GitUnavailable,
    /// git ran and refused, carrying its own words.
    Refused(String),
}

/// Stage exactly `paths` — never a blanket `git add -A`. Its own function because
/// [`commit_install`] runs it **twice** on the refusal path (the soft-member retry).
fn stage_paths(repo_root: &Path, paths: &[String]) -> StageOutcome {
    let mut add: Vec<&str> = vec!["add", "--"];
    add.extend(paths.iter().map(String::as_str));
    match git_output(repo_root, add) {
        Some(out) if out.status.success() => StageOutcome::Staged,
        Some(out) => StageOutcome::Refused(git_said(&out)),
        None => StageOutcome::GitUnavailable,
    }
}

/// What git said when it refused — stdout + stderr, trimmed, quoted verbatim into the
/// finding so the reader sees git's own words rather than a paraphrase of them.
fn git_said(out: &std::process::Output) -> String {
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    format!("{}{}", stdout.trim(), stderr.trim())
}

/// Whether `path` (repo-relative) is gitignored in `repo_root` (`git check-ignore -q`):
/// honors the requirement that the install commit never stage a gitignored path.
fn git_path_ignored(repo_root: &Path, path: &str) -> bool {
    git_output(repo_root, ["check-ignore", "-q", "--", path])
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Run `git -C <repo_root> <args>`, returning the captured output if git ran (whatever
/// its exit), or `None` if git could not be spawned. The install-commit path is
/// best-effort: a git hiccup degrades to [`InstallCommit::Skipped`], never a setup
/// failure. (Distinct from `task.rs`'s finalize git helpers, which commit the whole
/// index via `-F <msg>` and `bail!` on any failure — the wrong shape for a best-effort,
/// pathspec-limited install commit.)
fn git_output<I, S>(repo_root: &Path, args: I) -> Option<std::process::Output>
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    std::process::Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .args(args)
        .output()
        .ok()
}

/// The result of a `jigc uninstall` teardown: the located repo root, so the
/// dispatcher can render a precise summary of what was torn down.
#[derive(Debug)]
pub struct UninstallSummary {
    /// The repo-root-relative always-loaded file the bootstrap reference was
    /// unwired from.
    pub line_file: String,
    /// The repo-root-relative settings file the allowlist permit was removed from.
    pub allowlist_file: String,
    /// Which of the seven repo-local artifacts were **actually** present and removed —
    /// so the teardown summary reports the real removal set and never claims to have
    /// removed an already-absent artifact (M36 completion honesty fix; the runtime
    /// mirror of the `project-setup.md` G5 "exactly the enumerated set" correction).
    pub removed: RemovedArtifacts,
    /// The teardown's **non-blocking** findings — what it declined to remove and why (today:
    /// a user-modified guide artifact, [`guide_kept_finding`]). Empty on an ordinary
    /// teardown. A blocking outcome is not here: it is [`uninstall`]'s `Err` arm, which
    /// removes nothing at all (M48 Increment 10 / T3).
    pub findings: engine::finding::Findings,
}

/// The per-artifact removal ledger [`uninstall`] fills — one flag per repo-local
/// `setup` artifact, `true` iff that artifact was present and this run removed it. An
/// all-`false` ledger is an idempotent no-op teardown (a second `uninstall`).
#[derive(Debug, Default)]
pub struct RemovedArtifacts {
    /// The `.jigc/` tree was present and removed.
    pub jigc_dir: bool,
    /// A jigc-injected bootstrap reference was unwired from the always-loaded file.
    pub reference: bool,
    /// The `Bash(jigc:*)` allowlist permit was dropped from the settings file.
    pub allowlist: bool,
    /// jigc's `SessionStart` hook command was dropped from the settings file.
    pub hook: bool,
    /// The `deny` safety floor was dropped from the settings file.
    pub deny: bool,
    /// The jigc-managed `pre-commit` hook was removed or unwrapped.
    pub precommit: bool,
    /// The adapter's owned guide artifact was present, **still jigc's own** (or `--force`
    /// was given), and removed. `false` also covers the artifact this teardown deliberately
    /// left standing — a user-modified copy — which the summary reports as a finding rather
    /// than as a removal it did not make (M48 Increment 10).
    pub guide: bool,
}

impl RemovedArtifacts {
    /// Whether this teardown removed nothing — an idempotent no-op (every artifact was
    /// already absent), so the summary reports a clean "nothing to remove" state rather
    /// than claiming removals it did not make.
    pub fn is_empty(&self) -> bool {
        !(self.jigc_dir
            || self.reference
            || self.allowlist
            || self.hook
            || self.deny
            || self.precommit
            || self.guide)
    }
}

/// Run `jigc uninstall` from `start`: locate the repo root and reverse the **complete**
/// repo-local `setup`-created set — remove `.jigc/` (which subsumes the bootstrap
/// `AGENT.md`, the cascade config layer, the `compose-embedded-methodology` marker, and
/// the index/state working area), unwire the `CLAUDE.md` `## Project interface` section
/// and its `@.jigc/AGENT.md` import line, remove **all three** `.claude/settings.json`
/// writes — the `Bash(jigc:*)` permit (`permissions.allow`), the `SessionStart` hook
/// (`hooks.SessionStart`), and the `deny` safety floor (`permissions.deny`) — and prune
/// the git `pre-commit` hook (the M36 symmetry fix: both hooks must come out, or they
/// fire against a removed install; `design/project-setup.md` → Flow 2 hardening →
/// Teardown / cleanup (G5), bullet (b)) — **and the adapter's owned guide artifact**, the
/// seventh member, because the set is what `setup` writes rather than a fixed list (M48
/// Increment 10). Every settings removal is **surgical**: a foreign permit / hook / deny
/// entry sharing the file survives.
///
/// **The guide artifact comes out only while it is still jigc's** ([`guide_ownership`]): a
/// copy the user has edited is left byte-identical and reported as the
/// [`guide_kept_finding`] advisory, since deleting authored bytes at exit 0 is the class
/// the guards below close. `force` removes it either way.
///
/// **Explicitly NOT** the machine-global `doc-code` probe sibling beside the `jigc`
/// binary — it is shared across every jigc repo on the machine, so deleting it would
/// break `jigc validate` for sibling repos (design-review B2). Machine-global removal
/// is `cargo uninstall jigc` + manual probe removal, never this per-project verb.
///
/// **It refuses over the three things inside `.jigc/` that live nowhere else**, before it
/// removes anything (`DECISIONS.md` 2026-08-13 → the Settle, F3; 2026-09-05 → M50 Inc 4 /
/// T4):
///
/// - a **worktree-shaped path under `.jigc/worktrees/` holding content** blocks with
///   `uninstall.dirty-worktree` ([`dirty_fanout_worktrees`]). Since M47 Inc 3 a live
///   worktree holding uncommitted code is a **normal, promised-safe** state (the aborted
///   fan-out finalize leaves it alive for the retry), and the subject is the *path*, not
///   the registered set — a copied or moved repo's worktrees are registered at the
///   *source's* path, so a registered-set guard is inert exactly where the live work is;
/// - an **open task's staged doc** in `.jigc/tasks/<id>/docs/*.md` blocks with
///   `uninstall.staged-prose` ([`crate::task::staged_task_prose`]) — bytes that are in no
///   object DB at all (the reproduced pre-1.0.0 loss authored them; the mint's own skeleton
///   is refused on the same footing, since neither is provably disposable);
/// - **any other file under `.jigc/` that no index has a copy of** — the `ENTRIES`
///   complement ([`workbench_paths`]) — blocks with
///   `uninstall.untracked-workbench-file`. The same ground as the two above, stated over
///   the rest of the tree: it is an **added** third subject, and the two directories those
///   guards own are excluded from it by construction so their codes and routes keep
///   answering for them. In the **index** is the line, not committed — a `git add`-ed file
///   is `git checkout`-recoverable, so it is narrated rather than refused. The line is
///   drawn on the **bytes**, not on the path: a tracked file the operator has edited
///   without staging is listed in the index carrying the *old* content, so it refuses
///   here alongside the never-tracked ones ([`classify_workbench_paths`]).
///
/// `force` is the operator's consent to delete. It skips the three guards, and — the one
/// other thing this teardown refuses on its own — takes the adapter's owned guide artifact
/// even when the user has edited it.
///
/// **Idempotent:** each step is independently a clean no-op when its artifact is already
/// absent — an already-removed `.jigc/`, a `CLAUDE.md` without the section, an
/// `allow`/`deny` array or `hooks` object without the jigc entry, an absent-or-foreign
/// `pre-commit` hook, and an absent guide artifact — so a second `uninstall` exits 0
/// leaving the (restored) host files
/// byte-untouched. `Ok(summary)` on a clean teardown; `Err(finding)` is a single blocking
/// `uninstall.*` finding carrying a route — the dispatcher renders it and exits non-zero.
pub fn run_uninstall(start: &Path, force: bool) -> Result<UninstallSummary, Finding> {
    // The shared not-in-a-repository answer, exactly as `setup`'s door gives it.
    let ctx = locate::locate(start).map_err(|err| {
        locate::locate_finding(
            "uninstall.repo-root",
            start,
            &err,
            "run `jigc uninstall` from inside the target git repository",
        )
    })?;

    let profile = adapter::load_profile(SETUP_ASSISTANT).map_err(|err| {
        Finding::block(
            "uninstall.profile-load",
            format!("cannot load the `{SETUP_ASSISTANT}` adapter profile: {err}"),
            "reinstall jigc — the embedded adapter profile is missing or malformed",
        )
    })?;

    uninstall(&ctx.repo_root, &profile, force)
}

/// Reverse the repo-local install against `repo_root` with `profile`, mapping an IO
/// failure to a blocking `uninstall.*` finding with a route. The testable core of
/// [`run_uninstall`] (no location step). Each step is independently idempotent, so the
/// whole teardown is a clean no-op on a re-run — but unless `force`, it removes nothing at
/// all while `.jigc/` holds the sole copy of anything: a worktree-shaped path with content
/// ([`dirty_fanout_worktrees`]), an open task's staged docs
/// ([`crate::task::staged_task_prose`]), or any other workbench file no index has a copy of
/// ([`untracked_workbench_files`]) — all three probed in step 0.
fn uninstall(
    repo_root: &Path,
    profile: &AdapterProfile,
    force: bool,
) -> Result<UninstallSummary, Finding> {
    // 0. The WIP guards, BEFORE anything is removed — everything below is `remove_dir_all`
    //    on a tree that holds the sole copy of two kinds of work.
    if !force {
        let dirty = dirty_fanout_worktrees(repo_root)?;
        if !dirty.is_empty() {
            return Err(dirty_worktree_finding(repo_root, &dirty));
        }
        let staged = crate::task::staged_task_prose(
            repo_root,
            None,
            &crate::task::unverified_prose_finding,
        )?;
        if !staged.is_empty() {
            return Err(staged_prose_finding(&staged));
        }
        // The third subject: everything else under `.jigc/` that no index has a copy of
        // — the two guards above stated over the rest of the tree
        // ([`workbench_paths`]). It runs LAST so a corpus holding both a sole-copy
        // worktree and an uncommitted config delta is still answered by the door that
        // owns the sole copy.
        let untracked = untracked_workbench_files(repo_root)?;
        if !untracked.is_empty() {
            return Err(untracked_workbench_finding(&untracked));
        }
    }

    // 1. Remove the whole `.jigc/` tree — the bootstrap `AGENT.md`, the cascade config
    //    layer, the compose marker, and the transient index/state working area, all at
    //    once. An already-absent tree is a clean no-op.
    let mut removed = RemovedArtifacts::default();

    let jigc_dir = repo_root.join(".jigc");
    if jigc_dir.exists() {
        // Name the loss BEFORE the removal ([`narrate_teardown`], law 1) — the guards above
        // either cleared this tree or `force` consented past them, and neither is a reason to
        // destroy bytes in silence.
        narrate_teardown(repo_root);
        std::fs::remove_dir_all(&jigc_dir).map_err(|err| {
            Finding::block(
                "uninstall.remove-jigc",
                format!("cannot remove the repo-local `.jigc/` tree: {err}"),
                "ensure `.jigc/` is writable, then re-run `jigc uninstall`",
            )
        })?;
        removed.jigc_dir = true;
    }

    // 2. Unwire the `CLAUDE.md` bootstrap reference — strip jigc's appended
    //    `## Project interface` section, restoring the human's content byte-for-byte.
    let line_file = profile
        .reference()
        .map(|r| r.file.clone())
        .unwrap_or_else(|| "CLAUDE.md".to_string());
    removed.reference = adapter::unwire_reference(repo_root).map_err(|err| {
        Finding::block(
            "uninstall.unwire-reference",
            format!("cannot unwire the bootstrap reference from `{line_file}`: {err}"),
            format!("ensure `{line_file}` is writable, then re-run `jigc uninstall`"),
        )
    })?;

    // 3. Remove the `Bash(jigc:*)` permit from the allowlist — leaving unrelated permits and
    //    keys intact and the file valid JSON.
    let allowlist_file = profile.allowlist.file.clone();
    removed.allowlist = adapter::remove_allowlist(repo_root, profile).map_err(|err| {
        Finding::block(
            "uninstall.remove-allowlist",
            format!("cannot remove the allowlist permit from `{allowlist_file}`: {err}"),
            format!("ensure `{allowlist_file}` is writable, then re-run `jigc uninstall`"),
        )
    })?;

    // 4. Remove the `SessionStart` hook from the same settings file — surgically, so a
    //    foreign hook sharing the `hooks` object survives (the M36 symmetry fix: both
    //    hooks must come out, or they fire against a removed install).
    removed.hook = adapter::remove_hook(repo_root, profile).map_err(|err| {
        Finding::block(
            "uninstall.remove-hook",
            format!("cannot remove the session hook from `{allowlist_file}`: {err}"),
            format!("ensure `{allowlist_file}` is writable, then re-run `jigc uninstall`"),
        )
    })?;

    // 5. Remove the `deny` safety floor from the same settings file — dropping only the
    //    profile's floor patterns, preserving any user `deny` entry.
    removed.deny = adapter::remove_deny(repo_root, profile).map_err(|err| {
        Finding::block(
            "uninstall.remove-deny",
            format!("cannot remove the deny safety floor from `{allowlist_file}`: {err}"),
            format!("ensure `{allowlist_file}` is writable, then re-run `jigc uninstall`"),
        )
    })?;

    // 6. Remove the `pre-commit` hook — a standalone jigc hook is deleted; a foreign
    //    hook setup wrapped is restored (only the jigc block is pruned).
    removed.precommit = remove_precommit_hook(repo_root).map_err(|err| {
        Finding::block(
            "uninstall.remove-precommit",
            format!("cannot remove the `pre-commit` hook from the repo's hooks dir: {err}"),
            "ensure the repo's git hooks directory is writable, then re-run `jigc uninstall`",
        )
    })?;

    // 7. Remove the adapter's **owned guide artifact** — the seventh repo-local file
    //    `setup` writes, and an ordinary member of this set because the set is *what setup
    //    wrote*, not a fixed list (`design/project-setup.md` → Teardown / cleanup (G5)).
    //
    //    Removed on the **same ownership question the writing door asks**: jigc takes back
    //    what jigc wrote ([`GuideOwnership::Owned`]) and leaves a copy the user has edited
    //    standing, reporting it rather than deleting it — a door that destroys authored
    //    bytes at exit 0 is precisely the class step 0's guards close, and this file is not
    //    exempt from it. `force` is the operator's consent, the same flag that consents to
    //    those two, so it removes the artifact whoever wrote it.
    //
    //    An invalid guide target is skipped: `install` gates the same target before any
    //    write ([`adapter::validate_guide_target`]), so nothing jigc wrote can be sitting
    //    behind one — and a teardown must not follow a path the install refused to take.
    let mut findings: Vec<Finding> = Vec::new();
    if let Some(guide) = profile
        .guide()
        .filter(|guide| adapter::validate_guide_target(guide).is_ok())
    {
        let target = repo_root.join(&guide.file);
        if target.is_file() {
            if force || guide_ownership(repo_root, guide) == GuideOwnership::Owned {
                std::fs::remove_file(&target).map_err(|err| {
                    Finding::block(
                        "uninstall.remove-guide",
                        format!(
                            "cannot remove the adapter guide artifact `{}`: {err}",
                            guide.file
                        ),
                        format!(
                            "ensure `{}` is writable, then re-run `jigc uninstall`",
                            guide.file
                        ),
                    )
                })?;
                // The directories jigc created to hold it go with it once they empty —
                // but never one that houses another host file (`.claude/` holds the
                // settings file this teardown just edited *surgically*).
                let keep: Vec<PathBuf> = [line_file.as_str(), allowlist_file.as_str()]
                    .iter()
                    .filter_map(|file| repo_root.join(file).parent().map(Path::to_path_buf))
                    .collect();
                prune_empty_dirs(repo_root, &target, &keep);
                removed.guide = true;
            } else {
                findings.push(guide_kept_finding(&guide.file));
            }
        }
    }

    // The machine-global `doc-code` probe sibling is deliberately left in place (B2):
    // it is shared across every jigc repo on the machine, so this per-project verb must
    // not delete it.

    Ok(UninstallSummary {
        line_file,
        allowlist_file,
        removed,
        findings: findings.into(),
    })
}

/// Remove the now-empty directories that held `artifact`, walking outward from its own
/// parent and stopping at the first directory that is **not** jigc's to take: one that still
/// holds something, one that houses another of the profile's host files (`keep`), the repo
/// root, or anything outside it. `setup` creates this chain for the artifact
/// ([`write_guide_artifact`]); leaving an empty `.claude/skills/jigc/` behind would leave the
/// teardown visibly unfinished.
///
/// **Best-effort by design.** The artifact is what the teardown promised to remove; an empty
/// directory that resists removal is residue, not a failed teardown, so a stubborn `rmdir`
/// ends the walk instead of failing the verb (and instead of a finding: an empty directory
/// is not a fact a user needs routed).
fn prune_empty_dirs(repo_root: &Path, artifact: &Path, keep: &[PathBuf]) {
    let mut dir = artifact.parent();
    while let Some(current) = dir {
        if current == repo_root
            || !current.starts_with(repo_root)
            || keep.iter().any(|kept| kept == current)
        {
            return;
        }
        let empty = std::fs::read_dir(current).is_ok_and(|mut entries| entries.next().is_none());
        if !empty || std::fs::remove_dir(current).is_err() {
            return;
        }
        dir = current.parent();
    }
}

/// The worktree-shaped paths under `<repo_root>/.jigc/worktrees/` that hold content the
/// teardown must not take, each paired with what would be destroyed — [`uninstall`]'s
/// fan-out WIP guard, over the same [`crate::milestone::probe_leftover`] classifier
/// `jigc milestone provision` and `jigc milestone discard` ask (`design/team-ready-state.md`
/// → Abandon refuses on a dirty worktree).
///
/// **The subject is the path, not the registered set** (`DECISIONS.md` 2026-08-13 → the
/// Settle, F3). A `cp -R` or `mv` of the repo — how every trial corpus is made — leaves the
/// copy's live worktrees registered at the **source's** path, so *no* path under the copy's
/// own `.jigc/worktrees/` is registered and a registered-set guard is inert precisely where
/// the live work is; `remove_dir_all` then took the lot at exit 0. So every directory under
/// that root is classified instead: a live worktree of its own answers through the shipped
/// `git status --porcelain` probe, and a path git cannot vouch for refuses on any content.
///
/// **Why `uninstall` needs it.** The teardown's first step is
/// `remove_dir_all(<repo>/.jigc)`, and since M31 Inc 4/5 the fan-out worktrees live
/// **inside** that tree, each the *sole copy* of a sub-agent's code. M47 Inc 3 made
/// "a provisioned worktree holding uncommitted work" a **normal, promised-safe** state —
/// an aborted fan-out finalize deliberately leaves the worktrees alive so the re-run can
/// recover them — so an unguarded removal destroyed exactly the work the tool had just
/// promised to keep (plus the sub-tasks' staged docs in `.jigc/tasks/<id>/docs/`, which are
/// in no object DB at all), at exit 0.
///
/// **Scoped to `.jigc/worktrees/`**: only paths under that root are probed — a human's
/// worktree elsewhere in the repo is none of this verb's business, and the main checkout is
/// never under that root. An absent worktrees dir short-circuits before any `git` call, so
/// the no-fan-out teardown (and the idempotent second run over an already-removed `.jigc/`)
/// pays nothing.
///
/// **The probe fails closed**, under the same `uninstall.dirty-worktree` code: an
/// unreadable directory or `git status` leaves the worktrees' safety *unknown*, and the
/// operator's action is the same either way — make the fan-out worktrees safe, then re-run.
/// Removing on an unverified probe is the very defect this guard closes.
///
/// **And it fails closed for that path alone** (M50 Increment 12 / T2; RC-m50 N9). The probe's
/// failure used to propagate with `?`, so an unreadable path ended the walk: over a directory
/// holding work **and** a leftover file, this door named the file, dropped the directory's
/// good refusal on the floor, and printed a route about `git` being on PATH. The failure is a
/// [`crate::milestone::LeftoverShape::Unreadable`] hold now, collected with its siblings, so
/// the refusal names every path it would take.
///
/// **Each hold also carries whether *this* repo registered the path** — the fact the
/// refusal's route rests on, not decoration. Widening the *subject* to the path widened the
/// guard's domain past what its response was written for: the milestone teardown removes
/// **registered** worktrees and skips everything else (`crate::milestone::remove_worktrees`'
/// contract, stated verbatim by `discard`'s own refusal), so on a path registered nowhere the
/// abandon arm the route used to name unconditionally sends the operator through an
/// irreversible, committed `discard --force` that clears nothing. Only the registered set can
/// tell the two apart, so it is read here and answered in [`dirty_worktree_finding`].
fn dirty_fanout_worktrees(repo_root: &Path) -> Result<Vec<HeldWorktreePath>, Finding> {
    let paths = fanout_worktree_paths(repo_root)
        .map_err(|err| unverified_worktrees_finding(anyhow::Error::new(err)))?;

    let mut holds: Vec<HeldWorktreePath> = Vec::new();
    for path in paths {
        if let Some(hold) = crate::milestone::probe_leftover(repo_root, &path) {
            holds.push(HeldWorktreePath {
                path,
                hold,
                registered: None,
            });
        }
    }
    if holds.is_empty() {
        // The clean teardown pays for no extra `git` call — nothing is going to be routed.
        return Ok(holds);
    }
    // git stores canonical paths at `worktree add` time, so both sides canonicalize (the
    // `held_subtask_worktrees` convention: on macOS `/tmp/…` lists as `/private/tmp/…`).
    if let Ok(registered) = crate::milestone::registered_worktrees(repo_root) {
        let registered: Vec<PathBuf> = registered
            .into_iter()
            .map(|w| w.canonicalize().unwrap_or(w))
            .collect();
        for hold in &mut holds {
            let canonical = hold
                .path
                .canonicalize()
                .unwrap_or_else(|_| hold.path.clone());
            hold.registered = Some(registered.iter().any(|w| w == &canonical));
        }
    }
    Ok(holds)
}

/// Every worktree-shaped path under `<repo>/.jigc/worktrees/` — the subject both this
/// door's refusal ([`dirty_fanout_worktrees`]) and its loss narration ([`narrate_teardown`])
/// work over, enumerated once so the two cannot drift into disagreeing about which paths the
/// teardown takes.
///
/// **Sorted**, so neither surface varies with readdir order. An absent root is the empty set,
/// so the no-fan-out teardown (and the idempotent second run over an already-removed
/// `.jigc/`) short-circuits before any `git` call.
///
/// **Every child, not only the directories** (M49). The subject was `path.is_dir()`, which is
/// a claim about *shape* where the door's question is about *bytes*: `remove_dir_all(.jigc/)`
/// takes a plain file under `.jigc/worktrees/` exactly as hard as a worktree, and the filter
/// made that file invisible to both surfaces at once — the guard never probed it, so
/// `jigc uninstall` destroyed it at **exit 0**, and the narration never named it either. The
/// path's shape is [`crate::milestone::probe_leftover`]'s question to answer (fail-closed: a
/// path it cannot read refuses), never a reason to drop it from the set.
fn fanout_worktree_paths(repo_root: &Path) -> std::io::Result<Vec<PathBuf>> {
    let worktrees_root = repo_root.join(".jigc").join("worktrees");
    if !worktrees_root.is_dir() {
        return Ok(Vec::new());
    }
    let mut paths: Vec<PathBuf> = Vec::new();
    for entry in std::fs::read_dir(&worktrees_root)? {
        paths.push(entry?.path());
    }
    paths.sort();
    Ok(paths)
}

/// Every path under `<repo>/.jigc/` that lies **outside** the transient
/// [`crate::gitignore::ENTRIES`] prefixes — repo-relative, `/`-separated and sorted.
///
/// This is the subject of the teardown's **third** guard
/// ([`untracked_workbench_files`]) and of its third narration line, enumerated once so
/// the two cannot drift into disagreeing about which paths the teardown takes — the
/// [`fanout_worktree_paths`] convention, applied to the other half of the workbench.
///
/// **It is a derivation, not a registry.** Membership is a path computation over the
/// seven `ENTRIES` prefixes plus (in the caller) one `git` query; nothing enumerates the
/// files themselves. A prefix added to `ENTRIES` narrows this set automatically, which is
/// the point of asking that constant rather than re-listing it here.
///
/// **The `ENTRIES` complement, not "everything untracked under `.jigc/`."** `tasks/` and
/// `worktrees/` are *inside* `ENTRIES`, and both hold bytes no index has a copy of by
/// design — the sole-copy state M46 Inc 2, M47 Inc 3 and M49 built the other two guards
/// for. Swallowing them here would replace those guards with one that names the wrong
/// subject and prints the wrong route, so they are excluded by construction and answered
/// by the doors that own them.
///
/// **`displaced/` is the one excluded prefix no door owns** — stated here rather than left
/// to be rediscovered (M50 Increment 4 validation, N8). It is in `ENTRIES`, so the relocation
/// workbench sits outside this subject exactly as `tasks/` and `worktrees/` do; unlike them,
/// nothing else refuses or narrates over it, so a file parked there by
/// [`crate::relocate::relocate_stranded`] is taken by the teardown at exit 0 and named by
/// nothing. The re-point that could park a **managed committed** doc there is closed at its
/// own door ([`crate::config`]'s root-value fold, M50 Inc 4 validation), leaving reachable
/// only a *foreign* file whose displacement was printed when it happened — narrow enough to
/// declare rather than guard, and declared so the next reader need not derive the gap again.
///
/// **Every child that is not a directory**, symlinks included (M49's lesson at
/// [`fanout_worktree_paths`]): `remove_dir_all(.jigc/)` takes them all, so the shape of a
/// path is a reason to recurse into it, never a reason to drop it from the set. Recursion
/// is decided on `symlink_metadata`, so a symlink is a leaf rather than a door out of the
/// tree.
fn workbench_paths(repo_root: &Path) -> std::io::Result<Vec<String>> {
    let jigc_dir = repo_root.join(".jigc");
    if !jigc_dir.is_dir() {
        return Ok(Vec::new());
    }
    let transient: Vec<&str> = crate::gitignore::ENTRIES
        .lines()
        .map(|entry| entry.trim_end_matches('/'))
        .collect();

    let mut stack: Vec<PathBuf> = Vec::new();
    for entry in std::fs::read_dir(&jigc_dir)? {
        let entry = entry?;
        if transient
            .iter()
            .any(|prefix| entry.file_name() == std::ffi::OsStr::new(prefix))
        {
            continue;
        }
        stack.push(entry.path());
    }

    let mut paths: Vec<String> = Vec::new();
    while let Some(path) = stack.pop() {
        if std::fs::symlink_metadata(&path)?.is_dir() {
            for entry in std::fs::read_dir(&path)? {
                stack.push(entry?.path());
            }
            continue;
        }
        paths.push(crate::render::repo_relative(repo_root, &path));
    }
    paths.sort();
    Ok(paths)
}

/// The workbench paths split on the one question both shipped guards already rest on:
/// **does any index have a copy of these bytes?** Returns `(untracked, tracked)`, each
/// sorted.
///
/// **The question is bytes, not path membership** — the distinction M50's own validation
/// caught this classifier collapsing. `git ls-files --cached` answers *"is this path in
/// the index?"*, which is a strictly weaker question: a tracked file the operator has
/// edited without staging is listed there, yet the index carries the **old** bytes and
/// none of the new ones, so the `git checkout -- <path>` this split licenses does not
/// bring the edit back — it throws it away. `.jigc/config/packs.yaml` is the ordinary
/// cell: `setup` tracks it, a human hand-edits it, and before this the teardown destroyed
/// that edit at exit 0 while printing that it was restorable.
///
/// So membership is the first leg and **index-copy-equals-working-copy is the second**:
/// `git status --porcelain` names every path under `.jigc/` whose *worktree* column is not
/// clean — its working-tree content (or mode, which `git checkout` restores from the index
/// too) differs from its index entry — and those join the never-tracked half. Both legs are
/// the same predicate said once: *`git checkout -- <path>` reproduces this file*.
///
/// **`status`, not `diff-files`, because only `status` compares bytes.** `git diff-files`
/// is stat-based: it reports every entry whose cached `stat` is stale as modified without
/// opening it, so a corpus that was *copied* — a restored backup, a `cp -R`, this repo's
/// own [`crate::gitignore`]-era fixture copies — would refuse a teardown over files whose
/// content the index carries exactly. `status` refreshes before it answers, and
/// `--no-optional-locks` keeps that refresh out of the on-disk index, so a probe run by a
/// door that may yet refuse writes nothing.
///
/// **In the index is the line, not committed.** `git ls-files --cached` lists a staged
/// add, and a staged add is `git checkout -- <path>`-recoverable after the teardown has
/// taken the working-tree copy — so it is narrated, not refused; the same holds for an
/// edit that *has* been staged, whose bytes the index now carries. What no index carries
/// is gone for good, which is exactly the ground `uninstall.dirty-worktree` and
/// `uninstall.staged-prose` refuse on; stating it over the third subject makes the two
/// shipped guards' grounds one ground said three times rather than three rules.
///
/// Two `git` calls for the whole tree — the clean teardown of a repo with no `.jigc/`
/// short-circuits in [`workbench_paths`] before either.
fn classify_workbench_paths(repo_root: &Path) -> anyhow::Result<(Vec<String>, Vec<String>)> {
    let paths = workbench_paths(repo_root)?;
    if paths.is_empty() {
        return Ok((Vec::new(), Vec::new()));
    }
    let listed = crate::task::git_capture(
        repo_root,
        &["ls-files", "-z", "--cached", "--full-name", "--", ".jigc"],
    )?;
    let cached: Vec<&str> = listed.split('\0').filter(|s| !s.is_empty()).collect();
    // The second leg: of the paths the index *lists*, the ones whose bytes it does not
    // carry. Porcelain v1 `-z` records are `XY <path>\0`; the worktree column is `Y`, and
    // `--no-renames` keeps every record to one path so the offset is fixed.
    let status = git_capture_untrimmed(
        repo_root,
        &[
            "--no-optional-locks",
            "status",
            "--porcelain",
            "-z",
            "--no-renames",
            "--",
            ".jigc",
        ],
    )?;
    let modified: Vec<&str> = status
        .split('\0')
        .filter(|record| record.len() > 3 && record.as_bytes()[1] != b' ')
        .map(|record| &record[3..])
        .collect();
    let (tracked, untracked): (Vec<String>, Vec<String>) = paths.into_iter().partition(|path| {
        cached.iter().any(|entry| entry == path) && !modified.iter().any(|entry| entry == path)
    });
    Ok((untracked, tracked))
}

/// Run `git <args>` in `repo_root` and return stdout **verbatim**, bailing on a spawn
/// failure or a non-zero exit — [`crate::task::git_capture`]'s fail-closed shape without
/// its `trim()`.
///
/// The trim is why this exists. A `git status --porcelain -z` record is `XY <path>\0`, and
/// `X` is a **space** for the ordinary unstaged edit — so trimming the capture eats the
/// first record's index column and shifts every offset in it by one, silently reading the
/// path as one byte short. Positional parsing and a trimming capture cannot both be right.
fn git_capture_untrimmed(repo_root: &Path, args: &[&str]) -> anyhow::Result<String> {
    use anyhow::Context;
    let out = std::process::Command::new("git")
        .args(args)
        .current_dir(repo_root)
        .output()
        .context("could not run `git` (is it on PATH?)")?;
    if !out.status.success() {
        anyhow::bail!(
            "`git {}` failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    String::from_utf8(out.stdout).context("`git` produced non-UTF-8 output")
}

/// The teardown's **third** guard: workbench bytes no index has a copy of
/// ([`classify_workbench_paths`]), refused before `remove_dir_all(<repo>/.jigc)` runs.
///
/// **Added, never substituted** — see [`workbench_paths`] for why the subject is the
/// `ENTRIES` complement.
///
/// **Fails closed**, under the same code: an unreadable workbench or an unrunnable `git`
/// leaves the recoverability of those bytes *unknown*, and removing on an unverified
/// probe is the defect this guard closes.
fn untracked_workbench_files(repo_root: &Path) -> Result<Vec<String>, Finding> {
    classify_workbench_paths(repo_root)
        .map(|(untracked, _)| untracked)
        .map_err(unverified_workbench_finding)
}

/// The third subject's refusal: a blocking, route-bearing finding naming every workbench
/// path no index has a copy of.
///
/// **The route names the cheap exit first.** `git add <path>` is enough — the guard's
/// question is the index, not `HEAD` — so the operator is not told to commit bytes they
/// may not want in history; deleting what they do not need is the other exit, and
/// `--force` is the consent that proceeds anyway, the single consent every destroying
/// door takes.
fn untracked_workbench_finding(untracked: &[String]) -> Finding {
    let listing: Vec<String> = untracked.iter().map(|path| format!("  {path}")).collect();
    Finding::block(
        "uninstall.untracked-workbench-file",
        format!(
            "`.jigc/` holds {} file(s) that no index has a copy of — removing `.jigc/` would \
             destroy them:\n{}",
            untracked.len(),
            listing.join("\n"),
        ),
        "put them where they can be recovered (`git add <path>` is enough — the index keeps a \
         copy `git checkout -- <path>` restores) or delete the ones you do not need, then \
         re-run `jigc uninstall`; `jigc uninstall --force` deletes them with the install",
    )
}

/// The fail-closed half of [`untracked_workbench_files`]: the probe could not run, so the
/// teardown refuses rather than remove `.jigc/` with those bytes' recoverability unknown.
/// Same code as the refusal — the operator's next action is identical.
fn unverified_workbench_finding(err: anyhow::Error) -> Finding {
    Finding::block(
        "uninstall.untracked-workbench-file",
        format!(
            "cannot check `.jigc/` for files no index has a copy of, so removing it could \
             destroy them: {err:#}"
        ),
        "make sure `git` is on PATH and the `.jigc/` tree is readable, then re-run \
         `jigc uninstall` — or, once you have confirmed it holds nothing you need, \
         `jigc uninstall --force`",
    )
}

/// Name what `remove_dir_all(<repo>/.jigc)` is about to destroy, on stderr, **before** it
/// runs — [`crate::milestone::DESTROYING_DOORS`]' narration law at this door
/// (`design/surface-contract.md` → law 1: a door that exits 0 must not also have silently
/// destroyed work). Until M46 Inc 2 this teardown reported only `- removed .jigc/`.
///
/// Its two subjects are exactly the two the guards in [`uninstall`] refuse on, and for the
/// same reason: the tree holds the sole copy of both. So the narration is **not** conditional
/// on `force` — `force` is what skips the *guards* — and a teardown those guards cleared
/// still takes any gitignored byte their probe deliberately does not look at (the *visible,
/// not prevented* bound: `crate::milestone::narrate_removal`).
///
/// Best-effort throughout: an unreadable workbench yields no warning rather than failing a
/// teardown that has already been cleared to run.
fn narrate_teardown(repo_root: &Path) {
    for path in fanout_worktree_paths(repo_root).unwrap_or_default() {
        crate::milestone::narrate_removal(repo_root, &path);
    }
    // The second subject, which no worktree probe can see: `.jigc/tasks/<id>/docs/*.md` is in
    // no object DB at all. A door that names only half of what it takes is a law-1
    // half-truth, so the set [`crate::task::staged_task_prose`] refuses on is the set named
    // here.
    narrate_staged_prose(repo_root, "removing `.jigc/`", None);
    // The third subject ([`workbench_paths`]), in both of its halves — because both are
    // removed. The untracked half reaches here only under `--force` (the guard refuses on
    // it otherwise), and is the half that is gone for good; the tracked half is what the
    // guard deliberately lets through, and law 1 owes it a name too — a teardown that took
    // a file in silence is a half-truth whether or not the file is recoverable. Each line
    // says which of the two it is, so the reader is not left to guess.
    narrate_workbench_files(repo_root);
}

/// Name the workbench files `remove_dir_all(<repo>/.jigc)` is about to take that are
/// neither a fan-out worktree nor an open task's staged prose — [`workbench_paths`]'
/// subject, split on recoverability by [`classify_workbench_paths`].
///
/// Two lines because they are two claims: bytes no index has a copy of are **not
/// recoverable**, and bytes the index carries are restored by `git checkout` after the
/// teardown. Collapsing them into one warning would overclaim on the tracked half and
/// underclaim on the untracked half, which is the same law-1 lie in both directions.
///
/// Best-effort, like every narration: an unreadable workbench or an unrunnable `git`
/// yields no warning rather than failing a teardown the guards already cleared.
fn narrate_workbench_files(repo_root: &Path) {
    let Ok((untracked, tracked)) = classify_workbench_paths(repo_root) else {
        return;
    };
    if !untracked.is_empty() {
        eprintln!(
            "warning: removing `.jigc/` destroys {} file(s) under it that no index has a copy \
             of:\n{}\n  note: nothing has a copy of those bytes — they are not recoverable.",
            untracked.len(),
            untracked
                .iter()
                .map(|path| format!("    {path}"))
                .collect::<Vec<_>>()
                .join("\n"),
        );
    }
    if !tracked.is_empty() {
        eprintln!(
            "warning: removing `.jigc/` also removes {} tracked file(s) under it:\n{}\n  \
             note: each is in the index, so `git checkout -- <path>` brings it back.",
            tracked.len(),
            tracked
                .iter()
                .map(|path| format!("    {path}"))
                .collect::<Vec<_>>()
                .join("\n"),
        );
    }
}

/// Name the **authored task prose** a door is about to destroy — the staged `*.md` under
/// `.jigc/tasks/<id>/docs/`, which is in no object DB at all, so the workbench is its only
/// copy.
///
/// **One emitter for every door that removes a task area**, the sibling of
/// [`crate::milestone::narrate_removal`]'s worktree-shaped subject and for the same reason:
/// two call sites of one rule drift. Its callers are the two doors that take those bytes
/// without a commit having carried them — `jigc uninstall`
/// ([`narrate_teardown`], whose `remove_dir_all(<repo>/.jigc)` takes the whole workbench)
/// and `jigc milestone discard` ([`crate::milestone::run_discard`], whose teardown
/// `remove_dir_all`s each sub-task area). The abandon door destroyed them **silently at
/// exit 0** until M46 Inc 2's validation, while `uninstall` refused over byte-identical
/// state — one door naming half the axis is exactly the half-truth that increment removes.
///
/// `action` is what the door is doing, in the door's own words ("removing `.jigc/`",
/// "discarding milestone:<id>"), so the warning names a destruction the reader is actually
/// standing at rather than a generic one.
///
/// `only` scopes the set to the task ids the caller's removal reaches — `None` for a door
/// that takes the whole workbench. A door must not name prose it will not touch: claiming a
/// destruction that does not happen is the same law-1 lie as performing one it never named.
/// The scoping is the probe's ([`crate::task::staged_task_prose`]'s `only`), not a filter
/// applied after the fact, so an out-of-scope area is never even read.
///
/// Best-effort, like every narration: an in-scope area it cannot read yields no warning
/// rather than failing a teardown the guards already cleared. It is surface over a removal,
/// never itself a gate — the declared bound stays *visible, not prevented*.
pub(crate) fn narrate_staged_prose(repo_root: &Path, action: &str, only: Option<&[String]>) {
    let Ok(staged) =
        crate::task::staged_task_prose(repo_root, only, &crate::task::unverified_prose_finding)
    else {
        return;
    };
    if staged.is_empty() {
        return;
    }
    let listing: Vec<String> = staged
        .iter()
        .map(|(task, docs)| format!("    {task}: {}", docs.join(", ")))
        .collect();
    eprintln!(
        "warning: {action} discards the staged docs of {} open task(s), which no commit \
         has a copy of:\n{}\n  note: the workbench is the only copy of those bytes — they are \
         not recoverable.",
        staged.len(),
        listing.join("\n"),
    );
}

/// One worktree-shaped path [`uninstall`] would destroy, and what git can say about it —
/// [`dirty_fanout_worktrees`]' element, and everything [`dirty_worktree_finding`] needs to
/// name an exit that is actually reachable from this path.
struct HeldWorktreePath {
    /// The `<repo>/.jigc/worktrees/<name>` path, as read (the refusal prints it).
    path: PathBuf,
    /// What removing it would destroy — `git status --porcelain` entries for a worktree of
    /// its own, the directory's sorted child names for one nothing vouches for, the bare fact
    /// that the path is a file, or the reason nothing could be read at all
    /// ([`crate::milestone::LeftoverHold`]).
    hold: crate::milestone::LeftoverHold,
    /// Whether **this** repository has the path registered as a worktree, and therefore
    /// whether a milestone teardown reaches it at all. `None` when `git worktree list` could
    /// not be read: the route then claims nothing either way rather than guessing — the same
    /// fail-closed stance the probe itself takes.
    registered: Option<bool>,
}

/// The teardown's refusal: a blocking, route-bearing finding naming every fan-out worktree
/// path that holds content, what is inside it, and — where git could tell — whether this
/// repository has it registered. The `milestone.dirty-worktree` sibling's shape, so the
/// doors read as one family.
///
/// **It claims "content", not "uncommitted work"**: a path git cannot vouch for (the copied
/// repo's worktree, a plain directory) is listed by its child names, and calling those
/// bytes *uncommitted work* would be a claim the probe cannot back
/// ([surface-contract.md](../../design/surface-contract.md) → law 1).
///
/// **The abandon arm is conditional on the same fact, for the same law.** A milestone
/// teardown removes the worktrees this repo **registered** and leaves every other path on
/// disk (`crate::milestone::remove_worktrees`; `discard`'s own refusal states it per path),
/// so naming `jigc milestone discard <id> --force` over a path registered nowhere routes the
/// operator through an irreversible, committed abandon that provably leaves this very
/// finding blocking the re-run — the exact shape the guard's path-subject exists to cover
/// (`cp -R` of a repo registers nothing under the copy's own `.jigc/worktrees/`). So the arm
/// appears only when some listed path is registered here, says what it leaves behind when
/// only some are, and is replaced by the plain statement that no abandon reaches them when
/// none is. Unknown registrations (`git worktree list` unreadable) promise neither.
fn dirty_worktree_finding(repo_root: &Path, dirty: &[HeldWorktreePath]) -> Finding {
    let listing: Vec<String> = dirty
        .iter()
        .map(|held| {
            let fate = match held.registered {
                Some(true) => "; registered as a worktree of this repository",
                Some(false) => "; registered as a worktree nowhere in this repository",
                None => "",
            };
            format!(
                "  {} — {}{fate}",
                crate::milestone::hold_line(repo_root, &held.path, &held.hold),
                crate::milestone::because(&held.hold),
            )
        })
        .collect();
    let any_registered = dirty.iter().any(|held| held.registered == Some(true));
    let any_unregistered = dirty.iter().any(|held| held.registered == Some(false));
    // A path that is not a readable directory is not a worktree either, so every worktree
    // remedy named below is inert over it — the route says so rather than leaving the reader
    // to discover it (RC-m50 W-2: this door routed at `git` being on PATH over a plain file).
    let any_non_worktree = dirty
        .iter()
        .any(|held| !matches!(held.hold.shape, crate::milestone::LeftoverShape::Directory));
    let mut route = String::from(
        "get the work out of those paths first (commit, stash, or copy it), then re-run \
         `jigc uninstall`",
    );
    if any_registered {
        route.push_str(
            " — or abandon the milestone with `jigc milestone discard <milestone-id> --force`, \
             which destroys the uncommitted work in the path(s) registered here",
        );
        if any_unregistered {
            route.push_str(
                " (the path(s) registered nowhere are left on disk and still block the teardown)",
            );
        }
        route.push_str(", and re-run `jigc uninstall`");
    } else if any_unregistered {
        route.push_str(
            " — abandoning the milestone will not clear them: a milestone teardown removes only \
             the worktrees this repository has registered, and none of these paths is",
        );
    }
    if any_non_worktree {
        route.push_str(
            " (a path listed above as a file, or as one nothing could be read at, is not a \
             worktree at all, so no worktree removal clears it — look at it and move it \
             aside or delete it yourself)",
        );
    }
    route.push_str("; `jigc uninstall --force` deletes them with the install");
    Finding::block(
        "uninstall.dirty-worktree",
        format!(
            "`.jigc/` holds {} fan-out sub-task worktree path(s) that removing it would \
             destroy and nothing can say are disposable:\n{}",
            dirty.len(),
            listing.join("\n"),
        ),
        route,
    )
}

/// The fail-closed half of [`dirty_fanout_worktrees`]: the worktrees root itself could not be
/// enumerated, so the teardown refuses rather than remove `.jigc/` with the fan-out worktrees'
/// safety unknown. Same code as the dirty refusal — the operator's next action is identical.
///
/// **Its subject is the enumeration, not a path** (M50 Increment 12 / T2). A path the probe
/// cannot read is a hold now ([`crate::milestone::LeftoverShape::Unreadable`]) and is answered
/// by [`dirty_worktree_finding`] beside its siblings; what is left here is the one failure
/// with no path to name — `read_dir` on `.jigc/worktrees/` itself — where there is no set to
/// enumerate at all.
fn unverified_worktrees_finding(err: anyhow::Error) -> Finding {
    Finding::block(
        "uninstall.dirty-worktree",
        format!(
            "cannot check `.jigc/worktrees/` for uncommitted fan-out work, so removing `.jigc/` \
             could destroy it: {err:#}"
        ),
        "make sure `git` is on PATH and the repository is readable, then re-run \
         `jigc uninstall` — or, once you have confirmed the fan-out worktrees hold nothing \
         you need, remove them yourself (`git worktree list`, then `git worktree remove`) and \
         re-run",
    )
}

/// The staged-doc refusal: a blocking, route-bearing finding naming every open task and
/// the doc identities staged in it. **Its own code**, not the worktree door's —
/// `uninstall.dirty-worktree` printed over `.jigc/tasks/<id>/docs/` with no worktree in
/// sight would name the wrong subject
/// ([surface-contract.md](../../design/surface-contract.md) → law 1). The code is the
/// door's stable identity and stays `uninstall.staged-prose`; what moved is the *claim*.
///
/// **It claims "staged doc(s)", not "authored prose"** — the sibling repair, applied to the
/// arm that shipped without it. [`crate::task::staged_task_prose`] cannot distinguish a
/// pristine machine-written skeleton from prose someone typed, and the **dominant** cell is the
/// skeleton: `jigc start` alone auto-creates `commit:<task>` with every slot empty and
/// trips this guard. Calling those bytes *authored doc prose* is a claim the probe cannot
/// back (law 1) — that the doc is **staged and in no commit** is exactly what it measured.
///
/// **The route names the reachable exit first.** Over that same dominant cell `jigc task
/// finalize` cannot succeed (the empty skeleton fails `schema-conformance`), so `jigc task
/// discard` leads and `finalize` follows with the condition that makes it available. The
/// listed identities are the addresses `jigc doc show <addr> --task <id>` takes, so reading
/// what you are about to lose is followable as printed.
///
/// **Its discard span carries `--force`** (M50 Inc 3 / T2). Since the task door took the same
/// guard, a bare `jigc task discard <task-id>` refuses in exactly the state that printed this
/// route — this finding fires *because* that task stages a doc — so the exit it names would
/// have been a route the wave's own guard blocks. `--force` is inert where nothing is staged,
/// so the consent can be named unconditionally.
fn staged_prose_finding(staged: &[(String, Vec<String>)]) -> Finding {
    let listing: Vec<String> = staged
        .iter()
        .map(|(task, docs)| format!("  {task}: {}", docs.join(", ")))
        .collect();
    let docs: usize = staged.iter().map(|(_, docs)| docs.len()).sum();
    Finding::block(
        "uninstall.staged-prose",
        format!(
            "`.jigc/` holds {docs} staged doc(s) for {} open task(s) that no commit has a copy \
             of — removing `.jigc/` would destroy them:\n{}",
            staged.len(),
            listing.join("\n"),
        ),
        "read what is in them with `jigc doc show <address> --task <task-id>`, then throw the \
         task away with `jigc task discard <task-id> --force` — or, once its doc is complete, \
         land it with `jigc task finalize <task-id>` (which refuses while a required slot is \
         empty) — then re-run `jigc uninstall`; `jigc uninstall --force` deletes them with the \
         install",
    )
}

/// The compose-marker key `pack::read_compose_marker` reads from `packs.yaml`.
const COMPOSE_MARKER_KEY: &str = "compose-embedded-methodology";

/// Write the `compose-embedded-methodology: true` marker into the project layer's
/// `<repo_root>/.jigc/config/packs.yaml` (step 2b of [`install`]).
///
/// **A `packs:` list is no longer a veto** (M49 Inc 6). The marker composes the two
/// *in-binary* packs, and since T1 the loader composes that pair *with* the listed
/// filesystem packs as `[listed… ▸ dev ▸ methodology]`, demoting the listed packs for
/// any doctype the embedded manifests declare frozen (`design/multi-pack.md` →
/// Embedded second pack). So the two configurations combine, and setup wires the
/// marker over whatever pack-set the operator declared instead of choosing between
/// them. The prior refusal — M42's repair of an even worse silent drop — was total,
/// and since setup writes the marker into *every* project it meant declaring one house
/// pack cost the whole methodology surface.
///
/// **Non-destructive:** an absent file is created carrying just the marker; an existing
/// `packs.yaml` is parsed and the marker set alongside whatever else it carries
/// (parse-mutate-serialize, the human's other keys — the `packs:` entries among them —
/// survive in order and content). A malformed `packs.yaml` is surfaced, never
/// clobbered. *Bound:* the round-trip is through the YAML value model, so a hand-written
/// file's comments and layout are re-emitted canonically, not preserved verbatim.
///
/// **Idempotent:** the serialized bytes are written only when they differ from what
/// is on disk, so a second `setup` over an already-marked file is a byte-identical
/// no-op (`serde_yaml_ng` serialization of a stable mapping is deterministic).
fn write_compose_marker(repo_root: &Path) -> std::io::Result<()> {
    use serde_yaml_ng::{Mapping, Value};

    let config_dir = repo_root.join(".jigc").join("config");
    std::fs::create_dir_all(&config_dir)?;
    let path = config_dir.join("packs.yaml");

    // Parse the existing file (preserving its content) or start from an empty
    // mapping. A malformed existing `packs.yaml` is a real authoring fault — surface
    // it rather than clobbering the human's bytes.
    let mut mapping = match std::fs::read_to_string(&path) {
        Ok(text) if text.trim().is_empty() => Mapping::new(),
        Ok(text) => serde_yaml_ng::from_str::<Value>(&text)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?
            .as_mapping()
            .cloned()
            .ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("{} is not a YAML mapping", path.display()),
                )
            })?,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Mapping::new(),
        Err(e) => return Err(e),
    };

    mapping.insert(Value::from(COMPOSE_MARKER_KEY), Value::Bool(true));

    let mut rendered = serde_yaml_ng::to_string(&Value::Mapping(mapping))
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    if !rendered.ends_with('\n') {
        rendered.push('\n');
    }

    // Idempotent: write only when the bytes change, so a second setup touches nothing.
    if std::fs::read_to_string(&path).ok().as_deref() != Some(rendered.as_str()) {
        std::fs::write(&path, rendered)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A throwaway directory that removes itself on drop (the project's
    /// no-tempfile pattern).
    struct TempDir(PathBuf);

    impl TempDir {
        fn new() -> Self {
            let mut path = std::env::temp_dir();
            let unique = format!(
                "jigc-setup-unit-{}-{:?}",
                std::process::id(),
                engine::tempname::unique_nanos(),
            );
            path.push(unique);
            std::fs::create_dir_all(&path).expect("create temp dir");
            TempDir(path)
        }

        fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    /// A clean install writes both host files and reports their paths from the
    /// profile.
    #[test]
    fn install_writes_both_targets_and_reports_paths() {
        let dir = TempDir::new();
        // `install` now installs the pre-commit hook, which resolves the repo's real
        // hooks dir via git — so the install target must be a git repo (as the real
        // `jigc setup` always is: `locate` requires one).
        git(dir.path(), &["init", "-q"]);
        let profile = adapter::load_profile("claude-code").expect("the shipped profile loads");

        let summary = install(dir.path(), &profile).expect("install succeeds");

        assert_eq!(summary.line_file, "CLAUDE.md");
        assert_eq!(summary.allowlist_file, ".claude/settings.json");
        assert!(
            dir.path().join("CLAUDE.md").exists(),
            "install must write CLAUDE.md",
        );
        assert!(
            dir.path().join(".claude/settings.json").exists(),
            "install must write .claude/settings.json",
        );
    }

    /// Install over the SHIPPED profile (a valid spawn template) does not trip the
    /// spawn-template gate — the success path is unbroken (this asserts the gate
    /// lets the shipped template through; the full write success is covered above).
    #[test]
    fn install_accepts_shipped_spawn_template() {
        let dir = TempDir::new();
        // The hook-install step resolves the real hooks dir via git (see above).
        git(dir.path(), &["init", "-q"]);
        let profile = adapter::load_profile("claude-code").expect("the shipped profile loads");

        install(dir.path(), &profile).expect("the shipped valid spawn template installs clean");
    }

    /// Install over a profile whose spawn template violates the decidable rule
    /// fails *before* any write with a single blocking `setup.spawn-template`
    /// finding whose route names the violated clause (the `SpawnTemplateReason`
    /// pointer). The gate runs ahead of the host-file writes, so a broken template
    /// touches nothing on disk.
    #[test]
    fn install_rejects_broken_spawn_template_with_clause_route() {
        use engine::finding::Severity;

        let dir = TempDir::new();
        let mut profile = adapter::load_profile("claude-code").expect("the shipped profile loads");
        // Strip the `{{task_id}}` placeholder — the first decidable clause.
        profile.spawn.template =
            "Use your Task tool to run: `jigc workflow {{workflow}} --task X`".to_string();

        let finding =
            install(dir.path(), &profile).expect_err("a broken spawn template must fail install");

        assert_eq!(finding.severity, Severity::Blocking);
        assert_eq!(
            finding.code, "setup.spawn-template",
            "the block must be the spawn-template family code; got `{}`",
            finding.code,
        );
        let route = finding
            .route
            .as_deref()
            .expect("a hard block must carry a route");
        assert!(
            route.contains(&adapter::SpawnTemplateReason::MissingTaskIdPlaceholder.to_string()),
            "the route must name the violated clause; got `{route}`",
        );
        // Nothing was written: the gate runs before the host-file writes.
        assert!(
            !dir.path().join("CLAUDE.md").exists(),
            "a rejected template must touch no host files",
        );
    }

    /// Write an executable fake-`jigc` shim that emits `stdout` on stdout, `stderr`
    /// on stderr, and exits with `code` — a canned `jigc validate --format json`
    /// stand-in the rendered hook drives as a real subprocess. Returns its path.
    fn write_fake_jigc(dir: &Path, name: &str, stdout: &str, stderr: &str, code: i32) -> PathBuf {
        let bin = dir.join(name);
        // `printf '%s'` (not `echo`) so the canned JSON is emitted byte-for-byte; the
        // shim drains nothing — the hook only reads its stdout.
        let script = format!(
            "#!/bin/sh\nprintf '%s' '{}' \nprintf '%s' '{}' >&2\nexit {code}\n",
            stdout.replace('\'', "'\\''"),
            stderr.replace('\'', "'\\''"),
        );
        std::fs::write(&bin, script).expect("write fake jigc shim");
        let mut perms = std::fs::metadata(&bin).expect("stat shim").permissions();
        std::os::unix::fs::PermissionsExt::set_mode(&mut perms, 0o755);
        std::fs::set_permissions(&bin, perms).expect("chmod shim");
        bin
    }

    /// Run the rendered hook (`precommit_hook_body` pointed at `jigc_path`) through a
    /// real `sh`, returning `(stderr, exit_code)`. Executes the **emitted bytes**
    /// verbatim via `sh -c "$body"` — never a reconstruction — so the test drives the
    /// contract an installed hook actually runs.
    fn run_rendered_hook(jigc_path: &Path) -> (String, i32) {
        let body = precommit_hook_body(jigc_path);
        let out = std::process::Command::new("sh")
            .arg("-c")
            .arg(&body)
            .output()
            .expect("run the rendered pre-commit hook under sh");
        (
            String::from_utf8_lossy(&out.stderr).into_owned(),
            out.status.code().expect("the hook exits with a code"),
        )
    }

    /// The load-bearing B2 output-discipline contract, executed end-to-end: the
    /// rendered hook is driven by `sh` against canned `jigc validate --format json`
    /// outputs across all six cases. It warns IFF the report's top-level
    /// `blocking_probes` array names `doc-code` — keyed on the findings, **never** on the
    /// exit code — and **always exits 0** (warn-only; it never blocks a commit).
    ///
    /// **This is the shape-robustness pin, not the acceptance** (M47). A hand-written
    /// report cannot see a serde or renderer change, so the behavioural contract is proven
    /// in `crates/cli/tests/precommit_hook_acceptance.rs` against **real** reports from the
    /// real binary; what these canned cases add is that the match survives *spacing*
    /// (pretty vs compact) and the array's multi-line pretty rendering.
    #[test]
    fn rendered_hook_warns_iff_doc_code_content_finding_always_exits_zero() {
        let dir = TempDir::new();

        // The `doc-code` content-finding marker the hook keys on, in the pretty-JSON
        // spacing `jigc validate --format json` emits (`render::json` → to_string_pretty).
        let warning = "doc<->code drift detected";

        // (1) Clean store: nothing blocked, no findings, exit 0 -> silent.
        let clean =
            "{\n  \"blocking_probes\": [],\n  \"findings\": [],\n  \"schema_version\": 3\n}";
        let jigc = write_fake_jigc(dir.path(), "jigc-clean", clean, "", 0);
        let (stderr, code) = run_rendered_hook(&jigc);
        assert_eq!(code, 0, "clean store must exit 0; stderr:\n{stderr}");
        assert!(
            !stderr.contains(warning),
            "a clean store must warn nothing; stderr:\n{stderr}",
        );

        // (2) A BLOCKING doc-code finding (exit 0, per the detect-and-report rule) ->
        // the only case that warns. `blocking_probes` is the keyed surface, and it
        // renders multi-line — the case the newline collapse exists for.
        let drift = "{\n  \"blocking_probes\": [\n    \"doc-code\"\n  ],\n  \"findings\": [\n    {\n      \"severity\": \"blocking\",\n      \"probe\": \"doc-code\",\n      \"check\": \"symbol-exists\",\n      \"code\": \"doc-code.symbol-exists\",\n      \"message\": \"anchor crates/engine/src/cache.rs#evict_lru does not resolve\",\n      \"address\": \"decisions/cache.md\",\n      \"route\": null\n    }\n  ],\n  \"schema_version\": 3\n}";
        let jigc = write_fake_jigc(dir.path(), "jigc-drift", drift, "", 0);
        let (stderr, code) = run_rendered_hook(&jigc);
        assert_eq!(
            code, 0,
            "a doc-code finding is warn-only — exit 0; stderr:\n{stderr}"
        );
        assert!(
            stderr.contains(warning),
            "a BLOCKING doc-code finding MUST warn; stderr:\n{stderr}",
        );

        // (3) Not-a-jigc-project: the operational-error envelope on STDERR, a
        // non-zero exit. No `blocking_probes` array on stdout at all -> silent, exit 0
        // (keyed on the finding, never the exit code).
        let jigc = write_fake_jigc(
            dir.path(),
            "jigc-notproject",
            "",
            "{\n  \"error\": \"not a jigc project: no `.jigc/config/` layer found\"\n}",
            1,
        );
        let (stderr, code) = run_rendered_hook(&jigc);
        assert_eq!(
            code, 0,
            "not-a-project must still exit 0; stderr:\n{stderr}"
        );
        assert!(
            !stderr.contains(warning),
            "not-a-jigc-project must warn nothing (a non-zero exit must not warn); stderr:\n{stderr}",
        );

        // (4) A blocking `pack-probe-integrity` meta-finding, with an ADVISORY doc-code
        // finding sitting in `findings` right after the array -> silent, exit 0. Two
        // properties at once: the discipline must not warn on a meta-finding, and the
        // match must stay INSIDE the array — bounded by its own `]`, it cannot reach the
        // `"probe": "doc-code"` member that follows.
        let meta = "{\n  \"blocking_probes\": [\n    \"pack-probe-integrity\"\n  ],\n  \"findings\": [\n    {\n      \"severity\": \"blocking\",\n      \"probe\": \"pack-probe-integrity\",\n      \"check\": \"crash\",\n      \"code\": \"pack-probe-integrity.crash\",\n      \"message\": \"the doc-code probe exited 2 without emitting JSON\",\n      \"route\": null\n    },\n    {\n      \"severity\": \"advisory\",\n      \"probe\": \"doc-code\",\n      \"check\": \"title-names-symbol\",\n      \"code\": \"doc-code.title-names-symbol\",\n      \"message\": \"component title `sessionStore` names a symbol its anchor does not implement\",\n      \"route\": null\n    }\n  ],\n  \"schema_version\": 3\n}";
        let jigc = write_fake_jigc(dir.path(), "jigc-meta", meta, "", 1);
        let (stderr, code) = run_rendered_hook(&jigc);
        assert_eq!(
            code, 0,
            "a probe-integrity run must still exit 0; stderr:\n{stderr}"
        );
        assert!(
            !stderr.contains(warning),
            "a pack-probe-integrity meta-finding must warn nothing, and the match must not \
             escape the array into the advisory doc-code finding beside it; stderr:\n{stderr}",
        );

        // (5) jigc absent: the embedded path does not resolve to an executable, so
        // the hook's command substitution fails, `report` is empty -> silent, exit 0
        // (the silent-exit-0 guard covers a removed/stale jigc).
        let absent = dir.path().join("jigc-does-not-exist");
        let (stderr, code) = run_rendered_hook(&absent);
        assert_eq!(
            code, 0,
            "an absent jigc must still exit 0; stderr:\n{stderr}"
        );
        assert!(
            !stderr.contains(warning),
            "an absent jigc must warn nothing (it no-ops cleanly); stderr:\n{stderr}",
        );

        // (6) A blocking doc-code finding, but the JSON is emitted **compact** (no space
        // after the colons): `{"blocking_probes":["doc-code"],…}`. The detection must NOT
        // couple to the pretty-print spacing — if `jigc validate --format json` is ever
        // emitted compact (or with different spacing), the hook must still warn, not go
        // silently dead and ship drift as a false-clean.
        let compact = "{\"blocking_probes\":[\"doc-code\"],\"findings\":[{\"severity\":\"blocking\",\"probe\":\"doc-code\",\"check\":\"symbol-exists\",\"code\":\"doc-code.symbol-exists\",\"message\":\"anchor crates/engine/src/cache.rs#evict_lru does not resolve\",\"address\":\"decisions/cache.md\",\"route\":null}],\"schema_version\":3}";
        let jigc = write_fake_jigc(dir.path(), "jigc-compact", compact, "", 0);
        let (stderr, code) = run_rendered_hook(&jigc);
        assert_eq!(
            code, 0,
            "a compact doc-code finding is warn-only — exit 0; stderr:\n{stderr}"
        );
        assert!(
            stderr.contains(warning),
            "a compact (no-space-after-colon) blocking doc-code finding MUST still warn; \
             stderr:\n{stderr}",
        );
    }

    /// Golden-lock the rendered `pre-commit` body for a fixed `jigc_path`: the
    /// sentinel marker is present, the **absolute** path is embedded (quoted),
    /// `--format json` is invoked, the discipline keys on the `blocking_probes` array
    /// over a newline-collapsed copy of the report (a POSIX ERE, bounded by the array's
    /// own `]`), and the script always `exit 0`. Pins the
    /// bytes an installed hook would run — a
    /// rename, a reorder, or a discipline slip breaks it (the B2 contract).
    #[test]
    fn precommit_hook_body_golden() {
        let body = precommit_hook_body(Path::new("/abs/install/bin/jigc"));
        assert_eq!(
            body,
            "#!/bin/sh\n\
             # jigc-managed pre-commit hook (doc<->code backstop) — regenerated by `jigc setup`\n\
             #\n\
             # Warn-only doc<->code drift backstop: runs `jigc validate` over the\n\
             # committed store and prints a warning ONLY when a doc-code check raised a\n\
             # BLOCKING finding. Always exits 0 — it never blocks the commit. Keys on the\n\
             # findings, never on the exit code (the exit code is wrong-way-round).\n\
             \n\
             jigc='/abs/install/bin/jigc'\n\
             \n\
             # Run the sweep, capturing stdout only. If jigc is absent or fails to run,\n\
             # `report` is empty and nothing below matches -> silent exit 0.\n\
             report=\"$(\"$jigc\" validate --format json 2>/dev/null)\"\n\
             \n\
             # Warn IFF the report's top-level `blocking_probes` array names `doc-code` —\n\
             # the probes that raised a BLOCKING finding. Keying on SEVERITY, not on mere\n\
             # presence: an advisory-only doc-code finding (the stale-heading guard; a\n\
             # citation the probe cannot check) must not warn on every commit forever. A\n\
             # clean store, a not-a-jigc-project / probe-missing run (an `error` envelope\n\
             # on stderr, nothing matching here on stdout), and any `pack-probe-integrity`\n\
             # meta-finding all fall through to a silent exit 0.\n\
             #\n\
             # The report is pretty-printed, so the array spans several lines: the newline\n\
             # collapse is what lets one ERE bind the key to a member (without it this\n\
             # matches nothing). The collapse is PIPELINE-LOCAL — `$report` itself keeps\n\
             # its newlines, because the rename block below pairs one route per LINE. The\n\
             # match is bounded by the array's OWN `]`, so it cannot reach into `findings`;\n\
             # whitespace is POSIX-classed so it holds on BSD/macOS grep and survives\n\
             # compact or differently-spaced JSON.\n\
             if printf '%s' \"$report\" | tr -d '\\n\
             ' | grep -Eq '\"blocking_probes\"[[:space:]]*:[[:space:]]*\\[[^]]*\"doc-code\"'; then\n\
             \techo 'jigc: doc<->code drift detected in committed docs — run `jigc validate` for details (commit not blocked).' >&2\n\
             fi\n\
             \n\
             # M35 — block this commit IFF it ITSELF stages an out-of-band managed-doc rename (a\n\
             # bare `git mv` committed without `jigc rename`). The sweep above flags every\n\
             # recorded-but-missing managed doc as a `reconciliation.rename` finding whose route\n\
             # names the pair as `git mv <new> <old>` (the revert direction). A move landed in a\n\
             # PRIOR commit must NOT block an unrelated later commit (the masking trap), so block\n\
             # ONLY when BOTH the old and new paths are staged in THIS commit. Keys on the finding\n\
             # plus the staged set, never on jigc's exit code.\n\
             moves=\"$(printf '%s' \"$report\" | grep -o 'git mv [^`]*')\"\n\
             if [ -n \"$moves\" ]; then\n\
             \t# Every path THIS commit stages, rename-aware: a staged `git mv` shows as `R old new`\n\
             \t# under --find-renames; a delete+add as `D old` / `A new`. One path per line.\n\
             \tstaged=\"$(git diff --cached --name-status --find-renames 2>/dev/null | cut -f2- | tr '\\t' '\\n\
             ')\"\n\
             \t# Block iff some `git mv <new> <old>` route has BOTH its paths in the staged set.\n\
             \tif { printf '%s\\n\
             ' \"$staged\"; echo '---'; printf '%s\\n\
             ' \"$moves\"; } | awk '$0 == \"---\" { seen = 1; next } seen == 0 { S[$0] = 1; next } NF >= 4 && ($3 in S) && ($4 in S) { hit = 1 } END { exit hit ? 0 : 1 }'; then\n\
             \t\techo 'jigc: out-of-band managed-doc rename staged in this commit — a bare `git mv` bypasses jigc identity tracking; use `jigc rename` instead (commit blocked).' >&2\n\
             \t\texit 1\n\
             \tfi\n\
             \techo 'jigc: an out-of-band managed-doc rename exists in the committed tree — run `jigc validate` for details (not staged in this commit; commit not blocked).' >&2\n\
             fi\n\
             \n\
             exit 0\n",
        );
    }

    /// Run `git -C <dir> <args...>`, asserting success — the test driver for the
    /// throwaway `git init` repos the hook-install tests resolve a real hooks dir
    /// against.
    fn git(dir: &Path, args: &[&str]) {
        let status = std::process::Command::new("git")
            .arg("-C")
            .arg(dir)
            .args(args)
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .env("GIT_CONFIG_SYSTEM", "/dev/null")
            .output()
            .expect("run git");
        assert!(
            status.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&status.stderr),
        );
    }

    /// Run `git -C <dir> <args>`, asserting success, returning trimmed stdout — the
    /// capturing companion to [`git`] used by the install-commit test to inspect git
    /// state (HEAD subject, the committed file set, the staged set).
    fn git_str(dir: &Path, args: &[&str]) -> String {
        let out = std::process::Command::new("git")
            .arg("-C")
            .arg(dir)
            .args(args)
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .env("GIT_CONFIG_SYSTEM", "/dev/null")
            .output()
            .expect("run git");
        assert!(
            out.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr),
        );
        String::from_utf8_lossy(&out.stdout).trim().to_string()
    }

    /// `jigc setup` commits its own install as a dedicated commit (M26 shakedown): the
    /// install files land in their OWN commit, the user's unrelated staged work is NOT
    /// swept in, and a second `setup` is a clean no-op (no new commit).
    #[test]
    fn install_commits_only_its_own_files_in_a_dedicated_commit_idempotently() {
        let dir = TempDir::new();
        git(dir.path(), &["init", "-q"]);
        git(dir.path(), &["config", "user.email", "t@t"]);
        git(dir.path(), &["config", "user.name", "t"]);
        // Override any ambient global signing config so the install commit can land in
        // CI / on a signing-enabled dev machine.
        git(dir.path(), &["config", "commit.gpgsign", "false"]);
        // A first commit so HEAD exists (the shakedown repo had an initial commit).
        std::fs::write(dir.path().join("README.md"), "hi\n").expect("seed README");
        git(dir.path(), &["add", "README.md"]);
        git(dir.path(), &["commit", "-q", "-m", "initial"]);

        // A pre-existing UNRELATED working file the user has already staged — setup must
        // not sweep it into its install commit.
        std::fs::write(dir.path().join("user-work.txt"), "wip\n").expect("seed user file");
        git(dir.path(), &["add", "user-work.txt"]);

        let profile = adapter::load_profile("claude-code").expect("the shipped profile loads");
        let summary = install(dir.path(), &profile).expect("install succeeds");

        // (1) setup committed its install in its OWN commit naming the install files.
        assert!(
            matches!(summary.install_commit, InstallCommit::Committed(_)),
            "setup must report a committed install; got {:?}",
            summary.install_commit,
        );
        assert_eq!(
            git_str(dir.path(), &["log", "-1", "--format=%s"]),
            INSTALL_COMMIT_MESSAGE,
            "the HEAD commit must be the dedicated install commit",
        );
        let committed = git_str(dir.path(), &["show", "--name-only", "--format=", "HEAD"]);
        for f in [
            ".jigc/AGENT.md",
            ".jigc/.gitignore",
            ".jigc/config/.gitkeep",
            ".jigc/config/packs.yaml",
            "CLAUDE.md",
            ".claude/settings.json",
        ] {
            assert!(
                committed.lines().any(|l| l == f),
                "the install commit must include `{f}`; got:\n{committed}",
            );
        }

        // (2) the user's unrelated staged file is NOT swept into the install commit, and
        //     remains staged (uncommitted) for them.
        assert!(
            !committed.lines().any(|l| l == "user-work.txt"),
            "the user's unrelated file must not be swept into the install commit; got:\n{committed}",
        );
        let staged = git_str(dir.path(), &["diff", "--cached", "--name-only"]);
        assert!(
            staged.lines().any(|l| l == "user-work.txt"),
            "the user's unrelated file must stay staged after setup; got:\n{staged}",
        );

        // (3) a second setup is a clean no-op: no new commit, reported as `Nothing`.
        let head_before = git_str(dir.path(), &["rev-parse", "HEAD"]);
        let summary2 = install(dir.path(), &profile).expect("re-install succeeds");
        assert!(
            matches!(summary2.install_commit, InstallCommit::Nothing),
            "a second setup over an unchanged install must report Nothing; got {:?}",
            summary2.install_commit,
        );
        assert_eq!(
            head_before,
            git_str(dir.path(), &["rev-parse", "HEAD"]),
            "a second setup must make no new commit",
        );
    }

    /// A git step that **ran and refused** on the install-commit path is never absorbed
    /// into a silent success — on the **staging** half as much as on the commit half —
    /// and the finding it surfaces routes on git's own cause, never at the git identity
    /// by default (surface-contract law 1).
    ///
    /// The refusal is forced with a held `index.lock`, the one cause that makes `git add`
    /// decline for a reason that has nothing to do with `user.email`: before this, that
    /// branch returned `Ok(Skipped)`, so the whole install commit vanished behind the
    /// success banner while `jigc setup` listed everything as installed.
    #[test]
    fn a_refused_install_stage_is_loud_and_routes_on_gits_own_cause() {
        let dir = TempDir::new();
        git(dir.path(), &["init", "-q"]);
        git_identity(dir.path());
        git(dir.path(), &["commit", "-q", "--allow-empty", "-m", "init"]);
        std::fs::write(dir.path().join("CLAUDE.md"), "x\n").expect("write an install file");
        std::fs::write(dir.path().join(".git/index.lock"), "").expect("hold the index lock");

        let rejection = commit_install(
            dir.path(),
            "CLAUDE.md",
            ".claude/settings.json",
            false,
            Path::new("/nonexistent/hooks/pre-commit"),
            None,
        )
        .expect_err("a git step that ran and refused must not degrade to a silent skip");

        let finding = rejection.finding();
        assert_eq!(finding.code, "setup.install-commit");
        assert!(
            finding.message.contains("git add"),
            "the message must name the step git refused; got:\n{}",
            finding.message,
        );
        assert!(
            !finding.message.contains("staged"),
            "a refused `git add` staged nothing — the message must not claim it did; got:\n{}",
            finding.message,
        );
        let route = finding
            .route
            .expect("a blocking finding carries a route")
            .to_string();
        assert!(
            !route.contains("user.email"),
            "the identity route is a lie for a cause that is not the identity; got: {route}",
        );

        // …and the identity cause still gets the identity route (the widening keeps the
        // case the route was written for).
        let identity = InstallCommitRejection::Commit(
            "Author identity unknown\n*** Please tell me who you are.\n\
             fatal: unable to auto-detect email address (got 'u@h.(none)')"
                .to_string(),
        )
        .finding();
        let identity_route = identity.route.expect("a blocking finding carries a route");
        assert!(
            identity_route.to_string().contains("user.email"),
            "an identity rejection must still route at the git identity; got: {identity_route}",
        );
    }

    /// Give the repo at `dir` a committable identity + disabled signing, so the real
    /// install path's `git commit` lands in CI / on a signing-enabled dev machine.
    fn git_identity(dir: &Path) {
        git(dir, &["config", "user.email", "t@t"]);
        git(dir, &["config", "user.name", "t"]);
        git(dir, &["config", "commit.gpgsign", "false"]);
    }

    /// A **fresh** (zero-commit) repo gets the secrets-floor `.gitignore` seeded with the
    /// real secret set under the sentinel; the function reports it seeded.
    #[test]
    fn seed_secrets_gitignore_writes_floor_on_zero_commit_repo() {
        let dir = TempDir::new();
        git(dir.path(), &["init", "-q"]);

        let seeded = seed_secrets_gitignore(dir.path()).expect("seed succeeds on a fresh repo");

        assert!(
            seeded,
            "a zero-commit repo is fresh — the floor must be seeded"
        );
        let gi = dir.path().join(".gitignore");
        let body = std::fs::read_to_string(&gi).expect("the floor `.gitignore` must be written");
        assert!(
            body.contains(SECRETS_GITIGNORE_SENTINEL),
            "the seeded floor must carry the sentinel; got:\n{body}",
        );
        // Spot-check load-bearing secret patterns actually reach disk (not a tautology).
        for pat in [
            ".env",
            ".env.*",
            "!.env.example",
            "*.pem",
            "*.key",
            "credentials",
            ".npmrc",
        ] {
            assert!(
                body.lines().any(|l| l.starts_with(pat)),
                "the floor must ignore `{pat}`; got:\n{body}",
            );
        }
    }

    /// The seeded floor's `!.env.example` negation must **actually** un-ignore a
    /// committed template — not merely appear as a line. gitignore has no inline
    /// comments, so a trailing `# …` on the negation line would make git read the
    /// whole `!.env.example  # …` string as the (literal) filename and never fire the
    /// negation, leaving `.env.example` silently ignored by `.env.*`. Exercise git
    /// itself (not string presence): after seeding, `.env.example` is NOT ignored while
    /// `.env` still is.
    #[test]
    fn seed_secrets_gitignore_negation_actually_unignores_env_example() {
        let dir = TempDir::new();
        git(dir.path(), &["init", "-q"]);

        seed_secrets_gitignore(dir.path()).expect("seed succeeds on a fresh repo");

        assert!(
            !git_path_ignored(dir.path(), ".env.example"),
            "the seeded `!.env.example` negation must un-ignore a committed template",
        );
        assert!(
            git_path_ignored(dir.path(), ".env"),
            "the floor must still ignore a real `.env`",
        );
    }

    /// An **established** repo (>=1 commit) is left completely untouched — no seed.
    #[test]
    fn seed_secrets_gitignore_leaves_committed_repo_untouched() {
        let dir = TempDir::new();
        git(dir.path(), &["init", "-q"]);
        git_identity(dir.path());
        std::fs::write(dir.path().join("README.md"), "hi\n").expect("seed README");
        git(dir.path(), &["add", "README.md"]);
        git(dir.path(), &["commit", "-q", "-m", "initial"]);

        let seeded = seed_secrets_gitignore(dir.path()).expect("no-op succeeds");

        assert!(!seeded, "a repo with >=1 commit is established — no seed");
        assert!(
            !dir.path().join(".gitignore").exists(),
            "an established repo's root `.gitignore` must not be created",
        );
    }

    /// A directory that is **not a git repo** yields no seed (conservative-on-unclear).
    #[test]
    fn seed_secrets_gitignore_no_seed_when_not_a_repo() {
        let dir = TempDir::new();

        let seeded = seed_secrets_gitignore(dir.path()).expect("no-op succeeds off a repo");

        assert!(
            !seeded,
            "not a git repo — the signal is unclear, so no seed"
        );
        assert!(
            !dir.path().join(".gitignore").exists(),
            "a non-repo must never get a secrets floor dropped into it",
        );
    }

    /// A fresh repo carrying a **foreign** root `.gitignore` keeps its lines verbatim and
    /// gains only the sentinel floor block appended; a re-run is byte-stable.
    #[test]
    fn seed_secrets_gitignore_merges_foreign_gitignore_never_clobbers() {
        let dir = TempDir::new();
        git(dir.path(), &["init", "-q"]);
        let foreign = "node_modules/\ntarget/\n";
        std::fs::write(dir.path().join(".gitignore"), foreign).expect("seed foreign .gitignore");

        let seeded = seed_secrets_gitignore(dir.path()).expect("merge succeeds");
        assert!(
            seeded,
            "a fresh repo with a foreign `.gitignore` still gets the floor"
        );

        let merged = std::fs::read_to_string(dir.path().join(".gitignore")).expect("read merged");
        assert!(
            merged.starts_with(foreign),
            "the human's foreign lines must be preserved verbatim at the top; got:\n{merged}",
        );
        assert!(
            merged.contains(SECRETS_GITIGNORE_SENTINEL) && merged.contains(".env"),
            "the floor must be appended under the sentinel; got:\n{merged}",
        );

        // Re-run: the sentinel is present, so it is a byte-stable no-op.
        let seeded2 = seed_secrets_gitignore(dir.path()).expect("re-run succeeds");
        assert!(seeded2, "the still-fresh re-run reports the floor present");
        let after = std::fs::read_to_string(dir.path().join(".gitignore")).expect("read re-run");
        assert_eq!(
            after, merged,
            "a re-run over a seeded floor must be byte-identical"
        );
    }

    /// Create-from-absent then re-run is byte-stable (the absent-file path's idempotency).
    #[test]
    fn seed_secrets_gitignore_create_then_rerun_byte_stable() {
        let dir = TempDir::new();
        git(dir.path(), &["init", "-q"]);

        seed_secrets_gitignore(dir.path()).expect("first seed");
        let first = std::fs::read_to_string(dir.path().join(".gitignore")).expect("read first");
        seed_secrets_gitignore(dir.path()).expect("second seed");
        let second = std::fs::read_to_string(dir.path().join(".gitignore")).expect("read second");

        assert_eq!(first, second, "a re-run must be byte-identical");
        assert_eq!(
            first, SECRETS_GITIGNORE_BLOCK,
            "an absent-file seed writes exactly the floor block",
        );
    }

    /// Real `install` on a **fresh** repo writes the secrets floor to root `.gitignore`
    /// AND commits it in the dedicated install commit.
    #[test]
    fn install_seeds_and_commits_secrets_gitignore_on_fresh_repo() {
        let dir = TempDir::new();
        git(dir.path(), &["init", "-q"]);
        git_identity(dir.path());
        let profile = adapter::load_profile("claude-code").expect("the shipped profile loads");

        install(dir.path(), &profile).expect("install succeeds on a fresh repo");

        let gi = dir.path().join(".gitignore");
        let body = std::fs::read_to_string(&gi).expect("install must seed root .gitignore");
        assert!(
            body.contains(SECRETS_GITIGNORE_SENTINEL) && body.contains(".env"),
            "install must write the secrets floor; got:\n{body}",
        );
        let committed = git_str(dir.path(), &["show", "--name-only", "--format=", "HEAD"]);
        assert!(
            committed.lines().any(|l| l == ".gitignore"),
            "the seeded root `.gitignore` must be in the install commit; got:\n{committed}",
        );
    }

    /// Real `install` on an **established** repo leaves a pre-existing foreign root
    /// `.gitignore` byte-untouched and never sweeps it into the install commit.
    #[test]
    fn install_leaves_established_repo_gitignore_untouched_and_uncommitted() {
        let dir = TempDir::new();
        git(dir.path(), &["init", "-q"]);
        git_identity(dir.path());
        std::fs::write(dir.path().join("README.md"), "hi\n").expect("seed README");
        git(dir.path(), &["add", "README.md"]);
        git(dir.path(), &["commit", "-q", "-m", "initial"]);
        // A pre-existing foreign root `.gitignore` the user owns (unstaged working edit).
        let foreign = "node_modules/\n";
        std::fs::write(dir.path().join(".gitignore"), foreign).expect("seed foreign .gitignore");

        let profile = adapter::load_profile("claude-code").expect("the shipped profile loads");
        install(dir.path(), &profile).expect("install succeeds on an established repo");

        assert_eq!(
            std::fs::read_to_string(dir.path().join(".gitignore")).expect("read foreign"),
            foreign,
            "an established repo's foreign `.gitignore` must be byte-untouched",
        );
        let committed = git_str(dir.path(), &["show", "--name-only", "--format=", "HEAD"]);
        assert!(
            !committed.lines().any(|l| l == ".gitignore"),
            "the user's `.gitignore` must not be swept into the install commit; got:\n{committed}",
        );
    }

    /// The file mode of `path`, masked to the permission bits.
    fn mode(path: &Path) -> u32 {
        use std::os::unix::fs::PermissionsExt;
        std::fs::metadata(path)
            .expect("stat installed hook")
            .permissions()
            .mode()
            & 0o777
    }

    /// A default `git init` repo gets an executable `.git/hooks/pre-commit` carrying
    /// the sentinel block (the rendered T1 body), and a second install is
    /// **byte-identical** (idempotent — the block is regenerated, not appended twice).
    #[test]
    fn install_precommit_writes_executable_sentinel_hook_idempotently() {
        let dir = TempDir::new();
        git(dir.path(), &["init", "-q"]);
        let jigc = PathBuf::from("/abs/install/bin/jigc");

        install_precommit_hook(dir.path(), &jigc).expect("first install succeeds");

        let hook = dir.path().join(".git/hooks/pre-commit");
        let first = std::fs::read_to_string(&hook).expect("hook written to .git/hooks");
        assert!(
            first.contains(PRECOMMIT_SENTINEL),
            "the installed hook must carry the sentinel block",
        );
        assert_eq!(
            first,
            precommit_hook_body(&jigc),
            "a fresh install is exactly the rendered T1 body",
        );
        assert_eq!(
            mode(&hook) & 0o100,
            0o100,
            "the hook must be owner-executable"
        );

        // A second install is byte-identical (regenerated in place, not duplicated).
        install_precommit_hook(dir.path(), &jigc).expect("second install succeeds");
        let second = std::fs::read_to_string(&hook).expect("hook still present");
        assert_eq!(
            second, first,
            "a re-install must be byte-identical (idempotent)"
        );
        assert_eq!(
            mode(&hook) & 0o100,
            0o100,
            "the hook stays executable on re-install"
        );
    }

    /// A repo with `core.hooksPath` set gets the hook in **that** dir, not the naive
    /// `.git/hooks` join (the resolution must honor `core.hooksPath`).
    #[test]
    fn install_precommit_honors_core_hookspath() {
        let dir = TempDir::new();
        git(dir.path(), &["init", "-q"]);
        let hooks = dir.path().join("my-hooks");
        git(
            dir.path(),
            &["config", "core.hooksPath", hooks.to_str().unwrap()],
        );

        install_precommit_hook(dir.path(), Path::new("/abs/bin/jigc")).expect("install succeeds");

        assert!(
            hooks.join("pre-commit").exists(),
            "the hook must land in the core.hooksPath dir",
        );
        assert!(
            !dir.path().join(".git/hooks/pre-commit").exists(),
            "with core.hooksPath set, nothing is written to .git/hooks",
        );
    }

    /// A linked worktree (the `.git`-is-a-file case) resolves to the **common**
    /// hooks dir under the main checkout's `.git/hooks`, not a per-worktree dir.
    #[test]
    fn install_precommit_resolves_worktree_common_hooks_dir() {
        let main = TempDir::new();
        git(main.path(), &["init", "-q"]);
        git(main.path(), &["config", "user.email", "t@t"]);
        git(main.path(), &["config", "user.name", "t"]);
        git(
            main.path(),
            &["commit", "-q", "--allow-empty", "-m", "init"],
        );

        let linked = main.path().join("linked");
        git(
            main.path(),
            &["worktree", "add", "-q", linked.to_str().unwrap()],
        );
        // The linked worktree's `.git` is a file (the git-worktree case).
        assert!(
            linked.join(".git").is_file(),
            "the linked worktree's .git must be a file",
        );

        install_precommit_hook(&linked, Path::new("/abs/bin/jigc")).expect("install succeeds");

        // The hook lands in the COMMON hooks dir (the main checkout's .git/hooks),
        // not under any per-worktree git dir.
        assert!(
            main.path().join(".git/hooks/pre-commit").exists(),
            "a worktree install resolves to the common .git/hooks dir",
        );
    }

    /// [`committable_hook_path`] admits the hook **iff** it is a working-tree file **of
    /// this repo**, over the whole hooks-dir axis — and in particular refuses the default
    /// `.git/hooks/pre-commit`, which *is* under the repo root. A `.git`-internal
    /// pathspec entry is inert (`git add` on it stages nothing and exits 0), so it cannot
    /// be caught downstream by the committed set: the refusal has to be pinned here.
    ///
    /// Cells (5) and (6) are the **ownership** half of the rule, on the same axis: a
    /// hooks dir sitting under the root and outside git's own dirs may still belong to
    /// **another** repository — a submodule (the common shared-hooks pattern) or a plain
    /// embedded repo — and this index cannot take that path (`git add` is a fatal refusal
    /// on the first shape, and `git commit -- <path>` a "did not match any file(s) known
    /// to git" on the second). Both are refused by asking git which repo owns the hook's
    /// own directory, which is why an un-enumerated seventh shape is refused too.
    #[test]
    fn committable_hook_path_admits_only_working_tree_hooks() {
        let jigc = Path::new("/abs/bin/jigc");

        // (1) The default hooks dir: under the repo root, but inside git's own dir.
        let default = TempDir::new();
        git(default.path(), &["init", "-q"]);
        let hook = install_precommit_hook(default.path(), jigc).expect("install");
        assert_eq!(
            committable_hook_path(default.path(), &hook),
            None,
            "`.git/hooks/pre-commit` is not a tracked file — an inert `.git`-internal \
             pathspec entry must never be produced",
        );

        // (2) An in-worktree `core.hooksPath`, in its relative form: an ordinary
        //     working-tree file, and the shape the premise was blind to.
        let in_tree = TempDir::new();
        git(in_tree.path(), &["init", "-q"]);
        git(in_tree.path(), &["config", "core.hooksPath", "my-hooks"]);
        let hook = install_precommit_hook(in_tree.path(), jigc).expect("install");
        assert_eq!(
            committable_hook_path(in_tree.path(), &hook).as_deref(),
            Some("my-hooks/pre-commit"),
            "an in-worktree `core.hooksPath` hook is committable, named as the FILE \
             (never the dir, which may hold the user's own hooks)",
        );

        // (3) A `core.hooksPath` outside the repo: in no working tree at all.
        let outside_repo = TempDir::new();
        let outside_hooks = TempDir::new();
        git(outside_repo.path(), &["init", "-q"]);
        git(
            outside_repo.path(),
            &[
                "config",
                "core.hooksPath",
                outside_hooks.path().to_str().expect("utf-8 temp path"),
            ],
        );
        let hook = install_precommit_hook(outside_repo.path(), jigc).expect("install");
        assert_eq!(
            committable_hook_path(outside_repo.path(), &hook),
            None,
            "a hooks dir outside the repo is not committable at all",
        );

        // (4) A linked worktree: the hook resolves to the main checkout's COMMON hooks
        //     dir, outside the tree this worktree commits from.
        let main = TempDir::new();
        git(main.path(), &["init", "-q"]);
        git_identity(main.path());
        git(
            main.path(),
            &["commit", "-q", "--allow-empty", "-m", "init"],
        );
        let linked = main.path().join("linked");
        git(
            main.path(),
            &[
                "worktree",
                "add",
                "-q",
                linked.to_str().expect("utf-8 path"),
            ],
        );
        let hook = install_precommit_hook(&linked, jigc).expect("install");
        assert_eq!(
            committable_hook_path(&linked, &hook),
            None,
            "the common hooks dir a linked worktree resolves to is outside its tree",
        );

        // (5) A `core.hooksPath` inside a **submodule** — shared hooks vendored as a
        //     submodule, a live pattern. Under the root, outside git's dirs, and still
        //     not this index's to take: `git add` refuses it fatally, which would take
        //     the WHOLE install commit down with it.
        let source = TempDir::new();
        git(source.path(), &["init", "-q"]);
        git_identity(source.path());
        std::fs::create_dir_all(source.path().join("hooks")).expect("create the source hooks dir");
        std::fs::write(source.path().join("hooks/keep"), "x\n").expect("seed the hooks dir");
        git(source.path(), &["add", "-A"]);
        git(source.path(), &["commit", "-q", "-m", "hooks"]);
        let outer = TempDir::new();
        git(outer.path(), &["init", "-q"]);
        git_identity(outer.path());
        git(
            outer.path(),
            &["commit", "-q", "--allow-empty", "-m", "init"],
        );
        git(
            outer.path(),
            &[
                "-c",
                "protocol.file.allow=always",
                "submodule",
                "add",
                "-q",
                source.path().to_str().expect("utf-8 temp path"),
                "shared-hooks",
            ],
        );
        git(
            outer.path(),
            &["config", "core.hooksPath", "shared-hooks/hooks"],
        );
        let hook = install_precommit_hook(outer.path(), jigc).expect("install");
        assert_eq!(
            committable_hook_path(outer.path(), &hook),
            None,
            "a hooks dir inside a submodule belongs to the submodule's index, not this one",
        );

        // (6) A `core.hooksPath` inside an **embedded, unregistered** git repo — the same
        //     ownership question with no `.gitmodules` entry. Here `git add` exits 0 while
        //     staging nothing, and the pathspec-limited `git commit` then fails with
        //     "did not match any file(s) known to git".
        let host = TempDir::new();
        git(host.path(), &["init", "-q"]);
        git_identity(host.path());
        git(
            host.path(),
            &["commit", "-q", "--allow-empty", "-m", "init"],
        );
        git(host.path(), &["init", "-q", "nested"]);
        git(host.path(), &["config", "core.hooksPath", "nested/hooks"]);
        let hook = install_precommit_hook(host.path(), jigc).expect("install");
        assert_eq!(
            committable_hook_path(host.path(), &hook),
            None,
            "a hooks dir inside an embedded repo belongs to that repo, not this index",
        );

        // (7) The same submodule as (5), **registered but not checked out** — a plain
        //     `git clone` without `--recursive`, reproduced here by `submodule deinit`
        //     (identical shape: a gitlink in the index over an empty directory). The
        //     filesystem cannot answer the ownership question at all: there is no `.git`
        //     inside, so `rev-parse --show-toplevel` from the hook's dir answers THIS
        //     root. Only the index knows, and `git add` still refuses fatally.
        git(
            outer.path(),
            &["submodule", "deinit", "-f", "--", "shared-hooks"],
        );
        // The fixture's premise, asserted rather than assumed: an empty directory whose
        // path the index holds as a gitlink.
        assert!(
            !outer.path().join("shared-hooks/.git").exists(),
            "the deinit'd submodule must have no `.git` — that is what blinds the \
             filesystem test",
        );
        assert!(
            git_str(outer.path(), &["ls-files", "--stage", "--", "shared-hooks"])
                .starts_with("160000 "),
            "the index must still hold the submodule's gitlink",
        );
        let hook = install_precommit_hook(outer.path(), jigc).expect("install");
        assert_eq!(
            committable_hook_path(outer.path(), &hook),
            None,
            "a hooks dir inside a NOT-CHECKED-OUT submodule is still the submodule's — \
             the index says so even though the filesystem cannot",
        );

        // …and a submodule that is merely a SIBLING under a shared parent must not drag
        // an otherwise committable hook down with it: the ancestor pathspec matches the
        // gitlink, but the gitlink does not contain the hook.
        let sibling = TempDir::new();
        git(sibling.path(), &["init", "-q"]);
        git_identity(sibling.path());
        git(
            sibling.path(),
            &["commit", "-q", "--allow-empty", "-m", "init"],
        );
        git(sibling.path(), &["config", "core.hooksPath", "my-hooks"]);
        std::fs::create_dir_all(sibling.path().join("my-hooks"))
            .expect("create the in-tree hooks dir");
        git(
            sibling.path(),
            &[
                "-c",
                "protocol.file.allow=always",
                "submodule",
                "add",
                "-q",
                source.path().to_str().expect("utf-8 temp path"),
                "my-hooks/vendored",
            ],
        );
        let hook = install_precommit_hook(sibling.path(), jigc).expect("install");
        assert_eq!(
            committable_hook_path(sibling.path(), &hook).as_deref(),
            Some("my-hooks/pre-commit"),
            "a submodule BESIDE the hook is not an ancestor of it — the hook stays \
             committable",
        );
    }

    /// The `pre-commit` hook is a **soft** member of the install pathspec: a `git add`
    /// refusal the hook entry is responsible for costs the hook's membership, not the
    /// whole install commit — and when the hook is *not* the cause, the refusal is still
    /// loud (the fallback must not swallow a real one).
    ///
    /// Forced with a **sparse-checkout** that excludes the in-tree hooks dir: a real,
    /// un-enumerated shape that no location or index test catches — `committable_hook_path`
    /// says "committable", and `git add` refuses anyway (verified live, git 2.53). That is
    /// exactly the case the soft membership exists for, so it is what pins it.
    #[test]
    fn a_refused_hook_entry_costs_the_hook_not_the_install_commit() {
        let dir = TempDir::new();
        git(dir.path(), &["init", "-q"]);
        git_identity(dir.path());
        git(dir.path(), &["commit", "-q", "--allow-empty", "-m", "init"]);
        git(dir.path(), &["config", "core.hooksPath", "my-hooks"]);
        std::fs::write(dir.path().join("CLAUDE.md"), "x\n").expect("write an install file");
        let hook = install_precommit_hook(dir.path(), Path::new("/abs/bin/jigc")).expect("install");
        // Everything but the hooks dir is in the sparse cone.
        git(dir.path(), &["sparse-checkout", "init", "--cone"]);
        git(dir.path(), &["sparse-checkout", "set", ".claude"]);
        // The premise: the discriminator admits the hook, and git refuses it anyway.
        assert_eq!(
            committable_hook_path(dir.path(), &hook).as_deref(),
            Some("my-hooks/pre-commit"),
            "the fixture's premise: every committability test says yes",
        );
        assert!(
            matches!(
                stage_paths(dir.path(), &["my-hooks/pre-commit".to_string()]),
                StageOutcome::Refused(_),
            ),
            "the fixture's premise: git refuses the hook anyway",
        );

        let outcome = commit_install(
            dir.path(),
            "CLAUDE.md",
            ".claude/settings.json",
            false,
            &hook,
            None,
        )
        .expect("a refusal the hook caused must not sink the install commit");

        assert!(
            matches!(outcome.commit, InstallCommit::Committed(_)),
            "the install must still be committed; got {:?}",
            outcome.commit,
        );
        // …and the outcome SAYS the hook did not ride it. This is the cell no
        // committability test can predict — every one of them called the hook committable
        // and git refused it anyway — so the summary's clause is keyed on the pathspec the
        // commit was made from rather than on `committable_hook_path`'s answer alone.
        assert!(
            !outcome.hook_committed,
            "a hook git refused is not in the install commit, whatever the tests said",
        );
        let committed = git_str(dir.path(), &["show", "--name-only", "--format=", "HEAD"]);
        assert!(
            committed.lines().any(|p| p == "CLAUDE.md"),
            "the install files must be in the commit; got:\n{committed}",
        );
        assert!(
            !committed.lines().any(|p| p == "my-hooks/pre-commit"),
            "the dropped hook must not be in the commit; got:\n{committed}",
        );

        // The fallback is a retry, not a swallow: a refusal that survives dropping the
        // hook is still surfaced loudly.
        std::fs::write(dir.path().join(".git/index.lock"), "").expect("hold the index lock");
        std::fs::write(dir.path().join("CLAUDE.md"), "y\n").expect("dirty an install file");
        let rejection = commit_install(
            dir.path(),
            "CLAUDE.md",
            ".claude/settings.json",
            false,
            &hook,
            None,
        )
        .expect_err("a refusal the hook did NOT cause must stay loud");
        assert!(
            matches!(rejection, InstallCommitRejection::Stage(_)),
            "the surviving refusal is the staging one; got {rejection:?}",
        );
    }

    /// A repo with a pre-existing `pre-commit` keeps the original content
    /// **verbatim** AND gains our sentinel block (non-destructive — the existing
    /// hook is preserved/wrapped, never clobbered).
    #[test]
    fn install_precommit_preserves_existing_hook() {
        let dir = TempDir::new();
        git(dir.path(), &["init", "-q"]);
        let hook = dir.path().join(".git/hooks/pre-commit");
        let existing = "#!/bin/sh\n# someone's hand-rolled hook\necho hello\nexit 0\n";
        std::fs::write(&hook, existing).expect("seed an existing pre-commit");

        install_precommit_hook(dir.path(), Path::new("/abs/bin/jigc")).expect("install succeeds");

        let after = std::fs::read_to_string(&hook).expect("hook still present");
        // The foreign shebang stays first; the jigc block is spliced in *after* it and
        // *before* the foreign body (so the backstop runs before a foreign `exit`), so
        // the foreign body is preserved verbatim though the file is no longer one
        // contiguous run.
        assert!(
            after.starts_with("#!/bin/sh\n"),
            "the foreign shebang must stay first",
        );
        assert!(
            after.contains("# someone's hand-rolled hook\necho hello\nexit 0\n"),
            "the foreign hook body must be preserved verbatim",
        );
        assert!(
            after.contains(PRECOMMIT_SENTINEL),
            "the wrapped hook must also carry our sentinel block",
        );
        assert!(
            after.find(PRECOMMIT_SENTINEL).unwrap() < after.find("echo hello").unwrap(),
            "the jigc block must run before the foreign hook body",
        );
        assert_eq!(
            mode(&hook) & 0o100,
            0o100,
            "a wrapped hook stays executable"
        );

        // Idempotent over a pre-existing hook too: re-install is byte-identical and
        // does not stack a second sentinel block.
        let once = after.clone();
        install_precommit_hook(dir.path(), Path::new("/abs/bin/jigc")).expect("re-install");
        let twice = std::fs::read_to_string(&hook).expect("hook present");
        assert_eq!(
            twice, once,
            "re-installing over a wrapped hook is byte-identical"
        );
        assert_eq!(
            twice.matches(PRECOMMIT_SENTINEL).count(),
            1,
            "the sentinel block must appear exactly once after a re-install",
        );
    }

    /// A pre-existing **foreign** hook that happens to contain jigc's start-sentinel
    /// string on a line but has NO matching end-marker must be treated as foreign and
    /// **preserved**, not mistaken for a wholly-jigc standalone hook and overwritten
    /// (synthetic data-loss). A block counts as jigc-managed only when BOTH the start
    /// sentinel AND the end marker bracket it (or it is byte-identical to a freshly
    /// rendered standalone hook).
    #[test]
    fn install_precommit_preserves_foreign_hook_carrying_start_sentinel_without_end_marker() {
        let dir = TempDir::new();
        git(dir.path(), &["init", "-q"]);
        let hook = dir.path().join(".git/hooks/pre-commit");
        // A foreign hook that mentions the start-sentinel string (e.g. a comment, or a
        // copied fragment) on its own line but carries no end marker and is NOT our
        // rendered body — its real work must survive a `jigc setup`.
        let foreign = format!(
            "#!/bin/sh\n{PRECOMMIT_SENTINEL}\n# foreign hook that references the sentinel\necho 'foreign work runs'\nexit 0\n",
        );
        std::fs::write(&hook, &foreign).expect("seed a foreign pre-commit");

        install_precommit_hook(dir.path(), Path::new("/abs/bin/jigc")).expect("install succeeds");

        let after = std::fs::read_to_string(&hook).expect("hook still present");
        assert!(
            after.contains("echo 'foreign work runs'\nexit 0\n"),
            "the foreign hook body must be preserved, not overwritten; got:\n{after}",
        );
        assert!(
            after.contains(PRECOMMIT_SENTINEL_END),
            "the foreign hook must be wrapped (gain the jigc block), not stripped to ours",
        );
    }

    /// [`remove_precommit_hook`] removes a **standalone** jigc hook entirely (the
    /// setup→uninstall round-trip for a repo that had no pre-existing hook), and a
    /// second remove over the now-absent file is a clean no-op. The removal keys on the
    /// sentinel structure, not the installing-`jigc` path, so it fires even though
    /// `uninstall` does not know that path.
    #[test]
    fn remove_precommit_removes_standalone_jigc_hook_then_is_a_no_op() {
        let dir = TempDir::new();
        git(dir.path(), &["init", "-q"]);
        let hook = dir.path().join(".git/hooks/pre-commit");

        install_precommit_hook(dir.path(), Path::new("/some/other/bin/jigc"))
            .expect("install a standalone hook");
        assert!(hook.exists(), "sanity: setup wrote the standalone hook");

        remove_precommit_hook(dir.path()).expect("remove the standalone hook");
        assert!(
            !hook.exists(),
            "a standalone jigc pre-commit hook must be removed entirely",
        );

        // Idempotent: a second remove over the absent file is a clean no-op.
        remove_precommit_hook(dir.path()).expect("second remove is a no-op");
        assert!(!hook.exists(), "the removed hook stays absent");
    }

    /// [`remove_precommit_hook`] over a **wrapped** foreign hook prunes only the jigc
    /// block (bracketed by the start/end sentinels), restoring the foreign hook's real
    /// work — the setup→uninstall round-trip for a repo whose pre-existing hook setup
    /// wrapped. The RED obligation: a setup-wrapped foreign pre-commit hook is restored.
    #[test]
    fn remove_precommit_restores_wrapped_foreign_hook() {
        let dir = TempDir::new();
        git(dir.path(), &["init", "-q"]);
        let hook = dir.path().join(".git/hooks/pre-commit");
        let foreign = "#!/bin/sh\n# someone's hand-rolled hook\necho hello\nexit 0\n";
        std::fs::write(&hook, foreign).expect("seed a foreign pre-commit");

        // setup wraps the foreign hook (jigc block spliced in, foreign body preserved).
        install_precommit_hook(dir.path(), Path::new("/abs/bin/jigc")).expect("install wraps");
        let wrapped = std::fs::read_to_string(&hook).expect("wrapped hook present");
        assert!(
            wrapped.contains(PRECOMMIT_SENTINEL) && wrapped.contains(PRECOMMIT_SENTINEL_END),
            "sanity: the wrapped hook carries both sentinels",
        );

        remove_precommit_hook(dir.path()).expect("remove the jigc block");
        let after = std::fs::read_to_string(&hook).expect("foreign hook restored, not deleted");
        assert!(
            !after.contains(PRECOMMIT_SENTINEL) && !after.contains(PRECOMMIT_SENTINEL_END),
            "the jigc block (both sentinels) must be gone; got:\n{after}",
        );
        assert_eq!(
            after, foreign,
            "the foreign hook must be restored to its pre-wrap bytes; got:\n{after}",
        );

        // Idempotent: a second remove over the now-foreign-only hook is byte-identical.
        remove_precommit_hook(dir.path()).expect("second remove is a no-op");
        assert_eq!(
            std::fs::read_to_string(&hook).expect("hook present"),
            foreign,
            "a second remove leaves the restored foreign hook untouched",
        );
    }

    /// [`remove_precommit_hook`] leaves a **purely foreign** hook (no jigc sentinel)
    /// untouched — the never-installed case, and the data-loss guard mirror of
    /// [`strip_managed_block`].
    #[test]
    fn remove_precommit_leaves_foreign_hook_untouched() {
        let dir = TempDir::new();
        git(dir.path(), &["init", "-q"]);
        let hook = dir.path().join(".git/hooks/pre-commit");
        let foreign = "#!/bin/sh\n# a foreign hook jigc never touched\necho work\nexit 0\n";
        std::fs::write(&hook, foreign).expect("seed a foreign pre-commit");

        remove_precommit_hook(dir.path()).expect("remove over a foreign hook");
        assert_eq!(
            std::fs::read_to_string(&hook).expect("hook present"),
            foreign,
            "a foreign hook must be left byte-untouched",
        );
    }

    /// The embedded `doc-code` probe is non-empty and begins with a native
    /// executable magic (ELF on Linux / `0xFEEDFACE`-family Mach-O on macOS). This
    /// is the cheap proof the `(I)`-pick build-ordering held: `build.rs` actually
    /// built and copied the probe into `OUT_DIR` *before* `include_bytes!` expanded,
    /// so a real, runnable executable rides in the `jigc` binary (not a stale/empty
    /// placeholder).
    #[test]
    fn embedded_doc_code_probe_is_a_native_executable() {
        assert!(
            !DOC_CODE_PROBE.is_empty(),
            "the embedded doc-code probe must be non-empty",
        );
        let elf = DOC_CODE_PROBE.starts_with(&[0x7f, b'E', b'L', b'F']);
        // Mach-O: 32/64-bit, little/big-endian, and the fat (universal) magics.
        let macho = matches!(
            DOC_CODE_PROBE.get(..4),
            Some([0xFE, 0xED, 0xFA, 0xCE])
                | Some([0xCE, 0xFA, 0xED, 0xFE])
                | Some([0xFE, 0xED, 0xFA, 0xCF])
                | Some([0xCF, 0xFA, 0xED, 0xFE])
                | Some([0xCA, 0xFE, 0xBA, 0xBE])
                | Some([0xBE, 0xBA, 0xFE, 0xCA])
        );
        assert!(
            elf || macho,
            "the embedded doc-code probe must begin with a native executable magic; \
             got first bytes {:02x?}",
            &DOC_CODE_PROBE[..DOC_CODE_PROBE.len().min(4)],
        );
    }

    /// `extract_doc_code_probe` applies the **heal/upgrade** policy: it writes an
    /// executable `doc-code` sibling whose bytes equal the embedded copy when the
    /// target dir has none; a second extract over a byte-identical sibling is a no-op;
    /// a pre-existing sibling whose bytes **differ** (a stale post-upgrade probe, or a
    /// corrupt stub) is **overwritten** with the embedded copy (heal/upgrade).
    #[test]
    fn extract_writes_absent_noops_on_match_and_heals_different() {
        let dir = TempDir::new();

        // (i) Absent → write the embedded bytes, executable.
        extract_doc_code_probe(dir.path()).expect("extract into an empty dir succeeds");
        let probe = dir.path().join("doc-code");
        assert_eq!(
            std::fs::read(&probe).expect("the probe sibling was written"),
            DOC_CODE_PROBE,
            "the written sibling must be byte-identical to the embedded copy",
        );
        assert_eq!(
            mode(&probe) & 0o100,
            0o100,
            "the extracted probe must be owner-executable",
        );

        // (ii) Re-extract over a matching sibling → byte-identical no-op.
        extract_doc_code_probe(dir.path()).expect("re-extract succeeds");
        assert_eq!(
            std::fs::read(&probe).expect("the probe sibling still present"),
            DOC_CODE_PROBE,
            "a re-extract over a matching sibling must leave it byte-identical",
        );

        // (iii) A pre-existing DIFFERENT sibling (stale upgrade leftover / corrupt
        //       stub) is OVERWRITTEN with the embedded copy and made executable
        //       (heal/upgrade — a byte-mismatch is healed, never left to surface as a
        //       probe-integrity failure).
        let sentinel = b"#!/bin/sh\n# a stale / corrupt leftover probe\nexit 0\n";
        std::fs::write(&probe, sentinel).expect("seed a different sibling");
        extract_doc_code_probe(dir.path()).expect("extract over a different sibling succeeds");
        assert_eq!(
            std::fs::read(&probe).expect("the healed sibling is present"),
            DOC_CODE_PROBE,
            "a pre-existing byte-different sibling must be overwritten with the embedded copy",
        );
        assert_eq!(
            mode(&probe) & 0o100,
            0o100,
            "the healed probe must be owner-executable",
        );
    }

    /// A write failure surfaces a blocking `setup.*` finding carrying a route, not
    /// a panic and not a bare error. Here `CLAUDE.md` is a *directory*, so the
    /// line write fails.
    #[test]
    fn install_failure_yields_blocking_finding_with_route() {
        use engine::finding::Severity;

        let dir = TempDir::new();
        // Make the line target unwritable: a directory where a file must go.
        std::fs::create_dir_all(dir.path().join("CLAUDE.md")).expect("seed a directory");
        let profile = adapter::load_profile("claude-code").expect("the shipped profile loads");

        let finding = install(dir.path(), &profile).expect_err("the line write must fail");

        assert_eq!(finding.severity, Severity::Blocking);
        assert!(
            finding.code.starts_with("setup."),
            "the block code must be in the `setup.*` family; got `{}`",
            finding.code,
        );
        assert!(
            finding.route.is_some(),
            "a hard block must carry a route directing the next action",
        );
    }
}
