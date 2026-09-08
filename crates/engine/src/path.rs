//! **How a surface names a filesystem path** — the one home for law 1's printed-path rule
//! ([surface-contract.md](../../../design/surface-contract.md): *every printed path is
//! repo-real or a typed identity*; [write-commands.md](../../../design/write-commands.md)
//! states the same rule as *a surface prints no host filesystem*).
//!
//! It lives in the engine because **both crates compose surfaces that name a path**: the CLI's
//! doors, and the engine's own `store.*` read blocks, which ride the 1.0-pinned
//! `jigc doc show --format json` contract. A second copy in the engine would be a second home
//! for one rule, which is the shape M46/M48/M49 each had to sweep afterwards.
//!
//! This module reads no filesystem content and holds none: it renders a path it is handed
//! against a root it is handed (`canonicalize` is the one syscall, and only to reconcile the
//! two spellings of the same directory), so the engine's empty-by-invariant property is
//! untouched.

use std::path::Path;

/// A filesystem path as a **surface** may name it: repo-relative and `/`-separated when it
/// lies inside `repo_root`, `.` for the root itself, and the honest absolute for a path that
/// is genuinely outside the repository.
///
/// **The one home for a rule that had none** (M50 Increment 12 / T1; RC-m50 → N25). Law 1
/// says *every printed path is repo-real or a typed identity* — yet `strip_prefix(repo_root)`
/// was hand-written at seven sites, none of them a surface renderer, so the four
/// destroying/provisioning doors named their subject with the host path of the machine they
/// ran on: a locus that is not portable across the two checkouts of the same repo a fan-out is
/// made of, printed one line under a `Spawn:` line that spells the same path
/// `.jigc/worktrees/<id>`.
///
/// **It moved here from `cli::render` at the M50 completion audit** (finding 3), which found
/// the rule enforced at the four doors and nowhere else: `config set placement-root` printed
/// the host path from the *shared* trackability predicate that `rename` alone had been taught
/// to work around, and `store.not-found` carried one onto the pinned read contract from the
/// engine — a crate that could not reach the rule's home at all.
///
/// **Both spellings are tried, because the doors build their paths off a canonicalized home.**
/// `provision` and `discard` join onto `jigc_home.canonicalize()` to match the canonical paths
/// git stores at `worktree add` time, so on macOS the subject reads `/private/var/...` while
/// `repo_root` reads `/var/...` and the plain strip misses. The canonicalizing retry is what
/// makes the relative spelling reachable at all there — and the root-only retry after it is
/// what keeps it reachable for a path that no longer exists (a narration printed after the
/// removal), where `path.canonicalize()` itself fails.
///
/// **The absolute fallback is the honest answer, not a failure**: a path outside the
/// repository has no repo-relative spelling, and inventing one with `../..` would name a
/// location that means something different from every other cwd. The sites that reach this
/// helper knowing their subject is outside the repo say so where they call it.
pub fn repo_relative(repo_root: &Path, path: &Path) -> String {
    fn render(relative: &Path) -> String {
        let joined = relative
            .components()
            .map(|c| c.as_os_str().to_string_lossy().into_owned())
            .collect::<Vec<_>>()
            .join("/");
        if joined.is_empty() {
            // The root itself. `.` is the repo-real spelling of "here"; the empty string
            // names nothing, and a message that interpolates it reads as a missing value.
            ".".to_string()
        } else {
            joined
        }
    }

    if let Ok(relative) = path.strip_prefix(repo_root) {
        return render(relative);
    }
    if let (Ok(root), Ok(real)) = (repo_root.canonicalize(), path.canonicalize())
        && let Ok(relative) = real.strip_prefix(&root)
    {
        return render(relative);
    }
    if let Ok(root) = repo_root.canonicalize()
        && let Ok(relative) = path.strip_prefix(&root)
    {
        return render(relative);
    }
    path.to_string_lossy().into_owned()
}
