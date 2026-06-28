//! `jigc rename <old-slug> --to "<New Title>"` — the CLI-owned identity refactor
//! (M35 Increment 1, T2: the verb + the committed-in-place atomic transaction).
//!
//! A top-level, **task-less self-committing** direct-store op (the shape of `unmanage` /
//! `migrate-corpus`, [write-commands.md](../../../design/write-commands.md) → `jigc rename`):
//! a doc-level identity change runs through **one** explicit, atomic op so cross-refs never
//! dangle under the move. The CLI derives the new slug = [`engine::slug::slugify`] (`--slug`
//! overrides) — so a renamed doc is byte-identical to a freshly-created one of that title;
//! walks the target's **inverse edges** ([`engine::index::referrers_of`]) and repoints every
//! persisted referrer's ref-field old→new ([`engine::write::repoint_ref`], scalar replaced
//! whole / list-valued re-emitting the canonical whole list); rewrites the moved doc's `# H1`
//! to the new title; and `git mv`s old→new.
//!
//! It commits as **one atomic transaction** with a real [`rollback_rename`]: pre-image
//! capture of the old path + every referrer (restored via `git restore --staged --worktree`)
//! plus the file-state record (inside the rollback inventory, [DECISIONS.md](../../../DECISIONS.md)
//! 2026-06-28, pin I3). The verb runs its **own** pre-commit integrity assertion
//! ([`rebuild_committed`](engine::index::rebuild_committed) →
//! [`ref_resolves_store`](engine::index::ref_resolves_store)), refusing the commit on any
//! dangling ref; the user's git hooks run, never `--no-verify`.
//!
//! The up-front validation gate (collision / no-op reslug / mid-fan-out) is T3; the advisory
//! prose/unmanaged-mention report is T5. This module is the transaction core.

use anyhow::{Context, Result, anyhow, bail};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use engine::file_state::{FileStateRecord, hash_bytes};
use engine::index;
use engine::schema::Schema;
use engine::slug::slugify;

use crate::ingest::{load_schemas, require_project_layer};
use crate::pack::make_pack;
use crate::task::{git_commit, git_head, git_run, path_at_head};

/// The outcome of a rename: the old/new `<type>:<slug>` identities, the repo-relative
/// paths the move spanned, the new title, and the repointed referrers (each
/// `<from>#<relation>`, sorted).
#[derive(Clone, Debug, serde::Serialize)]
pub struct RenameReport {
    /// The old `<type>:<slug>` identity.
    pub from: String,
    /// The new `<type>:<slug>` identity.
    pub to: String,
    /// The repo-relative old path (before the move).
    pub old_path: String,
    /// The repo-relative new path (after the move).
    pub new_path: String,
    /// The new H1 title the moved doc carries.
    pub title: String,
    /// The persisted referrers repointed, each `<from>#<relation>`, sorted.
    pub referrers: Vec<String>,
}

/// Run `jigc rename <old_addr> --to <title>` (optional `--slug`) against `cwd`: load the
/// rename substrate (schemas, committed index, file-state), re-derive the new identity,
/// repoint every referrer + rewrite the moved doc's H1, `git mv`, and commit as one
/// atomic transaction with a real rollback on any pre-commit failure.
pub(crate) fn run(
    cwd: &Path,
    old_addr: &str,
    title: &str,
    slug_override: Option<&str>,
) -> Result<RenameReport> {
    let repo_root = require_project_layer(cwd)?;
    let pack = make_pack();
    let resolved =
        crate::start::resolve_severity_cascade(pack.as_ref(), &repo_root.join(".jigc/config"))?;
    let schemas = load_schemas(pack.as_ref(), &resolved)?;
    let schema_map: BTreeMap<String, Schema> =
        schemas.iter().map(|s| (s.ty.clone(), s.clone())).collect();

    let jigc_root = repo_root.join(".jigc");
    let head = git_head(&repo_root)?;
    let index = index::load_committed(&repo_root, &jigc_root, &schema_map, &head);

    // Resolve the target's identity + on-disk path.
    let (ty, old_slug) = parse_addr(old_addr)?;
    let dir = location_dir(&schema_map, &ty)?;
    let old_rel = format!("{dir}/{old_slug}.md");
    let old_abs = repo_root.join(&old_rel);
    if !old_abs.is_file() {
        bail!("no managed doc `{ty}:{old_slug}` to rename (expected at {old_rel})");
    }
    let old_source = std::fs::read_to_string(&old_abs)
        .with_context(|| format!("could not read the doc to rename at {old_rel}"))?;

    let new_slug = match slug_override {
        Some(s) => s.to_string(),
        None => slugify(title),
    };
    if new_slug.is_empty() {
        bail!("`--to {title:?}` slugs to nothing — pass an explicit `--slug`");
    }
    let new_rel = format!("{dir}/{new_slug}.md");
    let old_id = format!("{ty}:{old_slug}");
    let new_id = format!("{ty}:{new_slug}");

    // Compute every referrer's old→new rewrite (grouped per source doc, so a doc that
    // references the target through more than one relation rewrites once over a running
    // buffer). Sorted by `referrers_of`'s deterministic `(from, relation, to)` order.
    let mut by_from: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for edge in index::referrers_of(&index, &old_id) {
        by_from
            .entry(edge.from.clone())
            .or_default()
            .push(edge.relation.clone());
    }
    let mut referrer_writes: Vec<ReferrerWrite> = Vec::new();
    let mut referrer_labels: Vec<String> = Vec::new();
    for (from_id, relations) in &by_from {
        let (fty, fslug) = parse_addr(from_id)?;
        let fschema = schema_map
            .get(&fty)
            .ok_or_else(|| anyhow!("referrer `{from_id}` has an unknown doctype `{fty}`"))?;
        let fdir = location_dir(&schema_map, &fty)?;
        let frel = format!("{fdir}/{fslug}.md");
        let fabs = repo_root.join(&frel);
        let mut source = std::fs::read_to_string(&fabs)
            .with_context(|| format!("could not read the referrer at {frel}"))?;
        for relation in relations {
            source = engine::write::repoint_ref(fschema, &source, relation, &old_id, &new_id)
                .map_err(|e| anyhow!("could not repoint {from_id}#{relation}: {e:?}"))?;
            referrer_labels.push(format!("{from_id}#{relation}"));
        }
        referrer_writes.push(ReferrerWrite {
            rel: frel,
            abs: fabs,
            source,
        });
    }

    // The moved doc's new bytes — only the `# H1` line changes (the unified retitle).
    let new_source =
        rewrite_h1(&old_source, title).ok_or_else(|| anyhow!("the doc at {old_rel} has no H1"))?;

    // Capture the file-state record's pre-image (the gitignored cache is not git-tracked,
    // so `git restore` can't recover it — it rides the rollback inventory by bytes, pin I3).
    let fs_path = FileStateRecord::path_in(&jigc_root);
    let fs_pre = match std::fs::read(&fs_path) {
        Ok(bytes) => Some(bytes),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => None,
        Err(e) => return Err(e).context("could not read the file-state record"),
    };
    // The tracked paths the transaction restores from HEAD on rollback (the old path +
    // every referrer — all committed at HEAD).
    let mut tracked_restore: Vec<String> = vec![old_rel.clone()];
    tracked_restore.extend(referrer_writes.iter().map(|w| w.rel.clone()));

    // The transaction proper: any failure rolls the store back byte-and-record identical.
    let outcome = apply_and_commit(
        &repo_root,
        &jigc_root,
        &schema_map,
        &head,
        &old_rel,
        &new_rel,
        &new_source,
        &referrer_writes,
    );
    if let Err(err) = outcome {
        rollback_rename(
            &repo_root,
            &tracked_restore,
            &new_rel,
            &fs_path,
            fs_pre.as_deref(),
        );
        return Err(err);
    }

    // Post-commit: invalidate the persisted index so the next read rebuilds against the
    // new HEAD (the finalize post-commit step; best-effort — the stale stamp self-heals).
    let _ = index::invalidate(&jigc_root);

    referrer_labels.sort();
    Ok(RenameReport {
        from: old_id,
        to: new_id,
        old_path: old_rel,
        new_path: new_rel,
        title: title.to_string(),
        referrers: referrer_labels,
    })
}

/// A referrer's computed rewrite: its repo-relative + absolute path and the new bytes.
struct ReferrerWrite {
    rel: String,
    abs: PathBuf,
    source: String,
}

/// Apply the rename mutations on disk, run the pre-commit integrity assertion, re-baseline
/// file-state, and commit — the inside of the transaction. Returns `Err` on any failure
/// (a `git mv` error, a dangling-ref integrity violation, or a hook/commit rejection); the
/// caller rolls back on `Err`.
#[allow(clippy::too_many_arguments)]
fn apply_and_commit(
    repo_root: &Path,
    jigc_root: &Path,
    schema_map: &BTreeMap<String, Schema>,
    head: &str,
    old_rel: &str,
    new_rel: &str,
    new_source: &str,
    referrer_writes: &[ReferrerWrite],
) -> Result<()> {
    // 1. Write each referrer's repointed bytes.
    for write in referrer_writes {
        std::fs::write(&write.abs, &write.source)
            .with_context(|| format!("could not write the repointed referrer at {}", write.rel))?;
    }
    // 2. Move the doc (stages the rename), then rewrite its H1 at the new path.
    git_run(repo_root, &["mv", old_rel, new_rel])?;
    std::fs::write(repo_root.join(new_rel), new_source)
        .with_context(|| format!("could not write the retitled doc at {new_rel}"))?;
    // 3. Stage the content changes (the move is staged; the H1 + referrer edits are not).
    git_run(repo_root, &["add", "--", new_rel])?;
    for write in referrer_writes {
        git_run(repo_root, &["add", "--", &write.rel])?;
    }

    // 4. The verb's own pre-commit integrity assertion: rebuild the committed index off the
    // mutated tree and refuse the commit if any cross-ref dangles (the rename must repoint
    // every referrer in lockstep). Reuses the store-scope `ref_resolves_store` backstop.
    let rebuilt = index::rebuild_committed(repo_root, schema_map, head);
    let dangling = index::ref_resolves_store(&rebuilt, repo_root, schema_map);
    if !dangling.is_empty() {
        bail!(
            "rename would leave {} dangling cross-reference(s) — refusing to commit",
            dangling.len()
        );
    }

    // 5. Re-baseline file-state inside the boundary (forget old, record new + every
    // rewritten referrer), so the renamed store is in-sync on the next sweep.
    let mut record = FileStateRecord::load(jigc_root)
        .with_context(|| format!("could not load the file-state record under {jigc_root:?}"))?;
    record.forget(old_rel);
    record.record(new_rel.to_string(), hash_bytes(new_source.as_bytes()));
    for write in referrer_writes {
        record.record(write.rel.clone(), hash_bytes(write.source.as_bytes()));
    }
    record
        .save(jigc_root)
        .with_context(|| format!("could not save the file-state record under {jigc_root:?}"))?;

    // 6. Commit (the user's hooks run, never `--no-verify`). A rejected hook bails here and
    // the caller rolls back.
    let msg_path = std::env::temp_dir().join(format!("jigc-rename-msg-{}", std::process::id()));
    std::fs::write(
        &msg_path,
        format!("rename {old_rel} -> {new_rel}\n\nRepoint every persisted referrer in lockstep.\n"),
    )
    .context("could not write the rename commit message")?;
    let commit = git_commit(repo_root, &msg_path);
    let _ = std::fs::remove_file(&msg_path);
    commit?;
    Ok(())
}

/// Roll the store back to its pre-rename state on a pre-commit failure: restore every
/// tracked path (the old doc + referrers) from HEAD, undo the move's landing, and restore
/// the gitignored file-state record from its captured pre-image bytes. Best-effort — a
/// failure is swallowed (the commit did not land, so the worst case is a stray file the
/// next op overwrites), mirroring `rollback_promotions`.
fn rollback_rename(
    repo_root: &Path,
    tracked_restore: &[String],
    new_rel: &str,
    fs_path: &Path,
    fs_pre: Option<&[u8]>,
) {
    // The old path + every referrer are committed at HEAD — `git restore --staged
    // --worktree` brings their index + worktree bytes back to HEAD (= pre-rename).
    for rel in tracked_restore {
        let _ = git_run(repo_root, &["restore", "--staged", "--worktree", rel]);
    }
    // The move's landing has no HEAD content: unstage it and drop the worktree copy.
    let _ = git_run(repo_root, &["restore", "--staged", new_rel]);
    if !path_at_head(repo_root, new_rel) {
        let _ = std::fs::remove_file(repo_root.join(new_rel));
    }
    // The file-state cache is not git-tracked: restore its captured pre-image (or remove
    // it when it did not exist before the transaction).
    match fs_pre {
        Some(bytes) => {
            let _ = std::fs::write(fs_path, bytes);
        }
        None => {
            let _ = std::fs::remove_file(fs_path);
        }
    }
}

/// Split a `<type>:<slug>` address into its parts. Rejects a missing `:` or an empty half.
fn parse_addr(addr: &str) -> Result<(String, String)> {
    let (ty, slug) = addr
        .split_once(':')
        .ok_or_else(|| anyhow!("`{addr}` is not a `<type>:<slug>` address"))?;
    if ty.is_empty() || slug.is_empty() {
        bail!("`{addr}` is not a `<type>:<slug>` address");
    }
    Ok((ty.to_string(), slug.to_string()))
}

/// The cascade-resolved on-disk directory (docs-root prefixed, trailing slash trimmed)
/// for `ty`. Errors on an unknown doctype or a transient (location-less) one.
fn location_dir(schema_map: &BTreeMap<String, Schema>, ty: &str) -> Result<String> {
    let schema = schema_map
        .get(ty)
        .ok_or_else(|| anyhow!("unknown doctype `{ty}`"))?;
    let location = schema.location.as_deref().ok_or_else(|| {
        anyhow!("`{ty}` is a transient doctype — it has no persisted file to rename")
    })?;
    Ok(location.trim_end_matches('/').to_string())
}

/// Rewrite the document's `# H1` title to `new_title`, preserving every other byte. The
/// H1 is the first ATX level-1 heading (`# `) outside a fenced code block (the document
/// title the writer renders from `id-from`). Returns `None` if the doc has no H1.
fn rewrite_h1(source: &str, new_title: &str) -> Option<String> {
    let mut out = String::with_capacity(source.len());
    let mut in_fence = false;
    let mut done = false;
    for line in source.split_inclusive('\n') {
        let body = line.strip_suffix('\n').unwrap_or(line);
        if !done {
            if body.starts_with("```") || body.starts_with("~~~") {
                in_fence = !in_fence;
            } else if !in_fence && body.starts_with("# ") {
                let nl = if line.ends_with('\n') { "\n" } else { "" };
                out.push_str(&format!("# {new_title}{nl}"));
                done = true;
                continue;
            }
        }
        out.push_str(line);
    }
    done.then_some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rewrite_h1_swaps_only_the_title_line() {
        let src = "---\nstatus: accepted\n---\n\n# Old title\n\n## Context\n\nProse.\n";
        let out = rewrite_h1(src, "New title").expect("an H1 is present");
        assert_eq!(
            out,
            "---\nstatus: accepted\n---\n\n# New title\n\n## Context\n\nProse.\n"
        );
    }

    #[test]
    fn rewrite_h1_ignores_h2_and_fenced_hash_lines() {
        // A `# ` inside a code fence must not be mistaken for the H1.
        let src = "```\n# not a heading\n```\n\n# Real title\n\n## Section\n";
        let out = rewrite_h1(src, "New").expect("the real H1 is found past the fence");
        assert_eq!(out, "```\n# not a heading\n```\n\n# New\n\n## Section\n");
    }

    #[test]
    fn rewrite_h1_none_without_an_h1() {
        assert!(rewrite_h1("## only an h2\n\nbody\n", "X").is_none());
    }

    #[test]
    fn parse_addr_rejects_malformed() {
        assert!(parse_addr("adr:slug").is_ok());
        assert!(parse_addr("noslug").is_err());
        assert!(parse_addr(":slug").is_err());
        assert!(parse_addr("adr:").is_err());
    }
}
