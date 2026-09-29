//! M14 Increment 1 / T5 — the real-binary acceptance for multi-pack composition:
//! the **single-pack byte-identity floor**, a **two-pack load** (union read), and
//! the **top-level precedence-winner** for a colliding doctype + knob — all driven
//! through the built `jigc` binary against throwaway git repos.
//!
//! Increment 1's tests assert **loading + the floor + the top-level winner**
//! (`design/worked-examples.md` → flow 17 setup (a)+(b) and the walk's top-level
//! collision lines; `design/multi-pack.md` → Collision resolution). **Increment 2**
//! (`pack_local_*`, below) adds the **correctness axis** — flow 17 assertion 3: a
//! definition's body-references resolve against *its own* pack regardless of which
//! pack wins a top-level id, so co-composition never silently corrupts the loser's
//! definitions (the M3-class trap). The M12 `methodology_pack_*` compose-alone tests
//! stay green untouched. **Increment 3** (`flow17_*`, below) lands the **consolidated
//! flow-17 acceptance** — all SIX acceptance-bar assertions over the real dev ×
//! methodology pair through the built binary in one walk: 1/2/3 re-assert the
//! inc-1/inc-2 surfaces; 4 (dev doctypes LOAD + VALIDATE — a `code-anchor` field
//! resolving from dev's own `field-types.yaml` though methodology, the precedence
//! winner, ships none), 5 (provenance by path + blake3 content-hash), and 6
//! (determinism under composition — `--task` recompose + reversed pack-order
//! invariance, hardening #7) are this increment's net-new acceptance.
//!
//! The four observables, each on the EMITTED bytes the agent would actually see
//! (increment-workflow hardening #4):
//!   - (floor) with NO `packs.yaml`, bare `jigc start "<intent>"` composes the
//!     cascade-default router **byte-identical** to the single-pack golden (the same
//!     `NO_OVERRIDE_ROUTER_GOLDEN` `start_compose.rs` pins) — proving `Composite([base])`
//!     does not perturb the one-pack path (the headline regression);
//!   - (two-pack load) with `packs.yaml` listing the on-disk methodology pack OVER
//!     the embedded dev base, both packs' non-colliding workflows resolve through the
//!     union: `jigc describe` narrates methodology's `dev-task` AND dev's `single-task`;
//!   - (knob winner) `default-workflow` resolves to methodology's `dev-task` (it wins
//!     the whole-`knobs.yaml` shadow), so bare `jigc start "<intent>"` mints under
//!     `.jigc/tasks/<slug>/` — dev's router (the base's default) is `creates-task: false`
//!     and would mint NOTHING, so the mint is the proof the knob resolved to the winner;
//!   - (doctype winner) the `commit` schema resolves to methodology's, which dropped
//!     dev's `implements→spec` field: `jigc doc set-field commit:<id>#implements` blocks
//!     non-zero ("no field addressed"), where the single-pack dev base resolves it fine.
//!
//! No external test crates: the binary path comes from `CARGO_BIN_EXE_jigc`, the
//! methodology pack from `CARGO_MANIFEST_DIR/packs/methodology`, the temp repo
//! is a real `git init`, and a self-cleaning `TempDir` keeps the test off the
//! developer's real repo / files.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-multipack-accept-{tag}-{}-{:?}",
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

/// The on-disk methodology pack home (`<root>/crates/cli/packs/methodology`) — the literal
/// directory a `.jigc/config/packs:` entry names.
fn methodology_pack_tree() -> PathBuf {
    Path::new(cli::pack_path!(methodology)).to_path_buf()
}

/// The single-pack floor golden: the no-override bare-`jigc start "<intent>"`
/// compose of the cascade-default router. This is byte-identical to the
/// `NO_OVERRIDE_ROUTER_GOLDEN` `start_compose.rs` pins — kept here as the floor
/// oracle the composite-of-one path must reproduce (the headline regression: the
/// multi-pack machinery adds zero observable change to a one-pack project).
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

/// Initialize a real git repo with one commit (composition mints, which reads HEAD
/// via `git rev-parse`) and create the `.jigc/config/` project layer (the setup gate
/// the cascade + `describe` require).
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

/// Record the listed pack-set in the in-repo project layer's `packs.yaml` — the
/// pre-cascade selector `make_pack()` CWD-discovers. The named directory sits at
/// highest precedence; the embedded dev pack is the implicit base below it.
fn write_packs_yaml(repo: &Path, listed: &Path) {
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", listed.display()),
    )
    .expect("write packs.yaml naming the listed pack");
}

/// Run `jigc <args>` with `cwd = repo` and `$HOME = home`, optionally piping stdin.
fn run(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("run the jigc binary")
}

#[test]
fn floor_single_pack_compose_is_byte_identical_to_the_golden() {
    // (floor) NO `packs.yaml` ⇒ the pack-set is `[base]` (the embedded dev pack) ⇒
    // composition flows through `Composite([base])`, which must be byte-identical to
    // the single-pack path. Bare `jigc start "<intent>"` composes the cascade-default
    // router; the emitted bytes must equal the golden exactly. A composite that
    // perturbs the one-pack path is the headline regression this rejects.
    let repo = TempDir::new("floor");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let out = run(repo.path(), home.path(), &["start", "add rate limiter"]);
    assert!(
        out.status.success(),
        "the no-packs.yaml bare compose must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    assert_eq!(
        stdout, NO_OVERRIDE_ROUTER_GOLDEN,
        "the composite-of-one compose must stay byte-identical to the single-pack golden",
    );
    // The router is `creates-task: false`: the floor mints nothing.
    assert!(
        !repo.path().join(".jigc").join("tasks").exists(),
        "the cascade-default router mints nothing — no .jigc/tasks/ dir may appear on the floor",
    );
}

#[test]
fn two_pack_load_unions_both_packs_non_colliding_workflows() {
    // (two-pack load) methodology listed OVER the embedded dev base. Both packs'
    // non-colliding workflows must resolve through the union read: `jigc describe`
    // narrates methodology's `dev-task` AND dev's `single-task` (and `router`). The
    // describe projection enumerates the *unfiltered* pack workflow set
    // (`introspection.md` → enumeration is over the unfiltered set), so the union is
    // directly observable on its emitted prose.
    let repo = TempDir::new("union");
    init_repo(repo.path());
    let home = TempDir::new("home");
    write_packs_yaml(repo.path(), &methodology_pack_tree());

    let out = run(repo.path(), home.path(), &["describe"]);
    assert!(
        out.status.success(),
        "`jigc describe` over the two-pack set must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    // methodology's workflow (the listed, highest-precedence pack) ...
    assert!(
        stdout.contains("dev-task"),
        "describe must narrate methodology's `dev-task` (the listed pack's workflow); got:\n{stdout}",
    );
    // ... and the base (embedded dev) pack's workflows — the union, base last.
    assert!(
        stdout.contains("single-task"),
        "describe must still narrate dev's `single-task` (the base pack's workflow); got:\n{stdout}",
    );
    assert!(
        stdout.contains("router"),
        "describe must still narrate dev's `router` (the base pack's workflow); got:\n{stdout}",
    );
}

#[test]
fn default_workflow_knob_resolves_to_the_precedence_winner() {
    // (knob winner) `default-workflow` collides: methodology declares `[dev-task]`,
    // dev declares `[router, …]` (disjoint enums). Methodology wins the whole-
    // `knobs.yaml` shadow, so the resolved default is `dev-task` (`creates-task: true`).
    // The proof is behavioral on the emitted bytes: bare `jigc start "<intent>"` mints
    // under `.jigc/tasks/<slug>/` — dev's router (the base's default) is
    // `creates-task: false` and would mint NOTHING, so the minted working area can only
    // mean the knob resolved to methodology's value.
    let repo = TempDir::new("knob-winner");
    init_repo(repo.path());
    let home = TempDir::new("home");
    write_packs_yaml(repo.path(), &methodology_pack_tree());

    let out = run(repo.path(), home.path(), &["start", "add rate limiter"]);
    assert!(
        out.status.success(),
        "bare `jigc start` over the two-pack set must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );

    // methodology's `dev-task` is `creates-task: true`, so the bare compose minted a
    // working area + base pin under the intent slug — the knob winner is `dev-task`.
    let task_dir = repo
        .path()
        .join(".jigc")
        .join("tasks")
        .join("add-rate-limiter");
    assert!(
        task_dir.join("base.json").is_file(),
        "the resolved `default-workflow` must be methodology's `dev-task` (creates-task: true), \
         which mints .jigc/tasks/add-rate-limiter/ — dev's router default would mint nothing",
    );
}

#[test]
fn commit_doctype_resolves_to_the_precedence_winner() {
    // (doctype winner) the `commit` schema collides: dev's carries an
    // `implements→spec` ref field in its header; methodology's dropped it (the M12
    // subsume). Methodology wins the whole-schema shadow, so the resolved `commit` is
    // methodology's — which has NO `implements` field. The proof is behavioral on the
    // emitted bytes: `jigc doc set-field commit:<id>#implements` addresses a field the
    // resolved schema lacks and blocks non-zero ("no field addressed"). A control
    // `#type` set-field (present in BOTH schemas) succeeds, so the block is the
    // dropped field, not a broken commit doc.
    let repo = TempDir::new("doctype-winner");
    init_repo(repo.path());
    let home = TempDir::new("home");
    write_packs_yaml(repo.path(), &methodology_pack_tree());

    // Mint the task (methodology's `dev-task` is the resolved default) — this
    // provisions the `commit:<id>` doc the set-field verbs address.
    let started = run(repo.path(), home.path(), &["start", "add rate limiter"]);
    assert!(
        started.status.success(),
        "the bare compose must mint the task; got {:?}\nstderr:\n{}",
        started.status,
        String::from_utf8_lossy(&started.stderr),
    );

    // Control: `#type` is in BOTH packs' commit schemas, so it sets cleanly — the
    // commit doc + the resolved schema are sound.
    let ok = run(
        repo.path(),
        home.path(),
        &[
            "doc",
            "set-field",
            "commit:add-rate-limiter#type",
            "--value",
            "feat",
        ],
    );
    assert!(
        ok.status.success(),
        "`set-field commit#type --value feat` must succeed against the resolved commit schema; got {:?}\nstderr:\n{}",
        ok.status,
        String::from_utf8_lossy(&ok.stderr),
    );

    // The discriminator: `#implements` exists only on DEV's commit schema. Methodology
    // (the precedence winner) dropped it, so addressing it blocks non-zero.
    let blocked = run(
        repo.path(),
        home.path(),
        &[
            "doc",
            "set-field",
            "commit:add-rate-limiter#implements",
            "--value",
            "spec:foo",
        ],
    );
    assert!(
        !blocked.status.success(),
        "the resolved (methodology) commit schema dropped `implements`, so set-field on it must block non-zero; got {:?}",
        blocked.status,
    );
    let stderr = String::from_utf8_lossy(&blocked.stderr);
    assert!(
        stderr.contains("implements"),
        "the block must name the unaddressable field (proving methodology's commit won the collision); got:\n{stderr}",
    );
}

#[test]
fn multi_pack_pack_header_names_both_packs_highest_first() {
    // (provenance) the multi-pack `Pack:` orientation header (T4) names the composed
    // pack-set highest-precedence first, joined by ` | `: methodology (listed) then dev
    // (base). Bare `jigc start` (no intent) is the read-only orientation path that emits
    // it. On the floor (one pack) the header degrades to today's single segment — proven
    // by the `floor` test composing byte-identically — so the bar here is only the
    // two-pack ordering.
    let repo = TempDir::new("provenance");
    init_repo(repo.path());
    let home = TempDir::new("home");
    write_packs_yaml(repo.path(), &methodology_pack_tree());

    let out = run(repo.path(), home.path(), &["start"]);
    assert!(
        out.status.success(),
        "bare `jigc start` (orientation) over the two-pack set must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    // The `Pack:` header lists methodology (highest) then dev (base), ` | `-joined —
    // the composed-set provenance, highest-first. The version tails come from each
    // pack's own identity (methodology's defaults.yaml `version: 0.1.0`; dev's binary
    // version). Asserted on the literal ordered segment, not reconstructed.
    assert!(
        stdout.contains("Pack: methodology/0.1.0 | dev/"),
        "the orientation header must name the composed pack-set highest-first (methodology | dev); got:\n{stdout}",
    );
}

// ---------------------------------------------------------------------------
// Increment 2 — the correctness axis (flow 17 assertion 3): body-references are
// pack-local. A definition's `{{include: step:X}}` and `{{cli.X}}` resolve in the
// pack that DEFINES the definition, independent of which pack wins a top-level id.
// Both packs ship a divergent `step:implement` and NON-superset command catalogs
// (methodology: test-first implement + `set-commit-*`, no `create-adr`; dev:
// direct-edit implement + `create-adr`), so co-composing them forces the real
// overlap. Asserted on the EMITTED bytes verbatim (hardening #4) — a reconstruction
// would mask the exact corruption (dev's implement injected into methodology's
// workflow, or dev's `single-task` stripped of `create-adr`) the rule exists to kill.

/// (loser-pack-workflow) Methodology is the precedence winner (its `dev-task` is the
/// resolved `default-workflow`, its whole `commit` schema + `knobs.yaml` shadow dev's).
/// Yet `--workflow single-task` composes DEV's `single-task`, and its body-references
/// resolve in DEV's own pack: dev's direct-edit `step:implement` (NOT methodology's
/// test-first one), and `{{cli.set-commit-summary}}`/`{{cli.create-adr}}` — `create-adr`
/// exists ONLY in dev's catalog (methodology's lacks it). Asserted byte-identical to the
/// dev single-pack compose: co-composition leaves the loser pack's workflow untouched.
///
/// The view opens with the frontend's `task minted:` header (M42) — this compose mints
/// `add-a-thing` (`workflow-dialect.md` → The `task minted:` header).
const DEV_SINGLE_TASK_UNDER_METHODOLOGY_GOLDEN: &str = "\
task minted: add-a-thing

Reason about the change. The intent is:
add a thing

The relevant code paths are not yet known. Inspect the codebase to confirm
scope before implementing.

Implement the change directly in the working tree. `git add` your code edits
before finalize — it commits only what you have staged.

If a decision is warranted, create an ADR and author its slots — a line per slot
usually suffices; an ADR earns its keep by capturing the *why*, not by running
long:

Run: `jigc doc create adr --title <TITLE> --task add-a-thing`

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

jigc doc set-slot adr:<slug>#context --from-file - --task add-a-thing <<'EOF'
<the forces that made this decision necessary>
EOF

jigc doc set-slot adr:<slug>#context --from-file - --task add-a-thing
jigc doc set-slot adr:<slug>#decision --from-file - --task add-a-thing
jigc doc set-slot adr:<slug>#consequences --from-file - --task add-a-thing

The `options` slot is optional — fill it only when alternatives were genuinely
weighed. Its `## Options` heading renders either way; an empty optional slot is
conformant and never blocks finalize:

jigc doc set-slot adr:<slug>#options --from-file - --task add-a-thing

Before you finalize, verify the change actually works: build it and run the
tests, and confirm the behaviour you set out to produce. Finalize commits your
staged work; it does not check that the work is correct.

`<slug>` is the slug the title minted, and this task's own index names it back —
every doc the task stages, each by the `<type>:<slug>` identity the read takes:

jigc doc list adr --task add-a-thing

Read your write back before you move on — with `--task` the read serves THIS
task's staged copy, the write you just made, which the committed store does not
carry yet:

jigc doc show adr:<slug> --task add-a-thing

If the change is user-facing — a feature, a fix, or a behaviour a user would
notice — record it on the changelog. Create-or-update the singleton first:

Run: `jigc doc create changelog --title Changelog --task add-a-thing`

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

jigc doc author changelog --from-file - --task add-a-thing <<'EOF'
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

jigc doc show changelog:changelog --task add-a-thing

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

jigc doc set-slot commit:add-a-thing#summary --from-file - --task add-a-thing <<'EOF'
<one line saying what changed>
EOF

Set the type, then stage the summary prose:

Run: `jigc doc set-field commit:add-a-thing#type --value <COMMIT_TYPE> --task add-a-thing`
The `type` value is one of: feat | fix | docs | style | refactor | perf | test | build | ci | chore | revert
Run: `jigc doc set-slot commit:add-a-thing#summary --from-file - --task add-a-thing`
<<author: commit:add-a-thing#summary>>

The `commit` schema is the authority on what you write into it — its required
slots and fields, each field's enum members, and every address a write can take:

jigc doc schema commit

The `scope` and `body` are optional: add a `scope` to name the area touched, or
author a `body` to explain the motivation, only when they earn their place —

jigc doc set-field commit:add-a-thing#scope --value <area> --task add-a-thing
jigc doc set-slot commit:add-a-thing#body --from-file - --task add-a-thing

When the work shares authorship — a co-author, or an agent that wrote it — record
it in a commit trailer. Add one trailer item, then set its value on the address
`add-item` prints:

jigc doc add-item commit:add-a-thing#trailers --title Co-Authored-By --task add-a-thing
jigc doc set-field commit:add-a-thing#trailers/<id>/value --value \"Name <email>\" --task add-a-thing

Read your write back before you move on — with `--task` the read serves THIS
task's staged copy, the write you just made, which the committed store does not
carry yet:

jigc doc show commit:add-a-thing --task add-a-thing

Validate and commit the task as one logical commit. Finalize commits only the
staged set plus the docs it manages; unstaged edits and untracked files are left
out, and with nothing staged over a dirty tree it refuses. Anything still staged
from BEFORE this task was minted makes finalize refuse too (one blocking finding
per carried path): unstage it, or pass `--carry-staged` to declare the carryover
deliberate.

To see what's left before committing, run `jigc task validate add-a-thing` — it
previews part of what finalize gates on (the repository posture finalize refuses
under, this task's content findings, the carryover gate, the owner-artifact causes
that need no staging, and the granted-but-unused changelog gate), without
committing anything; the staged set, promotion and the commit itself are decided
at finalize.

Run: `jigc task finalize add-a-thing`
resume: `jigc start --task add-a-thing`   — re-composes this workflow if context is lost
what's-left: `jigc task validate add-a-thing`   — previews part of the finalize gate: the repository posture finalize refuses under, this task's content findings, the carryover gate, the owner-artifact causes that need no staging, and the granted-but-unused changelog gate; the staged set, promotion and the commit surface at finalize
task scope: `jigc doc` writes default to the single active task; `--task add-a-thing` is the explicit override and wins when several are active — several open tasks are legal, each addressed by its own `--task`, so you can run them in parallel while their work stays disjoint; once a sibling task commits a path this one also touches, resuming or finalizing here blocks and names the overlapping paths
create-gates: adr, changelog   — the doc-types this task is allowed to create; any other type is refused
— jigc · run `jigc start` for orientation; all writes through `jigc`.
";

/// (winner-pack-workflow) Bare `jigc start "<intent>"` composes methodology's `dev-task`
/// (the resolved `default-workflow`). Its body-references resolve in METHODOLOGY's own
/// pack: methodology's test-first `step:implement`, and the `set-commit-type`/`-scope`/
/// `-summary`/`-body` command-refs — methodology's `set-field`/`set-slot` verb split, NOT
/// dev's single `set-commit-summary`. Asserted byte-identical to methodology composing
/// alone: the winner pack's workflow is unperturbed by the co-composed dev base below it.
///
/// The view opens with the frontend's `task minted:` header (M42) — this compose mints
/// `add-rate-limiter` (`workflow-dialect.md` → The `task minted:` header).
const METHODOLOGY_DEV_TASK_GOLDEN: &str = "\
task minted: add-rate-limiter

Scope the task before touching code. The intent is:

add rate limiter

Restate that intent in your own words, then name the single observable
done-criterion — a test, a command that exits cleanly, or a behaviour you can
point at. If your restatement reveals a different problem than the intent
asked for, stop and check with the human before proceeding: a clarifying
question costs less than solving the wrong problem.

Implement the change test-first. Write the failing test first and confirm it
fails for the right reason — the assertion you care about, not an incidental
compile error standing in for it. Only then write the minimal implementation
that makes it pass. Refactor while green, touching only what this task needs.

`git add` your code edits as you work — test and implementation both. finalize
commits only what you have staged, so anything you leave unstaged is silently
left out of the commit.

This ordering is yours to police: nothing here enforces that the test was
observed failing before the implementation. Hold the discipline yourself.

Run your project's own test, lint, and build gate — the commands this project
already uses to prove a change is sound — and confirm every one passes before
you finalize. Use whatever the project's configured gate is; do not assume a
particular toolchain. A green gate is what separates a finished change from one
that merely compiles in your head.

finalize renders the commit doc; it does not fill it, so set its header and prose
first. Inside slot prose, the reserved heading depths are schema-relative to the
address you write — the CLI owns the section, item, and sub-label heading levels
there, so your headings sit below them; Setext headings are rejected at every
depth, and a rejected write names the shallowest depth free at that address.

Set the Conventional-Commits type:

Run: `jigc doc set-field commit:add-rate-limiter#type --value <TYPE> --task add-rate-limiter`
The `type` value is one of: feat | fix | docs | style | refactor | perf | test | build | ci | chore | revert

The `commit` schema is the authority on what you write into it — its required
slots and fields, each field's enum members, and every address a write can take:

jigc doc schema commit

Set the scope — the area this change touches:

Run: `jigc doc set-field commit:add-rate-limiter#scope --value <SCOPE> --task add-rate-limiter`

Set the subject line — it renders as `<type>(<scope>): <summary>`, so write the
summary without a type or scope prefix of its own (the `type` field already
carries it):

Every `--from-file -` below reads its payload from stdin — attach it as a heredoc
directly to the `jigc` command, never as a `cat payload | jigc …` pipeline, which
jigc itself accepts but an agent harness that statically analyses shell commands
can refuse to run:

jigc doc set-slot commit:add-rate-limiter#summary --from-file - --task add-rate-limiter <<'EOF'
<one line saying what changed>
EOF

Run: `jigc doc set-slot commit:add-rate-limiter#summary --from-file - --task add-rate-limiter`

Set the body — why this change:

Run: `jigc doc set-slot commit:add-rate-limiter#body --from-file - --task add-rate-limiter`

Read your write back before you move on — with `--task` the read serves THIS
task's staged copy, the write you just made, which the committed store does not
carry yet:

jigc doc show commit:add-rate-limiter --task add-rate-limiter

Land the change as exactly one logical commit. finalize commits the git index —
only what you have staged (`git add`), plus the docs it manages. Unstaged edits
and untracked files are left out of the commit; with nothing staged over a dirty
tree, finalize refuses. Anything still staged from BEFORE this task was minted
makes finalize refuse too (one blocking finding per carried path): unstage it, or
pass `--carry-staged` to declare the carryover deliberate.

To see what's left before committing, run `jigc task validate add-rate-limiter` — it
previews part of what finalize gates on (the repository posture finalize refuses
under, this task's content findings, the carryover gate, the owner-artifact causes
that need no staging, and the granted-but-unused changelog gate), without
committing anything; the staged set, promotion and the commit itself are decided
at finalize.

Then validate and commit:

Run: `jigc task finalize add-rate-limiter`
resume: `jigc start --task add-rate-limiter`   — re-composes this workflow if context is lost
what's-left: `jigc task validate add-rate-limiter`   — previews part of the finalize gate: the repository posture finalize refuses under, this task's content findings, the carryover gate, the owner-artifact causes that need no staging, and the granted-but-unused changelog gate; the staged set, promotion and the commit surface at finalize
task scope: `jigc doc` writes default to the single active task; `--task add-rate-limiter` is the explicit override and wins when several are active — several open tasks are legal, each addressed by its own `--task`, so you can run them in parallel while their work stays disjoint; once a sibling task commits a path this one also touches, resuming or finalizing here blocks and names the overlapping paths
— jigc · run `jigc start` for orientation; all writes through `jigc`.
";

#[test]
fn pack_local_loser_pack_workflow_resolves_body_refs_in_its_own_pack() {
    // The loser-pack-workflow correctness proof. Methodology wins every top-level
    // collision (default-workflow knob, commit schema), yet `--workflow single-task`
    // composes DEV's single-task with DEV's body-references — its direct-edit
    // `step:implement` and `{{cli.create-adr}}` (a command-ref methodology's catalog
    // LACKS). The emitted bytes must equal the dev single-pack compose exactly: if
    // includes/command-refs resolved by precedence instead of pack-of-origin, dev's
    // single-task would get methodology's test-first implement and would fail to
    // resolve `create-adr` — the M3-class silent corruption this rule kills.
    let repo = TempDir::new("loser-body");
    init_repo(repo.path());
    let home = TempDir::new("home");
    write_packs_yaml(repo.path(), &methodology_pack_tree());

    let out = run(
        repo.path(),
        home.path(),
        &["start", "--workflow", "single-task", "add a thing"],
    );
    assert!(
        out.status.success(),
        "`jigc start --workflow single-task` over the two-pack set must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    assert_eq!(
        stdout, DEV_SINGLE_TASK_UNDER_METHODOLOGY_GOLDEN,
        "dev's single-task must compose with DEV's pack-local implement + command-refs \
         (incl. create-adr, which methodology lacks) even though methodology wins precedence",
    );
}

#[test]
fn pack_local_winner_pack_workflow_resolves_body_refs_in_its_own_pack() {
    // The winner-pack-workflow correctness proof. Bare `jigc start "<intent>"` composes
    // methodology's `dev-task` (the resolved default), and its body-references resolve in
    // METHODOLOGY's own pack: its test-first `step:implement` and its
    // `set-commit-type`/`-scope`/`-summary`/`-body` command-refs — NOT dev's direct-edit
    // implement or dev's single `set-commit-summary` verb. The emitted bytes must equal
    // methodology composing alone: the co-composed dev base below it changes nothing.
    let repo = TempDir::new("winner-body");
    init_repo(repo.path());
    let home = TempDir::new("home");
    write_packs_yaml(repo.path(), &methodology_pack_tree());

    let out = run(repo.path(), home.path(), &["start", "add rate limiter"]);
    assert!(
        out.status.success(),
        "bare `jigc start` over the two-pack set must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    assert_eq!(
        stdout, METHODOLOGY_DEV_TASK_GOLDEN,
        "methodology's dev-task must compose with METHODOLOGY's pack-local test-first implement \
         + set-commit-type/scope/summary/body command-refs, not dev's direct-edit implement",
    );
}

// ---------------------------------------------------------------------------
// Increment 3 — the consolidated flow-17 acceptance (the headline). Over ONE
// methodology-primary throwaway repo (the inc-1/inc-2 scaffolding — `init_repo`,
// `write_packs_yaml`, `methodology_pack_tree`, `run`) driven through the built
// binary, all SIX flow-17 acceptance-bar assertions on the EMITTED bytes
// (hardening #4), each forcing genuine cross-pack overlap (hardening #7).
// Assertions 1/2/3 re-assert the inc-1/inc-2 surfaces IN the consolidated walk;
// 4/5/6 are this increment's net-new acceptance (`design/worked-examples.md`
// → flow 17 → the six-part bar; `design/multi-pack.md` → Provenance).

/// Stand up a methodology-primary two-pack repo: a real git repo with the project
/// layer, `packs.yaml` listing `crates/cli/packs/methodology` (highest) over the embedded dev
/// base. The shared fixture every flow-17 assertion composes against — the genuine
/// dev × methodology overlap (`commit`, `default-workflow`, `step:implement` all
/// owned by BOTH packs).
fn methodology_primary_repo(tag: &str) -> (TempDir, TempDir) {
    let repo = TempDir::new(tag);
    init_repo(repo.path());
    let home = TempDir::new("home");
    write_packs_yaml(repo.path(), &methodology_pack_tree());
    (repo, home)
}

/// A 64-char lowercase-hex blake3 digest (the `--explain` `Pack input:` content-hash
/// form) — asserted as a SHAPE (not a pinned value: the methodology pack bytes / the
/// binary version mutate), so the test stays honest about *what* the hash is without
/// brittling on the exact digest.
fn is_blake3_hex(s: &str) -> bool {
    s.len() == 64
        && s.chars()
            .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase())
}

/// Flow 17, the consolidated six-part acceptance bar, driven end-to-end through the
/// built `jigc` binary over the real dev × methodology pair. Each assertion runs on
/// the EMITTED bytes the agent would see (hardening #4); the fixture forces genuine
/// cross-pack overlap (both packs own `commit`/`default-workflow`/`step:implement`).
#[test]
fn flow17_consolidated_acceptance_over_the_real_dev_x_methodology_pair() {
    // ===================================================================
    // Assertion 1 — the single-pack floor stays byte-identical (re-asserted in the
    // consolidated walk). With NO `packs.yaml` the pack-set is `[base]`, and bare
    // `jigc start "<intent>"` composes the cascade-default router byte-identical to
    // the single-pack golden — the composite-of-one adds zero observable change.
    {
        let repo = TempDir::new("flow17-floor");
        init_repo(repo.path());
        let home = TempDir::new("home");
        let out = run(repo.path(), home.path(), &["start", "add rate limiter"]);
        assert!(
            out.status.success(),
            "(1) the no-packs.yaml floor compose must exit 0; stderr:\n{}",
            String::from_utf8_lossy(&out.stderr),
        );
        let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
        assert_eq!(
            stdout, NO_OVERRIDE_ROUTER_GOLDEN,
            "(1) the composite-of-one floor must stay byte-identical to the single-pack golden",
        );
    }

    // ===================================================================
    // Assertion 2 — both top-level cross-pack collisions resolve deterministically
    // and the winners are visible in `--explain`: `default-workflow` → dev-task
    // (methodology's whole `knobs.yaml` wins the precedence shadow), and the `commit`
    // doctype → methodology's. Asserted on the literal collision-winner lines.
    let (repo, home) = methodology_primary_repo("flow17-explain");
    let explain = run(
        repo.path(),
        home.path(),
        &["start", "--explain", "add rate limiter"],
    );
    assert!(
        explain.status.success(),
        "(2) `jigc start --explain` over the two-pack set must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&explain.stderr),
    );
    let explain_out = String::from_utf8(explain.stdout).expect("utf-8 stdout");
    // The whole `knobs.yaml` resolves to methodology's (hence `default-workflow` →
    // `dev-task`) — named by the resource actually adjudicated.
    assert!(
        explain_out.contains("collision: config:knobs → won by methodology/0.1.0"),
        "(2) `--explain` must name methodology as the `config/knobs` collision winner; got:\n{explain_out}",
    );
    // The `commit` doctype resolves to methodology's — named as the winner.
    assert!(
        explain_out.contains("collision: doctype:commit → won by methodology/0.1.0"),
        "(2) `--explain` must name methodology as the `commit` doctype collision winner; got:\n{explain_out}",
    );

    // ===================================================================
    // Assertion 3 — body-references are pack-local (re-asserted in the consolidated
    // walk). Methodology wins every top-level collision, yet each composed workflow
    // resolves its `{{include:}}` / `{{cli.X}}` in its OWN pack: dev's `single-task`
    // keeps DEV's direct-edit implement + `{{cli.create-adr}}` (a command-ref
    // methodology lacks), and methodology's `dev-task` keeps METHODOLOGY's test-first
    // implement. Asserted byte-identical to each pack's pack-local golden.
    let dev_st = run(
        repo.path(),
        home.path(),
        &["start", "--workflow", "single-task", "add a thing"],
    );
    assert!(
        dev_st.status.success(),
        "(3) dev single-task compose must exit 0"
    );
    let dev_st_out = String::from_utf8(dev_st.stdout).expect("utf-8 stdout");
    assert_eq!(
        dev_st_out, DEV_SINGLE_TASK_UNDER_METHODOLOGY_GOLDEN,
        "(3) dev's single-task must keep DEV's pack-local implement + create-adr under methodology-primary",
    );
    let meth_dt = run(repo.path(), home.path(), &["start", "add rate limiter"]);
    assert!(
        meth_dt.status.success(),
        "(3) methodology dev-task compose must exit 0"
    );
    let meth_dt_out = String::from_utf8(meth_dt.stdout).expect("utf-8 stdout");
    // This compose runs with the `single-task` mint above still open, so it carries one
    // frontend block the pack-local golden (captured over a fresh repo, and asserted there
    // unchanged) does not: M50 Increment 5 / T3's `also open:` block. It is **pinned by
    // content** — it must name that task and the workflow that minted it — and only then
    // subtracted, so this stays a whole-output comparison with a stated exception rather
    // than a hole. The block is presentation appended after the composed text; the
    // body-reference resolution this assertion is about is untouched by it.
    let also_open = [
        "also open: 1 other task was already open before this call — nothing here touched it; several open tasks are legal, each addressed by its own `--task`:".to_owned(),
        "  - `add-a-thing` (workflow `single-task`) — resume it with `jigc start --task add-a-thing`".to_owned(),
        String::new(),
    ]
    .join("\n");
    assert!(
        meth_dt_out.contains(&also_open),
        "(3) the second compose names the task the first one minted; got:\n{meth_dt_out}",
    );
    assert_eq!(
        meth_dt_out.replace(&also_open, ""),
        METHODOLOGY_DEV_TASK_GOLDEN,
        "(3) methodology's dev-task must keep METHODOLOGY's pack-local test-first implement",
    );

    // ===================================================================
    // Assertion 4 — dev's doctypes don't just LIST, they LOAD + VALIDATE. The
    // integration witness for inc-2 T4's schema-pack-local field-type mechanism over
    // the REAL pair: a dev workflow composes (its steps + `{{cli.X}}` pack-local), a
    // dev doctype is `create`d, and its `code-anchor` field is `set-field`-set. The
    // field's TYPE resolves from DEV's own `field-types.yaml` though methodology (the
    // precedence winner) ships NONE — verified asserted on the VALIDATING outcome:
    //   - a WELL-FORMED `path#symbol` succeeds (the type resolved + the value passed
    //     the `code-anchor` write-time adjudicator), AND
    //   - a MALFORMED `path#a#b` value (two `#`) blocks with the `code-anchor`-specific
    //     "is not a code-anchor" message — which fires ONLY if the resolved
    //     `code-anchor` adjudicator RAN (a generic opaque check accepts two-`#`).
    // The malformed-block discriminator is the validating-outcome proof, NOT a
    // `describe | grep` (explicitly rejected as masking, worked-examples.md:1097).
    //
    // 4a — the `adr` header-field `code-anchor` (`status/cites-code`), authored via
    // dev's `single-task` create-gate (`allows-create: [{type: adr}]`). This field
    // routes through `set_field_validated` → the `code-anchor` write-time adjudicator.
    let started = run(
        repo.path(),
        home.path(),
        &["start", "--workflow", "single-task", "land the limiter"],
    );
    assert!(
        started.status.success(),
        "(4a) `jigc start --workflow single-task` must mint the task; stderr:\n{}",
        String::from_utf8_lossy(&started.stderr),
    );
    let adr_task = "land-the-limiter";
    let created = run(
        repo.path(),
        home.path(),
        &[
            "doc",
            "create",
            "adr",
            "--title",
            "Token bucket",
            "--task",
            adr_task,
        ],
    );
    assert!(
        created.status.success(),
        "(4a) `jigc doc create adr` (a dev doctype) must mint under methodology-primary; stderr:\n{}",
        String::from_utf8_lossy(&created.stderr),
    );
    let adr_addr = String::from_utf8(created.stdout)
        .expect("utf-8 stdout")
        .trim()
        .to_owned();
    assert_eq!(adr_addr, "adr:token-bucket", "(4a) the created adr address");
    // The well-formed `code-anchor` value resolves AND validates — set-field succeeds.
    let cites_field = format!("{adr_addr}#status/cites-code");
    let good = run(
        repo.path(),
        home.path(),
        &[
            "doc",
            "set-field",
            &cites_field,
            "--value",
            "src/limiter.rs#refill",
            "--task",
            adr_task,
        ],
    );
    assert!(
        good.status.success(),
        "(4a) set-field on a well-formed `code-anchor` must succeed — dev's `code-anchor` \
         field-type resolved (methodology, the precedence winner, ships none); stderr:\n{}",
        String::from_utf8_lossy(&good.stderr),
    );
    // The discriminating witness: a two-`#` value passes the generic opaque floor but
    // is rejected by the `code-anchor` adjudicator — so the block PROVES the resolved
    // `code-anchor` type's adjudicator actually ran (not an opaque fallback).
    let bad = run(
        repo.path(),
        home.path(),
        &[
            "doc",
            "set-field",
            &cites_field,
            "--value",
            "src/limiter.rs#a#b",
            "--task",
            adr_task,
        ],
    );
    assert!(
        !bad.status.success(),
        "(4a) a malformed (two-`#`) `code-anchor` value must block — the resolved \
         code-anchor adjudicator rejects it; got exit {:?}",
        bad.status,
    );
    let bad_err = String::from_utf8_lossy(&bad.stderr);
    assert!(
        bad_err.contains("is not a code-anchor"),
        "(4a) the block must carry the `code-anchor`-specific message (proving the \
         resolved dev field-type's adjudicator ran, not a generic opaque check); got:\n{bad_err}",
    );

    // 4b — the `arch-doc` repeatable-item `code-anchor` (`components/<id>/implemented-by`),
    // authored via dev's `architecture-documentation` create-gate. The item-leaf path
    // resolves its field-type through the schema load (an unresolved `code-anchor`
    // would make `task.schema()` fail "the `arch-doc` schema is malformed", blocking
    // set-field non-zero) — so the set-field SUCCESS is the validating outcome that
    // dev's `field-types.yaml` resolved the item-leaf field-type under methodology.
    let arch_started = run(
        repo.path(),
        home.path(),
        &[
            "start",
            "--workflow",
            "architecture-documentation",
            "document the index layer",
        ],
    );
    assert!(
        arch_started.status.success(),
        "(4b) `jigc start --workflow architecture-documentation` (a dev workflow) must compose + mint \
         under methodology-primary; stderr:\n{}",
        String::from_utf8_lossy(&arch_started.stderr),
    );
    let arch_task = "document-the-index-layer";
    let arch_created = run(
        repo.path(),
        home.path(),
        &[
            "doc",
            "create",
            "arch-doc",
            "--title",
            "Index layer",
            "--task",
            arch_task,
        ],
    );
    assert!(
        arch_created.status.success(),
        "(4b) `jigc doc create arch-doc` (a dev doctype) must mint; stderr:\n{}",
        String::from_utf8_lossy(&arch_created.stderr),
    );
    // Materialize a component and capture the EMITTED item address (run verbatim,
    // hardening #4 — never reconstructed).
    let added = run(
        repo.path(),
        home.path(),
        &[
            "doc",
            "add-item",
            "arch-doc:index-layer#components",
            "--title",
            "Edge index",
            "--task",
            arch_task,
        ],
    );
    assert!(
        added.status.success(),
        "(4b) `jigc doc add-item …#components` must materialize the component; stderr:\n{}",
        String::from_utf8_lossy(&added.stderr),
    );
    let item_addr = String::from_utf8(added.stdout)
        .expect("utf-8 stdout")
        .trim_end_matches('\n')
        .to_owned();
    assert_eq!(
        item_addr, "arch-doc:index-layer#components/edge-index",
        "(4b) the emitted item address is the canonical `#components/<id>` form",
    );
    // The item-leaf `code-anchor` resolves + validates: set-field on the EMITTED
    // address succeeds (the field-type came from DEV's `field-types.yaml`).
    let anchor = run(
        repo.path(),
        home.path(),
        &[
            "doc",
            "set-field",
            &format!("{item_addr}/implemented-by"),
            "--value",
            "crates/engine/src/index.rs#rebuild_committed",
            "--task",
            arch_task,
        ],
    );
    assert!(
        anchor.status.success(),
        "(4b) set-field on the arch-doc item-leaf `code-anchor` must succeed — dev's \
         `field-types.yaml` resolved the `implemented-by` field-type though methodology \
         ships none; stderr:\n{}",
        String::from_utf8_lossy(&anchor.stderr),
    );

    // ===================================================================
    // Assertion 5 — provenance names every composed pack by PATH + content (not just
    // id/version): `--explain` surfaces one `Pack input:` line per composed pack, each
    // naming its resolving directory path (methodology = the listed abs path; dev =
    // the `<embedded>` sentinel) + a blake3 content-hash. Asserted on the literal
    // emitted line (the path it actually resolved to + a real 64-hex blake3).
    let meth_path = methodology_pack_tree().display().to_string();
    let meth_input = explain_out
        .lines()
        .find(|l| l.contains("Pack input: methodology/0.1.0"))
        .unwrap_or_else(|| panic!("(5) a methodology `Pack input:` line; got:\n{explain_out}"));
    assert!(
        meth_input.contains(&format!("= {meth_path}  (blake3 ")),
        "(5) methodology's `Pack input:` must name its resolving path (the listed dir); got:\n{meth_input}",
    );
    let meth_hash = meth_input
        .rsplit("(blake3 ")
        .next()
        .and_then(|s| s.strip_suffix(')'))
        .map(str::trim)
        .unwrap_or("");
    assert!(
        is_blake3_hex(meth_hash),
        "(5) methodology's content-hash must be a 64-hex blake3 digest; got: {meth_hash:?}",
    );
    let dev_input = explain_out
        .lines()
        .find(|l| l.contains("Pack input: dev/"))
        .unwrap_or_else(|| panic!("(5) a dev `Pack input:` line; got:\n{explain_out}"));
    assert!(
        dev_input.contains("= <embedded>  (blake3 "),
        "(5) the embedded dev base's `Pack input:` must name the `<embedded>` sentinel path; got:\n{dev_input}",
    );
    let dev_hash = dev_input
        .rsplit("(blake3 ")
        .next()
        .and_then(|s| s.strip_suffix(')'))
        .map(str::trim)
        .unwrap_or("");
    assert!(
        is_blake3_hex(dev_hash),
        "(5) the dev base's content-hash must be a 64-hex blake3 digest; got: {dev_hash:?}",
    );

    // ===================================================================
    // Assertion 6 — determinism holds under composition. Two facets, both on the
    // emitted bytes:
    //   (6a) recomposing the SAME task (`jigc start --task <id>`) twice is byte-identical;
    //   (6b) a reversed pack-list ORDER over a genuinely-overlapping pack-set leaves the
    //        composed output byte-identical (re-execution under ≥2 divergent orders,
    //        hardening #7) — proven separately in `flow17_reversed_pack_order_*`.
    //
    // 6a — resume of a LOSER-PACK workflow must compose its OWN pack's body, exactly
    // as the fresh compose did. Assertion 3 above already minted dev's `single-task`
    // (the loser pack's workflow — methodology wins every top-level collision) fresh
    // via `--workflow single-task "add a thing"` and pinned its fresh bytes to
    // `DEV_SINGLE_TASK_UNDER_METHODOLOGY_GOLDEN`. Here we RESUME that same task via
    // `--task add-a-thing` and assert the resume re-resolves the pack-local
    // body-references against DEV's pack (its direct-edit `implement` +
    // `{{cli.create-adr}}`, which methodology's catalog LACKS) — NOT the
    // precedence-winner methodology. The load-bearing assertion is resume == the fresh
    // golden (crossing the mint→resume boundary on a loser-pack workflow: a resume
    // that mis-resolved to methodology's test-first `implement` / dropped `create-adr`
    // would diverge from the golden); recompose-determinism (resume twice
    // byte-identical) rides along.
    let det_task = "add-a-thing";
    let first = run(repo.path(), home.path(), &["start", "--task", det_task]);
    assert!(
        first.status.success(),
        "(6a) `jigc start --task <id>` resume of a loser-pack workflow must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&first.stderr),
    );
    let first_out = String::from_utf8(first.stdout).expect("utf-8");
    // The fresh golden opens with the `task minted: <id>` header the frontend appends on a
    // *mint* (M42); a resume mints nothing and announces nothing (`workflow-dialect.md` →
    // The `task minted:` header). The body-resolution claim is about the composed *view*:
    // the golden below its header is exactly what the resume must emit.
    let fresh_view = DEV_SINGLE_TASK_UNDER_METHODOLOGY_GOLDEN
        .strip_prefix(&format!("task minted: {det_task}\n\n"))
        .expect("the fresh golden opens with the mint header");
    assert_eq!(
        first_out, fresh_view,
        "(6a) resume of a loser-pack workflow must compose DEV's OWN-pack body \
         (direct-edit implement + create-adr), identical to the fresh golden (below its \
         mint header) — NOT methodology's (the precedence winner): mint == resume on \
         body-reference resolution",
    );
    let second = run(repo.path(), home.path(), &["start", "--task", det_task]);
    assert!(
        second.status.success(),
        "(6a) the second resume must exit 0"
    );
    assert_eq!(
        first_out,
        String::from_utf8(second.stdout).expect("utf-8"),
        "(6a) resuming the same task twice must be byte-identical under composition",
    );
}

/// Assertion 6b (hardening #7) — composed output is invariant under a reversed
/// pack-list ORDER over a genuinely-overlapping pack-set. The fixture forces real
/// overlap: TWO content-identical copies of the methodology pack (so they collide on
/// EVERY id — `commit`, `default-workflow`, every step), listed in BOTH orders. The
/// dedup-then-sort union + the precedence-winner read are pure functions of the
/// pack-set's CONTENTS, so reversing the listing order cannot change the composed
/// bytes — the order-invariance hardening #7 mandates (≥2 divergent orders →
/// byte-identical output, over input with forced overlap, not a disjoint set).
#[test]
fn flow17_reversed_pack_order_composes_byte_identical() {
    // Two content-identical methodology copies in self-cleaning temp dirs.
    let copy_a = TempDir::new("meth-copy-a");
    let copy_b = TempDir::new("meth-copy-b");
    let pack_a = copy_a.path().join("methodology");
    let pack_b = copy_b.path().join("methodology");
    copy_tree(&methodology_pack_tree(), &pack_a);
    copy_tree(&methodology_pack_tree(), &pack_b);

    // Compose a bare `jigc start "<intent>"` over the pack-set listed in the given
    // order, returning the emitted stdout. Each call uses a fresh repo so the only
    // varying input is the listing order.
    let compose = |first: &Path, second: &Path| -> String {
        let repo = TempDir::new("rev-order");
        init_repo(repo.path());
        let home = TempDir::new("home");
        fs::write(
            repo.path().join(".jigc").join("config").join("packs.yaml"),
            format!(
                "packs:\n  - {}\n  - {}\n",
                first.display(),
                second.display()
            ),
        )
        .expect("write the two-entry packs.yaml");
        let out = run(repo.path(), home.path(), &["start", "add rate limiter"]);
        assert!(
            out.status.success(),
            "the reversed-order compose must exit 0; stderr:\n{}",
            String::from_utf8_lossy(&out.stderr),
        );
        String::from_utf8(out.stdout).expect("utf-8 stdout")
    };

    let forward = compose(&pack_a, &pack_b);
    let reversed = compose(&pack_b, &pack_a);
    assert_eq!(
        forward, reversed,
        "(6b) composing over a genuinely-overlapping pack-set must be byte-identical \
         under a reversed pack-list order (hardening #7: ≥2 divergent orders)",
    );
}

/// Recursively copy `src` into `dst` (files + sub-dirs) — used to mint two
/// content-identical methodology copies for the reversed-order order-invariance
/// fixture. Kept test-local (no external crate).
fn copy_tree(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).expect("create copy dir");
    for entry in fs::read_dir(src).expect("read source dir") {
        let entry = entry.expect("dir entry");
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if from.is_dir() {
            copy_tree(&from, &to);
        } else {
            fs::copy(&from, &to).expect("copy file");
        }
    }
}
