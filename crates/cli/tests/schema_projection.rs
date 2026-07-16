//! M43 Increment 3 / T1 — the `{{schema:<doctype>}}` projection through the real
//! binary (`design/surface-contract.md` → The schema projection, law 1).
//!
//! A project step shadow carries a lone `{{schema:adr}}`; `jigc start` composes it
//! through the real cascade path and the **emitted text** must name every adr
//! section — including `options`, the slot the hand-written migrate template
//! famously omitted — at the **resolved** decisions path (`docs/decisions/…`, the
//! `docs-root`-prefixed home, not the schema-raw `decisions/`).
//!
//! The grammar-cannot-lie proof then runs the **emitted bytes themselves**: the
//! generated `jigc doc author … <<'EOF' … EOF` block is extracted from the composed
//! output, its fill-me placeholders filled, and executed verbatim through `sh` with
//! the real binary on PATH — a payload the real `doc author` rejects would mean the
//! projection taught a grammar the tool does not accept.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-schema-projection-{tag}-{}-{:?}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
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

/// Run a `git` command in `repo`, asserting success.
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

/// Run `jigc <args>` against the embedded dev pack.
fn jigc(repo: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("run the jigc binary")
}

/// A committed repo with the project layer present, whose `implement` step is
/// project-shadowed to solicit the ADR authoring via the generated projection.
fn seed_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write README");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    let steps = repo.join(".jigc").join("config").join("steps");
    fs::create_dir_all(&steps).expect("create project steps dir");
    fs::write(
        steps.join("implement.yaml"),
        "Implement the change. A decision is warranted here — author the whole ADR \
         in one batch payload:\n\
         \n\
         {{ schema:adr }}\n",
    )
    .expect("write the project step shadow");
}

/// Extract the generated `jigc doc author … <<'EOF' … EOF` block (inclusive) from
/// the composed output.
fn author_block(composed: &str) -> String {
    let mut lines = composed.lines();
    let command = lines
        .by_ref()
        .find(|l| l.starts_with("jigc doc author adr --from-file -"))
        .expect("the composed output carries the generated `jigc doc author` line");
    let mut block = format!("{command}\n");
    for line in lines {
        block.push_str(line);
        block.push('\n');
        if line == "EOF" {
            return block;
        }
    }
    panic!("the generated author block must close with its EOF marker:\n{composed}");
}

/// Fill the generated skeleton's placeholders: a real title for the `<the title>`
/// fill-me, real prose between every `<<…>>` slot marker (the markers kept — they
/// are required syntax, exactly as the projection states).
fn fill(block: &str) -> String {
    block
        .lines()
        .map(|line| {
            if let Some(indent) = line.find("title: \"<the title>\"") {
                format!(
                    "{}title: \"Choose the gateway rate limiter\"",
                    &line[..indent]
                )
            } else if let (Some(open), true) = (line.find("<<"), line.trim_end().ends_with(">>")) {
                format!("{}<<Real prose, filled by the agent.>>", &line[..open])
            } else {
                line.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
        + "\n"
}

/// The projection names every adr section — incl. `options` — at the resolved
/// (docs-root-prefixed) home, and its generated skeleton, filled, is accepted by
/// the real `jigc doc author` when the emitted block runs verbatim through `sh`.
#[test]
fn composed_schema_projection_names_every_adr_section_and_its_skeleton_authors() {
    let repo = TempDir::new("adr");
    seed_repo(repo.path());

    let out = jigc(
        repo.path(),
        &[
            "start",
            "record the gateway decision",
            "--workflow",
            "single-task",
        ],
    );
    assert!(
        out.status.success(),
        "start must compose: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let composed = String::from_utf8_lossy(&out.stdout).into_owned();

    // Law 1 — the emitted text names ALL FIVE adr sections (the tree), including
    // `options`, and the home is the RESOLVED path (docs-root applied), never the
    // schema-raw `decisions/`.
    assert!(
        composed.contains("`docs/decisions/<slug>.md`"),
        "the home renders resolved through docs-root; got:\n{composed}"
    );
    for section in [
        "- `status` (front-matter fields):",
        "- `context`: prose slot",
        "- `options`: prose slot (optional)",
        "- `decision`: prose slot",
        "- `consequences`: prose slot",
    ] {
        assert!(
            composed.contains(section),
            "the projection must name {section:?}; got:\n{composed}"
        );
    }
    // The enum members and the supersedes relation are projected, not hand-stated.
    assert!(
        composed.contains("enum, one of: proposed | accepted | superseded"),
        "the status enum members are projected; got:\n{composed}"
    );

    // The generated command line carries the composing task's id verbatim.
    let task_id = composed
        .lines()
        .find_map(|l| l.strip_prefix("task minted: "))
        .expect("start announces the minted task id")
        .trim()
        .to_owned();
    let block = author_block(&composed);
    assert!(
        block.starts_with(&format!(
            "jigc doc author adr --from-file - --task {task_id} <<'EOF'"
        )),
        "the generated command is copy-runnable for THIS task; got:\n{block}"
    );

    // The grammar-cannot-lie proof: fill the skeleton and run the emitted block
    // VERBATIM through `sh`, with the real binary on PATH as `jigc`.
    let bin_dir = repo.path().join("bin");
    fs::create_dir_all(&bin_dir).expect("create shim dir");
    std::os::unix::fs::symlink(env!("CARGO_BIN_EXE_jigc"), bin_dir.join("jigc"))
        .expect("symlink the jigc shim");
    let path_env = format!(
        "{}:{}",
        bin_dir.display(),
        std::env::var("PATH").unwrap_or_default()
    );
    let filled = fill(&block);
    let run = Command::new("sh")
        .arg("-c")
        .arg(&filled)
        .current_dir(repo.path())
        .env("PATH", &path_env)
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("run the emitted author block through sh");
    let stdout = String::from_utf8_lossy(&run.stdout);
    assert!(
        run.status.success(),
        "the filled generated skeleton must be ACCEPTED by `jigc doc author` \
         (the projection taught the real grammar); block:\n{filled}\nstderr: {}",
        String::from_utf8_lossy(&run.stderr)
    );
    assert!(
        stdout.contains("adr:choose-the-gateway-rate-limiter"),
        "the ack names the authored instance; got: {stdout}"
    );
}
