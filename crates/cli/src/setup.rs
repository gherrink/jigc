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
/// JSON report on stdout carries a `doc-code` content finding (a `"probe":
/// "doc-code"` entry); on a clean store, a not-a-jigc-project, a probe-missing run,
/// any `pack-probe-integrity` meta-finding, or a `jigc` that is absent / fails to
/// run, it **prints nothing and exits 0**.
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
         # committed store and prints a warning ONLY on a real doc-code content\n\
         # finding. Always exits 0 — it never blocks the commit. Keys on the\n\
         # findings, never on the exit code (the exit code is wrong-way-round).\n\
         \n\
         jigc='{jigc}'\n\
         \n\
         # Run the sweep, capturing stdout only. If jigc is absent or fails to run,\n\
         # `report` is empty and the doc-code probe never matches -> silent exit 0.\n\
         report=\"$(\"$jigc\" validate --format json 2>/dev/null)\"\n\
         \n\
         # Warn IFF the report carries a doc-code CONTENT finding. A clean store\n\
         # (empty findings), a not-a-jigc-project / probe-missing run (an `error`\n\
         # envelope on stderr, nothing matching here on stdout), and any\n\
         # `pack-probe-integrity` meta-finding (probe != doc-code) all fall through\n\
         # to a silent exit 0. The match is whitespace-tolerant after the colon (a\n\
         # POSIX ERE) so it survives compact or differently-spaced JSON — it keys on\n\
         # the `\"probe\": \"doc-code\"` finding, never on the pretty-printer's spacing.\n\
         if printf '%s' \"$report\" | grep -Eq '\"probe\"[[:space:]]*:[[:space:]]*\"doc-code\"'; then\n\
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
pub fn install_precommit_hook(repo_root: &Path, jigc_path: &Path) -> std::io::Result<()> {
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
    make_executable(&hook)
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
const SETUP_ASSISTANT: &str = "claude-code";

/// The result of a `jigc setup` install: the located repo root and the profile's
/// two host targets, so the dispatcher can render a precise success summary.
#[derive(Debug)]
pub struct SetupSummary {
    /// The repo-root-relative always-loaded file the bootstrap reference was
    /// injected into.
    pub line_file: String,
    /// The repo-root-relative settings file the allowlist was merged into.
    pub allowlist_file: String,
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
    let ctx = locate::locate(start).map_err(|err| {
        Finding::block(
            "setup.repo-root",
            format!("cannot locate the repository root: {err:#}"),
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
    //     it and the marker is NOT written (the two cannot be combined — see
    //     [`write_compose_marker`]); setup says so rather than silently choosing.
    let marker_wired = write_compose_marker(repo_root).map_err(|err| {
        Finding::block(
            "setup.compose-marker",
            format!(
                "cannot write the `compose-embedded-methodology` marker into `.jigc/config/packs.yaml`: {err}"
            ),
            "ensure `.jigc/config/` is writable, then re-run `jigc setup`",
        )
    })?;
    if !marker_wired {
        eprintln!(
            "warning: `.jigc/config/packs.yaml` already lists packs, so the embedded methodology \
             pack was left unwired (a `packs:` list cannot be combined with \
             `compose-embedded-methodology: true` — `design/multi-pack.md` → Embedded second pack)\n\
             route: to compose the embedded `[dev ▸ methodology]` pair instead, remove the \
             `packs:` list from `.jigc/config/packs.yaml` and re-run `jigc setup`"
        );
    }

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
    install_precommit_hook(repo_root, &jigc_path).map_err(|err| {
        Finding::block(
            "setup.install-hook",
            format!("cannot install the `pre-commit` hook into the repo's hooks dir: {err}"),
            "ensure the repo's git hooks directory is writable, then re-run `jigc setup`",
        )
    })?;

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
    //    degrade gracefully — but a genuine commit *rejection* (e.g. no git identity)
    //    leaves the install staged-but-uncommitted, so it fails loudly with an actionable
    //    finding rather than masquerading as a clean success (mirrors `finalize`'s
    //    identical git-identity failure).
    let install_commit = commit_install(repo_root, &line_file, &allowlist_file, seeded_gitignore)
        .map_err(|git_err| {
        Finding::block(
            "setup.install-commit",
            format!(
                "the jigc install files were written and staged, but `git commit` was rejected \
                 (no install commit was made):\n{git_err}"
            ),
            "tell git who you are — set `git config user.email \"you@example.com\"` and \
             `git config user.name \"Your Name\"` — then re-run `jigc setup` to commit the \
             staged install files",
        )
    })?;

    Ok(SetupSummary {
        line_file,
        allowlist_file,
        install_commit,
    })
}

/// The repo-relative install files `jigc setup` itself writes that are meant to be
/// tracked in git — the committable install footprint, enumerated **explicitly** so the
/// install commit never sweeps the user's unrelated working-tree changes (a blanket
/// `git add -A` would). Deliberately excludes: the transient `.jigc/` working area
/// (`tasks/`/`index/`/`state/`, gitignored by setup's own `.jigc/.gitignore`), the
/// `.git/hooks/pre-commit` (outside the worktree, in git's control dir — never a tracked
/// file), and the machine-global `doc-code` probe (beside the binary, not in the repo).
fn install_tracked_paths(
    line_file: &str,
    allowlist_file: &str,
    seeded_gitignore: bool,
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
    paths
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
/// staged-but-orphaned; the commit is a convenience there). But a genuine commit
/// **rejection** (git ran and declined — e.g.
/// no `user.email`/`user.name`) leaves the install files staged-but-uncommitted, so it
/// returns `Err(<git's rejection>)` for [`install`] to surface as a loud blocking
/// finding rather than a silent skip behind a success banner. Uses `--no-verify`: the
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
) -> Result<InstallCommit, String> {
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
        _ => return Ok(InstallCommit::Skipped),
    }

    // Only the files setup itself wrote, and only those present + not gitignored.
    let paths: Vec<String> = install_tracked_paths(line_file, allowlist_file, seeded_gitignore)
        .into_iter()
        .filter(|p| repo_root.join(p).exists())
        .filter(|p| !git_path_ignored(repo_root, p))
        .collect();
    if paths.is_empty() {
        return Ok(InstallCommit::Skipped);
    }

    // Stage exactly those paths — never a blanket `git add -A`.
    let mut add: Vec<&str> = vec!["add", "--"];
    add.extend(paths.iter().map(String::as_str));
    match git_output(repo_root, add) {
        Some(out) if out.status.success() => {}
        _ => return Ok(InstallCommit::Skipped),
    }

    // Nothing staged among our paths (a re-run over an unchanged install) → clean no-op.
    // `git diff --cached --quiet -- <paths>` exits 0 (success) when there is no staged
    // diff for those paths; with no HEAD it diffs against the empty tree, so a first
    // install still reports changes.
    let mut diff: Vec<&str> = vec!["diff", "--cached", "--quiet", "--"];
    diff.extend(paths.iter().map(String::as_str));
    if git_output(repo_root, diff)
        .map(|o| o.status.success())
        .unwrap_or(false)
    {
        return Ok(InstallCommit::Nothing);
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
        Some(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            let stderr = String::from_utf8_lossy(&out.stderr);
            return Err(format!("{}{}", stdout.trim(), stderr.trim()));
        }
        // git could not be spawned at all — benign skip (the writes still succeeded).
        None => return Ok(InstallCommit::Skipped),
    }

    // Resolve the short sha of the commit just made, for the success surface.
    match git_output(repo_root, ["rev-parse", "--short", "HEAD"]) {
        Some(out) if out.status.success() => {
            let sha = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if sha.is_empty() {
                Ok(InstallCommit::Skipped)
            } else {
                Ok(InstallCommit::Committed(sha))
            }
        }
        _ => Ok(InstallCommit::Skipped),
    }
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
    /// Which of the six repo-local artifacts were **actually** present and removed —
    /// so the teardown summary reports the real removal set and never claims to have
    /// removed an already-absent artifact (M36 completion honesty fix; the runtime
    /// mirror of the `project-setup.md` G5 "exactly the enumerated set" correction).
    pub removed: RemovedArtifacts,
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
            || self.precommit)
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
/// Teardown / cleanup (G5), bullet (b)). Every settings removal is **surgical**: a
/// foreign permit / hook / deny entry sharing the file survives.
///
/// **Explicitly NOT** the machine-global `doc-code` probe sibling beside the `jigc`
/// binary — it is shared across every jigc repo on the machine, so deleting it would
/// break `jigc validate` for sibling repos (design-review B2). Machine-global removal
/// is `cargo uninstall jigc` + manual probe removal, never this per-project verb.
///
/// **Idempotent + non-destructive:** each step is independently a clean no-op when its
/// artifact is already absent — an already-removed `.jigc/`, a `CLAUDE.md` without the
/// section, an `allow`/`deny` array or `hooks` object without the jigc entry, and an
/// absent-or-foreign `pre-commit` hook — so a second `uninstall` exits 0 leaving the
/// (restored) host files byte-untouched. `Ok(summary)` on a clean teardown;
/// `Err(finding)` is a single blocking `uninstall.*` finding carrying a route — the
/// dispatcher renders it and exits non-zero.
pub fn run_uninstall(start: &Path) -> Result<UninstallSummary, Finding> {
    let ctx = locate::locate(start).map_err(|err| {
        Finding::block(
            "uninstall.repo-root",
            format!("cannot locate the repository root: {err:#}"),
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

    uninstall(&ctx.repo_root, &profile)
}

/// Reverse the repo-local install against `repo_root` with `profile`, mapping an IO
/// failure to a blocking `uninstall.*` finding with a route. The testable core of
/// [`run_uninstall`] (no location step). Each step is independently idempotent, so the
/// whole teardown is a clean no-op on a re-run.
fn uninstall(repo_root: &Path, profile: &AdapterProfile) -> Result<UninstallSummary, Finding> {
    // 1. Remove the whole `.jigc/` tree — the bootstrap `AGENT.md`, the cascade config
    //    layer, the compose marker, and the transient index/state working area, all at
    //    once. An already-absent tree is a clean no-op.
    let mut removed = RemovedArtifacts::default();

    let jigc_dir = repo_root.join(".jigc");
    if jigc_dir.exists() {
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

    // The machine-global `doc-code` probe sibling is deliberately left in place (B2):
    // it is shared across every jigc repo on the machine, so this per-project verb must
    // not delete it.

    Ok(UninstallSummary {
        line_file,
        allowlist_file,
        removed,
    })
}

/// The compose-marker key `pack::read_compose_marker` reads from `packs.yaml`.
const COMPOSE_MARKER_KEY: &str = "compose-embedded-methodology";

/// The listed-pack key `pack::read_pack_list` reads from the same `packs.yaml` — the
/// M14 pack-set the compose marker cannot be combined with (see
/// [`write_compose_marker`]).
const PACKS_LIST_KEY: &str = "packs";

/// Write the `compose-embedded-methodology: true` marker into the project layer's
/// `<repo_root>/.jigc/config/packs.yaml` (step 2b of [`install`]). Returns whether the
/// marker is wired — `false` means it was **deliberately not written** (below).
///
/// **A non-empty `packs:` list wins: the marker is not written** (M42 Inc 6 fix). The
/// marker composes the two *in-binary* packs and, per `design/multi-pack.md` →
/// Embedded second pack, cannot be combined with listed filesystem packs — the loader
/// refuses that pack-set loudly ([`crate::pack::make_pack`]). Writing the marker over
/// an operator's hand-written list therefore **manufactured an unsupported
/// configuration**: it used to make the listed packs silently inert (a drifted frozen
/// schema in them went unchecked at every door), and under the loader's refusal it
/// would brick every command in the project. So setup preserves the operator's
/// declared pack-set and leaves the embedded pair unwired; [`install`] says so on
/// stderr with the route to the other choice.
///
/// **Non-destructive:** an absent file is created carrying just the marker; an
/// existing marker-only / list-free `packs.yaml` is parsed and the marker set
/// alongside whatever else it carries (parse-mutate-serialize, the human's other keys
/// survive). A malformed `packs.yaml` is surfaced, never clobbered.
///
/// **Idempotent:** the serialized bytes are written only when they differ from what
/// is on disk, so a second `setup` over an already-marked file is a byte-identical
/// no-op (`serde_yaml_ng` serialization of a stable mapping is deterministic).
fn write_compose_marker(repo_root: &Path) -> std::io::Result<bool> {
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

    // The operator's declared pack-set is not setup's to override: a non-empty `packs:`
    // list and the marker cannot both hold, so leave the file exactly as authored.
    let lists_packs = mapping
        .get(Value::from(PACKS_LIST_KEY))
        .and_then(Value::as_sequence)
        .is_some_and(|packs| !packs.is_empty());
    if lists_packs {
        return Ok(false);
    }

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
    Ok(true)
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
    /// outputs across all five cases. It warns IFF the report carries a `"probe":
    /// "doc-code"` content finding — keyed on the finding, **never** on the exit
    /// code — and **always exits 0** (warn-only; it never blocks a commit).
    #[test]
    fn rendered_hook_warns_iff_doc_code_content_finding_always_exits_zero() {
        let dir = TempDir::new();

        // The `doc-code` content-finding marker the hook keys on, in the pretty-JSON
        // spacing `jigc validate --format json` emits (`render::json` → to_string_pretty).
        let warning = "doc<->code drift detected";

        // (1) Clean store: a report with an empty findings array, exit 0 -> silent.
        let clean = "{\n  \"schema_version\": 1,\n  \"findings\": []\n}";
        let jigc = write_fake_jigc(dir.path(), "jigc-clean", clean, "", 0);
        let (stderr, code) = run_rendered_hook(&jigc);
        assert_eq!(code, 0, "clean store must exit 0; stderr:\n{stderr}");
        assert!(
            !stderr.contains(warning),
            "a clean store must warn nothing; stderr:\n{stderr}",
        );

        // (2) A doc-code CONTENT finding (exit 0, per the detect-and-report rule) ->
        // the only case that warns. The probe field is the keyed surface.
        let drift = "{\n  \"schema_version\": 1,\n  \"findings\": [\n    {\n      \"severity\": \"blocking\",\n      \"probe\": \"doc-code\",\n      \"check\": \"symbol-exists\",\n      \"code\": \"doc-code.symbol-exists\",\n      \"message\": \"anchor crates/engine/src/cache.rs#evict_lru does not resolve\",\n      \"address\": \"decisions/cache.md\",\n      \"route\": null\n    }\n  ]\n}";
        let jigc = write_fake_jigc(dir.path(), "jigc-drift", drift, "", 0);
        let (stderr, code) = run_rendered_hook(&jigc);
        assert_eq!(
            code, 0,
            "a doc-code finding is warn-only — exit 0; stderr:\n{stderr}"
        );
        assert!(
            stderr.contains(warning),
            "a doc-code content finding MUST warn; stderr:\n{stderr}",
        );

        // (3) Not-a-jigc-project: the operational-error envelope on STDERR, a
        // non-zero exit. No `"probe": "doc-code"` on stdout -> silent, exit 0
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

        // (4) A `pack-probe-integrity` meta-finding (probe crashed): a finding is
        // present and the command exits non-zero, but its probe is NOT `doc-code`
        // -> silent, exit 0. The discipline must not warn on a meta-finding.
        let meta = "{\n  \"schema_version\": 1,\n  \"findings\": [\n    {\n      \"severity\": \"blocking\",\n      \"probe\": \"pack-probe-integrity\",\n      \"check\": \"crash\",\n      \"code\": \"pack-probe-integrity.crash\",\n      \"message\": \"the doc-code probe exited 2 without emitting JSON\",\n      \"route\": null\n    }\n  ]\n}";
        let jigc = write_fake_jigc(dir.path(), "jigc-meta", meta, "", 1);
        let (stderr, code) = run_rendered_hook(&jigc);
        assert_eq!(
            code, 0,
            "a probe-integrity run must still exit 0; stderr:\n{stderr}"
        );
        assert!(
            !stderr.contains(warning),
            "a pack-probe-integrity meta-finding must warn nothing (probe != doc-code); stderr:\n{stderr}",
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

        // (6) A doc-code CONTENT finding, but the JSON is emitted **compact** (no space
        // after the colons): `{"findings":[{"probe":"doc-code",...}]}`. The detection
        // must NOT couple to the pretty-print spacing — if `jigc validate --format json`
        // is ever emitted compact (or with different spacing), the hook must still warn,
        // not go silently dead and ship drift as a false-clean. Keyed on the finding,
        // robust to whitespace after the colon.
        let compact = "{\"schema_version\":1,\"findings\":[{\"severity\":\"blocking\",\"probe\":\"doc-code\",\"check\":\"symbol-exists\",\"code\":\"doc-code.symbol-exists\",\"message\":\"anchor crates/engine/src/cache.rs#evict_lru does not resolve\",\"address\":\"decisions/cache.md\",\"route\":null}]}";
        let jigc = write_fake_jigc(dir.path(), "jigc-compact", compact, "", 0);
        let (stderr, code) = run_rendered_hook(&jigc);
        assert_eq!(
            code, 0,
            "a compact doc-code finding is warn-only — exit 0; stderr:\n{stderr}"
        );
        assert!(
            stderr.contains(warning),
            "a compact (no-space-after-colon) doc-code content finding MUST still warn; stderr:\n{stderr}",
        );
    }

    /// Golden-lock the rendered `pre-commit` body for a fixed `jigc_path`: the
    /// sentinel marker is present, the **absolute** path is embedded (quoted),
    /// `--format json` is invoked, the discipline keys on `"probe": "doc-code"`
    /// whitespace-tolerantly (a POSIX ERE), and the script always `exit 0`. Pins the
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
             # committed store and prints a warning ONLY on a real doc-code content\n\
             # finding. Always exits 0 — it never blocks the commit. Keys on the\n\
             # findings, never on the exit code (the exit code is wrong-way-round).\n\
             \n\
             jigc='/abs/install/bin/jigc'\n\
             \n\
             # Run the sweep, capturing stdout only. If jigc is absent or fails to run,\n\
             # `report` is empty and the doc-code probe never matches -> silent exit 0.\n\
             report=\"$(\"$jigc\" validate --format json 2>/dev/null)\"\n\
             \n\
             # Warn IFF the report carries a doc-code CONTENT finding. A clean store\n\
             # (empty findings), a not-a-jigc-project / probe-missing run (an `error`\n\
             # envelope on stderr, nothing matching here on stdout), and any\n\
             # `pack-probe-integrity` meta-finding (probe != doc-code) all fall through\n\
             # to a silent exit 0. The match is whitespace-tolerant after the colon (a\n\
             # POSIX ERE) so it survives compact or differently-spaced JSON — it keys on\n\
             # the `\"probe\": \"doc-code\"` finding, never on the pretty-printer's spacing.\n\
             if printf '%s' \"$report\" | grep -Eq '\"probe\"[[:space:]]*:[[:space:]]*\"doc-code\"'; then\n\
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
