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
//!    edge — a medial stopword is untouched, and so is a stopword that is only a
//!    **component of a hyphenated compound** (`On-call` keeps its `on`; the
//!    [rule-version-3 fork](#the-rule-is-itself-versioned-m42)),
//! 7. (the word cap always cuts on a `-` boundary, so no trailing `-` is
//!    exposed and no re-trim is needed),
//! 8. apply a char-length backstop — on a mid-word cut, retreat to the last
//!    complete word (via the final `-`); a single long word with no `-` still
//!    hard-truncates to a filesystem-safe length (the `NAME_MAX` backstop),
//! 9. re-drop any leading/trailing *droppable* stopword the char cap of step 8
//!    *exposes* — the retreat to a complete word can uncover an edge stopword step
//!    6 never saw — so mint output never ends (or starts) on standalone filler.
//!
//! The output always matches `^[a-z0-9-]*$` with no leading, trailing, or
//! doubled `-`, and the function is **idempotent by construction**:
//! `slugify(slugify(x)) == slugify(x)`. Step 9 is what secures this — without a
//! post-cap edge-stopword drop, the step-8 char cap could leave a fresh trailing
//! stopword that a second pass would strip. It stays secured under the
//! generation-3 word-awareness because a second pass's boundaries are *all* `-`
//! join chars, so nothing at an edge is separator-delimited — and the one shape
//! that could escape that, a slug reduced to a lone stopword, is always droppable
//! and so was already dropped on the first pass.
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
//!
//! **Version 3 (M47) is the second fork**: the edge-stopword drop (steps 6 and 9)
//! becomes **word-aware**. Generation 2 tokenized a hyphenated compound *before*
//! dropping edge stopwords, so a compound's first or last component was eaten:
//! `"On-call handoff artifact"` minted `call-handoff-artifact` and
//! `"Telemetry consent opt-in"` minted `telemetry-consent-opt`. Under generation 3
//! an edge token is droppable only when the boundary joining it to the rest of the
//! slug came from a **separator** (space, `_`, `/`, `.`) or the string edge — i.e.
//! only when it is a whole separator-delimited word of the id-source. `-` is the
//! join char, so `on_call handoff artifact` still drops its `on`. The provenance is
//! carried **positionally** (each word knows what its own left boundary was), not as
//! a set of "words that appeared whole", so `"on On-call"` drops the first `on` and
//! keeps the compound's. Only [`slugify`] forks: [`renormalize`] is byte-unchanged,
//! so all four recognition sites read committed bytes exactly as before.

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

/// The **stated-at statement** of the mint rule — the sentence a soliciting
/// surface renders at the id-source it slugs, so the rule is stated where it
/// binds, before it can surprise (`design/surface-contract.md` → The stated-at
/// fence, seam-generated tier; law 3). Built from the rule's own constants
/// ([`MAX_WORDS`]/[`MAX_CHARS`]/[`EDGE_STOPWORDS`]) — statement and enforcement
/// share one source, so drift between them is unrepresentable.
///
/// It states **all four** steps a reader can be surprised by, not only the caps
/// (B7, the M47 fix): the **renormalization** (a `.` splits a token, so `v1.1` is
/// two words — the reported "roughly five tokens, except when it isn't" was
/// entirely this), the strip, the two caps, and the **edge-stopword drop** with
/// its generation-3 glue condition. Naming the caps alone described a rule the
/// mint does not implement.
///
/// `minted_into` names *what* the id-source is slugged into, so each surface
/// stays honest: the `{{schema:<doctype>}}` projection at the `title:` line
/// passes `"the doc id"`; `jigc doc create --help` and `jigc doc add-item --help`
/// — the two soliciting help surfaces, the latter minting an item's `{#id}`
/// anchor from `--title` the same way — pass the doc id and the item anchor. Only
/// the drift-prone rule clauses are shared.
pub fn mint_statement(minted_into: &str) -> String {
    let filler = EDGE_STOPWORDS.join("/");
    format!(
        "the id-source — slugged lowercase-kebab into {minted_into}: space, `_`, `/` and \
         `.` each become `-` (so `v1.1` is two words) and every other non-alphanumeric is \
         dropped; the result is capped at the first {MAX_WORDS} words / {MAX_CHARS} chars; \
         then a leading or trailing filler word ({filler}) is dropped unless a hyphen glues \
         it to its neighbour"
    )
}

/// Conservative English **edge-stopword** set — articles plus the short
/// prepositions — dropped when the word cap leaves one at the leading or
/// trailing edge of a minted slug (`DECISIONS.md` 2026-07-11 → Fork 5: drop
/// leading/trailing `the`/`a`/`of`…). A *medial* stopword is untouched (it
/// carries meaning between two content words, e.g. `add-a-rate-limiter`), and so
/// is one that is merely a **component of a hyphenated compound** (`on-call` keeps
/// its `on` — the generation-3 fork; see [`drop_edge_stopwords`]). The set is
/// deliberately small: the slug is a legibility aid, so we trim only the filler
/// that reads as noise at an edge, never content.
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
/// `slugify` fixed point — the mint-time word cap means `is_slug("a-b-c-d-e-f")`
/// yet `slugify("a-b-c-d-e-f") == "a-b-c-d-e"`, and the edge-stopword drop means
/// `is_slug("the")` yet `slugify("the") == ""`. (The generation-3 fork narrowed the
/// second class but did not close it: `slugify("a-0") == "a-0"` now, because a
/// hyphen-glued edge stopword survives.) So a *recognition* site
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
    // 1–5: the one separator map, in the **word view** — each word carries the
    // provenance of the boundary on its left, which steps 6/9 need and the joined
    // string cannot express.
    let mut words = tokenize(id_source);

    // 6–7: cap at the first MAX_WORDS words (always a '-' boundary), then drop
    // the edge stopwords the cap leaves (so a leading stopword frees a word slot).
    words.truncate(MAX_WORDS);
    drop_edge_stopwords(&mut words);

    // 8: char-length backstop — bound a single long word (which the word cap
    // leaves untouched) to a filesystem-safe length.
    cap_chars(&mut words);

    // 9: the char cap can truncate at a word boundary that leaves a *fresh*
    // trailing (or, after a doc-level cut, leading) edge stopword step 6 never
    // saw, so re-drop edge stopwords here. This makes `slugify` idempotent by
    // construction: mint output carries no *droppable* edge stopword, and a second
    // pass — whose every boundary is a `-` join char — finds none to drop.
    drop_edge_stopwords(&mut words);

    words
        .iter()
        .map(|word| word.text.as_str())
        .collect::<Vec<_>>()
        .join("-")
}

/// One word of a tokenized id-source, carrying the **provenance of the boundary on
/// its left**: `glue_before` is `true` only when the dash preceding it came from a
/// literal `-` in the id-source — the join char of a hyphenated compound — rather
/// than from a *separator* (space, `_`, `/`, `.`) or the string edge.
///
/// That one bit is the whole generation-3 fork: it is what tells `On-call`'s `on`
/// (glued) from `the guide`'s `the` (separator-delimited), and it is carried
/// **positionally** so `"on On-call"` drops the first `on` and keeps the compound's
/// — a value-set test ("is `on` a word that appeared whole anywhere?") would eat
/// both.
struct Word {
    text: String,
    glue_before: bool,
}

/// Tokenize an id-source into [`Word`]s — steps 1–5 in the word view.
///
/// `tokenize(x).join("-") == renormalize(x)` **by construction**: this performs the
/// identical classification (case-fold, transliterate, separator-map, strip) and
/// the identical collapse/trim, and only additionally records where each surviving
/// dash *came from*. The agreement is pinned by
/// [`tests::tokenize_agrees_with_renormalize`], so the mint rule can never drift
/// from the one separator map the recognition sites read.
///
/// A run of dash-producing chars collapses to one boundary, and that boundary is
/// glue only when **every** char in the run was a literal `-` (so `a - b` is
/// separator-delimited, not glued). A leading or trailing run is trimmed away
/// entirely, leaving the string edge — which is never glue.
fn tokenize(id_source: &str) -> Vec<Word> {
    let mut words: Vec<Word> = Vec::new();
    let mut current = String::new();
    // The boundary on the left of the word being accumulated. Word 0's is the
    // string start.
    let mut current_glue = false;
    // The pending dash run between the last content char and the next one.
    let mut dash_open = false;
    let mut dash_literal_only = true;

    for ch in id_source.chars() {
        // 3: the separators map to `-` and mark the run as *not* glue.
        if matches!(ch, ' ' | '_' | '/' | '.') {
            dash_open = true;
            dash_literal_only = false;
            continue;
        }
        // The join char: a dash whose provenance is the compound itself.
        if ch == '-' {
            dash_open = true;
            continue;
        }
        // 1–2, 4: case-fold / transliterate / strip.
        let ascii = match ch {
            'a'..='z' | '0'..='9' => Some(ch),
            'A'..='Z' => Some(ch.to_ascii_lowercase()),
            _ => None,
        };
        let folded = if ascii.is_some() {
            None
        } else {
            transliterate(ch)
        };
        if ascii.is_none() && folded.is_none() {
            // Stripped entirely — and a stripped char neither opens nor closes a
            // dash run (`the-, guide` is one boundary, carrying the space).
            continue;
        }

        // 5: this content char closes any pending dash run, which becomes the next
        // word's left boundary. A run before the first content char is the trim.
        if dash_open {
            if !current.is_empty() {
                words.push(Word {
                    text: std::mem::take(&mut current),
                    glue_before: current_glue,
                });
                current_glue = dash_literal_only;
            }
            dash_open = false;
            dash_literal_only = true;
        }
        if let Some(ch) = ascii {
            current.push(ch);
        } else if let Some(folded) = folded {
            current.push_str(folded);
        }
    }

    // A trailing dash run is trimmed: the last word's right boundary is the string
    // edge, so nothing is recorded for it.
    if !current.is_empty() {
        words.push(Word {
            text: current,
            glue_before: current_glue,
        });
    }
    words
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

/// Drop any leading/trailing [`EDGE_STOPWORDS`] from a tokenized slug, greedily at
/// each edge, leaving medial stopwords untouched.
///
/// **Word-aware since generation 3**: an edge word is droppable only when the
/// boundary joining it to the rest of the slug is *not* [glue][`Word`] — that is,
/// only when it is a whole separator-delimited word of the id-source rather than a
/// component of a hyphenated compound it is still attached to. A *lone* word has no
/// such boundary and is always droppable, which is what keeps `slugify` idempotent
/// when the char cap strands a compound's first component (see the module docs).
///
/// Run twice on the [`slugify`] path: once after the word cap (so a leading
/// stopword frees a word slot before the cap counts words), and once as the final
/// step after [`cap_chars`] (so an edge stopword the char cap *exposes* at the
/// truncation boundary is dropped too). The second pass is what makes `slugify`
/// idempotent: its output never carries a *droppable* edge stopword, and a re-run
/// sees only `-` join chars, so every surviving edge stopword is glued. An
/// all-stopword input strips to `""` (the caller supplies the type-name fallback).
fn drop_edge_stopwords(words: &mut Vec<Word>) {
    // Leading: the boundary is the one on the word's RIGHT — i.e. the next word's
    // `glue_before`. A lone word (no next) has none.
    while words
        .first()
        .is_some_and(|word| EDGE_STOPWORDS.contains(&word.text.as_str()))
        && words.get(1).is_none_or(|next| !next.glue_before)
    {
        words.remove(0);
    }
    // Trailing: the boundary is the word's OWN `glue_before` (its right side is the
    // string edge). A lone word's is `false` by construction — the string start.
    while words
        .last()
        .is_some_and(|word| EDGE_STOPWORDS.contains(&word.text.as_str()) && !word.glue_before)
    {
        words.pop();
    }
}

/// Char-length backstop: bound a slug to at most [`MAX_CHARS`] characters,
/// retreating to the last **complete** word when the cut lands mid-word.
///
/// Applied after the word cap, this is the final ceiling that bounds a slug whose
/// words (or single word) overrun the filesystem-safe length: keep the longest
/// **whole-word prefix** that fits, so the slug ends on a complete word
/// (`…-outputs-contract` → `…-outputs`, never the mid-word lop `…-contrac`). When
/// not even the *first* word fits there is no `-` to retreat to, so it
/// hard-truncates at `MAX_CHARS` — the `NAME_MAX` backstop that keeps a
/// pathological single-word intent from overrunning `<slug>.md`. The prior steps
/// guarantee pure ASCII (`[a-z0-9-]`), so the `MAX_CHARS`-th char boundary is also
/// a byte boundary — but we cut on `char_indices` regardless so the truncation can
/// never split a multibyte char. A slug already within the cap is left unchanged.
fn cap_chars(words: &mut Vec<Word>) {
    let mut used = 0usize;
    let mut keep = words.len();
    for (i, word) in words.iter().enumerate() {
        // Every word but the first carries the `-` that joins it.
        let len = word.text.chars().count() + usize::from(i > 0);
        if used + len > MAX_CHARS {
            keep = i;
            break;
        }
        used += len;
    }
    if keep == 0 && !words.is_empty() {
        // A first word that alone overruns: hard-truncate it (NAME_MAX floor).
        let first = &mut words[0];
        if let Some((byte_idx, _)) = first.text.char_indices().nth(MAX_CHARS) {
            first.text.truncate(byte_idx);
        }
        keep = 1;
    }
    words.truncate(keep);
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
/// **3** (M47): the edge-stopword drop is word-aware — a compound's edge component
/// (`On-call`'s `on`) survives, where generation 2 ate it. **2** (M42): `/` and `.`
/// map to `-` instead of being stripped. Both forks are in the [module
/// docs](self#the-rule-is-itself-versioned-m42). **1** was the M39 word-capped rule
/// as first shipped; a corpus older than a bump carries its ids' generation forever.
pub const SLUG_RULE_VERSION: u32 = 3;

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
///   medial, trailing), **and hyphen-glued at each edge** — so widening or
///   narrowing the set moves the hash, and so does a change to *which* edge tokens
///   are droppable (the generation-3 fork: without the glued frames the whole
///   word-awareness class moved exactly one input in the whole vector — family 1's
///   framed `-` — so the fence would have caught it only by luck);
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

    // 3. Every edge stopword at each of the three positions, plus hyphen-glued at
    //    each edge (the generation-3 axis: glued components are NOT droppable,
    //    while the same word separator-delimited is).
    for word in EDGE_STOPWORDS {
        inputs.push(format!("{word} alpha beta"));
        inputs.push(format!("alpha {word} beta"));
        inputs.push(format!("alpha beta {word}"));
        inputs.push(format!("{word}-alpha beta"));
        inputs.push(format!("alpha beta-{word}"));
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
            // M47 Inc 1 — the word-aware fork, at slug-rule-version 3: an edge
            // stopword that is only a COMPONENT of a hyphenated compound survives
            // (generation 2 minted `call-handoff-artifact` / `telemetry-consent-opt`),
            // while the same word separator-delimited still drops. These four lines
            // are that reviewable diff.
            ("glued-leading-stopword-kept", "On-call handoff artifact"),
            ("glued-trailing-stopword-kept", "Telemetry consent opt-in"),
            ("underscore-delimited-stopword-dropped", "on_call handoff"),
            ("glue-provenance-is-positional", "on On-call"),
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
        glued-leading-stopword-kept: "On-call handoff artifact" -> "on-call-handoff-artifact"
        glued-trailing-stopword-kept: "Telemetry consent opt-in" -> "telemetry-consent-opt-in"
        underscore-delimited-stopword-dropped: "on_call handoff" -> "call-handoff"
        glue-provenance-is-positional: "on On-call" -> "on-call"
        "#);
    }

    /// The mint rule composes over the **one** separator map: `tokenize` performs
    /// the identical steps 1–5 as [`renormalize`] and only additionally records each
    /// dash's provenance, so its words rejoin to exactly `renormalize`'s output.
    ///
    /// This is the load-bearing invariant of the generation-3 fork: `renormalize` is
    /// **byte-unchanged**, so the four production *recognition* sites
    /// (`parse::heading_matches`, `write::present_body_sections`,
    /// `validate::surplus_sections_absent`, `migrate_corpus::has_section_heading`)
    /// read committed bytes exactly as before — and the fork cannot smuggle a
    /// separator-map change in through the tokenizer's back door.
    #[test]
    fn tokenize_agrees_with_renormalize() {
        for source in [
            "",
            "!!!___---",
            "  -hello-  ",
            "x---b__ c",
            "On-call handoff artifact",
            "on On-call",
            "auth/session",
            "1.0.0",
            "src/main.rs",
            "Æsop & œuvre",
            "Straße",
            "café",
            "日本語 test",
            "the-, guide",
            "a - b",
            "trailing-",
            "-leading",
        ] {
            let joined = tokenize(source)
                .iter()
                .map(|word| word.text.as_str())
                .collect::<Vec<_>>()
                .join("-");
            assert_eq!(
                joined,
                renormalize(source),
                "tokenize drifted from the one separator map on {source:?}"
            );
        }
    }

    proptest! {
        /// The same agreement, sampled — the finder beside the deterministic fence
        /// above.
        #[test]
        fn tokenize_agrees_with_renormalize_over_arbitrary_text(s in ".{0,400}") {
            let joined = tokenize(&s)
                .iter()
                .map(|word| word.text.as_str())
                .collect::<Vec<_>>()
                .join("-");
            prop_assert_eq!(joined, renormalize(&s));
        }
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
        // (The declared version this fork shipped at is 2; the *current* version and
        // its fingerprint are pinned by `the_declared_generation_pins_the_shipped_fingerprint`,
        // which is the one place the shipped integer is asserted — so a later fork
        // does not have to edit its predecessors' tests.)
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

    /// **The M47 fork (slug-rule-version 3).** The edge-stopword drop is
    /// **word-aware**: a token at an edge is droppable only when the boundary
    /// joining it to the rest of the slug came from a *separator* (space, `_`,
    /// `/`, `.`) or the string edge — i.e. it is a whole separator-delimited word
    /// of the id-source, never a component of a hyphenated compound it is still
    /// attached to. Under generation 2 the tokenizer split the compound first, so
    /// `"On-call handoff artifact"` minted `call-handoff-artifact` and
    /// `"Telemetry consent opt-in"` minted `telemetry-consent-opt` — and no shipped
    /// command could reach the correct identity (`add-item` has no `--slug`,
    /// `retitle-item` freezes the anchor).
    ///
    /// Iterates the axis {leading · trailing · char-cap-exposed} × {whole source
    /// word ⇒ dropped, hyphen-glued component ⇒ kept}, so the fix is proven over
    /// its class rather than its reported repro.
    #[test]
    fn edge_stopword_drop_is_word_aware_at_rule_version_3() {
        // 43 `a`s + a hyphen-glued `opt-in` + 20 `z`s: the 50-char cap lands on the
        // `-` after `in`, so the compound's trailing `in` is *exposed* at the edge
        // by the char cap (step 9) — the case step 6 never sees. At generation 2
        // this minted `…-opt`.
        let capped_glued = format!("{} opt-in {}", "a".repeat(43), "z".repeat(20));
        let capped_glued_expected = format!("{}-opt-in", "a".repeat(43));

        let cases: [(&str, &str, &str, &str); 6] = [
            // (edge, provenance, id-source, minted id)
            ("leading", "whole source word", "the guide of", "guide"),
            (
                "leading",
                "hyphen-glued component",
                "On-call handoff artifact",
                "on-call-handoff-artifact",
            ),
            (
                "trailing",
                "whole source word",
                "handoff artifact for",
                "handoff-artifact",
            ),
            (
                "trailing",
                "hyphen-glued component",
                "Telemetry consent opt-in",
                "telemetry-consent-opt-in",
            ),
            (
                "char-cap-exposed",
                "whole source word",
                "reticulate splines to abcdefghijklmnopqrstuvwxyzabcd",
                "reticulate-splines",
            ),
            (
                "char-cap-exposed",
                "hyphen-glued component",
                &capped_glued,
                &capped_glued_expected,
            ),
        ];
        for (edge, provenance, source, expected) in cases {
            assert_eq!(
                slugify(source),
                expected,
                "{edge} / {provenance}: slugify({source:?})"
            );
        }

        // The decided `_` cell: "separator-delimited" resolves against the rule's
        // OWN separator map (space, `_`, `/`, `.`), and `-` is the join char — so
        // `on_call` is two whole words and its leading `on` still drops. Decided,
        // not accidental (`DECISIONS.md` → 2026-07-26 M47 planning → Decision 2).
        assert_eq!(slugify("on_call handoff artifact"), "call-handoff-artifact");

        // The provenance is carried POSITIONALLY, not as a value-set membership
        // test: the first `on` is a whole source word and drops; the second is the
        // compound's first component and survives.
        assert_eq!(slugify("on On-call"), "on-call");

        // The declared version says so — the rule change and the version bump are
        // one event, and the pack-load fence checks this integer against every
        // shipped manifest.
        assert_eq!(SLUG_RULE_VERSION, 3);
    }

    /// The **generation table**: every shipped `slug-rule.version` paired with the
    /// fingerprint the engine computed under it. Generation 1 predates
    /// [`rule_fingerprint`], so the table starts at 2.
    ///
    /// This is what makes the version bump a **tested obligation**: the
    /// [`crate::manifest`] version arm compares the *manifest* against
    /// [`SLUG_RULE_VERSION`], and one author edits both sides — so a rule change
    /// re-pinned into both manifests without bumping the integer passes every
    /// existing fence. Here the fingerprint must equal the entry recorded *for the
    /// declared version*, and all entries must be distinct: change the rule without
    /// bumping and the current version's pinned digest no longer matches; bump
    /// without changing and the new entry collides with its predecessor.
    const GENERATIONS: &[(u32, &str)] = &[
        (
            2,
            "de51355900e8f0b268030c1da591f3b1c9a49ce63a9b818b71e1c97a14a40a83",
        ),
        (
            3,
            "1291873dcac22ad132c3cfdb1cd507a83e09fa1af552d35af4d5ae2a6f8e8e35",
        ),
    ];

    /// The bump is an obligation, not an option — see [`GENERATIONS`].
    #[test]
    fn the_declared_generation_pins_the_shipped_fingerprint() {
        let pinned = GENERATIONS
            .iter()
            .find(|(version, _)| *version == SLUG_RULE_VERSION)
            .unwrap_or_else(|| {
                panic!("slug-rule version {SLUG_RULE_VERSION} has no GENERATIONS entry")
            });
        assert_eq!(
            rule_fingerprint(),
            pinned.1,
            "the shipped fingerprint is not the one recorded for slug-rule version \
             {SLUG_RULE_VERSION} — a rule change owes its declared version bump + a new \
             GENERATIONS entry, in the same commit that re-pins every manifest",
        );

        // Every generation is a distinct rule: a bump that changes nothing, or a
        // rule change that reuses a recorded digest, is a lie.
        for (i, (version, hash)) in GENERATIONS.iter().enumerate() {
            for (other_version, other_hash) in &GENERATIONS[i + 1..] {
                assert_ne!(
                    hash, other_hash,
                    "generations {version} and {other_version} pin the same fingerprint"
                );
            }
        }
        assert!(
            GENERATIONS.iter().any(|(v, _)| *v == SLUG_RULE_VERSION),
            "the shipped version is recorded"
        );
    }

    /// Idempotence at the generation-3 fork, deterministically (the proptest beside
    /// it is a finder, not a fence — `implementation/dev-workflow.md` → *a proptest
    /// is not a gate*). A surviving hyphen-glued edge stopword must still be a fixed
    /// point: a second pass sees only `-` join chars, so no edge word is a whole
    /// separator-delimited word — and a *lone* stopword is always droppable, which is
    /// what keeps the char-cap retreat idempotent when it strands one.
    #[test]
    fn idempotent_when_a_hyphen_glued_edge_stopword_survives() {
        for source in [
            "On-call handoff artifact",
            "Telemetry consent opt-in",
            "on On-call",
            // The char cap keeps the leading prefix, stranding the compound's first
            // component alone: a LONE edge stopword is always droppable, so the
            // output is never the non-fixed-point `on`.
            &format!("on-{}", "z".repeat(60)),
        ] {
            let once = slugify(source);
            assert_eq!(slugify(&once), once, "{source:?} not idempotent: {once:?}");
        }
        // …and the strand normalizes away entirely — the sanctioned escape of the
        // total function, exactly as a bare `slugify("on")` does; the mint site
        // supplies the type-name fallback.
        assert_eq!(slugify(&format!("on-{}", "z".repeat(60))), "");
        assert_eq!(slugify("on"), "");
    }

    /// **B7 (M47) — the mint rule states itself where it mints.** The stated-at
    /// sentence named only the two *caps*, so the two steps that produce every
    /// reported "except when it isn't" were stated nowhere: the **renormalization**
    /// (a `.` splits a token, so `v1.1` is two words — the whole `scope-v1-1`
    /// surprise) and the **edge-stopword drop** (a leading/trailing filler word is
    /// dropped, unless a hyphen glues it to its neighbour).
    ///
    /// Every clause is checked **against the rule itself**, never against a
    /// hand-typed mirror — a sentence that lists members by hand is exactly the
    /// failure mode this fences:
    ///
    /// - the stopword members are **parsed back out of the sentence** and compared
    ///   with [`EDGE_STOPWORDS`], so widening or narrowing the const reddens here
    ///   until the sentence follows;
    /// - the separator set is derived from **behaviour** — every printable-ASCII
    ///   char [`renormalize`] maps to `-` must be named, and every char it does not
    ///   must not be, so a separator-map fork cannot leave the sentence behind;
    /// - each listed member is *shown* to drop as a whole source word and to
    ///   survive hyphen-glued, so the sentence cannot over-claim the set.
    #[test]
    fn mint_statement_states_the_whole_mint_rule() {
        let statement = mint_statement("the doc id");

        // The caps clause (M43), still built from the enforcing constants.
        assert!(
            statement.contains(&format!("first {MAX_WORDS} words / {MAX_CHARS} chars")),
            "the statement must state the mint caps; got: {statement}"
        );

        // The edge-stopword set, read back out of the sentence and compared with
        // the const that enforces it.
        let listed = statement
            .split_once("filler word (")
            .and_then(|(_, rest)| rest.split_once(')'))
            .map(|(list, _)| list)
            .unwrap_or_else(|| {
                panic!(
                    "the statement must name the edge-stopword set as \
                     `filler word (<members>)`; got: {statement}"
                )
            });
        assert_eq!(
            listed.split('/').collect::<Vec<_>>(),
            EDGE_STOPWORDS.to_vec(),
            "the sentence's filler-word list and EDGE_STOPWORDS disagree"
        );

        // The separator map, derived from behaviour in both directions. `-` is the
        // join char, not a separator, and the sentence names it as the target.
        for byte in b' '..=b'~' {
            let ch = byte as char;
            if ch == '-' {
                continue;
            }
            let maps = renormalize(&format!("a{ch}b")) == "a-b";
            assert_eq!(
                maps,
                statement.contains(&statement_rendering(ch)),
                "the statement and the separator map disagree about {ch:?} \
                 (maps to `-`: {maps}); got: {statement}"
            );
        }

        // Each listed member really is droppable as a whole source word, and really
        // does survive hyphen-glued — the two halves the sentence claims.
        for word in EDGE_STOPWORDS {
            assert_eq!(
                slugify(&format!("{word} alpha beta")),
                "alpha-beta",
                "{word} is listed as filler but does not drop as a whole source word"
            );
            assert_eq!(
                slugify(&format!("{word}-alpha beta")),
                format!("{word}-alpha-beta"),
                "{word} is listed as filler but does not survive hyphen-glued"
            );
        }
        assert!(
            statement.contains("hyphen"),
            "the statement must name the glue condition on the drop; got: {statement}"
        );
    }

    /// How a char is written **in** the mint statement: the space as the word
    /// `space`, every other char in backticks. Used by
    /// [`mint_statement_states_the_whole_mint_rule`] to ask the sentence, for an
    /// arbitrary char, "do you name this one?".
    fn statement_rendering(ch: char) -> String {
        if ch == ' ' {
            "space".to_owned()
        } else {
            format!("`{ch}`")
        }
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
        // Every edge stopword at each of the three positions, and hyphen-glued at
        // each edge — the generation-3 axis.
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
            assert!(
                has(&format!("{word}-alpha beta")),
                "{word}: glued-leading missing"
            );
            assert!(
                has(&format!("alpha beta-{word}")),
                "{word}: glued-trailing missing"
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
    /// — the property the pack-load gate rests on. Proven against the *separator
    /// map* of generation 1 (`/` and `.` **stripped**, the shape this rule shipped
    /// as until the M42 fork): re-running the generated vector with those two chars
    /// pre-stripped from every input — so the current map has nothing to map — yields
    /// a different digest, so the fence could not, and did not, sleep through that
    /// fork. The *other* half of the moves-when-it-moves property, across the
    /// generation 2 → 3 edge-stopword fork, is pinned by the distinct entries of
    /// [`GENERATIONS`].
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
