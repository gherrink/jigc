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
//! The up-front validation gate runs **before** the transaction (T3): a **dirty working tree**
//! blocks (a rename commits in place with no pathspec, so any pre-existing tracked change would
//! be swept into the one atomic rename commit — commit or stash first); a **collision** with a
//! *different* committed doc blocks (an identity refactor never silently suffixes); a **no-op
//! reslug** (the new title slugs to the doc's own current slug) degrades to a **retitle-only**
//! (rewrite the H1 + commit, no `git mv`, no referrer repoint); a **mid-fan-out** guard
//! blocks whenever any task working area *or* milestone is in-flight ([DECISIONS.md] 2026-06-28,
//! pins I2/I4); a **placement-singleton** reslug rejects with the real rule (identity fixed to
//! the type; retitle-only) and a **milestone-record** reslug refuses always — between milestones
//! too (M40 A4). The advisory prose/unmanaged-mention report is T5.

use anyhow::{Context, Result, anyhow, bail};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use engine::file_state::{FileStateRecord, hash_bytes};
use engine::index;
use engine::schema::Schema;
use engine::slug::slugify;

use crate::ingest::{load_schemas, require_project_layer};
use crate::pack::make_pack;
use crate::task::{git_capture, git_commit, git_head, git_run, git_status_entries, path_at_head};

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
    /// **Advisory** old-slug occurrences in managed-doc prose + unmanaged tracked files
    /// (each `<repo-rel-path>:<line>`, sorted) that the CLI **reports but never rewrites** —
    /// the determinism boundary's honest line (it rewrites only structured ref-fields and
    /// authors no prose). A word-boundary/token match, scoped to `git ls-files`. Never
    /// changes the verb's exit status; empty for a retitle-only (the slug is unchanged).
    pub prose_mentions: Vec<String>,
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
    let pack = make_pack()?;
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
    let old_rel = doc_path(&schema_map, &ty, &old_slug)?;
    let old_abs = repo_root.join(&old_rel);
    if !old_abs.is_file() {
        bail!(
            "no managed doc `{ty}:{old_slug}` to rename (expected at {old_rel})\n  route: check the id (or run `jigc describe` for the doctype surface)"
        );
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
    let new_rel = doc_path(&schema_map, &ty, &new_slug)?;
    let new_abs = repo_root.join(&new_rel);
    let old_id = format!("{ty}:{old_slug}");
    let new_id = format!("{ty}:{new_slug}");

    // The up-front validation gate (runs before any mutation):
    // (a) **clean-tree precondition** — a rename is a deliberate standalone op that commits
    //     in place with **no pathspec** (`git_commit` runs `git commit -F`, capturing the
    //     whole index), so any pre-existing tracked change would be swept into the "one
    //     atomic rename commit" (a staged change) or clobbered by the referrer rewrites (an
    //     unstaged edit to a managed doc). Refuse up front whenever the working tree carries
    //     any staged or unstaged **tracked** change, routing the user to commit or stash
    //     first. Untracked files are not part of the commit (the verb stages only the moved
    //     doc + its referrers, never `git add --all`), so they do not count — they are
    //     filtered out (`??`). (write-commands.md → `jigc rename` step 2, "tree clean".)
    let dirty: Vec<String> = git_status_entries(&repo_root)?
        .into_iter()
        .filter(|(code, _)| code != "??")
        .map(|(_, path)| path)
        .collect();
    if !dirty.is_empty() {
        bail!(
            "cannot rename with a dirty working tree — commit or stash your changes first \
             (a rename is a deliberate standalone op that commits in place): {}",
            dirty.join(", ")
        );
    }
    // (b) **mid-fan-out guard** — a rename changes the by-task-id join's same-doc-clash key
    //     and a task working area may hold an old-slug copy that would promote stale; block
    //     whenever any task working area *or* milestone is in-flight (the coarse guard, I4).
    if let Some(marker) = mid_fan_out_marker(&jigc_root) {
        bail!(
            "cannot rename while {marker} is in flight — finalize or discard it first \
             (a rename changes the by-task-id join key)"
        );
    }
    // (c) **no-op reslug** — when the new slug equals the doc's own current slug the identity
    //     is unchanged, so the rename degrades to a retitle-only (rewrite H1 + commit, no
    //     `git mv`, no referrer repoint — nothing dangles).
    let is_retitle = new_slug == old_slug;
    // (d) **placement-singleton reslug** — undefined, not merely blocked (M40 A4;
    //     write-commands.md → Placement singletons): a placement doctype's identity is fixed
    //     to its type — the singleton's slug IS the type id and the doc lives at its literal
    //     `placement.file` — so there is no reslug to perform. Reject with the real rule
    //     rather than falling through to (f)'s misleading collision text (`doc_path` resolves
    //     the placement literal ignoring the slug, so `new_abs == old_abs`: it is the SAME
    //     file, not a collision). Retitle-only (the degenerate arm (c)) stays supported.
    if !is_retitle && schema_map[ty.as_str()].placement.is_some() {
        bail!(
            "cannot reslug `{old_id}` — a placement singleton's identity is fixed to its type \
             (the slug IS the type id `{ty}` and the doc lives at the literal {old_rel}); only \
             a retitle is supported: pass a `--to` title that keeps the slug `{old_slug}`"
        );
    }
    // (e) **milestone-record reslug** — refused always, between milestones too (M40 A4.4;
    //     write-commands.md → Milestone-record reslug): the record's slug IS the milestone
    //     work-unit id — it keys `.jigc/milestones/<id>` and every milestone op — so a reslug
    //     would sever the committed record from its work unit. Keyed on the (necessarily
    //     committed) rename target's doctype, fresh-clone survivable — no workbench read; the
    //     coarse mid-fan-out guard (b) covers only the in-flight window.
    if !is_retitle && ty == crate::milestone::MILESTONE_RECORD_TYPE {
        bail!(
            "cannot reslug `{old_id}` — a milestone-record's slug IS its milestone \
             work-unit id (it keys `.jigc/milestones/{old_slug}` and every milestone op), \
             so a reslug would sever the committed record from its work unit; only a \
             retitle is supported: pass a `--to` title that keeps the slug `{old_slug}`"
        );
    }
    // (f) **collision** — a non-degenerate new slug must be free; a collision with a
    //     *different* committed doc blocks (an identity refactor, never the join's suffix).
    if !is_retitle && new_abs.is_file() {
        bail!("cannot rename to `{new_id}` — a different doc already exists at {new_rel}");
    }

    // Compute every referrer's old→new rewrite (grouped per source doc, so a doc that
    // references the target through more than one relation rewrites once over a running
    // buffer). Sorted by `referrers_of`'s deterministic `(from, relation, to)` order. A
    // retitle-only has no identity change, so it repoints nothing.
    let mut referrer_writes: Vec<ReferrerWrite> = Vec::new();
    let mut referrer_labels: Vec<String> = Vec::new();
    if !is_retitle {
        let mut by_from: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for edge in index::referrers_of(&index, &old_id) {
            by_from
                .entry(edge.from.clone())
                .or_default()
                .push(edge.relation.clone());
        }
        for (from_id, relations) in &by_from {
            let (fty, fslug) = parse_addr(from_id)?;
            let fschema = schema_map
                .get(&fty)
                .ok_or_else(|| anyhow!("referrer `{from_id}` has an unknown doctype `{fty}`"))?;
            let frel = doc_path(&schema_map, &fty, &fslug)?;
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

    // The advisory prose/unmanaged-mention report (the determinism boundary's honest line):
    // scan the *post-rename* tracked worktree for surviving old-slug occurrences — the CLI
    // has already rewritten every structured ref-field, so what remains is exactly the prose
    // + unmanaged mentions it cannot author. A retitle-only leaves the slug unchanged, so it
    // has nothing to report. Best-effort + advisory: it never blocks and never changes the
    // exit status.
    let prose_mentions = if is_retitle {
        Vec::new()
    } else {
        scan_prose_mentions(&repo_root, &old_slug)
    };

    referrer_labels.sort();
    Ok(RenameReport {
        from: old_id,
        to: new_id,
        old_path: old_rel,
        new_path: new_rel,
        title: title.to_string(),
        referrers: referrer_labels,
        prose_mentions,
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
    // 0. Capture the **pre-rename** dangling-edge set off the committed store *before* any
    // mutation (disk == HEAD here). The integrity gate (step 4) refuses only on a dangle the
    // rename itself *introduces* — a pre-existing dangle unrelated to the move must pass
    // through untouched (the M18/M19 masking-trap guard: a transaction is never blocked by
    // drift it did not cause; pre-existing rot stays a report-only `jigc validate` concern).
    let before = index::rebuild_committed(repo_root, schema_map, head);
    let before_dangling: BTreeSet<(String, String, String)> =
        index::dangling_edges(&before, repo_root, schema_map)
            .into_iter()
            .map(edge_key)
            .collect();

    // 1. Write each referrer's repointed bytes.
    for write in referrer_writes {
        std::fs::write(&write.abs, &write.source)
            .with_context(|| format!("could not write the repointed referrer at {}", write.rel))?;
    }
    // 2. Move the doc via the shared move primitive — `git mv` old→new (skipped for a
    // retitle-only, where old_rel == new_rel and there is no identity change) plus the
    // moved-doc file-state re-key (forget old, record new at the retitled bytes' hash) —
    // then rewrite its H1 at the (possibly unchanged) path. The primitive re-keys only the
    // moved doc; the referrers' re-keys are layered on in step 5 (the two record saves
    // compose byte-identically).
    crate::relocate::move_doc(
        repo_root,
        jigc_root,
        old_rel,
        new_rel,
        &hash_bytes(new_source.as_bytes()),
    )?;
    std::fs::write(repo_root.join(new_rel), new_source)
        .with_context(|| format!("could not write the retitled doc at {new_rel}"))?;
    // 3. Stage the content changes (the move is staged; the H1 + referrer edits are not).
    git_run(repo_root, &["add", "--", new_rel])?;
    for write in referrer_writes {
        git_run(repo_root, &["add", "--", &write.rel])?;
    }

    // 4. The verb's own pre-commit integrity assertion: rebuild the committed index off the
    // mutated tree and refuse the commit only on a dangle the rename *introduced* — an edge
    // dangling *after* the move that was *not* dangling *before* it. Scoped by the before/after
    // diff (step 0), so a pre-existing dangle unrelated to the rename never blocks the move
    // and is never misattributed to it (the M18/M19 masking-trap guard). A resolvable rename
    // repoints every referrer in lockstep, so it introduces zero new dangles.
    let rebuilt = index::rebuild_committed(repo_root, schema_map, head);
    let after_dangling = index::dangling_edges(&rebuilt, repo_root, schema_map);
    let introduced = introduced_dangles(&before_dangling, after_dangling);
    if !introduced.is_empty() {
        bail!(
            "rename would introduce {} dangling cross-reference(s) — refusing to commit: {}",
            introduced.len(),
            introduced.join(", "),
        );
    }

    // 5. Re-baseline the referrers' file-state inside the boundary (the moved doc's own
    // re-key already happened in the move primitive, step 2), so the renamed store is
    // in-sync on the next sweep.
    let mut record = FileStateRecord::load(jigc_root)
        .with_context(|| format!("could not load the file-state record under {jigc_root:?}"))?;
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

/// The first in-flight fan-out marker, if any — a task working area
/// (`<jigc>/tasks/<id>/`) or a milestone (`<jigc>/milestones/<id>/`), each labelled for the
/// block message. A rename mid-fan-out would change the by-task-id join's same-doc-clash key
/// and a working area may hold an old-slug copy that would promote stale, so the verb refuses
/// the identity op while either is live (the coarse guard, [DECISIONS.md] 2026-06-28 pin I4).
/// Both enumerations are sorted (the first id is deterministic), so the message is stable.
fn mid_fan_out_marker(jigc_root: &Path) -> Option<String> {
    if let Some(id) = engine::state::list_active_task_ids(jigc_root).first() {
        return Some(format!("task `{id}`"));
    }
    first_dir_name(&jigc_root.join("milestones")).map(|id| format!("milestone `{id}`"))
}

/// The lexicographically-first sub-directory name under `dir`, or `None` when `dir` is
/// absent or holds no sub-directory (a missing dir is not an error — no fan-out has run).
fn first_dir_name(dir: &Path) -> Option<String> {
    let mut names: Vec<String> = match std::fs::read_dir(dir) {
        Ok(entries) => entries
            .filter_map(std::result::Result::ok)
            .filter(|entry| entry.path().is_dir())
            .filter_map(|entry| entry.file_name().into_string().ok())
            .collect(),
        Err(_) => Vec::new(),
    };
    names.sort();
    names.into_iter().next()
}

/// Scan every **tracked** worktree file for word-boundary occurrences of `needle` (the old
/// slug), returning a sorted `<repo-rel-path>:<line>` label per hit. This is the determinism
/// boundary's **advisory** report: the CLI has already rewritten every structured ref-field,
/// so the surviving occurrences are exactly the managed-doc *prose* and *unmanaged* mentions
/// it cannot author — reported, never rewritten. Scoped to `git ls-files` (tracked,
/// `.gitignore`-respecting). Best-effort: an unreadable/binary file (or a `git` failure) is
/// skipped, never an error — the report must never change the verb's exit status.
fn scan_prose_mentions(repo_root: &Path, needle: &str) -> Vec<String> {
    let Ok(listing) = git_capture(repo_root, &["ls-files"]) else {
        return Vec::new();
    };
    let mut hits: Vec<String> = Vec::new();
    for rel in listing.lines().filter(|l| !l.is_empty()) {
        let Ok(content) = std::fs::read_to_string(repo_root.join(rel)) else {
            continue;
        };
        for (i, line) in content.lines().enumerate() {
            if line_has_token(line, needle) {
                hits.push(format!("{rel}:{}", i + 1));
            }
        }
    }
    hits.sort();
    hits
}

/// True iff `needle` occurs in `line` as a standalone slug **token** — bounded on both sides
/// by a non-token byte (or the line edge). A token byte is ASCII-alphanumeric, `-`, or `_`
/// (the slug alphabet plus the separators that would extend it), so the old slug `cache`
/// matches `the cache layer` but never `caches` — the word-boundary discrimination the
/// advisory report rests on.
fn line_has_token(line: &str, needle: &str) -> bool {
    if needle.is_empty() {
        return false;
    }
    let bytes = line.as_bytes();
    let n = needle.len();
    let mut start = 0;
    while let Some(off) = line[start..].find(needle) {
        let i = start + off;
        let before_ok = i == 0 || !is_token_byte(bytes[i - 1]);
        let end = i + n;
        let after_ok = end >= bytes.len() || !is_token_byte(bytes[end]);
        if before_ok && after_ok {
            return true;
        }
        start = i + 1;
    }
    false
}

/// A byte that extends a slug token: ASCII-alphanumeric or a slug separator (`-`/`_`).
fn is_token_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'-' || b == b'_'
}

/// The order-independent identity of a forward edge — `(from, relation, to)` — for diffing
/// the rename's before/after dangling-edge sets (step 0 vs step 4 of [`apply_and_commit`]).
fn edge_key(edge: &index::Edge) -> (String, String, String) {
    (edge.from.clone(), edge.relation.clone(), edge.to.clone())
}

/// The dangling cross-refs the rename **introduced** — every `after`-rename dangling edge
/// whose `(from, relation, to)` identity was **not** already dangling `before` the rename.
/// This is the M18/M19 masking-trap guard made precise: a pre-existing dangle (present in
/// both sets) is excluded, so the verb's integrity gate refuses only on rot the move itself
/// caused and never on drift it merely inherited (which stays a report-only `jigc validate`
/// concern). Each returned label is `<from>#<relation> -> <to>` for an accurate block
/// message that names only the newly-introduced edges.
fn introduced_dangles<'a>(
    before: &BTreeSet<(String, String, String)>,
    after: impl IntoIterator<Item = &'a index::Edge>,
) -> Vec<String> {
    after
        .into_iter()
        .filter(|edge| !before.contains(&edge_key(edge)))
        .map(|edge| format!("{}#{} -> {}", edge.from, edge.relation, edge.to))
        .collect()
}

/// Split a `<type>:<slug>` address into its parts. Rejects a missing `:` or an empty half —
/// both bail sites carry an actionable route (a concrete example + the `jigc describe`
/// pointer at the doctype surface), since a bare slug (`foo`) is the natural first guess.
fn parse_addr(addr: &str) -> Result<(String, String)> {
    let malformed = || {
        anyhow!(
            "`{addr}` is not a `<type>:<slug>` address — e.g. `adr:single-node-cache`\n  \
             route: run `jigc describe` for the doctype surface"
        )
    };
    let (ty, slug) = addr.split_once(':').ok_or_else(malformed)?;
    if ty.is_empty() || slug.is_empty() {
        return Err(malformed());
    }
    Ok((ty.to_string(), slug.to_string()))
}

/// The on-disk repo-relative path for the `<ty>:<slug>` doc. A **placement** doctype
/// lives at its literal `placement.file` (the slug is fixed = type id, so it is ignored),
/// making a placement doc reachable as a rename target *and* repointable as a referrer of
/// a renamed doc (`design/storage.md` → Placement — census site `rename`). A
/// `location:`-bearing doctype resolves to `<location>/<slug>.md` (docs-root prefixed,
/// trailing slash trimmed) — unchanged. Errors on an unknown or transient (neither
/// `location` nor `placement`) doctype.
fn doc_path(schema_map: &BTreeMap<String, Schema>, ty: &str, slug: &str) -> Result<String> {
    let schema = schema_map
        .get(ty)
        .ok_or_else(|| anyhow!("unknown doctype `{ty}`"))?;
    if let Some(placement) = &schema.placement {
        return Ok(placement.file.clone());
    }
    let location = schema.location.as_deref().ok_or_else(|| {
        anyhow!("`{ty}` is a transient doctype — it has no persisted file to rename")
    })?;
    Ok(format!("{}/{slug}.md", location.trim_end_matches('/')))
}

/// Rewrite the document's `# H1` title to `new_title`, preserving every other byte. The
/// H1 is the first ATX level-1 heading (`# `) outside a fenced code block and outside the
/// leading front-matter block (the document title the writer renders from `id-from`).
/// Returns `None` if the doc has no H1.
fn rewrite_h1(source: &str, new_title: &str) -> Option<String> {
    let mut out = String::with_capacity(source.len());
    let mut in_fence = false;
    let mut done = false;
    // The leading `---`-fenced YAML front-matter block (if present) is metadata, not the
    // document body — a human-authored `# ` YAML comment there must never be taken for the
    // H1. Copy it through verbatim and only scan for the H1 past its closing fence. Mirrors
    // the engine's metadata-block detection (`engine::parse::scan_blocks`, pulldown
    // YAML-style metadata): the block opens only when `---` is the document's first line
    // and closes on a `---`/`...` line.
    let mut in_front_matter = source.starts_with("---\n") || source == "---";
    let mut opening_fence = in_front_matter;
    for line in source.split_inclusive('\n') {
        let body = line.strip_suffix('\n').unwrap_or(line);
        if in_front_matter {
            if opening_fence {
                opening_fence = false;
            } else if body == "---" || body == "..." {
                in_front_matter = false;
            }
            out.push_str(line);
            continue;
        }
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
    fn rewrite_h1_skips_a_front_matter_yaml_comment() {
        // Humans edit managed docs through git (storage.md), so front matter can carry a
        // `# `-prefixed YAML comment. That line is metadata — it must not be taken for the
        // H1: the front matter stays byte-for-byte, and the real body H1 is the one rewritten.
        let src = "---\nstatus: accepted\n# a human-added yaml comment\ndate: 2026-01-01\n---\n\n# Single node cache\n\n## Context\n\nProse.\n";
        let out = rewrite_h1(src, "Distributed cache")
            .expect("the body H1 is found past the front matter");
        assert_eq!(
            out,
            "---\nstatus: accepted\n# a human-added yaml comment\ndate: 2026-01-01\n---\n\n# Distributed cache\n\n## Context\n\nProse.\n"
        );
    }

    #[test]
    fn rewrite_h1_none_without_an_h1() {
        assert!(rewrite_h1("## only an h2\n\nbody\n", "X").is_none());
    }

    #[test]
    fn line_has_token_matches_a_standalone_slug_token() {
        let slug = "single-node-cache";
        // Bounded by spaces / line edges / punctuation → a real mention.
        assert!(line_has_token("the single-node-cache decision", slug));
        assert!(line_has_token("single-node-cache", slug));
        assert!(line_has_token("// single-node-cache: legacy", slug));
        assert!(line_has_token("[adr:single-node-cache]", slug));
    }

    #[test]
    fn line_has_token_rejects_a_caches_style_near_match() {
        let slug = "single-node-cache";
        // A trailing alnum (`s`) or a separator (`-`/`_`) extends the token → not a mention.
        assert!(!line_has_token(
            "we considered single-node-caches instead",
            slug
        ));
        assert!(!line_has_token("single-node-cache-v2 supersedes it", slug));
        assert!(!line_has_token("pre-single-node-cache prefix", slug));
        assert!(!line_has_token("single-node-cache_legacy", slug));
    }

    fn edge(from: &str, relation: &str, to: &str) -> index::Edge {
        index::Edge {
            from: from.to_string(),
            relation: relation.to_string(),
            to: to.to_string(),
        }
    }

    #[test]
    fn introduced_dangles_excludes_a_preexisting_dangle() {
        // The masking-trap guard: a dangle present *before* the rename (in `before`) is
        // inherited rot, not caused by the move — it must NOT be flagged as introduced, so
        // the rename is never blocked by drift it did not cause.
        let preexisting = edge("adr:unrelated-dangler", "supersedes", "adr:ghost");
        let before: BTreeSet<(String, String, String)> =
            [edge_key(&preexisting)].into_iter().collect();
        // After the rename the same pre-existing dangle is still present (untouched).
        let after = vec![preexisting.clone()];
        assert!(
            introduced_dangles(&before, &after).is_empty(),
            "a pre-existing dangle must not count as introduced by the rename",
        );
    }

    #[test]
    fn introduced_dangles_flags_a_newly_introduced_dangle() {
        // The gate must still fire: a dangle that appears *only after* the rename (not in
        // `before`) is one the move itself caused — it is flagged with an accurate label.
        let preexisting = edge("adr:unrelated-dangler", "supersedes", "adr:ghost");
        let before: BTreeSet<(String, String, String)> =
            [edge_key(&preexisting)].into_iter().collect();
        // The pre-existing dangle survives AND a new one appears (a referrer the move failed
        // to repoint, say) — only the new one is reported.
        let newly = edge("adr:missed-referrer", "supersedes", "adr:single-node-cache");
        let after = vec![preexisting, newly];
        assert_eq!(
            introduced_dangles(&before, &after),
            vec!["adr:missed-referrer#supersedes -> adr:single-node-cache".to_string()],
            "a newly-introduced dangle must be flagged, naming only the new edge",
        );
    }

    #[test]
    fn parse_addr_rejects_malformed() {
        assert!(parse_addr("adr:slug").is_ok());
        // Both bail sites (missing `:` and empty half) carry the actionable route: a
        // concrete `<type>:<slug>` example and the `jigc describe` pointer.
        for bad in ["noslug", ":slug", "adr:"] {
            let msg = parse_addr(bad).unwrap_err().to_string();
            assert!(msg.contains("<type>:<slug>"), "form: {msg}");
            assert!(msg.contains("adr:"), "example: {msg}");
            assert!(msg.contains("jigc describe"), "describe pointer: {msg}");
        }
    }

    fn schema_map(yamls: &[&str]) -> BTreeMap<String, Schema> {
        yamls
            .iter()
            .map(|y| {
                let s = engine::schema::load_schema(y.as_bytes()).expect("schema loads");
                (s.ty.clone(), s)
            })
            .collect()
    }

    /// (M38 inc-2 T3) `doc_path` resolves a **placement** doctype's on-disk path to its
    /// literal `placement.file` (the slug is fixed = type id), so a placement doc that
    /// references a renamed doc repoints at its real home — never the `{dir}/{slug}.md`
    /// composition that a location doctype uses (`design/storage.md` → Placement — census
    /// site `rename`). A `location:`-bearing doctype is unchanged: `<location>/<slug>.md`.
    #[test]
    fn doc_path_resolves_a_placement_literal_and_leaves_location_docs_unchanged() {
        let map = schema_map(&[
            "type: foo\nplacement: { file: FOO.md }\nsections: []\n",
            "type: adr\nlocation: decisions/\nid-from: title\nsections:\n  - id: body\n    slot: { hint: x }\n",
        ]);

        // A placement doctype resolves to its literal file, ignoring the passed slug.
        assert_eq!(
            doc_path(&map, "foo", "foo").expect("placement path resolves"),
            "FOO.md",
            "a placement doctype's path is its literal placement.file",
        );
        // A location doctype is byte-unchanged: `<location>/<slug>.md`.
        assert_eq!(
            doc_path(&map, "adr", "single-node-cache").expect("location path resolves"),
            "decisions/single-node-cache.md",
            "a location doctype still resolves to <location>/<slug>.md",
        );
    }
}
