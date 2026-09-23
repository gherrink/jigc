//! **The route-span carve-out** — the one place a law-1 host-path scan is allowed to look
//! away, and the reason it is allowed to (M53 — the cwd census, the route class;
//! `design/surface-contract.md` → The printed-path fence, the *pasteable shell bytes*
//! disposition).
//!
//! Law 1 renders every printed path repo-relative, and several suites prove it the same way:
//! drive a door in a fixture repo whose root came from `mktemp -d`, then scan the whole of
//! stdout+stderr for that root's prefix. That scan is exactly right for a message, an `at:`
//! locus and a `(code, target)` key — the three surfaces a reader reads and a driver keys on.
//!
//! It is wrong for the one span that is not read but **run**. Since M53 every operator-facing
//! `git` span carrying a path operand is rendered by `engine::finding::git_at` as
//! `` `git -C <absolute checkout> <subcommand> …` ``, because git resolves a pathspec against
//! the *caller's* cwd: driven from `docs/deep`, the shipped `git restore --staged --
//! docs/deep/carried.txt` exited **1** and `git add -- docs/deep/untracked.md` exited **128**;
//! from a linked worktree the same bytes reach the wrong index, which no relative spelling can
//! fix. The absolute in those spans is the fix, not the defect.
//!
//! **The same disposition reaches one other verb** (M53 post-review-fix review, HIGH 2):
//! `jigc migrate <PATH>` resolves its argument against the caller's cwd too, and takes no
//! `-C`, so the only spelling that names the same file from every directory is an absolute
//! one — `engine::finding::migrate_at`'s render. Driven from `docs/deep` on a `fresh` corpus,
//! the store sweep's own adoption advisory emitted `jigc migrate CHANGELOG.md --as changelog`
//! and running it verbatim answered *"could not read the foreign `changelog` source"*. Those
//! bytes are run, not read, exactly as the `git` span's are.
//!
//! So the scan keeps its subject and loses exactly two regions: a backticked span whose first
//! two tokens are `git -C`, and one whose first two are `jigc migrate` followed by an
//! **absolute** operand. **Everything else still carries the whole rule** — the message the
//! span sits in, the locus, the key, every other backticked span, any absolute in a `git`
//! span that is *not* aimed, and a `jigc migrate` span whose operand is relative (both of
//! which are the defects this wave closed and must stay visible). A prefix appearing anywhere
//! outside such a span still fails the suite that calls this.
//!
//! The span's bytes are replaced by a marker of the same shape rather than deleted, so a
//! failure message still reads as the surface it came from.

/// `text` with every backticked **based** route span replaced by a marker of the same shape:
/// an aimed `git` span by `` `git -C <aimed> …` ``, a based `jigc migrate` span by
/// `` `jigc migrate <based> …` ``.
///
/// Only the span's *inside* is redacted; the backticks stay, so a caller asserting on the
/// surrounding prose is unaffected. A span that is neither — a bare `` `--force` ``, any other
/// `` `jigc …` `` line, an unaimed `` `git add` ``, a `jigc migrate` whose operand is
/// relative — is returned verbatim, which is what keeps this a carve-out rather than a hole.
pub fn redact_aimed_git_spans(text: &str) -> String {
    let pieces: Vec<&str> = text.split('`').collect();
    // An odd-indexed piece lies between a pair of backticks. An even number of pieces means
    // the final backtick is unpaired, so the tail is ordinary prose and opens no span — the
    // conservative reading `engine::finding::backticked_spans` already takes.
    let spans_end = if pieces.len().is_multiple_of(2) {
        pieces.len() - 1
    } else {
        pieces.len()
    };
    let mut out = String::with_capacity(text.len());
    for (i, piece) in pieces.iter().enumerate() {
        if i > 0 {
            out.push('`');
        }
        let mut words = piece.split_whitespace();
        let (first, second) = (words.next(), words.next());
        let aimed = first == Some("git") && second == Some("-C");
        // A based `jigc migrate`: the operand must actually be absolute, so an un-swept
        // producer's relative one stays visible to the scan.
        let based_migrate = first == Some("jigc")
            && second == Some("migrate")
            && words
                .next()
                .is_some_and(|operand| operand.trim_matches('\'').starts_with('/'));
        let marker = if aimed {
            Some("git -C <aimed> \u{2026}")
        } else if based_migrate {
            Some("jigc migrate <based> \u{2026}")
        } else {
            None
        };
        match marker.filter(|_| i % 2 == 1 && i < spans_end) {
            Some(marker) => out.push_str(marker),
            None => out.push_str(piece),
        }
    }
    out
}
