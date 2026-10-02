//! **N29** — a finding raised inside a *nested* repeatable carries the nested
//! repeatable's own hop, so the address it composes is one the tool's own grammar
//! accepts and its own read surface resolves.
//!
//! `design/command-output-contract.md` → the parse-conformance sub-table declares each
//! `conformance.*` code's target to be the address of the locus that raised it, assembled
//! outward hop by hop (`engine::parse::prefix_hop`). `engine::validate`'s adjudication
//! path already obeys that at the nested locus — `check_item_leaves` addresses a nested
//! child at `…/parent/<nested-section>/child/leaf`, "the canonical form an agent types and
//! `set-slot`/`add-item` emit" — but the **parse** path did not: `parse_items` recursed
//! into each declared nested repeatable and appended its findings straight into the parent
//! item's set, prefixing only the parent item's hop. The nested repeatable's own leaf id
//! (`changes`) was never a hop, while the comment three lines below asserted the
//! composition `<item>/<nested>/<leaf>`.
//!
//! The break is reachable on the stock shipped `changelog` at HEAD, and it terminates the
//! adopter's route in the engine source tree — the address the tool prints cannot be
//! pasted back into the tool:
//!
//! ```text
//! $ jigc validate --format json
//!   code: conformance.unknown-field  target: 'changelog:changelog#releases/1-0-0/added/bogus'
//! $ jigc doc show 'changelog:changelog#releases/1-0-0/added'      # on a clean corpus
//!   -> blocking · store.no-such-leaf … (exit 1)
//! $ jigc doc show 'changelog:changelog#releases/1-0-0/changes/added'
//!   -> resolves (exit 0)
//! ```
//!
//! **The axis is the recursion, not the changelog.** The fix prefixes the nested
//! repeatable's leaf id at the recursion site, so it holds for every declared nested
//! repeatable at every depth; this suite drives the one nesting doctype that ships, at the
//! three hop depths a nested item's body can raise a finding from — the nested item's own
//! anchor, a leaf inside it, and a bullet key inside it.
//!
//! Every composed address is checked three ways, never by string compare alone: exact
//! equality with the contract's form, a round-trip through `engine::address::Address::parse`
//! (the grammar the write verbs enforce), and resolution through `jigc doc show` on the
//! **clean** committed doc — a doc carrying the planted break does not parse, so the
//! resolution is proven against the corpus the address is meant to name. That last check is
//! the one that discriminates: the pre-fix address `#releases/1-0-0/added` is grammatically
//! fine and resolves to nothing.
//!
//! Drives the REAL binary — the emitted bytes are the contract.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-nested-finding-{tag}-{}-{:?}",
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

/// The embedded dev pack tree on disk — selected via `JIGC_PACK_DIR` so the binary
/// composes the exact bytes it ships.
fn dev_pack() -> PathBuf {
    Path::new(cli::pack_path!(dev)).to_path_buf()
}

/// A repo whose committed `CHANGELOG.md` carries one release with one nested change-group.
struct Fixture {
    _root: TempDir,
    home: TempDir,
    repo: PathBuf,
    /// The committed changelog's pristine bytes, restored before each planted break.
    pristine: String,
}

impl Fixture {
    fn run(&self, args: &[&str], stdin: Option<&[u8]>) -> Output {
        let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
        command
            .args(args)
            .current_dir(&self.repo)
            .env("HOME", self.home.path())
            .env("JIGC_PACK_DIR", dev_pack())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        if stdin.is_some() {
            command.stdin(Stdio::piped());
        }
        let mut child = command.spawn().expect("spawn the jigc binary");
        if let Some(bytes) = stdin {
            crate::support::child_stdin::feed(&mut child, bytes);
        }
        child.wait_with_output().expect("wait for jigc")
    }

    /// Run a `jigc` subcommand that must exit 0, returning its trimmed stdout.
    fn ok(&self, args: &[&str], stdin: Option<&[u8]>) -> String {
        let out = self.run(args, stdin);
        assert!(
            out.status.success(),
            "`jigc {}` must exit 0; stdout:\n{}\nstderr:\n{}",
            args.join(" "),
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        );
        String::from_utf8(out.stdout)
            .expect("utf-8 stdout")
            .trim_end_matches('\n')
            .to_owned()
    }

    fn changelog(&self) -> PathBuf {
        self.repo.join("CHANGELOG.md")
    }

    /// Restore the pristine committed changelog — the state every arm starts from.
    fn restore(&self) {
        fs::write(self.changelog(), &self.pristine).expect("restore CHANGELOG.md");
    }

    fn new() -> Self {
        let root = TempDir::new("root");
        let repo = root.path().join("repo");
        fs::create_dir_all(&repo).expect("mk repo");
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
        git(&["config", "commit.gpgsign", "false"]);
        fs::write(repo.join("README.md"), "hello\n").expect("write README");
        git(&["add", "."]);
        git(&["commit", "-q", "-m", "initial"]);

        let fx = Fixture {
            _root: root,
            home: TempDir::new("home"),
            repo,
            pristine: String::new(),
        };
        fx.ok(&["setup"], None);
        fx.ok(&["start", "--workflow", "record-change", "cut 1.0.0"], None);
        let task = "cut-1-0-0";
        fx.ok(
            &["doc", "create", "changelog", "--title", "Changelog"],
            None,
        );
        let release = fx.ok(
            &[
                "doc",
                "add-item",
                "changelog:changelog#releases",
                "--title",
                "1.0.0",
            ],
            None,
        );
        // Every downstream address is driven from the EMITTED one, never rebuilt here.
        let group = fx.ok(
            &[
                "doc",
                "add-item",
                &format!("{release}/changes"),
                "--title",
                "added",
            ],
            None,
        );
        assert_eq!(
            group, RELEASE_GROUP,
            "the binary's own nested-group address is what this suite iterates against",
        );
        fx.ok(
            &[
                "doc",
                "set-slot",
                &format!("{group}/notes"),
                "--from-file",
                "-",
            ],
            Some(b"- OAuth device-code flow\n"),
        );
        for (key, value) in [("type", "docs"), ("scope", "changelog")] {
            fx.ok(
                &[
                    "doc",
                    "set-field",
                    &format!("commit:{task}#{key}"),
                    "--value",
                    value,
                ],
                None,
            );
        }
        fx.ok(
            &[
                "doc",
                "set-slot",
                &format!("commit:{task}#summary"),
                "--from-file",
                "-",
            ],
            Some(b"cut 1.0.0\n"),
        );
        fx.ok(&["task", "finalize", task], None);
        let pristine = fs::read_to_string(fx.changelog()).expect("the changelog is committed");
        Fixture { pristine, ..fx }
    }
}

/// The nested change-group the fixture commits — the locus every arm plants inside.
const RELEASE_GROUP: &str = "changelog:changelog#releases/1-0-0/changes/added";

/// One hop depth inside a nested repeatable, named by the address `prefix_hop` assembles
/// for it. Each is a real corruption a human editor or a foreign tool can leave in a
/// committed file, not a synthesized `Finding`.
struct NestedHop {
    /// The hop depth, as the outward assembly builds it.
    name: &'static str,
    /// Pristine committed bytes in, broken bytes out.
    apply: fn(&str) -> String,
    /// A marker proving the corruption landed in the written bytes.
    planted: &'static str,
    /// The `conformance.*` code the break raises.
    code: &'static str,
    /// The address `jigc validate` must emit for it, in URI normal form.
    target: &'static str,
    /// The **declared** address whose resolution proves the composition, driven through
    /// `jigc doc show` on the clean doc. It is the target itself wherever the target names
    /// something the schema declares; for an *undeclared* bullet key it is the item that
    /// contains it, since an undeclared key names nothing to resolve by construction — and
    /// that container is exactly the prefix the missing hop corrupted.
    resolves: &'static str,
}

const NESTED_HOPS: &[NestedHop] = &[
    NestedHop {
        // The done-criterion's arm: an undeclared bullet key inside the nested item. The
        // change-group template declares NO bullet fields (`category` is the heading-derived
        // id source, `notes` a slot), so the group is read against an empty declared set and
        // any key in it is unknown (M40 triage).
        name: "nested item + field-key hop",
        apply: |src| format!("{src}\n<!-- fields -->\n- bogus: x\n"),
        planted: "- bogus: x",
        code: "conformance.unknown-field",
        target: "changelog:changelog#releases/1-0-0/changes/added/bogus",
        resolves: RELEASE_GROUP,
    },
    NestedHop {
        // The nested item's own hop: a second `#### added  {#added}` under the same release
        // duplicates an anchor whose uniqueness is parent-scoped.
        name: "nested item hop",
        apply: |src| format!("{src}\n#### added  {{#added}}\n\n- a second group\n"),
        planted: "- a second group",
        code: "conformance.item-anchor-duplicate",
        target: RELEASE_GROUP,
        resolves: RELEASE_GROUP,
    },
    NestedHop {
        // A leaf hop *under* the nested item: the change-group is single-slot, so a ceiling
        // violation in its prose carries the owning slot's hop (`notes`) before the item's.
        name: "nested item + leaf hop",
        apply: |src| format!("{src}\nA Setext heading\n================\n"),
        planted: "================",
        code: "conformance.slot-setext-heading",
        target: "changelog:changelog#releases/1-0-0/changes/added/notes",
        resolves: "changelog:changelog#releases/1-0-0/changes/added/notes",
    },
];

/// The blocking findings `jigc validate --format json` puts on the envelope.
fn findings(out: &Output) -> Vec<serde_json::Value> {
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    let body = if stdout.trim().is_empty() {
        stderr
    } else {
        stdout
    };
    let report: serde_json::Value = serde_json::from_str(body.trim()).unwrap_or_else(|e| {
        panic!("`jigc validate --format json` must emit the envelope ({e}); got:\n{body}")
    });
    report["findings"].as_array().cloned().unwrap_or_default()
}

#[test]
fn every_finding_raised_inside_a_nested_repeatable_carries_the_nested_sections_hop() {
    let fx = Fixture::new();

    for hop in NESTED_HOPS {
        // (1) The address this arm claims resolves — proven on the CLEAN doc, because a doc
        //     carrying the break does not parse and `doc show` would answer about the break
        //     instead of the address.
        fx.restore();
        let show = fx.run(&["doc", "show", hop.resolves], None);
        assert!(
            show.status.success(),
            "[{}] the address the finding composes must resolve on the clean corpus; \
             `jigc doc show {}` said:\n{}",
            hop.name,
            hop.resolves,
            String::from_utf8_lossy(&show.stderr),
        );

        // (2) Plant the break in the committed file.
        let broken = (hop.apply)(&fx.pristine);
        assert!(
            broken.contains(hop.planted),
            "[{}] the corruption must plant `{}`; got:\n{broken}",
            hop.name,
            hop.planted,
        );
        fs::write(fx.changelog(), &broken).expect("write the corrupted changelog");

        // (3) The address `jigc validate` emits for it.
        let out = fx.run(&["validate", "--format", "json"], None);
        let all = findings(&out);
        let finding = all
            .iter()
            .find(|f| f["code"] == hop.code)
            .unwrap_or_else(|| {
                panic!(
                    "[{}] `jigc validate` must raise `{}`; got:\n{}",
                    hop.name,
                    hop.code,
                    serde_json::to_string_pretty(&all).expect("render findings"),
                )
            });
        assert_eq!(
            finding["key"]["target"],
            serde_json::Value::String(hop.target.to_string()),
            "[{}] the finding is addressed at its nested hop; got:\n{finding:#}",
            hop.name,
        );

        // (4) The emitted address is one the tool's own grammar accepts, byte-identical.
        let emitted = finding["key"]["target"]
            .as_str()
            .expect("the target is a string");
        let parsed = engine::address::Address::parse(emitted).unwrap_or_else(|e| {
            panic!(
                "[{}] the emitted address must parse against the grammar the write verbs \
                 enforce; `{emitted}` said: {e:?}",
                hop.name,
            )
        });
        assert_eq!(
            parsed.to_string(),
            emitted,
            "[{}] the emitted address round-trips through the grammar",
            hop.name,
        );
    }
}

/// The discriminator, stated on its own so a regression cannot pass by composing *some*
/// address: the pre-fix form — the parent item's hop with the nested repeatable's hop
/// missing — is grammatically fine and names nothing. A tool that prints it has routed its
/// reader nowhere.
#[test]
fn the_hop_less_form_parses_and_resolves_to_nothing() {
    let fx = Fixture::new();
    let hopless = "changelog:changelog#releases/1-0-0/added";
    engine::address::Address::parse(hopless)
        .expect("the hop-less form is grammatically fine — that is why it went unnoticed");

    let out = fx.run(&["doc", "show", hopless], None);
    assert!(
        !out.status.success(),
        "the hop-less form must not resolve; `jigc doc show {hopless}` exited 0 with:\n{}",
        String::from_utf8_lossy(&out.stdout),
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("store.no-such-leaf"),
        "the hop-less form names no leaf on the clean corpus; got:\n{stderr}",
    );
}
