//! Workflow composition — include expansion + deterministic placeholder
//! resolution into the four-class emitted format.
//!
//! See `design/workflow-dialect.md` and `design/command-catalog.md`.
//!
//! This module lands the composition **inputs**: parsing one workflow definition
//! file into a [`WorkflowDef`] and one step definition file into a [`StepDef`]
//! (`workflow-dialect.md` → On-disk definition format — a step is a file, its id
//! is the filename, its body is the verbatim prompt). A workflow definition is a
//! Markdown body under a `---`-fenced front-matter block (`workflow-dialect.md`
//! → On-disk definition format), where the front-matter is **config-family YAML**
//! (`when`, `creates-task` default `true`, `allows-create`) and the body is
//! **include-only at top level**: each non-blank, non-comment line must be a
//! `{{ include: step:<id> }}` placeholder. The ordered include ids are the
//! composition order (principle #2: the file *is* the ordered list; ids carry no
//! position). This is phase-1 input gathering for one layer's workflow file
//! (`overrides.md` → Resolution algorithm).

use crate::finding::{Finding, Location};
use serde::{Deserialize, Serialize};

/// One `allows-create` entry: a doctype the agent may `jigc doc create` during
/// the task, bound to a context role (`{type: adr, as: decision}` →
/// `AllowsCreate { doc_type: "adr", as_role: "decision" }`). See
/// `workflow-dialect.md` → On-disk definition format and `write-commands.md` →
/// The create-gate.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AllowsCreate {
    /// The doctype id the create-gate admits (e.g. `adr`).
    #[serde(rename = "type")]
    pub doc_type: String,
    /// The context role the created instance binds to (e.g. `decision`).
    #[serde(rename = "as")]
    pub as_role: String,
}

/// A parsed workflow definition: its metadata front-matter plus the ordered
/// include id list lifted from the include-only body.
///
/// `includes` are step ids in physical body order — the composition order. The
/// `when` selection hint is carried for completeness but the catalog
/// ([`crate::catalog`]) is the surface that consumes it; composition consumes
/// `creates_task`, `allows_create`, and `includes`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowDef {
    /// The one-line `when` selection hint (router/catalog guidance).
    pub when: Option<String>,
    /// Whether running this workflow mints a task. Default `true` when the
    /// front-matter omits the key (`workflow-dialect.md` → On-disk format).
    pub creates_task: bool,
    /// The doctypes the agent may create during the task (default empty).
    pub allows_create: Vec<AllowsCreate>,
    /// The ordered step ids from the include-only body — the composition order.
    pub includes: Vec<String>,
}

/// The config-family front-matter of a workflow definition, as YAML.
///
/// `creates-task` defaults to `true` when omitted (`workflow-dialect.md`). The
/// catalog reads only `when` from the same block; this is composition's fuller
/// read.
#[derive(Deserialize)]
struct WorkflowFrontMatter {
    when: Option<String>,
    #[serde(rename = "creates-task", default = "default_creates_task")]
    creates_task: bool,
    #[serde(rename = "allows-create", default)]
    allows_create: Vec<AllowsCreate>,
}

fn default_creates_task() -> bool {
    true
}

/// A parsed step definition: its frozen id (the resource id the pack assigns from
/// the filename stem) and its verbatim prompt body.
///
/// A step is **one file** reusing the document-instance shape — a Markdown body
/// under an optional `---`-fenced front-matter block (`workflow-dialect.md` →
/// On-disk definition format). The **id is the filename**, never a front-matter
/// field; the **body is the prompt** (instruction prose with `{{placeholders}}`,
/// resolved later in composition). A *plain* step needs no front-matter at all —
/// it is just a prompt body. The body is carried **byte-for-byte verbatim** (the
/// post-fence remainder when a front-matter block is present, else the whole
/// file); placeholder resolution runs over it later, never at load time.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StepDef {
    /// The frozen step id — the resource id the pack assigns (the filename stem).
    pub id: String,
    /// The verbatim prompt body (post-front-matter remainder, byte-for-byte).
    pub body: String,
}

/// Parse one step definition's raw bytes into a [`StepDef`] under `id`.
///
/// The `id` is the resource id the caller already holds (the pack's filename stem,
/// per `packsource.rs` → `PackResourceKind::Steps`) — never read from the file.
/// The body is the prompt, carried **verbatim**: if the file opens with a
/// `---`-fenced front-matter block (config-family YAML — a step's optional config,
/// e.g. a `fan-out` marker), the body is the **post-fence remainder, byte-for-byte**;
/// otherwise the body is the **whole file, byte-for-byte** (a plain step needs no
/// front-matter). The only failure is non-UTF-8 bytes — a blocking conformance
/// [`Finding`] (the settled block envelope, `DECISIONS.md` 2026-05-31). The
/// front-matter is *not* parsed here: this task's single concern is the verbatim
/// id+body split; consuming a step's config lands when a step kind needs it.
pub fn load_step_def(id: impl Into<String>, bytes: &[u8]) -> Result<StepDef, Finding> {
    let text = std::str::from_utf8(bytes).map_err(|_| {
        Finding::blocking(
            "workflow-refs.not-utf8",
            "step definition is not valid UTF-8",
            Location::at(1, 1),
        )
    })?;
    let body = strip_optional_front_matter(text);
    Ok(StepDef {
        id: id.into(),
        body: body.to_owned(),
    })
}

/// Return the step's verbatim body: the post-front-matter remainder when `text`
/// opens with a `---`-fenced block, else `text` unchanged.
///
/// Reuses the one fence recognizer ([`crate::catalog::front_matter`]); the body is
/// the bytes after the closing `---` fence line's terminating newline (or after a
/// closing `---` at end-of-file). A file that does not open with a fence has no
/// front-matter, so its whole content is the body — byte-for-byte.
fn strip_optional_front_matter(text: &str) -> &str {
    match split_front_matter(text) {
        Some((_front, body)) => body,
        None => text,
    }
}

/// Parse one workflow definition's raw bytes into a [`WorkflowDef`].
///
/// Slices the `---`-fenced front-matter (config-family YAML) and the body, parses
/// the metadata, and reads the include-only body: each non-blank, non-comment
/// line must be a `{{ include: step:<id> }}` placeholder; blank lines and HTML
/// comment lines (`<!-- … -->`) are dropped. Any other top-level line — prose, an
/// ATX heading, a non-include placeholder — is a blocking conformance
/// [`Finding`] (`workflow-refs.body-include-only`, `workflow-dialect.md` →
/// Workflow body is include-only). The finding is the settled block envelope, not
/// a new error type (`DECISIONS.md` 2026-05-31 → block payload).
pub fn load_workflow_def(bytes: &[u8]) -> Result<WorkflowDef, Finding> {
    let text = std::str::from_utf8(bytes).map_err(|_| {
        Finding::blocking(
            "workflow-refs.not-utf8",
            "workflow definition is not valid UTF-8",
            Location::at(1, 1),
        )
    })?;
    let (front, body) = split_front_matter(text).ok_or_else(|| {
        Finding::blocking(
            "workflow-refs.missing-front-matter",
            "workflow definition has no `---`-fenced front-matter block",
            Location::at(1, 1),
        )
    })?;
    let meta: WorkflowFrontMatter = serde_yaml_ng::from_str(front).map_err(|source| {
        Finding::blocking(
            "workflow-refs.malformed-front-matter",
            format!("workflow front-matter is malformed config-family YAML: {source}"),
            Location::at(1, 1),
        )
    })?;

    // The body begins one line after the front-matter's closing `---` fence; that
    // closing fence sits on the line *after* the front-matter lines. Count the
    // lines consumed up to the body so findings point at real source lines.
    let body_start_line = text[..text.len() - body.len()].lines().count() + 1;
    let includes = parse_include_only_body(body, body_start_line)?;

    Ok(WorkflowDef {
        when: meta.when.filter(|w| !w.trim().is_empty()),
        creates_task: meta.creates_task,
        allows_create: meta.allows_create,
        includes,
    })
}

/// Split `text` into `(front_matter_yaml, body)` at the leading `---` fence.
///
/// Reuses the same fence recognition as [`crate::catalog::front_matter`] (one
/// recognizer), but also returns the body that follows the closing fence — what
/// composition needs that the catalog does not.
fn split_front_matter(text: &str) -> Option<(&str, &str)> {
    let front = crate::catalog::front_matter(text)?;
    // `front` is the slice strictly between the fences; the closing `---` fence
    // starts right after it, within the post-opening-fence remainder.
    let rest = text.strip_prefix("---\n")?;
    let after_close = &rest[front.len()..];
    let body = after_close
        .strip_prefix("---\n")
        .or_else(|| after_close.strip_prefix("---"))
        .unwrap_or(after_close);
    Some((front, body))
}

/// Read the include-only body into an ordered list of step ids.
///
/// Drops blank lines and HTML-comment lines; accepts `{{ include: step:<id> }}`
/// lines (whitespace-tolerant inside the braces); rejects anything else as a
/// blocking `body-include-only` finding located at the offending line.
fn parse_include_only_body(body: &str, body_start_line: usize) -> Result<Vec<String>, Finding> {
    let mut includes = Vec::new();
    for (offset, raw) in body.lines().enumerate() {
        let line_no = body_start_line + offset;
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with("<!--") && line.ends_with("-->") {
            continue;
        }
        match parse_include_line(line) {
            Some(id) => includes.push(id),
            None => {
                return Err(Finding::blocking(
                    "workflow-refs.body-include-only",
                    format!(
                        "workflow body line is not an `{{{{ include: step:<id> }}}}`, blank, or comment: `{line}`"
                    ),
                    Location::at(line_no, 1),
                ));
            }
        }
    }
    Ok(includes)
}

/// Match one `{{ include: step:<id> }}` line and return `<id>`, or `None` if the
/// line is not a well-formed top-level include placeholder.
fn parse_include_line(line: &str) -> Option<String> {
    let inner = line.strip_prefix("{{")?.strip_suffix("}}")?.trim();
    let target = inner.strip_prefix("include:")?.trim();
    let id = target.strip_prefix("step:")?.trim();
    if id.is_empty() || id.contains(char::is_whitespace) {
        return None;
    }
    Some(id.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The shipped single-task definition bytes — the embedded pack file this
    /// composer loads. Kept in sync with `crates/cli/pack/workflows/single-task.yaml`.
    const SINGLE_TASK: &str = "\
---
when: implement one scoped change end-to-end
creates-task: true
allows-create: [{type: adr, as: decision}]
---
{{ include: step:locate }}
{{ include: step:implement }}
{{ include: step:superseded-context }}
{{ include: step:finalize }}
";

    /// Core done-criterion: loading the shipped single-task bytes yields a
    /// `WorkflowDef` with `creates_task=true`, `allows_create=[{adr, decision}]`,
    /// and the four includes in body order. The golden pins the full serde
    /// projection so a field rename or reorder breaks it.
    #[test]
    fn workflow_def_parses_includes_and_front_matter() {
        let def = load_workflow_def(SINGLE_TASK.as_bytes()).expect("loads");

        assert!(def.creates_task);
        assert_eq!(
            def.allows_create,
            vec![AllowsCreate {
                doc_type: "adr".to_owned(),
                as_role: "decision".to_owned(),
            }]
        );
        assert_eq!(
            def.includes,
            vec!["locate", "implement", "superseded-context", "finalize"]
        );

        let json = serde_json::to_string_pretty(&def).expect("serializes");
        insta::assert_snapshot!(json, @r#"
        {
          "when": "implement one scoped change end-to-end",
          "creates_task": true,
          "allows_create": [
            {
              "type": "adr",
              "as": "decision"
            }
          ],
          "includes": [
            "locate",
            "implement",
            "superseded-context",
            "finalize"
          ]
        }
        "#);
    }

    /// `creates-task` defaults to `true` when the front-matter omits the key
    /// (`workflow-dialect.md` → On-disk format).
    #[test]
    fn creates_task_defaults_true_when_omitted() {
        let def =
            load_workflow_def(b"---\nwhen: pick a workflow\n---\n{{ include: step:present }}\n")
                .expect("loads");
        assert!(def.creates_task);
        assert!(def.allows_create.is_empty());
    }

    /// A `creates-task: false` workflow (the router shape) parses with the flag off.
    #[test]
    fn creates_task_false_parses() {
        let def = load_workflow_def(
            b"---\nwhen: help me pick\ncreates-task: false\n---\n{{ include: step:route }}\n",
        )
        .expect("loads");
        assert!(!def.creates_task);
    }

    /// Blank lines and HTML-comment lines are dropped; only include ids survive,
    /// in physical order.
    #[test]
    fn blanks_and_comments_are_dropped() {
        let def = load_workflow_def(
            b"---\nwhen: x\n---\n\n{{ include: step:a }}\n<!-- why b -->\n{{ include: step:b }}\n\n",
        )
        .expect("loads");
        assert_eq!(def.includes, vec!["a", "b"]);
    }

    /// A prose body line is a blocking `body-include-only` finding located at the
    /// offending source line — the `body-include-only` precursor.
    #[test]
    fn prose_body_line_is_a_blocking_finding() {
        let err = load_workflow_def(
            b"---\nwhen: x\n---\n{{ include: step:a }}\nthis is prose, not an include\n",
        )
        .expect_err("prose rejects");
        assert_eq!(err.code, "workflow-refs.body-include-only");
        assert_eq!(err.severity, crate::finding::Severity::Blocking);
        // Front-matter is 3 lines (`---`, `when: x`, `---`); body line 1 is the
        // include, line 2 is the prose → source line 5.
        assert_eq!(err.location.as_ref().expect("located").line, 5);
    }

    /// An ATX heading at body top-level is rejected (it is not an include).
    #[test]
    fn heading_body_line_is_rejected() {
        let err =
            load_workflow_def(b"---\nwhen: x\n---\n## a heading\n").expect_err("heading rejects");
        assert_eq!(err.code, "workflow-refs.body-include-only");
    }

    /// A non-include placeholder (a data-value, not an include) at top level is
    /// rejected — the body is include-*only*.
    #[test]
    fn non_include_placeholder_is_rejected() {
        let err = load_workflow_def(b"---\nwhen: x\n---\n{{ task.intent }}\n")
            .expect_err("data-value rejects");
        assert_eq!(err.code, "workflow-refs.body-include-only");
    }

    /// Missing front-matter is a clear blocking finding.
    #[test]
    fn missing_front_matter_is_a_blocking_finding() {
        let err = load_workflow_def(b"{{ include: step:a }}\n").expect_err("no front-matter");
        assert_eq!(err.code, "workflow-refs.missing-front-matter");
    }

    /// The shipped `locate` step body — kept in sync with
    /// `crates/cli/pack/steps/locate.yaml` (a plain, front-matter-less step).
    const STEP_LOCATE: &str = "\
Reason about the change. The intent is:
{{ @task.intent }}

The relevant code paths are not yet known. Inspect the codebase to confirm
scope before implementing.
";

    /// The shipped `superseded-context` step body — kept in sync with
    /// `crates/cli/pack/steps/superseded-context.yaml` (also front-matter-less).
    const STEP_SUPERSEDED: &str = "\
If your decision supersedes an earlier one, here is that decision for
reference — make your consequences explain what changes:
{{ @task.decision.supersedes#decision }}
";

    /// Core done-criterion: a front-matter-less step loads with its full body as
    /// prose, verbatim; a step preceded by a `---`-fenced front-matter block splits
    /// cleanly, with the body preserved byte-for-byte (the post-fence remainder).
    /// Goldens over the two shipped plain step bodies pin the canonical bytes.
    #[test]
    fn step_def_loads_body_verbatim_with_optional_front_matter() {
        // A plain step (no leading `---` fence): id is the resource id, body is the
        // whole input verbatim.
        let locate = load_step_def("locate", STEP_LOCATE.as_bytes()).expect("loads");
        assert_eq!(locate.id, "locate");
        assert_eq!(locate.body, STEP_LOCATE);
        insta::assert_snapshot!(locate.body, @r###"
        Reason about the change. The intent is:
        {{ @task.intent }}

        The relevant code paths are not yet known. Inspect the codebase to confirm
        scope before implementing.
        "###);

        let superseded =
            load_step_def("superseded-context", STEP_SUPERSEDED.as_bytes()).expect("loads");
        assert_eq!(superseded.id, "superseded-context");
        assert_eq!(superseded.body, STEP_SUPERSEDED);
        insta::assert_snapshot!(superseded.body, @r###"
        If your decision supersedes an earlier one, here is that decision for
        reference — make your consequences explain what changes:
        {{ @task.decision.supersedes#decision }}
        "###);

        // A step *with* front-matter: the body is exactly the post-fence remainder,
        // byte-for-byte — the front-matter (config-family YAML) is stripped, the
        // prose preserved verbatim including its internal blank lines.
        let fenced = load_step_def(
            "fan-out-step",
            b"---\nfan-out:\n  over: \"{{ milestone.tasks }}\"\n---\nSpawn a sub-task per item.\n\nEach runs the sub-workflow.\n",
        )
        .expect("loads");
        assert_eq!(fenced.id, "fan-out-step");
        assert_eq!(
            fenced.body,
            "Spawn a sub-task per item.\n\nEach runs the sub-workflow.\n"
        );
    }

    proptest::proptest! {
        /// Body bytes survive load unchanged: for arbitrary UTF-8 prose with no
        /// leading `---\n` fence, `StepDef.body` equals the input verbatim.
        #[test]
        fn front_matter_less_body_survives_load_verbatim(
            prose in "[^\\x00]{0,200}"
        ) {
            // Exclude inputs that happen to open with a front-matter fence — those
            // are the *with-front-matter* case, exercised separately below.
            proptest::prop_assume!(!prose.starts_with("---\n"));
            let def = load_step_def("s", prose.as_bytes()).expect("loads");
            proptest::prop_assert_eq!(def.body, prose);
        }

        /// Body bytes survive load unchanged for the *with-front-matter* case: for
        /// arbitrary prose preceded by a fenced front-matter block, `StepDef.body`
        /// equals the post-fence remainder verbatim.
        #[test]
        fn fenced_body_is_post_fence_remainder_verbatim(
            body in "[^\\x00]{0,200}"
        ) {
            // A body that itself opens with a `---` line would be ambiguous with
            // the front-matter's closing fence, so skip those; they cannot occur as
            // the first body content after a real closing fence anyway.
            proptest::prop_assume!(!body.starts_with("---"));
            let input = format!("---\nkey: value\n---\n{body}");
            let def = load_step_def("s", input.as_bytes()).expect("loads");
            proptest::prop_assert_eq!(def.body, body);
        }
    }

    proptest::proptest! {
        /// Round-trip property: an include-only body of N shuffled `{{ include:
        /// step:<slug> }}` lines, interleaved with blanks and HTML comments, parses
        /// to exactly the input id list in order, with blanks/comments dropped.
        #[test]
        fn include_only_bodies_round_trip_to_ordered_ids(
            ids in proptest::collection::vec("[a-z][a-z0-9-]{0,7}", 0..8)
        ) {
            // Build a body: each id on its own include line, with a blank line and
            // a comment between consecutive ids (both must be dropped).
            let mut body = String::from("---\nwhen: x\n---\n");
            for (i, id) in ids.iter().enumerate() {
                if i > 0 {
                    body.push('\n');
                    body.push_str("<!-- note -->\n");
                }
                body.push_str(&format!("{{{{ include: step:{id} }}}}\n"));
            }
            let def = load_workflow_def(body.as_bytes()).expect("loads");
            proptest::prop_assert_eq!(def.includes, ids);
        }
    }
}
