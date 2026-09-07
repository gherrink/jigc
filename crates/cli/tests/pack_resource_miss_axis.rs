//! M50 Increment 10 / T3 — **a pack-resource miss names the pack it searched, with a
//! code and a route.**
//!
//! The class: *what jigc says when a resource no composed pack ships is read*. At the
//! increment's baseline (`bc595f1`) one literal answered at four sites —
//! `crates/cli/src/start.rs`'s shared `read_pack` (serving `jigc start`/`describe`/
//! `validate`/`upgrade`/`ingest`/`doc list`), `config.rs`'s two hand-copied reads (`jigc
//! config get`/`list` and `jigc config set`) and `doc.rs`'s task-bound workflow read —
//! and every one of them said:
//!
//! ```text
//! the embedded pack is missing `config/knobs`: no pack resource of kind config with id `knobs`
//! ```
//!
//! Three defects in one sentence. **It blames the embedded pack**, which under a
//! `JIGC_PACK_DIR` base plus a listed pack is not the pack it searched and may not be in
//! the composition at all; it carries **no finding code**, so a text-scraping driver has
//! nothing to key on where the JSON envelope has nothing either (these doors flatten
//! through `finding_to_err` — `design/command-output-contract.md` → *outside the
//! envelope*); and it carries **no route**, against the route floor's blocking half
//! (`design/validation.md`; `design/surface-contract.md` → law 1 + the route fence).
//!
//! **The identity accessor is driven, not assumed.** The roadmap's decomposition split
//! the four sites — *the origin pack at `start.rs`, the pack set at the other three* —
//! on `PackSource::origin_pack`. That accessor **falls back to `self` when no
//! constituent owns the id** (`packsource.rs`'s default; `CompositePack`'s override,
//! `pack.rs`), so at a **miss** — the only state this finding exists for — it names the
//! precedence winner rather than the set actually searched, which is the same lie one
//! word over. The split does not hold **as a rule about sites**, and the correction is
//! sharper than it: the one producer names **its receiver's own** provenance
//! (`provenance_segments()` zipped with `provenance_entries()` — the ids/versions the
//! `Pack:` header renders, each with its resolving path, in precedence order), so every
//! door tells the truth about what *it* looked in with no site special-cased. That is
//! per-door, not uniform — `start::load_catalog` is handed a workflow's **origin pack**
//! by the pack-local body-reference rule, so its `config/commands` miss genuinely
//! searched one pack, while the two `config` doors and `doc`'s create-gate hold the
//! composite and name both. Each cell below declares its own searched set and this suite
//! asserts the **absent** half too (`DECISIONS.md` → 2026-09-07).
//!
//! **The route is `Human`, and the test follows it rather than running an argv.** No
//! jigc verb restores a pack resource, and under a `config/knobs` miss `jigc start`,
//! `describe`, `validate`, `upgrade`, `ingest` and `doc list` all block on this very
//! finding — so a mechanical route here would either repair nothing or hard-reject,
//! which `design/surface-contract.md` names as law 2 failing one level down, and which
//! M46's PT-1 already caught once (*a route that, followed exactly, changed nothing*).
//! So the acceptance is the stronger one the route kind allows: each cell **follows the
//! emitted route verbatim** — restoring the file the route names, under a pack root
//! **read out of the emitted message** — and then re-drives the same door and asserts it
//! now works. A printed path that is not repo-real, or a route that does not repair the
//! state, fails here.
//!
//! The composition is manufactured and genuinely two packs: a throwaway copy of the dev
//! pack as the `JIGC_PACK_DIR` base, a throwaway copy of the methodology pack listed
//! over it in `.jigc/config/packs.yaml`. A resource is deleted from **both** copies —
//! a composite `read` misses only when no constituent owns the id — and the embedded
//! pack, whose name the old literal took in vain, is not in the composition.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// The one code the family answers with.
const CODE: &str = "pack.resource-missing";

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-pack-resource-miss-{tag}-{}-{:?}",
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

/// Copy a directory tree — the throwaway pack copies both composed packs are made of.
fn copy_tree(from: &Path, to: &Path) {
    fs::create_dir_all(to).expect("create the copy root");
    for entry in fs::read_dir(from).expect("read the source tree") {
        let entry = entry.expect("read a source entry");
        let target = to.join(entry.file_name());
        if entry.file_type().expect("stat a source entry").is_dir() {
            copy_tree(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), &target).expect("copy a source file");
        }
    }
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .to_path_buf()
}

/// The two composed pack trees + the repo they compose over.
struct Rig {
    /// Kept only to hold the whole rig alive until the test ends.
    _root: TempDir,
    home: TempDir,
    repo: PathBuf,
    /// The `JIGC_PACK_DIR` base — a copy of the dev pack.
    base: PathBuf,
    /// The listed (highest-precedence) pack — a copy of the methodology pack.
    listed: PathBuf,
}

impl Rig {
    fn build() -> Self {
        let root = TempDir::new("rig");
        let home = TempDir::new("home");
        let base = root.path().join("devpack");
        let listed = root.path().join("methpack");
        copy_tree(&repo_root().join("crates").join("cli").join("pack"), &base);
        copy_tree(&repo_root().join("packs").join("methodology"), &listed);

        let repo = root.path().join("repo");
        fs::create_dir_all(&repo).expect("create the repo dir");
        let git = |args: &[&str]| {
            let out = Command::new("git")
                .args(args)
                .current_dir(&repo)
                .output()
                .expect("run git");
            assert!(
                out.status.success(),
                "git {args:?} failed: {}",
                String::from_utf8_lossy(&out.stderr),
            );
        };
        git(&["init", "-q"]);
        git(&["config", "user.email", "test@example.com"]);
        git(&["config", "user.name", "Test"]);
        fs::write(repo.join("README.md"), "hello\n").expect("write a first file");
        git(&["add", "."]);
        git(&["commit", "-q", "-m", "initial"]);

        let rig = Rig {
            _root: root,
            home,
            repo,
            base,
            listed,
        };
        let setup = rig.run(&["setup"]);
        assert!(
            setup.status.success(),
            "the fixture's `jigc setup` must succeed; stderr:\n{}",
            String::from_utf8_lossy(&setup.stderr),
        );
        fs::write(
            rig.repo.join(".jigc").join("config").join("packs.yaml"),
            format!("packs:\n  - {}\n", rig.listed.display()),
        )
        .expect("list the methodology copy over the dev base");
        // The composition is two packs, and the orientation header is where that is
        // observable — so the fixture proves its own premise before any cell runs.
        let orient = rig.run(&["start"]);
        let header = String::from_utf8_lossy(&orient.stdout).to_string();
        assert!(
            header.contains("Pack: methodology/") && header.contains(" | dev/"),
            "the fixture must compose TWO packs (methodology over dev); header was:\n{header}",
        );
        rig
    }

    fn run(&self, args: &[&str]) -> std::process::Output {
        Command::new(env!("CARGO_BIN_EXE_jigc"))
            .args(args)
            .current_dir(&self.repo)
            .env("HOME", self.home.path())
            .env("JIGC_PACK_DIR", &self.base)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .expect("run the jigc binary")
    }

    /// Delete `<kind>/<id>.yaml` from **both** composed packs, returning the bytes the
    /// base held so the route-following step can restore exactly them.
    fn break_resource(&self, kind: &str, id: &str) -> Vec<u8> {
        let rel = PathBuf::from(kind).join(format!("{id}.yaml"));
        let bytes = fs::read(self.base.join(&rel)).expect("the base pack must ship the resource");
        fs::remove_file(self.base.join(&rel)).expect("delete it from the base pack");
        let listed = self.listed.join(&rel);
        if listed.exists() {
            fs::remove_file(&listed).expect("delete it from the listed pack");
        }
        bytes
    }
}

/// One driven cell: the door's argv, the resource it must miss, and **the pack-set that
/// door actually searches**, as `<pack-id>/` prefixes in precedence order.
///
/// The set is per-cell because it is per-caller, and that is the corrected finding this
/// suite pins. `crate::start::load_catalog` is handed a **workflow's origin pack** — the
/// pack-local body-reference rule (`design/multi-pack.md`) — so a `config/commands` miss
/// there genuinely searched one pack, and naming the composite there would be the same
/// over-claim in the other direction. The producer names its receiver's own provenance, so
/// each door tells the truth about what *it* looked in, and no site needs a special case.
struct Cell<'a> {
    argv: &'a [&'a str],
    kind: &'a str,
    id: &'a str,
    searched: &'a [&'a str],
}

/// Drive one cell: break the resource, assert the refusal's head/identity/route, follow
/// the route **verbatim** off the emitted bytes, and re-drive the same door.
fn drive(rig: &Rig, cell: &Cell<'_>) {
    let saved = rig.break_resource(cell.kind, cell.id);
    let out = rig.run(cell.argv);
    let stderr = String::from_utf8(out.stderr).expect("utf-8 stderr");
    let argv = cell.argv;

    assert_eq!(
        out.status.code(),
        Some(1),
        "`jigc {}` must block at exit 1 on a missing `{}/{}`; stderr:\n{stderr}",
        argv.join(" "),
        cell.kind,
        cell.id,
    );

    // The head: the house findings line, so a text-scraping driver reads the same code
    // the (absent) envelope would have carried.
    let head = format!("blocking · {CODE} — ");
    let first = stderr.lines().next().unwrap_or_default();
    assert!(
        first.starts_with(&head),
        "`jigc {}` must lead with `{head}`; got:\n{stderr}",
        argv.join(" "),
    );

    // The message names the resource by its pack-relative id, never a `{:?}` enum.
    let resource = format!("`{}/{}`", cell.kind, cell.id);
    assert!(
        first.contains(&resource),
        "the message must name the missing resource {resource}; got:\n{stderr}",
    );

    // The packs ACTUALLY searched — each named by id/version AND resolving path, in
    // precedence order, and nothing else: a pack this door never looked in is as much a
    // lie as the embedded pack the literal used to blame.
    let roots = emitted_pack_roots(first);
    let expected: Vec<&Path> = cell
        .searched
        .iter()
        .map(|id| match *id {
            "methodology/" => rig.listed.as_path(),
            "dev/" => rig.base.as_path(),
            other => panic!("the cell names a pack the rig does not compose: {other}"),
        })
        .collect();
    for (id, root) in cell.searched.iter().zip(&expected) {
        assert!(
            first.contains(id),
            "the message must name the searched pack `{id}`; got:\n{stderr}",
        );
        assert!(
            root.is_dir(),
            "the pack root the message printed must be repo-real: {}",
            root.display(),
        );
    }
    assert_eq!(
        roots,
        expected
            .iter()
            .map(|p| p.display().to_string())
            .collect::<Vec<_>>(),
        "the message names exactly the pack roots this door searched, in precedence \
         order; got:\n{stderr}",
    );
    for absent in ["methodology/", "dev/"] {
        if !cell.searched.contains(&absent) {
            assert!(
                !first.contains(absent),
                "`{absent}` was never searched by this door and must not be named:\n{stderr}",
            );
        }
    }
    assert!(
        !stderr.contains("the embedded pack"),
        "no composed pack here is the embedded one — the refusal must not blame it:\n{stderr}",
    );

    // Exactly one route, and it names the file to restore and the pack list to fix.
    let routes: Vec<&str> = stderr
        .lines()
        .filter(|line| line.trim_start().starts_with("route: "))
        .collect();
    assert_eq!(
        routes.len(),
        1,
        "the refusal carries exactly one route; got {routes:?} in:\n{stderr}",
    );
    let route = routes[0];
    let file = format!("`{}/{}.yaml`", cell.kind, cell.id);
    assert!(
        route.contains(&file) && route.contains(".jigc/config/packs.yaml"),
        "the route must name {file} and `.jigc/config/packs.yaml`; got: {route}",
    );

    // **Follow the route verbatim.** The pack root comes out of the EMITTED message and
    // the file name out of the EMITTED route — nothing here is rebuilt from the fixture.
    // The **lowest-precedence** root is the one restored (the message orders highest
    // first), so a set that named only the precedence winner could not be followed.
    let root = roots.last().expect("the message must name a pack root");
    let named = first_backticked(route).expect("the route must name a file to restore");
    let dest = Path::new(root).join(&named);
    fs::create_dir_all(dest.parent().expect("the resource dir")).expect("the resource dir exists");
    fs::write(&dest, &saved).expect("restore the resource the route named");

    let after = rig.run(cell.argv);
    assert!(
        after.status.success(),
        "following the route must repair the state — `jigc {}` still failed:\n{}",
        argv.join(" "),
        String::from_utf8_lossy(&after.stderr),
    );
}

/// The first backticked span in `line` — the file the route names, read off the emitted
/// bytes so the repair uses the path the route actually printed.
fn first_backticked(line: &str) -> Option<String> {
    let open = line.find('`')? + 1;
    let rest = &line[open..];
    let close = rest.find('`')?;
    Some(rest[..close].to_owned())
}

/// The pack roots the message printed, in the order it printed them — each parenthesized
/// span, read off the emitted bytes rather than rebuilt from the fixture.
fn emitted_pack_roots(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = line;
    while let Some(open) = rest.find('(') {
        let after = &rest[open + 1..];
        let Some(close) = after.find(')') else { break };
        out.push(after[..close].to_owned());
        rest = &after[close + 1..];
    }
    out
}

#[test]
fn every_pack_resource_miss_names_the_searched_packs_with_a_code_and_a_route() {
    let rig = Rig::build();

    // Site 1 + 2 — `config.rs`'s two hand-copied reads. `config get`/`list` share the
    // first (`read_knobs`); `config set` has the second, and both said "the embedded
    // pack" over a composition holding no embedded pack.
    drive(
        &rig,
        &Cell {
            argv: &["config", "list"],
            kind: "config",
            id: "knobs",
            searched: &["methodology/", "dev/"],
        },
    );
    drive(
        &rig,
        &Cell {
            argv: &["config", "set", "docs-root", "docs"],
            kind: "config",
            id: "knobs",
            searched: &["methodology/", "dev/"],
        },
    );

    // Site 3 — the shared `start.rs::read_pack`, reached here through the command
    // catalog a `--workflow` compose loads. The re-drive mints the task site 4 needs.
    drive(
        &rig,
        &Cell {
            argv: &["start", "--workflow", "single-task", "probe intent"],
            kind: "config",
            id: "commands",
            // Scoped to `single-task`'s ORIGIN pack — the dev base — by the pack-local
            // body-reference rule, so the honest searched set here is one pack.
            searched: &["dev/"],
        },
    );

    // Site 4 — `doc.rs`'s create-gate read of the task's **recorded** workflow. The task
    // above was minted on `single-task`; deleting it from both packs is the shape a
    // de-listed pack produces in the field.
    assert!(
        rig.repo.join(".jigc/tasks/probe-intent/workflow").exists(),
        "the site-3 re-drive must have minted the task site 4 reads",
    );
    drive(
        &rig,
        &Cell {
            argv: &["doc", "create", "adr", "--title", "A Probe Decision"],
            kind: "workflows",
            id: "single-task",
            searched: &["methodology/", "dev/"],
        },
    );

    // The JSON posture the contract declares: these doors flatten through
    // `finding_to_err`, so the identity rides inside `{"error": …}` — carrying the same
    // head the text surface prints (`design/command-output-contract.md` → *outside the
    // envelope*, the fourth family).
    rig.break_resource("config", "knobs");
    let json = rig.run(&["config", "list", "--format", "json"]);
    let text = String::from_utf8(json.stdout).expect("utf-8 stdout")
        + &String::from_utf8(json.stderr).expect("utf-8 stderr");
    let value: serde_json::Value = serde_json::from_str(text.trim()).expect("a JSON envelope");
    let error = value
        .get("error")
        .and_then(|v| v.as_str())
        .expect("the operational-error envelope's `error` key");
    assert!(
        error.starts_with(&format!("blocking · {CODE} — ")),
        "the flattened refusal carries the code inside the envelope too; got: {error}",
    );
}
