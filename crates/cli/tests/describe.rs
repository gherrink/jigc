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
    describe_stdout_with(repo, home, &[])
}

/// [`describe_stdout`] with `extra` argv appended — the kind-filter arms run the same
/// binary with `--workflows` / `--doctypes` / `--commands` selected.
fn describe_stdout_with(repo: &Path, home: &Path, extra: &[&str]) -> String {
    let out = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .arg("describe")
        .args(extra)
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary");
    assert!(
        out.status.success(),
        "`jigc describe {}` must exit 0; got {:?}\nstderr:\n{}",
        extra.join(" "),
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
    describe_json_with(repo, home, &[])
}

/// [`describe_json`] with `extra` argv appended — the envelope half of the kind-filter
/// arms (the filter must select the same membership in the JSON envelope as in prose).
fn describe_json_with(repo: &Path, home: &Path, extra: &[&str]) -> serde_json::Value {
    let out = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(["describe", "--format", "json"])
        .args(extra)
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary");
    assert!(
        out.status.success(),
        "`jigc describe --format json {}` must exit 0; got {:?}\nstderr:\n{}",
        extra.join(" "),
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

/// The dev pack tree on disk — the same bytes `include_dir!` embeds, selectable as
/// the **base** pack via `JIGC_PACK_DIR`.
fn dev_pack() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("pack")
}

/// The methodology pack tree on disk — the same bytes `include_dir!` embeds,
/// selectable as the **highest-precedence listed** pack via `packs.yaml`.
fn methodology_pack() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("packs")
        .join("methodology")
}

/// The `doc show` command-ref a shipped pack's catalog carries, as `(id, hint)` —
/// **derived** from that pack's `config/commands.yaml` on disk, never a hand-typed
/// id or hint (F1: *the fence iterates the derivation, never a hand-copied list*;
/// a receipt outlives what it stands for). The entry is identified **structurally**
/// — its first two `args` are the literals `doc` and `show` — so renaming the entry
/// or rewording its hint keeps the arm honest, while deleting the read-back
/// command-ref reddens it. `None` when the catalog carries no such entry.
fn catalog_doc_show_entry(pack_root: &Path) -> Option<(String, String)> {
    let path = pack_root.join("config").join("commands.yaml");
    let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    let value: serde_yaml_ng::Value =
        serde_yaml_ng::from_str(&text).expect("the command catalog parses as YAML");
    let commands = value["commands"]
        .as_sequence()
        .expect("the catalog carries a `commands:` list");

    let mut found: Vec<(String, String)> = Vec::new();
    for entry in commands {
        let Some(args) = entry["args"].as_sequence() else {
            continue;
        };
        let leading: Vec<&str> = args.iter().take(2).filter_map(|a| a.as_str()).collect();
        if leading != ["doc", "show"] {
            continue;
        }
        let id = entry["id"]
            .as_str()
            .expect("a catalog entry carries an id")
            .to_owned();
        let hint = entry["hint"].as_str().unwrap_or_default().to_owned();
        found.push((id, hint));
    }
    assert!(
        found.len() <= 1,
        "{} declares {} `doc show` command-refs; the catalog is keyed by id, so the \
         read-back surface earns exactly one",
        path.display(),
        found.len(),
    );
    found.into_iter().next()
}

/// Run `jigc describe` with the shipped pack at `pack_root` selected as the **base**
/// pack (`JIGC_PACK_DIR`), i.e. loaded **alone**: no listed pack, no compose marker.
fn describe_over_base_pack(tag: &str, pack_root: &Path) -> String {
    let repo = TempDir::new(tag);
    set_up_repo(repo.path());
    let home = TempDir::new("home");
    let out = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .arg("describe")
        .current_dir(repo.path())
        .env("HOME", home.path())
        .env("JIGC_PACK_DIR", pack_root)
        .output()
        .expect("run the jigc binary");
    assert!(
        out.status.success(),
        "`jigc describe` over {} must exit 0; got {:?}\nstderr:\n{}",
        pack_root.display(),
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout).expect("utf-8 stdout")
}

/// Run `jigc describe` with the shipped pack at `pack_root` **listed** in
/// `packs.yaml` — highest-precedence, over the embedded base. Since M49 the command
/// surface is the **union** of every constituent's catalog, each entry attributed to
/// the pack that declares it (`design/introspection.md` → Command surface), so this
/// arm sees the listed pack's own entries beside the base's.
fn describe_over_listed_pack(tag: &str, pack_root: &Path) -> String {
    let repo = TempDir::new(tag);
    set_up_repo(repo.path());
    let home = TempDir::new("home");
    fs::write(
        repo.path().join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", pack_root.display()),
    )
    .expect("write the pack list");
    let out = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .arg("describe")
        .current_dir(repo.path())
        .env("HOME", home.path())
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("run the jigc binary");
    assert!(
        out.status.success(),
        "`jigc describe` over the listed pack {} must exit 0; got {:?}\nstderr:\n{}",
        pack_root.display(),
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout).expect("utf-8 stdout")
}

/// The `pack-id` a shipped pack tree declares in its `config/defaults.yaml` — the
/// same string `PackSource::own_pack_id` reads, and the origin attribution
/// `describe` carries on every projected command-ref. Derived from the pack source,
/// never typed here.
fn pack_id_of(pack_root: &Path) -> String {
    let path = pack_root.join("config").join("defaults.yaml");
    let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    let value: serde_yaml_ng::Value =
        serde_yaml_ng::from_str(&text).expect("the pack defaults parse as YAML");
    value["pack-id"]
        .as_str()
        .unwrap_or_else(|| panic!("{} declares no `pack-id`", path.display()))
        .to_owned()
}

/// Every command-ref a shipped pack tree declares, as `(pack-id, ref id, hint)` —
/// read from that pack's own `config/commands.yaml` on disk, never a hand-typed
/// list, so a catalog entry added or reworded moves the expectation with it.
fn shipped_catalog_entries(pack_root: &Path) -> Vec<(String, String, String)> {
    let pack_id = pack_id_of(pack_root);
    let path = pack_root.join("config").join("commands.yaml");
    let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    let value: serde_yaml_ng::Value =
        serde_yaml_ng::from_str(&text).expect("the command catalog parses as YAML");
    value["commands"]
        .as_sequence()
        .expect("the catalog carries a `commands:` list")
        .iter()
        .map(|entry| {
            (
                pack_id.clone(),
                entry["id"]
                    .as_str()
                    .expect("a catalog entry carries an id")
                    .to_owned(),
                entry["hint"]
                    .as_str()
                    .expect("a catalog entry carries a hint")
                    .to_owned(),
            )
        })
        .collect()
}

/// The `(pack, id, hint)` triples `jigc describe --commands --format json` projects,
/// sorted — the emitted envelope's own view of the composed command surface.
fn projected_command_entries(repo: &Path, home: &Path) -> Vec<(String, String, String)> {
    let json = describe_json_with(repo, home, &["--commands"]);
    let mut out: Vec<(String, String, String)> = json["commands"]
        .as_array()
        .expect("the projection carries a `commands` array")
        .iter()
        .map(|c| {
            (
                c["pack"]
                    .as_str()
                    .unwrap_or_else(|| {
                        panic!(
                            "every projected command-ref must name the pack that declares it — \
                             the composed set spans more than one catalog, and an unattributed \
                             id cannot say which hint it carries. Got:\n{c:#}"
                        )
                    })
                    .to_owned(),
                c["id"].as_str().expect("a projected id").to_owned(),
                c["hint"].as_str().expect("a projected hint").to_owned(),
            )
        })
        .collect();
    out.sort();
    out
}

/// M49 Increment 11 / T6 — **the command surface is the union of every declaring
/// pack's catalog, each entry attributed to its origin.**
///
/// `describe` read `config/commands` through the composite's winner-take-all
/// whole-file `read`, so the `[dev ▸ methodology]` composition an ordinary
/// `jigc setup` project runs under projected **dev's 16 ids and nothing else** —
/// while composition resolves each workflow's `{{cli.X}}` against *its own* origin
/// pack's catalog (`start.rs` → `origin_pack(Workflows, id)` → `load_catalog`), so
/// eleven methodology command-refs that every methodology workflow really composes
/// appeared on no menu at all. That is law 2 (`surface-contract.md`): the capability
/// exists and the surface that exists to name it hides it.
///
/// Driven over the **emitted bytes** of the real binary, with the expectation
/// **derived from both pack sources on disk** rather than a hand-typed set — a
/// catalog entry added to either pack moves this arm with it.
#[test]
fn describe_commands_carry_the_union_of_every_declaring_pack() {
    let repo = TempDir::new("catalog-union");
    set_up_repo(repo.path());
    fs::write(
        repo.path().join(".jigc").join("config").join("packs.yaml"),
        "compose-embedded-methodology: true\n",
    )
    .expect("write the compose marker");
    let home = TempDir::new("home");

    let dev = shipped_catalog_entries(&dev_pack());
    let methodology = shipped_catalog_entries(&methodology_pack());

    // Non-vacuity, derived: the two catalogs must genuinely diverge, or a
    // winner-take-all projection would satisfy the union assert by accident.
    let dev_ids: std::collections::BTreeSet<&str> =
        dev.iter().map(|(_, id, _)| id.as_str()).collect();
    let methodology_only: Vec<String> = methodology
        .iter()
        .map(|(_, id, _)| id.clone())
        .filter(|id| !dev_ids.contains(id.as_str()))
        .collect();
    assert!(
        !methodology_only.is_empty(),
        "the two shipped catalogs must declare at least one divergent id, or this arm cannot \
         tell a union from the precedence winner",
    );

    let mut expected: Vec<(String, String, String)> = dev.into_iter().chain(methodology).collect();
    expected.sort();

    assert_eq!(
        projected_command_entries(repo.path(), home.path()),
        expected,
        "`jigc describe --commands --format json` must carry every command-ref either pack \
         declares, attributed to the pack that declares it",
    );

    // The prose arm an agent actually reads carries the same union — the envelope is
    // not a private surface (`introspection.md` → the filter selects membership, and
    // both renderings are the same assembled projection).
    let out = describe_stdout_with(repo.path(), home.path(), &["--commands"]);
    for id in &methodology_only {
        assert!(
            out.contains(id),
            "the emitted prose menu must name the methodology-only command-ref `{id}`; \
             got:\n{out}",
        );
    }
    assert_non_contractual_prose(&out).unwrap_or_else(|why| {
        panic!(
            "the unioned command menu must stay hostile-to-parsing: {why}\n--- output ---\n{out}"
        )
    });
}

/// M48 Increment 3, T1 — **`jigc doc show` joins both packs' command catalogs.**
///
/// Six consecutive trials landed the discoverability lens, and the pre-1.0.0 trial
/// made it countable: all three blind sessions went to the filesystem to read their
/// own in-flight work while `jigc doc show --task <id>` has shipped since M43. The
/// menu is where a capability stops hiding (`surface-contract.md` → law 2), and the
/// menu projects the **command catalog** — which carried no read verb at all in
/// either shipped pack.
///
/// Driven over the **emitted bytes** of the real binary, per shipped pack **loaded
/// alone** (dev as the `JIGC_PACK_DIR` base, methodology as the highest-precedence
/// listed pack — `config/commands` resolves winner-take-all, so each arm projects
/// exactly one catalog), with the expectation **derived from the loaded catalog**
/// rather than a hand-typed string.
#[test]
fn describe_names_doc_show_in_both_shipped_catalogs() {
    let dev = dev_pack();
    let methodology = methodology_pack();

    for (label, pack_root, out) in [
        (
            "dev",
            dev.as_path(),
            describe_over_base_pack("readback-dev", &dev),
        ),
        (
            "methodology",
            methodology.as_path(),
            describe_over_listed_pack("readback-methodology", &methodology),
        ),
    ] {
        let (id, hint) = catalog_doc_show_entry(pack_root).unwrap_or_else(|| {
            panic!(
                "the {label} pack's catalog must carry a `jigc doc show` command-ref — the \
                 read-back of an in-flight write is the capability six trials went to the \
                 filesystem for; {}/config/commands.yaml declares none",
                pack_root.display(),
            )
        });
        assert!(
            !hint.trim().is_empty(),
            "the {label} pack's `{id}` command-ref must carry a non-empty hint — the hint IS \
             what describe projects",
        );
        assert!(
            out.contains(&format!("{id} ({} pack) {hint}", pack_id_of(pack_root))),
            "`jigc describe` over the {label} pack alone must name its `{id}` command-ref, the \
             pack that declares it, and its authored hint; got:\n{out}",
        );
        // The new entry must not cost the surface its posture — describe stays a menu,
        // not a table (`introspection.md` → The operational format contract).
        assert_non_contractual_prose(&out).unwrap_or_else(|why| {
            panic!(
                "the {label} projection must stay hostile-to-parsing: {why}\n--- output ---\n{out}"
            )
        });
    }
}

/// The composite arm of the same claim: the `[dev ▸ methodology]` composition an
/// ordinary `jigc setup` project runs under names the read-back surface — **from
/// both catalogs**. The declared bound this arm carried (*"`CompositePack::read` is
/// winner-take-all whole-file for `config/commands`, so the composite projects the
/// dev catalog only"*) is discharged by M49 Increment 11 / T6: the projection is the
/// union of every declaring pack's catalog, so the methodology entry is no longer
/// reachable only through the methodology-alone arm.
#[test]
fn describe_names_doc_show_on_the_composite_path() {
    let repo = TempDir::new("readback-composite");
    set_up_repo(repo.path());
    fs::write(
        repo.path().join(".jigc").join("config").join("packs.yaml"),
        "compose-embedded-methodology: true\n",
    )
    .expect("write the compose marker");
    let home = TempDir::new("home");

    let out = describe_stdout(repo.path(), home.path());
    for pack_root in [dev_pack(), methodology_pack()] {
        let (id, hint) = catalog_doc_show_entry(&pack_root).unwrap_or_else(|| {
            panic!(
                "{} must carry a `jigc doc show` command-ref",
                pack_root.display()
            )
        });
        let pack_id = pack_id_of(&pack_root);
        assert!(
            out.contains(&format!("{id} ({pack_id} pack) {hint}")),
            "the composite `[dev ▸ methodology]` projection must name the {pack_id} catalog's \
             `{id}` command-ref; got:\n{out}",
        );
    }
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

/// One shipped workflow's catalog-relevant front-matter, read from the pack source
/// on disk: its id, whether it mints a task, whether it is selectable, and the
/// `suppressed.reason` it declares (if any). Both booleans default **true** when the
/// key is omitted, exactly as `WorkflowFrontMatter` does.
struct ShippedWorkflow {
    id: String,
    creates_task: bool,
    selectable: bool,
    reason: Option<String>,
}

/// Every workflow the two shipped embedded packs declare, read from the pack sources
/// on disk (the same files `include_dir!` embeds). Derived, not hard-coded — the
/// workflow set grows and shrinks with the packs.
fn shipped_workflows() -> Vec<ShippedWorkflow> {
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
    let mut out = Vec::new();
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
            out.push(ShippedWorkflow {
                id: path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .expect("workflow id from file stem")
                    .to_owned(),
                creates_task: value["creates-task"].as_bool().unwrap_or(true),
                selectable: value["selectable"].as_bool().unwrap_or(true),
                reason: value["suppressed"]["reason"].as_str().map(str::to_owned),
            });
        }
    }
    out
}

/// The `suppressed: {reason, …}` declarations of every workflow the router catalog
/// leaves out — its complement, `!(creates-task && selectable)`, which is exactly the
/// population `catalog.rs` filters away — keyed `(workflow id, reason)`. Derived from
/// the pack sources, never a hand list; the `expect` is the fence's own claim, so a
/// shipped workflow that falls off the catalog with no declared reason reddens here.
fn shipped_off_catalog_workflows() -> Vec<(String, String)> {
    shipped_workflows()
        .into_iter()
        .filter(|w| !(w.creates_task && w.selectable))
        .map(|w| {
            let reason = w.reason.unwrap_or_else(|| {
                panic!(
                    "workflow `{}` sits off the router catalog and declares no \
                     `suppressed.reason` — every workflow the catalog leaves out owes the \
                     reader the reason it is absent (the pack-load suppression fence)",
                    w.id,
                )
            });
            (w.id, reason)
        })
        .collect()
}

/// The hidden (`selectable: false`) subset of the above — the M43 fence's original
/// population, kept as its own derivation so the arm that pins it stays about
/// *deliberate* suppression rather than the wider catalog complement.
fn shipped_hidden_workflows() -> Vec<(String, String)> {
    shipped_workflows()
        .into_iter()
        .filter(|w| !w.selectable)
        .map(|w| {
            let reason = w.reason.expect(
                "a hidden shipped workflow carries suppressed.reason (the pack-load fence)",
            );
            (w.id, reason)
        })
        .collect()
}

/// M49 Increment 11 / T6 — **every workflow the router catalog leaves out states why
/// it is absent.**
///
/// `step:route-to-workflow` tells the reader that the catalog is a subset and that
/// `jigc describe --workflows` carries each absent one's reason. The M43 fence bought
/// that promise for `selectable: false` only, so the three workflows off the catalog
/// for the *other* reason — `creates-task: false` — narrated nothing at all
/// (`ingest-existing`, `router`, `increment`). The fence's subject is now the
/// catalog's **complement**, not one of its two causes.
///
/// The set is **derived** from both pack sources (`shipped_off_catalog_workflows`),
/// never hand-listed, and the assertion runs over the emitted bytes of the real
/// binary in both renderings.
#[test]
fn describe_states_why_every_off_catalog_workflow_is_absent() {
    let repo = TempDir::new("off-catalog");
    set_up_repo(repo.path());
    fs::write(
        repo.path().join(".jigc").join("config").join("packs.yaml"),
        "compose-embedded-methodology: true\n",
    )
    .expect("write the compose marker");
    let home = TempDir::new("home");

    let off_catalog = shipped_off_catalog_workflows();
    let hidden: std::collections::BTreeSet<String> = shipped_hidden_workflows()
        .into_iter()
        .map(|(id, _)| id)
        .collect();
    // Non-vacuity, derived: the arm only proves something new if the complement is
    // strictly wider than the `selectable: false` set M43 already fenced.
    assert!(
        off_catalog.iter().any(|(id, _)| !hidden.contains(id)),
        "the shipped packs must carry at least one `creates-task: false` workflow, or this arm \
         re-proves the M43 hidden set; got {off_catalog:?}",
    );

    let json = describe_json(repo.path(), home.path());
    let definitions = json["definitions"]
        .as_array()
        .expect("the projection carries a definitions array");
    let out = describe_stdout(repo.path(), home.path());

    for (id, reason) in &off_catalog {
        let definition = definitions
            .iter()
            .find(|d| d["id"].as_str() == Some(id))
            .unwrap_or_else(|| panic!("off-catalog workflow `{id}` must be narrated by describe"));
        let reason = reason.trim().trim_end_matches('.');
        assert_eq!(
            definition["router_hidden"].as_str(),
            Some(reason),
            "`{id}` is off the router catalog, so `router_hidden` must carry its declared \
             reason; got:\n{definition:#}",
        );
        let prose = definition["prose"].as_str().expect("a definition prose");
        assert!(
            prose.contains("hidden from the router catalog"),
            "`{id}`'s entry must say it is absent from the router catalog; got: {prose:?}",
        );
        assert!(
            out.contains(reason),
            "the emitted prose must carry `{id}`'s reason for being off the catalog; got:\n{out}",
        );
    }

    assert_non_contractual_prose(&out).unwrap_or_else(|why| {
        panic!(
            "the projection with every off-catalog reason woven in must stay non-contractual \
             prose: {why}\n--- output ---\n{out}"
        )
    });
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

/// M48 Inc 7 / T5 — **the judgment tier's one close.** `jigc describe --format json`
/// carries a hidden workflow's suppression as a **structured key** (`router_hidden`),
/// not only as a substring of the woven `prose`.
///
/// The census's rule is the wave's: *a value the human/agent text already prints, but
/// the `--format json` envelope withholds, is a gap* (DECISIONS → 2026-08-13 the
/// Settle, the pre-1.0 additive-key window). `describe`'s prose surface has named the
/// router-hidden state — and the declared reason for it — since M43's suppression fence
/// (`surface-contract.md` → law 2), while a driver reading the envelope could only
/// recover the fact by substring-matching the prose sentence. `describe`'s **prose** tier
/// stays deliberately non-contractual (the format predicate above); the key joins the
/// envelope, which is the arm nothing forbids parsing.
///
/// Driven through the **emitted bytes** of the real binary over the **shipped** packs, so
/// the arm is about what an agent actually receives: every definition whose prose states
/// the router-hidden clause carries `router_hidden` = the declared reason, every other
/// definition carries `null`, and at least one hidden workflow exists (non-vacuity).
#[test]
fn describe_json_carries_the_router_hidden_suppression_as_a_key() {
    const CLAUSE: &str = "is hidden from the router catalog: ";

    let repo = TempDir::new("router-hidden");
    set_up_repo(repo.path());
    let home = TempDir::new("home");

    let json = describe_json(repo.path(), home.path());
    let definitions = json["definitions"]
        .as_array()
        .expect("the projection carries `definitions`")
        .clone();

    let mut hidden_seen = 0usize;
    for definition in &definitions {
        let id = definition["id"].as_str().unwrap_or("<no id>");
        let prose = definition["prose"].as_str().unwrap_or_default();
        let key = definition.get("router_hidden").unwrap_or_else(|| {
            panic!(
                "`{id}`'s projection must carry `router_hidden` — the router-hidden state the \
                 prose states is a value the envelope withheld (M48, the pre-1.0 additive-key \
                 window). Got:\n{definition:#}"
            )
        });

        match prose.split_once(CLAUSE) {
            Some((_, tail)) => {
                let reason = tail.strip_suffix('.').unwrap_or(tail);
                assert_eq!(
                    key.as_str(),
                    Some(reason),
                    "`{id}` is hidden, so `router_hidden` must carry the DECLARED REASON its \
                     prose names, not a same-shaped different fact. Got:\n{definition:#}",
                );
                hidden_seen += 1;
            }
            None => assert!(
                key.is_null(),
                "`{id}` is not hidden from the router catalog, so `router_hidden` must be \
                 null — a key that lies is worse than one that is absent. Got:\n{definition:#}",
            ),
        }
    }

    assert!(
        hidden_seen > 0,
        "the shipped packs must still carry at least one router-hidden workflow, or this arm \
         proves nothing; got {} definitions",
        definitions.len(),
    );
}

/// M48 Inc 8 / T3 — **the menu can be asked for the part it needs.**
///
/// `jigc describe` is the whole menu: 33 workflows (18 of them narrating "hidden from
/// the router catalog", 12 of them `migrate-*`) plus every doctype plus every
/// composed pack's command-refs on one 1,167-char line — 24kB an agent reads to find one kind of thing
/// (DECISIONS → 2026-08-13 the Settle, `describe` — the filter only, not the positional
/// form). `--workflows` / `--doctypes` / `--commands` select **which entries** the menu
/// returns; they are combinable, and **no flag is the whole menu** (today's behaviour,
/// byte-unchanged).
///
/// The arm pins **membership only** — which ids the surface returns and which it
/// withholds — never what any entry *says*: `describe`'s prose tier is fenced
/// non-contractual by design (`introspection.md` → The operational format contract), so
/// every filtered arm additionally re-runs [`assert_non_contractual_prose`]. The
/// expectation is **derived** from the binary's own unfiltered inventory rather than a
/// hand-typed id list — the packs grow every wave.
///
/// The axis is the **whole subset lattice** (all 2³ flag combinations, the empty one
/// included), driven through the emitted bytes of both format arms: a filter that
/// selected in prose while the envelope kept serving the whole menu — or the reverse —
/// would pass a single happy-path arm and fail here.
#[test]
fn describe_kind_filter_selects_which_entries_the_menu_returns() {
    // The three selectable kinds: the flag that selects it, and the prose transition
    // that leads its group (present iff the kind is selected).
    const KINDS: [(&str, &str); 3] = [
        ("--workflows", "The workflows you can compose here."),
        ("--doctypes", "The doc-types you can author."),
        (
            "--commands",
            "And the commands jigc hands you along the way.",
        ),
    ];

    let repo = TempDir::new("kind-filter");
    set_up_repo(repo.path());
    let home = TempDir::new("home");

    // The whole menu, derived from the binary itself — `(id, the text the prose surface
    // carries for that entry)` per kind.
    let whole = describe_json(repo.path(), home.path());
    let definitions = whole["definitions"]
        .as_array()
        .expect("the projection carries a definitions array");
    let of_kind = |kind: &str| -> Vec<(String, String)> {
        definitions
            .iter()
            .filter(|d| d["kind"].as_str() == Some(kind))
            .map(|d| {
                (
                    d["id"].as_str().expect("a definition id").to_owned(),
                    d["prose"].as_str().expect("a definition prose").to_owned(),
                )
            })
            .collect()
    };
    let workflows = of_kind("workflow");
    let doctypes = of_kind("doctype");
    let commands: Vec<(String, String)> = whole["commands"]
        .as_array()
        .expect("the projection carries a commands array")
        .iter()
        .map(|c| {
            let id = c["id"].as_str().expect("a command-ref id");
            let pack = c["pack"].as_str().expect("a command-ref origin pack");
            let hint = c["hint"].as_str().expect("a command-ref hint");
            // The prose sentence names the declaring pack — an id alone does not
            // identify an entry under a composed pack-set (M49 Inc 11 / T6).
            (id.to_owned(), format!("{id} ({pack} pack) {hint}"))
        })
        .collect();
    let menu = [&workflows, &doctypes, &commands];

    for (kind, entries) in KINDS.iter().zip(menu) {
        assert!(
            !entries.is_empty(),
            "the shipped packs must narrate at least one `{}` entry, or this arm proves nothing",
            kind.0,
        );
    }

    for mask in 0u8..8 {
        let selected: Vec<&str> = KINDS
            .iter()
            .enumerate()
            .filter(|(i, _)| mask & (1 << i) != 0)
            .map(|(_, (flag, _))| *flag)
            .collect();
        // No flag selects nothing — it selects everything (the whole menu is the default).
        let want: [bool; 3] = if mask == 0 {
            [true; 3]
        } else {
            [mask & 1 != 0, mask & 2 != 0, mask & 4 != 0]
        };
        let shown = format!("jigc describe {}", selected.join(" "));

        // ── the envelope half ────────────────────────────────────────────────────
        let json = describe_json_with(repo.path(), home.path(), &selected);
        let got = json["definitions"]
            .as_array()
            .expect("the filtered projection still carries a definitions array");
        for (i, kind) in ["workflow", "doctype"].iter().enumerate() {
            let got_ids: Vec<&str> = got
                .iter()
                .filter(|d| d["kind"].as_str() == Some(kind))
                .map(|d| d["id"].as_str().expect("a definition id"))
                .collect();
            let want_ids: Vec<&str> = if want[i] {
                menu[i].iter().map(|(id, _)| id.as_str()).collect()
            } else {
                Vec::new()
            };
            assert_eq!(
                got_ids, want_ids,
                "`{shown} --format json` must return exactly the selected {kind} membership",
            );
        }
        let got_command_ids: Vec<&str> = json["commands"]
            .as_array()
            .expect("the filtered projection still carries a commands array")
            .iter()
            .map(|c| c["id"].as_str().expect("a command-ref id"))
            .collect();
        let want_command_ids: Vec<&str> = if want[2] {
            commands.iter().map(|(id, _)| id.as_str()).collect()
        } else {
            Vec::new()
        };
        assert_eq!(
            got_command_ids, want_command_ids,
            "`{shown} --format json` must return exactly the selected command-ref membership",
        );

        // ── the prose half ───────────────────────────────────────────────────────
        let out = describe_stdout_with(repo.path(), home.path(), &selected);

        // Membership, by entry: every selected entry's text is carried, every
        // withheld one's is gone. An entry whose text a *selected* kind also carries
        // is skipped rather than asserted absent (ids are unique within a kind, not
        // across kinds — the derivation stays honest if that ever collides).
        let included: Vec<&str> = KINDS
            .iter()
            .enumerate()
            .filter(|(i, _)| want[*i])
            .flat_map(|(i, _)| menu[i].iter().map(|(_, text)| text.as_str()))
            .collect();
        for (i, (flag, lead)) in KINDS.iter().enumerate() {
            for (id, text) in menu[i] {
                if want[i] {
                    assert!(
                        out.contains(text.as_str()),
                        "`{shown}` selects `{flag}`, so `{id}`'s entry must be returned; got:\n{out}",
                    );
                } else if !included.contains(&text.as_str()) {
                    assert!(
                        !out.contains(text.as_str()),
                        "`{shown}` does not select `{flag}`, so `{id}`'s entry must be withheld; got:\n{out}",
                    );
                }
            }
            assert_eq!(
                out.contains(lead),
                want[i],
                "`{shown}`: the `{flag}` group transition must appear iff the kind is selected; got:\n{out}",
            );
        }

        // The footer rides every arm, and the prose tier stays fenced non-contractual —
        // the filter pins membership, never prose.
        assert!(
            out.trim_end().ends_with(ROUTING_FOOTER),
            "`{shown}` must still carry the routing footer; got:\n{out}",
        );
        assert_non_contractual_prose(&out).unwrap_or_else(|why| {
            panic!("`{shown}` must stay hostile-to-parsing: {why}\n--- output ---\n{out}")
        });
    }
}

/// The foreclosure **stands**: a filter still returns the menu, so it does not cross the
/// `describe` / `--explain` boundary `design/introspection.md` → Command surface draws —
/// and the single-item positional form it forecloses (`jigc describe <id>`) is still
/// refused, filter flags or not (DECISIONS → 2026-08-13 the Settle, `describe` — the
/// filter only, not the positional form). Driven through the emitted bytes: the run
/// fails and prints no menu.
#[test]
fn describe_still_refuses_a_positional_argument() {
    let repo = TempDir::new("no-positional");
    set_up_repo(repo.path());
    let home = TempDir::new("home");

    for argv in [
        vec!["describe", "single-task"],
        vec!["describe", "--workflows", "single-task"],
    ] {
        let out = Command::new(env!("CARGO_BIN_EXE_jigc"))
            .args(&argv)
            .current_dir(repo.path())
            .env("HOME", home.path())
            .output()
            .expect("run the jigc binary");
        assert!(
            !out.status.success(),
            "`jigc {}` must be refused — the single-item form is not built; got {:?}",
            argv.join(" "),
            out.status,
        );
        assert!(
            out.stdout.is_empty(),
            "a refused `jigc {}` must print no menu; got:\n{}",
            argv.join(" "),
            String::from_utf8_lossy(&out.stdout),
        );
    }
}

/// Pull every backticked `jigc …` span out of a tip line — the emitted spans an agent
/// would copy, in the order the tip prints them.
fn backticked_jigc_spans(tip: &str) -> Vec<String> {
    tip.split('`')
        .skip(1)
        .step_by(2)
        .filter(|span| span.starts_with("jigc "))
        .map(str::to_string)
        .collect()
}

/// **Law 2 — the designated recovery is named by the surface that produces the state.**
/// The single-item form stays foreclosed (the arm above holds unmodified), but the
/// refusal no longer stops at clap's bare `unexpected argument`: it now carries the
/// answer `crates/cli/src/cli.rs`'s own `Describe` doc has recorded all along — the kind
/// filters narrow the tour, and `jigc start --explain` is the resolution trace
/// (`completions/artifacts/M46/razor-ledger.md` §1 S-2).
///
/// Driven through the **emitted bytes**: every `jigc …` span the tip prints is pulled out
/// of the real binary's stderr and **run verbatim**, and each must exit 0 — a tip naming
/// an affordance that does not run is law 1's lie moved into the tip slot.
#[test]
fn the_foreclosed_positional_refusal_names_what_answers_it() {
    let repo = TempDir::new("positional-tip");
    set_up_repo(repo.path());
    let home = TempDir::new("home");

    for argv in [
        vec!["describe", "adr"],
        vec!["describe", "--workflows", "single-task"],
    ] {
        let shown = argv.join(" ");
        let out = Command::new(env!("CARGO_BIN_EXE_jigc"))
            .args(&argv)
            .current_dir(repo.path())
            .env("HOME", home.path())
            .output()
            .expect("run the jigc binary");
        let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
        let tip = stderr
            .lines()
            .find(|line| line.trim_start().starts_with("tip: "))
            .unwrap_or_else(|| {
                panic!("`jigc {shown}` must name what answers the foreclosed form; got:\n{stderr}")
            })
            .to_string();

        for flag in ["--workflows", "--doctypes", "--commands"] {
            assert!(
                tip.contains(&format!("jigc describe {flag}")),
                "`jigc {shown}`: the tip must name the `{flag}` kind filter; got:\n{tip}",
            );
        }
        assert!(
            tip.contains("jigc start --explain"),
            "`jigc {shown}`: the tip must name the resolution trace `cli.rs` records; got:\n{tip}",
        );

        let spans = backticked_jigc_spans(&tip);
        assert!(
            !spans.is_empty(),
            "`jigc {shown}`: the tip must name its affordances as runnable spans; got:\n{tip}",
        );
        for span in spans {
            let words: Vec<&str> = span.split_whitespace().collect();
            let run = Command::new(env!("CARGO_BIN_EXE_jigc"))
                .args(&words[1..])
                .current_dir(repo.path())
                .env("HOME", home.path())
                .output()
                .expect("run the tip's own span verbatim");
            assert!(
                run.status.success(),
                "`jigc {shown}`: the tip's span `{span}`, run verbatim, must exit 0; got {:?}\nstderr:\n{}",
                run.status,
                String::from_utf8_lossy(&run.stderr),
            );
        }
    }
}
