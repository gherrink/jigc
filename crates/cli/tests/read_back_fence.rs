//! Real-binary acceptance for the M48 **read-back fence** (law 2 —
//! `design/surface-contract.md` → The stated-at fence, the write-solicit tier;
//! `design/doc-read-surface.md` → R7, the staged read): a step that solicits a
//! managed-doc write must name the read that shows the agent what it just wrote —
//! `jigc doc show <addr> --task <id>`, which serves the task's **staged** copy the
//! committed store does not carry yet.
//!
//! Six consecutive trials went to the filesystem to read their own in-flight work
//! while that capability had shipped since M43. The mechanism behind all six is
//! that no soliciting surface named it, so this suite pins the *mechanism*: the
//! owe-set is **recomputed from the pack tree itself**, never hand-listed —
//!
//!   - the tree's own `config/commands.yaml`, filtered to the catalog entries whose
//!     argv is `jigc doc <write-verb>`, where the write-verb partition is the
//!     production one (`cli::doc::doc_write_verbs` — the clap `doc` leaf set minus
//!     the declared read verbs `show`/`schema`/`list`), matched against each step
//!     body's lone-line `{{cli.<id>}}` refs;
//!   - **union** each step body's `{{schema:<T>}}` authoring-payload refs (the
//!     migrate author templates solicit their whole write through the projection
//!     and carry no `{{cli.<id>}}` ref at all).
//!
//! — and asserted equal to the set of steps declaring `read.staged-read-back` in
//! that tree. Then the fence's own redness is driven over that same derived set,
//! member by member, through the real binary (the `stated_at_fence.rs` mutation
//! mold): withdrawing one step's declaration from a copied tree blocks pack load
//! non-zero, naming the step and the code.
//!
//! The shipped token axis — the named facts a declaration buys
//! (`cli::pack::CONSTRAINT_REQUIRED_TOKENS`'s `read.staged-read-back` row:
//! `jigc doc show` · `--task`) — is swept by `stated_at_fence.rs` and
//! `flow47_acceptance.rs` arm 5, which iterate the map rather than a list, so the
//! new row joins them by construction.
//!
//! Declared bounds, recorded rather than silently narrowed: the **reverse**
//! direction (a declarer that solicits no write) is out of scope for this tier —
//! the fence buys presence at the solicit, never the absence of a stray
//! declaration; and a step whose writes are solicited only as **literal** command
//! lines (the dev pack's `locate-from-spec` sets fields with hand-written
//! `jigc doc set-field` lines and no `{{cli.<id>}}` ref) carries no structural
//! signal, so it states the read-back without joining the fenced set.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-read-back-{tag}-{}-{:?}",
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

/// The embedded dev pack tree — the faithful source the on-disk copies mirror.
fn embedded_pack_tree() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("pack")
}

/// The on-disk methodology pack home (`<root>/packs/methodology`).
fn methodology_pack_tree() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("packs")
        .join("methodology")
}

/// Recursively copy `src` into `dst` (both directories), creating `dst`.
fn copy_tree(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).expect("create copy target dir");
    for entry in fs::read_dir(src).expect("read pack dir") {
        let entry = entry.expect("dir entry");
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if from.is_dir() {
            copy_tree(&from, &to);
        } else {
            fs::copy(&from, &to).expect("copy pack file");
        }
    }
}

/// Copy a shipped pack tree into a fresh temp dir and return the copy's root.
fn pack_copy(tag: &str, src: &Path) -> TempDir {
    let dir = TempDir::new(tag);
    copy_tree(src, dir.path());
    dir
}

/// Initialize a real git repo with one commit (composition reads HEAD) plus the
/// `.jigc/config/` project layer the cascade expects.
fn init_repo(root: &Path) {
    let git = |args: &[&str]| {
        let out = Command::new("git")
            .args(args)
            .current_dir(root)
            .output()
            .expect("run git");
        assert!(
            out.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    };
    git(&["init", "-q"]);
    git(&["config", "user.email", "test@example.com"]);
    git(&["config", "user.name", "Test"]);
    fs::write(root.join("README.md"), "hello\n").expect("write file");
    git(&["add", "."]);
    git(&["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(root.join(".jigc").join("config")).expect("create project layer");
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, `JIGC_PACK_DIR = pack`.
fn run_with_pack(repo: &Path, home: &Path, pack: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", pack)
        .output()
        .expect("spawn the jigc binary")
}

/// Run `jigc <args>` with NO `JIGC_PACK_DIR` (the embedded / listed-pack path).
fn run_embedded(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("spawn the jigc binary")
}

/// `workflow <id> --preview` composes through the identical pack load and mints
/// nothing, so a per-member sweep can run it once per mutation without minting a
/// task each time.
const PREVIEW: &[&str] = &["workflow", "single-task", "--preview"];

/// Point a project at a copied pack tree through `packs.yaml` (the listed-pack
/// path — each manifest-shipping constituent is fenced in isolation).
fn list_pack(repo: &Path, pack: &Path) {
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", pack.display()),
    )
    .expect("write packs.yaml naming the pack copy");
}

// ---------------------------------------------------------------------------
// The derivation — recomputed here from the tree's own bytes and the production
// write-verb partition, so the suite pins the RULE and not a snapshot of today's
// pack contents.
// ---------------------------------------------------------------------------

/// Load a pack tree's step defs, sorted by id.
fn step_defs(pack: &Path) -> Vec<(String, engine::compose::StepDef)> {
    let mut steps: Vec<PathBuf> = fs::read_dir(pack.join("steps"))
        .expect("read the copied steps dir")
        .map(|entry| entry.expect("dir entry").path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "yaml"))
        .collect();
    steps.sort();
    steps
        .into_iter()
        .map(|path| {
            let id = path
                .file_stem()
                .expect("step file stem")
                .to_string_lossy()
                .into_owned();
            let bytes = fs::read(&path).expect("read the copied step");
            let def =
                engine::compose::load_step_def(&id, &bytes).expect("step front-matter parses");
            (id, def)
        })
        .collect()
}

/// The tree's own catalog ids whose command-ref is a `jigc doc <write-verb>` call —
/// the write half of the derivation's first signal. The write-verb partition is the
/// production one, so a new `doc` verb widens this set without a second hand list.
fn doc_write_command_ids(pack: &Path) -> BTreeSet<String> {
    let bytes = fs::read(pack.join("config").join("commands.yaml"))
        .expect("the pack ships a command catalog");
    let catalog = engine::compose::load_command_catalog(&bytes).expect("the catalog parses");
    let write_verbs = cli::doc::doc_write_verbs();
    catalog
        .commands
        .iter()
        .filter(|(_, command)| {
            if command.command != "jigc" {
                return false;
            }
            let mut literals = command.args.iter().filter_map(|arg| match arg {
                engine::compose::CommandArg::Literal { literal } => Some(literal.as_str()),
                _ => None,
            });
            literals.next() == Some("doc")
                && literals
                    .next()
                    .is_some_and(|verb| write_verbs.iter().any(|write| write == verb))
        })
        .map(|(id, _)| id.clone())
        .collect()
}

/// Every lone-line `{{ cli.<id> }}` ref of a step body — the compose seam's own
/// class rule (a `{{cli.…}}` renders only as a whole line), re-read here rather
/// than borrowed from the fence.
fn cli_refs(body: &str) -> Vec<String> {
    body.lines()
        .filter_map(|line| {
            let inner = line.trim().strip_prefix("{{")?.strip_suffix("}}")?.trim();
            let id = inner.strip_prefix("cli.")?.trim();
            (!id.is_empty() && !id.contains(char::is_whitespace)).then(|| id.to_owned())
        })
        .collect()
}

/// Every `{{schema:<T>}}` ref of a step body — the derivation's second signal (a
/// migrate author template solicits its whole write through the projection).
fn schema_refs(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = body;
    while let Some(open) = rest.find("{{") {
        rest = &rest[open + 2..];
        let Some(close) = rest.find("}}") else { break };
        if let Some(ty) = rest[..close].trim().strip_prefix("schema:") {
            out.push(ty.trim().to_owned());
        }
        rest = &rest[close + 2..];
    }
    out
}

/// The two arms of the derivation, kept apart so the union can be proven to be a
/// real union (each arm contributes a member the other does not reach).
fn owe_set_arms(pack: &Path) -> (BTreeSet<String>, BTreeSet<String>) {
    let writes = doc_write_command_ids(pack);
    let mut by_command = BTreeSet::new();
    let mut by_schema = BTreeSet::new();
    for (id, def) in step_defs(pack) {
        if cli_refs(&def.body).iter().any(|r| writes.contains(r)) {
            by_command.insert(id.clone());
        }
        if !schema_refs(&def.body).is_empty() {
            by_schema.insert(id);
        }
    }
    (by_command, by_schema)
}

/// The full owe-set: every step of the tree that solicits a managed-doc write.
fn owe_set(pack: &Path) -> BTreeSet<String> {
    let (by_command, by_schema) = owe_set_arms(pack);
    by_command.union(&by_schema).cloned().collect()
}

/// The tree's `read.staged-read-back` declarers, read off the shipped front-matter.
fn declarers(pack: &Path) -> BTreeSet<String> {
    step_defs(pack)
        .into_iter()
        .filter(|(_, def)| {
            def.states_constraints
                .iter()
                .any(|code| code == cli::pack::STAGED_READ_BACK_CODE)
        })
        .map(|(id, _)| id)
        .collect()
}

/// Withdraw ONE declared code from a copied step's `states-constraints:` line,
/// leaving its siblings standing — the mutation the fence must catch, and narrower
/// than emptying the list (a step declaring `create.singleton-copy-in` too must
/// fail on *this* fence, not on that one).
fn withdraw_code(pack: &Path, step: &str, code: &str) {
    let path = pack.join("steps").join(format!("{step}.yaml"));
    let text = fs::read_to_string(&path).expect("read the copied step");
    let mut out = String::new();
    let mut hit = false;
    for line in text.lines() {
        if let Some(list) = line.strip_prefix("states-constraints:")
            && list.contains(code)
        {
            let kept: Vec<&str> = list
                .trim()
                .trim_start_matches('[')
                .trim_end_matches(']')
                .split(',')
                .map(str::trim)
                .filter(|member| !member.is_empty() && *member != code)
                .collect();
            out.push_str(&format!("states-constraints: [{}]\n", kept.join(", ")));
            hit = true;
            continue;
        }
        out.push_str(line);
        out.push('\n');
    }
    assert!(
        hit,
        "the shipped `{step}` step must declare `{code}` for its withdrawal to be a real mutation",
    );
    fs::write(&path, out).expect("write the mutated step");
}

/// Drive the whole derived owe-set through the real binary: withdraw each member's
/// declaration in turn, assert the load blocks naming step + code, restore, repeat.
fn drive_read_back_axis(pack: &Path, run: &dyn Fn() -> std::process::Output) {
    let owed = owe_set(pack);
    assert!(
        !owed.is_empty(),
        "this pack must solicit at least one managed-doc write for the axis to sweep",
    );

    for step in owed {
        let path = pack.join("steps").join(format!("{step}.yaml"));
        let original = fs::read_to_string(&path).expect("read the step");
        withdraw_code(pack, &step, cli::pack::STAGED_READ_BACK_CODE);
        let out = run();
        let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
        let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
        fs::write(&path, &original).expect("restore the step");

        assert!(
            !out.status.success(),
            "withdrawing `{}` from the write-soliciting step `{step}` must block pack load; \
             stdout:\n{stdout}\nstderr:\n{stderr}",
            cli::pack::STAGED_READ_BACK_CODE,
        );
        assert!(
            stderr.contains(&format!("`{step}`")),
            "stderr must name the `{step}` step; got:\n{stderr}",
        );
        assert!(
            stderr.contains(cli::pack::STAGED_READ_BACK_CODE),
            "stderr must name the `{}` code; got:\n{stderr}",
            cli::pack::STAGED_READ_BACK_CODE,
        );
    }
}

/// The derivation, dev pack: the owe-set computed from the tree's own catalog +
/// steps is **exactly** the set of steps declaring the code. Equality both ways —
/// a step that starts soliciting a write joins the owe-set and must declare;
/// a declaration on a step that solicits nothing would be a receipt for a
/// statement no write needs.
#[test]
fn the_dev_owe_set_is_exactly_its_read_back_declarers() {
    let pack = embedded_pack_tree();
    let (by_command, by_schema) = owe_set_arms(&pack);
    assert!(
        by_command.difference(&by_schema).next().is_some(),
        "the `{{cli.<id>}}` arm must reach a step the schema arm does not, or the union is \
         not a union; by-command {by_command:?}, by-schema {by_schema:?}",
    );
    assert!(
        by_schema.difference(&by_command).next().is_some(),
        "the `{{schema:<T>}}` arm must reach a step the command arm does not; \
         by-command {by_command:?}, by-schema {by_schema:?}",
    );
    assert_eq!(
        owe_set(&pack),
        declarers(&pack),
        "every dev step that solicits a managed-doc write must declare `{}` — and only those",
        cli::pack::STAGED_READ_BACK_CODE,
    );
}

/// The derivation, methodology pack: same equality over the second shipped tree,
/// whose migrate author templates are reached only by the schema arm.
#[test]
fn the_methodology_owe_set_is_exactly_its_read_back_declarers() {
    let pack = methodology_pack_tree();
    let (by_command, by_schema) = owe_set_arms(&pack);
    assert!(
        by_command.difference(&by_schema).next().is_some()
            && by_schema.difference(&by_command).next().is_some(),
        "both arms must contribute a member the other does not reach; \
         by-command {by_command:?}, by-schema {by_schema:?}",
    );
    assert_eq!(
        owe_set(&pack),
        declarers(&pack),
        "every methodology step that solicits a managed-doc write must declare `{}` — and only \
         those",
        cli::pack::STAGED_READ_BACK_CODE,
    );
}

/// The dev-pack seam: every member of the derived owe-set, its declaration
/// withdrawn in turn from a `JIGC_PACK_DIR` copy, blocks the composing binary at
/// pack load.
#[test]
fn every_write_soliciting_dev_step_is_fenced_at_pack_load() {
    let repo = TempDir::new("dev-repo");
    let home = TempDir::new("dev-home");
    let pack = pack_copy("dev", &embedded_pack_tree());
    init_repo(repo.path());

    let clean = run_with_pack(repo.path(), home.path(), pack.path(), PREVIEW);
    assert!(
        clean.status.success(),
        "the unmutated dev copy must load clean, so every failure below is the mutation's; \
         stderr:\n{}",
        String::from_utf8_lossy(&clean.stderr),
    );

    drive_read_back_axis(pack.path(), &|| {
        run_with_pack(repo.path(), home.path(), pack.path(), PREVIEW)
    });
}

/// The methodology seam: the same axis over the on-disk methodology tree through
/// the `packs.yaml`-listed pack path — each manifest-shipping constituent is fenced
/// in isolation, so the dev pack's intact declarations in the same composition buy
/// the methodology copy nothing.
#[test]
fn every_write_soliciting_methodology_step_is_fenced_at_pack_load() {
    let repo = TempDir::new("meth-repo");
    let home = TempDir::new("meth-home");
    let pack = pack_copy("meth", &methodology_pack_tree());
    init_repo(repo.path());
    list_pack(repo.path(), pack.path());

    let clean = run_embedded(repo.path(), home.path(), PREVIEW);
    assert!(
        clean.status.success(),
        "the unmutated methodology copy must load clean; stderr:\n{}",
        String::from_utf8_lossy(&clean.stderr),
    );

    drive_read_back_axis(pack.path(), &|| {
        run_embedded(repo.path(), home.path(), PREVIEW)
    });
}

/// The controls: an unmutated dev copy, the bare embedded base, and the composed
/// `[dev ▸ methodology]` pair all load clean — the fence is inert on every
/// production composition.
#[test]
fn the_shipped_compositions_all_load_clean() {
    let repo = TempDir::new("clean-repo");
    let home = TempDir::new("clean-home");
    let pack = pack_copy("clean", &embedded_pack_tree());
    init_repo(repo.path());
    let out = run_with_pack(repo.path(), home.path(), pack.path(), PREVIEW);
    assert!(
        out.status.success(),
        "an unmutated dev-pack copy must load clean; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );

    let repo = TempDir::new("emb-repo");
    let home = TempDir::new("emb-home");
    init_repo(repo.path());
    let out = run_embedded(repo.path(), home.path(), PREVIEW);
    assert!(
        out.status.success(),
        "the embedded base must load clean; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );

    let repo = TempDir::new("pair-repo");
    let home = TempDir::new("pair-home");
    init_repo(repo.path());
    fs::write(
        repo.path().join(".jigc").join("config").join("packs.yaml"),
        "compose-embedded-methodology: true\n",
    )
    .expect("write the compose marker");
    let out = run_embedded(repo.path(), home.path(), PREVIEW);
    assert!(
        out.status.success(),
        "the composed [dev ▸ methodology] pair must load clean; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
}

/// The omitting context: a **manifest-less** pack is outside the read-back fence as
/// it is outside every pack-load fence (the `assert_schema_freeze` opt-in
/// precedent) — the same withdrawn declaration the manifest-bearing copy blocks on
/// loads clean once `config/schema-manifest.yaml` is dropped. Never an error: a
/// seeded / project-local pack that ships no manifest stays on skip-on-absent.
#[test]
fn a_manifest_less_pack_is_unchecked() {
    let repo = TempDir::new("nm-repo");
    let home = TempDir::new("nm-home");
    let pack = pack_copy("nm", &embedded_pack_tree());
    init_repo(repo.path());
    let step = owe_set(pack.path())
        .into_iter()
        .next()
        .expect("the dev pack solicits at least one managed-doc write");
    withdraw_code(pack.path(), &step, cli::pack::STAGED_READ_BACK_CODE);
    fs::remove_file(pack.path().join("config").join("schema-manifest.yaml"))
        .expect("drop the freeze manifest");

    let out = run_with_pack(repo.path(), home.path(), pack.path(), PREVIEW);
    assert!(
        out.status.success(),
        "a manifest-less pack must be outside the read-back fence; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
}
