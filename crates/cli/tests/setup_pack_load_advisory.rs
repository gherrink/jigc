//! M50 Increment 12 / T3 — **`setup` says what the next door will refuse, and still refuses
//! nothing** (RC-m50 → D5, fork 7; `completions/artifacts/M50/settle-record.md` → D5;
//! `design/project-setup.md` → What the install says about the corpus it installed into;
//! `design/corpus-migration.md` → The freeze).
//!
//! The pre-v1 baseline drove it: a repo carrying a **shape-changing project schema shadow**
//! (`.jigc/config/schemas/<ty>.yaml`) gets `jigc setup` at **exit 0, silent** — and then
//! `describe` / `doc schema` / `validate` / `start` / `doc list` all exit 1 with the pack-load
//! freeze block. The install is the one door that met the state and said nothing about it.
//!
//! **The class is the pack LOAD, not the freeze.** Driven while building this suite: a
//! *malformed* shadow reaches the identical exit-0 silence, and the block it produces at the
//! next door is code-less and route-less — so the advisory owes a route of its own rather than
//! relaying one. Both failing shapes are therefore arms here, and what the advisory names is
//! the **pack set failing to load**, whichever fence refused it.
//!
//! **The declared bound stands, and is asserted rather than assumed** (D5): the bootstrap door
//! does **not** itself refuse. Every arm asserts `setup` exits 0 *and* that the install landed
//! (the adapter's bootstrap reference is on disk), so the advisory can never be mistaken for a
//! half-abort. The warning rides the **existing** `findings` key — an already-shipped slot on
//! both surfaces, not new contract surface.
//!
//! **The set this suite iterates is a manufactured one, and says so.** There is no code-side
//! registry of project-layer states; the axis is *what a project layer can be* at the moment
//! `setup` runs — it loads, it is malformed, or it loads and violates a fence — so the three
//! cells are written out as [`Layer`], the deliberate departure M49's flow-50 arm 1 records.
//! Each failing cell is proven to be the state it claims by the **production loader** rather
//! than by the fixture's construction: the shape-changing shadow must parse and hash
//! differently, the malformed one must fail to parse. And each failing arm drives the **next
//! door** (`jigc validate`) and asserts it really does refuse — an advisory that forecasts a
//! refusal that does not come is the law-1 lie this task exists to close.
//!
//! The fixture is the **`bare`** state: a git repo with one commit and no `jigc setup`, the
//! only state the install itself can be probed from. It is built here rather than taken from
//! `support::trial_corpus`, whose every state has already run `setup`.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use cli::pack::EmbeddedPack;
use engine::packsource::{PackResourceKind, PackSource, ResourceId};

/// The finding code the install raises over a pack set that will not load — read from the
/// producer, so a renamed code reddens the production seam rather than this literal.
const CODE: &str = cli::setup::PACK_LOAD_CODE;

/// The doctype whose project-layer shadow the two failing arms plant. `adr` is a
/// manifest-governed dev-pack doctype, so a shape change to it is exactly what the freeze
/// refuses.
const SHADOWED: &str = "adr";

/// The project-layer home of that shadow (`design/overrides.md` → Authored metadata on a
/// definition resolves by whole-file shadow).
const SHADOW_PATH: &str = ".jigc/config/schemas/adr.yaml";

// ---------------------------------------------------------------------------------
// Fixture
// ---------------------------------------------------------------------------------

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-setup-packload-{tag}-{}-{:?}",
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

/// The `bare` state: a git repo with one commit and a usable identity, no `jigc setup`.
fn bare_repo(root: &Path) {
    git(root, &["init", "-q"]);
    git(root, &["config", "user.email", "test@example.com"]);
    git(root, &["config", "user.name", "Test"]);
    git(root, &["config", "commit.gpgsign", "false"]);
    fs::write(root.join("README.md"), "bare\n").expect("write README.md");
    git(root, &["add", "."]);
    git(root, &["commit", "-q", "-m", "initial"]);
}

fn git(root: &Path, args: &[&str]) {
    let out = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_CONFIG_SYSTEM", "/dev/null")
        .output()
        .unwrap_or_else(|err| panic!("run git {args:?}: {err}"));
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
}

/// Run the built binary with `cwd = repo` and `$HOME = home`.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .env_remove("JIGC_ADAPTERS_DIR")
        .output()
        .expect("run the jigc binary")
}

/// The shipped `adr` schema's bytes — the base both shadows are built from, read from the
/// embedded pack rather than retyped, so a reworded schema cannot leave this fixture
/// describing a doctype that no longer exists.
fn shipped_adr() -> String {
    let pack = EmbeddedPack::new();
    let bytes = pack
        .read(PackResourceKind::Schemas, &ResourceId::from(SHADOWED))
        .expect("the embedded pack ships the `adr` schema");
    String::from_utf8(bytes).expect("the shipped schema is UTF-8")
}

// ---------------------------------------------------------------------------------
// The manufactured state set
// ---------------------------------------------------------------------------------

/// What the project layer is when `setup` runs. Manufactured, not read from a registry —
/// see the module banner.
#[derive(Clone, Copy, PartialEq)]
enum Layer {
    /// No project layer at all — the ordinary adopter.
    Clean,
    /// A whole-file schema shadow that parses and **changes the doctype's shape**: what the
    /// freeze refuses at every layer.
    ShapeChanging,
    /// A whole-file schema shadow that does not parse: the same exit-0 silence, reached
    /// through the loader rather than the freeze.
    Malformed,
}

impl Layer {
    const ALL: &'static [Layer] = &[Layer::Clean, Layer::ShapeChanging, Layer::Malformed];

    fn name(self) -> &'static str {
        match self {
            Layer::Clean => "clean",
            Layer::ShapeChanging => "shape-changing",
            Layer::Malformed => "malformed",
        }
    }

    /// Whether the next door refuses over this layer — the fact the advisory forecasts.
    fn next_door_refuses(self) -> bool {
        self != Layer::Clean
    }

    /// Plant the layer into `repo`, proving through the **production loader** that the
    /// planted bytes really are the state this cell claims.
    fn plant(self, repo: &Path) {
        if self == Layer::Clean {
            return;
        }
        let pack = EmbeddedPack::new();
        let base = shipped_adr();
        let text = match self {
            Layer::Clean => unreachable!(),
            // A shape change: one more section than the frozen schema declares.
            Layer::ShapeChanging => format!(
                "{base}  - id: postscript\n    slot: {{ hint: \"An extra section the freeze never saw.\" }}\n"
            ),
            // Bytes that are not YAML at all.
            Layer::Malformed => format!("{base}  - id: broken\n    slot: {{ hint: [unclosed\n"),
        };
        let path = repo.join(SHADOW_PATH);
        fs::create_dir_all(path.parent().expect("the shadow has a parent"))
            .expect("create the project schema dir");
        fs::write(&path, &text).expect("write the project schema shadow");

        // The fixture's own premise, checked rather than believed.
        let loaded = cli::pack::load_pack_schema(&pack, text.as_bytes());
        match self {
            Layer::ShapeChanging => {
                let shadow = loaded.expect(
                    "the shape-changing arm's fixture must PARSE — if it does not, this arm \
                     is silently running the malformed cell twice",
                );
                let pristine = cli::pack::load_pack_schema(&pack, base.as_bytes())
                    .expect("the shipped schema parses");
                assert_ne!(
                    engine::manifest::schema_hash(&shadow),
                    engine::manifest::schema_hash(&pristine),
                    "the shape-changing arm's fixture must move the `schema-hash` — a shadow \
                     the presentation projection erases would not reach the freeze at all",
                );
            }
            Layer::Malformed => assert!(
                loaded.is_err(),
                "the malformed arm's fixture must fail to parse",
            ),
            Layer::Clean => unreachable!(),
        }
    }
}

// ---------------------------------------------------------------------------------
// The arms
// ---------------------------------------------------------------------------------

/// Every cell of the manufactured set, driven through the real binary from `bare`.
#[test]
fn setup_forecasts_the_next_door_over_every_project_layer_state() {
    for layer in Layer::ALL {
        let dir = TempDir::new(layer.name());
        let repo = dir.path().join("repo");
        let home = dir.path().join("home");
        fs::create_dir_all(&repo).expect("create repo dir");
        fs::create_dir_all(&home).expect("create home dir");
        bare_repo(&repo);
        layer.plant(&repo);

        // --- the install itself: exit 0, and it really installed ----------------------
        let out = jigc(&repo, &home, &["setup"]);
        assert!(
            out.status.success(),
            "[{}] `jigc setup` must exit 0 — the bootstrap door does not refuse; got {:?}\n\
             stderr:\n{}",
            layer.name(),
            out.status,
            String::from_utf8_lossy(&out.stderr),
        );
        assert!(
            repo.join("CLAUDE.md").exists() && repo.join(".jigc/AGENT.md").exists(),
            "[{}] the install must have LANDED, not half-aborted",
            layer.name(),
        );
        let text = String::from_utf8_lossy(&out.stdout).into_owned();

        // --- the forecast, and the door it forecasts ----------------------------------
        let validate = jigc(&repo, &home, &["validate"]);
        assert_eq!(
            !validate.status.success(),
            layer.next_door_refuses(),
            "[{}] the fixture's own premise: the next door must {} over this layer; \
             `jigc validate` gave {:?}\nstderr:\n{}",
            layer.name(),
            if layer.next_door_refuses() {
                "refuse"
            } else {
                "pass"
            },
            validate.status,
            String::from_utf8_lossy(&validate.stderr),
        );

        // --- what `setup` said about it ------------------------------------------------
        let json_out = jigc(&repo, &home, &["--format", "json", "setup"]);
        assert!(
            json_out.status.success(),
            "[{}] `jigc --format json setup` must exit 0 too",
            layer.name(),
        );
        let value: serde_json::Value = serde_json::from_slice(&json_out.stdout)
            .unwrap_or_else(|err| panic!("[{}] setup's JSON must parse: {err}", layer.name()));
        let findings = value["findings"]
            .as_array()
            .unwrap_or_else(|| panic!("[{}] setup's JSON must carry `findings`", layer.name()));

        if !layer.next_door_refuses() {
            assert!(
                findings.is_empty(),
                "[{}] a clean project layer must leave `findings` empty; got {findings:?}",
                layer.name(),
            );
            assert!(
                !text.contains(CODE),
                "[{}] a clean install must say nothing about the pack set; got:\n{text}",
                layer.name(),
            );
            continue;
        }

        let ours: Vec<_> = findings.iter().filter(|f| f["code"] == CODE).collect();
        assert_eq!(
            ours.len(),
            1,
            "[{}] exactly one `{CODE}` advisory; got {findings:?}",
            layer.name(),
        );
        let finding = ours[0];
        assert_eq!(
            finding["severity"],
            "advisory",
            "[{}] the forecast is advisory — the install did not fail",
            layer.name(),
        );
        let message = finding["message"].as_str().expect("a message");
        assert!(
            message.contains(SHADOW_PATH),
            "[{}] the advisory must NAME the shadow; got `{message}`",
            layer.name(),
        );
        let route = finding["route"]
            .as_str()
            .unwrap_or_else(|| panic!("[{}] the advisory must carry a route", layer.name()));
        assert!(
            !route.trim().is_empty(),
            "[{}] the route must say something",
            layer.name(),
        );

        // The agent text carries the same advisory, through the house finding line.
        assert!(
            text.contains(CODE) && text.contains(SHADOW_PATH) && text.contains("  route: "),
            "[{}] the agent text must carry the advisory and its route; got:\n{text}",
            layer.name(),
        );
    }
}
