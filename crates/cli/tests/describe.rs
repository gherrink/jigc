//! End-to-end format-property test for `jigc describe` (M11 Increment 2, T3).
//!
//! The HEADLINE proof of the increment: the describe projection is **provably
//! hostile to parsing** (`design/introspection.md` → The operational format
//! contract; `design/worked-examples.md` → flow 14, acceptance bar #2). "Non-
//! contractual" is only real if it is checkable, so this drives the **emitted
//! bytes** of the real `jigc describe` binary and holds them to a positive format
//! predicate — asserted **both directions**: it PASSES on the real prose output and
//! FAILS on a synthetic structured catalog (a keyed/bulleted list of the same defs)
//! and on a JSON blob. A predicate that only ran one direction would be intent
//! dressed as enforcement, not enforcement.
//!
//! The four pinned properties of the predicate ([`assert_non_contractual_prose`]):
//!   (a) not valid JSON, and does not parse as a top-level YAML mapping/sequence
//!       (a bare-scalar parse is fine — prose *is* a YAML scalar);
//!   (b) no key-shaped line (`^\s*[\w-]+:\s`) and no bullet row (`^\s*[-*]\s`);
//!   (c) no per-definition extractable key/delimiter (a tabular `id<TAB>…` /
//!       `id | …` / `id = …` row a consumer could pull a field from) — *light
//!       unkeyed prose section grouping is permitted*;
//!   (d) a prose-density floor (reads as paragraphs, not a list).
//!
//! A byte-snapshot is kept *additionally* but is **necessary-but-insufficient** — a
//! snapshot of a bulleted list passes a byte-snapshot happily while *failing* the
//! format predicate; the predicate is the load-bearing assertion.
//!
//! No external test crates: the binary path comes from `CARGO_BIN_EXE_jigc`, temp
//! repos are built with `std::fs`, and a self-cleaning `TempDir` keeps the test off
//! the developer's real repo / `~/.config`. `serde_json` / `serde_yaml_ng` are the
//! same parsers a would-be consumer would reach for — the predicate proves *they*
//! cannot deserialize the output into keyed fields.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-describe-{tag}-{}-{:?}",
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
/// requires (the same gate `jigc ingest` uses). describe reads the embedded pack
/// **pack-only** in M11, so a bare `.git` + `.jigc/config/` is all it needs.
fn set_up_repo(root: &Path) {
    fs::create_dir_all(root.join(".git")).expect("create .git marker");
    fs::create_dir_all(root.join(".jigc").join("config")).expect("create project layer");
}

/// Run the built `jigc describe` binary with `cwd = repo` and `$HOME = home`,
/// returning its captured stdout. Asserts a clean (exit 0) run.
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

/// Run the built `jigc describe --format json` binary, returning the parsed
/// projection — the *definition inventory* the prose surface must narrate. Used to
/// derive the expected definition count from the real pack rather than hard-coding
/// it (the pack grows every wave).
fn describe_json(repo: &Path, home: &Path) -> serde_json::Value {
    let out = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(["describe", "--format", "json"])
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary");
    assert!(
        out.status.success(),
        "`jigc describe --format json` must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    serde_json::from_slice(&out.stdout).expect("describe --format json emits valid JSON")
}

const ROUTING_FOOTER: &str =
    "— jigc · run `jigc start` for orientation; all writes through `jigc`.";

/// The reusable **operational format predicate** — the four pinned properties of a
/// non-contractual describe projection (`design/introspection.md` → The operational
/// format contract). Returns `Ok(())` when `out` is provably hostile to parsing,
/// `Err(reason)` naming the first property it violates. The footer is stripped first
/// — it is framing, not a definition, and its em-dash lead is prose either way.
///
/// This is the single predicate the both-directions assertions reuse: it PASSES on
/// the real binary output and FAILS on a structured catalog / JSON.
fn assert_non_contractual_prose(out: &str) -> Result<(), String> {
    let body = out.trim_end().trim_end_matches(ROUTING_FOOTER);

    // (a) not valid JSON.
    if serde_json::from_str::<serde_json::Value>(body).is_ok() {
        return Err("output parses as valid JSON — a consumer could deserialize it".into());
    }
    // (a) does not parse as a top-level YAML mapping or sequence (a bare scalar is
    // fine — prose *is* a YAML scalar; what's forbidden is structure a consumer can
    // deserialize into keyed fields / items). A YAML parse *error* also satisfies
    // "does not parse as a mapping/sequence".
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
        // could split a field out of (`id<TAB>…`, `id | …`, `id = …`). Light unkeyed
        // prose grouping (a sentence-shaped transition) carries none of these.
        let trimmed = line.trim_start();
        if trimmed.contains('\t') || trimmed.contains(" | ") || trimmed.contains(" = ") {
            return Err(format!(
                "a tabular delimiter exposes an extractable field: {line:?}"
            ));
        }
    }

    // (d) a prose-density floor — the non-blank lines read as paragraphs, not a
    // list. A structured catalog's lines are short (one short entry each); prose
    // runs long and sentence-dense. Require an average of clearly-more-than-a-list
    // words per non-blank line and at least a couple of sentence-terminating periods.
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
/// consumer could split `key: value` from). Authored prose may carry a mid-line
/// colon ("e.g.", "Reach for it:") — only a line *starting* with the colon-term is
/// forbidden. Hand-rolled to avoid a `regex` dependency in the test crate.
fn regex_key_shaped() -> impl Fn(&str) -> bool {
    |line: &str| {
        let trimmed = line.trim_start();
        let Some(colon) = trimmed.find(':') else {
            return false;
        };
        let key = &trimmed[..colon];
        // A non-empty run of `[\w-]` before the colon, then whitespace after it.
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

#[test]
fn describe_real_output_is_non_contractual_prose() {
    let repo = TempDir::new("prose");
    set_up_repo(repo.path());
    let home = TempDir::new("home");

    let out = describe_stdout(repo.path(), home.path());

    // The HEADLINE: the predicate PASSES on the real emitted bytes.
    assert_non_contractual_prose(&out).unwrap_or_else(|why| {
        panic!("real describe output must be hostile-to-parsing: {why}\n--- output ---\n{out}")
    });

    // The footer still rides the prose surface (agent-text), even though the body is
    // held to the format predicate above (the footer is stripped before the check).
    assert!(
        out.trim_end().ends_with(ROUTING_FOOTER),
        "the prose surface carries the routing footer; got:\n{out}",
    );
}

#[test]
fn describe_carries_the_authored_strings() {
    let repo = TempDir::new("authored");
    set_up_repo(repo.path());
    let home = TempDir::new("home");

    let out = describe_stdout(repo.path(), home.path());

    // The prose comes verbatim (assembled, never generated) from the authored
    // `description:` fields on the real shipped pack definitions (worked-examples.md
    // → flow 14, bar #3). Assert authored substrings from a workflow + a doctype.
    assert!(
        out.contains("end-to-end scoped change"),
        "the single-task workflow's authored description must survive into the projection; got:\n{out}",
    );
    assert!(
        out.contains("dated architectural decision record"),
        "the adr doctype's authored description must survive into the projection; got:\n{out}",
    );
    // A command-ref `hint` is projected verbatim (hint's first projection consumer).
    assert!(
        out.contains("One task → one commit."),
        "the finalize command-ref hint must be projected into the prose; got:\n{out}",
    );
}

/// T12 (M45 Inc 10) — the real `jigc describe` binary routes to the preview surface:
/// it names `jigc workflow <id> --preview` as the way to read a workflow's step text
/// (findings §69 — the pull-tier `--preview` shipped with nothing routing to it).
#[test]
fn describe_routes_to_workflow_preview() {
    let repo = TempDir::new("preview-route");
    set_up_repo(repo.path());
    let home = TempDir::new("home");

    let out = describe_stdout(repo.path(), home.path());

    assert!(
        out.contains("`jigc workflow <id> --preview`"),
        "the real describe binary must route to the preview surface; got:\n{out}",
    );
}

#[test]
fn describe_carries_the_arch_doc_doctype_and_workflow() {
    // M13 Increment 4, T3 — the M11 describe surface for the new doctype + workflow.
    // The arch-doc doc-type and the architecture-documentation workflow each carry
    // authored `description:`/`usage:` prose (T1/T2); this asserts that prose is woven
    // into the rendered menu, facts-not-advice, the M11 weave shape
    // ("X is <description>. Reach for it when <usage>."). Driven over the EMITTED bytes
    // of the real binary so a pack file that drops or garbles the prose fails here.
    let repo = TempDir::new("archdoc");
    set_up_repo(repo.path());
    let home = TempDir::new("home");

    let out = describe_stdout(repo.path(), home.path());

    // The new workflow's authored description + usage clauses, woven facts-not-advice.
    assert!(
        out.contains(
            "architecture-documentation is Authors living architecture documentation for one part of the system and commits it."
        ),
        "the architecture-documentation workflow's authored description must survive into the projection; got:\n{out}",
    );
    assert!(
        out.contains(
            "Reach for it when a part of the system needs a durable, code-checked description so a later reader can orient without reverse-engineering it."
        ),
        "the architecture-documentation workflow's authored usage must survive into the projection; got:\n{out}",
    );

    // The new doc-type's authored description + usage clauses, woven facts-not-advice.
    assert!(
        out.contains("arch-doc is Living architecture documentation for one part of the system"),
        "the arch-doc doctype's authored description must survive into the projection; got:\n{out}",
    );
    assert!(
        out.contains(
            "Reach for it when a part of the system is worth a durable description that stays honest against the code, so a later reader (or agent) can orient without reverse-engineering it."
        ),
        "the arch-doc doctype's authored usage must survive into the projection; got:\n{out}",
    );

    // The new prose must not break the non-contractual format contract — the projection
    // stays hostile-to-parsing even with the M13 defs woven in.
    assert_non_contractual_prose(&out).unwrap_or_else(|why| {
        panic!("the projection with the M13 arch-doc defs must stay non-contractual prose: {why}\n--- output ---\n{out}")
    });
}

#[test]
fn describe_narrates_no_empty_or_dangling_clause() {
    // skip-on-absent at the emitted-bytes level: the projection never narrates a
    // definition that carries no authored field. The real pack authors all defs, so
    // the observable proof is that no empty narration leaks — no "is ." identity
    // clause with an empty body, no dangling "Reach for it when ." (the artifact a
    // both-absent or blank-field definition would produce if it were *not* skipped).
    let repo = TempDir::new("skip");
    set_up_repo(repo.path());
    let home = TempDir::new("home");

    let out = describe_stdout(repo.path(), home.path());

    assert!(
        !out.contains("is .") && !out.contains("is  "),
        "a narrated definition must carry a non-empty identity clause (skip-on-absent); got:\n{out}",
    );
    assert!(
        !out.contains("Reach for it when ."),
        "a narrated definition must carry a non-empty usage clause (skip-on-absent); got:\n{out}",
    );
}

#[test]
fn describe_never_doubles_the_reach_for_it_lead() {
    // The weave supplies a fixed "Reach for it when " lead (introspect.rs); the
    // authored `usage:` fields must therefore be **bare clauses**, not carry their
    // own "Reach for it when/to/at/as …" lead — else every narration doubles the
    // lead ("Reach for it when Reach for it when …"), the malformed shape this guards
    // against (`design/introspection.md` → the weave example carries usage WITHOUT
    // the prefix). The assertion is over the **emitted bytes** of the real binary,
    // so a pack file that re-introduces the redundant lead fails here.
    let repo = TempDir::new("nodouble");
    set_up_repo(repo.path());
    let home = TempDir::new("home");

    let out = describe_stdout(repo.path(), home.path());

    assert!(
        !out.contains("Reach for it when Reach for it"),
        "a doubled 'Reach for it when Reach for it' lead leaked — the authored usage carries a redundant lead the weave then duplicates; got:\n{out}",
    );
}

#[test]
fn describe_breaks_a_paragraph_per_definition() {
    // M42 Increment 12, T4 — describe stops printing a wall. The prose surface used to
    // join every definition of a group with a single space, so ~15 workflow narrations
    // landed as ONE unreadable paragraph. Each narrated definition now gets its own
    // blank-line-separated paragraph (`design/introspection.md` → Non-contractual by
    // design: a blank line is *not* an extractable handle — no key, no bullet, no
    // delimiter — so the hostile-to-parsing posture is preserved while the menu becomes
    // readable).
    //
    // Driven over the EMITTED bytes of the real binary, with the expected definition
    // count taken from the binary's own `--format json` inventory (the pack grows every
    // wave — a hard-coded count would rot).
    let repo = TempDir::new("paragraphs");
    set_up_repo(repo.path());
    let home = TempDir::new("home");

    let json = describe_json(repo.path(), home.path());
    let definitions = json["definitions"]
        .as_array()
        .expect("the projection carries a definitions array");
    assert!(
        definitions.len() > 2,
        "the shipped pack must narrate several definitions for this test to mean anything; got {}",
        definitions.len(),
    );

    let out = describe_stdout(repo.path(), home.path());
    let body = out.trim_end().trim_end_matches(ROUTING_FOOTER);
    let paragraphs: Vec<&str> = body
        .split("\n\n")
        .map(str::trim)
        .filter(|p| !p.is_empty())
        .collect();

    // (1) The paragraph floor: at least one paragraph per definition (plus the lead-in
    // and the command paragraph). A wall fails here — the old output had 3 paragraphs
    // for 20+ definitions.
    assert!(
        paragraphs.len() >= definitions.len(),
        "the prose must break a paragraph per definition: {} paragraphs for {} definitions\n--- output ---\n{out}",
        paragraphs.len(),
        definitions.len(),
    );

    // (2) The load-bearing half: no paragraph welds two narrations together. Each
    // definition's woven prose sits in exactly one paragraph, and that paragraph
    // carries no other definition's prose.
    for definition in definitions {
        let id = definition["id"].as_str().expect("a definition id");
        let prose = definition["prose"].as_str().expect("a definition prose");
        let carriers: Vec<&&str> = paragraphs.iter().filter(|p| p.contains(prose)).collect();
        assert_eq!(
            carriers.len(),
            1,
            "`{id}`'s narration must appear in exactly one paragraph; found {} carrying it\n--- output ---\n{out}",
            carriers.len(),
        );
        let carrier = carriers[0];
        for other in definitions {
            let other_id = other["id"].as_str().expect("a definition id");
            if other_id == id {
                continue;
            }
            let other_prose = other["prose"].as_str().expect("a definition prose");
            assert!(
                !carrier.contains(other_prose),
                "`{id}` and `{other_id}` are welded into one wall paragraph:\n{carrier}",
            );
        }
    }

    // (3) The posture holds: paragraph breaks introduce no key-shaped line, no bullet
    // row, no extractable per-definition delimiter.
    assert_non_contractual_prose(&out).unwrap_or_else(|why| {
        panic!(
            "the paragraphed projection must stay hostile-to-parsing: {why}\n--- output ---\n{out}"
        )
    });
}

/// The `suppressed: {reason, …}` declarations of every hidden (`selectable: false`)
/// workflow in the two shipped embedded packs, read from the pack sources on disk
/// (the same files `include_dir!` embeds), keyed `(workflow id, reason)`. Derived,
/// not hard-coded — the hidden set grows/shrinks with the packs.
fn shipped_hidden_workflows() -> Vec<(String, String)> {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    let pack_dirs = [
        manifest.join("pack").join("workflows"),
        manifest
            .join("..")
            .join("..")
            .join("packs")
            .join("methodology")
            .join("workflows"),
    ];
    let mut hidden = Vec::new();
    for dir in pack_dirs {
        for entry in fs::read_dir(&dir).expect("read pack workflows dir") {
            let path = entry.expect("dir entry").path();
            if path.extension().and_then(|e| e.to_str()) != Some("yaml") {
                continue;
            }
            let text = fs::read_to_string(&path).expect("read workflow yaml");
            let front_matter = text
                .strip_prefix("---\n")
                .and_then(|rest| rest.split_once("\n---\n"))
                .map(|(fm, _)| fm)
                .expect("workflow file carries front-matter");
            let value: serde_yaml_ng::Value =
                serde_yaml_ng::from_str(front_matter).expect("front-matter parses as YAML");
            if value["selectable"].as_bool() != Some(false) {
                continue;
            }
            let reason = value["suppressed"]["reason"]
                .as_str()
                .expect("a hidden shipped workflow carries suppressed.reason (the pack-load fence)")
                .to_owned();
            let id = path
                .file_stem()
                .and_then(|s| s.to_str())
                .expect("workflow id from file stem")
                .to_owned();
            hidden.push((id, reason));
        }
    }
    hidden
}

#[test]
fn describe_projects_the_suppression_reason_for_hidden_workflows() {
    // M43 Increment 4, T6 — `jigc describe` prints the suppressed reason for a
    // hidden workflow, so describe (the unfiltered menu) and the orient catalog
    // (which filters `selectable: false` out) stop contradicting each other
    // (`design/surface-contract.md` → The suppression fence, last sentence).
    // Driven over the EMITTED bytes of the real binary against the real shipped
    // hidden set, derived from the pack sources — every hidden workflow's entry
    // must say it is hidden from the router catalog and carry its declared reason.
    let repo = TempDir::new("suppression");
    set_up_repo(repo.path());
    // Compose the embedded methodology pack too (the in-binary marker), so the
    // narrated menu covers BOTH shipped packs' hidden sets — the derivation below
    // walks both pack sources.
    fs::write(
        repo.path().join(".jigc").join("config").join("packs.yaml"),
        "compose-embedded-methodology: true\n",
    )
    .expect("write the compose marker");
    let home = TempDir::new("home");

    let hidden = shipped_hidden_workflows();
    assert!(
        hidden.iter().any(|(id, _)| id == "sub-task"),
        "the shipped hidden set must include sub-task (the fan-out unit); got {hidden:?}",
    );

    let json = describe_json(repo.path(), home.path());
    let definitions = json["definitions"]
        .as_array()
        .expect("the projection carries a definitions array");
    let out = describe_stdout(repo.path(), home.path());

    for (id, reason) in &hidden {
        // The entry: the hidden workflow's own narration carries the clause + the
        // verbatim declared reason (the weave strips at most one trailing period).
        let prose = definitions
            .iter()
            .find(|d| d["id"].as_str() == Some(id))
            .unwrap_or_else(|| panic!("hidden workflow `{id}` must be narrated by describe"))["prose"]
            .as_str()
            .expect("a definition prose");
        let reason = reason.trim().trim_end_matches('.');
        assert!(
            prose.contains("hidden from the router catalog"),
            "`{id}`'s entry must say it is hidden from the router catalog; got: {prose:?}",
        );
        assert!(
            prose.contains(reason),
            "`{id}`'s entry must carry its declared suppression reason; got: {prose:?}",
        );
        // The emitted prose surface carries the same narration (the bytes an agent
        // reads), not just the JSON projection.
        assert!(
            out.contains(reason),
            "the emitted prose must carry `{id}`'s suppression reason; got:\n{out}",
        );
    }

    // The new clause must not break the non-contractual posture — the projection
    // stays hostile-to-parsing with the suppression reasons woven in.
    assert_non_contractual_prose(&out).unwrap_or_else(|why| {
        panic!(
            "the projection with suppression reasons must stay non-contractual prose: {why}\n--- output ---\n{out}"
        )
    });
}

#[test]
fn predicate_fails_on_a_structured_catalog_and_on_json() {
    // The OTHER direction — the predicate must REJECT a parseable rendering of the
    // SAME definitions. Without this, "passes on real output" proves nothing: a
    // clean machine-readable list would pass a one-directional check. (worked-
    // examples.md → flow 14, bar #2: "asserted both directions".)

    // (1) Whole-catalog forms of the SAME definitions are all rejected — these are
    // the shapes M11 forbids. A keyed catalog and a bulleted catalog both deserialize
    // as a top-level YAML mapping (a consumer could pull fields straight out); a JSON
    // blob is the most parseable form of all. The headline is that each is *rejected*,
    // not which property catches it (a structured catalog violates several at once).
    let keyed_catalog = "\
single-task: an end-to-end scoped change
adr: a dated architectural decision record
finalize: validate and commit the task
";
    let bulleted_catalog = "\
The menu:
- single-task — an end-to-end scoped change
- adr — a dated architectural decision record
- finalize — validate and commit the task
";
    let json_catalog = serde_json::json!({
        "definitions": [
            {"id": "single-task", "prose": "an end-to-end scoped change"},
            {"id": "adr", "prose": "a dated architectural decision record"},
        ],
    })
    .to_string();
    for (label, catalog) in [
        ("keyed", keyed_catalog),
        ("bulleted", bulleted_catalog),
        ("json", json_catalog.as_str()),
    ] {
        assert!(
            assert_non_contractual_prose(catalog).is_err(),
            "a {label} catalog of the same definitions must FAIL the predicate, not pass it",
        );
    }
    // The JSON form is rejected *by the JSON property specifically* — the most
    // parseable shape trips the first, strongest check.
    let err = assert_non_contractual_prose(&json_catalog).expect_err("json rejected");
    assert!(
        err.contains("JSON"),
        "json must be caught by the JSON check: {err}"
    );

    // (2) Each LINE-LEVEL property is independently live — proven by an input that is
    // otherwise prose (so it clears the JSON + YAML checks) and trips exactly one line
    // check. This guards against a property being dead code that never fires.
    let lead = "Here is a long discursive paragraph of running prose about the menu, what each \
thing is for, and when you would reach for it in real work.\n\n";

    // A key-shaped line (`^\s*[\w-]+:\s`) embedded in prose.
    let err = assert_non_contractual_prose(&format!("{lead}single-task: an end-to-end change\n"))
        .expect_err("a key-shaped line must FAIL the predicate");
    assert!(err.contains("key-shaped"), "wrong rejection reason: {err}");

    // A bullet row (`^\s*[-*]\s`) embedded in prose.
    let err =
        assert_non_contractual_prose(&format!("{lead}- single-task is an end-to-end change\n"))
            .expect_err("a bullet row must FAIL the predicate");
    assert!(err.contains("bullet"), "wrong rejection reason: {err}");

    // A tabular delimiter (`id | …`) embedded in prose — extractable per-definition
    // field with no leading colon and no bullet.
    let err = assert_non_contractual_prose(&format!("{lead}single-task | an end-to-end change\n"))
        .expect_err("a tabular delimiter must FAIL the predicate");
    assert!(err.contains("tabular"), "wrong rejection reason: {err}");
}

// `describe`'s byte snapshot folded into the compose-golden sweep (M45 Inc 11 /
// pinning.md §1 — *`describe` is double-pinned today, and the golden wins*): the
// `insta` snapshot that lived here pinned `describe`'s stdout, but the sweep pins
// `describe` × every fixture state regardless, so two regen paths over the same
// bytes were the one-file-one-purpose violation the doc calls out. The
// **non-contractual-prose format predicate stays here** — it asserts a *property*,
// not bytes, and nothing in the sweep replaces it (`describe_real_output_is_...`
// above already re-runs it over the emitted bytes).

/// M47 Inc 10 / T6 — **the surface stops forbidding a parse it serves.** `jigc
/// describe --format json` has always emitted a keyed serde object (`render::describe`'s
/// `Format::Json` arm), while the verb's own help said *"don't parse it"* and
/// `design/introspection.md`'s format predicate declared the output *"not valid JSON"*.
/// Both statements were written about the **prose** surface — the one the predicate
/// above actually holds — so the binary contradicted itself for any driver that ran the
/// documented global `--format json` (law 1; the Settle routed it as settle-by-doing).
///
/// This drives the **emitted bytes** of both surfaces: the json arm parses as a keyed
/// object, and the help scopes its prohibition to the prose while stating the json arm's
/// posture — it exists, it is unpinned, and `jigc doc schema` is the versioned structural
/// read. No surface is added or removed: the arm shipped, only its description changes.
#[test]
fn describe_json_parses_and_no_surface_forbids_parsing_it() {
    let repo = TempDir::new("json-posture");
    set_up_repo(repo.path());
    let home = TempDir::new("home");

    // (a) the emitted bytes parse — a keyed object a driver can deserialize.
    let json = describe_json(repo.path(), home.path());
    assert!(
        json.as_object()
            .is_some_and(|map| map.contains_key("definitions")),
        "`describe --format json` emits a keyed object; got: {json}",
    );

    // (b) no surface forbids that parse. The help's prohibition is scoped to the prose
    // surface, and the json arm's posture is stated rather than denied.
    let help = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(["describe", "--help"])
        .current_dir(repo.path())
        .env("HOME", home.path())
        .output()
        .expect("run the jigc binary");
    assert!(help.status.success(), "`jigc describe --help` must exit 0");
    let help = String::from_utf8(help.stdout).expect("utf-8 stdout");

    assert!(
        !help.contains("don't parse it"),
        "the blanket prohibition contradicts the shipped json arm; got:\n{help}",
    );
    assert!(
        help.contains("--format json"),
        "the help must name the arm it is describing; got:\n{help}",
    );
    assert!(
        help.contains("`jigc doc schema`"),
        "and route a driver that wants a pinned structural read to the versioned \
         surface; got:\n{help}",
    );
}

#[test]
fn predicate_passes_on_legitimate_colon_bearing_prose() {
    // The predicate must NOT over-reject: legitimate discursive prose that carries
    // mid-line colons in the *permitted* forms — a line-terminal "Reach for it:" or a
    // "e.g." abbreviation — is fine. Only a line that *begins* with a colon-term, or a
    // `word: word` span a YAML parser reads as a mapping, is forbidden. This guards the
    // predicate from being so strict it would reject the very prose it admits (worked-
    // examples.md → flow 14, bar #2: the predicate "must pass on legitimate
    // colon-bearing prose"). Paragraphs are blank-line separated, like the real output.
    let prose = "\
This project lets you compose a handful of workflows and author a few document types, e.g. \
a single-task change or an adr decision record. Here is what is on the menu and when each \
is for.

Among them, single-task carries one well-scoped change all the way from intent to a \
committed result. Reach for it when the work is one coherent change you can hold in your \
head, like a bug fix, one feature, or a focused refactor. The adr captures a decision \
worth keeping, with its context and consequences, so a later reader can recover why the \
call was made.
";
    assert_non_contractual_prose(prose).unwrap_or_else(|why| {
        panic!("legitimate colon-bearing prose must PASS the predicate: {why}")
    });
}
