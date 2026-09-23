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
//! So the scan keeps its subject and loses exactly one region: a backticked span whose first
//! token is `git` and whose second is `-C`. **Everything else still carries the whole rule** —
//! the message the span sits in, the locus, the key, every other backticked span, and any
//! absolute in a `git` span that is *not* aimed (which is the defect this wave closed and must
//! stay visible). A prefix appearing anywhere outside such a span still fails the suite that
//! calls this.
//!
//! The span's bytes are replaced by a marker of the same shape rather than deleted, so a
//! failure message still reads as the surface it came from.

/// `text` with every backticked aimed-`git` span replaced by `` `git -C <aimed> …` ``.
///
/// Only the span's *inside* is redacted; the backticks stay, so a caller asserting on the
/// surrounding prose is unaffected. A span that is not a `git … -C …` command line — a bare
/// `` `--force` ``, a `` `jigc …` `` line, an unaimed `` `git add` `` — is returned verbatim,
/// which is what keeps this a carve-out rather than a hole.
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
        let aimed = words.next() == Some("git") && words.next() == Some("-C");
        if i % 2 == 1 && i < spans_end && aimed {
            out.push_str("git -C <aimed> \u{2026}");
        } else {
            out.push_str(piece);
        }
    }
    out
}
