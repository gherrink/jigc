<!-- M51 baseline · companion 1 of 4 — caller-supplied tokens that become filesystem paths or git arguments. Driven 2026-09-10 by one Opus capability-auditor against the release binary `1.0.0-rc.14` at HEAD `bd348a83`; no cargo run, no repo file edited. Verbatim as returned; consolidated in [baseline-ledger.md](baseline-ledger.md). -->

# M51 baseline — area: caller-supplied tokens that become filesystem paths or git arguments

Verified at **HEAD `bd348a83`**, release binary `/Users/maurice/projects/gherrink-jigc/target/release/jigc` (1.0.0-rc.14).
**A map, not gospel** — verified at this sha, nothing settled. No repo file edited, no cargo run.
Corpora from `dev/jigc-rig <state> --binary …` (two-step eval; roots under the session scratchpad).

---

## 1. The registries at HEAD

| Registry | Home | Classifies | Fence (test → what it asserts) | Explicit exemption + stated premise |
|---|---|---|---|---|
| `ArgToken` / `ARG_TOKENS` | `crates/cli/src/cli.rs:1491` (enum), `:1552` (table) | **Every** clap argument id of every leaf verb, as one of `Doctype(Bare\|Address)` · `WorkUnitId` · `SlugOverride` · `Plain`. Total function, complement erased. | `cli.rs:2733 cli_parse::every_clap_argument_is_classified` — ⇔ in both directions against `clap_leaves_with_args()`: no tree argument unclassified, no classification naming a vanished argument, no id classified twice. | **`ArgToken::Plain`** covers `path`, `from`, `file`, `from_file`, `target` (`cli.rs:1500-1508`, `:1577-1591`). Stated premise, verbatim: *"a path argument is a path the caller **means** as one, adjudicated by the filesystem and by their own doors, not a token jigc turns into an identity behind the caller's back."* Declared bound at `:1545`: *"nothing can check `Plain` from the outside."* |
| `DOCTYPE_DOORS` | `cli.rs:1675` (17 rows) | Leaf verbs taking a doctype, with the shape (`Bare`/`Address`). | `cli.rs:2772 every_doctype_door_is_registered` — ⇔ over the tree **plus** a row's declared shape == the shape its own argument carries. | none |
| `WORK_UNIT_ID_DOORS` | `cli.rs:1738` (25 rows, each with runnable `argv` + `WORK_UNIT_ID_SLOT`) | Leaf verbs taking a task/milestone id. | `cli.rs:2845 every_work_unit_id_door_is_registered` — ⇔ over the tree; each row's argv **parses**, lands on the named leaf and delivers the sentinel through the named `arg`; every `arg` ∈ `work_unit_id_arg_ids()`. Axis driven by `crates/cli/tests/work_unit_id_axis.rs`. | `milestone create` absent — *"it MINTS its id from a title rather than taking one"* (`cli.rs:1889`). |
| `SLUG_DOORS` | `cli.rs:1978` (6 rows) | Leaf verbs taking `--slug` (a **verbatim**, never re-slugified, mint-time override). | `cli.rs:2927 every_slug_door_is_registered` — same three assertions. Axis: `crates/cli/tests/slug_override_axis.rs`. | `start`'s row is the *minting* form (`--workflow single-task`) on purpose — the bare router form makes the override inert and *"a row written that way would pass while proving nothing"* (`cli.rs:1974`). |
| `ROOT_KNOBS` | `crates/cli/src/config.rs:55` — `["docs-root","placement-root"]` | The two knobs that re-home every managed doc. | `crates/cli/tests/root_knob_rules.rs:66 every_root_knob_is_a_declared_knob` (each member is a declared pack knob) + the 16-cell and 10-cell shape loops at `:368`/`:470`. | (see §3 for the three value/home predicates it gates) |

**The gap the table makes visible:** four families have a code-side door registry **and** an axis
suite. The `Plain` path family has **neither** — no registry enumerates the path-taking doors and
no `*_axis.rs` sweeps them. `crates/cli/tests/untrackable_home_axis.rs` says so in its own header:
*"The axis is the destination, not the knob"* — the **source** side is unswept.

---

## 2. Path-or-file tokens × shape → exit / code / effect on bytes

**The full path-token inventory** (read off the clap tree; `target` is included because
`config fill`/`fork`/`replace-step`/`remove-step` turn its `#<id>` tail into a path component):

| clap arg | door(s) | becomes a path how |
|---|---|---|
| `path` | `jigc migrate <path> --as <ty>` · `jigc unmanage <path>` | `repo_root.join(path)` for the read; **recorded** for the destructive retire |
| `from` | `jigc relocate <ty> --from <home>` | source-home *filter* only |
| `file` | `jigc config insert-step … <file>` · `jigc config replace-step <target> <file>` | `fs::read(file)` (cwd-relative), then `file_stem()` → `.jigc/config/steps/<stem>.yaml` |
| `from_file` | `jigc doc set-slot` · `jigc doc author` · `jigc config fill` | `read_handoff` — `-` = stdin, else `fs::read_to_string(path)` |
| `target` | `jigc config fill/fork/replace-step/remove-step` | `<fill-id>` → `.jigc/config/fills/<id>.md`; `<step-id>` → `.jigc/config/steps/<id>.yaml` |

### 2a. `jigc migrate <path>` — **latent defect (data loss), the deepest cell in this area**

| shape | mint | recorded `source-path` | `--approve` | bytes |
|---|---|---|---|---|
| absolute outside repo | **rc=0** | the absolute host path, verbatim | **rc=0** | **external file deleted, unrecoverable** |
| `../` escape | rc=0, outside content read | lexically collapsed to a nonexistent in-repo path | rc=0 | source **not** deleted; retire is a **silent no-op** while the ack + commit claim adoption of a path that does not exist |
| in-repo symlink → outside | rc=0, target content read | `link.md` | rc=0 | symlink deleted, target intact; outside content committed |
| `.git/config` | rc=0 | `.git/config` | **rc=0** | **`.git/config` deleted** |
| `.git/HEAD` | rc=0 | `.git/HEAD` | **rc=1** | HEAD deleted mid-transaction → git calls fail → rollback restores HEAD but **leaves the index carrying staged deletions of `.jigc/.gitignore`, `.jigc/config/*`, `.jigc/version` with the files untracked on disk** (reproduced twice; a fresh rig's `git status` is empty) |
| `.jigc/AGENT.md` (workbench) | rc=0 | `.jigc/AGENT.md` | (not driven to approve) | the `config.workbench-root` rule M50 shipped for the knobs is not asked here |
| a directory | rc=1 | — | — | none. Message: `could not read the foreign 'changelog' source at <ABSOLUTE HOST PATH>` — no finding code, **and it prints the host path for a token the caller typed relative** |
| nonexistent / `-` / `""` | rc=1 | — | — | none |

Repro (absolute, driven verbatim):
```
OUT=$(mktemp -d "$SCRATCH/outside.XXXXXX"); printf '# Foreign Changelog\n\n## [1.0.0] - 2026-01-01\n\n### Added\n\n- a thing\n' > "$OUT/EXTERNAL.md"
jigc migrate "$OUT/EXTERNAL.md" --as changelog          # rc=0, task minted, file contents composed into the workflow
jigc doc author changelog --from-file - --task <id> …    # rc=0
jigc task finalize <id>                                  # rc=4 review hold
jigc task finalize <id> --approve                        # rc=0 → "finalized efcbe02 … promoted CHANGELOG.md"; EXTERNAL.md GONE
```

**Three facts beyond EC-1 as written, all driven:**

1. **The sink is not re-validated, so a door-only fix is insufficient.** Minted against a benign
   in-repo `foreign.md`, then `.jigc/tasks/<id>/source-path` overwritten with an absolute outside
   path (the working area is an ordinary mutable file): `--approve` **rc=0**, ack reads
   `finalized 64606da — docs(changelog): adopt foreign.md as a managed changelog`, `foreign.md`
   listed as `left-out` — and `<victim>/keepme.txt` (`IRREPLACEABLE`) is gone.
2. **No surface names the file `--approve` deletes.** The exit-4 review hold prints the fidelity
   diff and *"retire the foreign original"* with **no path** (`grep -c "$EXT" hold.txt` → `0`);
   the pinned `--format json` hold carries exactly `["review","rewrites","source","task"]` — no
   retire key; `jigc task validate <id>` (M47's "preview what finalize gates on") prints only the
   `file-state.staged-copy` advisory.
3. **The absolute host path lands in permanent git history.** The commit subject is
   `docs(changelog): adopt /private/tmp/claude-501/…/EXTERNAL.md as a managed changelog`
   (`crates/cli/src/start.rs:395 migration_commit_summary` interpolates the recorded string). The
   law-1 fence (`crates/cli/tests/repo_relative_paths.rs`) only counts `.display()` sites, so a
   `String`-typed absolute path is outside its subject by construction.
4. **The argument's own help text is a law-1 lie:** `<PATH>  The repo-relative path of the foreign
   document to migrate` — the door accepts absolute, `../`, symlink and `.git/` spellings.
5. **`UNSWEPT_PRODUCERS`' stated reason for `migrate.rs` is falsified.**
   `crates/cli/tests/repo_relative_paths.rs:769` records `("crates/cli/src/migrate.rs", 1, "an
   anyhow read fault on the operator's own supplied path, quoted back as given")`. Driven with
   `jigc migrate docs --as changelog`, the surface prints `…/repo/docs`, not `docs` — it is
   `repo_root.join(path).display()` (`migrate.rs:254`, `:266`), not the token as given.

**Status: latent defect / shape-limited.** Gap: `jigc migrate`'s `path` is subject to **no**
home, grammar or trackability rule at the door, and the recorded `source-path` is read raw at the
destructive sink.

### 2b. `jigc unmanage <path>` — **built + proven**

Register-only (never writes or deletes a managed file; the token is only a record key).
Driven across `../../../etc/hosts` · `/etc/hosts` · `.git/config` · `""` · `docs` · `../..`:
**all rc=0, `no-op: <token> is not managed (nothing to drop)`**, every file intact
(`/etc/hosts ok`, `.git/config ok`). The genuine path works
(`unmanage CHANGELOG.md` → `unmanaged CHANGELOG.md (changelog:changelog) …`).
*Nit only:* `unmanage ""` prints `no-op:  is not managed` — a blank subject; the `""` axis M50
swept for work-unit ids is unswept here, harmlessly.

### 2c. `jigc relocate <ty> --from <home>` — **deferred-by-design (inert on the shipped surface) + cleared by drive**

- **Inert as shipped.** All 15 shipped doctypes are manifest-governed, so the freeze-exempt branch
  refuses. Driven over `{vision, research, idea, dogfood-record, spec, adr}` × `{../../../etc, /etc,
  .git, docs/../../x, "", docs/vision.md}`: every non-empty cell is `rc=1 … is a frozen doctype —
  relocate it through the version-gated 'jigc migrate-corpus'`; `""` is `rc=1 '--from' (the prior
  home) is required`. The VERDICT's *"Unprobed, not cleared"* is now driven.
- **Cleared on a reachable pack.** `dev/jigc-rig fresh --pack-from-dev` (manifest dropped → `adr`
  freeze-exempt), an adr committed then stranded at `old-home/`: `--from` ∈ {`../../../etc`, `/etc`,
  `.git`, `old-home/../../x`, `../..`} → **rc=0, `0 moved, 0 displaced, 0 blocked`, `git status`
  empty every time**; `--from old-home` → `1 moved, old-home/cache-choice.md -> docs/decisions/cache-choice.md`.
  *Structural reason:* sources come from `orphan::committed_markdown(repo_root)` (git-enumerated,
  repo-relative), destinations from the schema home guarded by `move_doc`'s `untrackable_reason`
  (`relocate.rs:58`). `--from` can only *filter*.
- *Nit:* an escaping `--from` is an exit-0 silent `0 moved` — no advisory that the named home
  matched nothing.

### 2d. `jigc config insert-step … <file>` / `replace-step <target> <file>` — **latent defect (read escape into the repo and into the agent's composed context)**

| shape | exit | effect |
|---|---|---|
| absolute path outside the repo | **rc=0** | `config: inserted step 'pwned' into 'single-task' after 'implement'` — file content copied verbatim to `.jigc/config/steps/pwned.yaml`, an in-repo committable file |
| `../rel.yaml` (relative escape) | **rc=0** | same, `.jigc/config/steps/rel.yaml` |
| `.git/config` | **rc=0** | git's config copied to `.jigc/config/steps/config.yaml` |
| a directory | rc=1 | `could not read source step file docs: Is a directory (os error 21)` — no code, no route |
| nonexistent / `-` | rc=1 | `os error 2` — no code, no route (`-` is **not** stdin at this door) |

**And it composes.** After the two inserts, `jigc start --workflow single-task "probe compose"`
renders `Relative escape body.`, `Secret from outside.` **and `repositoryformatversion = 0` /
`bare = false`** into the step text the agent reads.

Traversal is structurally impossible in the *destination*: `file.file_stem()` (`config.rs:955`,
`:1029`) yields one component, so `../../pwned.yaml` → step id `pwned`. The escape is on the
**read** side and on what the read lands in-repo.

### 2e. `--from-file` (`doc set-slot` · `doc author` · `config fill`) — **shape-limited / by-design read, route-floor gap**

- `jigc config fill 'step:implement#extra-guidance' --from-file /etc/hosts` → **rc=0**, `/etc/hosts`
  lands in `.jigc/config/fills/extra-guidance.md` and composes at the fill point.
- `jigc doc set-slot adr:probe#context --from-file .git/config --task <id>` → **rc=0**,
  `set slot adr:probe#context (192 chars)`.
- `--from-file -` is stdin (`doc.rs:6232`). Miss shapes carry no code and no route:
  `could not read slot prose from 'adir': Is a directory (os error 21)`;
  `could not read slot prose from '': No such file or directory (os error 2)`.
- Classification: the *destination* is a declared slot/fill point and the content is visible via
  `doc show`, so the unbounded read is arguably the exemption's stated premise working as intended.
  The **route floor** is the gap: four miss shapes at three doors answer with a bare `anyhow` +
  errno.

### 2f. `target` (`config fill/fork/replace-step/remove-step`) — **built + proven**

The `<fill-id>`/`<step-id>` tail *does* become a path component, and it is fenced by a
**declared-existence** check rather than a grammar:

```
jigc config fill 'step:implement#../../../pwned' --from-file -   rc=1  blocking · config.fill-point-absent  (+ route)
jigc config fill 'step:implement#/tmp/pwned'     --from-file -   rc=1  blocking · config.fill-point-absent  (+ route)
jigc config fork  'workflow:single-task#../../x'                 rc=1  blocking · config.anchor-absent      (+ route, include list enumerated)
jigc config remove-step 'workflow:single-task#../..'             rc=1  blocking · config.anchor-absent      (+ route)
```
Each echoes the typed token verbatim. Nothing written.

### 2g. The three fenced identity families (context — these all hold)

- **`work-unit.malformed-id`** at `task discard`: `""`, `../..`, `/etc`, `../../../tmp`, `a/b`
  → all **rc=1 `blocking · work-unit.malformed-id`**, `.jigc/tasks/` intact. `unknown-task` →
  rc=1 `no task 'unknown-task' — list live tasks with 'jigc task list'`. **built + proven.**
- **`store.malformed-slug`** at the address `<slug>` head: `doc show adr:../../etc/passwd` ·
  `adr:/etc/passwd` · `adr:..` → **rc=1** with code + route; `changelog:changelog` → rc=0.
  `milestone add-from-spec <m> spec:../../../etc/passwd` → **rc=1 `store.malformed-slug`** (the
  M50 completion-audit HIGH is closed at HEAD). **built + proven.**
  *Inconsistency:* `doc show adr:` (empty slug) answers `malformed address 'adr:': empty slug` with
  **no finding code** while every malformed sibling carries `store.malformed-slug`.
- **`SLUG_DOORS` × the OS ceiling — EC-28, and it is a 3-door class, not 1.** With a 300-char slug:

| door | exit | surface |
|---|---|---|
| `jigc doc rename adr:keeper --to Y --slug <300>` | 1 | `could not move the staged 'adr:keeper' to 'adr:aaa…': File name too long (os error 63)` — **no code, no route, no `at:`** (EC-28 as reported) |
| `jigc start --workflow single-task "x" --slug <300>` | 1 | `blocking · task.working-area-io … File name too long (os error 63)`, `at: task:aaa…`, route: *"resolve the underlying I/O condition (a disk or permissions problem…)"* — **the route is false**; there is no disk or permissions problem |
| `jigc doc create adr --title X --slug <300>` | 1 | `blocking · task.working-area-io — could not provision the instance for task 'adr:aaa…'`, `at: task:adr:aaa…` (a malformed address), same false I/O route. **EC-28's claim that `doc create --slug` "refuses correctly at the create-gate" is REFUTED as written** — it fails on the same OS ceiling and calls it an I/O fault |
| `jigc doc add-item roadmap:roadmap#milestones --title Z --slug <300>` | **0** | accepted — an item id is not a filename, so no ceiling; a 300-char `{#…}` anchor lands |
| `jigc migrate foreign.md --as changelog --slug <300>` | 0 | minted; masked downstream by `changelog` being a fixed-slug singleton — **the migrate cell of this axis is unreached, not clear** |
| `jigc rename …` | (masked by `rename.in-flight` in the fixture) | — |

  State is intact in all cells; the defect is the surface. **latent defect (route floor + a
  false route).**

### 2h. `ROOT_KNOBS` value rule — **EC-27 confirmed and widened**

| `config set docs-root <v>` | exit | `config get docs-root` reads back |
|---|---|---|
| `"   "` (3 spaces) | **0** | `docs-root =      (project)` — visually identical to unset |
| `" "` | **0** | same |
| `"  x  "` | **0** | `docs-root =   x    (project)` — a directory literally named `  x  ` |
| `"-"` | **0** | `docs-root = -  (project)` — a git-ambiguous pathspec |
| `"\t"` | 1 | `blocking · config.value-rejected — … must not contain control characters` |
| `".."` | 1 | `blocking · config.untrackable-root — '..' … resolves outside the repository` |

The predicate the knob wants is *"a value that reads back as itself"*, not *"no control
characters"*. **latent defect** (recoverable — tracked — so not loss).

---

## 3. Shared predicates a path-family rule could reuse

| Predicate | Home | Answers |
|---|---|---|
| `crate::trackable::untrackable_reason(repo_root, relative) -> Option<String>` | `crates/cli/src/trackable.rs:55` | *Can git record a path here?* Five tests: under the repo root (lexical + canonicalized) · no `.git` component (case-insensitive) · outside `--git-dir`/`--git-common-dir` · owned by *this* repo (not a submodule/embedded repo) · not under an index gitlink. **Deliberately does NOT ask gitignore.** Callers: `config.rs:402` (`config.untrackable-root`), `relocate.rs:58` (`move_doc`, the one home for four movers), `rename.rs:532` (`write.untrackable-destination`), `setup.rs:1499` (`committable_hook_path`). **Never called by `migrate.rs`.** This is the ready-made predicate for `migrate`'s `path`, and it already refuses every destructive cell in §2a. |
| `config::unusable_root_reason(repo_root, value)` | `crates/cli/src/config.rs:677` | *Is this a root the store can describe?* absolute · symlinked (any existing component) · file-shaped (any existing component). Code `config.unusable-root`. **Does not ask about whitespace** (EC-27's gap). |
| `config::is_workbench_root(value)` | `crates/cli/src/config.rs:622` | *Does the (lexically normalized) value land inside `.jigc/`?* First normalized component, case-insensitive. Code `config.workbench-root`. Not asked by `migrate`. |
| `config::normalize_root_value(value)` | `crates/cli/src/config.rs:~540` | Folds `./`, applies `..`, collapses separators — so refusals adjudicate the *reached* home while quoting the typed one. |
| `engine::path::repo_relative(repo_root, path)` | `crates/engine/src/path.rs:48` | Law 1's one home; CLI door `crate::render::repo_relative` (`render.rs:2840`). Tries plain strip, then a canonicalizing retry, then a root-only retry; falls back to the absolute form for a genuinely outside path (declared honest). Fenced by `crates/cli/tests/repo_relative_paths.rs` (`GUARDED_SRC` = `cli/milestone.rs`, `cli/trackable.rs`, `engine/store.rs`, `engine/milestone.rs`) with the remainder **counted** in `UNSWEPT_PRODUCERS` (`:721`) — whose fence reads `.display()` sites only. |
| `engine::finding::is_route_exempt(code)` | `crates/engine/src/finding.rs:323` | The route floor's one-home exemption: exactly `CONFORMANCE_PARSE_DIAGNOSTICS` (11 members, M49 narrowed it to an enumeration with a per-member reason). `finalize.commit-rejected` is the other re-affirmed exemption but is an anyhow path, not a `Finding`. **Every route-floor gap in §2a/2d/2e/2g escapes the floor by being a bare `anyhow`, not by exemption** — the same escape M50 closed at the write-miss resolvers by typing them `Result<_, Finding>`. |
| `crate::task::shell_token` | `crates/cli/src/task.rs` | Shell-safe rendering for a route argv (the fence `Route::mechanical` asserts on debug builds). Relevant to the `adoption_route` deferral in §5. |

---

## 4. `migrate`'s `source-path`, mint → retire

**Call path (all at HEAD):**

1. `crates/cli/src/cli.rs:445` `Command::Migrate{path, r#as, slug}` → `run_migrate` (`:677`) → `migrate::run`.
2. `crates/cli/src/migrate.rs:219 migrate_in_repo` — validates `--as` (`ensure_migratable`) and `--slug` (`engine::slug::is_slug`) **before minting**; then `:254 let foreign_path = repo_root.join(path)` (an absolute `path` replaces the root) and `:259 fs::read_to_string(&foreign_path)`. **No home, grammar or trackability check on `path`.**
3. `:278 let recorded = repo_relative_source_path(&repo_root, path)` (`:196`) — strips a `repo_root` prefix, drops `.`, pops `..`. An absolute outside path survives whole; a `../` escape collapses to a nonexistent in-repo path.
4. `:290 start::mint_migration_in_repo(…, &recorded)` → `crates/cli/src/start.rs:255 migration_task_id` (`blake3(source-path)` + slug of the stem → the path becomes the **task id**), `:395 migration_commit_summary` / `:401 migration_commit_body` (the path becomes **commit-message prose**).
5. `:295 state::persist(minted.dir.join(state::SOURCE_PATH_FILE), recorded)` — `.jigc/tasks/<id>/source-path`, a plain mutable file.
6. `crates/engine/src/finalize.rs:400 plan_retirements` — `state::read_source_path(task_dir)`, `store::lexical_normalize`, blocks if `promotions.is_empty()` (F1), skips if source == a promotion destination (the same-path carve-out), else returns `vec![foreign]`. **No re-validation of the token.**
7. `crates/cli/src/task.rs:3016 retire` — `let path = repo_root.join(retirement); fs::read(&path)` (capture for rollback) then **`fs::remove_file(&path)`**. An absolute `retirement` replaces the root.
8. `crates/cli/src/task.rs:3064 stage_migration` — stages the deletion only if `path_in_index`, so an out-of-repo or `.git/` deletion is silently unstaged and invisible in the manifest.

**Second consumer of the same untrusted token:** `crates/cli/src/task.rs:1426` and `:1794` read
`state::read_source_path(&self.dir)` as `retire_exempt` and pass it to
`crates/engine/src/finalize.rs:738` — a tampered `source-path` also buys a **carryover-gate
exemption** for an arbitrary path.

**Which functions a fix touches:**

- **At the door (necessary, not sufficient):** `migrate.rs::migrate_in_repo` — one call to
  `crate::trackable::untrackable_reason(&repo_root, path)` (plus `is_workbench_root`) before
  `fs::read_to_string`, so the read never escapes and nothing mints. Cheapest complete cell set,
  and the predicate already ships. Also `repo_relative_source_path` (`:196`), whose `..`-popping
  currently *manufactures* an in-repo path that names nothing.
- **At the sink (necessary — driven):** `engine::finalize::plan_retirements` (`:400`) and/or
  `cli::task::retire` (`:3016`). The working area is mutable and the tamper repro lands byte loss
  at exit 0 through a door whose argument was benign, so a door-only fix does not close the class.
  The engine cannot call `untrackable_reason` (it is CLI-side and shells out to git), so the sink
  guard is either a **lexical** engine-side refusal (absolute · any `..` component · any `.git`
  component → a `Finding` from `plan_retirements`) or the CLI validating the plan's retirements
  before `retire` runs. `task.rs:1426`/`:1794` want the same guard for `retire_exempt`.
- **The surface (independent of both):** the exit-4 review hold — `crates/cli/src/render.rs`'s
  migration-review render and the pinned JSON keys `["review","rewrites","source","task"]` — names
  **nothing** about the retire. **The one human gate on jigc's only byte-destructive op does not
  name the file it will delete, in text or in JSON, and `jigc task validate` does not preview it.**
  Driven: `grep -c "$EXT" hold.txt` → `0`; `'retire' in json` → `False`.
- Law-1 riders: `start.rs:395/:401` (host path into the permanent commit message),
  `migrate.rs:266` (host path in the read-fault message), and the falsified
  `UNSWEPT_PRODUCERS` row at `repo_relative_paths.rs:769`.

---

## 5. Known stubbed / deferred / latent in this area (from `implementation/decisions-pending.md`)

- **`jigc ingest` adopts an unaddressable identity** — `decisions-pending.md:593` (recorded
  2026-09-05, M50 Inc 2 T1). **Confirmed live at HEAD, and the surfaces contradict each other.**
  With a conformant `docs/research/OddName.md` committed:
  `jigc ingest` → rc=0 `adoptable docs/research/OddName.md → research (adopted — indexed +
  baselined, no file moved)` and it **does** persist (`.jigc/state/file-state.json` gains the key);
  `jigc doc list research` → `research:OddName  docs/research/OddName.md  unregistered`;
  `jigc doc show research:OddName` → rc=1 `store.malformed-slug`;
  `jigc validate` → **rc=1**, `advisory · schema-conformance.unadopted-instance — … never adopted
  by jigc`, **routed `run 'jigc ingest' to route it`** — the verb that just said it adopted it.
  A second `jigc ingest` repeats the claim. **Three doors, two answers, and a route that,
  followed exactly, changes nothing** (PT-1's shape at the ingest door). *Status:* deferred with a
  written trigger; the ledger entry understates it — it names the registration, not the
  self-contradiction or the looping route.
- **`engine::validate::adoption_route` emits an unquoted path token** — `decisions-pending.md:594`.
  A foreign file whose path contains a space panics `jigc validate` at exit 101 on a **debug**
  build (`Route::mechanical`'s shell-safety fence) and prints a route that runs against the wrong
  path on release. *Not re-driven here* (the rig drives the release binary by default, where the
  fence is compiled out). Deferred; the repair is an engine/CLI seam decision (`shell_token` is
  CLI-side).
- **`store.malformed-slug` scope** — `decisions-pending.md:249` records the M50 Inc 2 shipped
  scope: the nine `DoctypeArg::Address` doors, deliberately **not** inside `Address::parse`
  (the planning spike measured that reddening `doc_read_surface`'s law-1 fence, because
  `doc schema --format json` advertises type-level addresses carrying `<slug>`). Verified
  built + proven in §2g.
- **`UNSWEPT_PRODUCERS`' 79-producer law-1 remainder** — `decisions-pending.md:602` (N31 struck
  2026-09-09; the remainder is the declared bound). `migrate.rs`'s row is one of them and its
  stated reason is falsified (§2a.5).
- **N26, a manifest-less project pack stamps nothing** — `decisions-pending.md:599`. Adjacent:
  it is what makes `jigc relocate`'s freeze-exempt branch reachable at all (§2c) — no shipped
  doctype can reach it.

---

## Rows, classified

| # | Capability | Status | Evidence | Gap |
|---|---|---|---|---|
| 1 | `ARG_TOKENS` totality + the three derived door registries | **built + proven** | `cli.rs:1552`; `cli_parse::{every_clap_argument_is_classified, every_doctype_door_is_registered, every_work_unit_id_door_is_registered, every_slug_door_is_registered}` | The `Plain` verdict is uncheckable by design (declared, `cli.rs:1545`), and the five path args sit in it with **no registry and no axis suite** |
| 2 | `work-unit.malformed-id` at the resolve seams | **built + proven** | 6 driven cells at `task discard`, all rc=1 with code, `.jigc/tasks/` intact | — |
| 3 | `store.malformed-slug` at address `<slug>` heads incl. `add-from-spec` | **built + proven** | 3+3 driven cells, rc=1 + code + route | `adr:` (empty slug) answers code-less |
| 4 | `config fill/fork/replace-step/remove-step` `target` id | **built + proven** | 4 driven cells → `config.fill-point-absent` / `config.anchor-absent`, code + route, token echoed verbatim | — |
| 5 | `jigc unmanage <path>` | **built + proven** | 7 driven cells, all rc=0 no-op, no file touched | `""` renders a blank subject |
| 6 | `jigc relocate --from` | **deferred-by-design (inert)** + cleared | 36 driven cells on shipped doctypes (all `frozen doctype` rc=1); 6 more on a `--pack-from-dev` freeze-exempt `adr` (`0 moved`, clean status) | Reachable only via a pack with no manifest entry; an escaping `--from` is a silent exit-0 `0 moved` |
| 7 | `jigc migrate <path>` door validation | **latent defect (data loss)** | §2a table; absolute → external file deleted at exit 0; `.git/config` deleted; `.git/HEAD` corrupts the index at exit 1 | No home/grammar/trackability rule at the door; `trackable::untrackable_reason` already ships and is not called |
| 8 | `source-path` re-validation at the destructive sink | **latent defect (data loss)** | Tamper repro: benign mint, `.jigc/tasks/<id>/source-path` rewritten, `--approve` rc=0 deletes an arbitrary host file while the ack names `foreign.md` | `plan_retirements` / `retire` read the token raw; `retire_exempt` (task.rs:1426/:1794) is a second raw consumer |
| 9 | The migration review hold names its retire target | **stubbed / absent** | `grep -c "$EXT" hold.txt` → 0; JSON keys `["review","rewrites","source","task"]`; `task validate` silent | The only human gate on the only byte-destructive op names no path |
| 10 | `config insert-step/replace-step <file>` | **latent defect (read escape)** | absolute, `../`, `.git/config` all rc=0 → copied into `.jigc/config/steps/` **and composed into `jigc start`'s step text** | No home rule on `file`; miss shapes carry no code/route |
| 11 | `--from-file` at `doc set-slot`/`doc author`/`config fill` | **shape-limited** | `/etc/hosts` and `.git/config` read at rc=0 into a slot / fill file | Read is unbounded by design (the `Plain` premise); the **route floor** is the real gap — 4 miss shapes, bare `anyhow` + errno |
| 12 | `SLUG_DOORS` × filesystem name-length ceiling | **latent defect (route floor + false route)** | `doc rename` bare os error 63; `start` and `doc create` answer `task.working-area-io` with a route that blames *"a disk or permissions problem"* | EC-28's *"`doc create --slug` refuses correctly at the create-gate"* is **refuted**; `doc add-item` has no ceiling (rc=0); the `migrate --slug` cell is masked by the singleton |
| 13 | `ROOT_KNOBS` value rule × the whitespace class | **latent defect** | `"   "`, `" "`, `"  x  "`, `"-"` all rc=0; `config get` reads back indistinguishable from unset | Predicate is "no control characters", wants "reads back as itself" |
| 14 | Law-1 over the path family | **shape-limited** | `migrate`'s read fault prints the joined host path; the migration commit **subject** carries the absolute host path into permanent history | `repo_relative_paths.rs` fences `.display()` sites only, so `String`-typed absolute paths are outside its subject; `UNSWEPT_PRODUCERS:769`'s reason for `migrate.rs` is falsified |
| 15 | `jigc ingest` × an unaddressable identity | **deferred (with a live self-contradiction)** | ingest `adoptable … adopted, indexed + baselined` at rc=0; `doc list` `unregistered`; `doc show` `store.malformed-slug`; `validate` rc=1 *"never adopted"* routed at `jigc ingest` | Ledger entry `decisions-pending.md:593` names the registration, not the three-door disagreement or the looping route |
| 16 | Echoed vs parsed token at address-refusing doors (EC-25) | **latent defect, ≥3 doors not 1** | `doc retitle-item …#milestones/../../../etc` → message `no section "milestones/../.."` (one component dropped) while `at:` carries the typed form; same at `doc add-item` and `doc set-slot` (`#../../x` → `no section ".."`) | The message renders a *normalized* token; the reader is taught an address they did not type |

### Reachable-but-unmeasured cells (named, not claimed)

- `jigc migrate .jigc/<x>` driven only to mint (rc=0); the `--approve` deletion of a workbench file
  is inferred from the same `retire` path, not driven.
- `jigc migrate --slug <300>` past mint — masked by `changelog` being a fixed-slug singleton; a
  non-singleton migratable doctype would be needed.
- `jigc rename --slug <300>` — masked by `rename.in-flight` in the fixture used.
- `engine::validate::adoption_route`'s space-token panic — debug-build only; the rig drives release.
