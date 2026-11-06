//! **`dev/runner-faithful` runs the image it built, and its header holds the image's
//! posture** (the review of the plan before the opening's task `U0`, its lead L1 —
//! [DECISIONS.md](../DECISIONS.md) → *2026-11-06 — CI's legs re-bounded*).
//!
//! The image's tag is keyed by the toolchain and the platform, never by the script that
//! built it, so a worktree holding an older copy of the script can retag it between this
//! script's `docker build` and its `docker create`: the run then gets an image with no
//! `node` and no `CI`, every suite that needs node skips, and a proxy measured in it is
//! wrong without a word. Two things close that: the container is created from the image
//! id the build wrote (`--iidfile`), never from the tag, and the run's header asserts, in
//! the container, the two parts of the posture that run's measurements stand on — node's
//! major, 22, and `CI=true` — stopping at 125 before anything runs.
//!
//! **Nothing here builds an image.** A stand-in `docker` is put first on `PATH`: its
//! `build` writes the id it is given to the `--iidfile` it is handed, its `create` records
//! the image it was asked for and the script it was handed, and its `start` runs that
//! script here, as the container would, with stand-ins for what the container provides
//! (`node`, `nproc`, `rustc`, the init behind `/proc/1/comm`, and a `cargo` that records
//! that it ran). The script runs in `registry` mode, the one that bundles no source.

use std::fs;
use std::process::{Command, Output};

use crate::support::ci_workflow::repo_root;
use crate::support::scratch::ScratchDir;

use super::placed_executable;

/// The id the stand-in's `build` writes to its `--iidfile`, after `sha256:`.
const IMAGE_ID: &str = "0123456789abcdef";

/// The stand-in `docker`. It logs every call, and keeps what a run's verdict is read from
/// in `$STUB_DIR`: `created-image`, the image `create` was asked for; `inner.sh`, the
/// script it was handed.
const DOCKER: &str = r#"#!/usr/bin/env bash
printf '%s\n' "$*" >>"$STUB_DIR/docker.log"
case "$1" in
    version) [ "${2-}" = --format ] && echo linux/arm64; exit 0 ;;
    info) echo 8; exit 0 ;;
    build)
        shift
        while [ $# -gt 0 ]; do
            case "$1" in
                --iidfile) [ -z "$STUB_IMAGE_ID" ] || printf 'sha256:%s' "$STUB_IMAGE_ID" >"$2"; shift 2 ;;
                *) shift ;;
            esac
        done
        cat >/dev/null
        exit 0 ;;
    create)
        shift
        image=
        while [ $# -gt 0 ] && [ "$1" != bash ]; do image=$1; shift; done
        printf '%s' "$image" >"$STUB_DIR/created-image"
        printf '%s' "$3" >"$STUB_DIR/inner.sh"
        echo stub-container
        exit 0 ;;
    cp|rm) exit 0 ;;
    start)
        if [ -n "${STUB_CI+set}" ]; then export CI="$STUB_CI"; else unset CI; fi
        mkdir -p "$STUB_DIR/home"
        HOME="$STUB_DIR/home" PATH="$STUB_DIR/inside:$PATH" \
            bash -c "$(/bin/cat "$STUB_DIR/inner.sh")" runner-faithful
        exit $? ;;
esac
echo "stand-in docker: no $1" >&2
exit 3
"#;

/// The stand-ins for what the container provides, each `(name, body)`.
const INSIDE: [(&str, &str); 5] = [
    ("node", "#!/bin/sh\necho \"$STUB_NODE\"\n"),
    ("nproc", "#!/bin/sh\necho 4\n"),
    ("rustc", "#!/bin/sh\necho 'rustc 0.0.0 (stand-in)'\n"),
    (
        "cat",
        "#!/bin/sh\nif [ \"$1\" = /proc/1/comm ]; then echo docker-init; else exec /bin/cat \"$@\"; fi\n",
    ),
    (
        "cargo",
        "#!/bin/sh\necho \"cargo $*\" >>\"$STUB_DIR/cargo.log\"\nexit 1\n",
    ),
];

/// One run of `dev/runner-faithful registry`: what it printed and exited with, and what
/// the stand-in recorded.
struct Run {
    out: Output,
    stub: ScratchDir,
}

impl Run {
    fn stderr(&self) -> String {
        String::from_utf8_lossy(&self.out.stderr).into_owned()
    }

    fn recorded(&self, name: &str) -> Option<String> {
        fs::read_to_string(self.stub.path().join(name)).ok()
    }

    fn cargo_ran(&self) -> bool {
        self.stub.path().join("cargo.log").exists()
    }
}

/// Run the script against the stand-in, the container's node printing `node` and its
/// `CI` set to `ci` (unset where `None`), the build writing `image_id` (nothing where
/// empty).
fn run(node: &str, ci: Option<&str>, image_id: &str) -> Run {
    let stub = ScratchDir::new("runner-faithful-stub");
    let dir = stub.path();
    for sub in ["bin", "inside", "tmp"] {
        fs::create_dir_all(dir.join(sub)).expect("create a stand-in directory");
    }
    placed_executable::write(&dir.join("bin/docker"), DOCKER);
    for (name, body) in INSIDE {
        placed_executable::write(&dir.join("inside").join(name), body);
    }
    let path = format!(
        "{}:{}",
        dir.join("bin").display(),
        std::env::var("PATH").unwrap_or_default()
    );
    let mut command = Command::new(repo_root().join("dev/runner-faithful"));
    command
        .arg("registry")
        .env("PATH", path)
        .env("TMPDIR", dir.join("tmp"))
        .env("STUB_DIR", dir)
        .env("STUB_NODE", node)
        .env("STUB_IMAGE_ID", image_id)
        .env_remove("STUB_CI");
    if let Some(ci) = ci {
        command.env("STUB_CI", ci);
    }
    let out = command.output().expect("run dev/runner-faithful");
    Run { out, stub }
}

fn exit(run: &Run) -> Option<i32> {
    run.out.status.code()
}

/// The control: with node 22 and `CI=true` in the container the header passes, and the run
/// reaches its command — the stand-in `cargo`, whose failure is the run's exit 1.
#[test]
fn a_container_in_its_posture_runs_its_command() {
    let run = run("v22.23.4", Some("true"), IMAGE_ID);
    assert_eq!(exit(&run), Some(1), "stderr:\n{}", run.stderr());
    assert!(
        run.cargo_ran(),
        "the command never ran; stderr:\n{}",
        run.stderr()
    );
    assert!(
        run.stderr().contains("node      v22.23.4"),
        "the header names the container's node; stderr:\n{}",
        run.stderr()
    );
}

/// The container is created from the id the build wrote, never from the tag another copy
/// of the script can move.
#[test]
fn the_container_is_created_from_the_image_its_build_wrote() {
    let run = run("v22.23.4", Some("true"), IMAGE_ID);
    let created = run.recorded("created-image");
    assert_eq!(
        created.as_deref(),
        Some(format!("sha256:{IMAGE_ID}").as_str()),
        "`docker create` was asked for {created:?}, not the id the build wrote; stderr:\n{}",
        run.stderr()
    );
}

/// A build that wrote no id stops the run at 125, before any container exists.
#[test]
fn a_build_that_wrote_no_image_id_stops_before_any_container() {
    let run = run("v22.23.4", Some("true"), "");
    assert_eq!(exit(&run), Some(125), "stderr:\n{}", run.stderr());
    assert_eq!(
        run.recorded("created-image"),
        None,
        "a container was created"
    );
}

/// A container whose node is not 22 stops at 125 before its command, naming node.
#[test]
fn a_container_whose_node_is_not_22_stops_before_its_command() {
    let run = run("v20.11.1", Some("true"), IMAGE_ID);
    assert_eq!(exit(&run), Some(125), "stderr:\n{}", run.stderr());
    assert!(
        !run.cargo_ran(),
        "the command ran in a container off its posture"
    );
    assert!(
        run.stderr().contains("v20.11.1"),
        "the refusal names the node it found; stderr:\n{}",
        run.stderr()
    );
}

/// A container without `CI=true` stops at 125 before its command, naming `CI` — unset,
/// and set to anything else.
#[test]
fn a_container_without_ci_true_stops_before_its_command() {
    for ci in [None, Some("1")] {
        let run = run("v22.23.4", ci, IMAGE_ID);
        assert_eq!(
            exit(&run),
            Some(125),
            "CI {ci:?}; stderr:\n{}",
            run.stderr()
        );
        assert!(
            !run.cargo_ran(),
            "CI {ci:?}: the command ran off its posture"
        );
        assert!(
            run.stderr().contains("CI is not"),
            "CI {ci:?}: the refusal names CI; stderr:\n{}",
            run.stderr()
        );
    }
}

/// The stand-in's own sanity: the script it runs is the one `dev/runner-faithful` handed
/// `create`, so the header these arms read is the script's own.
#[test]
fn the_stand_in_runs_the_script_it_was_handed() {
    let run = run("v22.23.4", Some("true"), IMAGE_ID);
    let inner = run.recorded("inner.sh").unwrap_or_default();
    assert!(
        inner.contains("say \"node      $(node --version)\""),
        "the recorded script is not the header's; got {} bytes",
        inner.len()
    );
}
