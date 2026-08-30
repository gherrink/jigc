//! Real-binary acceptance for the M33 **pack-load freeze gate** (T3): a
//! `JIGC_PACK_DIR` on-disk dev-pack copy whose schema *shape* drifted from the
//! shipped `config/schema-manifest.yaml` without a version bump makes a `jigc`
//! command exit **non-zero**, naming the schema-hash mismatch; an unmutated copy
//! composes clean; a manifest-less copy is unaffected (the same drift passes once
//! the freeze manifest is dropped).
//!
//! This drives the **real `jigc` binary** end-to-end — the emitted exit code +
//! stderr are the contract, not a reconstructed equivalent. It is the runtime
//! sibling of the build-time freeze gate
//! (`pack::shipped_schema_manifest_matches_the_frozen_doctype_set`); the design is
//! `design/corpus-migration.md` → The enforcement gate fires at pack-load
//! (review Finding 3) + `design/worked-examples.md` → the freeze-enforcement flow.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-freeze-{tag}-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        ));
        fs::create_dir_all(&path).expect("create temp dir");
        TempDir(path)
    }

    /// A throwaway directory **whose path carries shell metacharacters** — one space
    /// and one apostrophe, the two bytes that break an unquoted and a naively
    /// single-quoted command line respectively.
    ///
    /// Every route this file follows is `sh -c`'d verbatim, so a route that
    /// interpolates a repo-relative-to-absolute path raw runs *correctly* under
    /// [`TempDir::new`] (whose name is drawn from `std::env::temp_dir()` plus an
    /// inert `jigc-freeze-<tag>-<pid>-<nanos>` segment — never a space) and
    /// *silently wrong* here. The fixture axis is the point: an assertion that "the
    /// emitted route must run" certifies only the path shapes it is handed.
    fn new_metachar(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc freeze's {tag}-{}-{:?}",
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

/// The embedded dev pack tree — the faithful source the on-disk copy mirrors.
fn embedded_pack_tree() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("pack")
}

/// The on-disk methodology pack home (`<root>/packs/methodology`) — the faithful
/// source the methodology-arm copies mirror.
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

/// Copy the dev pack into a fresh temp dir and return the copy's root.
fn dev_pack_copy(tag: &str) -> TempDir {
    let dir = TempDir::new(tag);
    copy_tree(&embedded_pack_tree(), dir.path());
    dir
}

/// Mutate the copied `adr` schema's **shape** (its `location:`) — a hash-affecting
/// change that bumps no `schema-version`, the un-migrated schema change the freeze
/// forbids. The manifest is deliberately left unbumped.
fn drift_adr_schema(pack: &Path) {
    let schema = pack.join("schemas").join("adr.yaml");
    let body = fs::read_to_string(&schema).expect("read the copied adr.yaml");
    let drifted = body.replacen("location: decisions/", "location: adr-records/", 1);
    assert_ne!(
        body, drifted,
        "adr.yaml must declare `location: decisions/`"
    );
    fs::write(&schema, drifted).expect("write the drifted adr.yaml");
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

/// Run `jigc start --workflow single-task "<intent>"` with `cwd = repo`,
/// `$HOME = home`, `JIGC_PACK_DIR = pack`.
fn run_start(repo: &Path, home: &Path, pack: &Path) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(["start", "--workflow", "single-task", "freeze-gate probe"])
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", pack)
        .output()
        .expect("spawn the jigc binary")
}

/// A schema-shape change with no manifest bump is **blocked at pack-load**: the
/// composing `jigc start` exits non-zero, and stderr names the schema-hash
/// mismatch on the drifted doctype.
#[test]
fn schema_shape_drift_without_manifest_bump_is_blocked() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = dev_pack_copy("drift");
    init_repo(repo.path());
    drift_adr_schema(pack.path());

    let out = run_start(repo.path(), home.path(), pack.path());
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !out.status.success(),
        "a drifted schema must make `jigc start` exit non-zero; stdout:\n{}\nstderr:\n{stderr}",
        String::from_utf8_lossy(&out.stdout),
    );
    assert!(
        stderr.contains("schema-hash mismatch"),
        "stderr must name the schema-hash mismatch; got:\n{stderr}",
    );
    assert!(
        stderr.contains("adr"),
        "stderr must name the drifted `adr` doctype; got:\n{stderr}",
    );
}

/// The control: an **unmutated** dev-pack copy matches its shipped manifest, so the
/// freeze gate is inert and `jigc start` composes clean.
#[test]
fn unmutated_pack_copy_composes_clean() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = dev_pack_copy("clean");
    init_repo(repo.path());

    let out = run_start(repo.path(), home.path(), pack.path());
    assert!(
        out.status.success(),
        "an unmutated dev-pack copy must compose clean; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
}

/// Copy the methodology pack into a fresh temp dir and return the copy's root.
fn methodology_pack_copy(tag: &str) -> TempDir {
    let dir = TempDir::new(tag);
    copy_tree(&methodology_pack_tree(), dir.path());
    dir
}

/// Mutate the copied `research` schema's **shape** (its `location:`) — a hash-affecting
/// change that bumps no `schema-version`. The methodology manifest is deliberately
/// left unbumped. Mirrors the dev-pack sibling [`drift_adr_schema`] exactly.
///
/// **This used to reword a slot `hint:`** — the conflation M47's presentation projection
/// ends: it called a prose reword "a schema-shape change" while the dev arm it was
/// modelled on drifted `location:`, and once prose left the hash it stopped drifting
/// anything at all. The prose side now has its own arm
/// ([`a_reworded_methodology_hint_composes_clean_with_no_bump`]).
fn drift_research_schema(pack: &Path) {
    let schema = pack.join("schemas").join("research.yaml");
    let body = fs::read_to_string(&schema).expect("read the copied research.yaml");
    // Anchored on the line-start key: `research.yaml` *mentions* `location: research/`
    // inside its header comment first, and an unanchored `replacen` rewrote the comment
    // and left the schema untouched — a drift function that drifts nothing (caught by
    // the gate, which is what the gate is for).
    let drifted = body.replacen(
        "\nlocation: research/\n",
        "\nlocation: investigations/\n",
        1,
    );
    assert_ne!(
        body, drifted,
        "research.yaml must declare `location: research/` as a top-level key"
    );
    fs::write(&schema, drifted).expect("write the drifted research.yaml");
}

/// Reword the copied `research` schema's `question` slot **hint** — authored prose, and
/// since M47 **outside** the frozen `schema-hash` ([`engine::manifest::schema_hash`] hashes
/// the presentation projection). No bump, no re-pin, and the gate must stay silent.
fn reword_research_hint(pack: &Path) {
    let schema = pack.join("schemas").join("research.yaml");
    let body = fs::read_to_string(&schema).expect("read the copied research.yaml");
    let reworded = body.replacen(
        "The question this research set out to answer.",
        "The question this research set out to resolve.",
        1,
    );
    assert_ne!(body, reworded, "research.yaml must carry the question hint");
    fs::write(&schema, reworded).expect("write the reworded research.yaml");
}

/// Compose `[dev ▸ methodology-copy]` via the listed-pack mechanism: `packs.yaml`
/// names the on-disk copy over the embedded dev base. `JIGC_PACK_DIR` must stay
/// unset so the embedded base (not an env pack) anchors the composition.
fn list_pack(repo: &Path, pack: &Path) {
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", pack.display()),
    )
    .expect("write packs.yaml naming the methodology copy");
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, and NO `JIGC_PACK_DIR`
/// (the listed-pack composition path — the pack rides in `packs.yaml`).
fn run_listed(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("spawn the jigc binary")
}

/// The M40 A1 headline: a **methodology** schema-shape change with no manifest bump
/// is **blocked at pack-load** — the methodology pack now ships its own
/// `config/schema-manifest.yaml`, so it is freeze-enforced exactly like the dev
/// pack (`design/corpus-migration.md` → M40 revises the dichotomy). The composing
/// `jigc start` exits non-zero and stderr names the schema-hash mismatch on the
/// drifted `research` doctype.
#[test]
fn methodology_schema_shape_drift_without_manifest_bump_is_blocked() {
    let repo = TempDir::new("m-repo");
    let home = TempDir::new("m-home");
    let pack = methodology_pack_copy("m-drift");
    init_repo(repo.path());
    list_pack(repo.path(), pack.path());
    drift_research_schema(pack.path());

    let out = run_listed(
        repo.path(),
        home.path(),
        &["start", "--workflow", "single-task", "freeze-gate probe"],
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !out.status.success(),
        "a drifted methodology schema must make `jigc start` exit non-zero; stdout:\n{}\nstderr:\n{stderr}",
        String::from_utf8_lossy(&out.stdout),
    );
    assert!(
        stderr.contains("schema-hash mismatch"),
        "stderr must name the schema-hash mismatch; got:\n{stderr}",
    );
    assert!(
        stderr.contains("research"),
        "stderr must name the drifted `research` doctype; got:\n{stderr}",
    );
}

/// The **invariance arm beside it** (M47 Decision 3), through the real binary: the very
/// mutation [`drift_research_schema`] used to make — a slot-`hint` reword — now composes
/// **clean** against an unbumped, un-re-pinned methodology manifest. While prose sat
/// inside the `schema-hash` the freeze was **unfenceable**: a hint reword and a real
/// structural change were the same keystroke (re-pin, green), guarded only by a comment.
/// The two arms together are the discrimination the projection buys.
#[test]
fn a_reworded_methodology_hint_composes_clean_with_no_bump() {
    let repo = TempDir::new("m-prose-repo");
    let home = TempDir::new("m-prose-home");
    let pack = methodology_pack_copy("m-prose");
    init_repo(repo.path());
    list_pack(repo.path(), pack.path());
    reword_research_hint(pack.path());

    let out = run_listed(
        repo.path(),
        home.path(),
        &["start", "--workflow", "single-task", "freeze-gate probe"],
    );
    assert!(
        out.status.success(),
        "a reworded authoring hint must need no bump and no re-pin; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// The stamp goes live with the manifest (one atomic unit): a **fresh methodology
/// mint** through the real `do-research` workflow carries `schema-version: 1` in
/// its committed front matter — the value the validate side's version-aware
/// routing demands (`design/corpus-migration.md` → The schema-version stamp).
#[test]
fn fresh_methodology_mint_carries_schema_version_1() {
    let repo = TempDir::new("mint-repo");
    let home = TempDir::new("mint-home");
    init_repo(repo.path());
    list_pack(repo.path(), &methodology_pack_tree());

    // Mint a research doc through the real workflow: start → create → author →
    // fill the commit doc → finalize.
    let start = run_listed(
        repo.path(),
        home.path(),
        &["start", "--workflow", "do-research", "study the cache"],
    );
    assert!(
        start.status.success(),
        "`jigc start --workflow do-research` must succeed; stderr:\n{}",
        String::from_utf8_lossy(&start.stderr),
    );
    let run = |args: &[&str]| {
        let out = run_listed(repo.path(), home.path(), args);
        assert!(
            out.status.success(),
            "jigc {args:?} must succeed; stdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        );
    };
    let set_slot = |addr: &str, prose: &[u8]| {
        use std::io::Write;
        let mut child = Command::new(env!("CARGO_BIN_EXE_jigc"))
            .args(["doc", "set-slot", addr, "--from-file", "-"])
            .current_dir(repo.path())
            .env("HOME", home.path())
            .env_remove("JIGC_PACK_DIR")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .expect("spawn jigc set-slot");
        child
            .stdin
            .take()
            .expect("stdin piped")
            .write_all(prose)
            .expect("write stdin");
        let out = child.wait_with_output().expect("wait for jigc");
        assert!(
            out.status.success(),
            "set-slot {addr} must succeed; stdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        );
    };
    run(&["doc", "create", "research", "--title", "Cache Study"]);
    for slot in ["question", "findings", "sources"] {
        set_slot(&format!("research:cache-study#{slot}"), b"Some prose.\n");
    }
    let task = "study-the-cache";
    run(&[
        "doc",
        "set-field",
        &format!("commit:{task}#type"),
        "--value",
        "docs",
    ]);
    set_slot(&format!("commit:{task}#summary"), b"record the study\n");
    run(&["task", "finalize", task]);

    let committed = fs::read_to_string(repo.path().join("research").join("cache-study.md"))
        .expect("the finalized research doc is committed at research/cache-study.md");
    assert!(
        committed.contains("schema-version: 1"),
        "a fresh methodology mint must carry `schema-version: 1` in its front matter; got:\n{committed}",
    );
}

/// Compose `[drifted-dev-copy ▸ methodology ▸ embedded-dev]` via the listed-pack
/// mechanism — the production shape a real project uses (`packs.yaml` names pack
/// **directories**, the trigger that discharged the M33 deferral). The methodology
/// pack rides along so the `milestone-record` doctype resolves and `jigc milestone
/// create` reaches its committing write path.
fn list_packs(repo: &Path, packs: &[&Path]) {
    let mut body = String::from("packs:\n");
    for pack in packs {
        body.push_str(&format!("  - {}\n", pack.display()));
    }
    fs::write(repo.join(".jigc").join("config").join("packs.yaml"), body)
        .expect("write packs.yaml naming the listed packs");
}

/// The door enumeration every pack-load fence arm sweeps — the read/report verbs
/// plus the orient front door. Factored into one constant so a *new* fence arm
/// cannot silently sweep a narrower set than the one before it: the M42 Inc 6
/// headline is that the freeze assert lives in the pack-source factory, so **every**
/// door is the claim, and each arm below iterates this same list rather than
/// re-typing its own.
const FREEZE_DOORS: [&[&str]; 5] = [
    &["validate"],
    &["describe"],
    &["doc", "schema", "adr"],
    &["migrate-corpus"],
    // The orient front door (bare `jigc start` — no workflow, no intent): distinct
    // from the composing `jigc start --workflow …` the assert already guarded.
    &["start"],
];

/// A repo + home + listed **drifted** dev-pack copy (with the methodology pack
/// composed for the `milestone-record` doctype). Every door run against it must
/// block, naming [`needles`](DriftedProject::needles).
struct DriftedProject {
    repo: TempDir,
    home: TempDir,
    _pack: TempDir,
    /// The substrings every blocked door's stderr must carry — what the operator
    /// reading the failure is owed: *what* drifted and *how to fix it*.
    needles: &'static [&'static str],
}

impl DriftedProject {
    /// The schema-shape drift (a frozen `adr` whose `location:` moved with no
    /// version bump).
    fn new(tag: &str) -> Self {
        Self::with(tag, drift_adr_schema, &["schema-hash mismatch", "adr"])
    }

    /// A listed dev-pack copy mutated by `drift`; every door must then block naming
    /// `needles`.
    fn with(tag: &str, drift: fn(&Path), needles: &'static [&'static str]) -> Self {
        let repo = TempDir::new(&format!("{tag}-repo"));
        let home = TempDir::new(&format!("{tag}-home"));
        let pack = dev_pack_copy(&format!("{tag}-pack"));
        init_repo(repo.path());
        list_packs(repo.path(), &[pack.path(), &methodology_pack_tree()]);
        drift(pack.path());
        DriftedProject {
            repo,
            home,
            _pack: pack,
            needles,
        }
    }

    /// Run one door and assert it exits non-zero naming the drift — the emitted exit
    /// code + stderr are the contract.
    fn door_blocks(&self, args: &[&str]) -> std::process::Output {
        let out = run_listed(self.repo.path(), self.home.path(), args);
        let stderr = String::from_utf8_lossy(&out.stderr);
        assert!(
            !out.status.success(),
            "`jigc {}` must exit non-zero on a drifted frozen pack; stdout:\n{}\nstderr:\n{stderr}",
            args.join(" "),
            String::from_utf8_lossy(&out.stdout),
        );
        for needle in self.needles {
            assert!(
                stderr.contains(needle),
                "`jigc {}` stderr must name {needle:?}; got:\n{stderr}",
                args.join(" "),
            );
        }
        out
    }
}

/// The M42 Inc 6 headline: the freeze assert lives in the **pack-source factory**, so
/// a drifted frozen schema blocks **every** door — not just the compose front door it
/// used to guard. The read/report verbs sailed past it before this task
/// (`implementation/decisions-pending.md` → the discharged M33 deferral: the trigger
/// "packs are ever loaded from the filesystem in production" fired at M14).
#[test]
fn every_read_door_blocks_on_a_drifted_frozen_schema() {
    let project = DriftedProject::new("doors");

    for door in FREEZE_DOORS {
        project.door_blocks(door);
    }
}

/// Rewrite the copied manifest's declared `slug-rule.hash` to a well-formed but
/// **wrong** digest — the identity-mint sibling of [`drift_adr_schema`]: it stands in
/// for a change to `engine::slug::slugify` (the function that mints every doc slug,
/// task id, and `{#id}` anchor) shipped without the declared `slug-rule-version` bump.
/// Mutating the *manifest* rather than the engine is what makes the drift reachable
/// from a test at all — the pack is data, the rule is compiled in — and it drives the
/// same `check` arm from the same side the gate reads.
fn drift_slug_rule_hash(pack: &Path) {
    let path = pack.join("config").join("schema-manifest.yaml");
    let body = fs::read_to_string(&path).expect("read the copied schema-manifest.yaml");
    let mut out = String::new();
    let mut hit = false;
    for line in body.lines() {
        if !hit && line.starts_with("  hash: ") {
            out.push_str(&format!("  hash: {}", "0".repeat(64)));
            hit = true;
        } else {
            out.push_str(line);
        }
        out.push('\n');
    }
    assert!(
        hit,
        "the shipped manifest must declare a `slug-rule.hash` to drift"
    );
    fs::write(&path, out).expect("write the drifted schema-manifest.yaml");
}

/// The M42 Inc 10 T1 headline — **the fence**: `slugify` is jigc's identity-derivation
/// function (every doc slug, task/milestone id, and `{#id}` anchor) and it sits in no
/// `schema-hash` and no manifest, so the gate that blocks renaming a *field* waved
/// through a change to the function that *names every id in every corpus* — a change
/// **no migration can repair** (no transform kind re-mints an id). With the rule
/// declared (`slug-rule: { version, hash }`), a manifest whose declared fingerprint no
/// longer matches the shipped rule blocks **every** door, naming the recomputed hash
/// and routing to the declared bump (`design/storage.md` → Identity → *The slug rule is
/// itself a versioned rule (M42)*).
#[test]
fn a_drifted_slug_rule_blocks_every_door() {
    let project = DriftedProject::with(
        "slugrule",
        drift_slug_rule_hash,
        &["the slug rule changed", "bump slug-rule-version + re-pin"],
    );

    for door in FREEZE_DOORS {
        project.door_blocks(door);
    }
}

/// **Delete** the copied manifest's whole `slug-rule:` block — the third silencer,
/// and the cheapest one: [`drift_slug_rule_hash`]'s route says *re-pin the hash*, but
/// an author (or an agent) who instead removes three lines got the gate to go quiet
/// entirely. A pack opts out of the freeze by shipping **no manifest** (a visible,
/// wholesale act — [`manifest_less_pack_is_unaffected`]); a manifest that freezes
/// doctype *shapes* may not quietly decline to declare the rule the ids inside them
/// are *minted* by.
fn delete_slug_rule_block(pack: &Path) {
    let path = pack.join("config").join("schema-manifest.yaml");
    let body = fs::read_to_string(&path).expect("read the copied schema-manifest.yaml");
    let mut out = String::new();
    let mut dropping = false;
    let mut hit = false;
    for line in body.lines() {
        if line.starts_with("slug-rule:") {
            dropping = true;
            hit = true;
            continue;
        }
        // The block's own body is indented; the next unindented key ends it.
        if dropping {
            if line.starts_with(' ') {
                continue;
            }
            dropping = false;
        }
        out.push_str(line);
        out.push('\n');
    }
    assert!(
        hit,
        "the shipped manifest must declare a `slug-rule:` block to delete"
    );
    assert!(
        !out.lines().any(|line| line.starts_with("slug-rule:")),
        "the declared block must be gone from the rewritten manifest; got:\n{out}"
    );
    fs::write(&path, out).expect("write the slug-rule-less schema-manifest.yaml");
}

/// The M42 completion-audit LOW: a manifest that **omits** `slug-rule:` asserted
/// nothing, so the fence [`a_drifted_slug_rule_blocks_every_door`] builds could be
/// silenced by deleting it — and a third-party / project pack authored without the
/// key inherited **no fence at all** over the one change no migration can repair.
/// "Absent means unchecked" is the exact shape M42 has been bitten by repeatedly: a
/// fence that only fires on the members it happens to know about is not a fence
/// (`DECISIONS.md` → 2026-07-14, *a census cannot enforce a predicate*). Every door
/// must block, and the route must hand the author the block to paste.
#[test]
fn a_manifest_omitting_the_slug_rule_blocks_every_door() {
    let project = DriftedProject::with(
        "slugless",
        delete_slug_rule_block,
        &["declares no `slug-rule:` block", "slug-rule:"],
    );

    for door in FREEZE_DOORS {
        project.door_blocks(door);
    }
}

/// The control (the omitting context's twin): an **unmutated** listed pack — whose
/// manifests declare the slug rule the engine really ships — runs every one of those
/// doors clean. Without this arm, a fence that blocked *unconditionally* would pass the
/// test above; it is also the standing proof that both shipped manifests' `slug-rule`
/// blocks stay in sync with `engine::slug::rule_fingerprint()`.
#[test]
fn an_unmutated_listed_pack_passes_the_slug_rule_gate() {
    let repo = TempDir::new("slugclean-repo");
    let home = TempDir::new("slugclean-home");
    let pack = dev_pack_copy("slugclean-pack");
    init_repo(repo.path());
    list_packs(repo.path(), &[pack.path(), &methodology_pack_tree()]);

    for args in [
        vec!["validate"],
        vec!["describe"],
        vec!["doc", "schema", "adr"],
        vec!["start"],
    ] {
        let out = run_listed(repo.path(), home.path(), &args);
        assert!(
            out.status.success(),
            "`jigc {}` must run clean against an unmutated pack; stdout:\n{}\nstderr:\n{}",
            args.join(" "),
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        );
    }
}

/// The **committing** door: `jigc milestone create` wrote and committed a brand-new
/// milestone record at exit 0 over a drifted frozen schema. It must block — and land
/// **no** record file and **no** record commit (the write is what makes this door the
/// dangerous one; Inc 7 builds a second write verb on it).
#[test]
fn the_committing_milestone_door_blocks_and_writes_nothing() {
    let project = DriftedProject::new("mcreate");

    project.door_blocks(&["milestone", "create", "drift probe"]);

    let records = project.repo.path().join("docs").join("milestone-records");
    let landed: Vec<PathBuf> = fs::read_dir(&records)
        .map(|dir| {
            dir.filter_map(|e| e.ok())
                .map(|e| e.path())
                .filter(|p| p.extension().is_some_and(|ext| ext == "md"))
                .collect()
        })
        .unwrap_or_default();
    assert!(
        landed.is_empty(),
        "a blocked `milestone create` must land no milestone-record file; found: {landed:?}",
    );

    let log = Command::new("git")
        .args(["log", "--oneline"])
        .current_dir(project.repo.path())
        .output()
        .expect("run git log");
    let log = String::from_utf8_lossy(&log.stdout);
    assert!(
        !log.contains("chore(milestone): open record"),
        "a blocked `milestone create` must land no record commit; git log:\n{log}",
    );
}

/// Run the **real** `jigc setup` — the only production install path (QUICKSTART) — in
/// `repo`. It writes `compose-embedded-methodology: true` into
/// `.jigc/config/packs.yaml` (`crates/cli/src/setup.rs` → step 2b).
fn run_setup(repo: &Path, home: &Path) -> std::process::Output {
    run_listed(repo, home, &["setup"])
}

/// The topology **`jigc setup` actually creates**: the setup-written compose marker
/// *plus* an operator-listed filesystem pack (the `packs:` list
/// `design/multi-pack.md` → The pack-set documents). The listed pack used to be
/// **silently dropped**, which left the freeze gate nothing to check: over a drifted
/// frozen `adr` every door, including the **committing** `jigc milestone create`, ran
/// at exit 0 and landed a record commit. M42 stopped the drop with a blanket refusal of
/// the whole combination; **M49 Increment 6 composes it instead**
/// (`[listed… ▸ dev ▸ methodology]` — `crates/cli/tests/project_pack_composition.rs`),
/// so the listed pack is now genuinely **loaded and checked**: its drifted `adr` is
/// named by the pack-load freeze gate at every door.
///
/// The claim this test pins is unchanged and is the one its name states — *a listed
/// pack is never silently dropped* — and it is now pinned to the **truthful**
/// diagnosis: the doors block naming the doctype that actually drifted, rather than
/// naming a pack-set combination the loader no longer refuses. The harm arm is
/// unchanged: the committing door lands no record file and no record commit.
#[test]
fn the_setup_written_marker_never_silently_drops_a_listed_pack() {
    let repo = TempDir::new("marker-repo");
    let home = TempDir::new("marker-home");
    let pack = dev_pack_copy("marker-pack");
    init_repo(repo.path());

    // The production install writes the marker — assert it verbatim, so this arm is
    // pinned to what setup really emits, not to a reconstruction of it.
    let setup = run_setup(repo.path(), home.path());
    assert!(
        setup.status.success(),
        "`jigc setup` must succeed; stderr:\n{}",
        String::from_utf8_lossy(&setup.stderr),
    );
    let packs_yaml = repo.path().join(".jigc").join("config").join("packs.yaml");
    let marker = fs::read_to_string(&packs_yaml).expect("read the setup-written packs.yaml");
    assert!(
        marker.contains("compose-embedded-methodology: true"),
        "`jigc setup` must write the compose marker; got:\n{marker}",
    );

    // The operator then lists their own pack alongside setup's marker bytes (verbatim),
    // and that pack's frozen `adr` schema has drifted with no manifest bump.
    drift_adr_schema(pack.path());
    fs::write(
        &packs_yaml,
        format!("packs:\n  - {}\n{marker}", pack.path().display()),
    )
    .expect("list the operator's pack alongside the setup-written marker");

    for args in [
        vec!["validate"],
        vec!["describe"],
        vec!["doc", "schema", "adr"],
        vec!["migrate-corpus"],
        vec!["start"],
        vec!["start", "--workflow", "single-task", "freeze-gate probe"],
        vec!["milestone", "create", "drift probe"],
    ] {
        let out = run_listed(repo.path(), home.path(), &args);
        let stderr = String::from_utf8_lossy(&out.stderr);
        assert!(
            !out.status.success(),
            "`jigc {}` must exit non-zero over a listed pack whose frozen `adr` has drifted; stdout:\n{}\nstderr:\n{stderr}",
            args.join(" "),
            String::from_utf8_lossy(&out.stdout),
        );
        assert!(
            stderr.contains("pack-load freeze check failed") && stderr.contains("`adr`"),
            "`jigc {}` stderr must name the drifted frozen doctype in the LISTED pack — \
             which is only possible because that pack was actually loaded, not dropped; \
             got:\n{stderr}",
            args.join(" "),
        );
    }

    // The committing door landed nothing — no record file, no record commit.
    let records = repo.path().join("docs").join("milestone-records");
    let landed: Vec<PathBuf> = fs::read_dir(&records)
        .map(|dir| {
            dir.filter_map(|e| e.ok())
                .map(|e| e.path())
                .filter(|p| p.extension().is_some_and(|ext| ext == "md"))
                .collect()
        })
        .unwrap_or_default();
    assert!(
        landed.is_empty(),
        "a blocked `milestone create` must land no milestone-record file; found: {landed:?}",
    );
    let log = Command::new("git")
        .args(["log", "--oneline"])
        .current_dir(repo.path())
        .output()
        .expect("run git log");
    let log = String::from_utf8_lossy(&log.stdout);
    assert!(
        !log.contains("chore(milestone): open record"),
        "a blocked `milestone create` must land no record commit; git log:\n{log}",
    );
}

/// The other order — `jigc setup` run **over** a project already on the M14 listed-pack
/// path. Setup must not wire the compose marker there (the marker would make the
/// operator's declared pack inert), so the listed pack stays loaded and the freeze gate
/// still sees it: the same `adr` drift blocks every door naming the schema-hash
/// mismatch. This is the independent proof the listed pack is genuinely **loaded** —
/// the setup-written marker is what dropped it.
#[test]
fn setup_over_a_listed_pack_leaves_that_pack_loaded_and_freeze_checked() {
    let repo = TempDir::new("m14-repo");
    let home = TempDir::new("m14-home");
    let pack = dev_pack_copy("m14-pack");
    init_repo(repo.path());
    list_packs(repo.path(), &[pack.path(), &methodology_pack_tree()]);

    let setup = run_setup(repo.path(), home.path());
    assert!(
        setup.status.success(),
        "`jigc setup` must succeed over a project that lists packs; stderr:\n{}",
        String::from_utf8_lossy(&setup.stderr),
    );
    let packs_yaml =
        fs::read_to_string(repo.path().join(".jigc").join("config").join("packs.yaml"))
            .expect("read packs.yaml after setup");
    assert!(
        !packs_yaml.contains("compose-embedded-methodology"),
        "`jigc setup` must not wire the compose marker over a listed `packs:` list (it would make the listed packs inert); got:\n{packs_yaml}",
    );
    assert!(
        packs_yaml.contains(&pack.path().display().to_string()),
        "the operator's listed pack must survive setup; got:\n{packs_yaml}",
    );

    // The listed pack is really loaded: drift its frozen `adr` and the gate fires.
    drift_adr_schema(pack.path());
    let out = run_listed(repo.path(), home.path(), &["validate"]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !out.status.success(),
        "the listed pack's drifted frozen schema must block `jigc validate`; stdout:\n{}\nstderr:\n{stderr}",
        String::from_utf8_lossy(&out.stdout),
    );
    assert!(
        stderr.contains("schema-hash mismatch") && stderr.contains("adr"),
        "stderr must name the drifted `adr` doctype's schema-hash mismatch; got:\n{stderr}",
    );
}

/// The omitting context: a **manifest-less** pack is unchecked. The *same* schema
/// drift that the manifest-bearing copy blocks composes clean once
/// `config/schema-manifest.yaml` is dropped — proving the manifest is the gate, and
/// that a seeded / composed pack with no manifest stays inert (never errors).
#[test]
fn manifest_less_pack_is_unaffected() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = dev_pack_copy("nomanifest");
    init_repo(repo.path());
    drift_adr_schema(pack.path());
    fs::remove_file(pack.path().join("config").join("schema-manifest.yaml"))
        .expect("drop the freeze manifest");

    let out = run_start(repo.path(), home.path(), pack.path());
    assert!(
        out.status.success(),
        "a manifest-less pack must be unaffected by the freeze gate; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
}

// ---------------------------------------------------------------------------
// The project layer (M49 Increment 3, T1)
//
// The freeze gate read the **pack** and nothing else, so `.jigc/config/schemas/
// <ty>.yaml` — the whole-file definition shadow `design/overrides.md` documents —
// changed a frozen doctype's shape at every surface while the gate stayed silent:
// the corpus validated clean at exit 0, `jigc doc schema` reported the frozen
// `schema-version` for an unfrozen shape, and `doc create` wrote a third. The fence
// now hashes the **resolved** schema for a project-owned id, so the freeze binds at
// every layer that can change a schema — and the *documented* capability survives
// intact, because the presentation keys are outside the hash by M47's projection.
// ---------------------------------------------------------------------------

/// The shipped `adr` schema's bytes — read from the pack tree, never re-typed, so a
/// shadow built from it differs from the frozen shape in **exactly** the mutation
/// under test (and a schema edit elsewhere cannot leave these arms asserting over a
/// stale copy).
fn shipped_adr_schema() -> String {
    fs::read_to_string(embedded_pack_tree().join("schemas").join("adr.yaml"))
        .expect("read the shipped adr.yaml")
}

/// The shipped `adr` shape with a required `owner` header field added — a schema
/// **shape** change made from the project layer.
fn adr_shadow_with_owner() -> String {
    let body = shipped_adr_schema();
    let shadowed = body.replacen(
        "      - { id: cites-code, type: code-anchor }\n",
        "      - { id: cites-code, type: code-anchor }\n      - { id: owner, type: string }\n",
        1,
    );
    assert_ne!(body, shadowed, "adr.yaml must declare the cites-code field");
    shadowed
}

/// The shipped `adr` shape relocated `decisions/` → `adrs/` — a **home** change,
/// version-gated exactly like a shape change since M38 (`location:` is inside the
/// `schema-hash`).
fn adr_shadow_relocated() -> String {
    let body = shipped_adr_schema();
    let shadowed = body.replacen("\nlocation: decisions/\n", "\nlocation: adrs/\n", 1);
    assert_ne!(
        body, shadowed,
        "adr.yaml must declare `location: decisions/` as a top-level key"
    );
    shadowed
}

/// The reworded `description:` the presentation-only arm asserts is visible through
/// `jigc describe` — authored prose, outside the frozen hash since M47.
const REWORDED_ADR_DESCRIPTION: &str =
    "A dated architectural decision record, in this project's own words.";

/// The shipped `adr` shape with only its authored `description:` reworded — the
/// capability `design/overrides.md` → *Authored metadata on a definition resolves by
/// whole-file shadow* documents, which must keep working.
fn adr_shadow_reworded() -> String {
    let body = shipped_adr_schema();
    let mut out = String::new();
    let mut hit = false;
    for line in body.lines() {
        if !hit && line.starts_with("description: ") {
            out.push_str(&format!("description: {REWORDED_ADR_DESCRIPTION}"));
            hit = true;
        } else {
            out.push_str(line);
        }
        out.push('\n');
    }
    assert!(hit, "adr.yaml must declare a top-level `description:`");
    out
}

/// Write a project-layer whole-file schema shadow at
/// `<repo>/.jigc/config/schemas/<ty>.yaml` and return its path.
fn install_schema_shadow(repo: &Path, ty: &str, body: &str) -> PathBuf {
    let dir = repo.join(".jigc").join("config").join("schemas");
    fs::create_dir_all(&dir).expect("mk .jigc/config/schemas/");
    let path = dir.join(format!("{ty}.yaml"));
    fs::write(&path, body).expect("write the project schema shadow");
    path
}

/// The first backtick-delimited span of a rendered `route:` line — the command an
/// operator would paste.
fn route_command(stderr: &str) -> String {
    let line = stderr
        .lines()
        .find(|l| l.trim_start().starts_with("route:"))
        .unwrap_or_else(|| panic!("the block must render a `route:` line; got:\n{stderr}"));
    let mut parts = line.split('`');
    parts.next();
    parts
        .next()
        .unwrap_or_else(|| panic!("the route must carry a backticked command; got: {line}"))
        .to_owned()
}

/// **The T1 headline** — a project-layer shadow that changes a manifest-governed
/// doctype's *shape* blocks every door, over both shape axes the freeze covers
/// (`design/corpus-migration.md` → The freeze — declared *and* enforced): a **field**
/// added, and the **home** moved (`location:` is inside the `schema-hash` since M38).
///
/// Before this fence the same shadow made `jigc validate` exit **0** over a corpus it
/// had just made non-conformant, while `jigc doc schema adr` reported the frozen
/// `schema-version` for a shape nothing froze. Each door must name *what* drifted
/// (`adr`, the hash mismatch), *where* (the shadow's own path), and *how to fix it* —
/// with a route that runs.
///
/// **Two axes, both iterated.** The shape axis is the freeze's own (`field` × `home`).
/// The second is the *fixture's*: the emitted route interpolates the shadow's absolute
/// path, so its runnability is a property of the **path shape**, not of the drift — and
/// a fixture drawn only from `std::env::temp_dir()` plus an inert segment can never see
/// it. Each shape therefore runs under both an inert path and a
/// [metacharacter-carrying one](TempDir::new_metachar), and the `sh -c <route>` arm
/// below is what distinguishes them.
#[test]
fn a_shape_changing_project_schema_shadow_blocks_every_door() {
    for (shape_tag, body) in [
        ("shadow-owner", adr_shadow_with_owner()),
        ("shadow-home", adr_shadow_relocated()),
    ] {
        for (path_tag, metachar) in [("inert-path", false), ("metachar-path", true)] {
            let body = body.clone();
            let tag = format!("{shape_tag}/{path_tag}");
            let repo = if metachar {
                TempDir::new_metachar(&format!("{shape_tag}-repo"))
            } else {
                TempDir::new(&format!("{shape_tag}-repo"))
            };
            let home = TempDir::new(&format!("{shape_tag}-{path_tag}-home"));
            init_repo(repo.path());
            let shadow = install_schema_shadow(repo.path(), "adr", &body);

            let mut route = String::new();
            for door in FREEZE_DOORS {
                let out = run_listed(repo.path(), home.path(), door);
                let stderr = String::from_utf8_lossy(&out.stderr);
                assert!(
                    !out.status.success(),
                    "[{tag}] `jigc {}` must exit non-zero over a shape-changing project schema shadow; stdout:\n{}\nstderr:\n{stderr}",
                    door.join(" "),
                    String::from_utf8_lossy(&out.stdout),
                );
                for needle in ["schema-hash mismatch", "adr", &shadow.display().to_string()] {
                    assert!(
                        stderr.contains(needle),
                        "[{tag}] `jigc {}` stderr must name {needle:?}; got:\n{stderr}",
                        door.join(" "),
                    );
                }
                route = route_command(&stderr);
            }

            // The route is followed **verbatim**, through a real shell, and it must clear
            // the block — a route that names the wrong file (or no file) reddens here.
            let ran = Command::new("sh")
                .arg("-c")
                .arg(&route)
                .current_dir(repo.path())
                .env("HOME", home.path())
                .output()
                .expect("spawn sh to follow the route");
            assert!(
                ran.status.success(),
                "[{tag}] the emitted route `{route}` must run; stderr:\n{}",
                String::from_utf8_lossy(&ran.stderr),
            );
            for door in FREEZE_DOORS {
                let out = run_listed(repo.path(), home.path(), door);
                assert!(
                    out.status.success(),
                    "[{tag}] `jigc {}` must run clean once the emitted route has been followed; stdout:\n{}\nstderr:\n{}",
                    door.join(" "),
                    String::from_utf8_lossy(&out.stdout),
                    String::from_utf8_lossy(&out.stderr),
                );
            }
        }
    }
}

/// The omitting context's twin, and the reason the fence hashes the **resolved**
/// schema rather than refusing a shadow by name: a shadow that reworks only the
/// *authored prose* changes no frozen byte (M47's presentation projection erases
/// `description:` / `usage:` / slot `hint:`), so it composes clean at exit 0 — and the
/// reworded prose is genuinely live, visible through `jigc describe`. Refusing the
/// shadow by name would have deleted the capability `design/overrides.md` documents
/// for all sixteen shipped doctypes.
#[test]
fn a_presentation_only_project_schema_shadow_composes_clean() {
    let repo = TempDir::new("shadow-prose-repo");
    let home = TempDir::new("shadow-prose-home");
    init_repo(repo.path());
    install_schema_shadow(repo.path(), "adr", &adr_shadow_reworded());

    for door in FREEZE_DOORS {
        let out = run_listed(repo.path(), home.path(), door);
        assert!(
            out.status.success(),
            "`jigc {}` must compose clean over a presentation-only schema shadow; stdout:\n{}\nstderr:\n{}",
            door.join(" "),
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        );
    }

    let describe = run_listed(repo.path(), home.path(), &["describe"]);
    let stdout = String::from_utf8_lossy(&describe.stdout);
    assert!(
        stdout.contains(REWORDED_ADR_DESCRIPTION),
        "the shadow's reworded `description:` must be live through `jigc describe`; got:\n{stdout}",
    );
}

/// The wholesale opt-out reaches the project layer too: a shadow of a doctype whose
/// **owning pack ships no manifest** is unchecked, exactly as that pack's own schemas
/// are ([`manifest_less_pack_is_unaffected`]). The freeze records what a pack
/// *declares* frozen; a pack that declares nothing freezes nothing, at either layer.
#[test]
fn a_project_shadow_of_a_manifest_less_pack_doctype_loads_clean() {
    let repo = TempDir::new("shadow-nomanifest-repo");
    let home = TempDir::new("shadow-nomanifest-home");
    let pack = dev_pack_copy("shadow-nomanifest-pack");
    init_repo(repo.path());
    fs::remove_file(pack.path().join("config").join("schema-manifest.yaml"))
        .expect("drop the freeze manifest");
    install_schema_shadow(repo.path(), "adr", &adr_shadow_with_owner());

    let out = run_start(repo.path(), home.path(), pack.path());
    assert!(
        out.status.success(),
        "a project shadow of a manifest-less pack's doctype must load clean; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}
