//! M50 Increment 8 / T1 — the **ref-target fence**, through the real binary.
//!
//! A `type: ref` field is a write address the schema advertises: `jigc doc schema`
//! projects it, `jigc doc set-field` accepts a `<type>:<slug>` value for it, and
//! `schema-conformance.ref-resolves` blocks a finalize over it with a route that says
//! *create the target*. Until this fence, nothing asked whether the declared target
//! doctype exists in the loaded composition — so a pack could advertise an address at
//! every one of those surfaces while the blocking route's `jigc doc create <target>`
//! arm answered `create.unknown-doctype`: a blocking finding with an arm that cannot be
//! run, inside M43's route floor. And `to:` — declared **required** at
//! `design/document-type-schema.md`:79 — was enforced by nothing at all; a pack simply
//! dropping it loaded clean.
//!
//! The fence lives in the `make_pack` assert chain
//! ([`cli::pack::make_pack`](../../src/pack.rs)), beside the freeze gate, so it blocks
//! at **pack-load** rather than at whichever door happens to read the schema. The arms
//! here are what proves that:
//!
//! - **the door axis** — a dev-pack copy whose `adr.supersedes` dangles (`to: research`,
//!   absent from the dev-alone composition) is driven through [`REF_FENCE_DOORS`], every
//!   leaf verb of the CLI, each carrying what a pack-load fault does to it; the same
//!   sweep runs again over a copy that drops `to:` entirely;
//! - **the composed-set rule, proven rather than asserted** — the two shipped
//!   compositions (`[dev]`, `[dev ▸ methodology]`) load clean, and a **listed** pack
//!   whose own doctype refs `to: adr` over the embedded dev base loads clean too: the
//!   membership question is asked of the *composite*, never of a constituent in
//!   isolation, or that cross-pack ref would be foreclosed;
//! - **the not-manifest-gated rule, likewise** — a manifest-less pack copy carrying a
//!   valid ref loads clean, and the same copy with a dangling ref still blocks. Every
//!   other pack-load fence is gated on `config/schema-manifest.yaml` because the
//!   *freeze* is an opt-in about shape stability; this one is composition closure, and a
//!   manifest-less project pack (the M49 PB-1 shape) reaches the identical dead end.
//! - **the layer axis** — the subject is the set the *doors* resolve, so a project
//!   whole-file schema shadow (`.jigc/config/schemas/<ty>.yaml`) is read where it wins.
//!   A shadow can dangle a ref from **either end** — its own `to:`, or the target file's
//!   `type:`, which takes that member out of the map every door keys by the declared
//!   type — and over a **manifest-less** pack's doctype neither freeze arm sees it at
//!   all. Its control: a shadow whose target *is* in the resolved set loads clean, so
//!   the arms measure resolvability rather than the presence of a shadow file.
//!
//! **Why a second door table beside `freeze_enforcement.rs`'s.** Its rows are
//! fixture-shaped — the argv name this file's own `adr:probe`, `ref-probe`,
//! `m-ref-probe` — and it carries a `reach` disposition about *this* fence, not about
//! the freeze; the freeze table additionally carries an inert-gate `clean` axis this one
//! has no use for. Sharing the array would couple two suites' fixtures to buy a
//! deduplication the [`every_leaf_verb_has_a_ref_fence_door`] bijection already makes
//! drift-proof: a verb added anywhere in the CLI reddens **both** tables.

use clap::Parser;
use cli::cli::{VERB_KINDS, VerbKind};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::support::frozen_pack;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-ref-target-{tag}-{}-{:?}",
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

/// Initialize a real git repo with one commit (composition reads HEAD), the
/// `.jigc/config/` project layer the cascade expects, and the `--from-file` payload the
/// `doc set-slot` / `doc author` / `config fill` door rows name.
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
    fs::write(root.join("payload.txt"), "probe prose\n").expect("write payload");
    git(&["add", "."]);
    git(&["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(root.join(".jigc").join("config")).expect("create project layer");
}

/// Rewrite the copied `adr` schema's `supersedes` ref to target **`research`** — a
/// methodology doctype, absent from the dev-alone composition this fixture loads. The
/// declaration stays otherwise byte-identical, so the only thing under test is the
/// target's membership in the composed set.
fn dangle_adr_supersedes(body: &str) -> String {
    body.replacen(
        "{ id: supersedes, type: ref, to: adr,",
        "{ id: supersedes, type: ref, to: research,",
        1,
    )
}

/// Drop the copied `adr` schema's `supersedes` `to:` key entirely — the second refused
/// shape: `to:` is declared **required** for a `type: ref`
/// (`design/document-type-schema.md`:79) and, before this fence, was enforced nowhere.
fn drop_adr_supersedes_target(body: &str) -> String {
    body.replacen(
        "{ id: supersedes, type: ref, to: adr,",
        "{ id: supersedes, type: ref,",
        1,
    )
}

/// A dev-pack copy at `root` carrying `reshape`, with its `adr` `schema-hash`
/// **re-pinned** so the pack passes its own freeze gate — otherwise the freeze would
/// answer first and this suite would be measuring the wrong fence.
fn reshaped_pack(tag: &str, reshape: impl FnOnce(&str) -> String) -> TempDir {
    let dir = TempDir::new(tag);
    frozen_pack::reshaped_dev_pack(dir.path(), "adr", reshape);
    dir
}

/// Run `jigc <argv>` with `cwd = repo`, `$HOME = home`, `JIGC_PACK_DIR = pack` — the
/// **dev-alone** composition (`JIGC_PACK_DIR` supersedes the compose marker).
fn run_dev_alone(repo: &Path, home: &Path, pack: &Path, argv: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(argv)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", pack)
        .output()
        .expect("spawn the jigc binary")
}

/// Run `jigc <argv>` with `cwd = repo`, `$HOME = home` and **no** `JIGC_PACK_DIR` — the
/// embedded-base path, where `packs.yaml` decides what composes over it.
fn run_embedded(repo: &Path, home: &Path, argv: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(argv)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("spawn the jigc binary")
}

/// The one string that says *the ref-target fence fired*, whichever door printed it.
const FENCE_BANNER: &str = "pack-load ref-target fence failed";

// ── The door axis ────────────────────────────────────────────────────────────────────

/// What a pack-load ref-target fault does to one door of [`REF_FENCE_DOORS`].
#[derive(Debug, Clone, Copy)]
enum Reach {
    /// The door's own path reaches `make_pack`, so the fault blocks it and the block
    /// **names the fence**.
    PackLoad,
    /// The door refuses *before* pack-load, on a precondition of its own the sweep
    /// fixture does not satisfy: it still exits non-zero and acts on nothing, but the
    /// diagnosis it prints is that precondition's. `says` is the substring it prints —
    /// asserted, so the disposition is a measurement rather than a claim.
    ///
    /// This is an **ordering** fact, not a hole, and it is proven rather than argued:
    /// [`the_precondition_doors_block_once_their_state_exists`] gives the fixture the
    /// task those rows name and drives every one of them into `make_pack`.
    RefusedEarlier {
        says: &'static str,
        why: &'static str,
    },
    /// The door completes at **exit 0**, because it loads no pack at all. The string
    /// states why that is the right answer rather than a hole.
    PackFree(&'static str),
}

/// One door — the argv after `jigc`, plus what a ref-target fault does to it.
struct RefFenceDoor {
    /// The argv a sweep runs, after `jigc`. Its leading segments must be a
    /// [`VERB_KINDS`] leaf, and the whole argv must parse against the real clap tree;
    /// [`every_leaf_verb_has_a_ref_fence_door`] checks both.
    argv: &'static [&'static str],
    reach: Reach,
}

/// The eight `jigc doc` write verbs' shared disposition: the active-task pre-check runs
/// ahead of pack-load, so over a ref-broken pack they answer with *that*, and they act
/// on nothing.
const NO_ACTIVE_TASK: Reach = Reach::RefusedEarlier {
    says: "no active task",
    why: "the active-task pre-check runs before pack-load and the sweep fixture holds \
          no task. The ordering is not a hole: a task minted before the pack broke \
          carries the same argv into `make_pack`, where it blocks naming the fence. \
          What is proven here is that the door refuses and writes nothing.",
};

/// The task id the [`NO_SUCH_TASK`] rows name — deliberately absent from the sweep
/// fixture, and minted by [`the_precondition_doors_block_once_their_state_exists`].
const TASK_PROBE_ID: &str = "ref-probe";

/// The five task-lifecycle verbs that take an id: the lookup precedes pack-load.
const NO_SUCH_TASK: Reach = Reach::RefusedEarlier {
    says: "no task `ref-probe`",
    why: "the task lookup runs before pack-load and the sweep fixture holds no task \
          `ref-probe`; the door refuses and acts on nothing. With a real id the same \
          argv reaches `make_pack` and blocks naming the fence.",
};

/// **The door axis of the ref-target fence** — every leaf verb of the CLI, each with
/// what a pack whose `ref` target the composition does not contain does to it.
///
/// Membership is **derived, not remembered**: [`every_leaf_verb_has_a_ref_fence_door`]
/// asserts a bijection with [`VERB_KINDS`] — the table that bijects the clap leaf tree —
/// so a verb added anywhere in the CLI owes this table a row before it can ship, and no
/// row may name a verb the tree does not carry.
const REF_FENCE_DOORS: &[RefFenceDoor] = &[
    // ── Top level ────────────────────────────────────────────────────────────────
    RefFenceDoor {
        argv: &["start"],
        reach: Reach::PackLoad,
    },
    RefFenceDoor {
        argv: &["workflow", "single-task", "--preview"],
        reach: Reach::PackLoad,
    },
    RefFenceDoor {
        argv: &["setup"],
        reach: Reach::PackFree(
            "the BOOTSTRAP door: it wires the adapter into the repo and resolves no \
             doctype, so refusing to install over a pack whose ref dangles would be \
             circular — the install is how the operator's agent learns `jigc` exists at \
             all. The diagnosis, with its route, is emitted by the first door that does \
             load the pack. Its freeze twin is dispositioned identically.",
        ),
    },
    RefFenceDoor {
        argv: &["uninstall"],
        reach: Reach::PackFree(
            "the teardown twin of `setup`, exempt for the mirror reason: removing an \
             install must not be gated on the pack being loadable, or a broken pack \
             would trap the operator inside an install they cannot remove.",
        ),
    },
    RefFenceDoor {
        argv: &["upgrade"],
        reach: Reach::PackLoad,
    },
    RefFenceDoor {
        argv: &["ingest"],
        reach: Reach::PackLoad,
    },
    RefFenceDoor {
        argv: &["migrate", "README.md", "--as", "adr"],
        reach: Reach::PackLoad,
    },
    RefFenceDoor {
        argv: &["migrate-corpus"],
        reach: Reach::PackLoad,
    },
    RefFenceDoor {
        argv: &["unmanage", "README.md"],
        reach: Reach::PackLoad,
    },
    RefFenceDoor {
        argv: &["rename", "adr:probe", "--to", "A New Title"],
        reach: Reach::PackLoad,
    },
    RefFenceDoor {
        argv: &["relocate", "vision", "--from", "docs/vision/"],
        reach: Reach::PackLoad,
    },
    RefFenceDoor {
        argv: &["describe"],
        reach: Reach::PackLoad,
    },
    RefFenceDoor {
        argv: &["validate"],
        reach: Reach::PackLoad,
    },
    // ── `jigc doc` — the managed-doc surface ─────────────────────────────────────
    RefFenceDoor {
        argv: &["doc", "create", "adr", "--title", "Ref Probe"],
        reach: NO_ACTIVE_TASK,
    },
    RefFenceDoor {
        argv: &[
            "doc",
            "add-item",
            "adr:probe#options",
            "--title",
            "Ref Probe",
        ],
        reach: NO_ACTIVE_TASK,
    },
    RefFenceDoor {
        argv: &["doc", "remove-item", "adr:probe#options/ref-probe"],
        reach: NO_ACTIVE_TASK,
    },
    RefFenceDoor {
        argv: &[
            "doc",
            "retitle-item",
            "adr:probe#options/ref-probe",
            "--title",
            "Ref Probe",
        ],
        reach: NO_ACTIVE_TASK,
    },
    RefFenceDoor {
        argv: &["doc", "rename", "adr:probe", "--to", "A New Title"],
        reach: NO_ACTIVE_TASK,
    },
    RefFenceDoor {
        argv: &[
            "doc",
            "set-field",
            "adr:probe#status",
            "--value",
            "accepted",
        ],
        reach: NO_ACTIVE_TASK,
    },
    RefFenceDoor {
        argv: &[
            "doc",
            "set-slot",
            "adr:probe#context",
            "--from-file",
            "payload.txt",
        ],
        reach: NO_ACTIVE_TASK,
    },
    RefFenceDoor {
        argv: &["doc", "author", "adr", "--from-file", "payload.txt"],
        reach: NO_ACTIVE_TASK,
    },
    RefFenceDoor {
        argv: &["doc", "show", "adr:probe"],
        reach: Reach::PackLoad,
    },
    RefFenceDoor {
        argv: &["doc", "schema", "adr"],
        reach: Reach::PackLoad,
    },
    RefFenceDoor {
        argv: &["doc", "list"],
        reach: Reach::PackLoad,
    },
    // ── `jigc task` — the task lifecycle ─────────────────────────────────────────
    RefFenceDoor {
        argv: &["task", "list"],
        reach: Reach::PackFree(
            "a WORKBENCH listing: it reads `.jigc/tasks/` and reports ids, intents and \
             workflows, resolving no doctype schema — so a dangling ref has nothing to \
             say about it, and blocking it would deny the operator the view of in-flight \
             work at exactly the moment every other door is refusing.",
        ),
    },
    RefFenceDoor {
        argv: &["task", "diff", TASK_PROBE_ID],
        reach: NO_SUCH_TASK,
    },
    RefFenceDoor {
        argv: &["task", "validate", TASK_PROBE_ID],
        reach: NO_SUCH_TASK,
    },
    RefFenceDoor {
        argv: &["task", "discard", TASK_PROBE_ID],
        reach: NO_SUCH_TASK,
    },
    RefFenceDoor {
        argv: &["task", "finalize", TASK_PROBE_ID],
        reach: NO_SUCH_TASK,
    },
    RefFenceDoor {
        argv: &["task", "bind", "decision", "adr:probe", TASK_PROBE_ID],
        reach: NO_SUCH_TASK,
    },
    // ── `jigc config` — the cascade surface ──────────────────────────────────────
    RefFenceDoor {
        argv: &["config", "set", "docs-root", "docs"],
        reach: Reach::PackLoad,
    },
    RefFenceDoor {
        argv: &[
            "config",
            "insert-step",
            "--workflow",
            "single-task",
            "--after",
            "orient",
            "payload.txt",
        ],
        reach: Reach::PackLoad,
    },
    RefFenceDoor {
        argv: &[
            "config",
            "replace-step",
            "workflow:single-task#orient",
            "payload.txt",
        ],
        reach: Reach::PackLoad,
    },
    RefFenceDoor {
        argv: &["config", "remove-step", "workflow:single-task#orient"],
        reach: Reach::PackLoad,
    },
    RefFenceDoor {
        argv: &[
            "config",
            "fill",
            "step:orient#ref-probe",
            "--from-file",
            "payload.txt",
        ],
        reach: Reach::PackLoad,
    },
    RefFenceDoor {
        argv: &["config", "fork", "workflow:single-task#orient"],
        reach: Reach::PackLoad,
    },
    RefFenceDoor {
        argv: &["config", "get", "docs-root"],
        reach: Reach::PackLoad,
    },
    RefFenceDoor {
        argv: &["config", "list"],
        reach: Reach::PackLoad,
    },
    // ── `jigc milestone` — the work-unit surface ─────────────────────────────────
    RefFenceDoor {
        argv: &["milestone", "create", "Ref Probe"],
        reach: Reach::PackLoad,
    },
    RefFenceDoor {
        argv: &["milestone", "add-task", "m-ref-probe", "probe intent"],
        reach: Reach::PackLoad,
    },
    RefFenceDoor {
        argv: &["milestone", "add-from-spec", "m-ref-probe", "spec:probe"],
        reach: Reach::PackLoad,
    },
    RefFenceDoor {
        argv: &["milestone", "list-tasks", "m-ref-probe"],
        reach: Reach::PackLoad,
    },
    RefFenceDoor {
        argv: &["milestone", "provision", "m-ref-probe"],
        reach: Reach::PackLoad,
    },
    RefFenceDoor {
        argv: &["milestone", "execute", "m-ref-probe"],
        reach: Reach::PackLoad,
    },
    RefFenceDoor {
        argv: &["milestone", "join", "m-ref-probe"],
        reach: Reach::PackLoad,
    },
    RefFenceDoor {
        argv: &["milestone", "finalize", "m-ref-probe"],
        reach: Reach::PackLoad,
    },
    RefFenceDoor {
        argv: &["milestone", "discard", "m-ref-probe"],
        reach: Reach::PackLoad,
    },
];

/// The [`VERB_KINDS`] leaf a door's argv reaches — the longest classified prefix, and
/// its read/write kind.
fn verb_of(argv: &[&str]) -> (&'static [&'static str], VerbKind) {
    VERB_KINDS
        .iter()
        .filter(|(path, _)| argv.len() >= path.len() && argv[..path.len()] == **path)
        .max_by_key(|(path, _)| path.len())
        .map(|(path, kind)| (*path, *kind))
        .unwrap_or_else(|| panic!("`jigc {}` reaches no VERB_KINDS leaf", argv.join(" ")))
}

/// Assert one door's declared [`Reach`] against a ref-broken pack.
fn assert_broken_door(
    label: &str,
    repo: &Path,
    home: &Path,
    pack: &Path,
    door: &RefFenceDoor,
    needles: &[&str],
) {
    let argv = door.argv;
    let out = run_dev_alone(repo, home, pack, argv);
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let said = format!("stdout:\n{stdout}\nstderr:\n{stderr}");
    match door.reach {
        Reach::PackLoad => {
            assert!(
                !out.status.success(),
                "[{label}] `jigc {}` is declared `PackLoad` — it must exit non-zero over \
                 a pack whose ref target the composition lacks; {said}",
                argv.join(" "),
            );
            for needle in needles {
                assert!(
                    stderr.contains(needle),
                    "[{label}] `jigc {}` stderr must name {needle:?}; got:\n{stderr}",
                    argv.join(" "),
                );
            }
        }
        Reach::RefusedEarlier { says, why } => {
            assert!(
                !out.status.success(),
                "[{label}] `jigc {}` is declared `RefusedEarlier` ({why}) — it must still \
                 exit non-zero, acting on nothing; {said}",
                argv.join(" "),
            );
            assert!(
                stderr.contains(says),
                "[{label}] `jigc {}` must refuse with {says:?} ({why}); got:\n{stderr}",
                argv.join(" "),
            );
            assert!(
                !stderr.contains(FENCE_BANNER) && !stdout.contains(FENCE_BANNER),
                "[{label}] `jigc {}` is declared `RefusedEarlier` but named the fence — \
                 it now reaches pack-load, so its row owes `Reach::PackLoad`; {said}",
                argv.join(" "),
            );
        }
        Reach::PackFree(why) => {
            assert!(
                out.status.success(),
                "[{label}] `jigc {}` is declared pack-free ({why}) — it must complete at \
                 exit 0; {said}",
                argv.join(" "),
            );
            assert!(
                !stderr.contains(FENCE_BANNER) && !stdout.contains(FENCE_BANNER),
                "[{label}] the pack-free door `jigc {}` must not name the fence; {said}",
                argv.join(" "),
            );
        }
    }
}

/// Drive **every** door of [`REF_FENCE_DOORS`] against a pack carrying `reshape`,
/// asserting each door's declared disposition and that every blocked door names
/// `needles`.
///
/// The doors that never act share one fixture — sound, because *a blocked door acting*
/// is precisely what the sweep denies. Each [`Reach::PackFree`] door gets its own,
/// because `jigc uninstall` removes the `.jigc/` tree and `jigc setup` rewrites it, so
/// sharing would erase the state the remaining doors are meant to meet.
fn sweep_broken(label: &str, reshape: impl Fn(&str) -> String, needles: &[&str]) {
    let shared_repo = TempDir::new(&format!("{label}-repo"));
    let shared_home = TempDir::new(&format!("{label}-home"));
    let shared_pack = reshaped_pack(&format!("{label}-pack"), |body| reshape(body));
    init_repo(shared_repo.path());

    for door in REF_FENCE_DOORS {
        if matches!(door.reach, Reach::PackFree(_)) {
            let repo = TempDir::new(&format!("{label}-solo-repo"));
            let home = TempDir::new(&format!("{label}-solo-home"));
            let pack = reshaped_pack(&format!("{label}-solo-pack"), |body| reshape(body));
            init_repo(repo.path());
            assert_broken_door(label, repo.path(), home.path(), pack.path(), door, needles);
        } else {
            assert_broken_door(
                label,
                shared_repo.path(),
                shared_home.path(),
                shared_pack.path(),
                door,
                needles,
            );
        }
    }
}

/// **(a)** A `ref` whose `to:` names a doctype the loaded composition does not contain
/// blocks every door, naming the doctype, the field, the dangling target and the loaded
/// pack-set. `research` is a *real* methodology doctype — it exists, just not in this
/// composition — which is exactly the shape D7 refused for `adr —supersedes→ research`.
#[test]
fn a_dangling_ref_target_blocks_every_door() {
    sweep_broken(
        "dangling",
        dangle_adr_supersedes,
        &[
            FENCE_BANNER,
            "adr",
            "supersedes",
            "to: research",
            "loaded pack-set: dev/",
        ],
    );
}

/// **(b)** A `type: ref` that declares no `to:` at all does the same. The key is
/// documented **required** (`design/document-type-schema.md`:79) and was enforced by
/// nothing: `check_ref_shape` carried the tell in its own comment — *"Type-equality only
/// when `to` is declared … (no shipped ref carries `to: None`)"* — a convention held by
/// a convention.
#[test]
fn a_ref_declaring_no_target_blocks_every_door() {
    sweep_broken(
        "no-target",
        drop_adr_supersedes_target,
        &[
            FENCE_BANNER,
            "adr",
            "supersedes",
            "declaring no `to:`",
            "loaded pack-set: dev/",
        ],
    );
}

/// The other half of [`Reach::RefusedEarlier`]: those thirteen doors are not *outside*
/// the fence, they are **ordered behind a precondition of their own**. Give the fixture
/// the task their rows name — minted before the pack breaks, which is the real sequence:
/// work in flight when someone edits a schema — and every one of them reaches
/// `make_pack` and blocks naming the fence.
///
/// Without this arm the sweep's answer for a third of the door set would stop at *"it
/// refuses for some other reason"* — the safety half, but not the claim.
#[test]
fn the_precondition_doors_block_once_their_state_exists() {
    let repo = TempDir::new("ordering-repo");
    let home = TempDir::new("ordering-home");
    let pack = TempDir::new("ordering-pack");
    frozen_pack::copy_dev_pack(pack.path());
    init_repo(repo.path());

    // The task those rows name, minted against an INTACT pack — the id is asserted
    // rather than read back, because the rows spell it literally.
    let minted = run_dev_alone(
        repo.path(),
        home.path(),
        pack.path(),
        &["start", "--workflow", "single-task", "ref probe"],
    );
    let stdout = String::from_utf8_lossy(&minted.stdout);
    assert!(
        minted.status.success(),
        "the fixture task must mint against an intact pack; stdout:\n{stdout}\nstderr:\n{}",
        String::from_utf8_lossy(&minted.stderr),
    );
    assert!(
        stdout.contains(&format!("task minted: {TASK_PROBE_ID}")),
        "the minted id must be the one REF_FENCE_DOORS' precondition rows name \
         (`{TASK_PROBE_ID}`); got:\n{stdout}",
    );

    // Now break the ref — the schema edit lands under the task, exactly as it would.
    let schema = pack.path().join("schemas").join("adr.yaml");
    let body = fs::read_to_string(&schema).expect("read the copied adr.yaml");
    fs::write(&schema, dangle_adr_supersedes(&body)).expect("write the dangling adr.yaml");
    frozen_pack::repin_manifest_hash(pack.path(), "adr");

    let ordered: Vec<&RefFenceDoor> = REF_FENCE_DOORS
        .iter()
        .filter(|door| matches!(door.reach, Reach::RefusedEarlier { .. }))
        .collect();
    assert!(
        !ordered.is_empty(),
        "the `RefusedEarlier` disposition must have members for this arm to mean anything",
    );
    for door in ordered {
        let out = run_dev_alone(repo.path(), home.path(), pack.path(), door.argv);
        let stderr = String::from_utf8_lossy(&out.stderr);
        assert!(
            !out.status.success(),
            "`jigc {}` must block once its task exists; stdout:\n{}\nstderr:\n{stderr}",
            door.argv.join(" "),
            String::from_utf8_lossy(&out.stdout),
        );
        for needle in [FENCE_BANNER, "supersedes", "to: research"] {
            assert!(
                stderr.contains(needle),
                "`jigc {}` must now name the fence ({needle:?}) rather than its own \
                 precondition; got:\n{stderr}",
                door.argv.join(" "),
            );
        }
    }
}

/// **The membership fence.** [`REF_FENCE_DOORS`] bijects [`VERB_KINDS`] — the registry
/// that bijects the clap leaf tree — so the door axis is *derived* rather than
/// remembered, and each argv parses against the real CLI rather than against a plausible
/// spelling of it. A verb added anywhere in the tree reddens here until its ref-fence
/// disposition is stated.
#[test]
fn every_leaf_verb_has_a_ref_fence_door() {
    let mut covered: BTreeSet<Vec<&str>> = BTreeSet::new();
    for door in REF_FENCE_DOORS {
        let mut argv = vec!["jigc"];
        argv.extend_from_slice(door.argv);
        cli::cli::Cli::try_parse_from(&argv).unwrap_or_else(|err| {
            panic!(
                "a REF_FENCE_DOORS argv must parse against the real clap tree — \
                 `{}` did not:\n{err}",
                argv.join(" "),
            )
        });
        let (leaf, _) = verb_of(door.argv);
        assert!(
            covered.insert(leaf.to_vec()),
            "`jigc {}` is covered twice — one door per leaf verb",
            leaf.join(" "),
        );
    }

    let declared: BTreeSet<Vec<&str>> = VERB_KINDS.iter().map(|(path, _)| path.to_vec()).collect();
    let missing: Vec<String> = declared
        .difference(&covered)
        .map(|path| path.join(" "))
        .collect();
    assert!(
        missing.is_empty(),
        "every leaf verb owes REF_FENCE_DOORS a row stating what a dangling `ref` target \
         does to it — undisposed: {missing:?}",
    );
    let stray: Vec<String> = covered
        .difference(&declared)
        .map(|path| path.join(" "))
        .collect();
    assert!(
        stray.is_empty(),
        "REF_FENCE_DOORS names verbs the clap tree does not carry: {stray:?}",
    );
}

// ── The composed-set rule ────────────────────────────────────────────────────────────

/// **(c)** The two shipped compositions load clean. Every shipped `type: ref` is
/// intra-pack — `commit.implements → spec`, `adr.supersedes → adr`,
/// `spec.derived-from → prd`, `arch-doc.cites → adr`, `vision.grounded-in → research` —
/// so neither `[dev]` nor `[dev ▸ methodology]` is missing a target, and the fence is
/// silent over both.
#[test]
fn the_two_shipped_compositions_load_clean() {
    for (label, marker) in [("dev-alone", false), ("dev-methodology", true)] {
        let repo = TempDir::new(&format!("shipped-{label}-repo"));
        let home = TempDir::new(&format!("shipped-{label}-home"));
        init_repo(repo.path());
        if marker {
            fs::write(
                repo.path().join(".jigc").join("config").join("packs.yaml"),
                "compose-embedded-methodology: true\n",
            )
            .expect("write the compose marker");
        }
        let out = run_embedded(repo.path(), home.path(), &["describe"]);
        assert!(
            out.status.success(),
            "[{label}] the shipped composition must load clean; stdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        );
        assert!(
            !String::from_utf8_lossy(&out.stderr).contains(FENCE_BANNER),
            "[{label}] the shipped composition must not trip the ref-target fence",
        );
    }
}

/// Seed a minimal **listed** pack at `root` declaring one doctype whose header field is
/// a `ref` `to: <target>`. Manifest-less by construction, so no other pack-load fence
/// has anything to say about it — the arm under test is the ref target alone.
fn seed_listed_pack(root: &Path, target: &str) {
    fs::create_dir_all(root.join("schemas")).expect("mk schemas/");
    fs::create_dir_all(root.join("config")).expect("mk config/");
    fs::write(
        root.join("config").join("defaults.yaml"),
        "pack-id: house\n",
    )
    .expect("seed the pack id");
    fs::write(
        root.join("schemas").join("house-note.yaml"),
        format!(
            "type: house-note\n\
             location: house-notes/\n\
             id-from: title\n\
             sections:\n\
             \x20 - id: meta\n\
             \x20   header: true\n\
             \x20   fields:\n\
             \x20     - {{ id: title, type: string }}\n\
             \x20     - {{ id: about, type: ref, to: {target}, card: \"0..1\", inverse: noted-by }}\n\
             \x20 - id: body\n\
             \x20   slot: {{ hint: What the note says. }}\n"
        ),
    )
    .expect("seed the house-note schema");
}

/// Record the listed pack in the project layer's `packs.yaml` — the pre-cascade selector
/// `make_pack()` CWD-discovers.
fn list_pack(repo: &Path, pack: &Path) {
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", pack.display()),
    )
    .expect("write packs.yaml naming the listed pack");
}

/// **(e)** The composed-set rule, proven and not asserted: a **listed** pack whose own
/// doctype refs `to: adr` — a doctype the *embedded base* provides, not itself — loads
/// clean. Asked of each constituent in isolation (the freeze's posture) this ref would
/// be refused, which would foreclose exactly the cross-pack relation a composition
/// legitimately supports.
#[test]
fn a_listed_pack_may_target_a_doctype_the_base_provides() {
    let repo = TempDir::new("listed-repo");
    let home = TempDir::new("listed-home");
    let pack = TempDir::new("listed-pack");
    init_repo(repo.path());
    seed_listed_pack(pack.path(), "adr");
    list_pack(repo.path(), pack.path());

    let out = run_embedded(repo.path(), home.path(), &["describe"]);
    assert!(
        out.status.success(),
        "a listed pack's ref to a base-provided doctype must load clean; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// Its discriminating twin: the same listed pack whose ref names a doctype **no**
/// constituent provides is refused — so the arm above measures composed-set membership
/// rather than a fence that never fires on a listed pack at all.
#[test]
fn a_listed_pack_targeting_nothing_in_the_set_is_refused() {
    let repo = TempDir::new("listed-bad-repo");
    let home = TempDir::new("listed-bad-home");
    let pack = TempDir::new("listed-bad-pack");
    init_repo(repo.path());
    seed_listed_pack(pack.path(), "nowhere");
    list_pack(repo.path(), pack.path());

    let out = run_embedded(repo.path(), home.path(), &["describe"]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !out.status.success(),
        "a listed pack's ref to a doctype no constituent provides must block; stdout:\n{}\nstderr:\n{stderr}",
        String::from_utf8_lossy(&out.stdout),
    );
    for needle in [FENCE_BANNER, "house-note", "about", "to: nowhere"] {
        assert!(
            stderr.contains(needle),
            "the refusal must name {needle:?}; got:\n{stderr}",
        );
    }
}

// ── The not-manifest-gated rule ──────────────────────────────────────────────────────

/// **(d)** A **manifest-less** dev-pack copy — the wholesale freeze opt-out — carrying
/// its shipped, valid refs loads clean.
#[test]
fn a_manifest_less_pack_with_valid_refs_loads_clean() {
    let repo = TempDir::new("mless-clean-repo");
    let home = TempDir::new("mless-clean-home");
    let pack = TempDir::new("mless-clean-pack");
    init_repo(repo.path());
    frozen_pack::manifest_less_dev_pack(pack.path());

    let out = run_dev_alone(repo.path(), home.path(), pack.path(), &["describe"]);
    assert!(
        out.status.success(),
        "a manifest-less pack with valid refs must load clean; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// Its discriminating twin, and the **scope statement made checkable**: the same
/// manifest-less copy with a dangling `to:` is still refused. Dropping the manifest is
/// the documented escape from the *freeze*; it is not an escape from composition
/// closure, because a manifest-less project pack (the M49 PB-1 shape) whose ref dangles
/// reaches the identical unfollowable route on an adopter's machine.
#[test]
fn a_manifest_less_pack_with_a_dangling_ref_is_still_refused() {
    let repo = TempDir::new("mless-broken-repo");
    let home = TempDir::new("mless-broken-home");
    let pack = TempDir::new("mless-broken-pack");
    init_repo(repo.path());
    frozen_pack::manifest_less_dev_pack(pack.path());
    let schema = pack.path().join("schemas").join("adr.yaml");
    let body = fs::read_to_string(&schema).expect("read the copied adr.yaml");
    let dangled = dangle_adr_supersedes(&body);
    assert_ne!(body, dangled, "the reshape must actually change adr.yaml");
    fs::write(&schema, dangled).expect("write the dangling adr.yaml");

    let out = run_dev_alone(repo.path(), home.path(), pack.path(), &["describe"]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !out.status.success(),
        "a manifest-less pack is outside the FREEZE, not outside composition closure; \
         stdout:\n{}\nstderr:\n{stderr}",
        String::from_utf8_lossy(&out.stdout),
    );
    assert!(
        stderr.contains(FENCE_BANNER),
        "the refusal must be the ref-target fence's, not the freeze's; got:\n{stderr}",
    );
}

// ── The locus axis ───────────────────────────────────────────────────────────────────

/// The fence walks **every locus**, not header fields alone: a `ref` declared as a leaf
/// of a repeatable item block is refused the same way at **every depth**, and each
/// refusal says *where*.
///
/// `engine::index`'s edge extraction already lifts a `ref` from a `Leaf::Field`, so an
/// item-level ref is a real shape — and a fence sweeping only the header-field locus
/// would be the incomplete sweep this wave exists to refuse. No shipped doctype declares
/// one, so the case is **manufactured**: the `changelog`'s shared `change-group`
/// fragment gains a dangling ref, and the fragment is included at *both* the staging
/// area (locus 2) and inside each cut release (locus 3) — so one edit yields one refusal
/// per locus, each naming its own hop path, which is what makes the recursion visible
/// rather than assumed. Locus 1 is the axis the two sweeps above already drive.
#[test]
fn an_item_level_ref_target_is_fenced_at_every_locus() {
    let repo = TempDir::new("item-locus-repo");
    let home = TempDir::new("item-locus-home");
    let pack = TempDir::new("item-locus-pack");
    init_repo(repo.path());
    frozen_pack::manifest_less_dev_pack(pack.path());

    // The shared `change-group` fragment gains a dangling ref leaf.
    let schema = pack.path().join("schemas").join("changelog.yaml");
    let body = fs::read_to_string(&schema).expect("read the copied changelog.yaml");
    let anchor =
        "    - { id: notes, slot: { hint: \"One bullet per change in this category.\" } }\n";
    assert!(
        body.contains(anchor),
        "changelog.yaml must carry the shared `change-group` fragment this arm reshapes",
    );
    let reshaped = body.replacen(
        anchor,
        &format!(
            "{anchor}    - {{ id: about, type: ref, to: nowhere, card: \"0..1\", inverse: noted-by }}\n"
        ),
        1,
    );
    fs::write(&schema, reshaped).expect("write the item-ref changelog.yaml");

    let out = run_dev_alone(repo.path(), home.path(), pack.path(), &["describe"]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !out.status.success(),
        "an item-block ref target outside the composed set must block; stdout:\n{}\nstderr:\n{stderr}",
        String::from_utf8_lossy(&out.stdout),
    );
    for needle in [
        FENCE_BANNER,
        "`changelog`'s `#unreleased-changes` item field `about`",
        "`changelog`'s `#releases/changes` item field `about`",
        "to: nowhere",
    ] {
        assert!(
            stderr.contains(needle),
            "the refusal must name {needle:?} — the locus, not just the doctype; got:\n{stderr}",
        );
    }
}

// ── The layer axis: the project's whole-file schema shadow ───────────────────────────

/// Seed a manifest-less listed pack shipping **two** doctypes — `house-note`, whose
/// `about` ref targets `house-tag`, and `house-tag` itself — so the *target end* of a
/// relation is a file the project layer can shadow. The single-doctype
/// [`seed_listed_pack`] covers the `to:` end.
fn seed_listed_pack_pair(root: &Path) {
    seed_listed_pack(root, "house-tag");
    fs::write(
        root.join("schemas").join("house-tag.yaml"),
        "type: house-tag\n\
         location: house-tags/\n\
         id-from: title\n\
         sections:\n\
         \x20 - id: body\n\
         \x20   slot: { hint: What the tag means. }\n",
    )
    .expect("seed the house-tag schema");
}

/// Write a project-layer whole-file schema shadow at `.jigc/config/schemas/<ty>.yaml`
/// (`design/overrides.md` → Authored metadata on a definition resolves by whole-file
/// shadow) — the layer that outranks every pack, and the one every door resolves a
/// doctype through ([`CascadeDefs::all_schemas`]).
fn shadow_schema(repo: &Path, ty: &str, body: &str) {
    let dir = repo.join(".jigc").join("config").join("schemas");
    fs::create_dir_all(&dir).expect("mk the project schema shadow dir");
    fs::write(dir.join(format!("{ty}.yaml")), body).expect("write the project schema shadow");
}

/// **The layer axis, arm 1 — the `to:` end.** The fence's subject is the schema set the
/// **doors** resolve, not the pack composite alone: a project whole-file shadow of a
/// **manifest-less** listed pack's doctype (the M49 PB-1 shape) is seen by neither the
/// freeze — which is manifest-gated at both layers — nor, before this, by the ref-target
/// fence, so it could point a `ref` at a doctype no layer provides. Every surface then
/// advertised an address that cannot resolve: `jigc doc schema` printed `ref -> nowhere`
/// at exit 0, and `jigc validate` reached the engine's inverse-cardinality sweep, whose
/// `debug_assert!` states that no production door can hand it an open ref target.
#[test]
fn a_project_schema_shadow_cannot_dangle_a_ref_target() {
    let repo = TempDir::new("shadow-dangle-repo");
    let home = TempDir::new("shadow-dangle-home");
    let pack = TempDir::new("shadow-dangle-pack");
    init_repo(repo.path());
    seed_listed_pack(pack.path(), "adr");
    list_pack(repo.path(), pack.path());
    let shipped = fs::read_to_string(pack.path().join("schemas").join("house-note.yaml"))
        .expect("read the seeded house-note.yaml");
    let shadowed = shipped.replace("to: adr", "to: nowhere");
    assert_ne!(
        shipped, shadowed,
        "the shadow must actually move the target"
    );
    shadow_schema(repo.path(), "house-note", &shadowed);

    for argv in [
        &["describe"][..],
        &["validate"][..],
        &["doc", "schema", "house-note"][..],
    ] {
        let out = run_embedded(repo.path(), home.path(), argv);
        let stderr = String::from_utf8_lossy(&out.stderr);
        assert!(
            !out.status.success(),
            "[{argv:?}] a project shadow that dangles a ref must be refused at pack-load; stdout:\n{}\nstderr:\n{stderr}",
            String::from_utf8_lossy(&out.stdout),
        );
        for needle in [FENCE_BANNER, "house-note", "about", "to: nowhere"] {
            assert!(
                stderr.contains(needle),
                "[{argv:?}] the refusal must name {needle:?}; got:\n{stderr}",
            );
        }
    }
}

/// **The layer axis, arm 2 — the target end.** A ref dangles when either end moves, and
/// the project layer can move the *other* one: a shadow of the **target** doctype that
/// changes its `type:` takes that member out of the resolved set (the map every door
/// reads is keyed by the declared `type:`, never by the filename), leaving the shipped
/// `to: house-tag` pointing at nothing. Checked over the resolved set this falls out of
/// the same membership question; checked over the pack composite it is invisible.
#[test]
fn a_project_schema_shadow_cannot_move_the_target_out_of_the_set() {
    let repo = TempDir::new("shadow-target-repo");
    let home = TempDir::new("shadow-target-home");
    let pack = TempDir::new("shadow-target-pack");
    init_repo(repo.path());
    seed_listed_pack_pair(pack.path());
    list_pack(repo.path(), pack.path());
    let shipped = fs::read_to_string(pack.path().join("schemas").join("house-tag.yaml"))
        .expect("read the seeded house-tag.yaml");
    let shadowed = shipped.replace("type: house-tag", "type: house-badge");
    assert_ne!(
        shipped, shadowed,
        "the shadow must actually rename the type"
    );
    shadow_schema(repo.path(), "house-tag", &shadowed);

    let out = run_embedded(repo.path(), home.path(), &["describe"]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !out.status.success(),
        "a project shadow that renames the target doctype must be refused; stdout:\n{}\nstderr:\n{stderr}",
        String::from_utf8_lossy(&out.stdout),
    );
    for needle in [FENCE_BANNER, "house-note", "about", "to: house-tag"] {
        assert!(
            stderr.contains(needle),
            "the refusal must name {needle:?}; got:\n{stderr}",
        );
    }
}

/// Its discriminating twin: the fence reads the shadow, it does not refuse shadows. The
/// same pack with a shadow that re-points `about` at another doctype of the resolved set
/// loads clean — so the two arms above measure resolvability, not the presence of a
/// project schema file.
#[test]
fn a_project_schema_shadow_with_a_resolvable_target_loads_clean() {
    let repo = TempDir::new("shadow-clean-repo");
    let home = TempDir::new("shadow-clean-home");
    let pack = TempDir::new("shadow-clean-pack");
    init_repo(repo.path());
    seed_listed_pack(pack.path(), "adr");
    list_pack(repo.path(), pack.path());
    let shipped = fs::read_to_string(pack.path().join("schemas").join("house-note.yaml"))
        .expect("read the seeded house-note.yaml");
    let shadowed = shipped.replace("to: adr", "to: spec");
    assert_ne!(
        shipped, shadowed,
        "the shadow must actually move the target"
    );
    shadow_schema(repo.path(), "house-note", &shadowed);

    let out = run_embedded(repo.path(), home.path(), &["describe"]);
    assert!(
        out.status.success(),
        "a project shadow whose ref target IS in the resolved set must load clean; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}
