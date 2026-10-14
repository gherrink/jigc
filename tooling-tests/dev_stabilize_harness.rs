//! **The stabilization harness a session invokes is generated from its commented source,
//! and held below the runtime's limit** ([DECISIONS.md](../DECISIONS.md) → *2026-10-15 — The
//! harness a session invokes is generated, without its comments*).
//!
//! The Workflow tool refuses a script over 524,288 bytes — by `scriptPath` and inline alike,
//! in the words `Workflow script file <path> exceeds 524288 bytes` (observed 2026-10-13, when a
//! session asked the harness for its self-test by its path). The commented harness had grown
//! past that, and no suite saw it: every one drives the harness through the runtime's
//! stand-in, which read the file with no limit. So `.claude/workflows/stabilize.js` — the path
//! every session, prompt file, tool and record names — is a **committed, generated** file:
//! [`dev/stabilize-harness`](../dev/stabilize-harness) writes it from the commented source,
//! `.claude/workflow-sources/stabilize.js`, with every comment removed and three header lines
//! saying so.
//!
//! - **(a) the byte fence**: the invoked file is exactly what the generator prints for the
//!   source as it stands — an edit to either without a regeneration is red, and the message
//!   names the command.
//! - **(b) the size fence**: the invoked file stays at or below [`BOUND`], which leaves
//!   64 KiB between it and the runtime's refusal.
//! - **(c)** the generator removes what is a comment and nothing else — every case a lexer
//!   can get wrong, each held to its expected bytes: `//` and `/*` in a string, in a template
//!   literal (a line of a prompt that begins with `//`), in a `${…}` nested in one, in a
//!   regular expression and in its class, beside a division; and every placement of a real
//!   comment on its line.
//! - **(d)** a source it cannot read to its end is refused, naming its line, and nothing is
//!   written.
//! - **(e)** the file it writes is the file it prints.
//! - **(f) a second lexer, where `node` is on `PATH`**: the parser `node` itself carries
//!   (acorn, reached through `--expose-internals`) reads the source and the invoked file, and
//!   finds the same tokens in the same order, the same line breaks between them, and in the
//!   invoked file no comment but the header's. The cases of (c) are held to it too, so that
//!   a case's expected bytes are themselves the right ones.
//!
//! **What else drives the generated file.** The fence over the harness
//! ([`stabilize_harness_fence`](super::stabilize_harness_fence)) and the simulation of a
//! stage ([`stabilize_simulation`](super::stabilize_simulation)) read and run
//! `.claude/workflows/stabilize.js` — the file a session runs — and the simulation's
//! stand-in for the runtime now refuses a script over the limit with the runtime's words
//! (that suite's `the_runtimes_stand_in_refuses_a_script_over_the_runtimes_limit`).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use serde_json::{Value, json};

use super::dev_stabilize_step::node_or_skip;
use super::placed_executable;
use super::rig_processes::RigDir;

const GENERATOR: &str = "dev/stabilize-harness";
const SOURCE: &str = ".claude/workflow-sources/stabilize.js";
const GENERATED: &str = ".claude/workflows/stabilize.js";

/// The Workflow tool's limit on a script, in bytes: `Workflow script file <path> exceeds
/// 524288 bytes`.
const LIMIT: u64 = 524_288;

/// What the invoked harness may weigh: seven-eighths of [`LIMIT`], so that the fence is red
/// while 64 KiB still lie between the file and the runtime's refusal — room for the change
/// that crossed it to be finished and its own cut planned, rather than a harness that a
/// session cannot start.
const BOUND: u64 = 458_752;
const _: () = assert!(BOUND < LIMIT);

/// The three lines the generator writes first.
const HEADER: &str = "// GENERATED — never edit this file by hand. It is .claude/workflow-sources/stabilize.js\n\
// with its comments removed, written by `dev/stabilize-harness`: edit the source, then\n\
// run that command (implementation/stabilization-workflow.md -> The harness's source).\n";

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("repo root is two levels above crates/cli")
        .to_path_buf()
}

/// What `dev/stabilize-harness --stdout` prints for the source as it stands.
fn generated_now() -> Vec<u8> {
    let out = Command::new(repo_root().join(GENERATOR))
        .arg("--stdout")
        .output()
        .unwrap_or_else(|e| panic!("run {GENERATOR} --stdout: {e}"));
    assert!(
        out.status.success(),
        "{GENERATOR} --stdout exited {:?}: {}",
        out.status.code(),
        String::from_utf8_lossy(&out.stderr)
    );
    out.stdout
}

/// A tree of its own with the generator copied in and `source` at the source's path.
struct Tree(RigDir);

impl Tree {
    fn new(source: &str) -> Self {
        let rig = RigDir::new("stabilize-harness");
        for dir in ["dev", ".claude/workflow-sources", ".claude/workflows"] {
            fs::create_dir_all(rig.path().join(dir)).expect("make the tree's directories");
        }
        placed_executable::copy(&repo_root().join(GENERATOR), &rig.path().join(GENERATOR));
        fs::write(rig.path().join(SOURCE), source).expect("write the source");
        Tree(rig)
    }

    fn run(&self, args: &[&str]) -> Output {
        Command::new(self.0.path().join(GENERATOR))
            .args(args)
            .output()
            .expect("run the generator")
    }

    fn generated(&self) -> Option<Vec<u8>> {
        fs::read(self.0.path().join(GENERATED)).ok()
    }
}

/// `source`, through the generator: its stdout without the header, or its refusal.
fn through(source: &str) -> Result<String, String> {
    let out = Tree::new(source).run(&["--stdout"]);
    if !out.status.success() {
        return Err(String::from_utf8_lossy(&out.stderr).into_owned());
    }
    let text = String::from_utf8(out.stdout).expect("the generator prints UTF-8");
    let body = text
        .strip_prefix(HEADER)
        .unwrap_or_else(|| panic!("the generator's output opens with the header: {text:?}"));
    Ok(body.to_owned())
}

/// Each case: a name, the source, and what the generator must make of it (without the header).
fn cases() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        (
            "a line that is a comment goes whole",
            "a()\n// gone\nb()\n",
            "a()\nb()\n",
        ),
        (
            "an indented one too",
            "{\n    // gone\n    b()\n}\n",
            "{\n    b()\n}\n",
        ),
        (
            "a blank line stays",
            "a()\n\n// gone\n\nb()\n",
            "a()\n\n\nb()\n",
        ),
        (
            "a comment after code goes with the spaces before it",
            "a()   // gone\nb()\n",
            "a()\nb()\n",
        ),
        ("a last line without a line break", "a()\n// gone", "a()\n"),
        (
            "a block comment on lines of its own goes whole",
            "a()\n/* gone\n   gone */\nb()\n",
            "a()\nb()\n",
        ),
        (
            "a block comment between code is one space",
            "a(/* gone */1)\n",
            "a( 1)\n",
        ),
        (
            "a block comment after code goes",
            "a() /* gone */\n",
            "a()\n",
        ),
        (
            "a block comment before code goes, the indent stays",
            "  /* gone */ a()\n",
            "   a()\n",
        ),
        (
            "a block comment across lines between code keeps one break",
            "a() /* gone\ngone */ b()\n",
            "a()\n b()\n",
        ),
        (
            "trailing spaces with no comment stay",
            "a(1 /* x */, 2)  \n",
            "a(1  , 2)  \n",
        ),
        (
            "`//` in a string is text",
            "a('// kept', \"// kept\")\n",
            "a('// kept', \"// kept\")\n",
        ),
        (
            "`/*` in a string is text",
            "a('/* kept */')\n",
            "a('/* kept */')\n",
        ),
        (
            "an escaped quote does not end a string",
            "a('it\\'s // kept') // gone\n",
            "a('it\\'s // kept')\n",
        ),
        (
            "a line continuation in a string",
            "a('one \\\n// kept')\n",
            "a('one \\\n// kept')\n",
        ),
        (
            "a line of a template that begins with `//` is text",
            "a(`one\n// kept\n  /* kept */\n`)\n",
            "a(`one\n// kept\n  /* kept */\n`)\n",
        ),
        (
            "a comment inside `${}` goes, the template's text stays",
            "a(`x ${ b // gone\n} // kept`)\n",
            "a(`x ${ b\n} // kept`)\n",
        ),
        (
            "templates and objects nested in `${}`",
            "a(`${ { k: `${'}'} // kept` } } // kept`) // gone\n",
            "a(`${ { k: `${'}'} // kept` } } // kept`)\n",
        ),
        (
            "an escaped backtick does not end a template",
            "a(`\\` // kept`)\n",
            "a(`\\` // kept`)\n",
        ),
        (
            "a regular expression holding `//`",
            "a(/\\/\\//.test(x)) // gone\n",
            "a(/\\/\\//.test(x))\n",
        ),
        (
            "a regular expression whose class holds `/`",
            "x = /[/]/g // gone\n",
            "x = /[/]/g\n",
        ),
        (
            "a regular expression whose class holds a quote",
            "x = /['`]/ // gone\n",
            "x = /['`]/\n",
        ),
        (
            "a regular expression whose class holds `/` and a quote",
            "x = /[/']/ // gone\n",
            "x = /[/']/\n",
        ),
        (
            "a regular expression that opens the text",
            "/'/.test(a) // gone\n",
            "/'/.test(a)\n",
        ),
        (
            "an object closed inside `${}`, and a comment after it",
            "a(`${ {}.x // gone\n} kept`)\n",
            "a(`${ {}.x\n} kept`)\n",
        ),
        (
            "a regular expression after `return`",
            "function f() { return /'/ } // gone\n",
            "function f() { return /'/ }\n",
        ),
        (
            "a division after a word",
            "x = a / 2 + '//' // gone\n",
            "x = a / 2 + '//'\n",
        ),
        (
            "a division after a word with `_` and `$`",
            "x = a_return + a$return / 2 + '//' // gone\n",
            "x = a_return + a$return / 2 + '//'\n",
        ),
        (
            "a division after a number",
            "x = 1 / 2 + '//' // gone\n",
            "x = 1 / 2 + '//'\n",
        ),
        (
            "a division after `)`",
            "x = (a) / 2 + '//' // gone\n",
            "x = (a) / 2 + '//'\n",
        ),
        (
            "a division after `]`",
            "x = q[1] / 2 + '//' // gone\n",
            "x = q[1] / 2 + '//'\n",
        ),
        (
            "a division after `}`",
            "x = {} / 2 + '//' // gone\n",
            "x = {} / 2 + '//'\n",
        ),
        (
            "a division after a string",
            "x = 'a' / 2 + '//' // gone\n",
            "x = 'a' / 2 + '//'\n",
        ),
        (
            "a division after a template",
            "x = `a` / 2 + '//' // gone\n",
            "x = `a` / 2 + '//'\n",
        ),
        (
            "a division after a regular expression",
            "x = /a/ / 2 + '//' // gone\n",
            "x = /a/ / 2 + '//'\n",
        ),
        (
            "a regular expression opening a `${}`",
            "a(`${/'/.source} // kept`)\n",
            "a(`${/'/.source} // kept`)\n",
        ),
        (
            "a last line of code and a comment, no line break",
            "a()\nb() // gone",
            "a()\nb()",
        ),
        ("a last line of code, no line break", "// gone\na()", "a()"),
    ]
}

// ---------------------------------------------------------------------------
// (a) and (b) — the file a session invokes
// ---------------------------------------------------------------------------

#[test]
fn a_the_invoked_harness_is_what_the_generator_makes_of_its_source() {
    let invoked = fs::read(repo_root().join(GENERATED))
        .unwrap_or_else(|e| panic!("read {GENERATED}: {e} — run `{GENERATOR}` to generate it"));
    assert!(
        invoked == generated_now(),
        "{GENERATED} is not what `{GENERATOR}` makes of {SOURCE}: edit the source, never the \
         generated file, and run `{GENERATOR}`"
    );
    assert!(
        invoked.starts_with(HEADER.as_bytes()),
        "{GENERATED} opens with the header that says it is generated, from which file and by \
         which command"
    );
}

#[test]
fn b_the_invoked_harness_stays_below_the_runtimes_limit_with_room_to_grow() {
    let size = fs::metadata(repo_root().join(GENERATED))
        .unwrap_or_else(|e| panic!("read {GENERATED}: {e}"))
        .len();
    println!("{GENERATED}: {size} bytes; the bound {BOUND}; the runtime's limit {LIMIT}");
    assert!(
        size <= BOUND,
        "{GENERATED} is {size} bytes, over this fence's bound of {BOUND}. The Workflow tool \
         refuses a script over {LIMIT} bytes — `Workflow script file <path> exceeds 524288 \
         bytes`, by `scriptPath` and inline alike (observed 2026-10-13; DECISIONS.md -> \
         2026-10-15, \"The harness a session invokes is generated, without its comments\") — \
         and this bound keeps 64 KiB between the file and that refusal. The generated file \
         holds no comment to remove: the cut is the code's, and it is planned, never made \
         by removing the source's prose"
    );
}

// ---------------------------------------------------------------------------
// (c), (d), (e) — the generator
// ---------------------------------------------------------------------------

#[test]
fn c_the_generator_removes_what_is_a_comment_and_nothing_else() {
    let mut wrong = Vec::new();
    for (name, source, expected) in cases() {
        match through(source) {
            Ok(body) if body == expected => {}
            Ok(body) => wrong.push(format!(
                "{name}: {source:?} became {body:?}, not {expected:?}"
            )),
            Err(refusal) => wrong.push(format!("{name}: {source:?} was refused: {refusal}")),
        }
    }
    assert!(
        wrong.is_empty(),
        "the generator's cases:\n{}",
        wrong.join("\n")
    );
}

#[test]
fn d_a_source_the_generator_cannot_read_to_its_end_is_refused_by_its_line() {
    let cases = [
        ("a()\nb('open\n", 2, "a string runs into a line break"),
        ("a()\nb('open", 2, "a string is left open"),
        ("a()\n\nb(`open\n", 3, "a template literal is left open"),
        (
            "a()\nb(`${ c }`, `${ d \n",
            2,
            "a template literal is left open",
        ),
        (
            "a()\nx = /open\n",
            2,
            "a regular expression runs into a line break",
        ),
        ("x = /open", 1, "a regular expression is left open"),
        ("a()\n/* open\n", 2, "a comment is left open"),
    ];
    for (source, line, why) in cases {
        let tree = Tree::new(source);
        for args in [&["--stdout"][..], &[][..]] {
            let out = tree.run(args);
            let stderr = String::from_utf8_lossy(&out.stderr);
            assert_eq!(
                out.status.code(),
                Some(1),
                "{source:?} {args:?} is refused: {stderr}"
            );
            assert!(out.stdout.is_empty(), "a refusal prints nothing on stdout");
            let said = format!("refused, nothing written — {SOURCE}:{line}: {why}");
            assert!(
                stderr.contains(&said),
                "{source:?} is refused with `{said}`: {stderr}"
            );
            assert_eq!(tree.generated(), None, "a refusal writes nothing");
        }
    }
    let usage = Tree::new("a()\n").run(&["--write"]);
    assert_eq!(
        usage.status.code(),
        Some(2),
        "an argument it does not take is a usage error"
    );
}

#[test]
fn e_the_generator_writes_the_file_it_prints() {
    let source = "// gone\na(`// kept`) // gone\n";
    let tree = Tree::new(source);
    fs::write(tree.0.path().join(GENERATED), "a hand edit\n").expect("write over the file");
    let printed = tree.run(&["--stdout"]);
    assert!(printed.status.success());
    let wrote = tree.run(&[]);
    assert!(
        wrote.status.success() && wrote.stdout.is_empty(),
        "a write prints nothing"
    );
    assert_eq!(
        tree.generated(),
        Some(printed.stdout),
        "the file written is the file printed"
    );
    let mode = fs::metadata(tree.0.path().join(GENERATED))
        .expect("the file")
        .permissions();
    assert_eq!(
        std::os::unix::fs::PermissionsExt::mode(&mode) & 0o777,
        0o644
    );
    let left: Vec<_> = fs::read_dir(tree.0.path().join(".claude/workflows"))
        .expect("the directory")
        .map(|entry| entry.expect("an entry").file_name())
        .collect();
    assert_eq!(
        left,
        vec![std::ffi::OsString::from("stabilize.js")],
        "no staged file is left"
    );
}

// ---------------------------------------------------------------------------
// (f) — a second lexer
// ---------------------------------------------------------------------------

/// Read with the parser `node` carries: for each file, its tokens (each with whether a line
/// break stands between it and the token before it) and its comments' text.
const SECOND_LEXER: &str = r#"
const acorn = require('internal/deps/acorn/acorn/dist/acorn')
const { readFileSync } = require('node:fs')
const out = []
for (const path of process.argv.slice(1)) {
  const text = readFileSync(path, 'utf8').replace(/^export const meta = /m, 'const meta = ')
  const tokens = []
  const comments = []
  acorn.parse(text, { ecmaVersion: 'latest', sourceType: 'script', allowReturnOutsideFunction: true,
    allowAwaitOutsideFunction: true, onComment: comments, onToken: tokens })
  let end = null
  out.push({
    tokens: tokens.map((t) => {
      const broken = end !== null && /[\n\r\u2028\u2029]/.test(text.slice(end, t.start))
      end = t.end
      return [t.type.label, t.value === undefined ? null : t.value, broken]
    }),
    comments: comments.map((c) => c.value),
  })
}
process.stdout.write(JSON.stringify(out))
"#;

fn second_lexer(paths: &[&Path]) -> Vec<Value> {
    let out = Command::new("node")
        .arg("--expose-internals")
        .arg("-e")
        .arg(SECOND_LEXER)
        .args(paths)
        .output()
        .expect("run node");
    assert!(
        out.status.success(),
        "node reads the files with the parser it carries (`internal/deps/acorn`, reached \
         through `--expose-internals`); where a node no longer carries it there, this arm \
         needs another parser: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    serde_json::from_slice::<Vec<Value>>(&out.stdout).expect("one JSON list")
}

#[test]
fn f_a_second_lexer_reads_the_same_tokens_in_the_source_and_the_invoked_harness() {
    if !node_or_skip(&format!(
        "the tokens of {GENERATED} were not held to {SOURCE}'s"
    )) {
        return;
    }
    let root = repo_root();
    let read = second_lexer(&[&root.join(SOURCE), &root.join(GENERATED)]);
    let (source, invoked) = (&read[0], &read[1]);
    let header: Vec<Value> = HEADER
        .lines()
        .map(|line| json!(line.strip_prefix("//").expect("a comment line")))
        .collect();
    assert_eq!(
        invoked["comments"],
        json!(header),
        "the invoked harness holds no comment but the header's"
    );
    let count = |v: &Value, k: &str| v[k].as_array().map_or(0, Vec::len);
    println!(
        "{SOURCE}: {} tokens, {} comments; {GENERATED}: {} tokens, {} comments",
        count(source, "tokens"),
        count(source, "comments"),
        count(invoked, "tokens"),
        count(invoked, "comments")
    );
    assert!(
        count(source, "comments") > 0,
        "the control: the source's comments are read"
    );
    if source["tokens"] != invoked["tokens"] {
        let (a, b) = (
            source["tokens"].as_array().unwrap(),
            invoked["tokens"].as_array().unwrap(),
        );
        let at = a
            .iter()
            .zip(b)
            .position(|(x, y)| x != y)
            .unwrap_or(a.len().min(b.len()));
        panic!(
            "the source and the invoked harness differ at token {at} ({} and {} tokens): {:?} \
             against {:?}",
            a.len(),
            b.len(),
            a.get(at),
            b.get(at)
        );
    }

    // THE CASES' EXPECTED BYTES ARE THE RIGHT ONES: each case's source and its expected
    // output read the same to this lexer, and the expected output holds no comment.
    let scratch = RigDir::new("stabilize-harness-lexer");
    let mut paths = Vec::new();
    for (k, (_, source, expected)) in cases().iter().enumerate() {
        for (side, text) in [("source", source), ("expected", expected)] {
            let path = scratch.path().join(format!("{k}-{side}.js"));
            fs::write(&path, text).expect("write a case");
            paths.push(path);
        }
    }
    let read = second_lexer(&paths.iter().map(PathBuf::as_path).collect::<Vec<_>>());
    for (k, (name, _, _)) in cases().iter().enumerate() {
        let (source, expected) = (&read[2 * k], &read[2 * k + 1]);
        assert_eq!(
            source["tokens"], expected["tokens"],
            "case `{name}`: the same tokens"
        );
        assert_eq!(
            expected["comments"],
            json!([]),
            "case `{name}`: no comment is left"
        );
    }
}
