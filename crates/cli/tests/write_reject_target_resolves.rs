//! A rejected write's `key.target` is **the write's own address**, in URI normal form, at
//! every break shape a staged source can carry (M49 completion audit).
//!
//! `command-output-contract.md` pins `(code, target)` as the stable driver key and declares
//! the `write.*` row's target to be *the write's own address, at the depth it names* —
//! `type:slug#<fragment>`. `engine::write::splice_error_finding` broke that for
//! `write.non-reparseable`: it cloned the carried parse finding's **whole** `Location`, and a
//! parse finding's address is a bare *fragment* (`header/bogus`, `trailers/BadAnchor`),
//! assembled outward by `parse::prefix_hop` and completed into URI normal form only by the
//! producer that holds the doc identity (`engine::finding::readdress_to_uri`, whose stated
//! precondition is exactly that its input is raw per-instance parse output). The write seam is
//! not that producer — the CLI is, and it stamps the write's address outward
//! (`cli::doc::stamp_target`) — but that stamp is deliberately *if-absent*, so the hitch-hiking
//! fragment survived it and shipped as the key:
//!
//! ```text
//! $ jigc doc set-slot 'commit:add-a-cache#body' … --format json
//!   code  : write.non-reparseable        target: 'header/bogus'
//! $ jigc doc show 'header/bogus' --task add-a-cache
//!   -> malformed address `header/bogus`: missing ':' between type and slug …
//! ```
//!
//! **The reported repro named one branch; the axis is every break `prefix_hop` reaches** —
//! which is every break raised inside a section, i.e. nearly all of them. The one shape that
//! emitted the contract's form was the break `prefix_hop` never touches. So this suite
//! iterates the **hop depths the outward assembly produces** — the section hop, the section +
//! field-key hop, the section + item hop — crossed with the write verbs that funnel through
//! the seam, and asserts of every cell that the emitted target is the write's own address and
//! that **pasting it back into a read verb resolves**. What the read then says about a
//! genuinely broken doc is not this suite's business; that it can be *addressed at all* is.
//!
//! The locus the carry existed for is pinned too: `line` still points at the offending line.
//! Dropping the address must not cost the coordinate.
//!
//! The last arm pins the seam itself, so a future `SpliceError` variant that starts carrying
//! findings cannot re-open the class by cloning a location again.
//!
//! Drives the REAL binary — the emitted bytes are the contract.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-write-target-{tag}-{}-{:?}",
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

/// Run `git <args>` in `cwd`, asserting success.
fn git_ok(cwd: &Path, args: &[&str]) {
    let out = Command::new("git")
        .args(args)
        .current_dir(cwd)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} in {cwd:?} failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
}

/// The task the fixture mints — `jigc start` slugs the intent.
const TASK: &str = "add-a-cache";

/// A repo with one task open, its transient `commit:<task>` doc staged in the workbench.
struct Fixture {
    _root: TempDir,
    home: TempDir,
    repo: PathBuf,
    /// The staged `commit:<task>` working copy every corruption is applied to.
    staged: PathBuf,
    /// The pristine bytes, restored before each corruption.
    pristine: String,
}

impl Fixture {
    fn new() -> Self {
        let root = TempDir::new("root");
        let repo = root.path().join("repo");
        fs::create_dir_all(&repo).expect("mk repo");
        git_ok(&repo, &["init", "-q"]);
        git_ok(&repo, &["config", "user.email", "test@example.com"]);
        git_ok(&repo, &["config", "user.name", "Test"]);
        git_ok(&repo, &["config", "commit.gpgsign", "false"]);
        fs::write(repo.join("README.md"), "hello\n").expect("write README");
        git_ok(&repo, &["add", "."]);
        git_ok(&repo, &["commit", "-q", "-m", "initial"]);

        let home = TempDir::new("home");
        let fx = Fixture {
            _root: root,
            home,
            repo,
            staged: PathBuf::new(),
            pristine: String::new(),
        };
        let setup = fx.run(&["setup"], None);
        assert!(
            setup.status.success(),
            "`jigc setup` must install the cascade; stderr:\n{}",
            String::from_utf8_lossy(&setup.stderr),
        );
        let started = fx.run(&["start", "--workflow", "quick-fix", "Add a cache"], None);
        assert!(
            started.status.success(),
            "`jigc start` must mint the task; stderr:\n{}",
            String::from_utf8_lossy(&started.stderr),
        );
        let staged = fx
            .repo
            .join(".jigc")
            .join("tasks")
            .join(TASK)
            .join("docs")
            .join(format!("commit:{TASK}.md"));
        let pristine = fs::read_to_string(&staged).expect("the commit doc is staged");
        Fixture {
            staged,
            pristine,
            ..fx
        }
    }

    fn run(&self, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
        use std::io::Write;
        use std::process::Stdio;
        let mut child = Command::new(env!("CARGO_BIN_EXE_jigc"))
            .args(args)
            .current_dir(&self.repo)
            .env("HOME", self.home.path())
            .env_remove("JIGC_PACK_DIR")
            .stdin(if stdin.is_some() {
                Stdio::piped()
            } else {
                Stdio::null()
            })
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("run the jigc binary");
        if let Some(bytes) = stdin {
            child
                .stdin
                .as_mut()
                .expect("stdin piped")
                .write_all(bytes)
                .expect("write stdin");
        }
        child.wait_with_output().expect("collect jigc output")
    }

    /// Restore the pristine staged doc, then apply `corrupt` to it, returning the 1-based
    /// line the reject is expected to point at — **read back off the written bytes**, so the
    /// locus assertion is derived from the fixture rather than hand-counted.
    fn corrupt_with(&self, corrupt: &Corruption) -> usize {
        let broken = (corrupt.apply)(&self.pristine);
        assert!(
            broken.contains(corrupt.planted),
            "corruption `{}` must plant `{}`; got:\n{broken}",
            corrupt.name,
            corrupt.planted,
        );
        fs::write(&self.staged, &broken).expect("write the corrupted staged doc");
        broken
            .lines()
            .position(|line| line.starts_with(corrupt.locus_prefix))
            .map(|zero_based| zero_based + 1)
            .unwrap_or_else(|| {
                panic!(
                    "corruption `{}` must carry its locus line `{}`; got:\n{broken}",
                    corrupt.name, corrupt.locus_prefix,
                )
            })
    }
}

/// One break shape a staged source can carry, named by the **address hop**
/// `engine::parse::prefix_hop` assembles for it — the axis this suite iterates.
struct Corruption {
    /// The hop depth, as the outward assembly builds it.
    name: &'static str,
    /// Pristine bytes in, broken bytes out.
    apply: fn(&str) -> String,
    /// A marker proving the corruption landed in the written bytes.
    planted: &'static str,
    /// The prefix of the line the reject is expected to point at, found in the written file
    /// so the expected locus is **derived from the fixture**, never counted by hand. It is
    /// not always the planted line: a field-group diagnostic locates at the group's base
    /// line, which is the parser's own long-standing granularity and not this fix's subject.
    locus_prefix: &'static str,
}

/// The hop depths, in the order `prefix_hop` assembles them: the section hop alone, the
/// section + field-key hop, and the section + item hop. Each is a real corruption an agent
/// or a human editor can leave behind, not a synthesized `Finding`.
const CORRUPTIONS: &[Corruption] = &[
    Corruption {
        // `conformance.section-renamed` — addressed at the bare section hop (`body`).
        name: "section hop",
        apply: |src| src.replace("## Body", "## Bodyz"),
        planted: "## Bodyz",
        locus_prefix: "## Bodyz",
    },
    Corruption {
        // `conformance.unknown-field` — the field-key hop under the section (`header/bogus`).
        // Located at the field group's base line, which is the first field bullet.
        name: "section + field-key hop",
        apply: |src| src.replacen("scope:", "bogus: x\nscope:", 1),
        planted: "bogus: x",
        locus_prefix: "type:",
    },
    Corruption {
        // `conformance.item-anchor-malformed` — the item hop under the section
        // (`trailers/BadAnchor`): an uppercase anchor is not a slug.
        name: "section + item hop",
        apply: |src| format!("{src}\n### Co-Authored-By  {{#BadAnchor}}\n"),
        planted: "{#BadAnchor}",
        locus_prefix: "### Co-Authored-By",
    },
];

/// One write verb that funnels through `splice_error_finding`, with the address it names.
struct Write {
    argv: &'static [&'static str],
    /// The write's own address, `{task}`-templated — what the contract declares the target
    /// to be.
    address: &'static str,
    stdin: Option<&'static [u8]>,
}

const WRITES: &[Write] = &[
    Write {
        argv: &["doc", "set-slot", "{addr}", "--from-file", "-"],
        address: "commit:{task}#summary",
        stdin: Some(b"A subject line.\n"),
    },
    Write {
        argv: &["doc", "set-field", "{addr}", "--value", "feat"],
        address: "commit:{task}#header/type",
        stdin: None,
    },
];

/// The blocking finding a `--format json` write reject puts on stderr.
fn sole_finding(out: &std::process::Output) -> serde_json::Value {
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    let report: serde_json::Value = serde_json::from_str(stderr.trim())
        .unwrap_or_else(|e| panic!("stderr must be the JSON envelope ({e}); got:\n{stderr}"));
    report["findings"][0].clone()
}

#[test]
fn every_break_shape_targets_the_writes_own_address_and_it_resolves() {
    let fx = Fixture::new();

    for corrupt in CORRUPTIONS {
        let break_line = fx.corrupt_with(corrupt);

        for write in WRITES {
            let address = write.address.replace("{task}", TASK);
            let argv: Vec<String> = write
                .argv
                .iter()
                .map(|arg| arg.replace("{addr}", &address))
                .chain([
                    "--task".into(),
                    TASK.into(),
                    "--format".into(),
                    "json".into(),
                ])
                .collect();
            let args: Vec<&str> = argv.iter().map(String::as_str).collect();
            let out = fx.run(&args, write.stdin);
            assert!(
                !out.status.success(),
                "[{} / {}] a write over a non-parsing staged source must block; stdout:\n{}",
                corrupt.name,
                address,
                String::from_utf8_lossy(&out.stdout),
            );
            let finding = sole_finding(&out);
            assert_eq!(
                finding["code"], "write.non-reparseable",
                "[{} / {address}] the broken staged source blocks the write; got:\n{finding:#}",
                corrupt.name,
            );

            // The claim: the key's target is the write's OWN address, in URI normal form.
            assert_eq!(
                finding["key"]["target"],
                serde_json::Value::String(address.clone()),
                "[{} / {address}] the target is the write's own address, never the parse \
                 break's bare fragment; got:\n{finding:#}",
                corrupt.name,
            );

            // The locus the carry exists for survives dropping the address.
            assert_eq!(
                finding["location"]["line"],
                serde_json::Value::from(break_line),
                "[{} / {address}] the reject still points at the offending line; got:\n{finding:#}",
                corrupt.name,
            );

            // And the whole point of a stable key: it can be pasted back into a read verb.
            let target = finding["key"]["target"]
                .as_str()
                .expect("the target is a string")
                .to_owned();
            let read = fx.run(&["doc", "show", &target, "--task", TASK], None);
            let read_err = String::from_utf8_lossy(&read.stderr).into_owned();
            assert!(
                !read_err.contains("malformed address"),
                "[{} / {address}] the emitted target must be an address a read verb can \
                 resolve; `jigc doc show {target}` said:\n{read_err}",
                corrupt.name,
            );
        }
    }
}

/// The seam itself, so the class cannot re-open through a *different* `SpliceError` variant
/// that starts carrying parse findings: a carried break contributes its **coordinate**, never
/// its address. (The CLI-side half is fenced by `cli::doc::stamp_target`'s debug assertion,
/// which every write-path test in the suite exercises.)
#[test]
fn the_splice_seam_carries_the_coordinate_not_the_fragment() {
    let carried = engine::finding::Finding::blocking(
        "conformance.unknown-field",
        "unknown field key `bogus`",
        engine::finding::Location::addressed("header/bogus", 7, 3),
    );
    let finding = engine::write::splice_error_finding(&engine::write::SpliceError::NotConformant {
        findings: vec![carried],
    });
    assert_eq!(finding.code, "write.non-reparseable");
    let location = finding
        .location
        .as_ref()
        .expect("a carried break locates the reject");
    assert_eq!(
        (location.line, location.col),
        (7, 3),
        "the coordinate is what the carry is for",
    );
    assert_eq!(
        location.address, None,
        "the parse break's bare fragment must not become the write finding's target — the \
         CLI stamps the write's own address over an absent one, and only over an absent one",
    );
}
