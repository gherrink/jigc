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
//! 6. cap at the first ~5 words (dash-separated), dropping the rest,
//! 7. (the word cap always cuts on a `-` boundary, so no trailing `-` is
//!    exposed and no re-trim is needed),
//! 8. apply a char-length backstop — truncate to a filesystem-safe length,
//!    trimming any `-` the cut exposes (bounds a single long word, which the
//!    word cap leaves untouched).
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
/// grammar a minted or authored slug must match — and the dual of [`slugify`]'s
/// *normalization*: every non-empty `slugify(x)` satisfies `is_slug`, and a
/// fixed point of `slugify` is exactly a valid slug. The length cap is *not*
/// enforced here (a cap is a mint-time concern; a ref pointing at an existing
/// slug must recognize it whatever its length).
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
/// words, dropping the rest.
///
/// The input is already collapsed and edge-trimmed, so splitting on `-` yields
/// clean, non-empty words; joining the first `MAX_WORDS` back with `-` always
/// lands on a word boundary and can never leave a leading, trailing, or doubled
/// `-`. A slug already within the cap is returned unchanged.
fn cap_words(s: &str) -> String {
    s.split('-').take(MAX_WORDS).collect::<Vec<_>>().join("-")
}

/// Char-length backstop: truncate a slug to at most [`MAX_CHARS`] characters,
/// then trim any `-` the cut exposed at the end.
///
/// Applied after [`cap_words`], this is the final ceiling that bounds a single
/// long word (which has no `-` for the word cap to cut on). The prior steps
/// guarantee the input is pure ASCII (`[a-z0-9-]`), so the `MAX_CHARS`-th char
/// boundary is also a byte boundary — but we cut on `char_indices` regardless so
/// the truncation can never split a multibyte char. A slug already within the
/// cap is returned unchanged.
fn cap_chars(s: &str) -> String {
    match s.char_indices().nth(MAX_CHARS) {
        None => s.to_string(),
        Some((byte_idx, _)) => s[..byte_idx].trim_end_matches('-').to_string(),
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
            ("collapse-dashes", "a---b__ c"),
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
        collapse-dashes: "a---b__ c" -> "a-b-c"
        trim-edges: "  -hello-  " -> "hello"
        ligatures: "Æsop & œuvre" -> "aesop-oeuvre"
        sharp-s: "Straße" -> "strasse"
        empty: "" -> ""
        strips-to-empty: "!!!___---" -> ""
        non-latin-dropped: "日本語 test" -> "test"
        cap-to-five-words: "this is a very long intent that should be truncated at a word boundary" -> "this-is-a-very-long"
        cap-one-long-word: "supercalifragilisticexpialidocioussupercalifragilisticexpialidocious" -> "supercalifragilisticexpialidocioussupercalifragili"
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
    /// first [`MAX_WORDS`], at a `-` boundary, still a valid slug.
    #[test]
    fn caps_to_five_words() {
        let out = slugify("move the session cache to a shared redis cluster");
        assert_eq!(out, "move-the-session-cache-to");
        assert!(
            out.split('-').count() <= MAX_WORDS,
            "over word cap: {out:?}"
        );
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
