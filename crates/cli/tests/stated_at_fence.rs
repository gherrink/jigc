//! Real-binary acceptance for the M43 **stated-at fence** (law 3, structural
//! tier — `design/surface-contract.md` → The stated-at fence; DECISIONS.md →
//! M43 Settle #6): every member of the **ambush-class owe-set**
//! (`finalize.promote-clobber` · the staging pair `finalize.left-out` +
//! `finalize.nothing-staged` · `finalize.carried-staged`) must have at least
//! one declarer among a **step-shipping** pack's steps'
//! `states-constraints:` front-matter — the soliciting step carries the
//! contract's statement, so the constraint is stated where it binds instead
//! of first appearing in its block message (an ambush).
//!
//! **Since M51 Increment 8 T2 the subject is the packs that ship steps, not the
//! packs that ship a manifest** (`settle-record.md` → D10, N26's first question):
//! a manifest with no steps beneath it blocked **vacuously** — the schema-only
//! project pack could not buy the freeze stamp without shipping workflow steps it
//! has no reason to own — while a step-shipping pack that froze nothing went
//! unchecked though its steps mint the task an operator finalizes.
//! `a_step_less_pack_is_unchecked` and `a_manifest_less_step_shipping_pack_is_checked`
//! drive the two halves; the three tiers below keep the **manifest** subject,
//! because each reads a schema or a catalog only a declaring pack has.
//!
//! **Since M51 Increment 8 that owe-set is a derivation, not a const**
//! (`cli::pack::ambush_class_codes` —
//! `derive(commit-on-behalf blocking codes) - Exempt(reason) union
//! DECLARED_CONTRACT_IDENTIFIERS`), because nothing reddened when a new
//! ambush-class contract stayed off a hand-list. Two legs of that derivation
//! run at the bottom of this file: the **site leg** (every row's code really is
//! minted at the production function it names) and the **exempt row's green
//! cell** — Increment 3's `setup.dirty-install-path`, which satisfies the
//! source-set rule, is declared by *no* step of either pack, and leaves
//! pack-load green anyway. Its third leg (the row's door classifies
//! `CommitsOnBehalf`) is a table lookup and runs beside the registry.
//!
//! A shipped-tree `JIGC_PACK_DIR` / listed-pack copy with one declarer
//! stripped exits **non-zero at pack-load naming the undeclared code**; the
//! unmutated copies, the embedded base, and the composed
//! `[dev ▸ methodology]` pair all load clean (each shipped pack loaded
//! **alone** carries every declarer — the methodology-alone dogfood path); a
//! pack that ships **no steps** is unchecked. Honest bound: the fence proves
//! presence-of-obligation, never prose quality.
//!
//! The M44 Inc 6 **per-soliciting-step tier** (D5 — the path-local-guidance
//! owe-set) rides the same molds: a step whose body references
//! `{{schema:<T>}}` with `T` a **singleton** doctype (a create-or-update
//! singleton author solicit) must declare `create.singleton-copy-in` in its
//! `states-constraints:` front-matter — the owe-set is derived from the
//! enumerable structural signal the step already renders, so a template that
//! omits the copy-in/append statement reddens at pack-load, not by author
//! diligence.
//!
//! This drives the **real `jigc` binary** end-to-end — the emitted exit code +
//! stderr are the contract (the `freeze_enforcement.rs` /
//! `suppression_fence.rs` pattern).
//!
//! The M47 Inc 9 **named-fact tier** rides the same molds one level deeper: a
//! declared code must buy the contract's *named facts*, not the declaration
//! alone. For every (declaring step × declared code × required token) triple the
//! two shipped pack trees and the code-side map owe — enumerated from the trees
//! and `cli::pack::CONSTRAINT_REQUIRED_TOKENS`, never hand-listed — deleting that
//! token's occurrences from a copied pack blocks pack load, naming step, code and
//! token. Its **conditional tier** (T2) rides the referenced singleton's own
//! structure: a copy-in declarer whose solicited singleton has ≥1 `repeatable:`
//! section owes the append/collision clause too (`cli::pack::COPY_IN_APPEND_TOKENS`
//! — corrected at M49 Inc 10 T2, where the clause stopped demanding the *"would
//! double"* the binary never does), and a declarer whose singleton is slot-only owes
//! nothing and stays clean — the exemption asserted over the enumerated declarer set,
//! never assumed.
//!
//! Its **obligation direction** (T3) closes the pair into a biconditional: every
//! `create.singleton-copy-in` declarer of the two trees (two in dev, four in
//! methodology — six across both, enumerated from the trees) must still reference
//! the `{{schema:<T>}}` its declaration guards, so deleting the ref cannot delete
//! the obligation itself while the composed step keeps promising a payload that
//! never follows.

use crate::support::{pack_locator, root_walk};
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
            "jigc-stated-at-{tag}-{}-{:?}",
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
    Path::new(cli::pack_path!(dev)).to_path_buf()
}

/// The on-disk methodology pack home (`<root>/packs/methodology`).
fn methodology_pack_tree() -> PathBuf {
    Path::new(cli::pack_path!(methodology)).to_path_buf()
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

/// Strip the named step's `states-constraints:` declaration from a copied pack
/// (the flow-style line becomes the empty list) — the mutation the fence must
/// catch: the contract's declarer withdrawn while the ambush-class code still
/// exists in the binary.
fn strip_states_constraints(pack: &Path, step: &str) {
    let path = pack.join("steps").join(format!("{step}.yaml"));
    let body = fs::read_to_string(&path).expect("read the copied step");
    let mut out = String::new();
    let mut hit = false;
    for line in body.lines() {
        if line.starts_with("states-constraints:") {
            out.push_str("states-constraints: []\n");
            hit = true;
            continue;
        }
        out.push_str(line);
        out.push('\n');
    }
    assert!(
        hit,
        "the shipped `{step}` step must carry a `states-constraints:` declaration to strip"
    );
    fs::write(&path, out).expect("write the stripped step");
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

const START: &[&str] = &[
    "start",
    "--workflow",
    "single-task",
    "stated-at fence probe",
];

/// The fence: a dev-pack copy whose `finalize` step withdrew its
/// `states-constraints:` declaration (the staging pair + the carried-staged
/// contract) is **blocked at pack-load** — the composing `jigc start` exits
/// non-zero, and stderr names an undeclared ambush-class code.
#[test]
fn a_stripped_staging_declarer_is_blocked_at_pack_load() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = pack_copy("strip", &embedded_pack_tree());
    init_repo(repo.path());
    strip_states_constraints(pack.path(), "finalize");

    let out = run_with_pack(repo.path(), home.path(), pack.path(), START);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !out.status.success(),
        "an ambush-class code with no declarer must make `jigc start` exit non-zero; stdout:\n{}\nstderr:\n{stderr}",
        String::from_utf8_lossy(&out.stdout),
    );
    assert!(
        stderr.contains("finalize.carried-staged"),
        "stderr must name the undeclared `finalize.carried-staged` code; got:\n{stderr}",
    );
    assert!(
        stderr.contains("states-constraints"),
        "stderr must name the missing `states-constraints:` declaration; got:\n{stderr}",
    );
}

/// Per-constituent isolation: a *listed* methodology-pack copy whose
/// `migration-finalize` step withdrew the `finalize.promote-clobber` declarer
/// blocks at pack-load too — even though the dev pack's own declarer is intact
/// in the same composition, each step-shipping pack must carry every
/// declarer itself (the methodology-alone dogfood path).
#[test]
fn a_stripped_clobber_declarer_blocks_through_the_listed_pack_path() {
    let repo = TempDir::new("m-repo");
    let home = TempDir::new("m-home");
    let pack = pack_copy("m-strip", &methodology_pack_tree());
    init_repo(repo.path());
    strip_states_constraints(pack.path(), "migration-finalize");
    fs::write(
        repo.path().join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", pack.path().display()),
    )
    .expect("write packs.yaml naming the methodology copy");

    let out = run_embedded(repo.path(), home.path(), START);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !out.status.success(),
        "a listed pack missing the `finalize.promote-clobber` declarer must block; stdout:\n{}\nstderr:\n{stderr}",
        String::from_utf8_lossy(&out.stdout),
    );
    assert!(
        stderr.contains("finalize.promote-clobber"),
        "stderr must name the undeclared `finalize.promote-clobber` code; got:\n{stderr}",
    );
}

/// The controls: an **unmutated** dev-pack copy, the bare **embedded** base,
/// and the composed **`[dev ▸ methodology]`** pair all load clean — both
/// shipped packs carry a declarer for all four ambush-class members, so the
/// fence is inert on every production composition.
#[test]
fn the_shipped_compositions_all_load_clean() {
    // The unmutated JIGC_PACK_DIR copy.
    let repo = TempDir::new("clean-repo");
    let home = TempDir::new("clean-home");
    let pack = pack_copy("clean", &embedded_pack_tree());
    init_repo(repo.path());
    let out = run_with_pack(repo.path(), home.path(), pack.path(), START);
    assert!(
        out.status.success(),
        "an unmutated dev-pack copy must load clean; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );

    // The embedded base (no JIGC_PACK_DIR, no marker).
    let repo = TempDir::new("emb-repo");
    let home = TempDir::new("emb-home");
    init_repo(repo.path());
    let out = run_embedded(repo.path(), home.path(), START);
    assert!(
        out.status.success(),
        "the embedded base must load clean; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );

    // The composed [dev ▸ methodology] pair via the setup-written marker.
    let repo = TempDir::new("pair-repo");
    let home = TempDir::new("pair-home");
    init_repo(repo.path());
    fs::write(
        repo.path().join(".jigc").join("config").join("packs.yaml"),
        "compose-embedded-methodology: true\n",
    )
    .expect("write the compose marker");
    let out = run_embedded(repo.path(), home.path(), START);
    assert!(
        out.status.success(),
        "the composed [dev ▸ methodology] pair must load clean; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
}

/// The **slug-rule block** the shipped dev manifest declares, lifted verbatim —
/// a fixture manifest that froze its own numerals would redden at the next
/// `slug-rule-version` bump for a reason that has nothing to do with this fence
/// (`engine::manifest::check` → `SlugRuleUndeclared` / the two drift arms).
fn shipped_slug_rule_block() -> String {
    let manifest = fs::read_to_string(
        embedded_pack_tree()
            .join("config")
            .join("schema-manifest.yaml"),
    )
    .expect("read the shipped dev manifest");
    let mut out = String::new();
    let mut inside = false;
    for line in manifest.lines() {
        if line.starts_with("slug-rule:") {
            inside = true;
            out.push_str(line);
            out.push('\n');
            continue;
        }
        if inside {
            if line.starts_with(char::is_whitespace) {
                out.push_str(line);
                out.push('\n');
                continue;
            }
            break;
        }
    }
    assert!(
        out.contains("version:") && out.contains("hash:"),
        "the shipped dev manifest must carry a `slug-rule:` block to lift; got:\n{out}"
    );
    out
}

/// Seed a **step-less** pack: a `config/schema-manifest.yaml` declaring the
/// freeze it opts into (an empty doctype set — it ships no schemas either) and
/// nothing else. The schema-only project pack `decisions-pending.md`'s N26 was
/// recorded about, reduced to the shape that matters here: it **opts into the
/// freeze** and owns **no step** to state a `jigc task finalize` contract in.
fn seed_step_less_pack(pack: &Path) {
    let config = pack.join("config");
    fs::create_dir_all(&config).expect("mk the step-less pack's config/");
    fs::write(config.join("defaults.yaml"), "pack-id: step-less\n")
        .expect("seed the pack's own id");
    fs::write(
        config.join("schema-manifest.yaml"),
        format!("{}doctypes: []\n", shipped_slug_rule_block()),
    )
    .expect("seed the step-less pack's freeze manifest");
}

/// **The re-keyed omitting context (M51 Increment 8 T2): a pack that ships no
/// steps is unchecked.** The subject of the structural tier is the constituents
/// that **ship steps**, not the ones that ship a manifest — so a listed pack
/// that opts into the freeze and owns no step at all sits outside this fence and
/// the composition loads clean.
///
/// It is the arm `a_manifest_less_pack_is_unchecked` used to be, re-keyed because
/// the subject move **falsifies** it in both directions: the manifest is no longer
/// what puts a pack inside the fence (see
/// [`a_manifest_less_step_shipping_pack_is_checked`]), and a manifest with no steps
/// beneath it used to block **vacuously** — every ambush-class code undeclared
/// because the pack had nowhere to declare one. That was N26's recorded defect: *a
/// schema-only project pack cannot buy the stamp without shipping workflow steps it
/// has no reason to own* (`implementation/decisions-pending.md` → N26, settled at
/// `settle-record.md` → D10).
#[test]
fn a_step_less_pack_is_unchecked() {
    let repo = TempDir::new("sl-repo");
    let home = TempDir::new("sl-home");
    let pack = TempDir::new("stepless");
    init_repo(repo.path());
    seed_step_less_pack(pack.path());
    fs::write(
        repo.path().join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", pack.path().display()),
    )
    .expect("write packs.yaml naming the step-less pack");

    let out = run_embedded(repo.path(), home.path(), START);
    assert!(
        out.status.success(),
        "a pack that ships no steps must be outside the stated-at fence; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// **The composing context the subject move opens (M51 Increment 8 T2): a
/// manifest-less pack that ships steps is checked.** The freeze manifest is the
/// opt-in for the freeze gate and for the three fence tiers that read a schema;
/// it is **not** what makes a pack owe an ambush-class statement. A pack whose
/// steps compose a `jigc task finalize` solicit ambushes its reader whether or not
/// it froze a doctype — so the same dev-pack copy that
/// [`a_step_less_pack_is_unchecked`]'s sibling used to let through, manifest
/// dropped and the `finalize` declarer withdrawn, now blocks at pack-load naming
/// the undeclared code.
#[test]
fn a_manifest_less_step_shipping_pack_is_checked() {
    let repo = TempDir::new("nm-repo");
    let home = TempDir::new("nm-home");
    let pack = pack_copy("nomanifest", &embedded_pack_tree());
    init_repo(repo.path());
    strip_states_constraints(pack.path(), "finalize");
    fs::remove_file(pack.path().join("config").join("schema-manifest.yaml"))
        .expect("drop the freeze manifest");

    let out = run_with_pack(repo.path(), home.path(), pack.path(), START);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !out.status.success(),
        "a manifest-less pack that ships steps must still owe its ambush-class \
         declarers; stdout:\n{}\nstderr:\n{stderr}",
        String::from_utf8_lossy(&out.stdout),
    );
    assert!(
        stderr.contains("finalize.carried-staged"),
        "stderr must name the undeclared `finalize.carried-staged` code; got:\n{stderr}",
    );
    assert!(
        stderr.contains("states-constraints"),
        "stderr must name the missing `states-constraints:` declaration; got:\n{stderr}",
    );
}

/// The unmutated control for the pair above: the **same** manifest-less dev-pack
/// copy, declarers intact, loads clean — so what the fence caught is the withdrawn
/// declaration and not the dropped manifest.
#[test]
fn a_manifest_less_step_shipping_pack_with_its_declarers_loads_clean() {
    let repo = TempDir::new("nmc-repo");
    let home = TempDir::new("nmc-home");
    let pack = pack_copy("nomanifest-clean", &embedded_pack_tree());
    init_repo(repo.path());
    fs::remove_file(pack.path().join("config").join("schema-manifest.yaml"))
        .expect("drop the freeze manifest");

    let out = run_with_pack(repo.path(), home.path(), pack.path(), START);
    assert!(
        out.status.success(),
        "a manifest-less dev-pack copy with its declarers intact must load clean; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
}

// ---------------------------------------------------------------------------
// M44 Inc 6 — the per-soliciting-step tier (D5): a `{{schema:<T>}}`-derived
// owe-set. A step soliciting a **singleton** doctype's authoring must declare
// `create.singleton-copy-in`.
// ---------------------------------------------------------------------------

/// The per-soliciting-step fence: a dev-pack copy whose singleton-authoring
/// migrate step (`author-migration`, soliciting the `changelog` singleton via
/// `{{schema:changelog}}`) withdrew its
/// `states-constraints: [create.singleton-copy-in]` declaration is **blocked at
/// pack-load** — the composing `jigc start` exits non-zero, and stderr names the
/// undeclared step and the `create.singleton-copy-in` code.
#[test]
fn a_stripped_singleton_copy_in_declarer_is_blocked_at_pack_load() {
    let repo = TempDir::new("ci-repo");
    let home = TempDir::new("ci-home");
    let pack = pack_copy("ci-strip", &embedded_pack_tree());
    init_repo(repo.path());
    strip_states_constraints(pack.path(), "author-migration");

    let out = run_with_pack(repo.path(), home.path(), pack.path(), START);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !out.status.success(),
        "a singleton-authoring step missing the copy-in declarer must make `jigc start` \
         exit non-zero; stdout:\n{}\nstderr:\n{stderr}",
        String::from_utf8_lossy(&out.stdout),
    );
    assert!(
        stderr.contains("create.singleton-copy-in"),
        "stderr must name the undeclared `create.singleton-copy-in` code; got:\n{stderr}",
    );
    assert!(
        stderr.contains("author-migration"),
        "stderr must name the offending `author-migration` step; got:\n{stderr}",
    );
    assert!(
        stderr.contains("states-constraints"),
        "stderr must name the missing `states-constraints:` declaration; got:\n{stderr}",
    );
}

/// Per-origin isolation: a *listed* methodology-pack copy whose
/// `author-migration-roadmap` step (soliciting the `roadmap` singleton) withdrew
/// its copy-in declarer blocks at pack-load too — the singleton resolves within
/// its own origin pack, so the fence fires per manifest-shipping constituent.
#[test]
fn a_stripped_singleton_copy_in_blocks_through_the_listed_pack_path() {
    let repo = TempDir::new("mci-repo");
    let home = TempDir::new("mci-home");
    let pack = pack_copy("mci-strip", &methodology_pack_tree());
    init_repo(repo.path());
    strip_states_constraints(pack.path(), "author-migration-roadmap");
    fs::write(
        repo.path().join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", pack.path().display()),
    )
    .expect("write packs.yaml naming the methodology copy");

    let out = run_embedded(repo.path(), home.path(), START);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !out.status.success(),
        "a listed pack whose singleton-authoring step dropped the copy-in declarer must \
         block; stdout:\n{}\nstderr:\n{stderr}",
        String::from_utf8_lossy(&out.stdout),
    );
    assert!(
        stderr.contains("create.singleton-copy-in"),
        "stderr must name the undeclared `create.singleton-copy-in` code; got:\n{stderr}",
    );
    assert!(
        stderr.contains("author-migration-roadmap"),
        "stderr must name the offending `author-migration-roadmap` step; got:\n{stderr}",
    );
}

/// The omitting context: a **manifest-less** pack is outside the per-soliciting-step
/// fence too — the *same* stripped copy-in declarer that the manifest-bearing copy
/// blocks on loads clean once `config/schema-manifest.yaml` is dropped.
#[test]
fn a_singleton_copy_in_manifest_less_pack_is_unchecked() {
    let repo = TempDir::new("nmci-repo");
    let home = TempDir::new("nmci-home");
    let pack = pack_copy("nmci", &embedded_pack_tree());
    init_repo(repo.path());
    strip_states_constraints(pack.path(), "author-migration");
    fs::remove_file(pack.path().join("config").join("schema-manifest.yaml"))
        .expect("drop the freeze manifest");

    let out = run_with_pack(repo.path(), home.path(), pack.path(), START);
    assert!(
        out.status.success(),
        "a manifest-less pack must be outside the per-soliciting-step fence; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
}

// ---------------------------------------------------------------------------
// M47 Inc 9 T1 — the named-fact tier: a declared code buys the contract's own
// named facts, not just the declaration. The axis is mechanically enumerated:
// (declaring step × declared code × required token) over each shipped pack tree
// and the code-side map.
// ---------------------------------------------------------------------------

/// Split a step file into `(front-matter prefix, body)` — the body is exactly
/// what the fence reads (`engine::compose::load_step_def`'s `body`), so the
/// deletion below never touches the `states-constraints:` line that declares the
/// code under test.
fn split_body(text: &str) -> (&str, &str) {
    if let Some(rest) = text.strip_prefix("---\n")
        && let Some(end) = rest.find("\n---\n")
    {
        let cut = "---\n".len() + end + "\n---\n".len();
        return text.split_at(cut);
    }
    ("", text)
}

/// Delete every occurrence of `token` from `body`, matching it the way the fence
/// reads the prose — whitespace runs collapsed, ASCII case folded — so a phrase
/// that wraps across a hard line break or opens a sentence capitalized is removed
/// too. Each match's raw span collapses to a single space.
fn delete_token(body: &str, token: &str) -> String {
    // The normalized view, plus a byte-for-byte map back into `body`.
    let mut norm = String::new();
    let mut origin: Vec<usize> = Vec::new();
    let mut pending_space = false;
    for (at, ch) in body.char_indices() {
        if ch.is_whitespace() {
            pending_space = true;
            continue;
        }
        if pending_space && !norm.is_empty() {
            norm.push(' ');
            origin.resize(norm.len(), at);
        }
        pending_space = false;
        norm.push(ch.to_ascii_lowercase());
        origin.resize(norm.len(), at);
    }

    let mut spans = Vec::new();
    let mut from = 0;
    while let Some(hit) = norm[from..].find(token) {
        let start = from + hit;
        let end = start + token.len();
        spans.push((
            origin[start],
            origin.get(end).copied().unwrap_or(body.len()),
        ));
        from = end;
    }

    let mut out = body.to_string();
    for (start, end) in spans.into_iter().rev() {
        out.replace_range(start..end, " ");
    }
    out
}

/// Delete a named fact from a copied pack's step, leaving its `states-constraints:`
/// declaration standing — the mutation the named-fact fence must catch. The
/// deletion is asserted to bite, so the loop also proves the shipped prose really
/// carries every token the map claims.
fn delete_fact(pack: &Path, step: &str, token: &str) {
    let path = pack.join("steps").join(format!("{step}.yaml"));
    let text = fs::read_to_string(&path).expect("read the copied step");
    let (front, body) = split_body(&text);
    let stripped = delete_token(body, token);
    assert_ne!(
        stripped, body,
        "the shipped `{step}` step must state \"{token}\" for its deletion to be a real mutation",
    );
    fs::write(&path, format!("{front}{stripped}")).expect("write the mutated step");
}

/// Every (declaring step × declared code × required token) triple a pack tree owes
/// — read off the tree's own steps and the code-side map, never a hand-list.
fn owed_triples(pack: &Path) -> Vec<(String, String, String)> {
    let mut steps: Vec<PathBuf> = fs::read_dir(pack.join("steps"))
        .expect("read the copied steps dir")
        .map(|entry| entry.expect("dir entry").path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "yaml"))
        .collect();
    steps.sort();

    let mut out = Vec::new();
    for path in steps {
        let id = path
            .file_stem()
            .expect("step file stem")
            .to_string_lossy()
            .into_owned();
        let bytes = fs::read(&path).expect("read the copied step");
        let def = engine::compose::load_step_def(&id, &bytes).expect("step front-matter parses");
        for code in &def.states_constraints {
            let Some((_, tokens)) = cli::pack::CONSTRAINT_REQUIRED_TOKENS
                .iter()
                .find(|(fenced, _)| fenced == code)
            else {
                continue;
            };
            for token in *tokens {
                out.push((id.clone(), code.clone(), (*token).to_owned()));
            }
        }
    }
    out
}

/// The fenced codes **no pack owes a declarer for** — stated where they can bind and nowhere
/// else (`cli::pack::AmbushDisposition::DeclaredWhereReachable`; the F-10 review's MEDIUM-4).
///
/// Read off the registry rather than named here, so a fourth such row joins this partition
/// with no edit — and so the partition cannot silently disagree with the one
/// `cli::pack::fenced_contract_codes` computes.
fn declared_where_reachable() -> BTreeSet<&'static str> {
    cli::pack::AMBUSH_CONTRACTS
        .iter()
        .filter_map(|row| match row.disposition {
            cli::pack::AmbushDisposition::DeclaredWhereReachable { .. } => Some(row.code),
            _ => None,
        })
        .collect()
}

/// Drive the whole axis through the real binary: delete each owed token in turn
/// from `pack`, assert the load blocks naming step + code + token, then restore
/// the step before the next triple. Returns the code set this pack's own steps covered.
///
/// **The floor is the universally-owed codes, not the whole map** (the F-10 review's
/// MEDIUM-4): a code stated only where it can bind is declared by the one pack whose
/// composition reaches its door, and demanding it here would demand a declarer in a pack that
/// cannot reach the arm — the very lie its disposition exists to avoid. The whole-map sweep is
/// still asserted, at the call site of the pack that does declare every one of them.
fn drive_named_fact_axis(
    pack: &Path,
    run: &dyn Fn() -> std::process::Output,
) -> BTreeSet<&'static str> {
    let triples = owed_triples(pack);
    let covered: BTreeSet<&str> = cli::pack::CONSTRAINT_REQUIRED_TOKENS
        .iter()
        .map(|(code, _)| *code)
        .filter(|code| triples.iter().any(|(_, declared, _)| declared == code))
        .collect();
    let universal: BTreeSet<&str> = cli::pack::fenced_contract_codes()
        .into_iter()
        .filter(|code| !declared_where_reachable().contains(code))
        .collect();
    assert!(
        covered.is_superset(&universal),
        "this pack must declare every universally-owed fenced code for the axis to sweep \
         them; missing {:?}",
        universal.difference(&covered).collect::<Vec<_>>(),
    );

    for (step, code, token) in triples {
        let path = pack.join("steps").join(format!("{step}.yaml"));
        let original = fs::read_to_string(&path).expect("read the step");
        delete_fact(pack, &step, &token);
        let out = run();
        let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
        let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
        fs::write(&path, &original).expect("restore the step");

        assert!(
            !out.status.success(),
            "deleting \"{token}\" from `{step}` (declaring `{code}`) must block pack load; \
             stdout:\n{stdout}\nstderr:\n{stderr}",
        );
        assert!(
            stderr.contains(&format!("`{step}`")),
            "stderr must name the `{step}` step; got:\n{stderr}",
        );
        assert!(
            stderr.contains(&code),
            "stderr must name the `{code}` code; got:\n{stderr}",
        );
        assert!(
            stderr.contains(&token),
            "stderr must name the missing \"{token}\" fact; got:\n{stderr}",
        );
    }
    covered
}

/// The dev-pack seam: every named fact the shipped dev tree owes, deleted one at a
/// time from a `JIGC_PACK_DIR` copy, blocks `jigc start` at pack load.
///
/// **And this is where the whole map is swept** (the F-10 review's MEDIUM-4): the dev pack
/// declares every fenced code — the universally-owed ones and the two the amend arm states
/// where it can bind — so the equality that used to sit inside the shared helper belongs
/// here, at the one pack it is true of. The methodology pack cannot reach the amend arm and
/// declares neither; demanding it there is the lie the third disposition exists to avoid.
#[test]
fn every_named_fact_the_dev_pack_owes_is_bought_at_pack_load() {
    let repo = TempDir::new("nf-repo");
    let home = TempDir::new("nf-home");
    let pack = pack_copy("nf-dev", &embedded_pack_tree());
    init_repo(repo.path());

    let covered = drive_named_fact_axis(pack.path(), &|| {
        run_with_pack(repo.path(), home.path(), pack.path(), START)
    });
    assert_eq!(
        covered,
        cli::pack::fenced_contract_codes(),
        "the dev pack declares every fenced constraint code, so the axis above swept the \
         whole map — a code the map fences and no shipped step declares would buy nothing \
         anywhere, which is the receipt-for-nothing MEDIUM-4 found",
    );
}

/// **Every `DeclaredWhereReachable` row's claim is true of the shipped pack** (the F-10
/// review's MEDIUM-4): the row says *no pack owes this, and **that** step declares it*, and
/// the second half is a claim about bytes. Unchecked, the disposition would be a place to
/// park a code nobody states — which is the `Exempt` cell wearing a different name.
#[test]
fn every_declared_where_reachable_row_is_declared_by_the_step_it_names() {
    let root = workspace_root();
    for row in cli::pack::AMBUSH_CONTRACTS {
        let cli::pack::AmbushDisposition::DeclaredWhereReachable { pack, declarer, .. } =
            row.disposition
        else {
            continue;
        };
        // The pack is named by identity, so it is found by identity (M54 S2): the tree
        // whose manifest's sibling `defaults.yaml` carries that `pack-id`.
        let manifests = pack_locator::locate(&root, pack_locator::Tree::Working)
            .unwrap_or_else(|dups| panic!("a pack id is claimed twice: {dups:?}"));
        let manifest = manifests.get(pack).unwrap_or_else(|| {
            panic!(
                "`{}` names the pack `{pack}`, which no pack in the working tree claims \
                 (found {:?})",
                row.code,
                manifests.keys().collect::<Vec<_>>(),
            )
        });
        let pack_root = &manifest[..manifest.len() - pack_locator::MANIFEST.len()];
        let path = root.join(pack_root).join(declarer);
        let bytes = fs::read(&path).unwrap_or_else(|err| {
            panic!(
                "`{}` names the declarer `{declarer}`, unreadable: {err}",
                row.code,
            )
        });
        let id = path
            .file_stem()
            .expect("the declarer is a file")
            .to_string_lossy()
            .into_owned();
        let def = engine::compose::load_step_def(&id, &bytes)
            .expect("the named declarer's front-matter parses");
        assert!(
            def.states_constraints.iter().any(|code| code == row.code),
            "`{}` claims `{declarer}` declares it; that step's `states-constraints:` carries \
             {:?}",
            row.code,
            def.states_constraints,
        );
    }
}

/// The methodology seam: the same axis over the on-disk methodology tree, reached
/// through the `packs.yaml`-listed pack path — each manifest-shipping constituent
/// is checked in isolation, so the dev pack's intact prose in the same composition
/// buys the methodology copy nothing.
#[test]
fn every_named_fact_the_methodology_pack_owes_is_bought_at_pack_load() {
    let repo = TempDir::new("nfm-repo");
    let home = TempDir::new("nfm-home");
    let pack = pack_copy("nf-meth", &methodology_pack_tree());
    init_repo(repo.path());
    fs::write(
        repo.path().join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", pack.path().display()),
    )
    .expect("write packs.yaml naming the methodology copy");

    drive_named_fact_axis(pack.path(), &|| {
        run_embedded(repo.path(), home.path(), START)
    });
}

// ---------------------------------------------------------------------------
// M47 Inc 9 T2 — the conditional append tier: the append half of the copy-in
// contract is owed wherever items can double, and the owe-set is derived from the
// referenced singleton's own `repeatable:` sections — never hand-listed, never an
// exclusion list.
// ---------------------------------------------------------------------------

/// The named-fact comparison view (`cli::pack::normalized_body`'s twin): whitespace
/// runs collapsed, ASCII case folded — how the fence reads a step's prose.
fn normalized(body: &str) -> String {
    let mut out = String::new();
    let mut pending_space = false;
    for ch in body.chars() {
        if ch.is_whitespace() {
            pending_space = true;
            continue;
        }
        if pending_space && !out.is_empty() {
            out.push(' ');
        }
        pending_space = false;
        out.push(ch.to_ascii_lowercase());
    }
    out
}

/// Every `<T>` a step body solicits via `{{schema:<T>}}` — read from the body the
/// same way the compose seam does, independently of the fence's own extractor.
fn solicited_types(body: &str) -> Vec<String> {
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

/// Load a pack tree's step defs, sorted by id — the shared read behind both
/// enumerations below.
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

/// The pack tree's `create.singleton-copy-in` declarers — the full declarer set the
/// conditional tier partitions.
fn copy_in_declarers(pack: &Path) -> BTreeSet<String> {
    step_defs(pack)
        .into_iter()
        .filter(|(_, def)| {
            def.states_constraints
                .iter()
                .any(|code| code == "create.singleton-copy-in")
        })
        .map(|(id, _)| id)
        .collect()
}

/// The **item-bearing** declarers: steps soliciting a singleton whose schema declares
/// ≥1 `repeatable:` section, so authored items append beside the committed ones and a
/// re-authored one collides. Enumerated from the shipped tree (steps × their
/// `{{schema:<T>}}` refs × `T`'s own schema), never hand-listed.
fn append_owing_steps(pack: &Path) -> BTreeSet<String> {
    let source = cli::pack::FilesystemPack::new(pack.to_path_buf());
    step_defs(pack)
        .into_iter()
        .filter(|(_, def)| {
            solicited_types(&def.body).into_iter().any(|ty| {
                let Ok(raw) = fs::read(pack.join("schemas").join(format!("{ty}.yaml"))) else {
                    return false;
                };
                let Ok(schema) = cli::pack::load_pack_schema(&source, &raw) else {
                    return false;
                };
                schema.singleton
                    && schema.sections.iter().any(|section| {
                        matches!(section.body, engine::schema::SectionBody::Repeatable { .. })
                    })
            })
        })
        .map(|(id, _)| id)
        .collect()
}

/// Drive the conditional tier's whole axis through the real binary: delete each fact
/// of the append/collision clause in turn from each item-bearing declarer, assert the
/// load blocks naming step, code and token, then restore the step before the next
/// deletion.
fn drive_append_axis(pack: &Path, run: &dyn Fn() -> std::process::Output) {
    let owing = append_owing_steps(pack);
    assert!(
        !owing.is_empty(),
        "this pack must ship at least one item-bearing copy-in declarer for the axis to sweep",
    );
    assert!(
        owing.is_subset(&copy_in_declarers(pack)),
        "every item-bearing solicit must already declare `create.singleton-copy-in`",
    );

    for step in owing {
        for token in cli::pack::COPY_IN_APPEND_TOKENS {
            let path = pack.join("steps").join(format!("{step}.yaml"));
            let original = fs::read_to_string(&path).expect("read the step");
            delete_fact(pack, &step, token);
            let out = run();
            let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
            let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
            fs::write(&path, &original).expect("restore the step");

            assert!(
                !out.status.success(),
                "deleting \"{token}\" from the item-bearing declarer `{step}` must block pack \
                 load; stdout:\n{stdout}\nstderr:\n{stderr}",
            );
            assert!(
                stderr.contains(&format!("`{step}`")),
                "stderr must name the `{step}` step; got:\n{stderr}",
            );
            assert!(
                stderr.contains("create.singleton-copy-in"),
                "stderr must name the `create.singleton-copy-in` code; got:\n{stderr}",
            );
            assert!(
                stderr.contains(token),
                "stderr must name the missing \"{token}\" fact; got:\n{stderr}",
            );
        }
    }
}

/// The dev-pack seam: every item-bearing copy-in declarer of the shipped dev tree
/// (`record-changelog` + `author-migration`, both soliciting the repeatable-bearing
/// `changelog` singleton) owes the append half — deleting it blocks `jigc start` at
/// pack load.
#[test]
fn every_item_bearing_dev_declarer_states_the_append_half() {
    let repo = TempDir::new("ap-repo");
    let home = TempDir::new("ap-home");
    let pack = pack_copy("ap-dev", &embedded_pack_tree());
    init_repo(repo.path());

    drive_append_axis(pack.path(), &|| {
        run_with_pack(repo.path(), home.path(), pack.path(), START)
    });
}

/// The methodology seam: the same axis over the on-disk methodology tree through the
/// `packs.yaml`-listed pack path — its item-bearing declarers (`roadmap`,
/// `decisions-log`, `deferral-ledger`) are checked in isolation from the dev pack's
/// intact prose in the same composition.
#[test]
fn every_item_bearing_methodology_declarer_states_the_append_half() {
    let repo = TempDir::new("apm-repo");
    let home = TempDir::new("apm-home");
    let pack = pack_copy("ap-meth", &methodology_pack_tree());
    init_repo(repo.path());
    fs::write(
        repo.path().join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", pack.path().display()),
    )
    .expect("write packs.yaml naming the methodology copy");

    drive_append_axis(pack.path(), &|| {
        run_embedded(repo.path(), home.path(), START)
    });
}

/// The omitting context that must stay **inert, never an error**: a copy-in declarer
/// whose singleton is **slot-only** (methodology's `author-migration-vision` — the
/// `vision` schema has no `repeatable:` section, and its prose says the slots
/// OVERWRITE) owes no clause fact and loads clean. Asserted, not assumed: the exempt
/// declarers are enumerated as the declarer set minus the item-bearing one, and each
/// is proven to state neither append fact — so the clean load below is the tier
/// standing down, not prose accidentally satisfying it. Flat-mapping the tokens would
/// make the fence demand statements that are false for this step (its slots have no
/// items to collide, so `write.already-present` cannot arise there).
#[test]
fn a_slot_only_copy_in_declarer_is_exempt_from_the_append_tier() {
    let repo = TempDir::new("apx-repo");
    let home = TempDir::new("apx-home");
    let pack = pack_copy("ap-exempt", &methodology_pack_tree());
    init_repo(repo.path());
    fs::write(
        repo.path().join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", pack.path().display()),
    )
    .expect("write packs.yaml naming the methodology copy");

    let owing = append_owing_steps(pack.path());
    let exempt: Vec<String> = copy_in_declarers(pack.path())
        .difference(&owing)
        .cloned()
        .collect();
    assert!(
        !exempt.is_empty(),
        "the methodology tree must ship a slot-only copy-in declarer for the exemption to be real",
    );
    for step in &exempt {
        let bytes = fs::read(pack.path().join("steps").join(format!("{step}.yaml")))
            .expect("read the exempt step");
        let def = engine::compose::load_step_def(step, &bytes).expect("step front-matter parses");
        let body = normalized(&def.body);
        for token in cli::pack::COPY_IN_APPEND_TOKENS {
            assert!(
                !body.contains(token),
                "the exempt `{step}` step must state no append fact for its clean load to prove \
                 the exemption; it says \"{token}\"",
            );
        }
    }

    let out = run_embedded(repo.path(), home.path(), START);
    assert!(
        out.status.success(),
        "a slot-only copy-in declarer must load clean — the append tier is inert where the \
         solicited singleton has no repeating items; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
}

/// The second omitting context: a **manifest-less** pack is outside the conditional
/// tier too — the same deleted append fact that the manifest-bearing copy blocks on
/// loads clean once `config/schema-manifest.yaml` is dropped.
#[test]
fn a_missing_append_fact_manifest_less_pack_is_unchecked() {
    let repo = TempDir::new("apnm-repo");
    let home = TempDir::new("apnm-home");
    let pack = pack_copy("apnm", &embedded_pack_tree());
    init_repo(repo.path());
    let step = append_owing_steps(pack.path())
        .into_iter()
        .next()
        .expect("the dev pack owes at least one append fact");
    delete_fact(pack.path(), &step, cli::pack::COPY_IN_APPEND_TOKENS[1]);
    fs::remove_file(pack.path().join("config").join("schema-manifest.yaml"))
        .expect("drop the freeze manifest");

    let out = run_with_pack(repo.path(), home.path(), pack.path(), START);
    assert!(
        out.status.success(),
        "a manifest-less pack must be outside the conditional append tier; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
}

/// The omitting context: a **manifest-less** pack is outside the named-fact tier
/// too — the same deleted fact that the manifest-bearing copy blocks on loads
/// clean once `config/schema-manifest.yaml` is dropped.
#[test]
fn a_named_fact_manifest_less_pack_is_unchecked() {
    let repo = TempDir::new("nfnm-repo");
    let home = TempDir::new("nfnm-home");
    let pack = pack_copy("nfnm", &embedded_pack_tree());
    init_repo(repo.path());
    let (step, _, token) = owed_triples(pack.path())
        .into_iter()
        .next()
        .expect("the dev pack owes at least one named fact");
    delete_fact(pack.path(), &step, &token);
    fs::remove_file(pack.path().join("config").join("schema-manifest.yaml"))
        .expect("drop the freeze manifest");

    let out = run_with_pack(repo.path(), home.path(), pack.path(), START);
    assert!(
        out.status.success(),
        "a manifest-less pack must be outside the named-fact fence; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
}

// ---------------------------------------------------------------------------
// M47 Inc 9 T3 — the OBLIGATION axis: `create.singleton-copy-in` becomes a
// BICONDITIONAL with the `{{schema:<T>}}` ref it guards. Every tier above runs
// ref ⇒ declaration (⇒ named facts); this runs declaration ⇒ ref, so deleting
// the ref cannot delete the obligation itself while the composed step keeps
// promising a payload that never follows.
// ---------------------------------------------------------------------------

/// Delete every `{{schema:<T>}}` reference from a copied pack's step **body**,
/// leaving its `states-constraints:` declaration and every contract sentence
/// standing — the mutation the obligation direction must catch. The deletion is
/// asserted to bite, so the sweep also proves each declarer really solicits a
/// schema payload today.
fn delete_schema_refs(pack: &Path, step: &str) {
    let path = pack.join("steps").join(format!("{step}.yaml"));
    let text = fs::read_to_string(&path).expect("read the copied step");
    let (front, body) = split_body(&text);

    let mut out = String::new();
    let mut rest = body;
    loop {
        let Some(open) = rest.find("{{") else {
            out.push_str(rest);
            break;
        };
        let Some(close) = rest[open + 2..].find("}}") else {
            out.push_str(rest);
            break;
        };
        let end = open + 2 + close + 2;
        if rest[open + 2..open + 2 + close]
            .trim()
            .starts_with("schema:")
        {
            out.push_str(&rest[..open]);
        } else {
            out.push_str(&rest[..end]);
        }
        rest = &rest[end..];
    }

    assert_ne!(
        out, body,
        "the shipped `{step}` step must reference a schema for its deletion to be a real mutation",
    );
    fs::write(&path, format!("{front}{out}")).expect("write the mutated step");
}

/// Drive the obligation axis through the real binary: for every
/// `create.singleton-copy-in` declarer of the tree, delete the `{{schema:<T>}}`
/// ref its declaration guards, assert the load blocks naming step + code, then
/// restore the step before the next declarer. The declarer set is read off the
/// tree (two in dev, four in methodology today — six across both), never
/// hand-listed.
fn drive_obligation_axis(pack: &Path, run: &dyn Fn() -> std::process::Output) {
    let declarers = copy_in_declarers(pack);
    assert!(
        !declarers.is_empty(),
        "this pack must ship at least one copy-in declarer for the obligation axis to sweep",
    );

    for step in declarers {
        let path = pack.join("steps").join(format!("{step}.yaml"));
        let original = fs::read_to_string(&path).expect("read the step");
        delete_schema_refs(pack, &step);
        let out = run();
        let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
        let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
        fs::write(&path, &original).expect("restore the step");

        assert!(
            !out.status.success(),
            "deleting the `{{{{schema:<T>}}}}` ref from the copy-in declarer `{step}` must block \
             pack load — the declared obligation would outlive the write it guards, while the \
             composed step still promises a payload that never follows; stdout:\n{stdout}\n\
             stderr:\n{stderr}",
        );
        assert!(
            stderr.contains(&format!("`{step}`")),
            "stderr must name the `{step}` step; got:\n{stderr}",
        );
        assert!(
            stderr.contains("create.singleton-copy-in"),
            "stderr must name the `create.singleton-copy-in` code; got:\n{stderr}",
        );
    }
}

/// The dev-pack seam: both copy-in declarers of the shipped dev tree
/// (`record-changelog` + `author-migration`) lose the write their declaration
/// guards when their `{{schema:changelog}}` ref goes — so its deletion blocks
/// `jigc start` at pack load.
#[test]
fn every_dev_copy_in_declarer_still_solicits_the_schema_it_promises() {
    let repo = TempDir::new("ob-repo");
    let home = TempDir::new("ob-home");
    let pack = pack_copy("ob-dev", &embedded_pack_tree());
    init_repo(repo.path());

    drive_obligation_axis(pack.path(), &|| {
        run_with_pack(repo.path(), home.path(), pack.path(), START)
    });
}

/// The methodology seam: the same axis over the on-disk methodology tree through
/// the `packs.yaml`-listed pack path — its four copy-in declarers
/// (`author-migration-roadmap` / `-decisions-log` / `-deferral-ledger` /
/// `-vision`, the slot-only one included: the obligation direction binds on the
/// declaration, not on whether items can double) checked in isolation from the
/// dev pack's intact steps in the same composition.
#[test]
fn every_methodology_copy_in_declarer_still_solicits_the_schema_it_promises() {
    let repo = TempDir::new("obm-repo");
    let home = TempDir::new("obm-home");
    let pack = pack_copy("ob-meth", &methodology_pack_tree());
    init_repo(repo.path());
    fs::write(
        repo.path().join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", pack.path().display()),
    )
    .expect("write packs.yaml naming the methodology copy");

    drive_obligation_axis(pack.path(), &|| {
        run_embedded(repo.path(), home.path(), START)
    });
}

/// The omitting context: a **manifest-less** pack is outside the obligation
/// direction too — the same deleted ref the manifest-bearing copy blocks on loads
/// clean once `config/schema-manifest.yaml` is dropped.
#[test]
fn a_ref_less_copy_in_declarer_in_a_manifest_less_pack_is_unchecked() {
    let repo = TempDir::new("obnm-repo");
    let home = TempDir::new("obnm-home");
    let pack = pack_copy("obnm", &embedded_pack_tree());
    init_repo(repo.path());
    let step = copy_in_declarers(pack.path())
        .into_iter()
        .next()
        .expect("the dev pack ships at least one copy-in declarer");
    delete_schema_refs(pack.path(), &step);
    fs::remove_file(pack.path().join("config").join("schema-manifest.yaml"))
        .expect("drop the freeze manifest");

    let out = run_with_pack(repo.path(), home.path(), pack.path(), START);
    assert!(
        out.status.success(),
        "a manifest-less pack must be outside the obligation direction; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
}

// ───────────────── the derived owe-set (M51 Increment 8 / T1) ─────────────────

/// The workspace root — two levels up from `crates/cli`.
fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .canonicalize()
        .expect("canonicalize the workspace root")
}

/// **The source-set rule's second leg** ([`cli::pack::AMBUSH_CONTRACTS`]): every row's
/// code is really minted where the row says it is. The first leg — the row's door
/// classifies `CommitsOnBehalf` — is a table lookup and runs beside the registry
/// (`cli::pack`'s `every_ambush_row_names_a_commit_on_behalf_door`); this one needs
/// production source, so it runs here on [`crate::mint_doors`]' scan molds.
///
/// Two asserts per row, both over comment-blanked source — a doc-comment naming the
/// code is not a mint — and both located against the same `#[cfg(test)]` regions, so a
/// unit-test fixture is not a producer either:
///
///   * the named `<rel-path>::<fn>` is a **production** function of that file, read off
///     `code_only` (string literals blanked, so a `{` inside one cannot throw the
///     module brace matching off), so a producer that was renamed or moved reddens
///     instead of leaving a row pointing at nothing;
///   * that file's production code carries the code **literal**, read off
///     `code_and_strings` — a finding code exists in source only as a `"…"`, so the
///     reading that blanks literals is blind to exactly the thing this leg asks about.
///
/// **Why the second assert is file-scoped and not fn-scoped** — stated, because it is a
/// real bound: `setup_dirty_install_finding` reaches its code through
/// `CarryoverBoundary::Setup.code()`, the boundary-keyed selector two hundred lines
/// above it in the same file, so a fn-scoped match would demand the producer re-spell a
/// code the engine deliberately derives once.
#[test]
fn every_ambush_row_is_minted_at_its_named_production_site() {
    use crate::support::rust_source::{
        cfg_test_regions, code_and_strings, code_only, is_test_domain,
    };

    let root = workspace_root();
    for row in cli::pack::AMBUSH_CONTRACTS {
        let (rel, func) = row
            .site
            .split_once("::")
            .unwrap_or_else(|| panic!("`{}`'s site is not `<path>::<fn>`", row.code));
        let path = root.join(rel);
        let body = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("`{}` names `{rel}`, unreadable: {err}", row.code));
        let code = code_only(&body);
        let regions = cfg_test_regions(&code);

        let needle = format!("fn {func}(");
        let defined = code
            .match_indices(&needle)
            .any(|(at, _)| !is_test_domain(&path, &regions, at));
        assert!(
            defined,
            "`{}` names the producer `{}`, which is not a production fn of `{rel}`",
            row.code, row.site,
        );

        let literal = format!("\"{}\"", row.code);
        let with_strings = code_and_strings(&body);
        let minted = with_strings
            .match_indices(&literal)
            .any(|(at, _)| !is_test_domain(&path, &regions, at));
        assert!(
            minted,
            "`{rel}` carries no production mint of `{}` — the row names a site that \
             does not produce it",
            row.code,
        );
    }
}

/// **The exempt row's green cell** — the one a hand-list could not express, driven
/// rather than reasoned: `setup.dirty-install-path` satisfies the source-set rule (a
/// blocking code minted by `jigc setup`, which `BEHALF_DOORS` classifies
/// `CommitsOnBehalf`), **no** step of either shipped pack declares it, and pack-load is
/// nonetheless **green at every door** — because the registry holds it out with a
/// stated reason instead of owing it.
///
/// Both halves are asserted, because either alone proves nothing: the absence from the
/// packs is what makes the cell real, and the green load is what the exclusion rule
/// buys. Flipping the row to `Owed` turns this into the mutant's blocking cell — the
/// composing door exits non-zero naming a code no step of either pack can legally
/// declare.
#[test]
fn the_exempt_contract_is_declared_by_no_step_and_the_packs_still_load() {
    let exempt = "setup.dirty-install-path";
    assert!(
        !cli::pack::ambush_class_codes().contains(exempt),
        "`{exempt}` must stay out of the owe-set — no step can declare it",
    );

    for tree in [embedded_pack_tree(), methodology_pack_tree()] {
        for path in root_walk::files_in(&tree.join("steps"), |_| true) {
            let body = fs::read_to_string(&path).expect("read a shipped step");
            assert!(
                !body.contains(exempt),
                "`{}` declares `{exempt}`; the exempt row's cell is that NO step does",
                path.display(),
            );
        }
    }

    // The composed [dev ▸ methodology] pair — the production composition — loads clean.
    let repo = TempDir::new("exempt-repo");
    let home = TempDir::new("exempt-home");
    init_repo(repo.path());
    fs::write(
        repo.path().join(".jigc").join("config").join("packs.yaml"),
        "compose-embedded-methodology: true\n",
    )
    .expect("write the compose marker");
    let out = run_embedded(repo.path(), home.path(), START);
    assert!(
        out.status.success(),
        "an exempt ambush-class code must leave pack-load green; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
}
