//! **The M45 rc.9-wave done-picture acceptance suite** — the *complete-fix
//! contract*, driven end-to-end through the **real `jigc` binary**
//! (`design/worked-examples.md` → flow 46; roadmap → Milestone 45, Increment 11).
//!
//! **The claim the wave proves: a fix is complete over its class's axis, not its
//! reported repro — and completeness is checkable by machinery, not diligence.**
//! Increments 1–10 shipped each fix as an instance *and* the axis-iterating
//! machinery that guards it; this suite is the composite acceptance that ties the
//! wave into done-picture arms over the real binary, **each arm iterating its class
//! axis rather than pinning a single instance** — so a member added to the class
//! after this suite is written is covered *by construction*, exactly the property
//! the wave exists to install ([`implementation/dev-workflow.md`] → *a fix is
//! complete over its class's axis, never its repro*).
//!
//! The five arms, each a `#[test]` over the real binary, each enumerating its axis
//! from the composed `[dev ▸ methodology]` registry (never a hand list) or from the
//! class's defining case-set:
//!
//!   (1) **Item-slot reserved-heading rejection over every doctype × item-slot** —
//!       the corruption class swept over the registry's item-slot contexts: for
//!       *every* shipped doctype with a prose item slot, a reserved-depth (`###`)
//!       heading smuggled into that slot is **atomically rejected** (non-zero exit,
//!       `write.slot-heading-depth`, staged bytes byte-identical, a followable
//!       route), and a heading-free write to the same slot lands (Inc 2).
//!
//!   (2) **Settability advertised⇺accepted parity** — the machine-maintained
//!       absolute swept over the registry: for *every* stamp-bearing doctype a
//!       forged `schema-version` stamp is refused through the write verb
//!       (`write.machine-maintained-field`), while an author-overridable `on-create`
//!       default (an ADR's `date`) stays writable — the two poles of the parity the
//!       `doc schema` projection advertises (Inc 3).
//!
//!   (3) **The role-binding render** — the copy-on-write create-gate class swept
//!       over the registry's object-form `{type, as:}` pairs: for *every* pair a
//!       re-`doc create` over a same-identity staged copy acks `existed` rather than
//!       routing away with `create.serial-collision`; and the headline symptom is
//!       cured concretely — `form-vision` on the set-field-first (revise) path binds
//!       its role and **renders its grounding** (Inc 5).
//!
//!   (4) **Owner-artifact staging** — the natural pre-staged authoring order (produce
//!       the artifact → `git add` → mint the recording task → author → finalize)
//!       **lands** at exit 0 for the registry's `owned-location` class: no
//!       `finalize.carried-staged` block, no `owner-artifact.present` finding, and
//!       the artifact + the promoted record commit together (Inc 8).
//!
//!   (5) **File-state history-gating** — the dangling-baseline severity swept over
//!       the corpus-state axis (history-absent vs history-present): a `git reset
//!       --hard` past a doc's creating commit downgrades to an **advisory**, routed at
//!       the branch switch and never at `jigc unmanage` (M55 Increment 5 / T1), and does
//!       not block, while a `git rm` + commit (history present) still **blocks** (exit 3)
//!       (Inc 7).
//!
//! Every arm asserts on the EMITTED bytes / exit codes / committed files of the real
//! binary (`CARGO_BIN_EXE_jigc`). The registry is read through the M45 enumeration
//! seam (`cli::pack`); the isolation preamble is inline (own pid+nanos `TempDir`,
//! `$HOME` and `JIGC_PACK_DIR` scrubbed), matching the flow-45 precedent. No external
//! test crates beyond `serde_json`.
//!
//! These arms are green the day they are written — the wave's fixes are all landed —
//! but each was **red on `1.0.0-rc.8`** (the reserved-depth heading corrupted, the
//! forged stamp committed, the copy-on-write role stayed unbound, the pre-staged
//! order dead-ended in a route cycle, the reset-hard baseline wedged every task):
//! the acceptance bar is that they iterate their axis, not merely that they pass.
//!
//! [`implementation/dev-workflow.md`]: ../../../implementation/dev-workflow.md

use crate::support;

use cli::pack::{CompositePack, EmbeddedPack, load_pack_schema};
use engine::compose::load_workflow_def;
use engine::packsource::{PackResourceKind, PackSource};
use engine::schema::{
    FieldType, Leaf, Repeatable, Schema, SectionBody, is_machine_maintained_absolute,
};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

// ═════════════════════════════════════════════════════════════════════════════
// Shared isolation + drive helpers (the flow-45 preamble)
// ═════════════════════════════════════════════════════════════════════════════

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-flow46-{tag}-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        ));
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

/// Run a `git` command in `repo`, asserting success, returning trimmed stdout.
fn git(repo: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout)
        .expect("utf-8 git stdout")
        .trim()
        .to_string()
}

/// Initialize a real git repo with one commit.
fn git_init(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, optionally piping `stdin`.
/// Never inherits a harness `JIGC_PACK_DIR` — the embedded packs are the base, so
/// the arms prove exactly what ships.
fn jigc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if stdin.is_some() {
        command.stdin(Stdio::piped());
    }
    let mut child = command.spawn().expect("spawn jigc");
    if let Some(bytes) = stdin {
        crate::support::child_stdin::feed(&mut child, bytes);
    }
    child.wait_with_output().expect("wait for jigc")
}

/// Assert a `jigc` invocation succeeded, surfacing its streams on failure.
fn assert_ok(out: &Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} must succeed; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// Stdout of an invocation, trailing newlines trimmed.
fn stdout_of(out: &Output) -> String {
    String::from_utf8(out.stdout.clone())
        .expect("utf-8 stdout")
        .trim_end_matches('\n')
        .to_string()
}

/// Stderr of an invocation as UTF-8.
fn stderr_of(out: &Output) -> String {
    String::from_utf8(out.stderr.clone()).expect("utf-8 stderr")
}

/// The minted task id, parsed from a task-minting compose's announcement line.
fn minted_task(composed: &str) -> String {
    composed
        .lines()
        .find_map(|l| l.strip_prefix("task minted: "))
        .expect("the compose announces the minted task id")
        .trim()
        .to_string()
}

/// A repo under a **real `jigc setup`** — the `[dev ▸ methodology]` pack-set a
/// dogfooding project ships, so both packs' doctypes/workflows compose.
fn setup_repo(tag: &str) -> (TempDir, TempDir) {
    let repo = TempDir::new(tag);
    let home = TempDir::new("home");
    git_init(repo.path());
    assert_ok(
        &jigc(repo.path(), home.path(), &["setup"], None),
        "`jigc setup`",
    );
    (repo, home)
}

/// Set one prose slot through the binary (stdin `--from-file -`), asserting success.
fn set_slot(repo: &Path, home: &Path, addr: &str, task: &str, prose: &[u8]) {
    assert_ok(
        &jigc(
            repo,
            home,
            &["doc", "set-slot", addr, "--from-file", "-", "--task", task],
            Some(prose),
        ),
        &format!("set-slot {addr}"),
    );
}

/// Set one header field through the binary, asserting success.
fn set_field(repo: &Path, home: &Path, addr: &str, task: &str, value: &str) {
    assert_ok(
        &jigc(
            repo,
            home,
            &["doc", "set-field", addr, "--value", value, "--task", task],
            None,
        ),
        &format!("set-field {addr}"),
    );
}

/// Fill the task's commit doc so a finalize renders a clean git message.
fn fill_commit(repo: &Path, home: &Path, task: &str, scope: &str) {
    set_field(repo, home, &format!("commit:{task}#type"), task, "docs");
    set_field(repo, home, &format!("commit:{task}#scope"), task, scope);
    set_slot(
        repo,
        home,
        &format!("commit:{task}#summary"),
        task,
        b"record it\n",
    );
    set_slot(
        repo,
        home,
        &format!("commit:{task}#body"),
        task,
        b"Driven by the flow-46 acceptance suite.\n",
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// The enumeration seam — the registry is the source of every axis
// ═════════════════════════════════════════════════════════════════════════════

/// The production composition, built the **CWD-free** way (`pinning.md` §1 —
/// `make_pack()` resolves against the process CWD, a hazard under parallel tests):
/// `[dev ▸ methodology]`, dev highest-precedence.
fn composite() -> CompositePack {
    CompositePack::new(vec![
        Box::new(EmbeddedPack::new()),
        Box::new(EmbeddedPack::methodology()),
    ])
}

/// Every doctype the composite registry ships, loaded through the **CLI** schema
/// loader against the doctype's own origin pack — so a pack-declared field type
/// resolves and the engine-injected `schema-version` stamp is present.
fn loaded_schemas(pack: &dyn PackSource) -> BTreeMap<String, Schema> {
    pack.list(PackResourceKind::Schemas)
        .iter()
        .map(|id| {
            let bytes = pack
                .read(PackResourceKind::Schemas, id)
                .unwrap_or_else(|e| panic!("read the `{id}` schema: {e}"));
            let origin = pack.origin_pack(PackResourceKind::Schemas, id);
            let schema = load_pack_schema(origin, &bytes)
                .unwrap_or_else(|e| panic!("load the `{id}` schema: {e}"));
            (schema.ty.clone(), schema)
        })
        .collect()
}

/// The `migrate-*` workflow granting each doctype's create gate, read out of the
/// composite registry — never a hand-written doctype→workflow map. `jigc migrate
/// <path> --as <doctype>` is the verb that composes it.
fn migrate_gates(pack: &dyn PackSource) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    for id in pack.list(PackResourceKind::Workflows) {
        if !id.as_str().starts_with("migrate-") {
            continue;
        }
        let bytes = pack
            .read(PackResourceKind::Workflows, &id)
            .unwrap_or_else(|e| panic!("read the `{id}` workflow: {e}"));
        let def = load_workflow_def(&bytes)
            .unwrap_or_else(|e| panic!("load the `{id}` workflow: {}", e.message));
        for gate in &def.allows_create {
            out.insert(gate.doc_type.clone(), id.as_str().to_string());
        }
    }
    out
}

/// The item title `jigc doc add-item` is driven with at one repeatable level,
/// **derived from the level's own `id-from` field** (an enum id-source takes its
/// first declared member, anything else takes a free string) — so an enum-keyed
/// repeatable joins the sweep without a hand-written title.
fn item_title(repeatable: &Repeatable) -> String {
    for leaf in &repeatable.block {
        let Leaf::Field(field) = leaf else { continue };
        if field.id != repeatable.id_from {
            continue;
        }
        if matches!(field.ty, FieldType::Enum) {
            return field
                .of
                .as_ref()
                .and_then(|members| members.first())
                .unwrap_or_else(|| {
                    panic!(
                        "the `{}` id-source is an enum and must declare members",
                        field.id
                    )
                })
                .clone();
        }
        break;
    }
    "Axis Item".to_string()
}

/// Write one foreign source per doctype and commit them **in one batch** — the
/// `item_slot_ceiling_axis.rs` pattern. Batching the sources into a single commit
/// (rather than a per-iteration commit) is deliberate: a per-migration commit written
/// in the same second as the preceding git index write intermittently loses the fresh
/// file to git's stat cache, so the sweep stages nothing and dies on "nothing to
/// commit". One upfront commit sidesteps it.
fn commit_foreign_sources<'a>(repo: &Path, doctypes: impl Iterator<Item = &'a String>) {
    fs::create_dir_all(repo.join("docs")).expect("create the foreign source dir");
    for doctype in doctypes {
        fs::write(
            repo.join(format!("docs/legacy-{doctype}.md")),
            format!("# Legacy {doctype}\n\nfree-form prose the migration rewrites.\n"),
        )
        .expect("write the foreign source");
    }
    git(repo, &["add", "docs"]);
    git(repo, &["commit", "-q", "-m", "the foreign sources"]);
}

/// Open `doctype`'s create gate: `jigc migrate docs/legacy-<doctype>.md --as
/// <doctype>` composes the doctype's own gate-granting `migrate-*` workflow (the
/// source is already committed by [`commit_foreign_sources`]). Returns the minted
/// task id.
fn migrate_task(repo: &Path, home: &Path, doctype: &str) -> String {
    let out = jigc(
        repo,
        home,
        &[
            "migrate",
            &format!("docs/legacy-{doctype}.md"),
            "--as",
            doctype,
        ],
        None,
    );
    assert_ok(&out, &format!("`jigc migrate --as {doctype}`"));
    minted_task(&stdout_of(&out))
}

/// `jigc doc create <doctype> --task <task>`, returning the id the binary emitted.
fn create_doc(repo: &Path, home: &Path, doctype: &str, task: &str) -> String {
    let out = jigc(
        repo,
        home,
        &[
            "doc",
            "create",
            doctype,
            "--title",
            // A singleton's `# H1` is the schema's own, so a divergent `--title` is
            // refused since M48 (`write.title-ignored`) — ask the schema.
            &support::create_title(doctype, "Axis Sweep"),
            "--task",
            task,
        ],
        None,
    );
    assert_ok(&out, &format!("`jigc doc create {doctype}`"));
    stdout_of(&out)
}

/// The staged bytes of one doc in a task's working area.
fn staged_doc(repo: &Path, task: &str, doc_id: &str) -> Vec<u8> {
    fs::read(repo.join(format!(".jigc/tasks/{task}/docs/{doc_id}.md")))
        .unwrap_or_else(|e| panic!("read staged {doc_id}: {e}"))
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 1 — item-slot reserved-heading rejection over every doctype × item-slot
// ═════════════════════════════════════════════════════════════════════════════

/// One doctype's first depth-1 item-slot context: `(section_id, item title, slot
/// id)`. `None` when the doctype ships no repeatable section carrying a prose slot.
fn first_item_slot(schema: &Schema) -> Option<(String, String, String)> {
    for section in &schema.sections {
        let SectionBody::Repeatable { repeatable } = &section.body else {
            continue;
        };
        if let Some(slot) = repeatable.block.iter().find_map(|leaf| match leaf {
            Leaf::Slot { id, .. } => Some(id.clone()),
            _ => None,
        }) {
            return Some((section.id.clone(), item_title(repeatable), slot));
        }
    }
    None
}

/// **Arm 1.** The item-slot corruption class, swept over the registry's item-slot
/// contexts. `###` is a reserved depth for *every* item-slot context the composite
/// pack-set ships (each context's ceiling reserves through at least H3 — a depth-1
/// item's own heading sits at H3, so a `###` inside its slot would mint a sibling
/// item), so a single universal poison sweeps the whole axis. For every shipped
/// doctype with a prose item slot: a `###`-bearing slot write is atomically
/// rejected (`write.slot-heading-depth`, staged bytes byte-identical, a followable
/// route), and a heading-free write to the same slot lands.
///
/// The contexts come from the engine-loaded schemas and the create gate from the
/// registry's `migrate-*` workflows — nothing hand-listed — so a new doctype with
/// an item slot joins the sweep the day it lands. Red on rc.8: a reserved-depth
/// heading in an item slot minted a ghost item / reattributed a sibling leaf /
/// hijacked a downstream section, all at exit 0 with a clean `task validate`.
#[test]
fn item_slot_reserved_heading_is_rejected_over_every_doctype() {
    let pack = composite();
    let schemas = loaded_schemas(&pack);
    let gates = migrate_gates(&pack);

    // The axis: every doctype with a prose item slot AND a create-gate door. Both
    // halves are registry-derived, so a doctype that gains an item slot (or loses
    // its migrate door) changes the swept set by construction.
    let axis: BTreeMap<String, (String, String, String)> = schemas
        .iter()
        .filter_map(|(doctype, schema)| {
            let context = first_item_slot(schema)?;
            gates
                .contains_key(doctype)
                .then(|| (doctype.clone(), context))
        })
        .collect();
    assert!(
        axis.len() >= 6,
        "the composite registry must enumerate the item-slot doctype class (a new \
         doctype is covered by construction); got: {:?}",
        axis.keys().collect::<Vec<_>>(),
    );
    // The class spans both packs — a dev-pack item slot (`spec.criteria`) and a
    // methodology-pack one (`roadmap.milestones`) are both in the sweep.
    assert!(
        axis.contains_key("spec"),
        "the dev-pack `spec` is swept: {:?}",
        axis.keys()
    );
    assert!(
        axis.contains_key("roadmap"),
        "the methodology-pack `roadmap` is swept: {:?}",
        axis.keys(),
    );

    let (repo, home) = setup_repo("item-slot");
    let repo = repo.path();
    let home = home.path();
    commit_foreign_sources(repo, axis.keys());

    for (doctype, (section, title, slot)) in &axis {
        let task = migrate_task(repo, home, doctype);
        let doc = create_doc(repo, home, doctype, &task);
        let item = stdout_of(&jigc(
            repo,
            home,
            &[
                "doc",
                "add-item",
                &format!("{doc}#{section}"),
                "--title",
                title,
                "--task",
                &task,
            ],
            None,
        ));
        let address = format!("{item}/{slot}");

        // The poison: a `###` heading smuggled into the item slot.
        let before = staged_doc(repo, &task, &doc);
        let rejected = jigc(
            repo,
            home,
            &[
                "doc",
                "set-slot",
                &address,
                "--from-file",
                "-",
                "--task",
                &task,
            ],
            Some(b"Axis prose.\n\n### Ghost  {#ghost}\n\nthe smuggled paragraph\n"),
        );
        let stderr = stderr_of(&rejected);
        assert!(
            !rejected.status.success(),
            "`{address}` ({doctype}) must refuse a `###` heading, not accept it:\n{stderr}",
        );
        assert!(
            stderr.contains("write.slot-heading-depth"),
            "`{address}` ({doctype}) refuses as the ceiling reject, not something else:\n{stderr}",
        );
        assert!(
            stderr.contains("route:"),
            "`{address}` ({doctype}) carries a route, not a dead end:\n{stderr}",
        );
        assert_eq!(
            staged_doc(repo, &task, &doc),
            before,
            "`{address}` ({doctype}): the refused write leaves the staged bytes byte-identical",
        );

        // A heading-free write to the same slot lands — the refusal is precise, not a
        // blanket lockout of the slot.
        set_slot(
            repo,
            home,
            &address,
            &task,
            b"Axis prose with no reserved heading.\n",
        );
        assert!(
            String::from_utf8_lossy(&staged_doc(repo, &task, &doc))
                .contains("Axis prose with no reserved heading."),
            "`{address}` ({doctype}): a heading-free write lands",
        );
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 2 — settability advertised⇺accepted parity
// ═════════════════════════════════════════════════════════════════════════════

/// The section id carrying `schema-version` under machine-maintained-absolute
/// semantics, if the doctype has one (the engine-injected freeze stamp).
fn stamp_section(schema: &Schema) -> Option<String> {
    for section in &schema.sections {
        let SectionBody::Simple { fields, .. } = &section.body else {
            continue;
        };
        if fields
            .iter()
            .any(|f| f.id == "schema-version" && is_machine_maintained_absolute(f))
        {
            return Some(section.id.clone());
        }
    }
    None
}

/// **Arm 2.** Settability advertised⇺accepted parity, swept over the machine-
/// maintained-absolute axis. For *every* stamp-bearing doctype the composite
/// registry ships (with a create-gate door), forging its `schema-version` freeze
/// stamp through `jigc doc set-field` is refused (`write.machine-maintained-field`,
/// a routed block) and the on-disk stamp is untouched — while an author-overridable
/// `on-create` default (an ADR's `date`) stays writable. These are the two poles of
/// the parity the `doc schema` projection advertises: a machine-maintained absolute
/// is advertised-but-write-refused, an `on-create` default is directly settable.
///
/// The stamp-bearing set is read from the engine-loaded (stamp-injected) schemas and
/// the gate from the registry's `migrate-*` workflows — so a new frozen doctype is
/// swept by construction. Red on rc.8: `set-field --value 99` on `#…/schema-version`
/// committed a forged freeze stamp that `migrate-corpus` then skipped forever.
#[test]
fn forging_the_freeze_stamp_is_refused_over_every_stamp_bearing_doctype() {
    let pack = composite();
    let schemas = loaded_schemas(&pack);
    let gates = migrate_gates(&pack);

    let axis: BTreeMap<String, String> = schemas
        .iter()
        .filter_map(|(doctype, schema)| {
            let section = stamp_section(schema)?;
            gates
                .contains_key(doctype)
                .then(|| (doctype.clone(), section))
        })
        .collect();
    assert!(
        axis.len() >= 6,
        "the composite registry must enumerate the stamp-bearing doctype class (a new \
         frozen doctype is covered by construction); got: {:?}",
        axis.keys().collect::<Vec<_>>(),
    );
    assert!(
        axis.contains_key("adr") && axis.contains_key("roadmap"),
        "the class spans both packs (`adr`, `roadmap`): {:?}",
        axis.keys(),
    );

    assert!(
        axis.contains_key("adr"),
        "the on-create positive rides adr's task: {:?}",
        axis.keys()
    );
    let (repo, home) = setup_repo("freeze-stamp");
    let repo = repo.path();
    let home = home.path();
    commit_foreign_sources(repo, axis.keys());

    for (doctype, section) in &axis {
        let task = migrate_task(repo, home, doctype);
        let doc = create_doc(repo, home, doctype, &task);
        let address = format!("{doc}#{section}/schema-version");

        let forged = jigc(
            repo,
            home,
            &[
                "doc",
                "set-field",
                &address,
                "--value",
                "99",
                "--task",
                &task,
            ],
            None,
        );
        let stderr = stderr_of(&forged);
        assert!(
            !forged.status.success(),
            "forging `{address}` ({doctype}) must block (non-zero exit):\n{stderr}",
        );
        assert!(
            stderr.contains("write.machine-maintained-field"),
            "the forge block on `{address}` ({doctype}) carries the machine-maintained code:\n{stderr}",
        );
        assert!(
            !String::from_utf8_lossy(&staged_doc(repo, &task, &doc)).contains("schema-version: 99"),
            "the forged value must not reach the staged bytes for `{doctype}`",
        );

        // The other pole of the parity, on adr's own task: an author-overridable
        // `on-create` default stays writable. The ADR `date` is `set: on-create` — the
        // changelog-migration historical-date overwrite depends on exactly this staying
        // legal, so it must NOT be swept up by the machine-maintained-absolute guard.
        if doctype == "adr" {
            set_field(
                repo,
                home,
                &format!("{doc}#status/date"),
                &task,
                "2019-02-12",
            );
            assert!(
                String::from_utf8_lossy(&staged_doc(repo, &task, &doc)).contains("2019-02-12"),
                "an on-create default (adr `date`) stays author-writable",
            );
        }
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 3 — the role-binding render
// ═════════════════════════════════════════════════════════════════════════════

/// Every **object-form** `{type, as:}` create-gate role, enumerated from the
/// composite registry (no hand list): `(workflow_id, doctype)`. A bare-form gate
/// (empty `as:`) declares no role and is skipped.
fn object_form_pairs(pack: &dyn PackSource) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for id in pack.list(PackResourceKind::Workflows) {
        let bytes = pack
            .read(PackResourceKind::Workflows, &id)
            .expect("workflow reads back");
        let def = load_workflow_def(&bytes).expect("workflow front-matter parses");
        for entry in &def.allows_create {
            if !entry.as_role.is_empty() {
                out.push((id.to_string(), entry.doc_type.clone()));
            }
        }
    }
    out
}

/// Start a workflow, returning the minted task id.
fn start(repo: &Path, home: &Path, workflow: &str, intent: &str) -> String {
    let out = jigc(repo, home, &["start", "--workflow", workflow, intent], None);
    assert_ok(&out, &format!("`jigc start --workflow {workflow}`"));
    minted_task(&stdout_of(&out))
}

/// Open a task on `workflow` **through whichever door composes it**, returning the task
/// id and the directory its `--task` writes run in.
///
/// A workflow whose `suppressed:` block declares a `door:` is verb-routed and `jigc
/// start --workflow <id>` refuses it (M52 Increment 9 / T2): the door binds the input
/// its body reads. Thirteen of this sweep's pairs are declared by such workflows — the
/// twelve `migrate-*` and `sub-task` — so reaching for `start` alone would drop them.
/// `tag` distinguishes the foreign source each migrate pair stages, all pairs sharing
/// one repo.
fn start_through_its_door(
    repo: &Path,
    home: &Path,
    workflow: &str,
    intent: &str,
    tag: usize,
) -> (String, PathBuf) {
    let Some(door) = declared_door(workflow) else {
        return (start(repo, home, workflow, intent), repo.to_path_buf());
    };
    let argv = engine::compose::Suppressed::door_argv(&door);
    if let Some(at) = argv.iter().position(|token| token == "--as") {
        // `jigc migrate <path> --as <doctype>` stages a committed foreign source and
        // mints the migration task.
        let doctype = argv[at + 1].clone();
        let rel = format!("foreign-{doctype}-{tag}.md");
        fs::write(
            repo.join(&rel),
            format!("# Foreign {doctype}\n\n## Section\n\nSome prose.\n"),
        )
        .expect("write the foreign source");
        // Committed, not merely written: `jigc migrate` refuses a source git has never
        // recorded.
        git(repo, &["add", &rel]);
        git(
            repo,
            &["commit", "-q", "-m", &format!("the foreign {doctype}")],
        );
        let out = jigc(repo, home, &["migrate", &rel, "--as", &doctype], None);
        assert_ok(&out, &format!("`jigc migrate {rel} --as {doctype}`"));
        return (minted_task(&stdout_of(&out)), repo.to_path_buf());
    }
    // `jigc workflow sub-task --task <task-id>` — the fanned sub-agent's re-entry, run
    // from its provisioned worktree: the main checkout's HEAD moves ahead of the shared
    // base pin when the milestone record commits, by design.
    assert!(
        argv.iter().any(|token| token == "<task-id>"),
        "`{workflow}`'s door `{door}` is neither the migrate nor the re-entry shape — \
         fill its placeholders here rather than skipping the member",
    );
    let milestone = format!("door-fixture-{tag}");
    assert_ok(
        &jigc(
            repo,
            home,
            &["milestone", "create", &format!("Door fixture {tag}")],
            None,
        ),
        "`jigc milestone create`",
    );
    let added = jigc(
        repo,
        home,
        &[
            "milestone",
            "add-task",
            &milestone,
            intent,
            "--workflow",
            workflow,
        ],
        None,
    );
    assert_ok(&added, "`jigc milestone add-task`");
    assert_ok(
        &jigc(repo, home, &["milestone", "provision", &milestone], None),
        "`jigc milestone provision`",
    );
    let sub = stdout_of(&added)
        .lines()
        .find_map(|line| line.trim().strip_prefix("added task:"))
        .and_then(|rest| rest.split_whitespace().next())
        .expect("`milestone add-task` acks the task it added")
        .to_string();
    let worktree = repo.join(".jigc").join("worktrees").join(&sub);
    assert_ok(
        &jigc(
            &worktree,
            home,
            &["workflow", workflow, "--task", &sub],
            None,
        ),
        &format!("`jigc workflow {workflow} --task {sub}`"),
    );
    (sub, worktree)
}

/// The `door:` a workflow's `suppressed:` block declares, read from the composite — the
/// same declaration the refusal routes at.
fn declared_door(workflow: &str) -> Option<String> {
    let bytes = composite()
        .read(
            PackResourceKind::Workflows,
            &engine::packsource::ResourceId::from(workflow),
        )
        .unwrap_or_else(|err| panic!("`{workflow}` must read from the composite: {err:?}"));
    engine::compose::load_workflow_def(&bytes)
        .unwrap_or_else(|f| panic!("`{workflow}` must load: {}", f.message))
        .suppressed
        .and_then(|s| s.door)
}

/// **Arm 3.** The copy-on-write create-gate class, swept over the registry's
/// object-form `{type, as:}` pairs. For *every* pair, a second `doc create` over the
/// already-staged same-identity copy **acks `existed`** (exit 0, "already existed")
/// instead of rejecting with `create.serial-collision` — which would route the agent
/// away from the only repairing action. Then the headline symptom is cured
/// concretely: `form-vision` on the set-field-first (revise) path binds `task.vision`
/// and its `{{ @task.vision.grounded-in#findings }}` slice **renders the grounding**.
///
/// The pair set is enumerated from the composite registry, so a new object-form gate
/// joins by construction. Red on rc.8: a copy-on-write left the role unbound (the
/// grounding slice resolved empty), and the re-create rejected with serial-collision.
#[test]
fn copy_on_write_binds_the_role_over_every_object_form_pair() {
    let pack = composite();
    let pairs = object_form_pairs(&pack);
    assert!(
        pairs.len() >= 20,
        "the composite registry must enumerate the object-form create-gate class (a new \
         pair is covered by construction); got {} pairs: {pairs:?}",
        pairs.len(),
    );
    // The class spans both symptom shapes: the silent-empty `@`-slice (`vision`) and
    // the slug-less `<<author:>>` address (`spec`).
    assert!(
        pairs.iter().any(|(_, t)| t == "vision") && pairs.iter().any(|(_, t)| t == "spec"),
        "the sweep covers both symptom shapes (vision `@`-slice, spec `<<author:>>`): {pairs:?}",
    );

    let (repo, home) = setup_repo("role-binding");
    let repo = repo.path();
    let home = home.path();

    for (index, (workflow, doctype)) in pairs.iter().enumerate() {
        // A per-pair intent so each mint slugs to a distinct, non-colliding task id
        // (all pairs share one repo, and no pair is finalized/discarded).
        let (task, cwd) = start_through_its_door(
            repo,
            home,
            workflow,
            &format!("sweep create gate pair {index}"),
            index,
        );
        // The mint title: a singleton's is the schema's own since M48, so ask.
        let title = support::create_title(doctype, "Sweep");
        // First create stages the instance and binds the role.
        assert_ok(
            &jigc(
                &cwd,
                home,
                &["doc", "create", doctype, "--title", &title, "--task", &task],
                None,
            ),
            &format!("first `doc create {doctype}` under `{workflow}`"),
        );
        // Second create over the same-identity staged copy must ack `existed`, not reject.
        let again = jigc(
            &cwd,
            home,
            &["doc", "create", doctype, "--title", &title, "--task", &task],
            None,
        );
        assert!(
            again.status.success(),
            "re-`doc create {doctype}` under `{workflow}` over a same-identity staged copy must \
             ack `existed` (exit 0), NOT reject with create.serial-collision; stderr:\n{}",
            stderr_of(&again),
        );
        assert!(
            stdout_of(&again).contains("already existed"),
            "the re-create ack must say `already existed` for `{workflow}`/`{doctype}`; got:\n{}",
            stdout_of(&again),
        );
    }

    // The headline render: `form-vision` on the set-field-first (revise) path renders
    // its grounding. Commit a grounding research doc + the vision singleton, then
    // revise the committed vision by setting `grounded-in` FIRST (a copy-on-write, no
    // prior `doc create`) and re-compose.
    const FINDINGS: &str = "A single node caps throughput under contention.";
    let research = commit_research(repo, home, FINDINGS);
    commit_vision(repo, home);

    let task = start(repo, home, "form-vision", "revise the vision");
    let before = stdout_of(&jigc(repo, home, &["start", "--task", &task], None));
    assert!(
        !before.contains(FINDINGS),
        "before `grounded-in` is set, the grounding findings must be absent; got:\n{before}",
    );
    set_field(
        repo,
        home,
        "vision:vision#meta/grounded-in",
        &task,
        &format!("[{research}]"),
    );
    let after = stdout_of(&jigc(repo, home, &["start", "--task", &task], None));
    assert!(
        after.contains(FINDINGS),
        "AFTER set-field-first copy-on-write binds `task.vision`, the \
         `@task.vision.grounded-in#findings` slice must render the grounding findings; got:\n{after}",
    );
}

/// Commit one grounding `research` doc through the real `do-research` workflow.
/// Returns the committed `research:<slug>` address.
fn commit_research(repo: &Path, home: &Path, findings: &str) -> String {
    let task = start(repo, home, "do-research", "benchmark the cache");
    let create = jigc(
        repo,
        home,
        &[
            "doc",
            "create",
            "research",
            "--title",
            "Cache Benchmarks",
            "--task",
            &task,
        ],
        None,
    );
    assert_ok(&create, "`jigc doc create research`");
    let addr = stdout_of(&create);
    set_slot(
        repo,
        home,
        &format!("{addr}#question"),
        &task,
        b"A question.\n",
    );
    set_slot(
        repo,
        home,
        &format!("{addr}#findings"),
        &task,
        format!("{findings}\n").as_bytes(),
    );
    set_slot(
        repo,
        home,
        &format!("{addr}#sources"),
        &task,
        b"Some sources.\n",
    );
    fill_commit(repo, home, &task, "research");
    assert_ok(
        &jigc(repo, home, &["task", "finalize", &task], None),
        "`jigc task finalize` (do-research grounding target)",
    );
    addr
}

/// Commit the `vision` singleton through the real `form-vision` create-first path.
fn commit_vision(repo: &Path, home: &Path) {
    let task = start(repo, home, "form-vision", "form the project vision");
    assert_ok(
        &jigc(
            repo,
            home,
            &[
                "doc", "create", "vision", "--title", "Vision", "--task", &task,
            ],
            None,
        ),
        "`jigc doc create vision`",
    );
    set_slot(
        repo,
        home,
        "vision:vision#thesis",
        &task,
        b"A context compiler.\n",
    );
    set_slot(
        repo,
        home,
        "vision:vision#invariants",
        &task,
        b"The CLI owns structure.\n",
    );
    set_slot(
        repo,
        home,
        "vision:vision#open-questions",
        &task,
        b"What earns a public pack platform?\n",
    );
    fill_commit(repo, home, &task, "vision");
    assert_ok(
        &jigc(repo, home, &["task", "finalize", &task], None),
        "`jigc task finalize` (committed VISION.md)",
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 4 — owner-artifact staging (the natural pre-staged order lands)
// ═════════════════════════════════════════════════════════════════════════════

/// The `owned-location` doctype class, enumerated from the composite registry — the
/// `owner-artifact` presence gate's members, keyed on the engine-native
/// [`FieldType::OwnedLocation`] type (not a field name), so a new owned-location
/// doctype joins by construction.
fn owned_location_doctypes(schemas: &BTreeMap<String, Schema>) -> BTreeSet<String> {
    schemas
        .iter()
        .filter(|(_, schema)| {
            schema.sections.iter().any(|s| match &s.body {
                SectionBody::Simple { fields, .. } => fields
                    .iter()
                    .any(|f| matches!(f.ty, FieldType::OwnedLocation)),
                _ => false,
            })
        })
        .map(|(doctype, _)| doctype.clone())
        .collect()
}

/// **Arm 4.** Owner-artifact staging — the natural pre-staged authoring order lands.
/// The recorded owner-artifact class is registry-derived (every doctype declaring an
/// `owner-artifact` `owned-location` field), and the natural order an orchestrator
/// follows — produce the artifact, `git add` it *before* minting the recording task,
/// author the record naming it, `finalize` — completes at exit 0: the pre-staged
/// artifact is exempt from the M43 carryover gate (no `finalize.carried-staged`), the
/// presence gate is satisfied (no `owner-artifact.present` finding), and the artifact
/// + the promoted record commit together.
///
/// Driven over a fixture `completion-record` pack (the same substrate
/// `owner_artifact_natural_order.rs` uses) so the arm proves the transaction reorder
/// without the real `completion` workflow's human-gated checkpoints. Red on rc.8: the
/// pre-mint `git add` tripped `finalize.carried-staged`, a closed route cycle.
#[test]
fn the_natural_pre_staged_order_lands_for_the_owner_artifact_class() {
    // The class is registry-derived and non-empty: the methodology pack ships
    // `owner-artifact` `owned-location` doctypes, so a new one joins by construction.
    let pack = composite();
    let schemas = loaded_schemas(&pack);
    let owned = owned_location_doctypes(&schemas);
    assert!(
        owned.contains("completion-record"),
        "the registry enumerates the owned-location class (completion-record present): {owned:?}",
    );

    let repo = TempDir::new("owner-artifact");
    let home = TempDir::new("home");
    let repo = repo.path();
    let home = home.path();
    git_init(repo);
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
    let fixture = TempDir::new("pack");
    seed_completion_pack(fixture.path());
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", fixture.path().display()),
    )
    .expect("write packs.yaml naming the fixture pack");

    // Produce the artifact and stage it FIRST — the natural order the M43 carryover
    // gate used to refuse.
    let artifact = "completions/artifacts/M45/audit.md";
    fs::create_dir_all(repo.join("completions/artifacts/M45")).expect("mk owned home");
    fs::write(repo.join(artifact), "the genuine audit transcript\n").expect("write owner-artifact");
    git(repo, &["add", artifact]);

    let task = "record-the-completion";
    assert_ok(
        &jigc(
            repo,
            home,
            &["start", "--workflow", "complete", "record the completion"],
            None,
        ),
        "`jigc start --workflow complete`",
    );
    let create = jigc(
        repo,
        home,
        &[
            "doc",
            "create",
            "completion-record",
            "--title",
            "M45 completion",
            "--task",
            task,
        ],
        None,
    );
    assert_ok(&create, "`doc create completion-record`");
    let addr = stdout_of(&create);
    set_field(
        repo,
        home,
        &format!("{addr}#meta/owner-artifact"),
        task,
        artifact,
    );
    set_slot(
        repo,
        home,
        &format!("{addr}#body"),
        task,
        b"The audit landed green.\n",
    );
    fill_commit(repo, home, task, "completion");

    let before = git(repo, &["rev-list", "--count", "HEAD"])
        .parse::<u32>()
        .unwrap();
    let out = jigc(repo, home, &["task", "finalize", task], None);
    let rendered = format!("{}{}", stdout_of(&out), stderr_of(&out));
    assert_ok(
        &out,
        &format!("the pre-staged natural order must land; got:\n{rendered}"),
    );
    assert!(
        !rendered.contains("finalize.carried-staged"),
        "the recorded owner-artifact is exempt from the carryover gate; got:\n{rendered}",
    );
    assert!(
        !rendered.contains("owner-artifact.present"),
        "a present + tracked owner-artifact surfaces no presence finding; got:\n{rendered}",
    );
    let after = git(repo, &["rev-list", "--count", "HEAD"])
        .parse::<u32>()
        .unwrap();
    assert_eq!(
        after,
        before + 1,
        "the natural order lands exactly one commit"
    );

    let committed = git(repo, &["show", "--name-only", "--format=", "HEAD"]);
    assert!(
        committed.contains(artifact),
        "the pre-staged owner-artifact lands in the commit; got:\n{committed}",
    );
    assert!(
        committed.contains("completions/m45-completion.md"),
        "the completion-record is promoted in the same commit; got:\n{committed}",
    );
}

/// Seed a fixture pack: a `completion-record` carrying the engine-native
/// `owned-location` `owner-artifact` field plus a body slot, and a `creates-task`
/// `complete` workflow whose create-gate admits it. Mirrors the substrate
/// `owner_artifact_natural_order.rs` proves the gate over.
fn seed_completion_pack(pack: &Path) {
    for sub in ["schemas", "workflows", "steps", "config"] {
        fs::create_dir_all(pack.join(sub)).expect("mk fixture pack subdir");
    }
    fs::write(
        pack.join("schemas/completion-record.yaml"),
        "type: completion-record\n\
         location: completions/\n\
         id-from: title\n\
         description: A fixture completion record carrying an owner-artifact owned location.\n\
         usage: the test substrate for the owner-artifact presence gate.\n\
         sections:\n\
         \x20 - id: meta\n\
         \x20   header: true\n\
         \x20   fields:\n\
         \x20     - { id: owner-artifact, type: owned-location }\n\
         \x20 - id: body\n\
         \x20   slot: {}\n",
    )
    .expect("seed completion-record schema");
    fs::write(
        pack.join("workflows/complete.yaml"),
        "---\n\
         when: record a milestone completion\n\
         description: A fixture workflow that creates a completion-record.\n\
         usage: proving the pre-staged natural authoring order through the binary.\n\
         creates-task: true\n\
         allows-create: [{type: completion-record, as: record}]\n\
         ---\n\
         {{ include: step:audit }}\n",
    )
    .expect("seed complete workflow");
    fs::write(
        pack.join("steps/audit.yaml"),
        "Record the milestone completion for the following intent:\n\n{{ task.intent }}\n",
    )
    .expect("seed audit step");
    fs::write(pack.join("config/commands.yaml"), "commands: []\n").expect("seed empty catalog");
    // A fixture pack that ships steps owes the four ambush-class statements since M51
    // Increment 8 T2: the stated-at fence's structural tier keys on the constituents
    // that SHIP STEPS and checks each in isolation.
    crate::support::seed_ambush_class_declarer(pack);
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 5 — file-state history-gating (the corpus-state axis)
// ═════════════════════════════════════════════════════════════════════════════

/// The committed ADR's canonical path / record key.
const ADR_PATH: &str = "docs/decisions/single-node-cache.md";

/// The findings array of a parsed JSON report envelope.
fn parse_findings(stdout: &str, what: &str) -> Vec<serde_json::Value> {
    let value: serde_json::Value = serde_json::from_str(stdout).unwrap_or_else(|e| {
        panic!("{what}: stdout must parse as the report envelope ({e}); got:\n{stdout}")
    });
    value["findings"]
        .as_array()
        .unwrap_or_else(|| {
            panic!("{what}: the envelope carries a `findings` array; got:\n{stdout}")
        })
        .clone()
}

/// The one `reconciliation.rename` finding naming `ADR_PATH`, or a panic.
fn rename_finding(findings: &[serde_json::Value], what: &str) -> serde_json::Value {
    let matches: Vec<&serde_json::Value> = findings
        .iter()
        .filter(|f| {
            f["code"] == "reconciliation.rename"
                && f["message"].as_str().is_some_and(|m| m.contains(ADR_PATH))
        })
        .collect();
    assert_eq!(
        matches.len(),
        1,
        "{what}: exactly one `reconciliation.rename` finding must name {ADR_PATH}; got:\n{findings:#?}",
    );
    matches[0].clone()
}

/// Task 0 — create + finalize `adr:single-node-cache`, whose landed finalize posts
/// its file-state baseline.
fn commit_prior_adr(repo: &Path, home: &Path) {
    assert_ok(
        &jigc(
            repo,
            home,
            &[
                "start",
                "--workflow",
                "single-task",
                "cache sessions in a single in-memory node",
            ],
            None,
        ),
        "`jigc start` (task 0)",
    );
    let task = "cache-sessions-in-a-single";
    assert_ok(
        &jigc(
            repo,
            home,
            &[
                "doc",
                "create",
                "adr",
                "--title",
                "Single-node cache",
                "--task",
                task,
            ],
            None,
        ),
        "`jigc doc create adr` (task 0)",
    );
    set_slot(
        repo,
        home,
        "adr:single-node-cache#context",
        task,
        b"Session lookups must stay fast.\n",
    );
    set_slot(
        repo,
        home,
        "adr:single-node-cache#decision",
        task,
        b"A single in-memory node.\n",
    );
    set_slot(
        repo,
        home,
        "adr:single-node-cache#consequences",
        task,
        b"A cold node loses sessions.\n",
    );
    fill_commit(repo, home, task, "cache");
    assert_ok(
        &jigc(repo, home, &["task", "finalize", task], None),
        "`jigc task finalize` (task 0)",
    );
    assert!(repo.join(ADR_PATH).exists(), "task 0 promotes {ADR_PATH}");
}

/// Mint a commit-only task with one staged code file + a filled commit doc, then
/// return its `task validate --format json` invocation.
fn validate_next_task(repo: &Path, home: &Path) -> Output {
    let task = "warm-the-read-cache";
    assert_ok(
        &jigc(
            repo,
            home,
            &["start", "--workflow", "single-task", "warm the read cache"],
            None,
        ),
        "`jigc start` (next task)",
    );
    fs::write(repo.join(format!("{task}.txt")), "the code change\n").expect("write code change");
    git(repo, &["add", &format!("{task}.txt")]);
    fill_commit(repo, home, task, "cache");
    jigc(
        repo,
        home,
        &["task", "validate", task, "--format", "json"],
        None,
    )
}

/// A repo with `single-task` available (dev pack) and the `.jigc/config/` layer — no
/// `jigc setup` (its pre-commit hook is orthogonal to the file-state probe).
fn adr_repo(tag: &str) -> (TempDir, TempDir) {
    let repo = TempDir::new(tag);
    let home = TempDir::new("home");
    git_init(repo.path());
    fs::create_dir_all(repo.path().join(".jigc").join("config")).expect("create project layer");
    (repo, home)
}

/// **Arm 5.** File-state history-gating, swept over the corpus-state axis — the two
/// poles of the dangling-baseline severity, distinguished purely by whether HEAD
/// carries history for the path:
///
///   * **history-absent** (`git reset --hard` past the ADR's creating commit) → the
///     dangling baseline downgrades to an **advisory** and the next task's `task validate`
///     does **not** block (exit 0). The repo has one branch, so no branch carries the doc
///     and the route offers `jigc unmanage` — never the switch back to a branch that does
///     not exist (M55 completion triage, CR2; the branch-switch arm, which never offers the
///     drop, is `file_state_history_gate`'s and `l2_branch_switch`'s);
///   * **history-present** (`git rm` + commit) → the same weak finding still
///     **blocks** (exit 3), a genuine deletion detected and routed.
///
/// One fix, re-derived across the whole corpus-state axis rather than the single
/// reset-hard repro. Red on rc.8: a history-less dangling baseline blocked every
/// subsequent task, wedging the corpus.
#[test]
fn file_state_history_gates_the_dangling_baseline_over_the_corpus_state_axis() {
    // ── Pole 1 · history-absent → advisory, no block. ──
    {
        let (repo, home) = adr_repo("reset");
        let repo = repo.path();
        let home = home.path();
        let root = git(repo, &["rev-parse", "HEAD"]);
        commit_prior_adr(repo, home);
        git(repo, &["reset", "--hard", &root]);
        assert!(
            !repo.join(ADR_PATH).exists(),
            "the reset removes the ADR from disk"
        );
        assert!(
            git(repo, &["log", "HEAD", "-1", "--", ADR_PATH]).is_empty(),
            "HEAD carries no history for the ADR path after the reset",
        );

        let out = validate_next_task(repo, home);
        let stdout = stdout_of(&out);
        assert!(
            out.status.success(),
            "a history-less dangling baseline must not block `task validate`; got {:?}\n{stdout}",
            out.status,
        );
        let rename = rename_finding(
            &parse_findings(&stdout, "history-less validate"),
            "history-less validate",
        );
        assert_eq!(
            rename["severity"], "advisory",
            "the history-less dangling baseline downgrades to advisory; got:\n{rename:#?}",
        );
        let route = rename["route"]
            .as_str()
            .expect("the advisory carries a route");
        assert!(
            !route.contains("switch back"),
            "no branch carries the doc, so no switch back is offered; got:\n{rename:#?}",
        );
        assert_eq!(
            route,
            format!(
                "no branch, local or remote-tracking, can bring {ADR_PATH} back — a hard \
                 reset or a rebase past its creating commit, or deleting it before it was \
                 ever committed, leaves this baseline behind: drop it with `jigc unmanage \
                 {ADR_PATH}`"
            ),
            "the advisory routes at `jigc unmanage`; got:\n{rename:#?}",
        );
    }

    // ── Pole 2 · history-present → still blocks. ──
    {
        let (repo, home) = adr_repo("gitrm");
        let repo = repo.path();
        let home = home.path();
        commit_prior_adr(repo, home);
        git(repo, &["rm", "-q", ADR_PATH]);
        git(repo, &["commit", "-q", "-m", "docs: drop the cache ADR"]);
        assert!(
            !git(repo, &["log", "HEAD", "-1", "--", ADR_PATH]).is_empty(),
            "HEAD carries history for the deleted ADR path",
        );

        let out = validate_next_task(repo, home);
        let stdout = stdout_of(&out);
        assert!(
            !out.status.success(),
            "a genuine deletion (history present) must still block; got success\n{stdout}",
        );
        assert_eq!(
            out.status.code(),
            Some(3),
            "a blocking validate exits 3; got {:?}",
            out.status
        );
        let rename = rename_finding(
            &parse_findings(&stdout, "genuine-deletion validate"),
            "genuine-deletion validate",
        );
        assert_eq!(
            rename["severity"], "blocking",
            "a genuine deletion keeps blocking; got:\n{rename:#?}",
        );
    }
}
