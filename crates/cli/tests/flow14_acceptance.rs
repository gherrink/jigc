//! End-to-end acceptance for flow 14 — the **override walk** through the binary
//! (M11 Increment 3, T4; `design/worked-examples.md` → flow 14;
//! `design/introspection.md`).
//!
//! The M11 headline proved on the emitted bytes of the real `jigc describe`
//! binary: the projection **reflects the resolved cascade** — a project-layer
//! **whole-file shadow** of a workflow definition (`single-task.yaml`) carrying an
//! edited `usage:` **visibly changes** the output (the project prose wins, the pack
//! prose is gone), while an **unshadowed** definition's pack prose is **unchanged**.
//! "describe mentions single-task" is rejected as masking — the load-bearing
//! assertion is that the override *changed the output*, asserted as a diff against
//! the pre-override baseline (bar #1).
//!
//! The acceptance walk:
//!
//!   1. Baseline: `jigc describe` over the **real shipped pack definitions** (no
//!      shadow) — capture the projection (bar #6: real defs, not a renderer-shaped
//!      fixture pack).
//!   2. Author a project whole-file shadow of `single-task.yaml` with an edited
//!      `usage:` (a WORKFLOW — never a command-ref; `hint` is pack-only and not
//!      asserted cascade-reflecting). `project > pack-default` only — the team layer
//!      is unfed.
//!   3. Override walk: the overridden output **contains** the project `usage` prose
//!      and **not** the pack `usage` prose for single-task; the unshadowed `adr`
//!      doctype's pack prose is **byte-identical** to the baseline (the shadow is
//!      inert for ids the project does not own).
//!   4. The inc-2 **non-contractual format predicate** holds on the REAL overridden
//!      output (ported verbatim from `tests/describe.rs` — each integration test is
//!      its own crate, so the contract is re-stated in full, not imported).
//!   5. The authored project string appears **verbatim** (assembled, never
//!      generated — bar #3).
//!
//! No external test crates: the binary path comes from `CARGO_BIN_EXE_jigc`, temp
//! repos are built with `std::fs`, and a self-cleaning `TempDir` keeps the test off
//! the developer's real repo / `~/.config`.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-flow14-{tag}-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        );
        path.push(unique);
        fs::create_dir_all(&path).expect("create temp dir");
        TempDir(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// Mark `root` as a git repo + project layer — the setup gate `jigc describe`
/// requires. Returns the `.jigc/config/` project layer dir the shadow is authored
/// under. describe reads the embedded pack, cascade-resolved against this layer, so
/// a bare `.git` + `.jigc/config/` is the whole gate.
fn set_up_repo(root: &Path) -> PathBuf {
    fs::create_dir_all(root.join(".git")).expect("create .git marker");
    let config = root.join(".jigc").join("config");
    fs::create_dir_all(&config).expect("create project layer");
    config
}

/// Run the built `jigc describe` binary with `cwd = repo` and `$HOME = home`,
/// returning its captured stdout. Asserts a clean (exit 0) run. No `--format` arg:
/// the default agent-text surface is the non-contractual prose the predicate holds.
fn describe_stdout(repo: &Path, home: &Path) -> String {
    let out = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .arg("describe")
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary");
    assert!(
        out.status.success(),
        "`jigc describe` must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout).expect("utf-8 stdout")
}

/// The pack `usage:` clause for single-task — the string that must be GONE once the
/// project shadow wins (from `crates/cli/pack/workflows/single-task.yaml`).
const PACK_SINGLE_TASK_USAGE: &str = "the work is one coherent change you can hold in your head";

/// The project shadow's edited `usage:` — a **bare clause** (the weave supplies the
/// "Reach for it when " lead) that begins no line with a colon-term (so the
/// overridden output still clears the format predicate). This is the verbatim string
/// bar #3 asserts and bar #1 asserts wins over the pack prose.
const PROJECT_SINGLE_TASK_USAGE: &str = "Our house rule makes single-task the default for any change under ~200 lines; anything larger goes through plan first.";

/// The unshadowed `adr` doctype's pack `description:` clause — a definition the
/// project does NOT own, whose prose must be unchanged by the single-task shadow.
const PACK_ADR_DESCRIPTION: &str = "A dated architectural decision record";

/// Author a project whole-file shadow of `single-task.yaml` carrying the edited
/// `usage:` — a full copy (whole-file REPLACE, no field-merge) of the workflow
/// front-matter. The body include is irrelevant to describe (it reads front-matter
/// only, composes nothing), so a placeholder step keeps the file a valid workflow
/// definition without coupling to the pack body.
fn author_single_task_shadow(project_config: &Path) {
    let workflows = project_config.join("workflows");
    fs::create_dir_all(&workflows).expect("mk project workflows shadow dir");
    fs::write(
        workflows.join("single-task.yaml"),
        format!(
            "---\nwhen: a scoped change\ndescription: An end-to-end scoped change.\nusage: {PROJECT_SINGLE_TASK_USAGE}\ncreates-task: true\n---\n{{{{ include: step:noop }}}}\n"
        ),
    )
    .expect("write the project workflow shadow");
}

/// Extract the sentence narrating a definition `id` — the run from "`<id> is`" up to
/// (and including) the terminating period of its `usage` clause. The narration shape
/// is "`<id> is <description> Reach for it when <usage>.`" (one clause per
/// definition, woven into a paragraph), so this isolates exactly the bytes the shadow
/// is allowed to change for that id — the basis for the byte-identical diff on an
/// unshadowed definition.
fn narration_for<'a>(out: &'a str, id: &str) -> &'a str {
    let needle = format!("{id} is ");
    // Match the needle only at a definition boundary, not as the tail of a longer id
    // (the `adr` doctype must not match inside `migrate-adr is …`): the byte before the
    // match must not continue an id (`[\w-]`).
    let start = out
        .match_indices(&needle)
        .find(|(idx, _)| {
            *idx == 0
                || !out[..*idx]
                    .chars()
                    .next_back()
                    .is_some_and(|c| c.is_alphanumeric() || c == '-' || c == '_')
        })
        .map(|(idx, _)| idx)
        .unwrap_or_else(|| panic!("`{id}` must be narrated; got:\n{out}"));
    let after = &out[start..];
    // The narration ends at the first ". " that closes the usage clause (the woven
    // clauses are separated by a single space after the period).
    let end = after
        .find(". ")
        .map(|rel| start + rel + 1)
        .unwrap_or(out.len());
    &out[start..end]
}

#[test]
fn override_walk_project_usage_wins_pack_usage_gone_unshadowed_unchanged() {
    // (1) Baseline: describe over the real shipped pack definitions — no shadow.
    let baseline_repo = TempDir::new("baseline");
    let baseline_home = TempDir::new("baseline-home");
    set_up_repo(baseline_repo.path());
    let baseline = describe_stdout(baseline_repo.path(), baseline_home.path());

    // Sanity: the baseline carries the PACK usage for single-task (the string the
    // override must displace) — proving the displacement is a real change, not a
    // string that was never there.
    assert!(
        baseline.contains(PACK_SINGLE_TASK_USAGE),
        "the baseline (no shadow) must carry the pack single-task usage; got:\n{baseline}",
    );
    assert!(
        !baseline.contains(PROJECT_SINGLE_TASK_USAGE),
        "the baseline (no shadow) must NOT carry the project usage; got:\n{baseline}",
    );

    // (2) Author the project whole-file shadow of single-task.yaml.
    let repo = TempDir::new("override");
    let home = TempDir::new("override-home");
    let project_config = set_up_repo(repo.path());
    author_single_task_shadow(&project_config);

    let overridden = describe_stdout(repo.path(), home.path());

    // (3) The override CHANGED the output — the headline, asserted as a diff:
    //   - the project usage prose is IN,
    //   - the pack usage prose is OUT (whole-file replace, no field-merge),
    //   - an UNSHADOWED definition's narration is byte-identical to the baseline.
    assert!(
        overridden.contains(PROJECT_SINGLE_TASK_USAGE),
        "the project shadow's usage must win in the projection; got:\n{overridden}",
    );
    assert!(
        !overridden.contains(PACK_SINGLE_TASK_USAGE),
        "the pack single-task usage must NOT survive the project shadow (whole-file replace); got:\n{overridden}",
    );

    // The diff against the pre-override baseline, scoped to the shadowed definition:
    // single-task's narration MUST differ; the unshadowed adr's narration MUST be
    // byte-identical. A "describe mentions single-task" assertion would be masking —
    // this asserts the override CHANGED that narration and left others untouched.
    let baseline_single = narration_for(&baseline, "single-task");
    let overridden_single = narration_for(&overridden, "single-task");
    assert_ne!(
        baseline_single, overridden_single,
        "the shadowed single-task narration must DIFFER from the baseline (the override changed the output)",
    );

    let baseline_adr = narration_for(&baseline, "adr");
    let overridden_adr = narration_for(&overridden, "adr");
    assert_eq!(
        baseline_adr, overridden_adr,
        "the unshadowed adr narration must be BYTE-IDENTICAL to the baseline (the shadow is inert for ids the project does not own)",
    );
    // And it carries its pack prose, so the byte-equality above is over real prose,
    // not two empty narrations.
    assert!(
        overridden_adr.contains(PACK_ADR_DESCRIPTION),
        "the unshadowed adr must keep its pack description prose; got: {overridden_adr:?}",
    );
}

#[test]
fn non_contractual_predicate_holds_on_the_real_overridden_output() {
    // (4) The inc-2 format predicate holds on the REAL overridden bytes — the
    // cascade-reflecting override does not break the non-contractual contract.
    let repo = TempDir::new("predicate");
    let home = TempDir::new("predicate-home");
    let project_config = set_up_repo(repo.path());
    author_single_task_shadow(&project_config);

    let overridden = describe_stdout(repo.path(), home.path());

    assert_non_contractual_prose(&overridden).unwrap_or_else(|why| {
        panic!(
            "the overridden describe output must stay hostile-to-parsing: {why}\n--- output ---\n{overridden}"
        )
    });
    // The routing footer still rides the prose surface after the override.
    assert!(
        overridden.trim_end().ends_with(ROUTING_FOOTER),
        "the overridden prose surface carries the routing footer; got:\n{overridden}",
    );
}

#[test]
fn authored_project_string_appears_verbatim() {
    // (5) The authored project usage appears verbatim — assembled, never generated
    // (bar #3). This is the same string the override walk asserts wins, pinned here
    // as an exact substring of the emitted bytes.
    let repo = TempDir::new("verbatim");
    let home = TempDir::new("verbatim-home");
    let project_config = set_up_repo(repo.path());
    author_single_task_shadow(&project_config);

    let overridden = describe_stdout(repo.path(), home.path());

    assert!(
        overridden.contains(PROJECT_SINGLE_TASK_USAGE),
        "the authored project string must appear verbatim in the projection; got:\n{overridden}",
    );
}

// ---------------------------------------------------------------------------------
// The inc-2 non-contractual format predicate, ported verbatim from
// `tests/describe.rs` (`assert_non_contractual_prose` + its helpers). Each
// integration test is its own crate, so the predicate — the load-bearing contract —
// is re-stated here rather than imported. See `design/introspection.md` → The
// operational format contract; `design/worked-examples.md` → flow 14, bar #2.
// ---------------------------------------------------------------------------------

const ROUTING_FOOTER: &str =
    "— jigc · run `jigc start` for orientation; all writes through `jigc`.";

/// The four pinned properties of a non-contractual describe projection: (a) not
/// valid JSON, not a top-level YAML mapping/sequence; (b) no key-shaped line, no
/// bullet row; (c) no per-definition extractable key/delimiter; (d) a prose-density
/// floor. Returns `Ok(())` when `out` is provably hostile to parsing.
fn assert_non_contractual_prose(out: &str) -> Result<(), String> {
    let body = out.trim_end().trim_end_matches(ROUTING_FOOTER);

    // (a) not valid JSON.
    if serde_json::from_str::<serde_json::Value>(body).is_ok() {
        return Err("output parses as valid JSON — a consumer could deserialize it".into());
    }
    // (a) does not parse as a top-level YAML mapping or sequence (a bare scalar is
    // fine — prose *is* a YAML scalar).
    if let Ok(value) = serde_yaml_ng::from_str::<serde_yaml_ng::Value>(body) {
        if value.is_mapping() {
            return Err("output parses as a top-level YAML mapping — a keyed contract".into());
        }
        if value.is_sequence() {
            return Err("output parses as a top-level YAML sequence — a list contract".into());
        }
    }

    // (b) + (c), line by line.
    let key_shaped = regex_key_shaped();
    let bullet_row = regex_bullet_row();
    for line in body.lines() {
        if key_shaped(line) {
            return Err(format!(
                "a key-shaped line exposes a field handle: {line:?}"
            ));
        }
        if bullet_row(line) {
            return Err(format!("a bullet row exposes a list item: {line:?}"));
        }
        // (c) no per-definition extractable key/delimiter — a tabular row a consumer
        // could split a field out of (`id<TAB>…`, `id | …`, `id = …`).
        let trimmed = line.trim_start();
        if trimmed.contains('\t') || trimmed.contains(" | ") || trimmed.contains(" = ") {
            return Err(format!(
                "a tabular delimiter exposes an extractable field: {line:?}"
            ));
        }
    }

    // (d) a prose-density floor — the non-blank lines read as paragraphs, not a list.
    let nonblank: Vec<&str> = body.lines().filter(|l| !l.trim().is_empty()).collect();
    if nonblank.is_empty() {
        return Err("output is empty — no prose".into());
    }
    let words: usize = nonblank.iter().map(|l| l.split_whitespace().count()).sum();
    let avg_words = words as f64 / nonblank.len() as f64;
    if avg_words < 12.0 {
        return Err(format!(
            "prose-density floor not met: {avg_words:.1} avg words/line reads as a list, not paragraphs"
        ));
    }
    if body.matches(". ").count() + body.matches(".\n").count() < 2 {
        return Err("prose-density floor not met: too few sentences to read as prose".into());
    }

    Ok(())
}

/// `^\s*[\w-]+:\s` — a line that *begins* with a colon-term (a key-shaped line a
/// consumer could split `key: value` from). Hand-rolled to avoid a `regex`
/// dependency in the test crate.
fn regex_key_shaped() -> impl Fn(&str) -> bool {
    |line: &str| {
        let trimmed = line.trim_start();
        let Some(colon) = trimmed.find(':') else {
            return false;
        };
        let key = &trimmed[..colon];
        !key.is_empty()
            && key
                .chars()
                .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
            && trimmed[colon + 1..]
                .chars()
                .next()
                .is_some_and(char::is_whitespace)
    }
}

/// `^\s*[-*]\s` — a bullet row.
fn regex_bullet_row() -> impl Fn(&str) -> bool {
    |line: &str| {
        let trimmed = line.trim_start();
        let mut chars = trimmed.chars();
        matches!(chars.next(), Some('-') | Some('*'))
            && chars.next().is_some_and(char::is_whitespace)
    }
}
