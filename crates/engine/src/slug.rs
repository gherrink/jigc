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
//! 6. cap at ~50 chars, truncating at a `-` (word) boundary,
//! 7. re-trim any trailing `-` the cap exposed.
//!
//! The output always matches `^[a-z0-9-]*$` with no leading, trailing, or
//! doubled `-`, and the function is idempotent: `slugify(slugify(x)) ==
//! slugify(x)`.

/// Soft cap on slug length, in bytes/chars (the slug is pure ASCII, so the two
/// coincide). The cap truncates at a `-` boundary, never mid-word, so the
/// actual result may be shorter.
const MAX_LEN: usize = 50;

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

    // 6–7: cap at a '-' boundary, then re-trim.
    cap_at_boundary(&collapsed)
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

/// Cap a collapsed, trimmed slug at [`MAX_LEN`], truncating at a `-` boundary.
///
/// If the slug is already within the cap, it is returned as-is. Otherwise we
/// cut at the last `-` at or before the cap so we never split a word; if there
/// is no `-` within the cap (one long word), we hard-cut at the cap. A trailing
/// `-` left by the cut is trimmed.
fn cap_at_boundary(s: &str) -> String {
    if s.len() <= MAX_LEN {
        return s.to_string();
    }
    // Prefer the last '-' at or before MAX_LEN (a word boundary).
    let head = &s[..MAX_LEN];
    let cut = match head.rfind('-') {
        Some(i) if i > 0 => i,
        _ => MAX_LEN, // no usable boundary: hard-cut the single long word.
    };
    s[..cut].trim_end_matches('-').to_string()
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
                "cap-at-boundary-50",
                "this is a very long intent that should be truncated at a dash boundary near fifty",
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
        cap-at-boundary-50: "this is a very long intent that should be truncated at a dash boundary near fifty" -> "this-is-a-very-long-intent-that-should-be"
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

    #[test]
    fn cap_truncates_at_dash_boundary() {
        let input =
            "this is a very long intent that should be truncated at a dash boundary near fifty";
        let out = slugify(input);
        assert!(out.len() <= MAX_LEN, "over cap: {out:?} ({})", out.len());
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
        fn idempotent(s in ".{0,200}") {
            let once = slugify(&s);
            prop_assert_eq!(slugify(&once), once);
        }

        /// Charset + shape invariant: output is `^[a-z0-9-]*$`, never starts or
        /// ends with `-`, never contains `--`, and respects the cap.
        #[test]
        fn charset_and_shape(s in ".{0,200}") {
            let out = slugify(&s);
            prop_assert!(
                out.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-'),
                "bad charset: {:?}", out
            );
            prop_assert!(!out.starts_with('-'), "leading dash: {:?}", out);
            prop_assert!(!out.ends_with('-'), "trailing dash: {:?}", out);
            prop_assert!(!out.contains("--"), "doubled dash: {:?}", out);
            prop_assert!(out.len() <= MAX_LEN, "over cap: {:?}", out);
        }
    }
}
