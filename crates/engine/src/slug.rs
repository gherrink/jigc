//! Slug normalization — the one minting-slug rule.
//!
//! [`slugify`] implements the settled lowercase-ASCII-kebab normalization
//! (`DECISIONS.md` 2026-05-31 → "Slug / minting normalization" + "Slug
//! normalization home"; [design/structural-grammar.md](../../../design/structural-grammar.md)
//! → IDs: provenance and minting). Minted IDs are **frozen content-slugs**:
//! slugged from a designated id-source, then frozen at creation.
//!
//! This is the **pure normalization only**. Collision disambiguation (the
//! numeric `-2`, `-3`, … suffix) and the empty→type-name fallback are the
//! *caller's* concern at the mint sites (inc 3/4), because only the caller
//! knows the type name and the set of already-minted siblings. Here, an input
//! that normalizes away entirely yields `""`.
//!
//! The rule, in order — steps 1–5 are [`renormalize`] (the **recognition** rule,
//! and the one separator map), which [`slugify`] (the **mint** rule) composes over:
//! 1. lowercase (ASCII case-fold),
//! 2. transliterate the common Latin accented letters to ASCII (`café`→`cafe`),
//! 3. map the **separators** — space, `_`, `/`, `.` — to `-` (the `/`·`.` half is
//!    the [rule-version-2 fork](#the-rule-is-itself-versioned-m42)),
//! 4. strip every remaining char outside `[a-z0-9-]`,
//! 5. collapse runs of `-` and trim leading/trailing `-`,
//! 6. cap at the first ~5 words (dash-separated), dropping the rest, then drop
//!    any leading/trailing stopwords (`the`/`a`/`of`…) the cap leaves at an
//!    edge — a medial stopword is untouched,
//! 7. (the word cap always cuts on a `-` boundary, so no trailing `-` is
//!    exposed and no re-trim is needed),
//! 8. apply a char-length backstop — on a mid-word cut, retreat to the last
//!    complete word (via the final `-`); a single long word with no `-` still
//!    hard-truncates to a filesystem-safe length (the `NAME_MAX` backstop),
//! 9. re-drop any leading/trailing stopword the char cap of step 8 *exposes* —
//!    the retreat to a complete word can uncover an edge stopword step 6 never
//!    saw — so mint output never ends (or starts) on filler.
//!
//! The output always matches `^[a-z0-9-]*$` with no leading, trailing, or
//! doubled `-`, and the function is **idempotent by construction**:
//! `slugify(slugify(x)) == slugify(x)`. Step 9 is what secures this — without a
//! post-cap edge-stopword drop, the step-8 char cap could leave a fresh trailing
//! stopword that a second pass would strip.
//!
//! # The rule is itself versioned (M42)
//!
//! `slugify` is jigc's **identity-derivation function** — it mints every doc slug,
//! every task/milestone id, and every `{#id}` item anchor — yet it sits in no
//! `schema-hash` and no manifest, so the pack-load freeze assert (the gate that
//! blocks renaming a *field*) was blind to a change in the function that *names*
//! every id. And no transform kind can re-mint an id, so a rule change **cannot be
//! migrated after the fact**: a corpus that spans one carries two id generations
//! permanently. So the rule is **declared**: [`SLUG_RULE_VERSION`] +
//! [`rule_fingerprint`] are pinned beside the schema manifest (`slug-rule:`) and
//! [`crate::manifest::check`] blocks loudly on a drift
//! ([design/storage.md](../../../design/storage.md) → Identity → *The slug rule is
//! itself a versioned rule (M42)*).
//!
//! **Version 2 (M42) is the first fork**: `/` and `.` join the separator map
//! (step 3) instead of being stripped. Under generation 1 a slash **fused two
//! words** (`auth/session` → `authsession`) and a dot **collided** (`1.0` and `10`
//! both minted `10` — live in a changelog whose release ids are minted from version
//! strings). Under generation 2 they map to `-` like a space. The split is
//! **permanent**: ids minted under generation 1 keep their bytes (they are frozen
//! anchors and frozen paths), and there is no migration — so the corpus, not the
//! rule, carries the history.

use crate::file_state::hash_bytes;

/// Cap on slug length, in **words** (dash-separated segments). A minted slug
/// keeps at most the first `MAX_WORDS` words of its id-source and drops the
/// rest, so a long intent yields a short, legible slug rather than one that
/// trails off into filler. The cut always lands on a `-` boundary, never
/// mid-word. This is a mint-time cap only; [`is_slug`] is uncapped so a ref to
/// a pre-existing longer slug still resolves.
///
/// `pub` (with [`MAX_CHARS`]) since the M43 pre-trial surface polish: `jigc doc
/// create --help` states the mint caps as static text, and its drift test pins
/// the emitted numbers against these constants.
pub const MAX_WORDS: usize = 5;

/// Character-length backstop, applied *after* the word cap. The word cap alone
/// leaves a single long word (no `-` to cut on) unbounded, and a minted slug is
/// used verbatim as a filename (`<location>/<slug>.md`), so a pathological
/// single-word intent could overrun the filesystem `NAME_MAX` (~255 bytes) and
/// fail the write. This final ceiling truncates such a slug to a filesystem-safe
/// length. `50` restores the pre-M39 backstop — well under `NAME_MAX` even with
/// the `.md` suffix and any `-N` collision suffix. Normal ≤5-word intents sit
/// far below it, so multi-word behaviour is unchanged.
pub const MAX_CHARS: usize = 50;

/// The **stated-at statement** of the mint-time caps — the sentence a
/// soliciting surface renders at the id-source it slugs, so the slug word-cap is
/// stated where it binds, before it can surprise (`design/surface-contract.md` →
/// The stated-at fence, seam-generated tier; law 3). Built from
/// [`MAX_WORDS`]/[`MAX_CHARS`] themselves — statement and enforcement share one
/// source, so drift between them is unrepresentable.
///
/// `minted_into` names *what* the id-source is slugged into, so each surface
/// stays honest: the `{{schema:<doctype>}}` projection at the `title:` line
/// passes `"the doc id"`; `jigc doc add-item --help` — the second soliciting
/// surface, which mints an item's `{#id}` anchor from `--title` the same way —
/// passes the item anchor. Only the drift-prone cap clause is shared.
pub fn mint_statement(minted_into: &str) -> String {
    format!(
        "the id-source — slugged lowercase-kebab into {minted_into}, \
         capped at the first {MAX_WORDS} words / {MAX_CHARS} chars"
    )
}

/// Conservative English **edge-stopword** set — articles plus the short
/// prepositions — dropped when the word cap leaves one at the leading or
/// trailing edge of a minted slug (`DECISIONS.md` 2026-07-11 → Fork 5: drop
/// leading/trailing `the`/`a`/`of`…). A *medial* stopword is untouched (it
/// carries meaning between two content words, e.g. `add-a-rate-limiter`). The
/// set is deliberately small: the slug is a legibility aid, so we trim only the
/// filler that reads as noise at an edge, never content.
const EDGE_STOPWORDS: &[&str] = &["a", "an", "the", "of", "to", "in", "on", "at", "by", "for"];

/// Apply the **deterministic collision suffix** to a base slug: `1` keeps the bare
/// `slug`, `2` yields `<slug>-2`, `3` yields `<slug>-3`, … (`design/structural-grammar.md`
/// → IDs: provenance and minting → "the lower-task-id instance keeps the bare slug; each
/// higher one takes the deterministic numeric suffix `-2`, `-3`, … in task-id order").
/// The first instance of a colliding slug is `nth == 1`; the suffix is applied only by the
/// caller that knows the task-id merge order (the by-task-id join), never at a mint site.
pub fn suffixed(slug: &str, nth: usize) -> String {
    if nth <= 1 {
        slug.to_string()
    } else {
        format!("{slug}-{nth}")
    }
}

/// Whether `s` is a **well-formed slug**: a non-empty `[a-z0-9-]` string with no
/// leading, trailing, or doubled `-`. This is the *recognition* predicate — the
/// grammar a minted or authored slug must match — and the *recognizer* paired
/// with [`slugify`]'s *normalization*: every non-empty `slugify(x)` satisfies
/// `is_slug`. The converse does **not** hold: a valid slug need not be a
/// `slugify` fixed point — the mint-time word cap and the F5 edge-stopword drop
/// mean `is_slug("a-0")` yet `slugify("a-0") == "0"`. So a *recognition* site
/// (an authored anchor, a ref body, a frozen id read back from disk) must use
/// `is_slug`, never `slugify(x) == x`, which would reject valid frozen ids. The
/// length/word caps are *not* enforced here (they are mint-time concerns; a ref
/// pointing at an existing slug must recognize it whatever its length).
///
/// Used by the write-time `ref` shape check to validate the `<slug>` body of a
/// `<type>:<slug>` reference (`design/auto-migration.md` → Write-time ref-shape
/// check).
pub fn is_slug(s: &str) -> bool {
    !s.is_empty()
        && s.bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
        && !s.starts_with('-')
        && !s.ends_with('-')
        && !s.contains("--")
}

/// **Renormalize** text to slug shape — steps 1–5 only: case-fold, transliterate,
/// separator-map, strip, collapse. The **one** separator map: [`slugify`] composes
/// over it, and every *recognition* site calls it directly.
///
/// This is the **recognition** rule, and it is the true inverse of the writer's
/// heading rendering (which splits a frozen id on `-`, capitalizes each word, and
/// joins with a space): `renormalize` lowercases and maps the space back, with no
/// cap and no stopword drop in between — so `renormalize(heading_text(id)) == id`
/// holds **by construction** for every well-formed slug ([`is_slug`]).
///
/// [`slugify`] must **never** be used to recognize an existing id. It is the *mint*
/// rule, and since the M41 F5 edge-stopword drop it is **not** a fixed point of
/// every valid slug: a frozen section id like `in-scope` renders `## In Scope`,
/// which `slugify` maps to `scope` — under a `slugify`-based compare that section
/// reads as *renamed* and aborts the whole-doc parse. The four production
/// recognition sites are `parse::heading_matches`, `write::present_body_sections`,
/// `validate::surplus_sections_absent`'s positional compare, and the CLI's
/// `migrate_corpus::has_section_heading` (`design/storage.md` → Identity → *The slug
/// rule is itself a versioned rule*).
pub fn renormalize(text: &str) -> String {
    // 1–4: case-fold, transliterate, separator-map, strip.
    let mut out = String::with_capacity(text.len());
    for ch in text.chars() {
        match ch {
            ' ' | '_' | '/' | '.' => out.push('-'),
            'a'..='z' | '0'..='9' | '-' => out.push(ch),
            'A'..='Z' => out.push(ch.to_ascii_lowercase()),
            _ => {
                if let Some(folded) = transliterate(ch) {
                    out.push_str(folded);
                }
                // else: drop the char entirely (stripped).
            }
        }
    }

    // 5: collapse runs of '-' and trim.
    collapse_dashes(&out)
}

/// Normalize an id-source into a frozen content-slug — the **mint** rule.
///
/// See the [module docs](self) for the full rule: [`renormalize`] (steps 1–5) plus
/// the mint-time caps and the edge-stopword drop. Pure and total: every input maps
/// to a valid slug (`^[a-z0-9-]*$`); an input with no slug-able content maps to `""`
/// (the caller supplies the type-name fallback).
///
/// Mint only. To *recognize* an id already minted (a heading read back off disk, an
/// authored anchor), use [`renormalize`] / [`is_slug`] — never `slugify`.
pub fn slugify(id_source: &str) -> String {
    // 1–5: the one separator map.
    let collapsed = renormalize(id_source);

    // 6–7: cap at the first MAX_WORDS words (always a '-' boundary).
    let capped = cap_words(&collapsed);

    // 8: char-length backstop — bound a single long word (which the word cap
    // leaves untouched) to a filesystem-safe length.
    let bounded = cap_chars(&capped);

    // 9: the char cap can truncate at a word boundary that leaves a *fresh*
    // trailing (or, after a doc-level cut, leading) edge stopword step 6 never
    // saw, so re-drop edge stopwords here. This makes `slugify` idempotent by
    // construction: mint output carries no edge stopword and is always a fixed
    // point of a second pass.
    drop_edge_stopwords(&bounded)
}

/// The transliteration table: each non-ASCII char that folds to an ASCII
/// skeleton, paired with that skeleton. Anything absent folds to nothing (it is
/// stripped).
///
/// Covers the common Latin-1 / Latin Extended-A accented letters and ligatures
/// — enough for the settled `café`→`cafe` rule and ordinary Latin-script titles.
/// Deliberately small and hand-rolled (no transliteration crate): the slug is a
/// *legibility* aid, not a faithful romanization.
///
/// It is a **table**, not a `match`, so [`rule_fingerprint`] can *enumerate* it:
/// the fingerprint's input vector is generated from this array, so adding,
/// removing, or re-pointing an entry moves the fingerprint by construction — a
/// hand-picked census would wave the change through.
const TRANSLITERATE: &[(char, &str)] = &[
    ('à', "a"),
    ('á', "a"),
    ('â', "a"),
    ('ã', "a"),
    ('ä', "a"),
    ('å', "a"),
    ('À', "a"),
    ('Á', "a"),
    ('Â', "a"),
    ('Ã', "a"),
    ('Ä', "a"),
    ('Å', "a"),
    ('æ', "ae"),
    ('Æ', "ae"),
    ('ç', "c"),
    ('Ç', "c"),
    ('è', "e"),
    ('é', "e"),
    ('ê', "e"),
    ('ë', "e"),
    ('È', "e"),
    ('É', "e"),
    ('Ê', "e"),
    ('Ë', "e"),
    ('ì', "i"),
    ('í', "i"),
    ('î', "i"),
    ('ï', "i"),
    ('Ì', "i"),
    ('Í', "i"),
    ('Î', "i"),
    ('Ï', "i"),
    ('ñ', "n"),
    ('Ñ', "n"),
    ('ò', "o"),
    ('ó', "o"),
    ('ô', "o"),
    ('õ', "o"),
    ('ö', "o"),
    ('ø', "o"),
    ('Ò', "o"),
    ('Ó', "o"),
    ('Ô', "o"),
    ('Õ', "o"),
    ('Ö', "o"),
    ('Ø', "o"),
    ('œ', "oe"),
    ('Œ', "oe"),
    ('ù', "u"),
    ('ú', "u"),
    ('û', "u"),
    ('ü', "u"),
    ('Ù', "u"),
    ('Ú', "u"),
    ('Û', "u"),
    ('Ü', "u"),
    ('ý', "y"),
    ('ÿ', "y"),
    ('Ý', "y"),
    ('Ÿ', "y"),
    ('ß', "ss"),
];

/// Transliterate a single non-ASCII char to its ASCII lowercase skeleton, or
/// `None` if it has no sensible ASCII fold (then it is stripped). A lookup in
/// [`TRANSLITERATE`] — reached only for chars outside `[a-zA-Z0-9- _]`, so the
/// linear scan of ~60 entries is off the hot path.
fn transliterate(ch: char) -> Option<&'static str> {
    TRANSLITERATE
        .iter()
        .find(|(c, _)| *c == ch)
        .map(|(_, skeleton)| *skeleton)
}

/// Collapse runs of `-` to a single `-` and trim leading/trailing `-`.
fn collapse_dashes(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut prev_dash = false;
    for ch in s.chars() {
        if ch == '-' {
            if !prev_dash {
                out.push('-');
            }
            prev_dash = true;
        } else {
            out.push(ch);
            prev_dash = false;
        }
    }
    // trim leading/trailing '-'
    out.trim_matches('-').to_string()
}

/// Cap a collapsed, trimmed slug at the first [`MAX_WORDS`] dash-separated
/// words, then drop any leading/trailing [`EDGE_STOPWORDS`] the cap exposes (via
/// [`drop_edge_stopwords`]).
///
/// The input is already collapsed and edge-trimmed, so splitting on `-` yields
/// clean, non-empty words; joining back with `-` always lands on a word
/// boundary and can never leave a leading, trailing, or doubled `-`. The
/// stopword drop is greedy at each edge (so `the guide of` → `guide`) and never
/// touches a medial stopword (`add a rate-limiter` → `add-a-rate-limiter`); an
/// all-stopword input strips to `""` (the caller supplies the type-name
/// fallback). A slug already within the cap with content at both edges is
/// returned unchanged.
fn cap_words(s: &str) -> String {
    drop_edge_stopwords(&s.split('-').take(MAX_WORDS).collect::<Vec<_>>().join("-"))
}

/// Drop any leading/trailing [`EDGE_STOPWORDS`] from a collapsed, trimmed slug,
/// greedily at each edge, leaving medial stopwords untouched.
///
/// Run twice on the [`slugify`] path: once inside [`cap_words`] (so a leading
/// stopword frees a word slot before the cap counts words), and once as the
/// final step after [`cap_chars`] (so an edge stopword the char cap *exposes* at
/// the truncation boundary is dropped too). The second pass is what makes
/// `slugify` idempotent: its output never carries an edge stopword, so a re-run
/// is a no-op. Input is already collapsed/edge-trimmed, so splitting on `-`
/// yields clean non-empty words and joining back can never leave a leading,
/// trailing, or doubled `-`; an all-stopword input strips to `""`.
fn drop_edge_stopwords(s: &str) -> String {
    let mut words: Vec<&str> = s.split('-').collect();
    while words.first().is_some_and(|w| EDGE_STOPWORDS.contains(w)) {
        words.remove(0);
    }
    while words.last().is_some_and(|w| EDGE_STOPWORDS.contains(w)) {
        words.pop();
    }
    words.join("-")
}

/// Char-length backstop: bound a slug to at most [`MAX_CHARS`] characters,
/// retreating to the last **complete** word when the cut lands mid-word.
///
/// Applied after [`cap_words`], this is the final ceiling that bounds a slug
/// whose words (or single word) overrun the filesystem-safe length. When the
/// `MAX_CHARS` cut falls mid-word, we retreat to the previous `-` boundary so
/// the slug ends on a whole word (`…-outputs-contract` → `…-outputs`, never the
/// mid-word lop `…-contrac`). A *single* long word has no `-` to retreat to, so
/// it hard-truncates at `MAX_CHARS` — the `NAME_MAX` backstop that keeps a
/// pathological single-word intent from overrunning `<slug>.md`. The prior
/// steps guarantee pure ASCII (`[a-z0-9-]`), so the `MAX_CHARS`-th char boundary
/// is also a byte boundary — but we cut on `char_indices` regardless so the
/// truncation can never split a multibyte char. A slug already within the cap
/// is returned unchanged.
fn cap_chars(s: &str) -> String {
    match s.char_indices().nth(MAX_CHARS) {
        None => s.to_string(),
        Some((byte_idx, _)) => {
            let truncated = &s[..byte_idx];
            if s[byte_idx..].starts_with('-') || truncated.ends_with('-') {
                // The cut lands on a word boundary: the last kept word is whole.
                truncated.trim_end_matches('-').to_string()
            } else if let Some(dash) = truncated.rfind('-') {
                // Mid-word cut: retreat to the last complete word.
                truncated[..dash].to_string()
            } else {
                // A single long word with no `-`: hard-truncate (NAME_MAX floor).
                truncated.to_string()
            }
        }
    }
}

/// The **declared version of the slug rule** — the identity-derivation rule
/// [`slugify`] implements. Bumped by (and only by) a deliberate change to that
/// rule, in the same commit that re-pins every shipped manifest's `slug-rule:`
/// block. A corpus that spans a bump carries **two permanent id generations**:
/// ids minted under the old rule keep their bytes (they are frozen anchors and
/// frozen paths), and no transform kind can re-mint an id, so there is no
/// migration to reconcile them. The version is what makes that split a
/// *declared* event rather than a discovered one.
///
/// **2** (M42): `/` and `.` map to `-` instead of being stripped — see the [module
/// docs](self#the-rule-is-itself-versioned-m42). **1** was the M39 word-capped rule
/// as first shipped; a corpus older than this bump carries generation-1 ids forever.
pub const SLUG_RULE_VERSION: u32 = 2;

/// The **fingerprint of the slug rule**: the lowercase-hex `blake3` digest of
/// `slugify`'s behaviour over a *generated* input vector — the identity-mint
/// sibling of [`crate::manifest::schema_hash`], and the value each pack's
/// `slug-rule.hash` pins.
///
/// The vector is **generated from the rule's own constants**, never hand-picked
/// (a hand-picked table would wave through the very mapping change it exists to
/// catch — "a census cannot enforce a predicate"):
///
/// - every **printable ASCII** char, alone and framed (`a<c>b`) — so a change to
///   the separator map, the strip set, or the case-fold moves the hash (the two
///   frames separate "what does this char *become*" from "what does it do
///   *between* two words": under generation 1 `/` was stripped, so `a/b` fused to
///   `ab`; under generation 2 it separates, so `a/b` is `a-b` — and this vector is
///   what made that fork move the hash at every door);
/// - every [`TRANSLITERATE`] entry, in both frames — so adding, removing, or
///   re-pointing a fold moves the hash;
/// - every [`EDGE_STOPWORDS`] word at each of the three positions (leading,
///   medial, trailing) — so widening or narrowing the set moves the hash;
/// - inputs that **cross both caps** ([`MAX_WORDS`] and [`MAX_CHARS`], including
///   a mid-word cut and a cut that exposes a fresh edge stopword) — so a cap
///   change, or a change to the retreat/re-drop steps, moves the hash.
///
/// Computed once per process (the pack-load gate calls it per manifest-owning
/// pack) and cached.
pub fn rule_fingerprint() -> &'static str {
    static FINGERPRINT: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    FINGERPRINT.get_or_init(|| {
        let census: Vec<String> = fingerprint_inputs()
            .iter()
            .map(|input| format!("{input:?} -> {:?}", slugify(input)))
            .collect();
        hash_bytes(census.join("\n").as_bytes())
    })
}

/// The generated input vector [`rule_fingerprint`] ranges over. See its docs for
/// the four families and why each is derived from a rule constant rather than
/// listed by hand.
fn fingerprint_inputs() -> Vec<String> {
    let mut inputs = Vec::new();

    // 1. Every printable ASCII char, alone and framed between two content words.
    for byte in b' '..=b'~' {
        let ch = byte as char;
        inputs.push(ch.to_string());
        inputs.push(format!("a{ch}b"));
    }

    // 2. Every transliteration-table char, in both frames.
    for (ch, _) in TRANSLITERATE {
        inputs.push(ch.to_string());
        inputs.push(format!("a{ch}b"));
    }

    // 3. Every edge stopword at each of the three positions.
    for word in EDGE_STOPWORDS {
        inputs.push(format!("{word} alpha beta"));
        inputs.push(format!("alpha {word} beta"));
        inputs.push(format!("alpha beta {word}"));
    }

    // 4. Inputs crossing both caps: more words than MAX_WORDS; a single word
    //    longer than MAX_CHARS; a multi-word input whose MAX_CHARS cut lands
    //    mid-word (the retreat); and one whose cut exposes a fresh trailing edge
    //    stopword (the post-cap re-drop).
    inputs.push(
        (0..MAX_WORDS + 3)
            .map(|i| format!("word{i}"))
            .collect::<Vec<_>>()
            .join(" "),
    );
    inputs.push("x".repeat(MAX_CHARS + 10));
    let long_word = "z".repeat(MAX_CHARS / 3 + 2);
    inputs.push(format!("{long_word} {long_word} {long_word}"));
    inputs.push(format!("alpha beta to {}", "y".repeat(MAX_CHARS)));

    inputs
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    /// Golden table: a fixed input→output map pinning every clause of the rule.
    /// Pinned as one insta snapshot so the whole contract reads at a glance and
    /// a rule change shows up as a reviewable diff.
    #[test]
    fn golden_table() {
        let cases = [
            // (label, input)
            ("transliterate", "café"),
            ("space-to-dash", "Add rate limiter"),
            ("underscore-to-dash", "add_rate_limiter"),
            ("case-fold", "AddRateLimiter"),
            ("strip-punctuation", "Add a rate-limiter!"),
            ("collapse-dashes", "x---b__ c"),
            ("trim-edges", "  -hello-  "),
            ("ligatures", "Æsop & œuvre"),
            ("sharp-s", "Straße"),
            ("empty", ""),
            ("strips-to-empty", "!!!___---"),
            ("non-latin-dropped", "日本語 test"),
            (
                "cap-to-five-words",
                "this is a very long intent that should be truncated at a word boundary",
            ),
            (
                "cap-one-long-word",
                "supercalifragilisticexpialidocioussupercalifragilisticexpialidocious",
            ),
            // F5: leading + trailing stopword dropped, medial stopword kept.
            ("edge-stopwords-medial-kept", "a node the app a"),
            // V7: a mid-word char-cap cut retreats to the last complete word
            // (no `contrac` mid-lop).
            (
                "word-boundary-retreat",
                "backwards-compatible-serialization-outputs-contract",
            ),
            // Transliteration-nasty inputs: the composition `is_slug(slugify(x))`
            // is only interesting where the *transliterator* — not the caller —
            // introduces characters the recognizer rejects. A lone combining mark
            // has no base to attach to; the punctuation dashes and the uppercase
            // Đ/ẞ transliterate *toward* the two shapes `is_slug` forbids (an edge
            // dash, an upper-case byte).
            ("combining-mark-alone", "\u{0301}"),
            ("transliterates-to-dash", "–—‒"),
            ("transliterates-to-uppercase", "Đorđe ẞ"),
            // M42 Inc 10 — the separator fork, at slug-rule-version 2: `/` and `.`
            // MAP to `-` (they were stripped under generation 1, where a slash
            // fused two words and `1.0`/`10` minted one colliding id). T1 pinned
            // the broken outputs here so the fork lands as a reviewable diff; these
            // four lines are that diff.
            ("slash-separates-words", "auth/session"),
            ("dotted-version-keeps-its-dots", "1.0"),
            ("dot-free-former-collision-partner", "10"),
            ("path-like-id-source", "src/main.rs"),
        ];
        let table: Vec<String> = cases
            .iter()
            .map(|(label, input)| format!("{label}: {input:?} -> {:?}", slugify(input)))
            .collect();

        // The **composition** — the clause neither `golden_table`'s input→output
        // pairs nor `is_slug_recognizes_well_formed_slugs` pins on its own: every
        // value `slugify` produces is one `is_slug` accepts (or empty — the total
        // function's one sanctioned escape, which callers handle). Asserted here,
        // deterministically, beside the table it ranges over: a proptest that
        // *samples* this is a finder, not a fence (dev-workflow → Gate).
        for (label, input) in &cases {
            let out = slugify(input);
            assert!(
                out.is_empty() || is_slug(&out),
                "slugify({input:?}) [{label}] produced {out:?}, which is_slug rejects"
            );
        }
        insta::assert_snapshot!(table.join("\n"), @r#"
        transliterate: "café" -> "cafe"
        space-to-dash: "Add rate limiter" -> "add-rate-limiter"
        underscore-to-dash: "add_rate_limiter" -> "add-rate-limiter"
        case-fold: "AddRateLimiter" -> "addratelimiter"
        strip-punctuation: "Add a rate-limiter!" -> "add-a-rate-limiter"
        collapse-dashes: "x---b__ c" -> "x-b-c"
        trim-edges: "  -hello-  " -> "hello"
        ligatures: "Æsop & œuvre" -> "aesop-oeuvre"
        sharp-s: "Straße" -> "strasse"
        empty: "" -> ""
        strips-to-empty: "!!!___---" -> ""
        non-latin-dropped: "日本語 test" -> "test"
        cap-to-five-words: "this is a very long intent that should be truncated at a word boundary" -> "this-is-a-very-long"
        cap-one-long-word: "supercalifragilisticexpialidocioussupercalifragilisticexpialidocious" -> "supercalifragilisticexpialidocioussupercalifragili"
        edge-stopwords-medial-kept: "a node the app a" -> "node-the-app"
        word-boundary-retreat: "backwards-compatible-serialization-outputs-contract" -> "backwards-compatible-serialization-outputs"
        combining-mark-alone: "\u{301}" -> ""
        transliterates-to-dash: "–—‒" -> ""
        transliterates-to-uppercase: "Đorđe ẞ" -> "ore"
        slash-separates-words: "auth/session" -> "auth-session"
        dotted-version-keeps-its-dots: "1.0" -> "1-0"
        dot-free-former-collision-partner: "10" -> "10"
        path-like-id-source: "src/main.rs" -> "src-main-rs"
        "#);
    }

    // A few explicit point assertions for the load-bearing done-criterion
    // cases, so a regression names itself without reading the snapshot.
    #[test]
    fn done_criterion_points() {
        assert_eq!(slugify("café"), "cafe");
        assert_eq!(slugify("Add rate limiter"), "add-rate-limiter");
        assert_eq!(slugify(""), "");
    }

    /// **The M42 fork (slug-rule-version 2).** `/` and `.` are *separators*, not
    /// noise: they map to `-` like a space, rather than being stripped. Both halves
    /// are defects of the generation-1 rule, and both are id-level:
    ///
    /// - a slash **fuses two words** (`auth/session` → `authsession`), so a
    ///   path-shaped id-source mints an unreadable id;
    /// - a dot **collides** (`1.0` and `10` both minted `10`) — live in a changelog
    ///   whose release ids are minted from version strings, so `1.0.0` and `100` are
    ///   one id.
    ///
    /// The separator map lives in [`renormalize`] (the one map), so *recognition*
    /// forks with the mint in lockstep — the two rules cannot drift apart.
    #[test]
    fn slash_and_dot_are_separators_at_rule_version_2() {
        // The slash no longer fuses.
        assert_eq!(slugify("auth/session"), "auth-session");
        assert_eq!(slugify("src/main.rs"), "src-main-rs");

        // The dotted-version collision class is gone: the two inputs that minted
        // one id now mint two.
        assert_ne!(
            slugify("1.0"),
            slugify("10"),
            "the dotted-version collision class survives"
        );
        assert_eq!(slugify("1.0"), "1-0");
        assert_eq!(slugify("10"), "10");
        assert_eq!(slugify("1.0.0"), "1-0-0");

        // The one map: recognition forks with the mint (no cap, no stopword drop).
        assert_eq!(renormalize("auth/session"), "auth-session");
        assert_eq!(renormalize("1.0.0"), "1-0-0");

        // The declared version says so — the rule change and the version bump are
        // one event, and the pack-load fence checks this integer against every
        // shipped manifest.
        assert_eq!(SLUG_RULE_VERSION, 2);
    }

    /// The char backstop: a single long-word id-source (no `-` for the word cap
    /// to cut on) is bounded to [`MAX_CHARS`], so a pathological intent can never
    /// mint a slug that overruns the filesystem `NAME_MAX` when used verbatim as
    /// `<slug>.md`. The cut lands on a char boundary and leaves a valid slug.
    #[test]
    fn caps_single_long_word_to_char_backstop() {
        let input = "x".repeat(300);
        let out = slugify(&input);
        assert_eq!(out.len(), MAX_CHARS, "single long word not capped: {out:?}");
        assert!(is_slug(&out), "backstop output not a slug: {out:?}");
    }

    /// The word cap: an intent with more than [`MAX_WORDS`] words caps to the
    /// first [`MAX_WORDS`], at a `-` boundary, still a valid slug. The trailing
    /// stopword the cap exposes (`to`) is then dropped by the F5 edge-stopword
    /// rule.
    #[test]
    fn caps_to_five_words() {
        let out = slugify("move the session cache to a shared redis cluster");
        assert_eq!(out, "move-the-session-cache");
        assert!(
            out.split('-').count() <= MAX_WORDS,
            "over word cap: {out:?}"
        );
    }

    /// V7 char-cap word-boundary retreat: a single long compound whose
    /// [`MAX_CHARS`] cut lands mid-word retreats to the last **complete** word
    /// (via the final `-`), never a mid-word lop like `contrac`. The result is
    /// still a valid slug within the char backstop.
    #[test]
    fn char_cap_retreats_to_complete_word() {
        // 51 chars, 5 dash words; the 50-char cut lands inside the last word
        // `contract` → old behaviour lopped it to `contrac`.
        let out = slugify("backwards-compatible-serialization-outputs-contract");
        assert_eq!(out, "backwards-compatible-serialization-outputs");
        assert!(is_slug(&out), "retreat output not a slug: {out:?}");
        assert!(out.chars().count() <= MAX_CHARS, "over char cap: {out:?}");
        assert!(!out.contains("contrac"), "mid-word lop survived: {out:?}");
    }

    /// F5 edge-stopword drop: a leading **and** a trailing article are both
    /// dropped at the word cap, while a **medial** article is untouched.
    #[test]
    fn edge_stopwords_dropped_medial_kept() {
        // leading `a` + trailing `a` drop; medial `the` stays.
        assert_eq!(slugify("a node the app a"), "node-the-app");
        // the canonical `a`/`the` articles at either edge.
        assert_eq!(slugify("the guide of"), "guide");
        // a medial article is preserved.
        assert_eq!(slugify("Add a rate-limiter!"), "add-a-rate-limiter");
    }

    /// The deterministic collision suffix: the first instance (`nth == 1`) keeps the
    /// bare slug; each higher one takes `-2`, `-3`, … in task-id merge order
    /// (`structural-grammar.md` → IDs: provenance and minting).
    #[test]
    fn suffixed_keeps_bare_first_then_numbers() {
        assert_eq!(suffixed("cache-strategy", 1), "cache-strategy");
        assert_eq!(suffixed("cache-strategy", 2), "cache-strategy-2");
        assert_eq!(suffixed("cache-strategy", 3), "cache-strategy-3");
        // `0` is not a valid position; treat it as the bare (defensive, like `1`).
        assert_eq!(suffixed("x", 0), "x");
    }

    /// The slug *recognition* predicate: the grammar a `<type>:<slug>` ref body
    /// must match. Non-empty `[a-z0-9-]`, no leading/trailing/doubled `-`.
    #[test]
    fn is_slug_recognizes_well_formed_slugs() {
        // valid
        assert!(is_slug("use-postgres"));
        assert!(is_slug("a"));
        assert!(is_slug("single-node-cache"));
        assert!(is_slug("adr-2"));
        // invalid
        assert!(!is_slug(""), "empty");
        assert!(!is_slug("-lead"), "leading dash");
        assert!(!is_slug("trail-"), "trailing dash");
        assert!(!is_slug("a--b"), "doubled dash");
        assert!(!is_slug("Caps"), "uppercase");
        assert!(!is_slug("a:b"), "colon");
        assert!(!is_slug("a, adr:b"), "comma-form body");
        assert!(!is_slug("auth#criteria/rate-limit"), "fragment / slash");
        assert!(!is_slug("with space"), "space");
    }

    /// The fingerprint's input vector is **generated over the rule's own
    /// constants**, never hand-picked — the property that lets the fence catch a
    /// change nobody listed in advance (a hand-picked table would wave through the
    /// very separator mapping it exists to detect). Pins all four families.
    #[test]
    fn fingerprint_vector_is_generated_over_the_rule_constants() {
        let inputs = fingerprint_inputs();
        let has = |s: &str| inputs.iter().any(|i| i == s);

        // Every printable ASCII char, alone and framed — `/` and `.` (the chars the
        // M42 rule change maps) are in there by construction, not by anyone's memory.
        for byte in b' '..=b'~' {
            let ch = byte as char;
            assert!(
                has(&ch.to_string()),
                "printable ASCII {ch:?} missing (alone)"
            );
            assert!(
                has(&format!("a{ch}b")),
                "printable ASCII {ch:?} missing (framed)"
            );
        }
        // Every transliteration-table entry, in both frames.
        for (ch, _) in TRANSLITERATE {
            assert!(
                has(&ch.to_string()),
                "transliterate entry {ch:?} missing (alone)"
            );
            assert!(
                has(&format!("a{ch}b")),
                "transliterate entry {ch:?} missing (framed)"
            );
        }
        // Every edge stopword at each of the three positions.
        for word in EDGE_STOPWORDS {
            assert!(
                has(&format!("{word} alpha beta")),
                "{word}: leading missing"
            );
            assert!(has(&format!("alpha {word} beta")), "{word}: medial missing");
            assert!(
                has(&format!("alpha beta {word}")),
                "{word}: trailing missing"
            );
        }
        // Both caps are genuinely crossed by some input.
        assert!(
            inputs
                .iter()
                .any(|i| slugify(i).split('-').count() == MAX_WORDS),
            "no input crosses the word cap"
        );
        assert!(
            inputs
                .iter()
                .any(|i| slugify(i).chars().count() == MAX_CHARS),
            "no input crosses the char cap"
        );
    }

    /// The fingerprint is a stable 64-hex digest that **moves when the rule moves**
    /// — the property the pack-load gate rests on. Proven against the *generation-1*
    /// rule (`/` and `.` **stripped**, the shape this rule shipped as until the M42
    /// fork): re-running the generated vector under that rule yields a different
    /// digest, so the fence could not — and did not — sleep through the fork.
    ///
    /// The variant reconstructs generation 1 by pre-stripping the two chars the
    /// fork added to the separator map: with them gone from the input, generation
    /// 2's map has nothing to map, so `slugify` reproduces generation-1 output
    /// exactly.
    #[test]
    fn fingerprint_moves_when_the_rule_changes() {
        let today = rule_fingerprint();
        assert_eq!(today.len(), 64, "not a blake3 hex digest: {today:?}");
        assert!(
            today.chars().all(|c| c.is_ascii_hexdigit()),
            "not hex: {today:?}"
        );
        assert_eq!(today, rule_fingerprint(), "fingerprint not stable");

        // The generation-1 rule, applied over the same generated vector.
        let generation_1: Vec<String> = fingerprint_inputs()
            .iter()
            .map(|input| {
                let stripped: String = input.chars().filter(|c| *c != '/' && *c != '.').collect();
                format!("{input:?} -> {:?}", slugify(&stripped))
            })
            .collect();
        assert_ne!(
            hash_bytes(generation_1.join("\n").as_bytes()),
            today,
            "the fingerprint is blind to the `/`·`.` separator mapping"
        );
    }

    // Every non-empty `slugify` output is a valid slug (the dual property).
    proptest! {
        #[test]
        fn slugify_output_is_a_slug_or_empty(s in ".{0,400}") {
            let out = slugify(&s);
            prop_assert!(out.is_empty() || is_slug(&out), "not a slug: {:?}", out);
        }
    }

    #[test]
    fn cap_truncates_at_word_boundary() {
        let input =
            "this is a very long intent that should be truncated at a dash boundary near fifty";
        let out = slugify(input);
        assert!(
            out.split('-').count() <= MAX_WORDS,
            "over word cap: {out:?} ({} words)",
            out.split('-').count()
        );
        assert!(!out.ends_with('-'), "trailing dash: {out:?}");
        // The cut must land on a word boundary: the truncated slug is a dash-joined
        // prefix of the full (uncapped) word sequence. The oracle is the **production**
        // `renormalize` — steps 1–5, the uncapped slug — never a hand-copied shadow of
        // the separator map (which drifts silently on a rule change, and was this
        // test's oracle until M42 inc 10 killed it *by use*).
        let full = renormalize(input);
        assert!(
            full.starts_with(&out) && full.as_bytes().get(out.len()) == Some(&b'-'),
            "cut not at a '-' boundary: out={out:?} full={full:?}"
        );
    }

    /// Regression (M41 Inc8 F5 reconciliation): the char backstop (step 8) can
    /// truncate at a word boundary that exposes a **fresh** trailing edge
    /// stopword the word-cap pass (step 6) never saw — so the edge-stopword drop
    /// must also run *after* the char cap, or `slugify` is non-idempotent. This
    /// pins the two deterministic cases the (flaky) `idempotent` proptest shrank
    /// to.
    #[test]
    fn idempotent_when_char_cap_exposes_edge_stopword() {
        // The proptest shrink: the 50-char cap lops the final word, leaving a
        // trailing `-a` edge stopword a second pass would otherwise drop.
        let shrink = "Aa0-0a0AAaaÀ0aaA0Aa A_A0aaÀA0aA0AAaaA0ßaÀaAaaA0A0a";
        let once = slugify(shrink);
        assert_eq!(slugify(&once), once, "shrink not idempotent: {once:?}");
        assert!(
            !once.ends_with("-a"),
            "char cap left a fresh trailing edge stopword: {once:?}"
        );

        // A readable case: ≤5 words within the word cap (no trailing stopword at
        // the word level), but the char cap retreats past a long final word to
        // land on the medial `to` — exposing it as the new trailing edge.
        let readable = "reticulate splines to abcdefghijklmnopqrstuvwxyzabcd";
        let out = slugify(readable);
        assert_eq!(slugify(&out), out, "readable not idempotent: {out:?}");
        assert!(
            !out.ends_with("-to"),
            "char cap left a fresh trailing edge stopword: {out:?}"
        );
    }

    proptest! {
        /// Idempotence: a slug is a fixed point of the normalization.
        #[test]
        fn idempotent(s in ".{0,400}") {
            let once = slugify(&s);
            prop_assert_eq!(slugify(&once), once);
        }

        /// Charset + shape invariant: output is `^[a-z0-9-]*$`, never starts or
        /// ends with `-`, never contains `--`, and respects the word cap.
        #[test]
        fn charset_and_shape(s in ".{0,400}") {
            let out = slugify(&s);
            prop_assert!(
                out.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-'),
                "bad charset: {:?}", out
            );
            prop_assert!(!out.starts_with('-'), "leading dash: {:?}", out);
            prop_assert!(!out.ends_with('-'), "trailing dash: {:?}", out);
            prop_assert!(!out.contains("--"), "doubled dash: {:?}", out);
            prop_assert!(out.split('-').count() <= MAX_WORDS, "over word cap: {:?}", out);
            prop_assert!(out.chars().count() <= MAX_CHARS, "over char cap: {:?}", out);
        }
    }
}
