//! **EC-13 — the 1.0 read contract's declared conformance witness is fenced against the
//! shape it witnesses** (M51 Increment 7 / T4).
//!
//! [`doc-read-surface.md`](../../../design/doc-read-surface.md):5 and `:57` name the
//! **milestone record** as the conformance witness of the pinned `jigc doc show
//! --format json` contract — *"the machine-consumed record whose shape pins the
//! contract"*, *"its shape is the pin's proof"*. The witness itself is enumerated once,
//! at [`team-ready-state.md`](../../../design/team-ready-state.md) → The read surface.
//!
//! At this suite's mint that enumeration was stale in **both** of its key sets, and
//! nothing read it: the binary emits **six** top-level keys (`fields`, `item-count`,
//! `schema-version`, `sections`, `slug`, `type`) where the witness named four, and
//! **five** item keys (`id`, `intent`, `status`, `task-id`, `workflow`) where it named
//! three. Every extra key is correctly declared at `doc-read-surface.md:59/68/69` and
//! ledgered at `:88` — **only the proof was wrong**, which is the worst place for it to
//! be: a witness that under-states the shape it witnesses certifies a contract narrower
//! than the one that ships.
//!
//! **The comparand is the binary, never a literal.** A fixture written into this file
//! would be a *third* hand-maintained home for the same fact and would drift exactly as
//! the first two did. So the suite mints a real milestone record by driving the real
//! `jigc` against a throwaway `[dev ▸ methodology]` repo (the model is
//! `doc_write_milestone_record.rs`) and reads its emitted key sets — the witness must
//! name **exactly** what `jigc doc show --format json` puts on the wire, at every level
//! of the shape, and the contract's own whole-doc block must agree with the witness.
//! A key added, removed or renamed anywhere in the projection reddens here until the
//! proof is corrected.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// The doc that enumerates the witness — the artifact under test.
const WITNESS_DOC: &str = "design/team-ready-state.md";

/// The prefix of the witness enumeration inside that doc.
const WITNESS_PREFIX: &str = "whole-doc = ";

/// The doc that pins the contract the witness proves.
const CONTRACT_DOC: &str = "design/doc-read-surface.md";

/// The heading of the contract's own whole-doc shape block.
const CONTRACT_BLOCK_PREFIX: &str = "**Whole-doc**";

/// The milestone the fixture mints, and the sub-task that gives its `tasks` section an
/// item — without one, the item-key arm would assert over an empty array.
const MILESTONE_TITLE: &str = "Cache rework";
const MILESTONE_SLUG: &str = "cache-rework";
const SUB_TASK_INTENT: &str = "Warm the read cache";

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-read-contract-witness-{tag}-{}-{:?}",
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

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("the workspace root is reachable from the cli crate")
}

fn read_doc(rel: &str) -> String {
    fs::read_to_string(repo_root().join(rel)).unwrap_or_else(|e| panic!("{rel} is readable: {e}"))
}

fn git(repo: &Path, args: &[&str]) {
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
}

/// Run `jigc` with `cwd = repo` and `$HOME = home`, never inheriting a harness
/// `JIGC_PACK_DIR` (the compose-marker path requires it ABSENT).
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    let out = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("run the jigc binary");
    assert!(
        out.status.success(),
        "jigc {args:?} must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    out
}

/// The real binary's emitted whole-doc json for a real milestone record.
fn emitted_witness(repo: &TempDir, home: &TempDir) -> serde_json::Value {
    let (repo, home) = (repo.path(), home.path());
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "README.md"]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("mk project config");
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        "compose-embedded-methodology: true\n",
    )
    .expect("write compose marker");

    jigc(repo, home, &["milestone", "create", MILESTONE_TITLE]);
    jigc(
        repo,
        home,
        &["milestone", "add-task", MILESTONE_SLUG, SUB_TASK_INTENT],
    );
    let out = jigc(
        repo,
        home,
        &[
            "doc",
            "show",
            &format!("milestone-record:{MILESTONE_SLUG}"),
            "--format",
            "json",
        ],
    );
    serde_json::from_slice(&out.stdout).expect("the pinned read contract emits json")
}

/// A key-bearing shape parsed out of a prose sketch — the only thing a prose
/// enumeration can be compared on.
#[derive(Debug)]
enum Shape {
    /// An object's keys in declaration order, each with the nested shape it names.
    Object(Vec<(String, Option<Shape>)>),
    /// An array's element shape.
    Array(Box<Shape>),
}

impl Shape {
    /// The key set at this level. Panics on an array — a sketch that puts an array
    /// where the emitted json has an object is itself the defect.
    fn keys(&self) -> BTreeSet<String> {
        match self {
            Shape::Object(keys) => keys.iter().map(|(k, _)| k.clone()).collect(),
            Shape::Array(_) => panic!("expected an object, found an array"),
        }
    }

    /// The shape this object's `key` names, or a panic naming what is missing.
    fn at(&self, key: &str) -> &Shape {
        match self {
            Shape::Object(keys) => keys
                .iter()
                .find(|(k, _)| k == key)
                .unwrap_or_else(|| panic!("the sketch has no `{key}` key"))
                .1
                .as_ref()
                .unwrap_or_else(|| panic!("the sketch's `{key}` names no nested shape")),
            Shape::Array(_) => panic!("expected an object, found an array"),
        }
    }

    /// The element shape of the array this object's `key` names.
    fn element(&self, key: &str) -> &Shape {
        match self.at(key) {
            Shape::Array(inner) => inner,
            Shape::Object(_) => panic!("the sketch's `{key}` is an object, not an array"),
        }
    }
}

/// A cursor over a prose shape sketch. Quotes, backticks, emphasis markers and the
/// elision `…` are filler: the sketches differ in decoration (`{ type, … }` against
/// `{ "type": <doctype>, … }`) and agree on exactly the thing under test — the keys.
struct Sketch {
    chars: Vec<char>,
    at: usize,
    source: String,
}

impl Sketch {
    fn new(text: &str, source: &str) -> Self {
        Sketch {
            chars: text.chars().collect(),
            at: 0,
            source: source.to_string(),
        }
    }

    fn fail(&self, what: &str) -> ! {
        panic!(
            "{}: {what} at offset {} of the shape sketch",
            self.source, self.at
        )
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.at).copied()
    }

    /// Skip whitespace, separators and decoration — everything a key position may
    /// carry that is not part of a key.
    fn filler(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() || matches!(c, ',' | '…' | '"' | '`' | '*') {
                self.at += 1;
            } else {
                break;
            }
        }
    }

    /// Skip only the decoration that may sit between a key and its `:` separator.
    fn decoration(&mut self) {
        while matches!(self.peek(), Some(c) if c.is_whitespace() || c == '"' || c == '`') {
            self.at += 1;
        }
    }

    /// Parse the whole sketch, which must be one object and nothing else.
    fn parse(mut self) -> Shape {
        self.filler();
        let shape = self.object();
        self.filler();
        if self.at < self.chars.len() {
            self.fail("trailing text after the shape");
        }
        shape
    }

    fn object(&mut self) -> Shape {
        if self.peek() != Some('{') {
            self.fail("expected `{`");
        }
        self.at += 1;
        let mut keys: Vec<(String, Option<Shape>)> = Vec::new();
        loop {
            self.filler();
            match self.peek() {
                None => self.fail("unterminated object"),
                Some('}') => {
                    self.at += 1;
                    return Shape::Object(keys);
                }
                _ => {}
            }
            let key = self.ident();
            self.decoration();
            // A bare key (`{ type, slug, … }`) names no nested shape; a `:` introduces
            // one, which is an object, an array, or a scalar placeholder.
            let nested = if self.peek() == Some(':') {
                self.at += 1;
                while matches!(self.peek(), Some(c) if c.is_whitespace()) {
                    self.at += 1;
                }
                match self.peek() {
                    Some('{') => Some(self.object()),
                    Some('[') => Some(self.array()),
                    _ => {
                        self.scalar();
                        None
                    }
                }
            } else {
                None
            };
            keys.push((key, nested));
        }
    }

    fn array(&mut self) -> Shape {
        if self.peek() != Some('[') {
            self.fail("expected `[`");
        }
        self.at += 1;
        let mut element = None;
        loop {
            self.filler();
            match self.peek() {
                None => self.fail("unterminated array"),
                Some(']') => {
                    self.at += 1;
                    return Shape::Array(Box::new(
                        element.unwrap_or_else(|| self.fail("the array names no element shape")),
                    ));
                }
                Some('{') => {
                    if element.is_some() {
                        self.fail("the array names two element shapes");
                    }
                    element = Some(self.object());
                }
                _ => self.scalar(),
            }
        }
    }

    /// A key name: the identifier characters the doc's own key vocabulary uses.
    fn ident(&mut self) -> String {
        let start = self.at;
        while matches!(self.peek(), Some(c) if c.is_ascii_alphanumeric() || c == '-' || c == '_') {
            self.at += 1;
        }
        if self.at == start {
            self.fail("expected a key name");
        }
        self.chars[start..self.at].iter().collect()
    }

    /// A scalar value placeholder (`<doctype>`, `<n>`, `null`, …) — consumed, not read.
    fn scalar(&mut self) {
        let start = self.at;
        while let Some(c) = self.peek() {
            if matches!(c, ',' | '}' | ']') {
                break;
            }
            if c == '{' || c == '[' {
                self.fail("a nested shape in scalar position");
            }
            self.at += 1;
        }
        if self.at == start {
            self.fail("expected a scalar placeholder");
        }
    }
}

/// The witness enumeration at `team-ready-state.md` → The read surface: the inline code
/// span introduced by `whole-doc = `.
fn witness_shape() -> Shape {
    let text = read_doc(WITNESS_DOC);
    let at = text
        .find(WITNESS_PREFIX)
        .unwrap_or_else(|| panic!("{WITNESS_DOC} no longer states `{WITNESS_PREFIX}`"));
    let rest = &text[at + WITNESS_PREFIX.len()..];
    let mut span = rest
        .strip_prefix('`')
        .unwrap_or_else(|| {
            panic!("{WITNESS_DOC}: `{WITNESS_PREFIX}` is not followed by a code span")
        })
        .splitn(2, '`');
    let sketch = span
        .next()
        .unwrap_or_else(|| panic!("{WITNESS_DOC}: the witness code span is unterminated"));
    assert!(
        span.next().is_some(),
        "{WITNESS_DOC}: the witness code span is unterminated"
    );
    Sketch::new(sketch, WITNESS_DOC).parse()
}

/// The contract's own whole-doc block at `doc-read-surface.md` → The pinned `--format
/// json` contract (1.0) — the fenced json sketch under `**Whole-doc**`.
fn contract_shape() -> Shape {
    let text = read_doc(CONTRACT_DOC);
    let mut lines = text
        .lines()
        .skip_while(|l| !l.starts_with(CONTRACT_BLOCK_PREFIX));
    assert!(
        lines.next().is_some(),
        "{CONTRACT_DOC} no longer states `{CONTRACT_BLOCK_PREFIX}`"
    );
    let block: String = lines
        .skip_while(|l| !l.starts_with("```"))
        .skip(1)
        .take_while(|l| !l.starts_with("```"))
        .collect::<Vec<&str>>()
        .join("\n");
    assert!(
        !block.trim().is_empty(),
        "{CONTRACT_DOC}: `{CONTRACT_BLOCK_PREFIX}` is followed by no fenced shape block"
    );
    Sketch::new(&block, CONTRACT_DOC).parse()
}

fn json_keys(value: &serde_json::Value, what: &str) -> BTreeSet<String> {
    value
        .as_object()
        .unwrap_or_else(|| panic!("the emitted {what} is a json object"))
        .keys()
        .cloned()
        .collect()
}

/// The witness names **exactly** the keys the binary emits, at every level of the
/// shape — the contract's proof and the contract's behaviour are the same shape.
#[test]
fn the_witness_names_every_key_the_binary_emits() {
    let repo = TempDir::new("witness");
    let home = TempDir::new("witness-home");
    let emitted = emitted_witness(&repo, &home);
    let witness = witness_shape();

    assert_eq!(
        witness.keys(),
        json_keys(&emitted, "whole-doc"),
        "{WITNESS_DOC} → The read surface enumerates a different TOP-LEVEL key set than \
         `jigc doc show milestone-record:<slug> --format json` emits — the 1.0 read \
         contract's own conformance witness is not the shape it witnesses \
         ({CONTRACT_DOC} → The pinned `--format json` contract)"
    );

    let fields = emitted
        .get("fields")
        .expect("the emitted whole-doc has fields");
    assert_eq!(
        witness.at("fields").keys(),
        json_keys(fields, "`fields` map"),
        "{WITNESS_DOC} → The read surface enumerates a different `fields` key set than the \
         binary emits"
    );

    let sections = emitted
        .get("sections")
        .expect("the emitted whole-doc has sections");
    assert_eq!(
        witness.at("sections").keys(),
        json_keys(sections, "`sections` map"),
        "{WITNESS_DOC} → The read surface enumerates a different `sections` key set than the \
         binary emits"
    );

    let items = sections
        .get("tasks")
        .and_then(|t| t.as_array())
        .expect("the emitted `sections.tasks` is an item array");
    let item = items
        .first()
        .expect("the fixture mints one sub-task, so the item array is non-empty");
    assert_eq!(
        witness.at("sections").element("tasks").keys(),
        json_keys(item, "`tasks` item"),
        "{WITNESS_DOC} → The read surface enumerates a different ITEM key set than the binary \
         emits — a driver reading the witness cannot address the items it reads back"
    );
}

/// The two homes agree with each other, not only each with the binary: the contract's
/// own whole-doc block and its witness state one top-level shape.
#[test]
fn the_contract_and_its_witness_state_one_top_level_shape() {
    assert_eq!(
        contract_shape().keys(),
        witness_shape().keys(),
        "{CONTRACT_DOC} → The pinned `--format json` contract and its declared conformance \
         witness at {WITNESS_DOC} → The read surface enumerate different top-level key sets — \
         the pin and its proof disagree"
    );
}
