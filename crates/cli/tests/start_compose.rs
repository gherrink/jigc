//! End-to-end integration test for `jigc start "<intent>"` composition.
//!
//! Drives the built `jigc` binary against a throwaway temp git repo and asserts
//! the Increment-3 deliverable (`implementation/roadmap.md` → Increment 3).
//! Post-flip (`DECISIONS.md` 2026-06-01 → M2 flips `default-workflow` to
//! `router`), a bare intent-bearing `jigc start` composes the cascade default —
//! the `creates-task: false` router — listing the selectable work-workflows
//! without minting; the explicit `--workflow <X>` (Form D) front door mints + runs
//! the `workflow-refs` gate + composes `<X>` with `{{task.intent}}` = the intent,
//! printing the composed four-class view through the selected format.
//!
//! No external test crates: the binary path comes from Cargo's
//! `CARGO_BIN_EXE_jigc`, the temp repo is a real `git init` (composition mints,
//! which reads HEAD), and a self-cleaning `TempDir` keeps the test off the
//! developer's real repo / `~/.config`.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-start-compose-{tag}-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        );
        path.push(unique);
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

const ROUTING_FOOTER: &str =
    "— jigc · run `jigc start` for orientation; all writes through `jigc`.";

/// The **no-override** bare-`jigc start "<intent>"` agent-text composition, byte
/// for byte — the cascade default (`router`) resolved with no project `scalar:`
/// delta. This is the determinism gate for wiring the resolved cascade live into
/// compose (M4 Increment 1, T5): the no-delta read path must stay byte-identical
/// to before the cascade was wired in. The router lists the selectable
/// work-workflows and carries no `{{task.intent}}`, so the golden is intent-stable.
const NO_OVERRIDE_ROUTER_GOLDEN: &str = "\
These are the selectable work-workflows, each with the situation it fits:

- architecture-documentation — document the architecture of a part of the system, tying its components to the code that implements them
- implement-from-spec — a committed spec already covers the intent, with acceptance criteria to build against
- plan — draft the specification for upcoming work before writing any code
- project-setup — bootstrap a brand-new project by developing the idea into its first product requirements
- quick-fix — apply a small commit-only fix that touches no documented code (code a managed doc names) and records no decision
- record-decision — capture a choice you have settled, preserving its rationale with no code to write
- single-task — implement one scoped change end-to-end, recording its decisions as ADRs and user-facing effects on the changelog

Pick the workflow whose situation best fits the intent, then re-run with that
choice and the original intent:

jigc start --workflow <chosen> \"<intent>\"

That catalog is the selectable subset. A workflow outside it is reached by name
with the same `--workflow` form — `jigc describe --workflows` lists every
workflow, the ones the catalog leaves out included, and each of those carries
the reason it is hidden from the catalog.
— jigc · run `jigc start` for orientation; all writes through `jigc`.
";

/// The **no-structural-delta** `single-task` agent-text composition, byte for
/// byte — the pack-default include list `[locate, implement, superseded-context,
/// finalize]` composed with the cascade carrying *only* a `scalar:` flip (no
/// `deltas:` block, no project `steps/` dir). This is the determinism guard for
/// wiring phases 2+4 live into compose (M4 Increment 2, T4): once the live step
/// source consults `Resolved::file_owner` and `def.includes` flows through the
/// phase-4 structural-delta pass, the **no-override** read path must stay
/// byte-identical to the pre-increment baseline (`overrides.md` → Read-side
/// determinism invariant; Resolution algorithm). Captured from the binary before
/// the wiring landed.
///
/// The view opens with the frontend's `task minted:` header (M42) — this compose mints
/// `add-rate-limiter` (`workflow-dialect.md` → The `task minted:` header).
const NO_DELTA_SINGLE_TASK_GOLDEN: &str = "\
task minted: add-rate-limiter

Reason about the change. The intent is:
add rate limiter

The relevant code paths are not yet known. Inspect the codebase to confirm
scope before implementing.

Implement the change directly in the working tree. `git add` your code edits
before finalize — it commits only what you have staged.

If a decision is warranted, create an ADR and author its slots — a line per slot
usually suffices; an ADR earns its keep by capturing the *why*, not by running
long:

Run: `jigc doc create adr --title <TITLE> --task add-rate-limiter`

The `adr` schema is the authority on what you write into it — its required slots
and fields, each field's enum members, and every address a write can take:

jigc doc schema adr

Author its three required slots on the address `create` prints — `context` (the
forces at play), `decision` (the call itself), `consequences` (tradeoffs and
follow-on effects). Inside slot prose, the reserved heading depths are schema-relative to
the address you write — the CLI owns the section, item, and sub-label heading
levels there, so your headings sit below them; Setext headings are rejected at
every depth, and a rejected write names the shallowest depth free at that
address:

Every `--from-file -` below reads its payload from stdin — attach it as a heredoc
directly to the `jigc` command, never as a `cat payload | jigc …` pipeline, which
jigc itself accepts but an agent harness that statically analyses shell commands
can refuse to run:

jigc doc set-slot adr:<slug>#context --from-file - --task add-rate-limiter <<'EOF'
<the forces that made this decision necessary>
EOF

jigc doc set-slot adr:<slug>#context --from-file - --task add-rate-limiter
jigc doc set-slot adr:<slug>#decision --from-file - --task add-rate-limiter
jigc doc set-slot adr:<slug>#consequences --from-file - --task add-rate-limiter

The `options` slot is optional — fill it only when alternatives were genuinely
weighed. Its `## Options` heading renders either way; an empty optional slot is
conformant and never blocks finalize:

jigc doc set-slot adr:<slug>#options --from-file - --task add-rate-limiter

Before you finalize, verify the change actually works: build it and run the
tests, and confirm the behaviour you set out to produce. Finalize commits your
staged work; it does not check that the work is correct.

`<slug>` is the slug the title minted, and this task's own index names it back —
every doc the task stages, each by the `<type>:<slug>` identity the read takes:

jigc doc list adr --task add-rate-limiter

Read your write back before you move on — with `--task` the read serves THIS
task's staged copy, the write you just made, which the committed store does not
carry yet:

jigc doc show adr:<slug> --task add-rate-limiter

If the change is user-facing — a feature, a fix, or a behaviour a user would
notice — record it on the changelog. Create-or-update the singleton first:

Run: `jigc doc create changelog --title Changelog --task add-rate-limiter`

If a changelog is ALREADY committed, the CLI does not mint a fresh one — it copies
the committed body in as your edit base, so what you author below APPENDS to the
entries already there and any slot you set overwrites. Author only what THIS change
adds; an entry whose title mints an id the committed changelog already holds is
refused (`write.already-present`), the WHOLE payload rejected and nothing staged, so
edit that entry in place with `jigc doc set-slot` rather than re-authoring it here.

Then author the entry itself. The target schema and its batch payload, both
generated from the resolved `changelog` schema, follow — one call places the whole
entry over the staged buffer:

The `changelog` schema — the managed singleton at `CHANGELOG.md`.

- `meta` (front-matter fields):
    - `schema-version`: int — CLI-stamped (schema-version) unless authored
- `unreleased-changes`: repeatable items, one per `category`:
    - `category`: enum, one of: added | changed | deprecated | removed | fixed | security — the item's id-source (authored as the item's `title:` payload key)
    - `notes`: prose slot — One bullet per change in this category.
- `releases`: repeatable items, one per `title`:
    - `title`: string — the item's id-source (authored as the item's `title:` payload key)
    - `date`: date — CLI-stamped (on-create) unless authored
    - `link`: string — optional
    - `changes`: repeatable items, one per `category`:
        - `category`: enum, one of: added | changed | deprecated | removed | fixed | security — the item's id-source (authored as the item's `title:` payload key)
        - `notes`: prose slot — One bullet per change in this category.

Author the whole document in ONE `jigc doc author` batch payload — fill each `<…>` value. The `<<…>>` wrapping on slot prose is REQUIRED literal syntax: keep the `<<`/`>>` markers and replace only the text between them (an inline field takes a bare value — wrapping one is rejected). Inside slot prose, the reserved depths differ by slot — headings must sit at `####` or deeper in `unreleased-changes.notes`, at `#####` or deeper in `releases.changes.notes`. Setext headings are rejected. An entry marked `# optional` may be omitted entirely. A repeatable's demonstrated item entry is a template — repeat one `- title:` entry per item. Multi-line slot prose wraps WHOLE inside one `<<…>>` pair within its block scalar — a two-line slot is authored as

<slot-id>: |-
  <<The first line of the prose
  and its second line, inside the SAME pair.>>

Pipe the payload on stdin:

jigc doc author changelog --from-file - --task add-rate-limiter <<'EOF'
title: Changelog
sections:
  - id: unreleased-changes
    items:
      - title: \"<added | changed | deprecated | removed | fixed | security>\"
        set:
          notes: |-
            <<One bullet per change in this category.>>
  - id: releases
    items:
      - title: \"<the title>\"
        set:
          link: \"<the link>\" # optional — omit if unused
        sections:
          - id: changes
            items:
              - title: \"<added | changed | deprecated | removed | fixed | security>\"
                set:
                  notes: |-
                    <<One bullet per change in this category.>>
EOF

Author the entry under `unreleased-changes` — the staging area — and OMIT the
`releases` entry: work records a staged change, it does not cut a release (cutting
one is its own deliberate pass, `jigc start --workflow record-change`). The item
`title` is the CATEGORY, one of the enum members the schema above lists, and the
category is the group's id — so several bullets in the same category are ONE item
with all its bullets merged into that item's `notes`.

Read your write back before you move on — with `--task` the read serves THIS
task's staged copy, the write you just made, which the committed store does not
carry yet:

jigc doc show changelog:changelog --task add-rate-limiter

If your decision supersedes an earlier one, set `supersedes` on the ADR; the
superseded decision then appears below for reference, so your consequences can
explain what changes (nothing appears if it supersedes none).

When done, set the required Conventional-Commits type — your editorial call on
what this change does. The subject renders as `<type>(<scope>): <summary>`, so
write the summary without a type or scope prefix of its own — the `type` field
already carries it. Inside slot prose, the reserved heading depths are schema-relative to
the address you write — the CLI owns the section, item, and sub-label heading
levels there, so your headings sit below them; Setext headings are rejected at
every depth, and a rejected write names the shallowest depth free at that
address.

Every `--from-file -` below reads its payload from stdin — attach it as a heredoc
directly to the `jigc` command, never as a `cat payload | jigc …` pipeline, which
jigc itself accepts but an agent harness that statically analyses shell commands
can refuse to run:

jigc doc set-slot commit:add-rate-limiter#summary --from-file - --task add-rate-limiter <<'EOF'
<one line saying what changed>
EOF

Set the type, then stage the summary prose:

Run: `jigc doc set-field commit:add-rate-limiter#type --value <COMMIT_TYPE> --task add-rate-limiter`
The `type` value is one of: feat | fix | docs | style | refactor | perf | test | build | ci | chore | revert
Run: `jigc doc set-slot commit:add-rate-limiter#summary --from-file - --task add-rate-limiter`
<<author: commit:add-rate-limiter#summary>>

The `commit` schema is the authority on what you write into it — its required
slots and fields, each field's enum members, and every address a write can take:

jigc doc schema commit

The `scope` and `body` are optional: add a `scope` to name the area touched, or
author a `body` to explain the motivation, only when they earn their place —

jigc doc set-field commit:add-rate-limiter#scope --value <area> --task add-rate-limiter
jigc doc set-slot commit:add-rate-limiter#body --from-file - --task add-rate-limiter

When the work shares authorship — a co-author, or an agent that wrote it — record
it in a commit trailer. Add one trailer item, then set its value on the address
`add-item` prints:

jigc doc add-item commit:add-rate-limiter#trailers --title Co-Authored-By --task add-rate-limiter
jigc doc set-field commit:add-rate-limiter#trailers/<id>/value --value \"Name <email>\" --task add-rate-limiter

Read your write back before you move on — with `--task` the read serves THIS
task's staged copy, the write you just made, which the committed store does not
carry yet:

jigc doc show commit:add-rate-limiter --task add-rate-limiter

Validate and commit the task as one logical commit. Finalize commits only the
staged set plus the docs it manages; unstaged edits and untracked files are left
out, and with nothing staged over a dirty tree it refuses. Anything still staged
from BEFORE this task was minted makes finalize refuse too (one blocking finding
per carried path): unstage it, or pass `--carry-staged` to declare the carryover
deliberate.

To see what's left before committing, run `jigc task validate add-rate-limiter` — it
previews part of what finalize gates on (the repository posture finalize refuses
under, this task's content findings, the carryover gate, the owner-artifact causes
that need no staging, and the granted-but-unused changelog gate), without
committing anything; the staged set, promotion and the commit itself are decided
at finalize.

Run: `jigc task finalize add-rate-limiter`
resume: `jigc start --task add-rate-limiter`   — re-composes this workflow if context is lost
what's-left: `jigc task validate add-rate-limiter`   — previews part of the finalize gate: the repository posture finalize refuses under, this task's content findings, the carryover gate, the owner-artifact causes that need no staging, and the granted-but-unused changelog gate; the staged set, promotion and the commit surface at finalize
task scope: `jigc doc` writes default to the single active task; `--task add-rate-limiter` is the explicit override and wins when several are active — several open tasks are legal, each addressed by its own `--task`, so you can run them in parallel while their work stays disjoint; once a sibling task commits a path this one also touches, resuming or finalizing here blocks and names the overlapping paths
create-gates: adr, changelog   — the doc-types this task is allowed to create; any other type is refused
— jigc · run `jigc start` for orientation; all writes through `jigc`.
";

/// Initialize a real git repo with one commit (composition mints, which reads
/// HEAD via `git rev-parse`), and create the `.jigc/config/` project layer so the
/// cascade resolves.
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
    git(&["add", "."]);
    git(&["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(root.join(".jigc").join("config")).expect("create project layer");
}

/// Run `jigc start <args>` with `cwd = repo` and `$HOME = home`.
fn run_start(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command.arg("start");
    command.args(args);
    command
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

/// Run `jigc <args>` with `cwd = repo` and `$HOME = home` — the whole verb surface
/// ([`run_start`] generalized), for the parallel-task arms that drive `doc` + `task`
/// alongside `start`.
fn run_jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command.args(args);
    command
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

/// Assert a `jigc` invocation succeeded, surfacing both streams on failure.
fn assert_ok(out: &std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// Run a `git` command in `repo`, asserting success, returning trimmed stdout.
fn git_in(repo: &Path, args: &[&str]) -> String {
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
    String::from_utf8(out.stdout)
        .expect("utf-8")
        .trim()
        .to_string()
}

/// Fill `task`'s provisioned commit doc to a finalize-clean state, every write addressed
/// by its **own** `--task` — the surface the composed `task scope:` line names, and with
/// two tasks open the only form the write door accepts. The summary rides a file under
/// `home` (never the repo), so no probe path is dirtied by the fixture itself.
fn fill_commit(repo: &Path, home: &Path, task: &str, summary: &str) {
    assert_ok(
        &run_jigc(
            repo,
            home,
            &[
                "doc",
                "set-field",
                &format!("commit:{task}#type"),
                "--value",
                "fix",
                "--task",
                task,
            ],
        ),
        &format!("set-field commit:{task}#type"),
    );
    let summary_file = home.join(format!("{task}-summary.txt"));
    fs::write(&summary_file, format!("{summary}\n")).expect("write the summary source");
    assert_ok(
        &run_jigc(
            repo,
            home,
            &[
                "doc",
                "set-slot",
                &format!("commit:{task}#summary"),
                "--from-file",
                summary_file.to_str().expect("utf-8 path"),
                "--task",
                task,
            ],
        ),
        &format!("set-slot commit:{task}#summary"),
    );
}

/// Run `jigc config <args>` with `cwd = repo` and `$HOME = home`.
fn run_config(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command.arg("config");
    command.args(args);
    command
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

#[test]
fn listed_pack_resource_resolves_through_the_cwd_discovered_pack_set() {
    // T3 — the load-bearing proof of the make_pack() elaboration pin: make_pack()
    // stays zero-arg and CWD-discovers the project config, so the discovered
    // pack-set equals the in-repo project dir's `.jigc/config/packs.yaml` list. A
    // listed second filesystem pack shipping an extra `creates-task: true`
    // selectable workflow must surface in the router catalog when `jigc start` runs
    // with cwd inside the repo — proving the listed pack's resource resolved through
    // the discovered set (the CLAIM the pin must prove, never assert).
    //
    // The base (embedded) pack is unchanged, so this *also* witnesses the union
    // read: the listed pack's `listed-extra` and the base pack's `single-task` both
    // list. The listed workflow is body-trivial (router only lists, never composes a
    // body), so this stays inside increment 1's scope (no cross-pack body-reference
    // resolution yet).
    let repo = TempDir::new("listed-pack-set");
    init_repo(repo.path());
    let home = TempDir::new("home");

    // Seed a second filesystem pack OUTSIDE the repo (a literal pack directory the
    // `packs:` list names), shipping one extra selectable work-workflow.
    let listed = TempDir::new("listed-pack");
    let listed_workflows = listed.path().join("workflows");
    fs::create_dir_all(&listed_workflows).expect("mk listed workflows/");
    fs::write(
        listed_workflows.join("listed-extra.yaml"),
        "---\n\
         when: a workflow shipped only by the listed pack\n\
         description: A listed-pack-only selectable work-workflow.\n\
         usage: proving the CWD-discovered pack-set includes the listed pack.\n\
         creates-task: true\n\
         ---\n\
         {{ include: step:locate }}\n",
    )
    .expect("seed listed-extra workflow");

    // Record the listed pack in the in-repo project layer's packs.yaml — the
    // pre-cascade selector make_pack() CWD-discovers.
    let config = repo.path().join(".jigc").join("config");
    fs::write(
        config.join("packs.yaml"),
        format!("packs:\n  - {}\n", listed.path().display()),
    )
    .expect("write packs.yaml naming the listed pack");

    // Bare `jigc start "<intent>"` composes the router (cascade default), which
    // lists the UNION of selectable work-workflows from the composed pack-set.
    let out = run_start(repo.path(), home.path(), &["add rate limiter"]);
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    assert!(
        out.status.success(),
        "bare `jigc start` over a two-pack set must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );

    // The listed pack's resource resolved through the CWD-discovered set — its
    // selectable workflow appears in the router catalog.
    assert!(
        stdout.contains("- listed-extra — a workflow shipped only by the listed pack"),
        "the listed pack's workflow must surface in the router catalog (proving CWD-discovery \
         loaded the in-repo packs.yaml set); got:\n{stdout}",
    );
    // The base (embedded) pack's `single-task` still lists — the union, base last.
    assert!(
        stdout.contains("- single-task — implement one scoped change end-to-end, recording its decisions as ADRs and user-facing effects on the changelog"),
        "the base pack's single-task must still list alongside the listed pack's workflow \
         (the union read); got:\n{stdout}",
    );
}

#[test]
fn bare_intent_composes_the_router_without_minting() {
    // Post-flip (`DECISIONS.md` 2026-06-01 → M2 flips `default-workflow` to
    // `router`): a bare `jigc start "<intent>"` composes the cascade default — now
    // the `creates-task: false` router — so it lists the selectable work-workflows
    // and mints NOTHING. Minting happens only via Form D (`--workflow <X>`).
    let repo = TempDir::new("compose");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let out = run_start(repo.path(), home.path(), &["add rate limiter"]);
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    assert!(
        out.status.success(),
        "`jigc start \"<intent>\"` must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );

    // The router is `creates-task: false`: no `.jigc/tasks/` working area appears.
    assert!(
        !repo.path().join(".jigc").join("tasks").exists(),
        "bare `jigc start` now composes the router — no .jigc/tasks/ dir may be minted",
    );

    // The router lists the selectable (`creates-task: true`) work-workflows, each as
    // a `- <id> — <when>` option line, and carries the agent-substitution re-run.
    assert!(
        stdout.contains("- single-task — implement one scoped change end-to-end, recording its decisions as ADRs and user-facing effects on the changelog"),
        "the router must list single-task as a `- <id> — <when>` option; got:\n{stdout}",
    );
    assert!(
        stdout.contains("- quick-fix — apply a small commit-only fix that touches no documented code (code a managed doc names) and records no decision"),
        "the router must list quick-fix as a `- <id> — <when>` option; got:\n{stdout}",
    );
    assert!(
        stdout.contains("jigc start --workflow <chosen> \"<intent>\""),
        "the router must carry the literal agent-substitution re-run prose; got:\n{stdout}",
    );
    assert!(
        stdout.trim_end().ends_with(ROUTING_FOOTER),
        "agent-text composition must end with the routing footer; got:\n{stdout}",
    );
}

#[test]
fn no_override_bare_intent_compose_is_byte_identical_to_the_golden() {
    // The determinism gate for T5 (M4 Increment 1): wiring the resolved cascade
    // live into compose must leave the **no-override** read path byte-identical.
    // `init_repo` creates `.jigc/config/` but writes no `manifest.yaml`, so the
    // project layer carries no `scalar:` delta — the cascade resolves the pack
    // default (`router`) and the composed bytes must equal the golden exactly.
    let repo = TempDir::new("byte-identical");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let out = run_start(repo.path(), home.path(), &["add rate limiter"]);
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    assert!(
        out.status.success(),
        "the no-override bare compose must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    assert_eq!(
        stdout, NO_OVERRIDE_ROUTER_GOLDEN,
        "the no-override compose output must stay byte-identical to today's golden",
    );
}

#[test]
fn project_scalar_set_flips_the_bare_intent_workflow() {
    // The cascade *applies* a delta: a project-layer `scalar: default-workflow:
    // single-task` resolved over the pack default (`router`) flips which workflow
    // a bare `jigc start "<intent>"` composes — from the no-mint router to the
    // `creates-task: true` `single-task`, which mints under `.jigc/tasks/<slug>/`.
    let repo = TempDir::new("flip");
    init_repo(repo.path());
    let home = TempDir::new("home");

    fs::write(
        repo.path()
            .join(".jigc")
            .join("config")
            .join("manifest.yaml"),
        "scalar:\n  default-workflow: single-task\n",
    )
    .expect("write project manifest");

    let out = run_start(repo.path(), home.path(), &["add rate limiter"]);
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    assert!(
        out.status.success(),
        "the flipped bare compose must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );

    // `single-task` is `creates-task: true`: bare `jigc start` now mints.
    let task_dir = repo
        .path()
        .join(".jigc")
        .join("tasks")
        .join("add-rate-limiter");
    assert!(
        task_dir.join("base.json").is_file(),
        "the project scalar-set must flip the default to single-task, which mints under .jigc/tasks/<slug>/; got:\n{stdout}",
    );
    // The composed view is the single-task body (embedding the resolved intent),
    // not the router's selectable-workflow catalog.
    assert!(
        stdout.contains("add rate limiter"),
        "the flipped compose must embed the resolved intent (single-task body); got:\n{stdout}",
    );
    assert!(
        !stdout.contains("These are the selectable work-workflows"),
        "the flipped compose must not be the router catalog; got:\n{stdout}",
    );
}

#[test]
fn no_structural_delta_single_task_compose_is_byte_identical_to_the_golden() {
    // The T4 determinism guard (M4 Increment 2): wiring the layer-aware step source
    // (phase 2) + the phase-4 structural-delta pass live into compose must leave the
    // **no-override** read path byte-identical. The manifest carries only a `scalar:`
    // flip to single-task — no `deltas:` block, no `.jigc/config/steps/` dir — so
    // the cascade resolves the pack include list `[locate, implement,
    // superseded-context, finalize]` and every step reads its pack body, unchanged.
    let repo = TempDir::new("no-delta-byte-identical");
    init_repo(repo.path());
    let home = TempDir::new("home");

    fs::write(
        repo.path()
            .join(".jigc")
            .join("config")
            .join("manifest.yaml"),
        "scalar:\n  default-workflow: single-task\n",
    )
    .expect("write project manifest");

    let out = run_start(repo.path(), home.path(), &["add rate limiter"]);
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    assert!(
        out.status.success(),
        "the no-delta single-task compose must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    assert_eq!(
        stdout, NO_DELTA_SINGLE_TASK_GOLDEN,
        "the no-structural-delta compose must stay byte-identical to the pre-increment baseline",
    );
}

#[test]
fn single_task_compose_enumerates_all_four_adr_slot_commands() {
    // The M40 A4 done-criterion (doctype-authoring.md → Authoring surface: adr's
    // inline step was the named anti-pattern): the composed `single-task` view must
    // enumerate ALL FOUR adr slots — the three required (`context`, `decision`,
    // `consequences`) plus the optional `options` — each with its `set-slot`
    // command, task-id resolved. Asserted on the emitted bytes through the binary:
    // the create-adr mint line first, then the slot commands, before the
    // finalize-verification prose.
    let repo = TempDir::new("adr-slot-enumeration");
    init_repo(repo.path());
    let home = TempDir::new("home");

    fs::write(
        repo.path()
            .join(".jigc")
            .join("config")
            .join("manifest.yaml"),
        "scalar:\n  default-workflow: single-task\n",
    )
    .expect("write project manifest");

    let out = run_start(repo.path(), home.path(), &["add rate limiter"]);
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    assert!(
        out.status.success(),
        "the compose must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );

    let create_at = stdout
        .find("Run: `jigc doc create adr --title <TITLE> --task add-rate-limiter`")
        .expect("the create-adr mint line composes");
    let verify_at = stdout
        .find("Before you finalize")
        .expect("the finalize-verification prose composes");
    for slot in ["context", "decision", "consequences", "options"] {
        let line =
            format!("jigc doc set-slot adr:<slug>#{slot} --from-file - --task add-rate-limiter");
        let at = stdout.find(&line).unwrap_or_else(|| {
            panic!("the composed view must carry the `{slot}` set-slot command; got:\n{stdout}")
        });
        assert!(
            create_at < at && at < verify_at,
            "the `{slot}` set-slot command must sit between the create-adr mint and the verification prose; got:\n{stdout}",
        );
    }
}

#[test]
fn slot_fill_content_composes_into_the_implement_body() {
    // The T4 done-criterion (worked-examples.md → 3c): a hand-authored project
    // manifest with a `slot-fill step:implement#extra-guidance` delta + a native
    // `.jigc/config/fills/extra-guidance.md` makes `jigc start "<intent>"` compose
    // `single-task` with the fill content spliced into `implement`'s emitted body —
    // proven on the emitted bytes through the binary (phase 5 wired live).
    //
    // The `{{fill: extra-guidance}}` point lands in the pack `implement` step at T6;
    // until then this test declares it via a native project `steps/implement.yaml`
    // shadow (the phase-2 mechanism), so the slot-fill targets a point the resolved
    // body declares — exercising the full apply path without pre-empting T6's pack
    // change.
    let repo = TempDir::new("slot-fill");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let config = repo.path().join(".jigc").join("config");
    // Flip to single-task *and* record the slot-fill delta in the one manifest.
    fs::write(
        config.join("manifest.yaml"),
        "scalar:\n  default-workflow: single-task\n\
         deltas:\n\
         \x20 - kind: slot-fill\n\
         \x20   target: step:implement#extra-guidance\n\
         \x20   content: fills/extra-guidance.md\n",
    )
    .expect("write project manifest with a slot-fill delta");
    // The project step shadow of `implement` declaring the `{{fill:}}` extension
    // point the slot-fill targets (T6 moves this point into the pack step).
    fs::create_dir_all(config.join("steps")).expect("mk steps/");
    fs::write(
        config.join("steps").join("implement.yaml"),
        "Implement the change directly in the working tree.\n\n\
         {{ fill: extra-guidance }}\n",
    )
    .expect("write the native implement shadow declaring the fill point");
    // The native fill content (basename = fill-id) the phase-5 pass splices in.
    fs::create_dir_all(config.join("fills")).expect("mk fills/");
    fs::write(
        config.join("fills").join("extra-guidance.md"),
        "Before you finalize, run the project lint probe and fix any findings.\n",
    )
    .expect("write the native fill content");

    let out = run_start(repo.path(), home.path(), &["add rate limiter"]);
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    assert!(
        out.status.success(),
        "the slot-fill compose must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );

    // The fill content composed into `implement`'s emitted body, in place of the
    // `{{fill: extra-guidance}}` point — proven on the emitted bytes.
    assert!(
        stdout.contains("Before you finalize, run the project lint probe and fix any findings."),
        "the slot-fill content must compose into the implement body; got:\n{stdout}",
    );
    // The `{{fill:}}` point itself is gone — phase 5 replaced it, never emitted raw.
    assert!(
        !stdout.contains("{{ fill: extra-guidance }}") && !stdout.contains("fill: extra-guidance"),
        "the raw `{{fill:}}` point must not survive into the composed view; got:\n{stdout}",
    );
}

#[test]
fn slot_fill_for_an_omitted_step_does_not_brick_the_router_front_door() {
    // Regression (M4 audit, HIGH): once a project records *any* slot-fill on the
    // shipped `step:implement#extra-guidance` point, bare `jigc start` (the cascade-
    // default `router`, which includes `present-catalog`/`route-to-workflow` — *not*
    // `implement`) must still compose cleanly. The orphan check keys on the *target
    // step's* resolved body, so a slot-fill whose target step the composed workflow
    // omits is **inert, not an orphan** — the primary front door stays open.
    //
    // No project `steps/` shadow: the pack `implement` step genuinely ships
    // `{{fill: extra-guidance}}`, so the slot-fill targets a real declared point. The
    // earlier `scenario_7` slot-fill coverage only ever composed `--workflow
    // single-task` (which *does* include `implement`), which is exactly why it missed
    // this false block.
    let repo = TempDir::new("router-front-door");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let config = repo.path().join(".jigc").join("config");
    // Record the slot-fill but leave `default-workflow` at the pack default (`router`),
    // so bare `jigc start "<intent>"` composes the router that omits `implement`.
    fs::write(
        config.join("manifest.yaml"),
        "deltas:\n\
         \x20 - kind: slot-fill\n\
         \x20   target: step:implement#extra-guidance\n\
         \x20   content: fills/extra-guidance.md\n",
    )
    .expect("write project manifest with an implement slot-fill");
    fs::create_dir_all(config.join("fills")).expect("mk fills/");
    fs::write(
        config.join("fills").join("extra-guidance.md"),
        "Before you finalize, run the project lint probe and fix any findings.\n",
    )
    .expect("write the native fill content");

    let out = run_start(repo.path(), home.path(), &["add rate limiter"]);
    assert!(
        out.status.success(),
        "bare `jigc start` (the router) must compose cleanly even with an `implement` \
         slot-fill recorded — the target step is omitted, so the fill is inert, not an \
         orphan; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    // It really composed the router (no `implement`), not single-task.
    assert!(
        !stdout.contains("{{ fill: extra-guidance }}") && !stdout.contains("fill: extra-guidance"),
        "no raw `{{fill:}}` point may leak into the router view; got:\n{stdout}",
    );
}

#[test]
fn orphaned_slot_fill_blocks_compose_with_its_route() {
    // The T4 wiring also runs the M4 `workflow-refs` fill checks live in the gate: a
    // `slot-fill` targeting a `<fill-id>` no resolved step body declares is a blocking
    // `slot-fill-orphan` finding, surfaced non-zero with its repair route — proven
    // through the binary. The target `nonesuch` is undeclared by any resolved body
    // (the pack `implement` step declares only `extra-guidance` as of T6), so it is
    // orphaned.
    let repo = TempDir::new("slot-fill-orphan");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let config = repo.path().join(".jigc").join("config");
    fs::write(
        config.join("manifest.yaml"),
        "scalar:\n  default-workflow: single-task\n\
         deltas:\n\
         \x20 - kind: slot-fill\n\
         \x20   target: step:implement#nonesuch\n\
         \x20   content: fills/nonesuch.md\n",
    )
    .expect("write project manifest with an orphaned slot-fill delta");
    fs::create_dir_all(config.join("fills")).expect("mk fills/");
    fs::write(config.join("fills").join("nonesuch.md"), "Some guidance.\n")
        .expect("write the native fill content");

    let out = run_start(repo.path(), home.path(), &["add rate limiter"]);
    let stderr = String::from_utf8(out.stderr).expect("utf-8 stderr");

    assert!(
        !out.status.success(),
        "an orphaned slot-fill must exit non-zero; got {:?}",
        out.status,
    );
    assert!(
        stderr.contains("nonesuch") && stderr.contains("route:"),
        "the orphan block must name the fill point and carry a route; got:\n{stderr}",
    );
}

#[test]
fn replace_step_delta_flips_the_composed_include_list() {
    // The T4 done-criterion (worked-examples.md → 3a): a hand-authored project
    // manifest with a `replace-step workflow:single-task#implement → step:project-
    // implement` delta, plus a native `.jigc/config/steps/project-implement.yaml`
    // re-including the pack `step:implement` and appending a house rule, flips the
    // composed include list to `[locate, project-implement(→implement + house rule),
    // superseded-context, finalize]` — proven on the emitted bytes through the binary.
    let repo = TempDir::new("replace-step");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let config = repo.path().join(".jigc").join("config");
    // Flip to single-task *and* record the structural delta in the one manifest.
    fs::write(
        config.join("manifest.yaml"),
        "scalar:\n  default-workflow: single-task\n\
         deltas:\n\
         \x20 - kind: replace-step\n\
         \x20   target: workflow:single-task#implement\n\
         \x20   with: step:project-implement\n",
    )
    .expect("write project manifest with a replace-step delta");
    // The native step file (id = filename basename) re-includes the pack step and
    // adds the house rule — flow 3a's augment-without-fork shape.
    fs::create_dir_all(config.join("steps")).expect("mk steps/");
    fs::write(
        config.join("steps").join("project-implement.yaml"),
        "{{ include: step:implement }}\n\n\
         Before you finalize, run the project lint probe and fix any findings.\n",
    )
    .expect("write the native project-implement step");

    let out = run_start(repo.path(), home.path(), &["add rate limiter"]);
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    assert!(
        out.status.success(),
        "the replace-step flipped compose must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );

    // The pack `implement` body still composes (the re-include pulled it in) ...
    assert!(
        stdout.contains("Implement the change directly in the working tree."),
        "the re-included pack `implement` body must still compose; got:\n{stdout}",
    );
    // ... immediately followed by the project house rule the native step appends —
    // proof the project step shadowed the include-list position and expanded.
    assert!(
        stdout.contains("Before you finalize, run the project lint probe and fix any findings."),
        "the project-implement house rule must compose after the re-included pack body; got:\n{stdout}",
    );
    // The surrounding pack steps are untouched — locate before, finalize after.
    let locate_at = stdout
        .find("Reason about the change.")
        .expect("locate step composes");
    let house_at = stdout
        .find("Before you finalize, run the project lint probe")
        .expect("house rule composes");
    let finalize_at = stdout
        .find("Validate and commit the task as one logical commit.")
        .expect("finalize step composes");
    assert!(
        locate_at < house_at && house_at < finalize_at,
        "the include list must be [locate, project-implement(→implement + house rule), superseded-context, finalize]; got:\n{stdout}",
    );
}

#[test]
fn config_set_writes_the_scalar_block_and_compose_reads_it() {
    // The T6 done-criterion: `jigc config set default-workflow single-task` writes
    // the project manifest's `scalar:` block (check_value-adjudicated), and a
    // subsequent bare `jigc start "<intent>"` resolves the cascade through that
    // delta — flipping the no-mint router to the `creates-task: true` single-task,
    // which mints. The write path and the read path agree on the one manifest.
    let repo = TempDir::new("config-set");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let set = run_config(
        repo.path(),
        home.path(),
        &["set", "default-workflow", "single-task"],
    );
    assert!(
        set.status.success(),
        "`jigc config set default-workflow single-task` must exit 0; got {:?}\nstderr:\n{}",
        set.status,
        String::from_utf8_lossy(&set.stderr),
    );

    // The write landed in the project manifest's `scalar:` block.
    let manifest = fs::read_to_string(
        repo.path()
            .join(".jigc")
            .join("config")
            .join("manifest.yaml"),
    )
    .expect("manifest written");
    assert!(
        manifest.contains("default-workflow: single-task"),
        "the manifest `scalar:` block must record the set value; got:\n{manifest}",
    );

    // Compose reads the just-written delta: bare `jigc start` now mints single-task.
    let out = run_start(repo.path(), home.path(), &["add rate limiter"]);
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    assert!(
        out.status.success(),
        "compose after `config set` must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        repo.path()
            .join(".jigc")
            .join("tasks")
            .join("add-rate-limiter")
            .join("base.json")
            .is_file(),
        "after `config set default-workflow single-task` the bare compose must mint single-task; got:\n{stdout}",
    );
}

#[test]
fn config_set_rejects_an_undeclared_key() {
    // Write-time closed-surface adjudication: a key the pack does not declare is
    // rejected non-zero with the routed finding — never silently recorded.
    let repo = TempDir::new("config-undeclared");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let set = run_config(repo.path(), home.path(), &["set", "not-a-knob", "anything"]);
    let stderr = String::from_utf8(set.stderr).expect("utf-8 stderr");
    assert!(
        !set.status.success(),
        "an undeclared key must exit non-zero; got {:?}",
        set.status,
    );
    assert!(
        stderr.contains("not-a-knob") && stderr.contains("route:"),
        "the rejection must name the undeclared key and carry a route; got:\n{stderr}",
    );
    // The rejection precedes any write — no manifest is created.
    assert!(
        !repo
            .path()
            .join(".jigc")
            .join("config")
            .join("manifest.yaml")
            .exists(),
        "an undeclared-key rejection must write no manifest",
    );
}

#[test]
fn config_set_rejects_a_wrong_type_value() {
    // Write-time `check_value` adjudication: a value outside the knob's declared
    // enum is rejected non-zero with the routed finding (the same adjudication the
    // doc write path uses — no second type system).
    let repo = TempDir::new("config-wrong-type");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let set = run_config(
        repo.path(),
        home.path(),
        &["set", "default-workflow", "not-a-workflow"],
    );
    let stderr = String::from_utf8(set.stderr).expect("utf-8 stderr");
    assert!(
        !set.status.success(),
        "a wrong-type value must exit non-zero; got {:?}",
        set.status,
    );
    assert!(
        stderr.contains("not-a-workflow") && stderr.contains("route:"),
        "the rejection must name the rejected value and carry a route; got:\n{stderr}",
    );
    assert!(
        !repo
            .path()
            .join(".jigc")
            .join("config")
            .join("manifest.yaml")
            .exists(),
        "a wrong-type rejection must write no manifest",
    );
}

#[test]
fn bare_intent_router_json_format_carries_no_footer() {
    let repo = TempDir::new("compose-json");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let out = run_start(
        repo.path(),
        home.path(),
        &["--format", "json", "add rate limiter"],
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    assert!(
        out.status.success(),
        "`--format json` composition must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        !stdout.contains(ROUTING_FOOTER),
        "JSON composition output must carry no routing footer; got:\n{stdout}",
    );
    let value: serde_json::Value = serde_json::from_str(&stdout)
        .unwrap_or_else(|e| panic!("must be valid JSON ({e}); got:\n{stdout}"));
    assert!(
        value["text"]
            .as_str()
            .is_some_and(|t| t.contains("- single-task —")),
        "the JSON view's `text` must carry the composed router catalog; got:\n{stdout}",
    );
    // The command-output contract (`command-output-contract.md` §1): the composed
    // JSON always carries a `task` key; on the `creates-task: false` router arm it is
    // `null` (no task minted). Present-and-null, not merely absent.
    let obj = value.as_object().expect("the composed JSON is an object");
    assert!(
        obj.contains_key("task"),
        "the composed JSON must carry a `task` key; got:\n{stdout}",
    );
    assert!(
        value["task"].is_null(),
        "the router (creates-task: false) arm mints no task, so `task` must be null; got:\n{stdout}",
    );
}

#[test]
fn form_d_single_task_json_format_carries_the_minted_task_id() {
    let repo = TempDir::new("compose-json-task");
    init_repo(repo.path());
    let home = TempDir::new("home");

    // Form D over the `creates-task: true` `single-task` mints `add-rate-limiter`
    // (the slug of "add rate limiter"); the composed JSON must surface that minted id
    // structurally in `task`, retiring the prose scrape (`command-output-contract.md`
    // §1).
    let out = run_start(
        repo.path(),
        home.path(),
        &[
            "--format",
            "json",
            "--workflow",
            "single-task",
            "add rate limiter",
        ],
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    assert!(
        out.status.success(),
        "`--format json --workflow single-task \"<intent>\"` must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    let value: serde_json::Value = serde_json::from_str(&stdout)
        .unwrap_or_else(|e| panic!("must be valid JSON ({e}); got:\n{stdout}"));
    assert_eq!(
        value["task"].as_str(),
        Some("add-rate-limiter"),
        "a work-minting compose must carry the minted task id in `task`; got:\n{stdout}",
    );
}

#[test]
fn form_d_named_workflow_mints_and_composes_through_dispatch() {
    let repo = TempDir::new("form-d");
    init_repo(repo.path());
    let home = TempDir::new("home");

    // `--workflow single-task <intent>`: the explicit-selection front door. The
    // embedded pack's `single-task` is `creates-task: true`, so Form D mints and
    // composes it end-to-end through the dispatch arm.
    let out = run_start(
        repo.path(),
        home.path(),
        &["--workflow", "single-task", "add rate limiter"],
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    assert!(
        out.status.success(),
        "`jigc start --workflow single-task \"<intent>\"` must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );

    // The named creates-task workflow minted under .jigc/tasks/<slug>/.
    let task_dir = repo
        .path()
        .join(".jigc")
        .join("tasks")
        .join("add-rate-limiter");
    assert!(
        task_dir.join("base.json").is_file(),
        "Form D over a creates-task workflow must open .jigc/tasks/add-rate-limiter/ with a base pin",
    );

    // The named workflow composed end-to-end, embedding the resolved intent and
    // ending with the routing footer (the same composed view `--format` renders).
    assert!(
        stdout.contains("add rate limiter"),
        "the composed view must embed the resolved intent; got:\n{stdout}",
    );
    assert!(
        stdout.trim_end().ends_with(ROUTING_FOOTER),
        "Form-D agent-text composition must end with the routing footer; got:\n{stdout}",
    );
}

#[test]
fn form_d_plan_mints_on_workflow_plan_and_emits_the_create_spec_gate() {
    let repo = TempDir::new("form-d-plan");
    init_repo(repo.path());
    let home = TempDir::new("home");

    // `--workflow plan <intent>`: the spec-authoring work-workflow (Increment 3).
    // It is `creates-task: true` with `allows-create: [{type: spec, as: spec}]`,
    // so Form D mints + composes it end-to-end; its `author-spec` step carries the
    // create-gate, which the composer emits as a `Run: jigc doc create spec` line.
    let out = run_start(
        repo.path(),
        home.path(),
        &["--workflow", "plan", "draft the rate-limiter spec"],
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    assert!(
        out.status.success(),
        "`jigc start --workflow plan \"<intent>\"` must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );

    // The work-workflow minted under .jigc/tasks/<slug>/, recording `workflow: plan`
    // (resume composes the task's own minting workflow — `state::read_workflow_id`).
    let task_dir = repo
        .path()
        .join(".jigc")
        .join("tasks")
        .join("draft-the-rate-limiter-spec");
    assert!(
        task_dir.join("base.json").is_file(),
        "plan is creates-task: true, so it must open .jigc/tasks/draft-the-rate-limiter-spec/ with a base pin",
    );
    let recorded =
        fs::read_to_string(task_dir.join("workflow")).expect("read recorded workflow id");
    assert_eq!(
        recorded.trim(),
        "plan",
        "the minted task must record `workflow: plan`",
    );

    // The composed view embeds the resolved `{{task.intent}}` ...
    assert!(
        stdout.contains("draft the rate-limiter spec"),
        "the composed view must embed the resolved intent; got:\n{stdout}",
    );
    // ... and carries the create-gate as a machine-extractable `Run:` line.
    assert!(
        stdout.contains("Run: `jigc doc create spec"),
        "the plan workflow's author-spec step must emit the `jigc doc create spec` create-gate; got:\n{stdout}",
    );
    // T3 (M36): author-spec now teaches per-criterion `add-item`, the non-goals
    // nudge, and the named-files nudge — proven on the emitted bytes.
    assert!(
        stdout.contains("jigc doc add-item spec:<slug>#criteria"),
        "author-spec must teach the per-criterion `add-item`; got:\n{stdout}",
    );
    assert!(
        stdout.contains("non-goals"),
        "author-spec must teach the non-goals nudge; got:\n{stdout}",
    );
    assert!(
        stdout.contains("concrete\nfiles or modules") || stdout.contains("files or modules"),
        "author-spec must teach the named-files nudge; got:\n{stdout}",
    );
}

#[test]
fn form_d_project_setup_mints_and_emits_the_create_prd_gate_with_two_author_slots_and_requirement_items()
 {
    let repo = TempDir::new("form-d-project-setup");
    init_repo(repo.path());
    let home = TempDir::new("home");

    // `--workflow project-setup <idea>`: the M9 new-project on-ramp work-workflow.
    // It is `creates-task: true` with `allows-create: [{type: prd, as: brief}]`, so
    // Form D mints + composes it end-to-end. Its `author-prd` step carries the
    // create-gate, which the composer emits as a `Run: jigc doc create prd` line,
    // followed by the two `<<author: …>>` slot lines for prd's fixed prose slots
    // (vision / context) plus the per-requirement `add-item`/`set-slot` guidance for
    // the repeatable `requirements` section (M25 Inc 5). `{{task.intent}}`
    // interpolates the idea.
    let out = run_start(
        repo.path(),
        home.path(),
        &["--workflow", "project-setup", "build a recipe sharing app"],
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    assert!(
        out.status.success(),
        "`jigc start --workflow project-setup \"<idea>\"` must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );

    // The work-workflow minted under .jigc/tasks/<slug>/, recording its own id.
    let task_dir = repo
        .path()
        .join(".jigc")
        .join("tasks")
        .join("build-a-recipe-sharing-app");
    assert!(
        task_dir.join("base.json").is_file(),
        "project-setup is creates-task: true, so it must open .jigc/tasks/build-a-recipe-sharing-app/ with a base pin",
    );
    let recorded =
        fs::read_to_string(task_dir.join("workflow")).expect("read recorded workflow id");
    assert_eq!(
        recorded.trim(),
        "project-setup",
        "the minted task must record `workflow: project-setup`",
    );

    // `{{task.intent}}` interpolated the idea into the develop-idea body.
    assert!(
        stdout.contains("build a recipe sharing app"),
        "the composed view must embed the resolved `{{task.intent}}`; got:\n{stdout}",
    );

    // The create-gate emits as a machine-extractable `Run:` line for `prd` ...
    assert!(
        stdout.contains("Run: `jigc doc create prd"),
        "the author-prd step must emit the `jigc doc create prd` create-gate; got:\n{stdout}",
    );

    // ... and the two fixed prose slots emit as exactly two `<<author: …>>` lines
    // (vision/context; the prd is not created yet, so each resolves empty —
    // `<<author: >>`). `requirements` is now a repeatable section, NOT a slot, so it
    // does not emit an author line. The `commit` directive is excluded: since M47
    // Inc 5 the workflow also composes `step:author-commit`, whose
    // `<<author: commit:<id>#summary>>` belongs to that step, not to `author-prd`
    // (its exactly-once-ness is the `commit_solicit_axis` suite's claim).
    let author_lines = stdout
        .lines()
        .map(str::trim_start)
        .filter(|l| l.starts_with("<<author:") && !l.contains("commit:"))
        .count();
    assert_eq!(
        author_lines, 2,
        "author-prd must emit exactly two `<<author: …>>` slot lines (vision/context); got:\n{stdout}",
    );

    // The per-requirement item verbs compose: add-item mints a requirement, then its
    // statement slot is set on the item leaf — mirroring the arch-doc component path.
    assert!(
        stdout.contains("jigc doc add-item prd:<slug>#requirements"),
        "author-prd must carry the per-requirement `add-item` guidance; got:\n{stdout}",
    );
    assert!(
        stdout.contains("#requirements/<id>/statement"),
        "author-prd must carry the per-requirement `set-slot …/statement` guidance; got:\n{stdout}",
    );

    assert!(
        stdout.trim_end().ends_with(ROUTING_FOOTER),
        "Form-D agent-text composition must end with the routing footer; got:\n{stdout}",
    );
}

#[test]
fn form_d_architecture_documentation_mints_and_emits_the_create_gate_author_slot_and_item_lines() {
    // M13 Increment 4 / T2. `--workflow architecture-documentation <intent>`: the
    // arch-doc-authoring work-workflow. It is `creates-task: true` with
    // `allows-create: [{type: arch-doc, as: arch-doc}]`, so Form D mints + composes
    // it end-to-end. Its `author-arch-doc` step carries the create-gate (emitted as a
    // `Run: jigc doc create arch-doc` line), the `overview` author slot (emitted as a
    // `<<author: …>>` line bound to `task.arch-doc#overview`, empty pre-create like
    // prd's slots), the doc-level `cites` set-field guidance, and the per-component
    // `add-item` / set-slot / set-field guidance lines (the inc-1-3 item verbs).
    //
    // This is the `start_compose.rs` home for the binary-driven compose e2e (the
    // `doc_write.rs`-style — driving the built `jigc` against a throwaway repo); it
    // mirrors `form_d_project_setup` / `form_d_plan`.
    let repo = TempDir::new("form-d-arch-doc");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let out = run_start(
        repo.path(),
        home.path(),
        &[
            "--workflow",
            "architecture-documentation",
            "document the compose pipeline",
        ],
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    assert!(
        out.status.success(),
        "`jigc start --workflow architecture-documentation \"<intent>\"` must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );

    // The work-workflow minted under .jigc/tasks/<slug>/, recording its own id.
    let task_dir = repo
        .path()
        .join(".jigc")
        .join("tasks")
        .join("document-the-compose-pipeline");
    assert!(
        task_dir.join("base.json").is_file(),
        "architecture-documentation is creates-task: true, so it must open .jigc/tasks/document-the-compose-pipeline/ with a base pin",
    );
    let recorded =
        fs::read_to_string(task_dir.join("workflow")).expect("read recorded workflow id");
    assert_eq!(
        recorded.trim(),
        "architecture-documentation",
        "the minted task must record `workflow: architecture-documentation`",
    );

    // The create-gate emits as a machine-extractable `Run:` line for `arch-doc`.
    assert!(
        stdout.contains("Run: `jigc doc create arch-doc"),
        "the author-arch-doc step must emit the `jigc doc create arch-doc` create-gate; got:\n{stdout}",
    );

    // The `overview` prose slot emits as exactly one `<<author: …>>` line. The
    // `arch-doc` role is unbound on this first compose (the doc is not created yet),
    // so the bound slug is not knowable — but the directive still names the slot
    // from the path's role + `#fragment` (`<<author: arch-doc#overview>>`) rather
    // than emitting an empty target the agent must guess into; a re-compose after
    // the create fills in the resolved slug. The `commit` directive is excluded:
    // since M47 Inc 5 the workflow also composes `step:author-commit`, whose
    // `<<author: commit:<id>#summary>>` belongs to that step, not to
    // `author-arch-doc` (its exactly-once-ness is the `commit_solicit_axis` claim).
    let author_lines: Vec<&str> = stdout
        .lines()
        .map(str::trim_start)
        .filter(|l| l.starts_with("<<author:") && !l.contains("commit:"))
        .collect();
    assert_eq!(
        author_lines.len(),
        1,
        "author-arch-doc must emit exactly one `<<author: …>>` slot line (the overview); got:\n{stdout}",
    );
    assert_eq!(
        author_lines[0], "<<author: arch-doc#overview>>",
        "the unbound overview author directive must name the `#overview` slot, not emit an empty target; got:\n{stdout}",
    );

    // The doc-level `cites` set-field guidance line composes (the n→n cites→adr edge).
    assert!(
        stdout.contains("jigc doc set-field arch-doc:<slug>#cites"),
        "author-arch-doc must carry the doc-level `cites` set-field guidance; got:\n{stdout}",
    );

    // V10 (M41 inc 8): the ordinary authoring path warns on committed-first ordering,
    // mirroring the migration sibling — a cited ADR must ALREADY be committed before
    // this task, arch-doc's `allows-create` cannot mint an ADR in-task, and a dangling
    // `cites` blocks finalize forever. The warning composes into the emitted step text.
    assert!(
        stdout.contains("already be committed")
            && stdout.contains("cannot mint an ADR in-task")
            && stdout.contains("blocks finalize forever"),
        "author-arch-doc must carry the committed-first / dangling-cites-blocks-finalize warning; got:\n{stdout}",
    );

    // The per-component item verbs compose: add-item mints a component, then its
    // description slot + implemented-by code-anchor field are set on the item leaf.
    assert!(
        stdout.contains("jigc doc add-item arch-doc:<slug>#components"),
        "author-arch-doc must carry the per-component `add-item` guidance; got:\n{stdout}",
    );
    assert!(
        stdout.contains("#components/<id>/description"),
        "author-arch-doc must carry the per-component `set-slot …/description` guidance; got:\n{stdout}",
    );
    assert!(
        stdout.contains("#components/<id>/implemented-by"),
        "author-arch-doc must carry the per-component `set-field …/implemented-by` guidance; got:\n{stdout}",
    );

    // The finalize line composes (the proven code-less finalize → promote spine).
    assert!(
        stdout.contains("Run: `jigc task finalize document-the-compose-pipeline`"),
        "author-arch-doc → finalize must emit the `jigc task finalize` line; got:\n{stdout}",
    );

    assert!(
        stdout.trim_end().ends_with(ROUTING_FOOTER),
        "Form-D agent-text composition must end with the routing footer; got:\n{stdout}",
    );
}

#[test]
fn arch_doc_component_anchor_prose_names_the_staged_index_and_repo_relative_base() {
    // M45 Inc 10 / T1 (law 1 — nothing lies; `validation.md` → the `doc-code` probe
    // reads the materialized STAGED INDEX at task/finalize scope, not the working
    // tree — §279/§281). author-arch-doc.yaml:24 previously told agents the
    // `path#symbol` anchor is "re-checked against the working tree", the exact false
    // mental model M42 caught in the finding message (a symbol written but not
    // `git add`ed is ABSENT from the index, so the citation dangles and finalize
    // blocks — the working tree is irrelevant to the adjudication). The composed
    // components-anchor prose must name the staged index the probe actually
    // adjudicates AND state the anchor base is repo-relative (the sibling
    // locate-from-spec step already does). Asserted on the emitted step bytes.
    let repo = TempDir::new("arch-doc-anchor-truth");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let out = run_start(
        repo.path(),
        home.path(),
        &[
            "--workflow",
            "architecture-documentation",
            "document the compose pipeline",
        ],
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    assert!(
        out.status.success(),
        "the arch-doc compose must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );

    // The false claim is gone: finalize does NOT re-check the anchor against the
    // working tree. (finalize.yaml carries no "working tree" mention, so the arch-doc
    // composed view names it nowhere once this step is fixed.)
    assert!(
        !stdout.contains("working tree"),
        "the components-anchor prose must not claim finalize re-checks against the working tree; got:\n{stdout}",
    );
    // It names the staged index — the root the probe adjudicates at finalize scope.
    assert!(
        stdout.contains("staged index"),
        "the components-anchor prose must name the staged index the probe adjudicates; got:\n{stdout}",
    );
    // And it states the anchor base is repo-relative (matching locate-from-spec).
    assert!(
        stdout.contains("repo-relative"),
        "the components-anchor prose must state the `path#symbol` anchor is repo-relative; got:\n{stdout}",
    );
}

#[test]
fn bare_intent_router_lists_architecture_documentation_with_its_when() {
    // M13 Increment 4 / T2. The new `creates-task: true` work-workflow auto-lists in
    // the router's selectable catalog (`selectable_workflows`, zero code) — bare
    // `jigc start "<intent>"` composes the no-mint router, which lists
    // `architecture-documentation` as a `- <id> — <when>` option alongside the
    // existing work-workflows, and mints NOTHING.
    let repo = TempDir::new("router-arch-doc");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let out = run_start(repo.path(), home.path(), &["document the compose pipeline"]);
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    assert!(
        out.status.success(),
        "bare `jigc start \"<intent>\"` must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );

    // The router is `creates-task: false`: no `.jigc/tasks/` working area appears.
    assert!(
        !repo.path().join(".jigc").join("tasks").exists(),
        "bare `jigc start` composes the router — no .jigc/tasks/ dir may be minted",
    );

    // The router lists the new workflow with its `when` hint, alongside the existing
    // work-workflows (single-task proves the rest of the catalog still composes).
    assert!(
        stdout.contains(
            "- architecture-documentation — document the architecture of a part of the system",
        ),
        "the router must list architecture-documentation as a `- <id> — <when>` option; got:\n{stdout}",
    );
    assert!(
        stdout.contains("- single-task — implement one scoped change end-to-end, recording its decisions as ADRs and user-facing effects on the changelog"),
        "the router must still list the existing single-task option; got:\n{stdout}",
    );
}

#[test]
fn form_d_implement_from_spec_mints_and_composes_the_locate_from_spec_step() {
    let repo = TempDir::new("form-d-impl-from-spec");
    init_repo(repo.path());
    let home = TempDir::new("home");

    // A committed spec instance so `{{store.specs}}` renders a non-empty Content
    // list (`committed_store` enumerates `docs/specs/<slug>.md` by filename). The file
    // content is irrelevant to enumeration; only the slug is read.
    fs::create_dir_all(repo.path().join("docs").join("specs")).expect("create specs dir");
    fs::write(
        repo.path()
            .join("docs")
            .join("specs")
            .join("gateway-rate-limiting.md"),
        "# Gateway rate limiting\n",
    )
    .expect("write committed spec");

    // `--workflow implement-from-spec <intent>`: the spec-driven work-workflow
    // (Increment 4). It is `creates-task: true` with `reads: [{role: spec,
    // type: spec}]`; its `step:locate-from-spec` surfaces `{{store.specs}}`, emits
    // the bind + re-compose `Run:` lines, and reads `{{@task.spec#criteria}}` —
    // which resolves empty on this first compose (nothing bound yet).
    let out = run_start(
        repo.path(),
        home.path(),
        &[
            "--workflow",
            "implement-from-spec",
            "implement gateway rate limiting",
        ],
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    assert!(
        out.status.success(),
        "`jigc start --workflow implement-from-spec \"<intent>\"` must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );

    // The work-workflow minted under .jigc/tasks/<slug>/, recording its own id.
    let task_dir = repo
        .path()
        .join(".jigc")
        .join("tasks")
        .join("implement-gateway-rate-limiting");
    assert!(
        task_dir.join("base.json").is_file(),
        "implement-from-spec is creates-task: true, so it must open .jigc/tasks/implement-gateway-rate-limiting/ with a base pin",
    );
    let recorded =
        fs::read_to_string(task_dir.join("workflow")).expect("read recorded workflow id");
    assert_eq!(
        recorded.trim(),
        "implement-from-spec",
        "the minted task must record `workflow: implement-from-spec`",
    );

    // The composed view embeds the resolved `{{task.intent}}` ...
    assert!(
        stdout.contains("implement gateway rate limiting"),
        "the composed view must embed the resolved intent; got:\n{stdout}",
    );
    // ... carries the `{{store.specs}}` Content list (the committed spec, as a
    // `> <type>:<slug>` blockquote line) ...
    assert!(
        stdout.contains("> spec:gateway-rate-limiting"),
        "the locate-from-spec step must surface the committed spec via {{store.specs}}; got:\n{stdout}",
    );
    // ... the bind `Run:` line — runnable as emitted: the agent fills <SPEC_ADDRESS>
    // (round-2 D5: the placeholder teaches the FULL `spec:<slug>` address form — the
    // old `<SPEC_ID>` invited the bare slug, which `task bind` rejects as a malformed
    // address) and the trailing task id is resolved (the `jigc task bind <role>
    // <addr> <id>` clap surface needs all three positionals) ...
    assert!(
        stdout
            .contains("Run: `jigc task bind spec <SPEC_ADDRESS> implement-gateway-rate-limiting`"),
        "the locate-from-spec step must emit a runnable `jigc task bind spec <SPEC_ADDRESS> <id>` line carrying the resolved task id; got:\n{stdout}",
    );
    // ... with the surrounding prose teaching the full-address form.
    assert!(
        stdout.contains("the `spec:` prefix included"),
        "the step prose must teach the full `spec:<slug>` address form; got:\n{stdout}",
    );
    // ... and the re-compose `Run:` line carrying the resolved task id.
    assert!(
        stdout.contains("Run: `jigc start --task implement-gateway-rate-limiting`"),
        "the locate-from-spec step must emit the `jigc start --task <id>` re-compose line with the resolved task id; got:\n{stdout}",
    );

    // `{{@task.spec#criteria}}` resolves empty on the first compose — nothing is
    // bound yet, so it emits no criteria CONTENT line (a `> ` blockquote sliced from a
    // committed spec: the item's `> ### <title>  {#id}` heading + its statement). The
    // property is asserted over the rendered slice, not over the bare `#criteria`
    // substring: since M42 Inc 11 the step's prose names the `#criteria` **address** —
    // the `maps-to-test` write it demands — while the slice itself stays unresolved.
    assert!(
        !stdout
            .lines()
            .any(|line| line.starts_with("> ") && line.contains("#criteria")),
        "on the first compose the spec role is unbound, so {{@task.spec#criteria}} must resolve empty; got:\n{stdout}",
    );
    assert!(
        !stdout.lines().any(|line| line.starts_with("> ###")),
        "on the first compose no criterion item may render — the spec role is unbound; got:\n{stdout}",
    );

    assert!(
        stdout.trim_end().ends_with(ROUTING_FOOTER),
        "Form-D agent-text composition must end with the routing footer; got:\n{stdout}",
    );
}

#[test]
fn form_d_quick_fix_mints_and_composes_without_adr_or_supersedes() {
    let repo = TempDir::new("form-d-quick-fix");
    init_repo(repo.path());
    let home = TempDir::new("home");

    // `--workflow quick-fix <intent>`: the second selectable work-workflow. It is
    // `creates-task: true`, so Form D mints + composes it — but it is commit-only
    // (`allows-create: []`), so its composed text must carry no ADR/create
    // affordance and no superseded-context line, materially differing from
    // `single-task`.
    let out = run_start(
        repo.path(),
        home.path(),
        &["--workflow", "quick-fix", "fix typo in readme"],
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    assert!(
        out.status.success(),
        "`jigc start --workflow quick-fix \"<intent>\"` must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );

    // The work-workflow minted under .jigc/tasks/<slug>/.
    let task_dir = repo
        .path()
        .join(".jigc")
        .join("tasks")
        .join("fix-typo-in-readme");
    assert!(
        task_dir.join("base.json").is_file(),
        "quick-fix is creates-task: true, so it must open .jigc/tasks/fix-typo-in-readme/ with a base pin",
    );

    // It composed end-to-end, embedding the resolved intent.
    assert!(
        stdout.contains("fix typo in readme"),
        "the composed view must embed the resolved intent; got:\n{stdout}",
    );

    // Materially different from single-task: no ADR/create affordance ...
    assert!(
        !stdout.contains("create-adr") && !stdout.to_lowercase().contains("adr"),
        "quick-fix is commit-only — its composed view must carry no ADR/create affordance; got:\n{stdout}",
    );
    // ... and no superseded-context line.
    assert!(
        !stdout.to_lowercase().contains("supersede"),
        "quick-fix must carry no superseded-context line; got:\n{stdout}",
    );
}

#[test]
fn form_d_router_lists_selectable_workflows_and_re_run_without_minting() {
    let repo = TempDir::new("form-d-router");
    init_repo(repo.path());
    let home = TempDir::new("home");

    // `--workflow router <intent>`: the selection workflow. It is
    // `creates-task: false`, so Form D composes it with no task context — no mint
    // — interpolating `{{catalog}}` to the selectable (`creates-task: true`)
    // work-workflows and emitting the literal agent-substitution re-run.
    let out = run_start(
        repo.path(),
        home.path(),
        &["--workflow", "router", "add rate limiter"],
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    assert!(
        out.status.success(),
        "`jigc start --workflow router \"<intent>\"` must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );

    // The router mints nothing — `creates-task: false` composes with no task
    // context, so no `.jigc/tasks/` dir may be opened.
    assert!(
        !repo.path().join(".jigc").join("tasks").exists(),
        "a `creates-task: false` router must mint nothing — no .jigc/tasks/ dir may be created",
    );

    // The catalog interpolated to one `- <id> — <when>` line per selectable
    // work-workflow: both `single-task` and `quick-fix`, with their `when` hints.
    assert!(
        stdout.contains("- single-task — implement one scoped change end-to-end, recording its decisions as ADRs and user-facing effects on the changelog"),
        "the router must list single-task as a `- <id> — <when>` option; got:\n{stdout}",
    );
    assert!(
        stdout.contains("- quick-fix — apply a small commit-only fix that touches no documented code (code a managed doc names) and records no decision"),
        "the router must list quick-fix as a `- <id> — <when>` option; got:\n{stdout}",
    );

    // The route-to-workflow step carries the literal agent-substitution re-run —
    // angle-bracket markers the agent fills, not placeholders or slots.
    assert!(
        stdout.contains("jigc start --workflow <chosen> \"<intent>\""),
        "the router must carry the literal agent-substitution re-run prose; got:\n{stdout}",
    );
}

#[test]
fn form_d_unknown_workflow_rejects_before_minting() {
    let repo = TempDir::new("form-d-unknown");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let out = run_start(
        repo.path(),
        home.path(),
        &["--workflow", "does-not-exist", "add rate limiter"],
    );
    let stderr = String::from_utf8(out.stderr).expect("utf-8 stderr");

    assert!(
        !out.status.success(),
        "an unknown `--workflow` id must exit non-zero; got {:?}",
        out.status,
    );
    assert!(
        stderr.contains("does-not-exist") && stderr.contains("route:"),
        "the rejection must name the unknown id and carry a route; got:\n{stderr}",
    );
    assert!(
        !repo.path().join(".jigc").join("tasks").exists(),
        "the rejection must precede minting — no .jigc/tasks/ dir may be created",
    );
}

#[test]
fn serial_re_run_of_the_same_intent_blocks() {
    let repo = TempDir::new("compose-collision");
    init_repo(repo.path());
    let home = TempDir::new("home");

    // Post-flip, minting goes through Form D — bare intent composes the router and
    // mints nothing, so the collision is provoked via `--workflow single-task`.
    let first = run_start(
        repo.path(),
        home.path(),
        &["--workflow", "single-task", "add rate limiter"],
    );
    assert!(
        first.status.success(),
        "the first mint+compose must succeed"
    );

    let second = run_start(
        repo.path(),
        home.path(),
        &["--workflow", "single-task", "add rate limiter"],
    );
    assert!(
        !second.status.success(),
        "a serial re-run of the same intent must exit non-zero (serial collision)",
    );
    let stderr = String::from_utf8(second.stderr).expect("utf-8 stderr");
    assert!(
        stderr.contains("add-rate-limiter"),
        "the serial-collision block must name the task; got:\n{stderr}",
    );
}

#[test]
fn form_d_refuses_a_verb_routed_workflow_and_mints_nothing() {
    let repo = TempDir::new("form-d-sub-task");
    init_repo(repo.path());
    let home = TempDir::new("home");

    // `sub-task` is `creates-task: true`, so Form D used to mint and compose it — and
    // that was the defect: the body writes into a milestone sub-area this door binds
    // none of, so the composed walk instructed an agent into a task with no commit
    // boundary (M52 Increment 9 / T2; `completions/artifacts/M52/baseline-surfaces.md`
    // §4.1). The pack declares the door it IS reached through
    // (`suppressed.door: jigc workflow sub-task --task <task-id>`), so this door now
    // refuses and names it. The composed body itself is pinned at that door, in
    // `workflow_reentry.rs`.
    let out = run_start(
        repo.path(),
        home.path(),
        &["--workflow", "sub-task", "add rate limiter"],
    );
    let stderr = String::from_utf8(out.stderr).expect("utf-8 stderr");

    assert_eq!(
        out.status.code(),
        Some(1),
        "`jigc start --workflow sub-task \"<intent>\"` must refuse at exit 1; stdout:\n{}\nstderr:\n{stderr}",
        String::from_utf8_lossy(&out.stdout),
    );
    assert!(
        stderr.contains("workflow.verb-routed")
            && stderr.contains("`jigc workflow sub-task --task <task-id>`"),
        "the refusal names the code and routes at the declared door; got:\n{stderr}",
    );
    // Refused before anything mints — a refused name strands no task dir.
    assert!(
        !repo
            .path()
            .join(".jigc")
            .join("tasks")
            .join("add-rate-limiter")
            .exists(),
        "a refused Form D must open no working area",
    );
}

#[test]
fn form_d_creates_task_workflow_with_no_intent_rejects() {
    // A `creates-task: true` workflow slugs its task id from the `<intent>`
    // positional (`design/write-commands.md` → Task origination), so `--workflow <X>`
    // with NO intent cannot mint. It must be rejected with an actionable message —
    // never silently fall through to the read-only orientation listing (the bug).
    let repo = TempDir::new("form-d-no-intent-reject");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let out = run_start(
        repo.path(),
        home.path(),
        &["--workflow", "architecture-documentation"],
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    let stderr = String::from_utf8(out.stderr).expect("utf-8 stderr");

    // Non-zero: the missing intent is a rejection, not a silent orient.
    assert!(
        !out.status.success(),
        "`--workflow <creates-task> ` with no intent must exit non-zero; got {:?}\nstdout:\n{stdout}",
        out.status,
    );
    // The message names the workflow and shows the correct intent-bearing form.
    assert!(
        stderr.contains("architecture-documentation") && stderr.contains("requires an intent"),
        "the rejection must name the workflow and say it requires an intent; got:\n{stderr}",
    );
    assert!(
        stderr.contains("jigc start \"<intent>\" --workflow architecture-documentation"),
        "the rejection must show the corrected `jigc start \"<intent>\" --workflow X` form; got:\n{stderr}",
    );
    // It must NOT emit the orientation listing (the silent-fallthrough bug).
    assert!(
        !stdout.contains(ROUTING_FOOTER) && stdout.trim().is_empty(),
        "a rejected `--workflow` with no intent must emit no orientation output on stdout; got:\n{stdout}",
    );
    // No task dir is opened — rejected before any mint.
    assert!(
        !repo.path().join(".jigc").join("tasks").exists(),
        "a rejected `--workflow <creates-task>` must mint nothing — no .jigc/tasks/ dir",
    );
}

#[test]
fn form_d_no_task_workflow_with_no_intent_still_composes() {
    // A `creates-task: false` workflow composes with NO intent — `jigc start
    // --workflow ingest-existing` is valid (`design/write-commands.md` → Task
    // origination): it mints nothing and the intent threads nowhere. This path must
    // NOT be broken by the missing-intent rejection (which is gated on creates-task).
    let repo = TempDir::new("form-d-no-intent-compose");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let out = run_start(repo.path(), home.path(), &["--workflow", "ingest-existing"]);
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    assert!(
        out.status.success(),
        "`--workflow ingest-existing` (creates-task: false) with no intent must compose + exit 0; \
         got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    // It composed the workflow body (its ingest launch line), not orientation.
    assert!(
        stdout.contains("Run: `jigc ingest`"),
        "the composed `ingest-existing` view must carry its `jigc ingest` launch line; got:\n{stdout}",
    );
    // `creates-task: false` mints nothing.
    assert!(
        !repo.path().join(".jigc").join("tasks").exists(),
        "a `creates-task: false` workflow must mint nothing — no .jigc/tasks/ dir",
    );
}

#[test]
fn store_placement_doctype_composes_its_committed_singleton() {
    // **M42 Inc-2 T4 — `{{store.<placement-type>}}` resolves** (`design/storage.md` →
    // The census, the 14th site). The compose-time `store` resolver keyed only by a
    // schema's `location` stem, so a **placement** doctype (`location: None`, homed at a
    // literal `placement.file`) fell through the transient arm: its committed singleton
    // was invisible, and an absent `store` key renders the *empty* case — empty text, no
    // finding, exit 0. A pack writing `{{store.changelog}}` therefore composed a workflow
    // reading *as if no changelog were committed*, silently. This drives the emitted
    // bytes through the real binary: a listed filesystem pack whose step body carries
    // `{{ store.changelog }}` must compose `> changelog:changelog` over a repo holding a
    // committed root `CHANGELOG.md` — and, over a repo holding none, must compose inert
    // (no address, no finding, exit 0).
    let repo = TempDir::new("store-placement");
    init_repo(repo.path());
    let home = TempDir::new("home");

    // A listed filesystem pack: its own command catalog (pack-local command-ref
    // resolution), one step body reading the placement collection, and a
    // `creates-task: false` workflow including it (nothing to mint — the step is a pure
    // read of the committed store).
    let listed = TempDir::new("listed-store-pack");
    let config = listed.path().join("config");
    let steps = listed.path().join("steps");
    let workflows = listed.path().join("workflows");
    fs::create_dir_all(&config).expect("mk listed config/");
    fs::create_dir_all(&steps).expect("mk listed steps/");
    fs::create_dir_all(&workflows).expect("mk listed workflows/");
    fs::write(config.join("commands.yaml"), "commands: []\n").expect("seed empty catalog");
    // A listed pack that ships steps is inside the stated-at fence's structural tier
    // since M51 Increment 8 T2, and the tier checks each constituent in ISOLATION —
    // the dev pack's declarers in the same composition do not answer for it.
    crate::support::seed_ambush_class_declarer(listed.path());
    fs::write(
        steps.join("read-changelog.yaml"),
        "The committed changelog:\n\n{{ store.changelog }}\n",
    )
    .expect("seed read-changelog step");
    fs::write(
        workflows.join("read-changelog.yaml"),
        "---\n\
         when: read the committed changelog singleton\n\
         description: A read workflow surfacing the committed changelog collection.\n\
         usage: proving `{{store.<placement-type>}}` resolves.\n\
         creates-task: false\n\
         ---\n\
         {{ include: step:read-changelog }}\n",
    )
    .expect("seed read-changelog workflow");
    fs::write(
        repo.path().join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", listed.path().display()),
    )
    .expect("write packs.yaml naming the listed pack");

    // The OMITTING context first: no committed `CHANGELOG.md` — the collection is empty,
    // which is the *empty* case (inert), never an error.
    let out = run_start(repo.path(), home.path(), &["--workflow", "read-changelog"]);
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    assert!(
        out.status.success(),
        "an uncommitted placement singleton must compose inert (exit 0); got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        !stdout.contains("changelog:changelog"),
        "with no committed CHANGELOG.md the collection is empty; got:\n{stdout}",
    );

    // The committed placement singleton at its literal repo-root home.
    fs::write(
        repo.path().join("CHANGELOG.md"),
        "# Changelog\n\n## Unreleased Changes\n",
    )
    .expect("write the committed CHANGELOG.md");

    let out = run_start(repo.path(), home.path(), &["--workflow", "read-changelog"]);
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    assert!(
        out.status.success(),
        "`--workflow read-changelog` must compose + exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    // The composed bytes carry the placement singleton as a `> <type>:<slug>` Content
    // line — keyed by the TYPE ID (`store.changelog`), addressed by the fixed singleton
    // slug (`changelog:changelog`).
    assert!(
        stdout.contains("> changelog:changelog"),
        "`{{store.changelog}}` must surface the committed placement singleton as a \
         `> changelog:changelog` Content line; got:\n{stdout}",
    );
}

/// M47 Increment 8 / T2 (N7): the composed `task scope:` line's parallelism claim, driven
/// **literally** — two open tasks, each addressed by its own `--task`, one finalizes and
/// the other resumes and finalizes on top of it, every call exit 0.
///
/// The line promised *"you can run them in parallel"* flat, while the resume door
/// blanket-refused any `base != HEAD` — so the very sequence it invited (finalize one of
/// the parallel pair, come back to the other) was refused by the binary at the door the
/// same footer names two lines above (`resume:`). The door is fixed (M47 Inc 8 / T1) and
/// the claim is now **scoped to what holds**: parallel work is carried while it stays
/// disjoint, and the counter-arm below drives the overlap the scoping names.
#[test]
fn two_open_tasks_run_in_parallel_and_both_finalize() {
    let repo = TempDir::new("parallel-disjoint");
    let home = TempDir::new("home");
    init_repo(repo.path());

    // Two open tasks through the same front door.
    let alpha = run_start(
        repo.path(),
        home.path(),
        &["--workflow", "quick-fix", "fix alpha"],
    );
    assert_ok(&alpha, "`jigc start --workflow quick-fix \"fix alpha\"`");
    let beta = run_start(
        repo.path(),
        home.path(),
        &["--workflow", "quick-fix", "fix beta"],
    );
    assert_ok(&beta, "`jigc start --workflow quick-fix \"fix beta\"`");

    // The claim is scoped where it is printed: the `task scope:` line names the condition
    // the parallel promise holds under, and the block that follows when it does not — a
    // flat promise here is the law-1 lie this arm exists to keep out.
    let composed = String::from_utf8(beta.stdout).expect("utf-8 composed stdout");
    let scope = composed
        .lines()
        .find(|l| l.starts_with("task scope:"))
        .unwrap_or_else(|| panic!("a task-scope line renders; got:\n{composed}"));
    assert!(
        scope.contains("several open tasks are legal") && scope.contains("parallel"),
        "the parallelize affordance survives the scoping; got:\n{scope}",
    );
    assert!(
        scope.contains("disjoint"),
        "the parallel claim must name the condition it holds under — the work stays \
         disjoint; got:\n{scope}",
    );
    assert!(
        scope.contains("blocks") && scope.contains("overlap"),
        "and must name what happens when it does not — the overlapping paths block, \
         which is what makes the promise followable; got:\n{scope}",
    );

    // Both commit docs are authored while BOTH tasks are open, each addressed by its own
    // `--task` — the line's other half, exercised rather than asserted.
    fill_commit(repo.path(), home.path(), "fix-alpha", "fix alpha");
    fill_commit(repo.path(), home.path(), "fix-beta", "fix beta");

    // Disjoint work: one file per task.
    let before: u32 = git_in(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    fs::write(repo.path().join("alpha.txt"), "alpha\n").expect("write alpha");
    git_in(repo.path(), &["add", "alpha.txt"]);
    assert_ok(
        &run_jigc(repo.path(), home.path(), &["task", "finalize", "fix-alpha"]),
        "the first parallel task finalizes",
    );

    // The second task's base has moved. Its work is disjoint, so the door the footer
    // names two lines above carries it through — this is the call that was refused.
    let resume = run_start(repo.path(), home.path(), &["--task", "fix-beta"]);
    assert_ok(
        &resume,
        "`jigc start --task fix-beta` after its parallel sibling committed",
    );

    fs::write(repo.path().join("beta.txt"), "beta\n").expect("write beta");
    git_in(repo.path(), &["add", "beta.txt"]);
    assert_ok(
        &run_jigc(repo.path(), home.path(), &["task", "finalize", "fix-beta"]),
        "the second parallel task finalizes on top of the first",
    );

    let after: u32 = git_in(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    assert_eq!(
        after,
        before + 2,
        "both parallel tasks land their own commit",
    );
}

/// M47 Increment 8 / T2 (N7), the counter-arm: the scoped wording's *other* half, driven.
/// Two open tasks whose work **overlaps** — one commits a path the other is editing — and
/// the second one's resume blocks, naming the overlapping path. That refusal is exactly
/// what the `task scope:` line now describes, so the scoping is a statement about
/// observed behaviour rather than a hedge.
#[test]
fn overlapping_parallel_work_blocks_and_names_the_path() {
    let repo = TempDir::new("parallel-overlap");
    let home = TempDir::new("home");
    init_repo(repo.path());

    for intent in ["fix alpha", "fix beta"] {
        assert_ok(
            &run_start(
                repo.path(),
                home.path(),
                &["--workflow", "quick-fix", intent],
            ),
            "`jigc start --workflow quick-fix`",
        );
    }
    fill_commit(repo.path(), home.path(), "fix-alpha", "fix alpha");
    fill_commit(repo.path(), home.path(), "fix-beta", "fix beta");

    // Both tasks touch the SAME file — the case the scoping carves out.
    fs::write(repo.path().join("shared.txt"), "from alpha\n").expect("write shared");
    git_in(repo.path(), &["add", "shared.txt"]);
    assert_ok(
        &run_jigc(repo.path(), home.path(), &["task", "finalize", "fix-alpha"]),
        "the first task commits the shared path",
    );
    fs::write(repo.path().join("shared.txt"), "from beta\n").expect("edit shared in beta");

    let resume = run_start(repo.path(), home.path(), &["--task", "fix-beta"]);
    assert!(
        !resume.status.success(),
        "overlapping parallel work must block, not silently re-pin; got:\n{}",
        String::from_utf8_lossy(&resume.stdout),
    );
    let stderr = String::from_utf8(resume.stderr).expect("utf-8 stderr");
    assert!(
        stderr.contains("shared.txt"),
        "the block must NAME the overlapping path — the footer's scoped wording promises \
         exactly that; got:\n{stderr}",
    );
}

/// The `also open:` block's lead line and its one row, for the fixture below — pinned
/// byte for byte, because the whole deliverable of M50 Increment 5 / T3 is *these bytes
/// exist*. The row names the id, the workflow that minted it, and the door that
/// re-composes it; the lead line carries the legality claim once (`design/
/// surface-contract.md` → law 2: nothing hides) and states that this call left it alone.
const ALSO_OPEN_ONE: &str = "\
also open: 1 other task was already open before this call — nothing here touched it; several open tasks are legal, each addressed by its own `--task`:
  - `fix-alpha` (workflow `quick-fix`) — resume it with `jigc start --task fix-alpha`
";

/// Slice the `also open:` block out of a composed agent-text render: the lead line
/// through the last row, i.e. everything from `also open:` up to the routing footer.
/// Panics when the block is absent, so a caller asserting its *content* can never pass
/// over an empty haystack.
fn also_open_block(composed: &str) -> String {
    let start = composed
        .find("also open:")
        .unwrap_or_else(|| panic!("an `also open:` block renders; got:\n{composed}"));
    let rest = &composed[start..];
    let end = rest
        .find(ROUTING_FOOTER)
        .unwrap_or_else(|| panic!("the composed text ends with the routing footer; got:\n{rest}"));
    rest[..end].to_string()
}

/// M50 Increment 5 / T3 (F-5): `jigc start --workflow <X> "<intent>"` — the explicit
/// work-starting form — **names the task already open** and **still mints its own**.
///
/// The silence was the finding: a worker whose session had lost context re-ran the front
/// door, got a clean compose, and learned nothing about the task already in flight — the
/// one door on F-5's axis that *changes the repo* while staying silent. Suppressing the
/// second mint was rejected at the Settle (B8): `render.rs`'s own `task scope:` line
/// composes *"several open tasks are legal … you can run them in parallel"* into every
/// task, so a refusal here would contradict the same binary's own surface.
#[test]
fn the_named_workflow_form_names_the_open_task_and_still_mints() {
    let repo = TempDir::new("also-open-form-d");
    let home = TempDir::new("home");
    init_repo(repo.path());

    // The FIRST call runs over a task-less repo: the omitting context renders no bytes,
    // and the task it just minted is not named to itself.
    let alpha = run_start(
        repo.path(),
        home.path(),
        &["--workflow", "quick-fix", "fix alpha"],
    );
    assert_ok(&alpha, "`jigc start --workflow quick-fix \"fix alpha\"`");
    let first = String::from_utf8(alpha.stdout).expect("utf-8 composed stdout");
    assert!(
        !first.contains("also open"),
        "over a repo with no other live task the block renders NO bytes — never `none`, \
         and never the freshly minted task named to itself; got:\n{first}",
    );

    // The SECOND call, with `fix-alpha` live.
    let beta = run_start(
        repo.path(),
        home.path(),
        &["--workflow", "quick-fix", "fix beta"],
    );
    assert_ok(&beta, "`jigc start --workflow quick-fix \"fix beta\"`");
    let second = String::from_utf8(beta.stdout).expect("utf-8 composed stdout");
    assert_eq!(
        also_open_block(&second),
        ALSO_OPEN_ONE,
        "the `also open:` block names the open task, its workflow and its resume door; \
         got:\n{second}",
    );
    assert!(
        !also_open_block(&second).contains("fix-beta"),
        "the task THIS call minted is not named to itself; got:\n{second}",
    );

    // And it still minted: two ids, at exit 0.
    let list = run_jigc(repo.path(), home.path(), &["task", "list"]);
    assert_ok(&list, "`jigc task list`");
    let roster = String::from_utf8(list.stdout).expect("utf-8 roster");
    assert!(
        roster.contains("fix-alpha") && roster.contains("fix-beta"),
        "naming the open task must not suppress the mint — both ids stay live; \
         got:\n{roster}",
    );
}

/// M50 Increment 5 / T3: the **cascade-default** form — `jigc start "<intent>"`, which
/// composes the `creates-task: false` router and mints nothing — names the open task too.
/// It is the door an agent reaches for first, and the one that produces no task id of its
/// own to anchor the state on, so its silence was the more complete one.
#[test]
fn the_router_form_names_the_task_already_open() {
    let repo = TempDir::new("also-open-router");
    let home = TempDir::new("home");
    init_repo(repo.path());

    assert_ok(
        &run_start(
            repo.path(),
            home.path(),
            &["--workflow", "quick-fix", "fix alpha"],
        ),
        "`jigc start --workflow quick-fix \"fix alpha\"`",
    );

    let router = run_start(repo.path(), home.path(), &["add a second thing"]);
    assert_ok(&router, "`jigc start \"<intent>\"`");
    let composed = String::from_utf8(router.stdout).expect("utf-8 composed stdout");
    assert_eq!(
        also_open_block(&composed),
        ALSO_OPEN_ONE,
        "the router form names the open task in the same block; got:\n{composed}",
    );

    // The router still mints nothing — the block is an addition to the read, not a door.
    let list = run_jigc(repo.path(), home.path(), &["task", "list"]);
    assert_ok(&list, "`jigc task list`");
    let roster = String::from_utf8(list.stdout).expect("utf-8 roster");
    assert!(
        !roster.contains("add-a-second-thing"),
        "the router composes without minting; got:\n{roster}",
    );
}

/// M50 Increment 5 / T3 (Settle B8, the N>1 shape): the block carries the active **set**,
/// so two already-open tasks are two rows under one lead line — never a singular claim
/// that reports one and hides the rest.
#[test]
fn two_already_open_tasks_render_as_two_rows() {
    let repo = TempDir::new("also-open-two");
    let home = TempDir::new("home");
    init_repo(repo.path());

    for intent in ["fix alpha", "fix beta"] {
        assert_ok(
            &run_start(
                repo.path(),
                home.path(),
                &["--workflow", "quick-fix", intent],
            ),
            "`jigc start --workflow quick-fix`",
        );
    }

    let third = run_start(
        repo.path(),
        home.path(),
        &["--workflow", "single-task", "add a rate limiter"],
    );
    assert_ok(&third, "`jigc start --workflow single-task`");
    let composed = String::from_utf8(third.stdout).expect("utf-8 composed stdout");
    assert_eq!(
        also_open_block(&composed),
        "\
also open: 2 other tasks were already open before this call — nothing here touched them; several open tasks are legal, each addressed by its own `--task`:
  - `fix-alpha` (workflow `quick-fix`) — resume it with `jigc start --task fix-alpha`
  - `fix-beta` (workflow `quick-fix`) — resume it with `jigc start --task fix-beta`
",
        "two open tasks render as two rows under one lead line; got:\n{composed}",
    );
}

/// M50 Increment 5 / T3: the block is **presentation** and reaches no tooling consumer —
/// `--format json` stays exactly the pinned `{task, text}` projection
/// (`design/command-output-contract.md` §1), with the block absent from `text` too.
///
/// This is the arm the `staged_listing_hint` stderr precedent would have failed: a
/// surface whose text says one thing and whose JSON says another in one invocation is the
/// gap `command-output-contract.md` names, so the addition is proven inert on the
/// contract side rather than assumed to be.
#[test]
fn the_composed_json_is_unchanged_while_a_task_is_already_open() {
    let repo = TempDir::new("also-open-json");
    let home = TempDir::new("home");
    init_repo(repo.path());

    assert_ok(
        &run_start(
            repo.path(),
            home.path(),
            &["--workflow", "quick-fix", "fix alpha"],
        ),
        "`jigc start --workflow quick-fix \"fix alpha\"`",
    );

    let out = run_start(
        repo.path(),
        home.path(),
        &["--format", "json", "--workflow", "quick-fix", "fix beta"],
    );
    assert_ok(&out, "`jigc start --format json --workflow quick-fix`");
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    let value: serde_json::Value = serde_json::from_str(&stdout)
        .unwrap_or_else(|e| panic!("must be valid JSON ({e}); got:\n{stdout}"));
    let obj = value.as_object().expect("the composed JSON is an object");
    let keys: Vec<&str> = obj.keys().map(String::as_str).collect();
    assert_eq!(
        keys,
        vec!["task", "text"],
        "the composed JSON key set stays exactly `{{task, text}}`; got:\n{stdout}",
    );
    assert_eq!(value["task"].as_str(), Some("fix-beta"));
    assert!(
        !value["text"]
            .as_str()
            .expect("`text` is a string")
            .contains("also open"),
        "the block is frontend-appended presentation and never joins the composed text; \
         got:\n{stdout}",
    );

    // And the router form's JSON, over the same live task, is equally untouched.
    let router = run_start(
        repo.path(),
        home.path(),
        &["--format", "json", "add a second thing"],
    );
    assert_ok(&router, "`jigc start --format json \"<intent>\"`");
    let router_out = String::from_utf8(router.stdout).expect("utf-8 stdout");
    let router_value: serde_json::Value = serde_json::from_str(&router_out)
        .unwrap_or_else(|e| panic!("must be valid JSON ({e}); got:\n{router_out}"));
    let router_keys: Vec<&str> = router_value
        .as_object()
        .expect("object")
        .keys()
        .map(String::as_str)
        .collect();
    assert_eq!(router_keys, vec!["task", "text"]);
    assert!(!router_out.contains("also open"), "got:\n{router_out}",);
}

/// **M53 Increment 5 / T2 — the fourth mint door refuses a title that yields no id, under
/// the mint class's own identity.**
///
/// Driven at `4185268f`, `jigc --format json start --workflow single-task "日本語"` answered
/// `{"error": "intent must contain at least one letter or digit (got \"日本語\")"}` and logged
/// `finding_codes: []` — a sentence that is **false** of the title it names (three letters,
/// none of them ASCII), with no code for a driver to key on and an anonymous exit 1 in the
/// invocation log. D5 gives the class one producer and one sentence
/// (`completions/artifacts/M53/settle-record.md` → **D5**), and this is the arm that drives
/// the **emitted bytes** rather than the finding value: the document on the wire, the stream
/// it rides, and the record the run leaves behind.
///
/// **The arm this door takes does not move** (M53 Increment 5 / T1, the owed spike): the
/// refusal is raised through `render::finding_error`, whose `--format json` arm is the
/// flattened `{"error": …}` — the surface's default, and `write.unslugable-title` joins
/// neither `ENVELOPE_ARM_CODES` nor `render::ENVELOPE_OWED_CODES`. What the fix buys is the
/// **identity inside** that document, and the code in the log, which is the half M50's
/// completion audit installed at the funnel for exactly this reason.
#[test]
fn a_title_that_slugs_to_nothing_refuses_under_the_mint_class_code() {
    let repo = TempDir::new("unslugable-title");
    let home = TempDir::new("home");
    init_repo(repo.path());
    assert_ok(
        &run_jigc(
            repo.path(),
            home.path(),
            &["config", "set", "invocation-log", "true"],
        ),
        "`jigc config set invocation-log true`",
    );

    let out = run_jigc(
        repo.path(),
        home.path(),
        &[
            "--format",
            "json",
            "start",
            "--workflow",
            "single-task",
            "日本語",
        ],
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    let stderr = String::from_utf8(out.stderr).expect("utf-8 stderr");
    assert!(
        !out.status.success(),
        "a title that yields no id must refuse; stdout:\n{stdout}\nstderr:\n{stderr}",
    );
    assert!(
        stdout.trim().is_empty(),
        "the refusal's one document is stderr's (stream discipline); stdout:\n{stdout}",
    );

    let value: serde_json::Value = serde_json::from_str(&stderr)
        .unwrap_or_else(|e| panic!("stderr must be one JSON value ({e}); got:\n{stderr}"));
    let keys: Vec<&str> = value
        .as_object()
        .expect("the refusal document is an object")
        .keys()
        .map(String::as_str)
        .collect();
    assert_eq!(
        keys,
        vec!["error"],
        "this door's reject arm is the flattened one and no arm moves; got:\n{stderr}",
    );
    let error = value["error"].as_str().expect("`error` is a string");
    assert!(
        error.starts_with("blocking · write.unslugable-title — "),
        "the flattened document must carry the class's identity; got:\n{stderr}",
    );
    assert!(
        error.contains(
            "ids are built from ASCII letters and digits, so a title in another script, \
             or of stopwords only, yields none"
        ),
        "the sentence must be true of a title in any script; got:\n{stderr}",
    );
    assert!(
        error.contains("\n  at: task\n"),
        "the block keys at the bare work-unit type token; got:\n{stderr}",
    );
    assert!(
        error.contains("\n  route: "),
        "a blocking refusal carries a route; got:\n{stderr}",
    );

    // Nothing minted: not even the `commit`-fallback id the guard exists to prevent.
    let tasks = repo.path().join(".jigc").join("tasks");
    let minted: Vec<String> = fs::read_dir(&tasks)
        .map(|dir| {
            dir.filter_map(Result::ok)
                .map(|e| e.file_name().to_string_lossy().to_string())
                .collect()
        })
        .unwrap_or_default();
    assert!(
        minted.is_empty(),
        "a refused mint leaves `.jigc/tasks/` empty; found {minted:?}",
    );

    // The identity the surface printed is the identity the run recorded.
    let log = fs::read_to_string(
        repo.path()
            .join(".jigc")
            .join("logs")
            .join("invocations.jsonl"),
    )
    .expect("the invocation log exists once the knob is on");
    let record: serde_json::Value = serde_json::from_str(
        log.lines()
            .rfind(|line| !line.trim().is_empty())
            .expect("the refused start appended a record"),
    )
    .expect("each log line is valid JSON");
    let codes: Vec<&str> = record["finding_codes"]
        .as_array()
        .expect("`finding_codes` is an array")
        .iter()
        .map(|code| code.as_str().expect("a code is a string"))
        .collect();
    assert!(
        codes.contains(&"write.unslugable-title"),
        "a refusal that names itself on the surface names itself in the log; record:\n{record}",
    );
}
