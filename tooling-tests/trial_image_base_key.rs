//! **The trial image's kept base, the half that needs no docker** — the key a base layer is
//! kept under, and the repository the trial harness reads its commits from
//! ([completions/trial-harness/README.md](../completions/trial-harness/README.md) → The kept
//! base layer; [DECISIONS.md](../DECISIONS.md) → *2026-11-06 — The trial image builds on a
//! kept base layer*).
//!
//! `completions/trial-harness/build-image.sh` builds the toolchain sync and the crate fetch
//! once, as the image `jigc-gate-base:<key>`, and builds every candidate with the same key on
//! it. So the key is the whole of what decides whether a candidate is built on a layer laid by
//! another commit: a key that misses an input builds a candidate on a layer that is wrong for
//! it, and a key that takes in too much rebuilds the layer for nothing. The script defines the
//! key as shell functions and runs nothing when it is sourced, so each arm here sources it and
//! drives the function — the script stays the one implementation, and this suite is what
//! holds it in the gate. Until this suite these arms were a case of
//! `completions/trial-harness/check-base-layer.sh`, which nothing ran but a hand (the review
//! of K7, S2 and its wanted tool); the cases that read built images stay there, run by hand.
//!
//! **What moves the key, and what does not.** In a throwaway repository with one commit per
//! kind of change: a code change keeps it, and so does a change to the manifests alone —
//! under `--locked` a tree's resolve is its lockfile, so keying the manifests would rebuild
//! the base on every added test target; a changed lockfile, a changed toolchain file and a
//! removed one each move it; so does an edit to the Dockerfile above its `build` stage, a
//! parser directive either of BuildKit's two readers takes at its head, and another
//! platform; an edit below the build stage, a comment, a blank line and a directive-shaped
//! comment past the head do not. A key with no platform, with a platform in any spelling but
//! the one the key takes, through a Dockerfile with no build-stage line or with a heredoc
//! above it, is refused, and each refusal is read for what it says. `docker_platform` reads
//! `DOCKER_DEFAULT_PLATFORM` and refuses it in another spelling, without docker.
//!
//! **The check after the build** (`base_check`) is pure once its five strings are read off
//! docker, so it is driven here too: a matching image passes silently, and a wrong key, an
//! empty layer, a layer the base tag does not hold, a platform recorded in another spelling
//! and an image docker reports with a variant each fail, saying which.
//!
//! **The repository** is `JIGC_REPO` when it is set and not empty, else the one the script
//! sits in. The arm drives a copy of the script placed in a repository of its own at another
//! path, so a regression to a path of one machine is red even where that path is this
//! checkout.
//!
//! **What it does not hold:** that a layer built under a key holds what the key says, and
//! that `build-image.sh` reads `base_check`'s inputs off the right images — that is
//! `check-base-layer.sh`'s docker cases and the builds themselves.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::support::ci_workflow::repo_root;
use crate::support::scratch::ScratchDir;

/// The script that defines the key, repository-relative.
const BUILD_IMAGE: &str = "completions/trial-harness/build-image.sh";

/// The Dockerfile the key reads, repository-relative.
const DOCKERFILE: &str = "completions/trial-harness/Dockerfile";

/// The line the key reads the Dockerfile down to.
const BUILD_STAGE_LINE: &str = "FROM ${JIGC_BASE} AS build";

/// The platform every arm keys for, but the one that changes it.
const PLATFORM: &str = "linux/arm64";

/// `build-image.sh` sourced, then `body` run with `args` as `$2…`, under `env`: its exit
/// status, stdout and stderr. Run with this machine's git configuration masked.
fn sourced(body: &str, args: &[&str], env: &[(&str, &str)]) -> (bool, String, String) {
    // A home of the call's own, minted through the shared seam (`temp_mint_fence`).
    let home = ScratchDir::new("trial-image-sourced");
    let mut c = hermetic("bash", home.path());
    c.env_remove("DOCKER_DEFAULT_PLATFORM")
        .arg("-c")
        .arg(format!(r#". "$1" && {body}"#))
        .arg("sourced")
        .arg(repo_root().join(BUILD_IMAGE))
        .args(args);
    for (k, v) in env {
        c.env(k, v);
    }
    let out = c.output().expect("run bash");
    (
        out.status.success(),
        String::from_utf8_lossy(&out.stdout).trim().to_string(),
        String::from_utf8_lossy(&out.stderr).trim().to_string(),
    )
}

/// That `got` is a refusal whose message says `needle`.
fn assert_refused(got: &Result<String, String>, needle: &str, what: &str) {
    match got {
        Ok(key) => panic!("{what} gave a key: {key}"),
        Err(e) => assert!(
            e.contains(needle),
            "{what} was refused, but not for its reason — the message does not say {needle:?}: {e}"
        ),
    }
}

/// A command with this machine's git configuration masked.
fn hermetic(program: &str, home: &Path) -> Command {
    let mut c = Command::new(program);
    c.env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("HOME", home)
        .env("GIT_AUTHOR_NAME", "fixture")
        .env("GIT_AUTHOR_EMAIL", "fixture@example.invalid")
        .env("GIT_COMMITTER_NAME", "fixture")
        .env("GIT_COMMITTER_EMAIL", "fixture@example.invalid");
    c
}

fn git(repo: &Path, args: &[&str]) -> String {
    let out = hermetic("git", repo)
        .arg("-C")
        .arg(repo)
        .args(["-c", "commit.gpgsign=false"])
        .args(args)
        .output()
        .unwrap_or_else(|e| panic!("run git {args:?}: {e}"));
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

/// One commit of every kind of change the key is asked about.
struct Fixture {
    scratch: ScratchDir,
    repo: PathBuf,
    base: String,
    code: String,
    manifest: String,
    lockfile: String,
    toolchain: String,
    no_toolchain: String,
}

impl Fixture {
    fn new() -> Self {
        let scratch = ScratchDir::new("trial-image-base-key");
        let repo = scratch.path().join("repo");
        fs::create_dir_all(repo.join("src")).expect("create the fixture repository");
        let write = |rel: &str, text: &str| {
            fs::write(repo.join(rel), text).unwrap_or_else(|e| panic!("write {rel}: {e}"))
        };
        git(&repo, &["init", "-q"]);
        write(
            "Cargo.toml",
            "[package]\nname = \"a\"\nversion = \"0.1.0\"\n",
        );
        write(
            "Cargo.lock",
            "version = 4\n[[package]]\nname = \"a\"\nversion = \"0.1.0\"\n",
        );
        write("rust-toolchain.toml", "[toolchain]\nchannel = \"1.95.0\"\n");
        write("src/main.rs", "fn main() {}\n");
        let commit = |message: &str| {
            git(&repo, &["add", "-A"]);
            git(&repo, &["commit", "-qm", message]);
            git(&repo, &["rev-parse", "HEAD"])
        };
        let base = commit("base");
        write("src/main.rs", "fn main() { println!(\"x\"); }\n");
        let code = commit("code");
        write(
            "Cargo.toml",
            "[package]\nname = \"a\"\nversion = \"0.1.0\"\n\n[[test]]\nname = \"t\"\npath = \"t.rs\"\n",
        );
        let manifest = commit("manifest");
        write(
            "Cargo.lock",
            "version = 4\n[[package]]\nname = \"a\"\nversion = \"0.1.0\"\n\n[[package]]\nname = \"b\"\nversion = \"2.0.0\"\n",
        );
        let lockfile = commit("lockfile");
        write(
            "Cargo.lock",
            "version = 4\n[[package]]\nname = \"a\"\nversion = \"0.1.0\"\n",
        );
        write("rust-toolchain.toml", "[toolchain]\nchannel = \"1.96.0\"\n");
        let toolchain = commit("toolchain");
        fs::remove_file(repo.join("rust-toolchain.toml")).expect("remove the toolchain file");
        let no_toolchain = commit("no toolchain");
        Fixture {
            scratch,
            repo,
            base,
            code,
            manifest,
            lockfile,
            toolchain,
            no_toolchain,
        }
    }

    /// The harness's Dockerfile, rewritten by `edit`, as a file of this fixture.
    fn dockerfile(&self, name: &str, edit: impl Fn(&str) -> String) -> PathBuf {
        let text = fs::read_to_string(repo_root().join(DOCKERFILE)).expect("read the Dockerfile");
        assert!(
            text.lines().any(|l| l == BUILD_STAGE_LINE),
            "the harness's Dockerfile has no `{BUILD_STAGE_LINE}` line"
        );
        let path = self.scratch.path().join(name);
        fs::write(&path, edit(&text)).expect("write a Dockerfile variant");
        path
    }

    fn harness_dockerfile(&self) -> PathBuf {
        repo_root().join(DOCKERFILE)
    }

    /// `base_key_of` for `sha`, read through `dockerfile`, for `platform`: the key, or the
    /// refusal's stderr.
    fn key(&self, sha: &str, dockerfile: &Path, platform: &str) -> Result<String, String> {
        let out = hermetic("bash", self.scratch.path())
            .arg("-c")
            .arg(r#". "$1" && base_key_of "$2" "$3" "$4" "$5""#)
            .arg("base-key")
            .arg(repo_root().join(BUILD_IMAGE))
            .arg(&self.repo)
            .arg(sha)
            .arg(dockerfile)
            .arg(platform)
            .output()
            .expect("run bash");
        let stdout = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if out.status.success() && !stdout.is_empty() {
            Ok(stdout)
        } else {
            Err(format!(
                "exit {:?}: {}",
                out.status.code(),
                String::from_utf8_lossy(&out.stderr).trim()
            ))
        }
    }

    /// The key of `sha` through the harness's own Dockerfile, for [`PLATFORM`] — which must
    /// be one.
    fn key_of(&self, sha: &str) -> String {
        let key = self
            .key(sha, &self.harness_dockerfile(), PLATFORM)
            .unwrap_or_else(|e| panic!("base_key_of gave no key for {sha}: {e}"));
        assert!(
            key.len() == 64 && key.bytes().all(|b| b.is_ascii_hexdigit()),
            "the key is not one sha256: {key:?}"
        );
        key
    }

    /// The key of the base commit through a Dockerfile variant, for [`PLATFORM`].
    fn key_through(&self, dockerfile: &Path) -> String {
        self.key(&self.base, dockerfile, PLATFORM)
            .unwrap_or_else(|e| panic!("base_key_of gave no key through {dockerfile:?}: {e}"))
    }
}

/// `text` with `line` inserted before its build-stage line.
fn above_the_build_stage(text: &str, line: &str) -> String {
    text.replacen(BUILD_STAGE_LINE, &format!("{line}\n{BUILD_STAGE_LINE}"), 1)
}

#[test]
fn a_code_change_keeps_the_layer() {
    let f = Fixture::new();
    assert_eq!(
        f.key_of(&f.code),
        f.key_of(&f.base),
        "a change to the code alone moved the key, so every candidate would build a new base"
    );
}

#[test]
fn a_manifest_change_keeps_the_layer() {
    let f = Fixture::new();
    assert_eq!(
        f.key_of(&f.manifest),
        f.key_of(&f.base),
        "a change to Cargo.toml alone moved the key: the manifests are in it, so every added \
         test target would rebuild the base, though --locked holds the resolve to the lockfile"
    );
}

#[test]
fn a_lockfile_change_builds_a_new_layer() {
    let f = Fixture::new();
    assert_ne!(
        f.key_of(&f.lockfile),
        f.key_of(&f.manifest),
        "a changed Cargo.lock kept the key, so a candidate would build on a base that lacks its crates"
    );
}

#[test]
fn a_toolchain_change_builds_a_new_layer() {
    let f = Fixture::new();
    assert_ne!(
        f.key_of(&f.toolchain),
        f.key_of(&f.manifest),
        "a changed rust-toolchain.toml kept the key"
    );
}

#[test]
fn a_removed_toolchain_file_builds_a_new_layer() {
    let f = Fixture::new();
    let removed = f.key_of(&f.no_toolchain);
    assert_ne!(
        removed,
        f.key_of(&f.toolchain),
        "a removed toolchain file kept the key"
    );
    assert_ne!(
        removed,
        f.key_of(&f.manifest),
        "a removed toolchain file kept an earlier key"
    );
}

#[test]
fn an_edit_above_the_build_stage_builds_a_new_layer() {
    let f = Fixture::new();
    let edited = f.dockerfile("Dockerfile.above", |t| above_the_build_stage(t, "RUN true"));
    assert_ne!(
        f.key_through(&edited),
        f.key_of(&f.base),
        "an instruction added above the build stage kept the key, though it reaches the layer"
    );
}

#[test]
fn an_edit_below_the_build_stage_keeps_the_layer() {
    let f = Fixture::new();
    let edited = f.dockerfile("Dockerfile.below", |t| format!("{t}RUN true\n"));
    assert_eq!(
        f.key_through(&edited),
        f.key_of(&f.base),
        "an instruction below the build stage moved the key, though nothing there reaches the layer"
    );
}

#[test]
fn a_comment_or_a_blank_line_keeps_the_layer() {
    let f = Fixture::new();
    let edited = f.dockerfile("Dockerfile.comment", |t| {
        above_the_build_stage(&format!("# a comment edit\n\n{t}"), "# another\n")
    });
    assert_eq!(
        f.key_through(&edited),
        f.key_of(&f.base),
        "a comment or a blank line moved the key"
    );
}

#[test]
fn a_parser_directive_builds_a_new_layer() {
    let f = Fixture::new();
    let plain = f.key_of(&f.base);
    let syntax = f.key_through(&f.dockerfile("Dockerfile.syntax", |t| {
        format!("# syntax=docker/dockerfile:1.7\n{t}")
    }));
    let escape = f.key_through(&f.dockerfile("Dockerfile.escape", |t| format!("# escape=`\n{t}")));
    let spaced = f.key_through(&f.dockerfile("Dockerfile.spaced", |t| {
        format!("#Syntax = docker/dockerfile:1.7\n{t}")
    }));
    let check =
        f.key_through(&f.dockerfile("Dockerfile.check", |t| format!("# check=skip=all\n{t}")));
    assert_ne!(
        check, plain,
        "a `# check=` directive kept the key: it is dropped as a comment"
    );
    assert_ne!(
        syntax, plain,
        "a `# syntax=` directive kept the key: it is dropped as a comment, \
         though it changes how the recipe is read"
    );
    assert_ne!(
        escape, plain,
        "a `# escape=` directive kept the key: it is dropped as a comment"
    );
    assert_ne!(syntax, escape, "two directives gave one key");
    assert_ne!(
        spaced, plain,
        "a directive spelled with spaces and another case kept the key: docker reads it"
    );
}

/// BuildKit has two readers of the directives, and they disagree (driven with
/// `docker build --check`): the parser reads `escape` indented and stops at a shebang; the
/// frontend's selection reads `syntax` and `check` past a first-line shebang. The key keeps
/// what either reads.
#[test]
fn a_directive_either_buildkit_reader_takes_builds_a_new_layer() {
    let f = Fixture::new();
    let plain = f.key_of(&f.base);
    let cases = [
        (
            "Dockerfile.indented",
            "  # escape=`\n",
            "an indented `# escape=`",
        ),
        (
            "Dockerfile.shebang-syntax",
            "#!/usr/bin/env docker-build\n# syntax=docker/dockerfile:1.7\n",
            "a `# syntax=` after a first-line shebang",
        ),
        (
            "Dockerfile.shebang-check",
            "#!/usr/bin/env docker-build\n# check=skip=all\n",
            "a `# check=` after a first-line shebang",
        ),
    ];
    for (name, head, what) in cases {
        let key = f.key_through(&f.dockerfile(name, |t| format!("{head}{t}")));
        assert_ne!(
            key, plain,
            "{what} kept the key: BuildKit reads it as a directive, and the key dropped it"
        );
    }
}

#[test]
fn a_directive_shaped_comment_past_the_head_keeps_the_layer() {
    let f = Fixture::new();
    let below_a_comment = f.dockerfile("Dockerfile.late", |t| {
        format!("# a comment first\n# syntax=docker/dockerfile:1.7\n{t}")
    });
    let unknown = f.dockerfile("Dockerfile.unknown", |t| format!("# owner=someone\n{t}"));
    let below_a_blank = f.dockerfile("Dockerfile.blank", |t| {
        format!("\n# syntax=docker/dockerfile:1.7\n{t}")
    });
    assert_eq!(
        f.key_through(&below_a_blank),
        f.key_of(&f.base),
        "a `# syntax=` line after a blank line moved the key; BuildKit reads it as a comment"
    );
    assert_eq!(
        f.key_through(&below_a_comment),
        f.key_of(&f.base),
        "a `# syntax=` line after another comment moved the key; docker reads it as a comment"
    );
    assert_eq!(
        f.key_through(&unknown),
        f.key_of(&f.base),
        "a comment shaped like a directive of no known name moved the key; docker reads it as a comment"
    );
}

#[test]
fn another_platform_builds_a_new_layer() {
    let f = Fixture::new();
    let other = f
        .key(&f.base, &f.harness_dockerfile(), "linux/amd64")
        .expect("a key for linux/amd64");
    assert_ne!(
        other,
        f.key_of(&f.base),
        "two platforms gave one key, so a kept base of one would be taken for the other"
    );
}

#[test]
fn a_key_with_no_platform_is_refused() {
    let f = Fixture::new();
    let got = f.key(&f.base, &f.harness_dockerfile(), "");
    assert_refused(&got, "no platform", "a key with no platform");
}

/// docker builds one platform under many spellings; the key takes one, so one platform never
/// has two bases, and the check after the build never compares two spellings of one.
#[test]
fn a_platform_in_another_spelling_is_refused() {
    let f = Fixture::new();
    for spelling in [
        "linux/aarch64",
        "linux/x86_64",
        "LINUX/ARM64",
        "linux/ARM64",
        "linux/arm64/v8",
        "linux",
    ] {
        let got = f.key(&f.base, &f.harness_dockerfile(), spelling);
        assert_refused(
            &got,
            "is not in the one spelling the base key takes",
            &format!("the platform {spelling:?}"),
        );
        assert_refused(&got, "linux/arm64", &format!("the platform {spelling:?}"));
    }
}

#[test]
fn docker_platform_reads_docker_default_platform_in_the_one_spelling() {
    let (ok, out, err) = sourced(
        "docker_platform",
        &[],
        &[("DOCKER_DEFAULT_PLATFORM", "linux/amd64")],
    );
    assert!(
        ok,
        "docker_platform failed with DOCKER_DEFAULT_PLATFORM=linux/amd64: {err}"
    );
    assert_eq!(
        out, "linux/amd64",
        "docker_platform did not read DOCKER_DEFAULT_PLATFORM"
    );
    let (ok, out, err) = sourced(
        "docker_platform",
        &[],
        &[("DOCKER_DEFAULT_PLATFORM", "linux/aarch64")],
    );
    assert!(
        !ok && out.is_empty() && err.contains("is not in the one spelling"),
        "docker_platform took DOCKER_DEFAULT_PLATFORM=linux/aarch64 ({out:?}, {err:?})"
    );
}

#[test]
fn a_heredoc_above_the_build_stage_is_refused() {
    let f = Fixture::new();
    let heredoc = f.dockerfile("Dockerfile.heredoc", |t| {
        above_the_build_stage(t, "RUN <<EOF\n# content, not a comment\n\nEOF")
    });
    let got = f.key(&f.base, &heredoc, PLATFORM);
    assert_refused(
        &got,
        "a heredoc above the build stage",
        "a heredoc above the build stage",
    );
}

#[test]
fn a_dockerfile_with_no_build_stage_line_gives_no_key() {
    let f = Fixture::new();
    let unmarked = f.dockerfile("Dockerfile.nomark", |t| {
        t.lines()
            .filter(|l| *l != BUILD_STAGE_LINE)
            .map(|l| format!("{l}\n"))
            .collect()
    });
    let got = f.key(&f.base, &unmarked, PLATFORM);
    assert_refused(
        &got,
        "no 'FROM ${JIGC_BASE} AS build' line",
        "a Dockerfile with no build-stage line",
    );
}

/// The key and platform `base_check` is asked about, and the records of an image built
/// right on them.
const CHECK_KEY: &str = "02cf45c1d84001766c8e87105cce52f24fb99a7320de6e9f58a896088c47efa4";
const CHECK_LAYER: &str = "72ad44a63ad83a241d50bb244e5563e0";

fn record(key: &str, layer: &str, platform: &str) -> String {
    format!("key {key}\nlayer {layer}\nplatform {platform}\n")
}

/// `base_check` over one image record, one tag record and docker's report of the image.
fn base_check(image: &str, tag: &str, image_platform: &str) -> (bool, String) {
    let (ok, _, err) = sourced(
        r#"base_check "$2" "$3" "$4" "$5" "$6""#,
        &[CHECK_KEY, PLATFORM, image, tag, image_platform],
        &[],
    );
    (ok, err)
}

#[test]
fn the_check_after_the_build_passes_an_image_built_on_the_base_keyed() {
    let right = record(CHECK_KEY, CHECK_LAYER, PLATFORM);
    let (ok, err) = base_check(&right, &right, PLATFORM);
    assert!(
        ok && err.is_empty(),
        "a right image failed the check: {err}"
    );
}

#[test]
fn the_check_after_the_build_fails_each_way_an_image_can_be_off_its_base() {
    let right = record(CHECK_KEY, CHECK_LAYER, PLATFORM);
    let cases = [
        (
            "a wrong key",
            record(&"0".repeat(64), CHECK_LAYER, PLATFORM),
            right.clone(),
            PLATFORM,
            "records base key",
        ),
        (
            "an empty layer",
            record(CHECK_KEY, "", PLATFORM),
            right.clone(),
            PLATFORM,
            "records no base layer",
        ),
        (
            "a layer that is not the tag's",
            right.clone(),
            record(CHECK_KEY, "ffffffffffffffffffffffffffffffff", PLATFORM),
            PLATFORM,
            "the base tag holds layer",
        ),
        (
            "a base platform recorded in another spelling",
            record(CHECK_KEY, CHECK_LAYER, "linux/aarch64"),
            record(CHECK_KEY, CHECK_LAYER, "linux/aarch64"),
            PLATFORM,
            "the base records platform",
        ),
        (
            "an image docker reports with a variant",
            right.clone(),
            right.clone(),
            "linux/arm64/v8",
            "docker reports the image as",
        ),
    ];
    for (what, image, tag, image_platform, says) in cases {
        let (ok, err) = base_check(&image, &tag, image_platform);
        assert!(!ok, "{what} passed the check after the build");
        assert!(
            err.contains(says),
            "{what} failed the check, but its message does not say {says:?}: {err}"
        );
    }
}

/// `default_repo` from `script`, with `JIGC_REPO` set to `jigc_repo`, run by `bash` — a
/// masked command for a repository of the arm's own, a plain one for this checkout, whose
/// ownership the machine's own git configuration may be what vouches for.
fn default_repo(bash: Command, script: &Path, jigc_repo: &str) -> String {
    let mut bash = bash;
    let out = bash
        .env("JIGC_REPO", jigc_repo)
        .arg("-c")
        .arg(r#". "$1" && default_repo"#)
        .arg("default-repo")
        .arg(script)
        .output()
        .expect("run bash");
    assert!(
        out.status.success(),
        "default_repo failed from {}: {}",
        script.display(),
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

#[test]
fn the_repository_is_the_one_the_harness_sits_in_wherever_that_is() {
    let scratch = ScratchDir::new("trial-image-default-repo");
    let copy = scratch.path().join("elsewhere");
    let harness = copy.join("completions/trial-harness");
    fs::create_dir_all(&harness).expect("create the copy's harness directory");
    // Sourced, never executed, so a plain data copy.
    let text = fs::read(repo_root().join(BUILD_IMAGE)).expect("read build-image.sh");
    let script = harness.join("build-image.sh");
    fs::write(&script, text).expect("write the copy of build-image.sh");
    git(&copy, &["init", "-q"]);
    let want = git(&copy, &["rev-parse", "--show-toplevel"]);
    let this = repo_root();
    assert_ne!(
        Path::new(&want),
        this.as_path(),
        "the copy is not at another path"
    );

    let got = default_repo(hermetic("bash", scratch.path()), &script, "");
    assert_eq!(
        got, want,
        "with no JIGC_REPO the harness read its commits from {got:?}, not from the repository \
         it sits in ({want:?})"
    );
    let got = default_repo(hermetic("bash", scratch.path()), &script, "/elsewhere");
    assert_eq!(
        got, "/elsewhere",
        "JIGC_REPO no longer names the repository"
    );
    let got = default_repo(Command::new("bash"), &repo_root().join(BUILD_IMAGE), "");
    assert_eq!(
        Path::new(&got).canonicalize().ok(),
        Some(this),
        "from this checkout, with no JIGC_REPO, the repository is {got:?}, not this checkout"
    );
}
