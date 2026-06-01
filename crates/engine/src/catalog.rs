//! The shared leading-`---` front-matter fence recognizer.
//!
//! A definition (workflow or step) opens with an optional `---`-fenced
//! front-matter block; [`front_matter`] slices the YAML inside it. The
//! recognizer is reused across the engine — [`crate::compose`] splits a
//! definition's front-matter from its body over this one function, so the two
//! never drift on what a fence is.
//!
//! (The live workflow **catalog** the router and orientation present is built
//! on the frontend from the resolved cascade — `cli::start::selectable_workflows`
//! enumerates the `creates-task: true` work-workflows; the engine resolves the
//! `catalog` data-value root over that fed-in list. The engine compiles in no
//! pack content of its own.)

/// Slice out the YAML inside the leading `---`-fenced front-matter block.
///
/// Recognizes a `---` fence on the first line and the matching closing `---`
/// line (a line that is exactly `---`); returns the bytes between them. Returns
/// `None` if the text does not open with a fence (no front-matter block). The
/// canonical writer emits LF (`parsing.md` → Round-trip guarantees), so this
/// reads the LF form.
pub(crate) fn front_matter(text: &str) -> Option<&str> {
    let rest = text.strip_prefix("---\n")?;
    // The closing fence is a line that is exactly `---`.
    let end = rest
        .match_indices("---")
        .find(|(i, _)| {
            let at_line_start = *i == 0 || rest[..*i].ends_with('\n');
            let after = &rest[i + 3..];
            let at_line_end = after.is_empty() || after.starts_with('\n');
            at_line_start && at_line_end
        })
        .map(|(i, _)| i)?;
    Some(&rest[..end])
}
