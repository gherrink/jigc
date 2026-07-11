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
//! The rule, in order:
//! 1. lowercase (ASCII case-fold),
//! 2. transliterate the common Latin accented letters to ASCII (`café`→`cafe`),
//! 3. map spaces and `_` to `-`,
//! 4. strip every remaining char outside `[a-z0-9-]`,
//! 5. collapse runs of `-` and trim leading/trailing `-`,
//! 6. cap at the first ~5 words (dash-separated), dropping the rest, then drop
//!    any leading/trailing stopwords (`the`/`a`/`of`…) the cap leaves at an
//!    edge — a medial stopword is untouched,
//! 7. (the word cap always cuts on a `-` boundary, so no trailing `-` is
//!    exposed and no re-trim is needed),
//! 8. apply a char-length backstop — on a mid-word cut, retreat to the last
//!    complete word (via the final `-`); a single long word with no `-` still
//!    hard-truncates to a filesystem-safe length (the `NAME_MAX` backstop).
//!
//! The output always matches `^[a-z0-9-]*$` with no leading, trailing, or
//! doubled `-`, and the function is idempotent: `slugify(slugify(x)) ==
//! slugify(x)`.

/// Cap on slug length, in **words** (dash-separated segments). A minted slug
/// keeps at most the first `MAX_WORDS` words of its id-source and drops the
/// rest, so a long intent yields a short, legible slug rather than one that
/// trails off into filler. The cut always lands on a `-` boundary, never
/// mid-word. This is a mint-time cap only; [`is_slug`] is uncapped so a ref to
/// a pre-existing longer slug still resolves.
const MAX_WORDS: usize = 5;

/// Character-length backstop, applied *after* the word cap. The word cap alone
/// leaves a single long word (no `-` to cut on) unbounded, and a minted slug is
/// used verbatim as a filename (`<location>/<slug>.md`), so a pathological
/// single-word intent could overrun the filesystem `NAME_MAX` (~255 bytes) and
/// fail the write. This final ceiling truncates such a slug to a filesystem-safe
/// length. `50` restores the pre-M39 backstop — well under `NAME_MAX` even with
/// the `.md` suffix and any `-N` collision suffix. Normal ≤5-word intents sit
/// far below it, so multi-word behaviour is unchanged.
const MAX_CHARS: usize = 50;

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

/// Normalize an id-source into a frozen content-slug.
///
/// See the [module docs](self) for the full rule. Pure and total: every input
/// maps to a valid slug (`^[a-z0-9-]*$`); an input with no slug-able content
/// maps to `""` (the caller supplies the type-name fallback).
pub fn slugify(id_source: &str) -> String {
    // 1–4: case-fold, transliterate, separator-map, strip.
    let mut out = String::with_capacity(id_source.len());
    for ch in id_source.chars() {
        match ch {
            ' ' | '_' => out.push('-'),
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
    let collapsed = collapse_dashes(&out);

    // 6–7: cap at the first MAX_WORDS words (always a '-' boundary).
    let capped = cap_words(&collapsed);

    // 8: char-length backstop — bound a single long word (which the word cap
    // leaves untouched) to a filesystem-safe length.
    cap_chars(&capped)
}

/// Transliterate a single non-ASCII char to its ASCII lowercase skeleton, or
/// `None` if it has no sensible ASCII fold (then it is stripped).
///
/// Covers the common Latin-1 / Latin Extended-A accented letters and ligatures
/// — enough for the settled `café`→`cafe` rule and ordinary Latin-script
/// titles. Deliberately small and hand-rolled (no transliteration crate): the
/// slug is a *legibility* aid, not a faithful romanization, so anything outside
/// this table is simply dropped.
fn transliterate(ch: char) -> Option<&'static str> {
    let s = match ch {
        'à' | 'á' | 'â' | 'ã' | 'ä' | 'å' | 'À' | 'Á' | 'Â' | 'Ã' | 'Ä' | 'Å' => "a",
        'æ' | 'Æ' => "ae",
        'ç' | 'Ç' => "c",
        'è' | 'é' | 'ê' | 'ë' | 'È' | 'É' | 'Ê' | 'Ë' => "e",
        'ì' | 'í' | 'î' | 'ï' | 'Ì' | 'Í' | 'Î' | 'Ï' => "i",
        'ñ' | 'Ñ' => "n",
        'ò' | 'ó' | 'ô' | 'õ' | 'ö' | 'ø' | 'Ò' | 'Ó' | 'Ô' | 'Õ' | 'Ö' | 'Ø' => "o",
        'œ' | 'Œ' => "oe",
        'ù' | 'ú' | 'û' | 'ü' | 'Ù' | 'Ú' | 'Û' | 'Ü' => "u",
        'ý' | 'ÿ' | 'Ý' | 'Ÿ' => "y",
        'ß' => "ss",
        _ => return None,
    };
    Some(s)
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
/// words, then drop any leading/trailing [`EDGE_STOPWORDS`] the cap exposes.
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
    let mut words: Vec<&str> = s.split('-').take(MAX_WORDS).collect();
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
        ];
        let table: Vec<String> = cases
            .iter()
            .map(|(label, input)| format!("{label}: {input:?} -> {:?}", slugify(input)))
            .collect();
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
        // The cut must land on a word boundary: the truncated slug is a
        // dash-joined prefix of the full (uncapped) word sequence.
        let full = slugify_uncapped(input);
        assert!(
            full.starts_with(&out) && full.as_bytes().get(out.len()) == Some(&b'-'),
            "cut not at a '-' boundary: out={out:?} full={full:?}"
        );
    }

    /// The full slug with the length cap *not* applied — for asserting the cap
    /// cuts on a real boundary. Mirrors steps 1–5 of [`slugify`].
    fn slugify_uncapped(s: &str) -> String {
        let mut out = String::new();
        for ch in s.chars() {
            match ch {
                ' ' | '_' => out.push('-'),
                'a'..='z' | '0'..='9' | '-' => out.push(ch),
                'A'..='Z' => out.push(ch.to_ascii_lowercase()),
                _ => {
                    if let Some(f) = transliterate(ch) {
                        out.push_str(f);
                    }
                }
            }
        }
        collapse_dashes(&out)
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
