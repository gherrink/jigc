# Findings — every lead driven, with its repro block

**The rule** ([protocol.md](protocol.md) §9, carried from `RC-rc14/findings-verification.md`):
every CONFIRMED, PARTIAL **and REFUTED** verdict arrives with a repro block driven on a fresh
corpus against the published `jigc 1.0.0-rc.24`, and a `pinned-by: <suite>::<test>` verified **by
reading what the cited test asserts**, or a stated `UNPINNED: <why>`. No mechanical checker fences
this; [pinning.md](../../../implementation/pinning.md) §3 refuses one by name.

**What this file is.** The scorer's 39 leads — 24 product (`L-`), 15 tooling (`T-`) — each handed
to its own verifier, which drove it and returned one file. Those files are concatenated here in id
order, unedited but for the framing: each file's title became the `## <id> · <claim> —
**VERDICT**` line with a `tier:` line under it, and its own section headings moved one level
down. A lead is not a finding until its row here says so; [README.md](README.md) → *Findings* carries the
rows in the shape the findings channel files, and → *Tooling findings* the harness's.

**The tally.** Product: **11 CONFIRMED · 11 PARTIAL · 2 REFUTED**. Tooling: **10 CONFIRMED ·
5 PARTIAL**. By tier, over the 22 product rows that are not REFUTED: **1 at tier 1 by the
predicate** (L-22's confirmed variant; its own row also sets out what argues it down to 3, and
leaves the weighing to the reader) · **1 at tier 2** (L-3) · **6 at tier 3** (L-2, L-4, L-6, L-10, L-11, L-18) · **14 with no
tier** — reproduced, and contradicting no contract. Tooling rows carry no tier: the scale grades
the product.

**Status of the ledger: OPEN.** Every row carries a `pinned-by:` citation or an `UNPINNED:` with
its reason inside its own *Pin* section, written by its verifier. The row-by-row re-read that
closed RC-rc14's ledger — opening each cited test again at the close — has not been run over this
file, and RC-rc14's closure check is not claimed: these rows state their dispositions in prose
under a heading, not on a line opening with the token, so that check's form does not apply as
written.

**Where it was driven.** Every product repro ran on the installed registry build (`jigc --version`
→ `jigc 1.0.0-rc.24`, release posture) on a throwaway rig under a `mktemp -d` root, never in a
session's out-dir. `$REPO` is the rig's repository, `<tmp>` its temporary root, `~/out/RC24-…` a
session's out-dir read as the lead's origin and never as its proof. `$TRIAL` is the trial's
scratch directory, which is not committed: `$TRIAL/protocol/…` is this directory, and
`$TRIAL/rubric/`, `$TRIAL/logs/` and `$TRIAL/scoring/` stayed on the machine that ran the trial
([evidence/README.md](evidence/README.md)).

---

## L-1 · The composed `architecture-documentation` text never mentions code/prose disagreement, and on the path a worker is handed the router catalog line is the one surface naming `report-inconsistency` — but `jigc describe` and `jigc workflow report-inconsistency --preview` name it too, so "only" is overclaimed; no contract is contradicted — **PARTIAL**

`tier:` none

**Kind:** product lead · **door:** `workflow:architecture-documentation`; `jigc start`
**Binary driven:** the installed registry build, `jigc --version` → `jigc 1.0.0-rc.24` (release posture).
**Rig:** fresh — `completions/trial-corpus-template/instantiate.sh` **without** `--clean-prose`
(the wart corpus: the three prose sites present, driven), then `jigc setup`, under a `mktemp -d`
root with `HOME` repointed inside it. The session's out-dir was read only to confirm the lead's
origin, never as proof.

### The claim

> The only surface that names `report-inconsistency` to a worker documenting architecture is one
> line of the router catalog; the composed `architecture-documentation` text says nothing about
> what to do when code and prose disagree.

### Verdict

**PARTIAL** — both halves of the scorer's repro reproduce exactly, and the second sentence of the
claim holds in full; the word **"only"** is carried on the path the binary puts in front of the
worker and is false as a statement about the binary's surfaces.

- **Holds (driven).** The composed `architecture-documentation` text — 138 lines — has **0**
  matches for `inconsisten|disagree|contradict|report-`. Bare `jigc start` names
  `report-inconsistency` on **1** line of the catalog. Nothing else the worker is handed without
  asking names it: the four adapter files `jigc setup` installs (`CLAUDE.md`, `.jigc/AGENT.md`,
  `.claude/settings.json`, `.claude/skills/jigc/SKILL.md`) — 0; `jigc --help`, `jigc start --help`,
  `jigc doc schema arch-doc`, `jigc doc author --help`,
  `jigc workflow architecture-documentation --preview` — 0; and every output on a full walk of the
  task (create · set-slot · add-item · set-field · doc show · commit type and summary ·
  `task validate` · `task finalize`, all exit 0) — 0.
- **The falsifying datum for "only".** Two pull surfaces name it on the same corpus:
  `jigc describe` (line 67 of 119, *"report-inconsistency is File one disagreement inside the
  project — code against a doc, or two docs — …"*, plus the `inconsistency` doctype's own entry)
  and `jigc describe --workflows`; and `jigc workflow report-inconsistency --preview` prints the
  whole filing workflow (127 lines) without minting. `.jigc/AGENT.md` points a worker at
  `jigc describe` by name (*"to learn how `jigc` itself behaves, ask the installed binary
  (`jigc --help`, `jigc describe`, `jigc doc schema`)"*). The intent-carrying router form,
  `jigc start "<intent>"`, prints the same catalog line a second time and ends by pointing at
  `jigc describe --workflows`.
- **A bound on the scorer's pattern.** The `report-` alternative also matches *"report-only"*
  (`.jigc/AGENT.md`, 1 hit) and *"Report-and-route only"* (`jigc --help`, 1 hit). Neither names
  the channel; neither affects the composed text's 0.
- **Origin, read back.** In `~/out/RC24-C` the composed text the session received (7,729 chars)
  has 0 matches for the same pattern and the SessionStart orientation names
  `report-inconsistency` once; the session's 15 invocation records hold no `describe` and no
  `workflow --preview`. So on the path that session walked, the catalog line was the one
  surface it met.

### Repro

```
# setup — wart corpus, adopted on rc.24 (run from the jigc checkout)
W=$(mktemp -d "<tmp>/l1.XXXXXX"); export HOME="$W/home"; mkdir -p "$HOME"
export GIT_AUTHOR_NAME='Corpus Owner'    GIT_AUTHOR_EMAIL='owner@example.invalid'
export GIT_COMMITTER_NAME='Corpus Owner' GIT_COMMITTER_EMAIL='owner@example.invalid'
jigc --version                                   # jigc 1.0.0-rc.24
completions/trial-corpus-template/instantiate.sh "$W/calderby" calderby     # exit 0, 7 commits
REPO="$W/calderby"; [ -d "$REPO/.git" ] || exit; cd "$REPO"
grep -rnE 'in front of|long-term store' src README.md package.json
#   src/store.ts:4-5 · README.md:3 · package.json:5      (the wart is present)
jigc setup                                       # exit 0, install commit, 8 commits

# argv 1 — orientation
jigc start > "$W/orient.out"                     # exit 0, 24 lines
grep -c  report-inconsistency "$W/orient.out"    # 1
grep -n  report-inconsistency "$W/orient.out"
#   17:  - report-inconsistency — code and a doc, or two docs, disagree and the
#        disagreement is worth a record until it is reconciled
#   (one of 13 lines under "Available workflows:")

# argv 2 — the composed text
jigc start --workflow architecture-documentation "x" > "$W/arch.out"   # exit 0, 138 lines, stderr empty
grep -ciE 'inconsisten|disagree|contradict|report-' "$W/arch.out"      # 0

# the other surfaces on the same corpus (count of the same pattern / of the workflow's name)
CLAUDE.md 0 · .claude/settings.json 0 · .claude/skills/jigc/SKILL.md 0
.jigc/AGENT.md 1  ("report-only" — not the channel; `report-inconsistency` 0)
jigc --help 1     ("Report-and-route only" — not the channel)
jigc start --help 0 · jigc doc schema arch-doc 0 · jigc doc author --help 0
jigc workflow architecture-documentation --preview 0
jigc start "document the architecture"           # exit 0 — the router; names it on 1 line
jigc describe                                    # exit 0 — names it (line 67 of 119)
jigc describe --workflows                        # exit 0 — names it (1 line)
jigc workflow report-inconsistency --preview     # exit 0 — 127 lines, no task minted

# the walked path, task `x`, every call exit 0, pattern count 0 in each output
jigc doc create arch-doc --title "Rollup Path" --task x
jigc doc set-slot  arch-doc:rollup-path#overview --from-file <f> --task x
jigc doc add-item  arch-doc:rollup-path#components --title "Router" --task x
jigc doc set-slot  arch-doc:rollup-path#components/router/description --from-file <f> --task x
jigc doc set-field arch-doc:rollup-path#components/router/implemented-by --value 'src/router.ts#Router' --task x
jigc doc show arch-doc:rollup-path --task x
jigc doc set-field commit:x#type --value docs --task x
jigc doc set-slot  commit:x#summary --from-file <f> --task x
jigc task validate x
jigc task finalize x          # finalized — docs: add the rollup path architecture doc; 1 file committed

# after
git log --oneline -2          # the arch-doc commit over setup's install commit
git status --porcelain        # empty
ls docs/inconsistencies       # exit 1 — no such directory; nothing was filed, nothing was asked for
jigc start | grep -c report-inconsistency    # 1 — the catalog line is unchanged after the commit
```

The `CLAUDECODE` variable was set and unchanged through every cell; no cell is compared against
another on it.

### The contract

**None is contradicted.** The design of record states what the binary does:

- `design/findings-channel.md` → *What M55 settles*, S3: *"`report-inconsistency` is
  router-visible."*
- `design/findings-channel.md` → §2 *The workflows — two to file, two to triage*: the Router row
  for `report-inconsistency` is *"**visible** — `selectable: true` + a `when:` hint (the
  `park-idea` precedent)"*, and *Why `report-inconsistency` is visible* gives the reason
  (*"parking a finding must cost less than losing it"*). The catalog line printed above is that
  `when:` hint, byte for byte
  (`crates/cli/packs/methodology/workflows/report-inconsistency.yaml`, `when:`).
- No rule in `design/findings-channel.md`, `design/workflow-dialect.md` or
  `design/surface-contract.md` says a work-workflow's composed text cross-references the channel,
  and none does: across the compose goldens the name appears only in the router, the orientation,
  `describe` and the workflow's own preview. `park-idea`, the precedent the design names, is
  advertised the same way — by its catalog line alone.
- `crates/cli/packs/dev/workflows/architecture-documentation.yaml` composes
  `step:author-arch-doc` · `step:author-commit` · `step:finalize`; the text makes no claim about
  disagreements, so there is no surface saying something the binary does not do.

What the binary does instead: it advertises the channel once, in the catalog every `jigc start`
prints (and the SessionStart hook runs `jigc start`), and describes it on request through
`jigc describe` and `--preview`. Whether one catalog line is enough for a worker that is already
inside another workflow is a design question for the human — the protocol's own reading for C-2
is *"the router line's wording is the first surface to look at"* (protocol §5.3) — not a defect.

### Tier

**none.** Tier 1 needs an exit-0 loss or repository harm through a committing, destroying or
moving door: every call exits 0 and nothing is lost or damaged (the arch-doc commit holds exactly
the one promoted file; the tree is clean). Tier 2 needs a posture or route dead end: the channel
is reachable in one call from the catalog line (`jigc start --workflow report-inconsistency
"<intent>"`), beside an open task. Tier 3 needs a surface that says something the binary does not
do: no surface says the composed text will raise the channel. The lead is a discoverability
observation; an arm's outcome class is not a finding.

### Pin

`pinned-by:`
- the catalog line — `findings_workflows::the_catalog_lists_report_inconsistency_and_none_of_the_hidden_three`
  (group `g_methodology`), and byte-wise `compose_goldens::sweep_fresh` over
  `crates/cli/tests/goldens/compose/composite/start-orient--start-orient--fresh.txt` (1 line
  naming it);
- the composed text — `compose_goldens::sweep_fresh` (and its five sibling state sweeps) over
  `crates/cli/tests/goldens/compose/dev/start--architecture-documentation--fresh.txt` (0 matches
  for the pattern). The golden equals the fresh rig's stdout line for line once the task id and
  the intent are normalised.

Two bounds on the pin: the **absence** is pinned as bytes only — no named assertion says the
composed text must or must not mention the channel, which is right while no contract says either;
and the suites were **not run** in this verification — the goldens were read and diffed against
the installed binary's output, and `crates/cli/packs`, `crates/cli/src`, `crates/cli/adapters` and
`crates/engine/src` are identical between the `jigc-v1.0.0-rc.24` tag and the checkout read.

### Not done

No fix, no commit, no repository edit; nothing under `~/out` or `~/ideas` was modified. The wart
corpus was not driven through a blind worker — this verifies what the surfaces print, not what a
worker does with them.

---

## L-2 · Orientation prints ``Run: `jigc start --workflow planning` `` and that line, run as printed, exits 1 with "requires an intent"; the sibling `ingest-existing` line runs as printed (exit 0) — **CONFIRMED**

`tier:` 3

**Lead (product):** Orientation prints ``Run: `jigc start --workflow planning`   — plan a milestone …``, and that command run as printed exits 1 with *requires an intent*.
**Source:** arm a #3 → #4; debrief-a item 1, first bullet. **Door:** `jigc start`.
**Binary:** `~/.local/bin/jigc`, `jigc --version` → `jigc 1.0.0-rc.24` (asserted first). Release posture. `crates/` at the working tree is byte-identical to tag `jigc-v1.0.0-rc.24` (`git diff --stat` over `crates` is empty), so the source cited below is the source of the driven binary.

### Verdict

**CONFIRMED** — for the `planning` line, on a fresh rig, in both orientation states that print it (clean, and with a live task).

**Scope correction, with its falsifying datum:** the sibling line does **not** share the defect. `jigc start --workflow ingest-existing`, run as printed, exits **0** and composes the workflow. The scorer's "do the same for the sibling line" does not reproduce; the lead is one line wide, not two.

### Repro

```text
# setup — fresh rig, installed registry binary; the trailer variable is set and held
# constant across every cell (no cell below makes a commit)
rig=$(dev/jigc-rig fresh --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit
$JIGC --version                      # jigc 1.0.0-rc.24
git -C "$REPO" rev-parse HEAD        # recorded as the before-control; .jigc/tasks empty

# cell 0 — the surface
$ jigc start                                              # exit 0
  jigc — orientation
  Pack: dev/1.0.0-rc.24 | methodology/1.0.0-rc.24 · Project config: <tmp>/repo/.jigc/config
  Available workflows:
    … (13 selectable workflows) …
  Run: `jigc start "<intent>"`   — presents the workflows above; pick one, then re-run with `--workflow <chosen>` to compose it
  Preview: `jigc workflow <id> --preview`   — …
  Run: `jigc start --workflow planning`   — plan a milestone — decompose it into increments and tasks
  Run: `jigc start --workflow ingest-existing`   — bring an existing repo's docs under management

# cell A — the planning line, copied verbatim
$ jigc start --workflow planning                          # exit 1, stdout empty
  stderr: workflow 'planning' requires an intent: jigc start "<intent>" --workflow planning

# cell B — the sibling line, copied verbatim  (THE FALSIFIER for the sibling half)
$ jigc start --workflow ingest-existing                   # exit 0, stderr empty
  stdout: Scan the repo for documents jigc can manage:
          Run: `jigc ingest`
          …

# cell C — the trial's own argv (arm a #4)
$ jigc start --workflow planning --format json            # exit 1, stdout empty
  stderr: { "error": "workflow 'planning' requires an intent: jigc start \"<intent>\" --workflow planning" }

# cell D — the machine surface carries the same route as data
$ jigc start --format json                                # exit 0
  state: clean
  next_steps: [{"id":"planning","gist":"plan a milestone — decompose it into increments and tasks"},
               {"id":"ingest-existing","gist":"bring an existing repo's docs under management"}]

# cell E — control: the form the refusal names
$ jigc start "plan milestone one" --workflow planning     # exit 0
  stdout: task minted: plan-milestone-one   …  Scope the milestone:

# cells F, G — state independence: with that task live, orientation still prints the line
$ jigc start                                              # exit 0; line 40 of the output:
  Run: `jigc start --workflow planning`   — plan a milestone — decompose it into increments and tasks
$ jigc start --workflow planning                          # exit 1, same stderr as cell A

# after — cells A–D: `git status --porcelain` unchanged, HEAD unchanged, .jigc/tasks unchanged.
# A refused run opens no working area and commits nothing. HEAD is unchanged after cell G too.
```

### What the binary does, and why

`crates/cli/src/render.rs` → `catalog_block` renders one line per off-catalog verb in **one shape** — ``Run: `jigc start --workflow <id>`   — <gist>`` — and its own comment says so ("same shape for each"). `crates/cli/src/orient.rs` → `OFF_CATALOG_VERBS` names two verbs whose front-matter differs on exactly the field that decides whether that shape runs:

- `ingest-existing` is `creates-task: false` — `crates/cli/src/start.rs` → `compose_named_no_intent_in_repo` composes it over an empty intent (cell B).
- `planning` is `creates-task: true, selectable: false` — the same function bails with *requires an intent* (cell A), because a minting workflow slugs its task id from the intent.

The line's shape is gated on pack membership only, never on `creates-task`. It is a hand-built string, not a `Route::mechanical`, so no fence reads it.

### Contract contradicted

1. **`design/surface-contract.md` → The three laws → Law 1 (nothing lies) and Law 2 (nothing hides).** Law 2: "Routes and `Run:` lines parse against the real CLI." The printed line does *parse* (clap accepts it), so the letter of that sentence holds; what fails is the followability the same doc demands one section down — **→ The fences → The route fence (law 2)**: "A mechanical route that hard-rejects, or repairs the wrong thing, is law 2 failing one level down", and P6: the parse fence "proves a mechanical route runs; it cannot prove it runs here". A `Run:` line that exits 1 when run is that case.
2. **The surface's own convention, on the same screen.** Two lines above, orientation prints ``Run: `jigc start "<intent>"` `` — it spells the intent placeholder where one is required. The `planning` line omits it, so it reads as complete.
3. **The binary's other surfaces spell the route with the intent.** The refusal itself: `jigc start "<intent>" --workflow planning`. The workflow's own front-matter (`crates/cli/packs/methodology/workflows/planning.yaml` → `suppressed.reason`): "invoked by name with the milestone in hand (`jigc start --workflow planning "<milestone>"`)". `design/methodology-docs.md` (Flow 19) and `design/worked-examples.md` spell it with an intent everywhere.
4. **`design/project-setup.md` → Off-catalog discoverability (G6)** asks for "a route-prose line naming the verb, the same shape for each". The shipped line satisfies *naming*; the "same shape" clause is what produces the defect, since the two verbs do not take the same argv.

### Tier

**Tier 3** — a surface says something the binary does not do.

- Not tier 1: exit is 1, not 0; no committing, destroying or moving door is reached; `git status`, HEAD and `.jigc/tasks` are unchanged after the refusal.
- Not tier 2: it is not a dead end. The refusal names the corrected argv, and that argv exits 0 (cell E). The trial session recovered in one retry (arm a #4 exit 1 → #5 exit 0, three seconds apart).

Cost as observed: one wasted invocation per reader who follows the line. The debrief attributes the miss to the reader's own assumption rather than to the printed line, so the session did not notice the surface had routed it there.

### Pin

`pinned-by: compose_goldens::sweep_fresh` (the printed line — `crates/cli/tests/goldens/compose/composite/start-orient--start-orient--fresh.txt:24`, and the same line in the `committed-singletons`, `migrated`, `chatty-hooks` and `vendored` goldens through their sibling sweeps) **and** `start_compose::form_d_creates_task_workflow_with_no_intent_rejects` (the refusal — driven over `architecture-documentation`, not `planning`).

Both halves are pinned as intended behaviour, in different suites; **the contradiction between them is UNPINNED** — no test runs an orientation `Run:` line as printed. `embedded_methodology_compose::with_marker_bare_start_orientation_names_both_off_catalog_verbs` asserts only that the text contains `jigc start --workflow planning`, and a unit test in `crates/cli/src/render.rs` pins the line's bytes. A fix on the surface side moves five goldens and that unit test.

### Notes

- The lead's origin is consistent with the fresh-rig result: arm a #3 is a bare `start` at exit 0, #4 is `start --workflow planning --format json` at exit 1, #5 is the intent-bearing form at exit 0.
- Not new to this trial: the same refusal over the same argv appears in an earlier trial's committed evidence (`completions/artifacts/RC-1.0-gate/evidence/`), and `completions/artifacts/M43/surface-inventory.md` flagged the refusal's inline command span "for law-2 review". Neither record ties it to the orientation line that routes there.
- The JSON envelope of the refusal carries `error` only, no code — that is lead L-3's subject, not driven further here.
- Nothing was modified under `~/out` or `~/ideas`; no repository edits, no commits. Cell E minted one task inside the throwaway rig only.

---

## L-3 · Two `jigc start` refusals (`--workflow planning` with no intent; `--task <sub-task>` from the shared checkout) answer `--format json` with a code-less `{"error": …}` and log `finding_codes: []` — the declared `{error}` arm, not a contract breach; only the missing identity survives — **PARTIAL**

`tier:` 2

**Lead (product, door `jigc start`, source arm a #4 and #41).** Under `--format json`, two
`jigc start` refusals are `{"error": "…"}` envelopes with no finding code and the invocation log
records `error_code: null` for both, rather than the `(code, target)` findings document protocol
§0.1 declares for a refusal. Scorer's proposed tier: 2 (a code-less refusal; the routes work).

### Verdict — PARTIAL

**The facts reproduce, both cells, on a fresh rig with the installed `jigc 1.0.0-rc.24`.** Each
refusal is a single-key `{"error": "<sentence>"}` document, exit 1, with no code anywhere in it and
`finding_codes: []` in the invocation log.

**The contract half does not carry.** The lead reads protocol §0.1 — *"A refusal under `--format
json` is a findings document keyed `(code, target)`"* — as the product's contract. It is the
protocol's paraphrase, and it drops the qualifier its own source carries: the M52 span says *"a
reject **carrying a finding** reaches a `--format json` driver as one document keyed `(code,
target)`"*. The product's contract declares **two** reject arms and says which one a bare refusal
takes (below). Three corrections to the lead as written:

1. **The `{"error": …}` shape is the declared arm, not a deviation from it.** No `design/` rule,
   help text or `AGENT.md` sentence says every refusal is a findings document.
2. **`error_code: null` is the designed value and discriminates nothing.** The keyed control
   (`jigc start --task nope`, a real `(code, target)` findings document) logs `error_code: null`
   too. The log datum that distinguishes these two refusals is **`finding_codes: []`**.
3. **The document rides stderr, not stdout** (the lead says *"read stdout's shape"*): stdout is
   0 bytes at both cells, which is what *Stream discipline* declares for a reject.

**What survives** is the narrower claim: two refusals an agent reaches through routes the
orientation itself prints carry **no identity at all** — not in the document, not in the log — at a
door where the neighbouring refusals carry one. That is a member of the class the project's own
tier scale names (*code-less refusal*) and that it has twice treated as owed an identity (below).
It is not a dead end: both printed routes were run and both work.

### Repro

Driven 2026-10-04, host registry build, release posture. `CLAUDECODE`: set, held constant across
every cell (no cell compared here depends on it; the refusals commit nothing).

```sh
# setup
jigc --version                                   # jigc 1.0.0-rc.24
rig=$(dev/jigc-rig fresh --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit
$JIGC config set invocation-log true             # 0
git -C "$REPO" add .jigc/config/manifest.yaml && git -C "$REPO" commit -q -m "chore: enable invocation log"

# (i) a task-minting workflow named with no intent
$JIGC start --workflow planning --format json; echo "exit $?"
#   exit 1 · stdout 0 bytes · stderr, whole:
#   {
#     "error": "workflow 'planning' requires an intent: jigc start \"<intent>\" --workflow planning"
#   }
#   log: {"argv":["start","--workflow","planning","--format","json"],"exit_code":1,
#         "finding_codes":[],"error_code":null,"output_bytes":101}

# (ii) a milestone sub-task resumed from the shared checkout
$JIGC milestone create "M" --format json         # 0 — minted milestone:m (shared base <a>)
$JIGC milestone add-task m "t" --format json     # 0 — added task:t; record commit <b>
$JIGC start                                      # 0 — prints: Run: `jigc start --task t`   — resume: …
$JIGC start --task t --format json; echo "exit $?"
#   exit 1 · stdout 0 bytes · stderr, whole:
#   {
#     "error": "task `t` is pinned to base <a> but you're on <b> — this is a sub-task of milestone
#       `m`, and a sub-task's work happens in its own worktree, cut from that base rather than in
#       this checkout: run `jigc milestone provision m` — it cuts the worktree this sub-task is
#       missing — then `cd $REPO/.jigc/worktrees/t` and re-run this command there"
#   }
#   log: {"argv":["start","--task","t","--format","json"],"exit_code":1,
#         "finding_codes":[],"error_code":null,"output_bytes":444}

# controls — same door, same rig, same format
$JIGC start --task nope --format json; echo "exit $?"
#   exit 1 · stderr: {"schema_version": 3, "findings": [{"code": "finalize.no-task",
#     "key": {"code": "finalize.no-task", "target": "task:nope"}, "severity": "blocking",
#     "route": "`jigc task list` lists the live tasks", …}]}
#   log: "finding_codes":["finalize.no-task"],"error_code":null        <- null here too
$JIGC start --workflow nope --format json; echo "exit $?"
#   exit 1 · stderr: {"error": "blocking · workflow-refs.unknown-workflow — no workflow `nope` — …\n  route: …"}
#   log: "finding_codes":["workflow-refs.unknown-workflow"],"error_code":null
#   (the declared flatten: the code rides inside the string, and the log has it)

# after — the routes each refusal prints, run as printed
$JIGC start "plan the thing" --workflow planning --format json    # 0 — keys [task, text], task plan-the-thing
$JIGC milestone provision m --format json                         # 0 — provisioned 1 worktree(s) … (t)
( cd "$REPO/.jigc/worktrees/t" && $JIGC start --task t --format json )   # 0 — keys [task, text], task t
$JIGC start --task t --format json                                # 1 — same {"error": …}, now the
#   provisioned arm: "… run this from that worktree — `cd $REPO/.jigc/worktrees/t`, then re-run this command"
git -C "$REPO" status --short                                     # (empty) — neither refusal wrote or committed
```

The text arm is the same sentence with no `blocking · <code> —` prefix, exit 1, at both cells
(log `finding_codes: []` there too).

**Origin, read not driven.** `~/out/RC24-A-frozen-work/turn01/stream.jsonl` carries the (i)
envelope and `turn02/stream.jsonl` the (ii) envelope, byte-shaped as above; `evidence/A/invocations.jsonl`
rows 4 and 41 are exit 1, `finding_codes: []`, `error_code: null`, and row 20 (`write.wrong-shape`,
the keyed refusal the scorer contrasts them with) is `error_code: null` as well.

### The contract

**What the binary is declared to do — and does.**

- `design/command-output-contract.md` → *Every other envelope* → **The two reject arms**: *"An
  operational error carries the single key `error` — the message, as `{"error": …}` on stderr,
  exit 1 — and nothing else. A rejected run that carries findings is the findings envelope."*
- Same doc → **Which reject arm a run takes, in one rule (M52)**: *"A reject that carries a finding
  takes the findings arm …; a reject with no finding behind it takes `{error}`."* and *"`{error}` is
  untouched and stays the default for a bare `anyhow`."*
- `crates/cli/src/render.rs` → `ENVELOPE_ARMS`, the `Reject::Error` row, `status: Pinned`: *"This
  arm keeps every bare-`anyhow` reject …"*.
- `design/surface-contract.md` → **The error-code namespace**: the `error_code` vocabulary is a
  closed registry of twelve identities (eleven committing doors' `*.commit-rejected` plus
  `migrate.review-pending`), *"deliberately not a per-verb code mint — errored verbs already write
  records"*. A `jigc start` refusal is not a member, so `null` is what the log is declared to say.
- `.jigc/AGENT.md` (installed): *"Exit codes: 1 error · 2 usage · 3 blocking findings at a
  task-scope gate"* — exit 1 with an error sentence is inside what the product tells an agent.

Both producers are bare `anyhow`: `crates/cli/src/start.rs` → `compose_named_no_intent_in_repo`
(`bail!("workflow '{workflow_id}' requires an intent: …")`) and `blanket_base_pin_refusal`
(`anyhow!("{pinned_to} — …")`). So the shape the lead reports **is** the contract's `{error}` arm.

**What the code-less half sits against.** Not a `design/` rule — there is none that says every
refusal carries an identity — but the project's own stated claim and precedent:

- `implementation/roadmap.md` → Milestone 52, **The claim**: *"every repository posture and every
  route a caller can reach answers with a code and a followable route"*. Cell (ii) is a posture
  refusal (HEAD has moved off the sub-task's base) reached through the route the orientation
  prints (`Run: jigc start --task t`); it answers with a followable route and no code. Cell (i) is
  reached through the orientation's own `Run: jigc start --workflow planning` line (that is lead
  L-2's subject, not this one's).
- `implementation/decisions-pending.md` → *The rc.16 wave (M52)*, the tier scale's one home: the
  middle tier is *"posture and route dead ends, code-less or undeclared refusals"*.
- Precedent at this door: `crates/cli/tests/start_compose.rs` →
  `a_title_that_slugs_to_nothing_refuses_under_the_mint_class_code` (M53 Increment 5) records the
  same shape at `jigc start --workflow <W> "<title>"` — `{"error": "intent must contain …"}`,
  `finding_codes: []` — as *"no code for a driver to key on and an anonymous exit 1 in the
  invocation log"*, and fixed it by minting `write.unslugable-title` **inside** the flattened
  document and into the log, the arm unmoved.
- Precedent in the findings ledger: `completions/artifacts/M55/seed/jigc-feedback/task-with-no-recorded.md`
  — *"refused without a code … flattened to `{"error": …}` on `--format json` … What is owed is an
  identity at both producers."* Open, tier-2/3 by the M53 Settle.

A related loose sentence, noted and not relied on: `design/measurement.md` (the invocation-log
paragraph) says `error_code` is *"`null` except on a `Finding`-less operational failure"*, which read
alone would give these two a non-null value; the registry's owning doc (`surface-contract.md`, above)
bounds it to twelve identities, and that is the reading the binary implements.

### Tier — 2, in its weakest form (never blocks)

- **Not tier 1.** Exit 1, not exit 0; `jigc start` in these forms is a read/compose door, nothing
  was written, committed, destroyed or moved (`git status --short` empty afterwards, no new commit).
- **Not a dead end.** Both refusals name a route and both routes were run as printed and exit 0.
  Under a strict *"posture or route dead end"* reading of tier 2 alone this would be tier `none`.
- **Tier 2 by the scale's own member list**, which is what protocol §1 pre-registered: *"a posture
  or route dead end; a code-less or undeclared refusal"*. These are two code-less refusals — a
  driver gets a sentence to show a human and nothing to key on, and the log gets an anonymous
  exit 1 indistinguishable from any other bare failure at the door. The refusal itself *is*
  declared (the `{error}` arm), so only the *code-less* member applies, not *undeclared*.
- **Not tier 3.** No product surface says these refusals are keyed; the sentence that does is the
  trial protocol's §0.1, which over-states its source. That sentence is the thing to correct in
  the record: it should read *"a refusal that carries a finding …"*.

### Pin

`UNPINNED` for the claim as verified — no test drives either cell under `--format json` or reads
its invocation-log record, so neither the `{error}` shape at these two cells nor their empty
`finding_codes` is fenced.

What is pinned nearby, and what each one does not reach:

- `start_compose::form_d_creates_task_workflow_with_no_intent_rejects` (group `g_compose`) — cell
  (i)'s **text-arm** sentence and non-zero exit, driven with `architecture-documentation`; not the
  JSON document, not the log.
- `start_resume::sub_task_read_doors_keep_the_blanket_base_pin_refusal` (group `g_compose`) and
  `anyhow_route_spans::pinned_base_mismatch_routes_by_unit_kind` (group `g_finalize`) — cell (ii)'s
  **text-arm** bytes, the refusal's verdict over both read doors, and its route run verbatim; not
  the JSON document, not the log. The second suite's name and header state the producer is an
  `anyhow` on purpose (*"anyhow-embedded route … prose-only error text untouched"*).
- `flow53_acceptance::every_pre_dispatch_fault_answers_every_leaf_on_a_declared_arm` — the
  `Reject::Error` arm's key set (`["error"]`) as a declared arm, over pre-dispatch faults; it does
  not enumerate which post-dispatch refusals take it.

### Notes

- Adjacent, not scored here: in cell (ii) the orientation printed in the shared checkout offers
  `Run: jigc start --task t — resume: re-composes this task's own workflow where it left off`, and
  that exact line refuses from the checkout it was printed in. That is a route-truth question about
  the orientation (the L-2 family), separate from whether the refusal carries a code.
- The door shows three wire states for a refusal on one rig: a keyed findings document (unknown
  task id), a flattened string with the code inside and in the log (unknown workflow), and the two
  bare sentences of this lead. Only the last has no identity on any surface.
- Bounds: one rig state (`fresh`), one platform (macOS host, registry release build). The provisioned
  arm of (ii) was driven once and answers on the same `{error}` arm. The top-level-task arm of the
  base-pin refusal (`git checkout <pin>` / `jigc task discard`) was not driven.

---

## L-4 · Orientation's resume line for a milestone sub-task says it re-composes the task and the binary refuses it from the checkout that printed it; no forward surface (orientation, planning tail, add-task ack) names `milestone provision` or `milestone execute` — **CONFIRMED**

`tier:` 3

**Lead (product).** Over an open milestone with unstarted sub-tasks, orientation names `resume`,
`task validate`, `milestone finalize` and `task discard` per sub-task and neither
`milestone provision` nor `milestone execute`; its first route (`jigc start --task <sub-task>`)
refuses from that checkout; the `planning` text's tail and the `milestone add-task` ack name
neither door either.

**Doors:** `jigc start` (orientation) · `jigc start --task <sub>` · `workflow:planning` ·
`jigc milestone add-task`.

### Verdict — CONFIRMED

All four observed halves reproduce verbatim on two fresh rigs with the installed registry build
(`jigc --version` → `jigc 1.0.0-rc.24`, release posture). One precision correction to the scorer's
repro and one scoping correction to what the lead is a defect *of* are stated below.

| Half | Driven | Result |
|---|---|---|
| orientation names neither door | `jigc start` over `create` + 2 × `add-task` | exit 0; `milestone provision` / `milestone execute`: **0 hits**; per sub-task exactly four `Run:` lines — `start --task`, `task validate`, `milestone finalize`, `task discard` |
| the first route refuses | the printed `jigc start --task t-one`, verbatim, from the checkout that printed it | **exit 1**, stdout empty, refusal on stderr; HEAD and `git status` unchanged |
| `planning` text names neither door | `jigc start "plan M" --workflow planning` (429 lines) | exit 0; `milestone provision` / `milestone execute`: **0 hits**; the tail names `milestone create`, `milestone add-task`, `milestone add-from-spec` only |
| `add-task` ack names neither door | `jigc milestone add-task m "t one"` | exit 0; three lines (`added …`, `record commit: …`, the footer) and **no `next:`** — while `milestone create`'s ack one verb earlier does print `next: jigc milestone add-task …` |

The route the refusal prints **works**, as the scorer said: `jigc milestone provision m` → exit 0,
then `cd $REPO/.jigc/worktrees/t-one` and the same `jigc start --task t-one` → exit 0 (72 composed
lines). So this is not a dead end.

**Precision correction (scorer's repro).** `grep -cE 'provision|execute'` is exact on the
orientation output (0) but returns **1** on the `planning` text — line 63 uses the plain verb
("…so they execute…"), not a door. The predicate that carries the claim is
`milestone (provision|execute)`, which is 0 on both.

**Origin corroborated, not relied on.** `evidence/A/invocations.jsonl` rows 40–43: `start` (0) →
`start --task <sub> --format json` (**1**) → `milestone provision …` (0) → `start --task <sub>` (0);
zero `milestone execute` rows in the arm's 48. The debrief's fourth bullet says the requirement
"only became visible once the sub-tasks actually went active".

### Repro

```
# setup — fresh rig, installed binary; CLAUDECODE set and held constant in every cell
rig=$(dev/jigc-rig fresh --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit
jigc --version                         # jigc 1.0.0-rc.24
jigc milestone create "M"              # exit 0
#   minted milestone:m (shared base <setup-sha>)
#   record commit: <sha>   …
#   next: `jigc milestone add-task m "<intent>"`   — add the milestone's first sub-task

# argv 1 — the add-task ack
jigc milestone add-task m "t one"      # exit 0
#   added task:t-one to milestone:m
#   record commit: <sha>   — task:t-one on the milestone record, committed on its own; …
#   — jigc · run `jigc start` for orientation; all writes through `jigc`.
jigc milestone add-task m "t two"      # exit 0, same three-line shape; no `next:` on either

# argv 2 — orientation
jigc start > start.out                 # exit 0
grep -cE 'milestone (provision|execute)' start.out      # 0   (and 0 for the looser 'provision|execute')
#   Active task: t-one
#     workflow: sub-task
#     base:     <setup-sha>
#     staged:   nothing staged yet
#   Run: `jigc start --task t-one`   — resume: re-composes this task's own workflow where it left off
#   Run: `jigc task validate t-one`   — previews part of the finalize gate: …
#   Run: `jigc milestone finalize m`   — validate + commit: this is a sub-task of milestone `m`, whose door is its only commit boundary — `jigc task finalize t-one` refuses here
#   Run: `jigc task discard t-one`   — abandon: removes the working area
#   (the same four lines for t-two)

# argv 3 — the printed resume line, verbatim, same checkout
jigc start --task t-one                # exit 1, stdout empty
#   task `t-one` is pinned to base <setup-sha> but you're on <add-task-2-sha> — this is a sub-task of
#   milestone `m`, and a sub-task's work happens in its own worktree, cut from that base rather than
#   in this checkout: run `jigc milestone provision m` — it cuts the worktree this sub-task is
#   missing — then `cd $REPO/.jigc/worktrees/t-one` and re-run this command there

# argv 4 — the planning text
jigc start "plan M" --workflow planning > planning.out  # exit 0, 429 lines
grep -cE 'milestone (provision|execute)' planning.out   # 0
grep -n 'jigc milestone' planning.out
#   392: jigc milestone create "<the title you just used>"
#   397: jigc milestone add-task <milestone-id> "<intent>"
#   400: … `jigc milestone add-from-spec <milestone-id> <spec-addr>` …
#   and its trailer: `t-one` (workflow `sub-task`) — resume it with `jigc start --task t-one`

# after
git rev-parse HEAD; git status --porcelain   # HEAD unchanged across argv 2–3; tree clean
ls .jigc/worktrees                           # No such file or directory — nothing provisioned
ls .jigc/tasks                               # t-one  t-two  (untouched)

# the route onward (second fresh rig, same setup) — it works
jigc milestone provision m             # exit 0: provisioned 2 worktree(s) for milestone:m at base <setup-sha> (t-one, t-two)
cd "$REPO/.jigc/worktrees/t-one" && jigc start --task t-one   # exit 0, 72 lines composed
cd "$REPO" && jigc start | grep -cE 'milestone (provision|execute)'   # still 0 after provisioning,
#   and the same `Run: jigc start --task t-one` line is still printed from the main checkout
jigc milestone execute m               # exit 0 — the only composed text carrying `Run: jigc milestone provision m`,
#   the `Spawn:` lines, `milestone join` and `milestone finalize`
```

The git history in the rig after setup is the setup commit plus three record-only commits
(`chore(milestone): open record …`, `… record task:t-one …`, `… record task:t-two …`) and **no
other commit**. So the cause of the refusal is isolated: `milestone create` pins the base at the
HEAD it runs on and then lands its own record commit, which moves HEAD off that pin. Under the
default `jigc setup` composition every sub-task is therefore off its pin in the main checkout from
the moment it exists, and the printed resume line refuses there on every run, not only after
unrelated history.

**Where the two doors are named at all** (driven, same rigs): the resume refusal (`provision`
only); `jigc milestone --help`; `jigc start --workflow milestone-execution` → `workflow.verb-routed`
routing to `jigc milestone execute <milestone-id>`; and `milestone execute`'s own composition.
The installed skill text (`$REPO/.claude/skills/jigc/SKILL.md`), `jigc describe`, orientation (from
the main checkout and from inside a worktree, before and after provisioning), the `planning`
composition, and the `create` / `add-task` / `provision` acks carry 0 hits for either.

### The contract

Two different things are in this lead, and only one of them contradicts a contract.

**(1) The resume line — contradicted.** Orientation prints, for a milestone sub-task,
`Run: jigc start --task <sub>` glossed *"resume: re-composes this task's own workflow where it left
off"*. Run verbatim from the checkout that printed it, the binary composes nothing and exits 1.

- `design/surface-contract.md` → *The three laws* → **Law 1 — nothing lies**: "Every claim a
  surface makes is generated from the thing it describes, or asserted against it … behaviour
  claims match knobs." The gloss is a behaviour claim; for this unit kind in this checkout it is
  false.
- The codebase's own rule for this very row: `crates/cli/tests/orientation_active_task.rs`, the doc
  comment on `a_sub_task_is_routed_to_the_milestone_door_and_omits_the_unneeded_consent` — "a route
  this binary's own guard blocks is a route-floor defect" — and `crates/cli/src/render.rs`
  (the orientation active-task render) applies it twice in the same block: the commit directive
  branches on `task.milestone` so it "names the door that runs", and the abandon directive is
  conditioned so it does not "print a route this binary's own guard blocks". The resume directive
  three lines above them is unconditional, and it is the one line of the four whose guard blocks
  it.
- `design/surface-contract.md` → *The route fence* → **P6 — route-followability** states the gap in
  general terms ("The parse fence proves a mechanical route **runs**; it cannot prove it runs
  *here*"), though P6 itself is fenced on `Finding` routes and orientation's `Run:` lines are not
  findings — cited as the stated principle, not as a fence that should have fired.

**(2) The refusal itself — not a defect.** It is designed, routed and pinned (M46 Increment 8 / T4,
`DECISIONS.md` → the B2-2 entries): a sub-task's work happens in its worktree, the refusal names
`jigc milestone provision <m>` and the absolute `cd`, and both run as printed.

**(3) "Names neither door" on orientation, the `planning` tail and the `add-task` ack — true, and
previously ruled no contract.** This is the surviving half of RC-1.0-gate **B2-2**. Its
verification (`completions/artifacts/RC-1.0-gate/findings-verification.md`, the B2-2 block) already
records "`milestone create` prints `next: jigc milestone add-task …`, and `milestone add-task`
prints **no `next:` at all**", and M46's razor refused the fix as "Capabilities or preferences; no
qualified rule" (`completions/artifacts/M46/razor-ledger.md` §3, *B2-2's `next:` half*). Law 2
("Every affordance that is the designated recovery for a state is named by the surfaces that
produce that state") is worded over *recoveries*; the forward step out of "seeded, unstarted" is
not one by that wording, and the recovery for the refusal state *is* named by the surface that
produces it. So on the record this half is a discoverability observation, tier `none` on its own.
What rc.24 adds to that record is the measured consequence: with every forward surface silent, the
only printed path to `provision` is through a refusal, and a blind worker took exactly that path
(arm a rows 40–42) and never met `milestone execute` or its `Spawn:` lines.

### Tier — 3

The predicate is *a surface says something the binary does not do*: the orientation line says the
command "re-composes this task's own workflow where it left off"; from the checkout that printed
it, the binary exits 1 and composes nothing.

- **Not tier 2.** The refusal carries a mechanical route, the route runs at exit 0, and the
  re-run in the worktree composes. Driven end to end above. No dead end.
- **Not tier 1.** Nothing is committed, destroyed or moved: exit 1, HEAD and `git status`
  unchanged, `.jigc/tasks/` untouched, no worktree created.
- The tier rests on half (1) alone. Halves (3) do not raise it and, taken alone, would be `none`.

### Pin

- The refusal and its route: **`pinned-by: start_resume::sub_task_read_doors_keep_the_blanket_base_pin_refusal`**
  (both sub-task read doors, both provisioning states, every emitted `jigc …` span run verbatim) and
  **`pinned-by: anyhow_route_spans::pinned_base_mismatch_routes_by_unit_kind`** (the sub-task arm
  asserts the `jigc milestone provision` route).
- The orientation sub-task row: `orientation_active_task::a_sub_task_is_routed_to_the_milestone_door_and_omits_the_unneeded_consent`
  asserts the `milestone finalize` and `task discard` directives and runs both; it does **not**
  read the resume directive, and nothing asserts the presence or absence of `provision` /
  `execute`. The resume line's bytes are pinned for a **top-level** task only
  (`orientation_active_task::the_active_block_renders_byte_for_byte`).
- The `planning` bytes: pinned by `compose_goldens` over
  `goldens/compose/methodology/start--planning--*.txt` (0 hits for either door in the golden too).
- **`UNPINNED:`** the contradicting cell — orientation's `jigc start --task <sub>` span lifted off
  the rendered bytes and run from the checkout that printed it — has no test; the sibling test lifts
  and runs the other two state-dependent directives of the same row and skips this one. Pinning the
  current pair (line printed, exit 1) would pin the defect as expected output.
- **`UNPINNED:`** `add-task`'s missing `next:` — razor-refused, so there is no sentence to cite; the
  chain fence one verb earlier is
  `milestone::milestone_create_names_its_record_commit_its_path_and_the_next_step`.

### Notes

- **A by-product worth the record:** B2-2's first `UNPINNED` half — "the milestone's own record
  commits are what move HEAD off the pin" — was left unisolated because the pinning arm also makes
  a disjoint commit. This rig makes none, and the refusal fires: the record commits alone do it.
- **Adjacent, not part of L-4 and not scored here:** the same orientation prints
  `Run: jigc start --workflow planning`; run verbatim it exits 1 with
  `workflow 'planning' requires an intent: jigc start "<intent>" --workflow planning`. Driven once,
  on the first rig. It is the same shape (a printed `Run:` line the binary refuses as printed) at a
  different door, and it belongs to whichever lead owns the `planning` front door.
- `jigc milestone provision`'s ack also carries no `next:` (one line plus the footer); the walk's
  only forward pointer to `execute` is `--help` or the `workflow.verb-routed` refusal.
- Source read at the working tree's `crates/` (no diff against the `1.0.0-rc.24` release commit);
  every behavioural statement above is from the installed binary, not from source.
- Nothing under `~/out` or `~/ideas` was modified; no repository edits, no commits outside the two
  `mktemp` rigs.

---

## L-5 · `milestone finalize` run verbatim while every sub-task shows "nothing staged yet" refuses at exit 3 with `milestone.zero-contribution` and a working route; nothing lands, the record stays `active` — **REFUTED**

`tier:` none

**Kind:** product · **Door:** `jigc milestone finalize` · **Binary:** `jigc 1.0.0-rc.24` (installed registry build, release posture)

### The claim

Orientation (`jigc start`) over an open milestone whose sub-tasks are all unstarted prints
`Run: jigc milestone finalize <id>` under every sub-task while each shows
`staged: nothing staged yet` and no worktree exists. The trial did not run that line at that
moment; the lead asks what it does — tier 1 if it lands an empty boundary or closes the
milestone at exit 0, tier 2 if it refuses with no route, none if it refuses and routes.

### Verdict

**REFUTED** as a defect. The precondition reproduces (orientation does print the line three
times at that moment). Run verbatim, the door **refuses at exit 3** with one blocking finding,
`milestone.zero-contribution`, keyed at `milestone:<id>`, carrying a route that names both exits
(`jigc milestone provision <id>` → execute → `git add` → re-run, or `jigc milestone discard <id>`).
Nothing is committed, the milestone record stays `active` on disk and at HEAD, the three
sub-task areas survive, no worktree is created. The route's first span runs at exit 0, and the
re-run after staging lands. This is the scorer's "none if it refuses and routes" branch.

### Repro

```
# setup — fresh rig, installed binary
rig=$(dev/jigc-rig fresh --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit
$JIGC --version                                   # jigc 1.0.0-rc.24
$JIGC milestone create "Bound the service"        # exit 0 · record commit 0131a6f
$JIGC milestone add-task bound-the-service "cap the store at 1000"    # exit 0 · 6b136e3
$JIGC milestone add-task bound-the-service "reject oversize bodies"   # exit 0 · db19c33
$JIGC milestone add-task bound-the-service "limit the id length"      # exit 0 · ba161d8

# the precondition (the lead's origin, reproduced)
$JIGC start                                       # exit 0
#   Active task: cap-the-store-at-1000 … staged:   nothing staged yet        (×3, one per sub-task)
#   Run: `jigc milestone finalize bound-the-service`   — validate + commit: this is a sub-task of
#        milestone `bound-the-service`, whose door is its only commit boundary — `jigc task
#        finalize cap-the-store-at-1000` refuses here                          (×3)
#   no line names `milestone provision` or `milestone execute`
git -C "$REPO" worktree list | wc -l              # 1  (no linked worktree)
git -C "$REPO" rev-parse --short HEAD             # ba161d8

# argv — the printed line, verbatim
$JIGC milestone finalize bound-the-service

# observed — exit 3, stdout empty, stderr:
#   blocking · milestone.zero-contribution — milestone:bound-the-service would land no work: no
#   sub-task's docs promote to the store, and no sub-task worktree holds staged code — the boundary
#   would commit only jigc's own bookkeeping and flip the milestone record to the terminal `joined`,
#   after which the milestone could never be finalized again
#     at: milestone:bound-the-service
#     route: `jigc milestone provision bound-the-service` gives every sub-task a working area (a
#     fresh clone has none; an existing one is reused); execute the sub-tasks, `git add` their work
#     inside their worktrees, then re-run `jigc milestone finalize bound-the-service` — or settle the
#     milestone as abandoned with `jigc milestone discard bound-the-service`
# `--format json` at the same moment: exit 3, one finding, code `milestone.zero-contribution`,
#   key.target `milestone:bound-the-service`, the same route string.

# after
git -C "$REPO" rev-parse --short HEAD             # ba161d8 — unchanged, 6 commits before and after
git -C "$REPO" status --porcelain                 # empty
git -C "$REPO" worktree list | wc -l              # 1
ls "$REPO/.jigc/tasks"                            # the same three sub-task areas
command grep -n status "$REPO/docs/milestone-records/bound-the-service.md"
#   3:status: active   16/23/30:- status: active  — record not flipped
# file list under .jigc/ identical before/after; one content delta: .jigc/index/edges.json
#   (the gitignored derived edge-index cache, `{"stamp": <base>, "edges": []}` after)

# the printed route, followed
$JIGC milestone provision bound-the-service       # exit 0 · "provisioned 3 worktree(s) … at base 0893fe8"
$JIGC milestone finalize bound-the-service        # exit 3 again — provisioned, still nothing staged; HEAD ba161d8
printf 'cap=1000\n' > "$REPO/.jigc/worktrees/cap-the-store-at-1000/cap.txt"
git -C "$REPO/.jigc/worktrees/cap-the-store-at-1000" add cap.txt
$JIGC milestone finalize bound-the-service        # exit 0 · "finalized 8023fa9 … 2 files committed"
#   sub-tasks: cap-the-store-at-1000: 1 code file · limit-the-id-length: nothing staged ·
#              reject-oversize-bodies: nothing staged
```

### The contract

No contract is contradicted; the binary does what its design of record says.

- `design/finalize.md` → *Empty commit* → **The zero-contribution refusal — the milestone
  boundary's own sanctioned block (M47)**: "`finalize` now refuses a boundary that would land no
  work: one blocking `milestone.zero-contribution` keyed at `milestone:<id>`, whose route names
  both honest exits — `jigc milestone provision <id>` … or `jigc milestone discard <id>` … The
  record flip is restored by its guard, so the milestone stays finalizable."
- The orientation line itself ("validate + commit … whose door is its only commit boundary")
  states nothing the binary fails to do: the door validates, and it is the only commit boundary
  for a sub-task. That orientation at this moment names neither `milestone provision` nor
  `milestone execute` is L-4's subject, not this lead's.

### Tier

**none.** Tier 1 needs exit 0 plus loss or harm through a committing door: the exit is 3, HEAD
is unchanged, the record is unflipped, no bytes are lost. Tier 2 needs a dead end: the refusal
carries a route and the route was followed to a landed boundary. Tier 3 needs a surface saying
something the binary does not do: none found on this door at this moment.

### Pin

`pinned-by: commit_rejected_axis::no_committing_door_dresses_an_empty_commit_as_a_rejection`
(group target `g_flow`) — the exact in-checkout shape (`milestone create` → `milestone add-task`
→ `milestone finalize`, no provision, nothing staged), both commit models (`squash: true` and
`squash: false`): exit 3, HEAD untouched, diagnosis "would land no work", the logged finding
`milestone.zero-contribution`.

Also: `milestone_zero_contribution::a_fresh_clone_cannot_silently_land_a_zero_contribution_finalize`
(group `g_milestone`) — the same refusal over a fresh clone: exit 3, route span parses against
the real CLI and leads with `jigc milestone provision <id>`, record `active` on disk and at
HEAD; and `milestone_record_fresh_clone::the_zero_contribution_route_is_followable_from_a_fresh_clone`
— the route followed.

### Notes

- Adjacent, outside this lead's moment and not scored here: once **one** sub-task has staged
  work, the boundary lands at exit 0 and flips **all three** sub-tasks to `joined`, the two
  non-contributors named in the landing manifest as `nothing staged`. That is the design's stated
  visible degrade (`design/finalize.md` → *Empty commit*, the milestone fan-out paragraph, and
  the suite header of `milestone_zero_contribution.rs`, call (b)(ii)); the fully provisioned
  never-contributing-sub-task manifest line is pinned by
  `milestone_zero_contribution::the_landing_manifest_names_the_never_provisioned_sub_task`.
  If the scorer's "closes the milestone with the sub-tasks undone" concern is to be pursued, it
  is this partial-contribution shape, a separate lead with a design sentence already covering it.
- The refusal rewrote one gitignored derived cache (`.jigc/index/edges.json`); no tracked file
  and no task working area changed.
- The rig's invocation log is not enabled, so the logged `exit_code`/`finding_codes` were not
  read here; the pin asserts them.
- Not driven: the `squash: false` knob (the rig ran the pack default); the pin covers both.

---

## L-6 · Orientation's `Project config:` header path is absolute and falls under neither of the two absolute kinds .jigc/AGENT.md names — **CONFIRMED**

`tier:` 3

**Kind:** product · **Door:** `start` (orientation); guide `.jigc/AGENT.md` · **Binary:** `jigc 1.0.0-rc.24` (installed registry build, release posture)

### The claim

Orientation prints `Project config: <absolute path>/.jigc/config`, a third kind of absolute
printed path, where `.jigc/AGENT.md` says only two kinds of printed path are absolute.

### Verdict

**CONFIRMED** — tier 3.

Both halves were driven on a fresh rig:

1. bare `jigc start` exits 0 and its header line carries the absolute path of the repository's
   own `.jigc/config` — in text, in `--format json` (the `header` field), and unchanged from a
   subdirectory;
2. the installed `.jigc/AGENT.md` (line 7) says *"Every path jigc prints … is relative to the
   **repository root** … Two kinds of printed path are absolute instead"* and names them:
   (a) backticked bytes printed to be **run**, (b) a path naming **a checkout that is not the
   repository root**. The header path is neither: the line carries no backtick, and the path is
   inside the one and only checkout (`git rev-parse --show-toplevel` equals the canonical
   `$REPO`; `git worktree list` has one row; `$REPO/.jigc/config` is a directory in it).

What narrows it, and why this is no more than tier 3:

- **The absolute header is deliberate and pinned; the stale half is the preload's sentence.**
  `design/surface-contract.md` → *The printed-path fence (law 1)* admits **three** reasons for a
  standing absolute, and the first one names this very line (*"the team cascade layer's
  cross-project home (`~/.config/jigc`, on the orientation header) … the orientation header is
  composed in the engine, which … has no repo root to be relative to"*). So the product's own
  design doc lists a class the preload's two rules do not reach. Which side moves is not a
  verifier's call.
- **The design's stated reason does not fit the project segment.** *"No repo-relative spelling
  exists"* is true of the `Team config:` segment (a home outside the repository). The
  `Project config:` segment names `$REPO/.jigc/config`, for which the repo-relative spelling
  `.jigc/config` exists — and `design/bootstrap.md` → Orientation output examples and
  `design/worked-examples.md` both sketch the line as `Project config: .jigc/config` (those
  sketches are illustrative notation, not a contract; noted, not relied on).
- **Scope of the header: orientation only.** On the same rig the `Pack: …` header is absent from
  `jigc start "<intent>"` (router), from `jigc start --workflow quick-fix "<intent>"` (compose)
  and from `jigc task validate <id>`. The lead's surface is the bare-`start` orientation, text
  and JSON, and nothing else was found printing it.
- **No dead end follows.** The preload's closing instruction is *"resolve a printed path against
  the repository root"*; an absolute path resolved against anything is itself, so an agent that
  follows the sentence literally still reaches the right directory. Nothing is lost, moved or
  refused. That is why this is not tier 2.
- **Not driven here, so not claimed:** the two other `DeclaredAbsolute` printed classes the
  design admits (`--explain`'s `Pack input:` line under `JIGC_PACK_DIR`; the quoted failed
  `git` invocation in a dirty-worktree message) are likewise outside the preload's two rules by
  reading, but neither was reproduced in this verification.

### Repro

```
# setup — fresh rig, installed binary, release posture
jigc --version                         # -> jigc 1.0.0-rc.24
rig=$(dev/jigc-rig fresh --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit
# (state `fresh` = `jigc setup` only; cwd = $REPO; <REPO> below is the canonical absolute $REPO)

# argv 1 — orientation, text
jigc start
# exit 0; stdout line 3:
#   Pack: dev/1.0.0-rc.24 | methodology/1.0.0-rc.24 · Project config: <REPO>/.jigc/config
# the printed path begins with `/`; the line contains 0 backticks; it is the only
# occurrence of <REPO> in the whole stdout; stderr empty

# argv 2 — orientation, JSON
jigc start --format json
# exit 0; line 4:
#   "header": "Pack: dev/1.0.0-rc.24 | methodology/1.0.0-rc.24 · Project config: <REPO>/.jigc/config",

# argv 3 — from a subdirectory (mkdir -p sub/deep; cd sub/deep)
jigc start
# exit 0; line 3 byte-identical to argv 1

# controls — the path is neither of AGENT.md's two kinds
test -d "$REPO/.jigc/config"                    # true: inside the repository root
git -C "$REPO" rev-parse --show-toplevel        # == <REPO>: it IS the repository root's checkout
git -C "$REPO" worktree list | wc -l            # 1: no other checkout exists

# the surface it contradicts
command grep -n 'Two kinds of printed path are absolute' "$REPO/.jigc/AGENT.md"
#   7:Every path jigc prints — an `at:` locus, a `jigc doc list` row, a path inside a
#     finding's message — is relative to the **repository root**, … Two kinds of printed
#     path are absolute instead, and both are on purpose. First, bytes jigc prints for you
#     to **run** … Second, a path naming a checkout that is not the repository root …
command grep -c 'Project config\|orientation header\|Team config' "$REPO/.jigc/AGENT.md"   # 0

# negative cells — where the header does NOT appear (same rig)
jigc start "add a greeting"                        # exit 0; no `Pack:` line
jigc start --workflow quick-fix "add a greeting"   # exit 0; `task minted: add-a-greeting`; no `Pack:` line
jigc task validate add-a-greeting                  # exit 3 (unfilled slots); no `Pack:` line

# after — nothing committed, nothing moved; the rig holds one minted task and an empty sub/deep
```

Origin, for the record (not the proof): in all three arms invocation #3 is `argv ["start"]`,
exit 0, and each session's stream carries `Project config: /work/.jigc/config` — `/work` being
the session container's repository root (`~/out/RC24-C`, `~/out/RC24-B-frozen-work/turn01`,
`~/out/RC24-A-frozen-work/turn01`).

### The contract contradicted

- **`.jigc/AGENT.md`, the printed-path paragraph (line 7 of the installed guide)** — source
  `crates/cli/src/adapter.rs` → `BOOTSTRAP_PATHS_AND_CWD`. It states a universal (*every path
  jigc prints is relative to the repository root*) with exactly two exceptions; orientation
  prints a path that is absolute and falls under neither.
- Cross-check inside the product's own docs: `design/surface-contract.md` → *The printed-path
  fence (law 1)* admits three reasons for an absolute, the first of which covers this header.
  The const's own doc-comment in `adapter.rs` says the exception is *"stated as a rule, not as
  a count"* because *"a count goes stale the next time a producer joins the class"* — the
  shipped sentence nonetheless opens with a count of rules, and the header's class is outside
  both.

### Tier

**3** — a surface says something the binary does not do. `.jigc/AGENT.md` promises every
printed path is repo-relative save two named kinds; the first screen an agent reads
(`jigc start`) prints an absolute of neither kind. Not tier 2: no posture or route dead-ends —
the absolute path resolves correctly whichever way the reader applies the paragraph. Not
tier 1: exit 0 with no loss, no repository harm, and no committing, destroying or moving door
involved.

### Pin

Each half is pinned on its own; the disagreement between them is not.

- The absolute header: `compose_goldens::sweep_fresh` (and its five sibling sweeps) over
  `crates/cli/tests/goldens/compose/composite/start-orient--start-orient--<state>.txt` and
  `start-orient-json--…` — `Project config: <REPO>/.jigc/config`, `<REPO>` being the harness's
  normalization of the absolute root; and the disposition row
  `("crates/engine/src/cascade.rs", "header", DeclaredAbsolute, …)` checked by
  `repo_relative_paths::every_path_a_door_prints_carries_a_disposition_the_source_backs`.
- The "Two kinds" sentence: the `agent-md--agent-md--<state>.txt` goldens under the same sweeps.

**UNPINNED** as a finding: no test relates the preload's enumeration of absolute kinds to the
`DeclaredAbsolute` rows of `PATH_TEXT_SITES`, so the two pinned surfaces can (and do) disagree
with every suite green. The driven door tables in `repo_relative_paths.rs` do not include bare
`jigc start`, which is why the host-prefix scan never meets this line.

---

## L-7 · The installed pre-commit hook's `jigc validate --format json` lands in the invocation log as an ordinary record with no caller marker — designed, per-invocation behaviour, not a defect — **CONFIRMED**

`tier:` none

**Kind:** product · **Door:** `jigc validate`; `jigc setup` (the pre-commit hook it installs) ·
**Binary driven:** the installed registry build, `jigc --version` → `jigc 1.0.0-rc.24` (release
posture; asserted before the first probe).

### The claim

The installed pre-commit hook writes a `validate --format json` record into
`.jigc/logs/invocations.jsonl` on every commit, with nothing in the record marking it as the
hook's rather than typed.

### Verdict

**CONFIRMED as an observation — tier `none`. It is the designed behaviour, it contradicts no
contract, and it is pinned.** The scorer's conditional (*"unless the log is specified as the
agent's own invocations, then 3"*) resolves to the unconditional arm: the log is specified per
**`jigc` invocation**, not per agent call.

Both halves of the observation reproduce on a fresh rig:

1. With the `invocation-log` knob on, a commit on which the hook runs appends exactly one record
   whose `argv` is `["validate","--format","json"]` — once inside `jigc task finalize`'s own
   commit (landing **before** the finalize record, because a record is written when its process
   exits) and once on a raw `git commit`.
2. That record is byte-for-byte the shape of a typed `jigc validate --format json` record: same
   eight keys, same `argv`, and — on this corpus — every field equal once `timestamp` and
   `duration_ms` are dropped.

**One bound on "every commit":** it is every commit the hook *runs* on. A `git commit --no-verify`
appends nothing (driven, cell C), and `jigc setup`'s own install commit passes `--no-verify` by
recorded design. With the knob off (the shipped default) there is no log at all.

### Repro

```
# setup — fresh rig, installed binary, CLAUDECODE left as the session has it (set) in every cell
rig=$(dev/jigc-rig fresh --binary <installed jigc>) || exit; eval "$rig"; [ -n "$REPO" ] || exit
$JIGC --version                                  # jigc 1.0.0-rc.24
$JIGC config set invocation-log true             # exit 0 (this run itself is not logged: the knob was off when it started)
$JIGC start --workflow single-task "add a greeting helper"      # exit 0, task add-a-greeting-helper
printf 'export function greet(n) { return "hello " + n }\n' > greet.js; git add greet.js
$JIGC doc set-field commit:add-a-greeting-helper#type --value feat --task add-a-greeting-helper   # exit 0
$JIGC doc set-slot commit:add-a-greeting-helper#summary --from-file - --task add-a-greeting-helper <<'EOF'
add a greeting helper
EOF
L=$REPO/.jigc/logs/invocations.jsonl

# cell A — jigc task finalize (the hook fires inside jigc's own commit)
wc -l < $L                                        # 3   (start, set-field, set-slot)
$JIGC task finalize add-a-greeting-helper         # exit 0 — "finalized 0b95b84 — feat: add a greeting helper"
wc -l < $L                                        # 5   — two records appended
tail -2 $L
#  {"timestamp":"2026-10-03T22:56:37Z","argv":["validate","--format","json"],"exit_code":0,"duration_ms":67,"finding_codes":[],"output_bytes":112,"binary_version":"1.0.0-rc.24","error_code":null}
#  {"timestamp":"2026-10-03T22:56:37Z","argv":["task","finalize","add-a-greeting-helper"],"exit_code":0,"duration_ms":602,"finding_codes":["changelog-recording.gate-granted-unused"],"output_bytes":540,"binary_version":"1.0.0-rc.24","error_code":null}

# cell B — raw git commit (the hook fires)
printf 'one\n' > notes.txt; git add notes.txt
git commit -q -m "chore: raw commit"              # exit 0
wc -l < $L                                        # 6   — one record appended
tail -1 $L
#  {"timestamp":"2026-10-03T22:56:49Z","argv":["validate","--format","json"],"exit_code":0,"duration_ms":64,"finding_codes":[],"output_bytes":112,"binary_version":"1.0.0-rc.24","error_code":null}

# cell C — control: raw git commit --no-verify (the hook does not run)
printf 'two\n' >> notes.txt; git add notes.txt
git commit -q --no-verify -m "chore: raw commit, no-verify"     # exit 0
wc -l < $L                                        # 6   — nothing appended

# cell D — control: the same argv, typed
$JIGC validate --format json                      # exit 0
wc -l < $L                                        # 7
tail -1 $L
#  {"timestamp":"2026-10-03T22:56:49Z","argv":["validate","--format","json"],"exit_code":0,"duration_ms":124,"finding_codes":[],"output_bytes":112,"binary_version":"1.0.0-rc.24","error_code":null}

# after
#  records 4 (hook, inside finalize), 6 (hook, raw commit) and 7 (typed) are equal once
#  `timestamp` and `duration_ms` are dropped; every record in the file carries the same eight keys:
#  argv · binary_version · duration_ms · error_code · exit_code · finding_codes · output_bytes · timestamp
#  git log --oneline: c14400d · 4583c79 · 0b95b84 feat: add a greeting helper · 3c830df chore(jigc): install … · f6c80b1 initial
#  the hook, as installed at $REPO/.git/hooks/pre-commit, runs:  report="$("$jigc" validate --format json 2>/dev/null)"
```

The origin agrees with the rig: in `~/out/RC24-C` the log's record #14 is
`["validate","--format","json"]`, exit 0, at the same second as record #15, the `task finalize`
that made the commit.

### The contract

No contract is contradicted; four statements say the log is per `jigc` invocation, and one design
row and one suite count the hook's record on purpose.

- `design/measurement.md` → *The in-repo invocation log (M36)*: the knob *"appends one JSONL
  record per `jigc` invocation"*; item 1 names the event class as *"a jigc invocation"*; item 2
  (*Honest capability boundary*) bounds the log to *"what jigc did (args, exit, findings,
  duration), never what was done around it"*. Who started the process is outside that list. The
  same section declares the record shape closed at eight keys and **additive-only**; none of the
  eight names a caller.
- `crates/cli/guides/QUICKSTART.md` (the adopter guide `jigc setup` embeds): *"with the opt-in
  `invocation-log` knob on, **every** jigc run — these reads included — appends one record"*.
- `crates/cli/packs/dev/config/knobs.yaml`, the knob's own comment: *"appended per `jigc`
  invocation"*.
- `design/assistant-adapter.md` → *The doc↔code backstop (M19)*: *"the hook runs
  `jigc validate --format json`"* on every commit, fired inside `finalize`'s own commit too.
  A hook that runs `jigc`, under a log that records every `jigc` run, writes a record — the
  observed behaviour is the conjunction of two stated rules.
- `completions/artifacts/M54/VERDICT.md`, audit row 4: *"one record per `validate`, none for the
  probe child, at the CLI and the hook door"* — the hook-door record is counted as expected, and
  only `jigc`'s own probe child is excluded from the log (`implementation/module-layout.md` →
  Probe boundary).

One thing a reader could take the other way, stated so it is not missed: the source comment on
that probe-child exclusion in `crates/cli/src/main.rs` says the child is intercepted first *"so
`jigc`'s own probe child is never logged as an agent's call"*. That is a code comment's wording,
not a design rule, and it does not say the log holds only an agent's calls; no design doc, help
text, guide or composed step does. The trial protocol's own §8 describes the log as *"the
product's own record, exact: which door ran, when, and how it exited"*, which is what it is.

What the binary does instead of attributing: it records every `jigc` process that runs under a
knob-on project layer, whoever started it, with no caller field. A reader who needs the worker's
own calls has to subtract the hook's records by position (a `validate --format json` record
directly before a committing door's record, or standing alone beside a raw commit) — which is a
reading rule for the trial tooling, the subject of the tooling leads, not a product contradiction.

### Tier

**none.** No surface says something the binary does not do (tier 3 needs one), no route or posture
dead-ends (tier 2), and nothing is lost or harmed at exit 0 (tier 1): the log is a gitignored
append-only file and the extra record is one more appended line. An origin key would be an
additive change the log's declaration permits, and it is a design choice, not a repair.

### Pin

`pinned-by: probe_failure_doors::the_precommit_hook_records_each_probe_failure`
(`crates/cli/tests/probe_failure_doors.rs`, group `g_finalize`). It drives a raw `git commit`
through the hook `jigc setup` installed with the knob on and asserts the log grew by **exactly
one** record whose `argv` equals `["validate","--format","json"]` — the suite's header states the
reliance outright: *"At the hook door the finding is read from the invocation log."*

The "nothing marks it" half is pinned by the closed key set: the `jigc` crate's unit test
`invocation_log::tests::the_emitted_key_set_is_closed_by_the_destructured_fields`
(`crates/cli/src/invocation_log.rs`) fences the emitted keys at the eight above.

Not pinned by name, as far as a targeted search of `crates/cli/tests` found: the cell A ordering
(the hook's record landing inside a jigc-made commit, ahead of the committing door's own record).
It is the same mechanism as the pinned raw-commit cell.

### Bounds on this verification

- One corpus state (`fresh`), one committing door (`jigc task finalize`) plus the raw commit. The
  lead's other cited ordinals (arm a's milestone doors, arm b) were not re-driven door by door;
  the hook is door-independent — it is git's, and fires on any commit made without `--no-verify`.
- The two pinning tests were read, not run.
- Nothing was modified under `~/out` or `~/ideas`, and nothing was committed in the working
  repository; the rig's commits live in a `mktemp -d` root.

---

## L-8 · The hook's in-flight store sweep reports un-baselined / hash-matches for the commit's own writes; a post-commit validate is clean at every door — documented ordering, read by no surface — **CONFIRMED (as an observation; the defect condition — a post-commit `validate` still reporting the codes — is REFUTED, so no finding)**

`tier:` none

**Kind:** product · **door:** `jigc validate` (as run by the installed `pre-commit` hook) ·
**binary:** `jigc 1.0.0-rc.24` (the installed registry build, release posture)

### The claim

The hook's store-scope `validate`, run while a jigc commit is in flight, reports
`file-state.un-baselined` (the docs that commit is landing) and `file-state.hash-matches`
(the milestone record) as finding codes; whether a `validate` run after the commit is
clean, and what `hash-matches` is as a finding, was unread. Proposed tier: none unless a
post-commit `validate` still reports them — then 3.

### Verdict

**CONFIRMED as an observation — tier `none`: not a defect.** The in-flight codes reproduce
exactly on a fresh rig, at every door the trial hit. The condition the scorer set for a
tier — *a post-commit `validate` still reports them* — is **refuted**: a `validate` run
right after each of the five commits returns `findings: []`, `blocking_probes: []`, exit 0.
No contract is contradicted; the ordering that produces the in-flight reading is the
documented one.

The two unread halves, now read:

- **Post-commit `validate`:** clean after `task finalize`, after `milestone create`, after
  each `milestone add-task`, and after `milestone finalize` (fresh rig). Clean as well on a
  scratch copy of the two session corpora (`~/out/RC24-C`, `~/out/RC24-A-frozen-work/turn02`,
  copied out, originals untouched): `findings: []` on both.
- **What `hash-matches` is:** the store-scope drift check — the doc's on-disk bytes differ
  from the hash in the file-state record. In flight it is graded **`blocking`**
  (`blocking_probes: ["file-state"]`), message *"on-disk content of `<record>` differs from
  the recorded state"*, route *"review the out-of-band edit to `<record>` and re-author it
  through the owning workflow"*. It is the door's own write seen between the write and the
  baseline advance: the door rewrites the record, commits, and only then advances the
  baseline. The advisory lag arm does not apply in flight because the bytes are not yet
  `HEAD`'s blob.

Who reads the in-flight report: nobody. The installed hook discards it except for two
keys (a blocking `doc-code` probe; a `git -C … mv` rename route), prints nothing, exits 0 —
every ack in the repro carries `hook_output: ""`. The only trace is the invocation log's
`finding_codes`, which records every `jigc` invocation, the hook's included. Neither
session's agent ran a store-scope `validate` itself (transcripts: arm c one `task validate`,
zero store-scope; arm a zero in both turns), so every record the lead cites is the hook's.

### Repro

```
SETUP
  rig=$(dev/jigc-rig fresh --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit
  jigc --version                       -> jigc 1.0.0-rc.24
  # the rig's `jigc setup` installed .git/hooks/pre-commit (unmodified throughout)
  jigc config set invocation-log true ; git add .jigc/config/manifest.yaml ; git commit -m "chore: enable the invocation log"
  # the agent variable the co-author trailer keys on: set, held constant across all cells
  # OBSERVER (the verifier's, not jigc's): a .git/hooks/commit-msg that runs
  #   `jigc validate --format json > <tmp>/inflight-N.json` and exits 0 — it reads the same
  #   in-flight window the pre-commit hook reads, so the full report is on disk; each commit
  #   therefore logs TWO in-flight validate records (hook + observer), identical in codes and bytes.

CELL 1 — task finalize (two docs promoted)
  jigc start "Bound what the service will accept" --workflow planning --format json      exit 0
  jigc doc create roadmap --title Roadmap --task bound-what-the-service-will             exit 0
  jigc doc author roadmap --from-file - --task bound-what-the-service-will               exit 0
  jigc doc create decisions-log --title 'Decisions Log' --task bound-what-the-service-will   exit 0
  jigc doc author decisions-log --from-file - --task bound-what-the-service-will         exit 0
  jigc doc set-field commit:bound-what-the-service-will#type --value docs --task …       exit 0
  jigc doc set-slot  commit:bound-what-the-service-will#summary --from-file - --task …   exit 0
  jigc task finalize bound-what-the-service-will --format json                           exit 0
    ack: committed.hash 44ef767, promoted [docs/decisions-log.md, docs/roadmap.md], hook_output ""
  IN FLIGHT (log + observer):  exit 0, output_bytes 1208
    finding_codes ["file-state.un-baselined","file-state.un-baselined"]   blocking_probes []
    advisory · "committed doc `docs/roadmap.md` is not yet baselined in the file-state record"
    route: "no action needed — the doc is baselined on its next author or finalize"
    git status in flight: A docs/decisions-log.md / A docs/roadmap.md ; no .jigc/state/file-state.json yet
  AFTER:  jigc validate --format json   exit 0
    {"blocking_probes":[],"findings":[],"report_only":true,"schema_version":3,"scope":"store"}
    .jigc/state/file-state.json now carries both docs' hashes

CELL 2 — milestone create
  jigc milestone create "Bound what the service will accept" --format json               exit 0   (9304201, hook_output "")
  IN FLIGHT:  exit 0, output_bytes 766, finding_codes ["file-state.un-baselined"]  (the new record), advisory, blocking_probes []
  AFTER:  jigc validate --format json   exit 0, findings [], blocking_probes []

CELL 3 / 4 — milestone add-task (twice)
  jigc milestone add-task bound-what-the-service-will "Cap the store at 1000 distinct series" --format json   exit 0   (e0ce00e, hook_output "")
  jigc milestone add-task bound-what-the-service-will "Reject a long series name" --format json               exit 0   (0a85382, hook_output "")
  IN FLIGHT (each):  exit 0, output_bytes 836, finding_codes ["file-state.hash-matches"]
    blocking · blocking_probes ["file-state"] · report_only true
    "on-disk content of `docs/milestone-records/bound-what-the-service-will.md` differs from the recorded state"
    route: "review the out-of-band edit to `docs/milestone-records/bound-what-the-service-will.md` and re-author it through the owning workflow"
    git status in flight: M docs/milestone-records/bound-what-the-service-will.md
  AFTER (each):  jigc validate --format json   exit 0, findings [], blocking_probes []

CELL 5 — milestone finalize (two sub-tasks, squash default)
  jigc milestone provision bound-what-the-service-will                                   exit 0
  (in each worktree) jigc start --task <sub-id> ; write one file ; git add it            exit 0
  jigc milestone finalize bound-what-the-service-will --format json                      exit 0   (73bbb19, 3 paths, hook_output "")
  IN FLIGHT:  exit 0, output_bytes 836, finding_codes ["file-state.hash-matches"], blocking, blocking_probes ["file-state"]
  AFTER:  jigc validate --format json   exit 0, findings [], blocking_probes []

THE LOG, the rows that matter (argv · exit · finding_codes · output_bytes)
  validate --format json · 0 · [un-baselined, un-baselined] · 1208      <- hook
  validate --format json · 0 · [un-baselined, un-baselined] · 1208      <- observer
  task finalize …        · 0 · [staged-copy, staged-copy]   · 1625
  validate --format json · 0 · []                           · 112       <- post-commit
  validate --format json · 0 · [un-baselined] · 766  (x2)   then milestone create · 0 · []   then validate · 0 · [] · 112
  validate --format json · 0 · [hash-matches] · 836  (x2)   then milestone add-task · 0 · [] then validate · 0 · [] · 112   (twice)
  validate --format json · 0 · [hash-matches] · 836  (x2)   then milestone finalize · 0 · [] then validate · 0 · [] · 112

MATCH WITH THE TRIAL
  arm a #30/#31 (un-baselined x4 before task finalize), #32/#33 (un-baselined x1, 766 bytes, before milestone create),
  #34-#39 (hash-matches, 836 bytes, before each add-task), #47/#48 (hash-matches, 836 bytes, before milestone finalize);
  arm c #14/#15 (un-baselined x1 before task finalize). Same shape, same byte counts where the slug is the same.

SESSION CORPORA, read through a scratch copy (rsync out of ~/out; nothing under ~/out written)
  copy of ~/out/RC24-C                    HEAD d107e87   jigc validate --format json  exit 0  findings []  blocking_probes []
  copy of ~/out/RC24-A-frozen-work/turn02 HEAD cb4a90f   jigc validate --format json  exit 0  findings []  blocking_probes []
```

### The contract

None is contradicted. What the documents say, and the binary does:

- **The order is declared.** `design/finalize.md` → the shape at the top: `→ git commit
  (pre-commit hook may reject) → post-commit (invalidate caches, update hashes, cleanup)` —
  hashes advance after the commit lands. `design/team-ready-state.md` → *Each per-op record
  commit is a transaction with a rollback boundary (M47)*: a rejected commit restores the
  record "re-baselining nothing", which is only possible because the baseline moves after
  the commit. A hook runs between the two, so it reads the pre-advance record by construction.
- **`un-baselined` is declared informational.** `design/validation.md` → *read-only
  file↔CLI-state at store scope — detect, never absorb* → **UNKNOWN (un-baselined) ≠ clean**:
  reported "as a distinct un-baselined / not-yet-tracked outcome … informational, not an
  error — it does not flip the exit code"; and `file-state.un-baselined` is in the
  gates-nowhere set (same doc, condition (1)).
- **`hash-matches` is declared.** Same section: the twin "compares each committed managed
  doc's on-disk content hash to its recorded `FileStateRecord` and reports drift
  (`file-state.hash-matches`)", with the store-scope route "review / re-author through the
  owning workflow"; inventory row: `hash-matches` · blocking · tunable. The lag arm (M55)
  needs bytes equal to `HEAD`'s blob, which is false until the commit exists.
- **The hook is declared deaf to it.** `design/validation.md` → *The M19 pre-commit
  backstop stays doc↔code-keyed for content* ("M20 did **not** broaden the warn set");
  `design/finalize.md` → *The M19 doc↔code backstop fires here (and is warn-only by
  construction)*; the hook's own header: "prints a warning ONLY when a doc-code check raised
  a BLOCKING finding. Always exits 0".
- **The log is declared to record it.** `design/measurement.md` → the invocation log
  "appends one JSONL record per `jigc` invocation" — the hook's invocation is one.

### Tier

**none.** Exit 0 with no loss (every doc and record lands and is baselined: after-state
shown), no dead end (nothing is routed to the agent; no door refuses), and no surface that
reaches a reader says something the binary does not do — the in-flight report is discarded
by the hook and the log carries codes only.

Residue, stated and not claimed as a finding: for the length of the commit the store sweep
calls jigc's own record write an "out-of-band edit" at `blocking`. No shipped surface reads
that. A user-authored hook that failed a commit on a non-empty `blocking_probes` would
meet it on every `milestone add-task` / `milestone finalize` commit; **not driven** here,
and no document recommends such a hook (the docs call an exit-code- or block-driven hook
wrong-way-round).

### Pin

- The steady state is pinned: `g_finalize::l1_pull_absorption::after_the_landed_finalize_the_store_sweep_no_longer_reports_the_lag`
  (a landed finalize re-baselines, the next store sweep carries no `hash-matches`);
  `g_milestone::milestone_record_reconcile::clean_record_add_task_stays_green` (the baseline
  advances with each landed record commit — the next door sees it in sync);
  `g_compose::precommit_hook_acceptance::commit_is_silent_on_advisory_only_doc_code_beside_another_blocking_probe`
  (the hook says nothing about a blocking probe that is not `doc-code`).
- **UNPINNED:** the in-flight reading itself — no suite drives a jigc committing door with
  the installed hook and reads what the hook's `validate` reports mid-commit (the
  `un-baselined` / blocking `hash-matches` codes in the log), nor asserts a clean store-scope
  `validate` immediately after a landed `milestone add-task` / `milestone finalize`.

### Bounds on this verification

- One rig, one run per cell, macOS, methodology + dev packs as `jigc setup` installs them.
- Cell 1 used the `planning` workflow with two docs; the trial's arm c used
  `architecture-documentation` with one arch-doc — same door, same mechanism, not the same
  doctype.
- The observer `commit-msg` hook is the verifier's addition; its records are the second of
  each in-flight pair in the log and match the hook's record in codes and byte count.

---

## L-9 · The `task amend` composed text prints a bare `git log -1 --format=%s` — reproduced, but the `git -C` rule is scoped to spans carrying a path, so no contract is contradicted — **PARTIAL**

`tier:` none

**Kind:** product · **Door:** `jigc task amend` (`workflow:amend`, step `amend-message`);
`guide:AGENT.md` · **Binary driven:** the installed registry build, `jigc --version` →
`jigc 1.0.0-rc.24` (release posture; asserted before the first probe).

### The claim

The `task amend` composed text prints a bare `` `git log -1 --format=%s` `` for the worker to
run, where `.jigc/AGENT.md` says a `git` command jigc prints leads with
`git -C <the repository's absolute path>`.

### Verdict

**PARTIAL — the observation reproduces exactly; the contradiction does not hold. Tier `none`.**

1. **The observation (confirmed).** On a fresh rig whose `HEAD` is a jigc task commit,
   `jigc task amend` exits 0 and its composed text carries two backticked `git` spans —
   `` `git log -1 --format=%s` `` (to run) and `` `git commit --amend` `` (prose) — and **zero**
   `git -C`. Same on the `--format json` door the session used (arm b #19), in the envelope's
   `text`.
2. **The contradiction (refuted).** The `-C` rule is scoped to spans that carry a **path**. The
   span carries none, and every statement of the rule — AGENT.md's own sentence read whole,
   MIGRATING.md, the design doc, the engine's fence — admits it bare. See *The contract*.
3. **The claimed remedy would be wrong in one driven cell.** `jigc task amend` pins the `HEAD` of
   the checkout it is run in. Minted from a linked worktree, the pinned commit is that
   worktree's; `git -C <the repository's absolute path> log -1 --format=%s` reads the **main**
   checkout's `HEAD`, a different commit. The bare span, run where the task was minted, reads
   the pinned one.

The session itself did not raise this. Its debrief (item 4) says only that the check *"was
literally the command jigc's own `task amend` output told me to run"*; the `-C` contrast is the
scorer's reading of AGENT.md.

### Repro

```
# setup — fresh rig, installed binary; CLAUDECODE left as the session has it (set) in every cell
rig=$(dev/jigc-rig fresh --binary <installed jigc>) || exit; eval "$rig"; [ -n "$REPO" ] || exit
$JIGC --version                                   # jigc 1.0.0-rc.24
$JIGC start "add a greeting file" --workflow dev-task          # exit 0, task add-a-greeting-file
mkdir -p src/deep; echo hello > src/deep/greeting.txt; git -C "$REPO" add -- src/deep/greeting.txt
$JIGC doc set-field commit:add-a-greeting-file#type --value feat --task add-a-greeting-file   # exit 0
$JIGC doc set-slot commit:add-a-greeting-file#summary --from-file - --task add-a-greeting-file <<'EOF'
add a greeting file
EOF
$JIGC task finalize add-a-greeting-file           # exit 0 — "finalized bf2cf6b — feat: add a greeting file"

# cell A — the lead's own repro, driven from a subdirectory
cd "$REPO/src/deep"
$JIGC task amend > <tmp>/amend.txt                # exit 0, stderr empty, 87 lines
#  task minted: amend-bf2cf6b
#
#  amending: bf2cf6b "feat: add a greeting file"
grep -n 'git ' <tmp>/amend.txt
#  19:pinned (`git log -1 --format=%s`) and stop if it is one of them: a milestone
#  77:commits nothing else: `git commit --amend` would fold the whole index into the
grep -c 'git -C' <tmp>/amend.txt                  # 0
git log -1 --format=%s                            # exit 0 — "feat: add a greeting file" (the pinned commit, from the subdirectory)

# cell B — which HEAD the task pins, and what an aimed span would read (second fresh rig)
echo one > a.txt; git -C "$REPO" add -- a.txt; git -C "$REPO" commit -q -m "feat: main-side subject"   # main HEAD fc54817
git -C "$REPO" worktree add -q -b side <tmp>/wt HEAD~1
echo two > <tmp>/wt/b.txt; git -C <tmp>/wt add -- b.txt; git -C <tmp>/wt commit -q -m "feat: worktree-side subject"   # wt HEAD 0a7754c
cd <tmp>/wt; $JIGC task amend                     # exit 0
#  task minted: amend-0a7754c
#
#  amending: 0a7754c "feat: worktree-side subject"
#    that is the `HEAD` of the linked worktree at `<tmp>/wt` on branch `side` — not of the main checkout jigc's workbench binds to
git log -1 --format=%s                            # feat: worktree-side subject   <- bare, where minted: the pinned commit
git -C "$REPO" log -1 --format=%s                 # feat: main-side subject       <- aimed at "the repository": another commit
cd "$REPO"; git log -1 --format=%s                # feat: main-side subject       <- bare, from the other checkout: another commit

# cell C — the door the session used
cd "$REPO"; $JIGC task amend --format json        # exit 0, keys [task, text], task amend-fc54817
#  backticked git spans in .text: `git log -1 --format=%s` · `git commit --amend`;  "git -C" count 0

# after — both rigs: one (cell A) / two (cells B, C) live amend tasks, no commit made by any amend door,
# trees untouched. Origin read-only: ~/out/RC24-B-frozen-work/turn02 invocation 19 is
# `task amend --format json`, exit 0, and its stream carries the same bare span once.
```

### The contract

No contract is contradicted. Four statements of the rule, all scoped to a path:

- **`.jigc/AGENT.md`** (installed text, the *paths and cwd* paragraph; source
  `crates/cli/src/adapter.rs` → `BOOTSTRAP_PATHS_AND_CWD`): *"a backticked command carries
  absolute paths **wherever a relative one would resolve against your directory rather than the
  repository's** — a `git` command leads with `git -C <the repository's absolute path>` …"*. The
  `-C` clause is the example of the qualified statement before the dash; the span has no path
  for a directory to resolve.
- **`crates/cli/guides/MIGRATING.md`** → *One thing to know before you read a route*: *"Every
  `git` command jigc prints **with a path in it** leads with `git -C …`"*.
- **`design/surface-contract.md`** → *The printed-path fence (law 1)*: the rule binds *"every
  operator-facing backticked `git` span **carrying a path operand**"* and admits three shapes —
  aimed, **no operand**, and a declared non-path command.
- **`crates/engine/src/finding.rs`** → `GIT_NON_PATH_COMMANDS` declares `log` with the reason
  *"its operands are revisions and revision ranges — `git log a..b`, `git log -1` — which name
  commits, not files"*; `unaimed_git_span` admits the span on two counts (every token after the
  command is a flag; `log` is declared and there is no `--`).

The step's choice of a runnable command is itself deliberate and tested:
`task_amend::every_composing_door_names_the_commit_the_amend_repairs` asserts the composed step
does not say *"in the ack above"*, because the `--format json` arm carries no `amending:` block.

**What the binary does instead:** prints the pinned short sha and subject in the `amending:`
block on the text doors, and in the step body names a pathless, read-only git command that
answers from any directory of the checkout the task was minted in.

### Tier

**`none`.** Tier 3 needs a surface that says something the binary does not do; AGENT.md's
sentence, read whole, promises `-C` where a relative path would resolve against the reader's
directory, and this span has no path. No loss, no dead end.

**A narrower cell, outside the lead as worded and not tiered here** (cell B, last line): the step
says the command shows *"the subject line of the commit this task pinned"*, and the span names no
revision, so it reads `HEAD` of whichever **checkout** the reader stands in. Mint in a linked
worktree, run it from the main checkout (or the reverse), and it prints another commit's subject.
AGENT.md's `-C <the repository's absolute path>` would not repair that — it is the form that
misreads in cell B; the pinned sha (`git log -1 --format=%s <sha>`) would. Exposure is small: the
mint ack names the worktree outright, the text doors already print the pinned subject, and
finalize refuses if `HEAD` moved.

**A wording note, not a defect:** AGENT.md's dash clause, lifted out of its sentence, reads as a
universal (*"a `git` command leads with `git -C`"*), which is how the scorer read it;
MIGRATING.md's *"with a path in it"* is the unambiguous spelling of the same rule.

### Pin

`UNPINNED` for the span's spelling: no golden holds the `amend` composed body — the ten
`goldens/compose/dev/{start,workflow-preview}--amend--*.txt` files pin the `workflow.verb-routed`
refusal of composing `amend` by name (exit 1), not the text `jigc task amend` composes — and no
test asserts the step carries `git log -1 --format=%s`.

What is pinned around it:

- `git_span_aim::the_predicate_admits_aimed_operandless_and_declared_non_path_spans_only` — the
  fence admits an operand-less span and a `git log` revision span bare. It is installed on the
  `Route` constructors only; pack step prose is not under it (the span would pass if it were).
- `task_amend::every_composing_door_names_the_commit_the_amend_repairs` — the `amending:` block
  on all three composing doors, and that the step points at a command rather than at an ack.
- `compose_goldens` → `goldens/compose/composite/agent-md--agent-md--*.txt` — AGENT.md's sentence,
  byte-for-byte.

---

## L-10 · The amend step says the amended message carries only the trailer items you add (example: Co-Authored-By); the binary lands `Co-Authored-By: Claude <noreply@anthropic.com>` with none added — **CONFIRMED**

`tier:` 3

**Kind:** product · **Door:** `jigc task amend` (the composed `step:amend-message`) → `jigc task finalize <amend-task>` · **Binary:** `jigc 1.0.0-rc.24` (the installed registry build, release posture)

### The claim

The text `jigc task amend` composes says *"the amended message carries only the trailer items
you add here, so re-add any the old message had that still apply"* and uses `Co-Authored-By`
as its example, yet the amended commit carries `Co-Authored-By: Claude <noreply@anthropic.com>`
with no trailer item added.

Origin: debrief-b item 2, second bullet; arm b invocations #19–#26 (`task amend`, two
`set-field`, two `set-slot`, `doc show`, `validate`, `task finalize` — no `doc add-item
…#trailers` record), commit `4b80510` in `~/out/RC24-B-frozen-work/turn02`, whose message ends
`Co-Authored-By: Claude <noreply@anthropic.com>`.

### Verdict

**CONFIRMED** — tier 3.

The sentence is false for exactly the key it uses as its example. Five cells on one fresh rig,
each amend authoring `type` and `summary` only (plus, in E, nothing more):

| cell | HEAD before the amend | `CLAUDECODE` at the amend | trailer items authored | amended commit's trailers |
|---|---|---|---|---|
| A (the lead) | carries the co-author trailer | set | none | `Co-Authored-By: Claude <noreply@anthropic.com>` |
| B | carries no trailer | set | none | `Co-Authored-By: Claude <noreply@anthropic.com>` |
| C | carries the co-author trailer | unset | none | `Co-Authored-By: Claude <noreply@anthropic.com>` |
| D (control) | carries no trailer | unset | none | none |
| E | `Refs: TKT-1` (authored item) + the co-author trailer | set | none | `Co-Authored-By: Claude <noreply@anthropic.com>` — `Refs: TKT-1` gone |

So the sentence holds for a trailer the commit doc authored (E: `Refs` is dropped unless
re-added) and fails for the adapter's trailer, which lands whenever jigc runs under the agent
(A, B) **or** `HEAD` already carries it (C — no agent variable needed). D is the falsifying
control: with neither basis, no trailer lands, so the line in A–C is jigc's, not git's or a
hook's.

The binary is doing what its design says. The defect is the surface: the step text predates
the co-author trailer (the step file's last change is the M54 pack move; the commit that added
the trailer did not touch it) and was never revised.

### Repro

```
# setup — CLAUDECODE: set, non-empty (held for A, B, E; cleared with `env -u CLAUDECODE` for C, D)
jigc --version                                   # jigc 1.0.0-rc.24
rig=$(dev/jigc-rig fresh --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit
# $REPO = <tmp>/jigc-rig-fresh-XXXXXX/repo ; HEAD = "chore(jigc): install jigc workspace config"

# an ordinary task commit, made under the agent
jigc start --workflow quick-fix "add a greeting file" --format json      # exit 0, task add-a-greeting-file
printf 'hello\n' > greeting.txt; git add greeting.txt
jigc doc set-field commit:add-a-greeting-file#type --value feat --task add-a-greeting-file      # exit 0
jigc doc set-slot  commit:add-a-greeting-file#summary --from-file - --task add-a-greeting-file <<'EOF'
add a greeting file
EOF
jigc task finalize add-a-greeting-file                                   # exit 0 → 85b1733
git log -1 --format='%(trailers)'
#   Co-Authored-By: Claude <noreply@anthropic.com>

# cell A — argv
jigc task amend                                                          # exit 0, "task minted: amend-85b1733"
#   composed text, lines 58–62:
#     Trailers are re-authored too — the amended message carries only the trailer items
#     you add here, so re-add any the old message had that still apply:
#
#     jigc doc add-item commit:amend-85b1733#trailers --title Co-Authored-By --task amend-85b1733
#     jigc doc set-field commit:amend-85b1733#trailers/<id>/value --value "Name <email>" --task amend-85b1733
jigc doc set-field commit:amend-85b1733#type --value feat --task amend-85b1733            # exit 0
jigc doc set-slot  commit:amend-85b1733#summary --from-file - --task amend-85b1733 <<'EOF'
add the greeting file
EOF
jigc doc show commit:amend-85b1733 --task amend-85b1733                  # exit 0; "## Trailers" is empty
jigc doc show commit:amend-85b1733 --task amend-85b1733 --format json    # /sections/trailers => []
jigc task finalize amend-85b1733                                         # exit 0

# observed
#   no findings — the task validates clean
#   amended 85b1733 → 327147f — feat: add the greeting file
#     the tree and the author are unchanged; the message is re-authored and the committer becomes you, now

# after
git log -1 --format='%B'
#   feat: add the greeting file
#
#   Co-Authored-By: Claude <noreply@anthropic.com>
git log -1 --format='%(trailers:key=Co-Authored-By,valueonly)'           # one line: Claude <noreply@anthropic.com>

# cell C — same HEAD (327147f, carries the trailer), every jigc call under `env -u CLAUDECODE`
env -u CLAUDECODE jigc task amend --format json                          # exit 0, task amend-327147f
env -u CLAUDECODE jigc doc set-field commit:amend-327147f#type --value feat --task amend-327147f
env -u CLAUDECODE jigc doc set-slot  commit:amend-327147f#summary --from-file - --task amend-327147f   # "add the greeting file again"
env -u CLAUDECODE jigc task finalize amend-327147f                       # exit 0 → 112b26f
#   trailers: [Co-Authored-By: Claude <noreply@anthropic.com>]

# cell D (control) — a commit made with CLAUDECODE unset (c48d6d1 "feat: add two", trailers: []),
# then the same three-write amend, all under `env -u CLAUDECODE`
#   finalize exit 0 → 33e4932 "feat: add two, reworded" ; trailers: []

# cell B — HEAD = 33e4932 (no trailer), the same three-write amend with CLAUDECODE set
#   finalize exit 0 → ac2ab6f "feat: add two, reworded twice" ; trailers: [Co-Authored-By: Claude <noreply@anthropic.com>]

# cell E — a task commit under the agent with one authored item
jigc doc add-item  commit:third-file#trailers --title Refs --task third-file              # exit 0, item "refs"
jigc doc set-field commit:third-file#trailers/refs/value --value "TKT-1" --task third-file
jigc task finalize third-file                                            # exit 0 → 089b8cf
#   trailers: [Refs: TKT-1, Co-Authored-By: Claude <noreply@anthropic.com>]
jigc task amend --format json ; type + summary only ; jigc task finalize amend-089b8cf    # exit 0 → 03ec452
#   trailers: [Co-Authored-By: Claude <noreply@anthropic.com>]
```

### The contract

The surface that is contradicted is the composed step's own words —
`crates/cli/packs/dev/steps/amend-message.yaml`, lines 54–58 (the `step:amend-message` body
`jigc task amend` prints): *"the amended message carries only the trailer items you add here"*,
followed by a `Co-Authored-By` example.

What the binary does instead is the designed behaviour, stated in two places:

- `design/assistant-adapter.md` → *The co-author trailer* → **The amend arm**: the message is
  re-authored from scratch, "so the trailer is re-derived — and **carried over** when `HEAD`'s
  message already carries the profile's trailer"; and the section's opening rule — the trailer
  "is the adapter's, applied when the message is handed to git, never a schema item".
- `design/finalize.md` → the amend arm, *Why the door exists at all, and why it does not
  parse*: "The one carry is the adapter's own co-author trailer, which is not part of the doc:
  when `HEAD`'s message carries it, the rewritten message carries it too".

The step text states a rule ("only the items you add") that those two sections make false, so
the governing law is `design/surface-contract.md` → **Law 1 — nothing lies** ("Every claim a
surface makes is generated from the thing it describes, or asserted against it").
`jigc task amend --help` is not contradicted: it says only that HEAD's message is not read
back into the doc and that the message is authored from scratch.

### Tier

**3** — a surface says something the binary does not do. No loss and no repository harm: the
tree is untouched (the amend's own invariant), the trailer that lands is true to the design,
and the reflog keeps the superseded commit. Not tier 2: every route in the step works and the
finalize lands at exit 0. The cost is the one the debrief names — a reader who trusts the
sentence believes the co-author line is theirs to keep or drop through `#trailers`, and it is
not: no `doc` write removes it under the agent or over a `HEAD` that carries it. Following the
example literally with the profile's address is harmless (the seam de-duplicates by address);
with a different address it lands a second co-author line beside the adapter's. That last
sentence is read from the design's de-duplication rule and its unit tests, not driven here.

### Pin

The **binary's** behaviour is pinned:
`pinned-by: agent_co_author::amend_preserves_the_trailer_without_duplicating_it` (cells A and
C — an agent's and a human's amend of an agent's commit each land the trailer once; the commit
doc is filled with type and summary only), with
`agent_co_author::a_human_amend_of_a_human_commit_lands_no_trailer` for cell D and the
`jigc task finalize (amend)` row of
`agent_co_author::every_committing_door_carries_the_agent_trailer_only_under_the_agent` for
the under-agent cell (group target `g_finalize`).

The **sentence** is `UNPINNED`: no suite reads the composed amend text against the commit
seam. The phrase appears in one file in the repository, the step itself; the twelve
`goldens/compose/dev/*--amend--*.txt` files pin the *refusal* of composing `amend` by name
(`workflow.verb-routed`, exit 1), not the text `jigc task amend` prints, so the stale claim
has no fence that would have gone red when the trailer landed.

### Notes

- The lead's wording ("appends … with no trailer item added") is exact; the scorer's repro
  (under an agent) is cell A. Cells B and C widen it: the session basis and the `HEAD`-carry
  basis each suffice on their own.
- Arm b's commit `4b80510` shows `Refs: TKT-221` above the co-author line with no `add-item`
  in the log: that line is a body paragraph shaped like a trailer (invocation #23 is a
  `set-slot …#body`), which the seam then appended into. That is the lossy-render ambiguity
  `design/finalize.md` already records, not part of this lead.
- The rig's invocation log is off by pack default (`invocation-log = false`), so the "no
  add-item record" half is carried here by `doc show … --format json` → `/sections/trailers => []`
  before the finalize, rather than by a log.
- Not driven: a profile without the `co-author` key; a non-Claude assistant; the
  different-address double-line case named under Tier.

---

## L-11 · Composed commit steps print the bare `#type`/`#scope` alias while `doc schema commit` — named in the same step as the authority on "every address a write can take" — lists only `#header/type`/`#header/scope`; both spellings write the same field at exit 0, so the worker's "would have failed" is refuted — **PARTIAL**

`tier:` 3

**Kind:** product · **Door:** `jigc doc set-field`; `jigc doc schema` · **Binary:** `jigc 1.0.0-rc.24` (installed registry build, release posture)

### The claim

Two halves, and the scorer already separated them:

1. **The surface disagreement.** A composed commit step prints
   `jigc doc set-field commit:<task>#type …` and `…#scope …`, while `jigc doc schema commit`
   lists only `commit:<slug>#header/type` and `commit:<slug>#header/scope` — and the same
   composed text calls that schema read *"the authority on what you write into it — … and
   every address a write can take"*.
2. **The worker's stronger claim** (debrief, arm b, item 2): pasting the workflow's own
   printed command verbatim *"would have failed"*.

### Verdict

**PARTIAL.**

- Half 1 is **CONFIRMED** on a fresh rig: both printed surfaces say what the lead says they
  say, on two composed steps from two packs (`dev-task`'s commit step, methodology pack; the
  `task amend` message step, dev pack).
- Half 2 is **REFUTED**. Both spellings exit 0, both write the same header field (a write
  through one is overwritten by a write through the other and read back in one `type:` line),
  and a commit doc whose header was written **only** through the bare spellings finalizes at
  exit 0 with the subject `fix(summary): …`. The falsifying datum from the trial itself: every
  bare-form call in the three arms' invocation logs exits 0 (arm a #25–#26, arm b #20–#21,
  arm c #11), there is no non-zero `set-field` row in any arm, and the worker never ran the
  form it says would fail in the turn it describes (arm b #10–#11 are the qualified form) —
  the claim is an inference from two surfaces disagreeing, not an observation.

The bare `#<field>` form is a deliberate, tested alias — not an accident the binary tolerates.
The binary names it itself on the miss path: *"the single-hop `#<field>` form searches every
declared section"*.

### Repro

```
# setup — fresh rig, installed binary; CLAUDECODE set and held constant throughout
rig=$(dev/jigc-rig fresh --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit
$JIGC --version                                   # jigc 1.0.0-rc.24
T=fix-get-summary-with-empty

# 1 — the composed step (methodology pack, workflow dev-task)
$JIGC start --workflow dev-task "Fix GET /summary/ with empty series name (TKT-212)"   # exit 0
#   Run: `jigc doc set-field commit:fix-get-summary-with-empty#type --value <TYPE> --task fix-get-summary-with-empty`
#   The `commit` schema is the authority on what you write into it — its required
#   slots and fields, each field's enum members, and every address a write can take:
#   jigc doc schema commit
#   Run: `jigc doc set-field commit:fix-get-summary-with-empty#scope --value <SCOPE> --task fix-get-summary-with-empty`

# 2 — the schema read the step names as the authority
$JIGC doc schema commit                           # exit 0
#   fields:
#     - type: enum [feat|fix|…|revert] (section: header) (set-field: commit:<slug>#header/type) *
#     - scope: string (section: header) (set-field: commit:<slug>#header/scope)
#     - implements: ref -> spec (section: header) (set-field: commit:<slug>#header/implements)
#   no line carries `#type` or `#scope` bare

# 3 — both spellings, one task, one field
$JIGC doc set-field "commit:$T#type" --value fix --task $T            # exit 0
#   set commit:fix-get-summary-with-empty#type = fix
$JIGC doc show "commit:$T" --task $T                                  # exit 0 ·  type: fix
$JIGC doc set-field "commit:$T#header/type" --value feat --task $T    # exit 0
#   set commit:fix-get-summary-with-empty#header/type = feat
$JIGC doc show "commit:$T" --task $T                                  # exit 0 ·  type: feat   (one line, overwritten)
$JIGC doc set-field "commit:$T#scope" --value summary --task $T       # exit 0
$JIGC doc set-field "commit:$T#header/scope" --value api --task $T    # exit 0
$JIGC doc show "commit:$T" --task $T --format json                    # exit 0
#   "fields": { "scope": "api", "type": "feat" }

# 4 — negative controls: the door does refuse an address it does not take
$JIGC doc set-field "commit:$T#nosuch" --value x --task $T            # exit 1
#   blocking · write.unknown-field — no field "nosuch" declared on any section of `commit`
#     (the single-hop `#<field>` form searches every declared section)
#     route: `jigc doc schema commit` to see the declared shape, then re-run the write at a declared address
$JIGC doc set-field "commit:$T#header/nosuch" --value x --task $T     # exit 1 · write.unknown-field
$JIGC doc set-field "commit:$T#body/type" --value x --task $T         # exit 1 · write.unknown-field

# 5 — the bare spelling carried to the commit boundary
$JIGC doc set-field "commit:$T#type" --value fix --task $T            # exit 0
$JIGC doc set-field "commit:$T#scope" --value summary --task $T       # exit 0
$JIGC doc set-slot "commit:$T#summary" --from-file - --task $T <<'EOF'
reject an empty series name
EOF
#                                                                     # exit 0
printf 'probe\n' > probe.txt; git -C "$REPO" add probe.txt
$JIGC task finalize $T                                                # exit 0
#   finalized 4723411 — fix(summary): reject an empty series name
git -C "$REPO" log -1 --format=%s
#   fix(summary): reject an empty series name

# 6 — the second composed step (dev pack, the amend message step)
$JIGC task amend                                                      # exit 0
#   Run: `jigc doc set-field commit:amend-4723411#type --value <COMMIT_TYPE> --task amend-4723411`
#   The `commit` schema is the authority on what you write into it — its required
#   slots and fields, each field's enum members, and every address a write can take:
#   jigc doc schema commit
#   jigc doc set-field commit:amend-4723411#scope --value <area> --task amend-4723411

# after — the header the bare spellings wrote is the header git carries; nothing lost,
#         nothing refused, no route dead-ended
```

The trial's own logs, read back (each out-dir's `.jigc/logs/invocations.jsonl`, parsed as JSON; row numbers are 1-based line numbers):

| out-dir | bare `#type` / `#scope` | qualified `#header/…` | non-zero `set-field` rows |
|---|---|---|---|
| `~/out/RC24-A-frozen-work/turn02` | #25, #26 — exit 0 | — | none |
| `~/out/RC24-B-frozen-work/turn02` | #20, #21 — exit 0 | #10, #11 — exit 0 | none |
| `~/out/RC24-C` | #11 — exit 0 | — | none |

### The contract

**What half 1 contradicts** — one sentence, in one word:

- The composed step's own words (`crates/cli/packs/methodology/steps/author-commit.yaml`,
  `crates/cli/packs/dev/steps/author-commit.yaml`, `crates/cli/packs/dev/steps/amend-message.yaml`):
  the `commit` schema is the authority on *"every address a write can take"*. The schema read
  lists one spelling per leaf; the write door takes a second one, and it is the one the same
  step prints three lines above the sentence. Read as *every spelling*, the sentence is false;
  read as *every writable leaf*, it is true — the schema read does carry an address for every
  leaf a write reaches. The sentence does not say which it means.
- `design/surface-contract.md` → The three laws → **Law 1 — nothing lies**: *"Every claim a
  surface makes is generated from the thing it describes, or asserted against it."* The
  `every address` clause is asserted against nothing for a slugged doctype — the repository's
  own fence for it (`fixed_identity_axis.rs`, M52 Increment 6 / T8) forbids the clause on
  fixed-identity doctypes only, on the stated premise that *"for a slugged doctype that is the
  whole truth"*. For `commit` it is the whole truth up to the alias.
- The stated intent of the pin that picked the canonical form
  (`crates/cli/src/doc.rs`, the doc comment on
  `schema_read_surface_picks_the_qualified_commit_address`): *"the surface states the canonical
  form exactly once, so no two printed surfaces can disagree on the wire."* Two printed
  surfaces do disagree on the spelling — the test scans the schema projection and never a
  composed step.

**What it does not contradict.** The alias is designed and documented: `jigc doc set-field
--help` prints the `<ADDR>` argument as *"`<type>:<slug>#<field>` (or `#<section>/<field>`)"*,
and `design/write-commands.md` → Worked example — the MVP write loop uses the bare form. The
write door's behaviour is exactly what its own help says. No design rule says a composed step
must print the schema-advertised spelling.

**This is a re-report, and the earlier ruling is the precedent.**
`completions/artifacts/RC-alpha3/findings-verification.md` (line 43) adjudicated the same
worker inference — *"Workflow-printed `#type`/`#scope` commit addresses error" — REFUTED* —
and recorded the residual: *"no surface states the equivalence or picks a canonical form …
Fix: pick the qualified form for printed surfaces or state the alias once."* The fix that
shipped picked the qualified form on `doc schema` alone; the composed steps kept the bare one.
A blind agent on rc.24 drew the same wrong inference from the same two surfaces.

### Tier

**Tier 3, weak** — for half 1 only. Predicate: *a surface says something the binary does not
do*. The composed step says the schema read lists every address a write can take; the binary's
schema read omits a spelling the write door takes and the step itself prints. Weak because:

- nothing is lost and nothing is refused — not tier 1 (no exit-0 loss, no repository harm:
  the commit header is byte-identical whichever spelling wrote it), not tier 2 (no dead end:
  both the step's printed command and the schema's advertised address run at exit 0);
- the sentence is false only under one of its two readings;
- the equivalence is stated on a printed surface (`jigc doc set-field --help`), just not on
  either of the two surfaces a workflow-following agent reads.

The observed cost is the one the trial shows: an agent that read both surfaces concluded the
workflow's printed command was wrong, wrote the other spelling, and reported a failure that
never happened.

Half 2 (*"would have failed"*) carries **no tier** — it is refuted.

### Pin

`pinned-by:` — each side of the disagreement is pinned separately, and the refutation is pinned:

- both spellings resolve to one field — `doc_read_surface::documented_alias_forms_round_trip`
  (group `g_doc`; drives the binary: a bare `#type` write read back through `#header/type`)
  and the unit `doc::tests::implements_resolves_section_qualified_and_flat_alias`
  (`crates/cli/src/doc.rs`);
- the schema read advertises the qualified form and never the bare one —
  `doc::tests::schema_read_surface_picks_the_qualified_commit_address`;
- the composed steps print the bare form — `compose_goldens::sweep_fresh` (and its five
  sibling states) over `crates/cli/tests/goldens/compose/methodology/start--dev-task--fresh.txt`
  (lines 41, 45, 51) and the dev-pack goldens that carry `set-commit-type`.

`UNPINNED:` the relation between the two. No test compares a composed step's printed write
address with the spelling `doc schema` advertises for the same leaf, and
`fixed_identity_axis.rs`'s `every address a write can take` fence covers fixed-identity
doctypes only — so the surface disagreement is frozen in place by goldens on one side and a
unit test on the other, with nothing that would go red if either moved toward the other.

### Notes

- Driven on one `fresh` rig, release posture, `CLAUDECODE` set throughout (one cell, nothing
  compared across it). The finalize in step 5 committed inside the rig's `$REPO` only; the
  working repository's status is unchanged.
- Not driven: the same two-spelling question for other doctypes' header fields (`adr#status`
  and the like), and the `--format json` schema projection's address fields — the plain-text
  projection is what the composed step names and what the worker read.
- No fix proposed; none made.

---

## L-12 · The pre-commit forecast stops at the subject line: `task finalize --dry-run` renders the header, but no surface renders the trailer block (authored trailers or the adapter's co-author line) before the commit — **PARTIAL**

`tier:` none

**Lead (product):** No verb renders the commit message a `task finalize` will write before it writes it; `doc show commit:<task> --task` shows slots, not the rendered header, body and trailers.
**Source:** debrief-b item 3. **Door:** `jigc task finalize`; `jigc task validate`.
**Binary:** `~/.local/bin/jigc`, `jigc --version` → `jigc 1.0.0-rc.24` (asserted first). Release posture. `crates/` and `design/` at the working tree are byte-identical to tag `jigc-v1.0.0-rc.24` (`git diff --stat jigc-v1.0.0-rc.24 -- crates design` is empty), so the docs and suites cited below are those of the driven binary.

### Verdict

**PARTIAL** — tier **none**.

The claim is three-wide (header, body, trailers) and the evidence carries two of the three.

- **Header — REFUTED.** `jigc task finalize <id> --dry-run` prints the rendered subject line before anything is committed (`would commit — fix(summary): return an empty string for empty input`; `"subject"` on the JSON envelope), and the string is byte-equal to the first line git stored after the real finalize. Its help text opens with *"Print the commit's forecast subject line"*. The falsifying datum for the session's own account: arm b's invocation log (27 rows, `~/out/RC24-B-debrief/.jigc/logs/invocations.jsonl`) holds **zero** `--dry-run` rows, zero `task finalize --help` rows and zero `task diff` rows — the session described `--dry-run` as *"checking for blocking findings"* without having run it or read its help.
- **Body — holds, but is immaterial.** No verb prints the body as a block of the message; the body slot is rendered verbatim, so the `doc show` read-back already equals it byte for byte.
- **Trailers — holds.** No pre-commit surface, in any of the three formats, prints a rendered trailer line. The authored item reads back as `### Refs  {#refs}` / `- value: TKT-12` and lands as `Refs: TKT-12`; and the adapter's `Co-Authored-By: Claude <noreply@anthropic.com>` line — which no commit doc carries — appears on no surface before the commit, on either commit model (ordinary and amend).

So the accurate statement is narrower than the lead: **the forecast stops at the subject line; the trailer block, including the one line the agent did not author, is first visible in `git log` after the commit.**

### Repro

```text
# setup — fresh rig, installed registry binary. The variable the co-author trailer keys
# on is SET in every cell below (held constant; its value never printed).
rig=$(dev/jigc-rig fresh --binary ~/.local/bin/jigc \
        --start dev-task "fix the summary helper on empty input") || exit
eval "$rig"; [ -n "$REPO" ] || exit
$JIGC --version                                   # jigc 1.0.0-rc.24
T=$RIG_TASK                                       # fix-the-summary-helper
git -C "$REPO" rev-list --count HEAD              # 2   (before-control)

# author one code file and a commit doc with every message part
printf 'export const summary = (xs) => xs.length ? xs.join(",") : "";\n' > summary.js; git add summary.js
jigc doc set-field commit:$T#header/type  --value fix     --task $T      # exit 0
jigc doc set-field commit:$T#header/scope --value summary --task $T      # exit 0
jigc doc set-slot  commit:$T#summary --from-file - --task $T <<'EOF'     # exit 0
return an empty string for empty input
EOF
jigc doc set-slot  commit:$T#body    --from-file - --task $T <<'EOF'     # exit 0
MARKER-BODY-LINE the helper joined an empty list into undefined.

Second paragraph of the body.
EOF
jigc doc add-item  commit:$T#trailers --title Refs --task $T             # exit 0
jigc doc set-field commit:$T#trailers/refs/value --value TKT-12 --task $T # exit 0

# cell 1 — the read-back the composed step names            exit 0
$ jigc doc show commit:$T --task $T
  ---
  type: fix
  scope: summary
  ---
  # fix-the-summary-helper
  ## Summary
  return an empty string for empty input
  ## Body
  MARKER-BODY-LINE the helper joined an empty list into undefined.
  Second paragraph of the body.
  ## Trailers
  ### Refs  {#refs}
  <!-- fields -->
  - value: TKT-12
  # (--format json: fields{type,scope}, sections{summary,body,trailers[{id,key,value}]})

# cell 2 — the scorer's doors                                exit 0 each, x3 formats
$ jigc task diff $T          # the code diff, then "# staged docs" + the same doc source as cell 1
$ jigc task validate $T      # "no findings — the task validates clean"

# cell 3 — THE FALSIFIER for the header third                exit 0, HEAD unmoved
$ jigc task finalize $T --dry-run
  finalize --dry-run — pre-commit manifest (nothing committed)
  would commit — fix(summary): return an empty string for empty input
    added summary.js
$ jigc task finalize $T --dry-run --format json
  { "dry_run": true, "findings": [], "left_out": [],
    "manifest": [ { "kind": "added", "path": "summary.js" } ],
    "subject": "fix(summary): return an empty string for empty input" }

# the count over every captured output — doc show, task diff, task validate,
# task finalize --dry-run, each in agent/json/human, plus `jigc start --task $T` (13 files):
#   rendered subject  `fix(summary): return an empty string…`   3 hits — the three dry-run files
#   rendered trailer  `Refs: TKT-12`                             0 hits
#   `Co-Authored-By`  (case-insensitive)                         0 hits
#   body marker       `MARKER-BODY-LINE`                         doc show x3, task diff agent/human

# the commit
$ jigc task finalize $T                                       # exit 0
  finalized bebefdb — fix(summary): return an empty string for empty input
    added summary.js
    1 file committed

# after
git rev-list --count HEAD                                     # 3
$ git log -1 --format=%B
  fix(summary): return an empty string for empty input
  <blank>
  MARKER-BODY-LINE the helper joined an empty list into undefined.
  <blank>
  Second paragraph of the body.
  <blank>
  Refs: TKT-12
  Co-Authored-By: Claude <noreply@anthropic.com>

# cell 4 — the amend arm, same rig                            every step exit 0
$ jigc task amend                 # mints amend-bebefdb
  (set #header/type = fix, #summary = `return "" for empty input`; no body, no trailer item)
$ jigc task finalize amend-bebefdb --dry-run
  would rewrite bebefdb "fix(summary): return an empty string for empty input" → "fix: return "" for empty input"
    the tree and the author are unchanged; the message is re-authored and the committer becomes you, now
$ jigc task finalize amend-bebefdb
  amended bebefdb → 4258f0c — fix: return "" for empty input
$ git log -1 --format=%B
  fix: return "" for empty input
  <blank>
  Co-Authored-By: Claude <noreply@anthropic.com>
```

What the repro shows: the subject is forecast and matches; the trailer block (`Refs: TKT-12`, then the adapter's co-author line; on the amend arm the carried co-author line alone) is printed by nothing before the commit lands. Nothing was lost and nothing landed that the design does not say lands.

### The contract

**None is contradicted — the binary does what its surfaces and its design say.**

- `jigc task finalize --help` → `--dry-run`: *"Print the commit's forecast subject line and the pre-commit manifest … and stop"*. It claims the subject, and delivers the subject.
- `design/finalize.md` → Open questions → *`finalize --dry-run`* (resolved B1 / M30, widened M50): the dry run *"prints the change-set manifest — the commit's forecast **subject line**, what would be committed … and what would be left out"*. No full-message preview is promised.
- `design/command-output-contract.md` → *"M50 closes the `--dry-run` forecast's own withheld value (F-7)"*: the forecast names the subject as *"a projection of the same render, `FinalizePlan::subject` (the message's first line)"*. This is the same capability gap an earlier trial raised (RC-m50 F-7), closed for the subject line and deliberately no further.
- `design/finalize.md` → Commit-doc rendering → *"One trailer the commit doc does not author (2026-10-03)"*: the co-author line is *"applied at the commit seam, after this render and outside the `commit` schema, so … the rendered `subject` and every pinned JSON key are unchanged"*. Its absence from the doc read-back and from the forecast is the designed behaviour, not a drift from it.
- The composed `dev-task` step's own words (`jigc start --task $T`): *"Set the subject line — it renders as `<type>(<scope>): <summary>`"* and *"Read your write back before you move on … `jigc doc show commit:<task> --task <task>`"*. The step states the subject mapping and routes to a slot read-back; it does not offer a message preview, and it does not mention `--dry-run` either — which is why the session never met the door that answers a third of its question.
- `jigc task validate` help and the step's what's-left line describe a finding preview and say so; neither claims to render the message.

### Tier

**none.** Not tier 1 (no loss, no harm: the commit carries exactly the authored parts plus the documented adapter trailer). Not tier 2 (no posture or route dead end — every command exits 0 and the task lands). Not tier 3 (no surface says something the binary does not do: the one surface that claims a forecast claims the subject, and the subject is what it prints). What remains is a capability gap of the F-7 family, one step further along: the rendered trailer block has no pre-commit surface, and the composed step does not route to the `--dry-run` forecast that does exist.

### Pin

`pinned-by: g_finalize::finalize_dry_run_subject::the_dry_run_forecasts_the_subject_the_commit_lands` — pins the half that refutes the header third: the forecast's emitted subject is the same string the landed ack prints.

The boundary of the forecast is pinned too: `g_milestone::cwd_verb_subject::task_finalize_names_the_checkout_it_commits_in_when_that_is_not_the_workbench_home` asserts the `--dry-run --format json` envelope's key set is exactly `dry_run, findings, left_out, manifest, subject` — so a full-message key cannot appear without that suite moving. The text↔JSON agreement of the subject on both commit models is `text_json_parity_axis` → `finalize_dry_run_subject_close`.

The **gap** itself (no rendered body/trailer preview) is `UNPINNED` in the only sense available: there is no behaviour to pin, and no suite asserts that a pre-commit surface shows — or withholds — the trailer block on the text surface.

### Notes

- The scorer's proposed repro (`task finalize --help`, `task diff`, `task validate`) answers itself at the first command: the help names `--dry-run` as the subject forecast. `task diff` prints the staged doc source; `task validate` prints findings only.
- The session's substitute, `doc show … --task`, is the route the composed step itself prints, so the session followed the product's own guidance; the debrief hedged honestly (*"never found (or looked hard enough for)"*).
- Trailer variable: set in every cell here, as in the trial (the environment probe answered MAIN-SET / SUB-SET). With it unset the co-author line would not land (`design/assistant-adapter.md` → The co-author trailer) — not driven here, since the comparison held the variable constant; the `Refs: TKT-12` half of the gap does not depend on it.
- Nothing under `~/out` or `~/ideas` was modified; no repository edits, no commits outside the throwaway rig.

---

## L-13 · `jigc start "<intent>"` composes the router byte-identically for every intent — as designed (the agent picks; the CLI never does) — **CONFIRMED**

`tier:` none

**Lead (product):** `jigc start "<intent>"` prints the same catalog whatever the intent says; nothing ranks or narrows by it.
**Source:** debrief-b item 1; arm b invocation #4. **Door:** `jigc start`.
**Binary:** `~/.local/bin/jigc`, `jigc --version` → `jigc 1.0.0-rc.24` (asserted first, and again inside each rig). Release posture. `crates/`, `design/` and `ideas/` at the working tree are byte-identical to tag `jigc-v1.0.0-rc.24` (`git diff --stat jigc-v1.0.0-rc.24 -- crates design ideas` is empty), so the docs and suites cited below are the ones the published binary was built from.

### Verdict

**CONFIRMED as a fact — tier none (not a defect).**

The observation reproduces exactly, and it is wider than the lead words it: the *whole* stdout is byte-identical across intents, in all three forms that take a bare intent (`agent` text, `--format json`, `--explain`). The intent is not echoed, not interpolated into the re-run line (which prints the literal marker `"<intent>"`), and not used to order or filter the catalog. Exit 0, empty stderr, nothing minted, nothing written outside the opt-in log.

No contract is contradicted. The design of record states this behaviour in so many words — *the agent picks; the CLI never does* — and the router's own printed text tells the reader to do the picking. So this is the binary doing what it is specified to do, and what the agent experienced as "fuzzy" is the cost of that design, not a deviation from it.

One bound on the origin: the arm (b) session ran the router form **once** (`invocations.jsonl` row 4, 2026 bytes) and the bare orientation twice (rows 3 and 18, 2243 bytes each). Its debrief sentence "regardless of what I put in the intent string" was therefore not measured across intents inside the session. The rig measurement below carries it across five; the session's 2026 bytes equal the rig's 2026 bytes for every intent, including the session's own intent verbatim.

### Repro

```text
# setup — fresh rig, installed registry binary (two-step eval, stdout only)
rig=$(dev/jigc-rig fresh --binary ~/.local/bin/jigc) || exit
eval "$rig"; [ -n "$REPO" ] || exit
$JIGC --version                                    # jigc 1.0.0-rc.24
W=$(mktemp -d <tmp>/L13.XXXXXX)
# before-control: sha of every file under .jigc/ (logs excluded), HEAD, porcelain status
snap > $W/before

# argv — five intents x three forms
i=0
for intent in "a" \
              "write the architecture doc" \
              "quick-fix" \
              "fix GET /summary/ with empty series name should return 400 instead of 200 (TKT-212)" \
              "record the decision to use sqlite; no code"; do
  i=$((i+1))
  jigc start "$intent"               > $W/b$i.out 2> $W/b$i.err
  jigc start "$intent" --format json > $W/j$i.out
  jigc start "$intent" --explain     > $W/e$i.out
done

# observed
#   agent   : exit 0 x5, 2026 bytes x5, stderr 0 bytes x5
#   json    : exit 0 x5, 2013 bytes x5      ({"task": null, "text": "These are the selectable ..."})
#   explain : exit 0 x5,  604 bytes x5      (workflow:router, includes present-catalog + route-to-workflow)
shasum $W/b?.out $W/j?.out $W/e?.out | awk '{print $1}' | sort | uniq -c
#   5 <one sha>   5 <one sha>   5 <one sha>          -> three distinct digests, one per form
diff $W/b1.out $W/b2.out                           # empty, exit 0   (the scorer's proposed repro)
grep -c "sqlite\|TKT-212" $W/b4.out $W/b5.out      # 0, 0            (the intent is not echoed)
#   "quick-fix" as the intent does not select, rank or mark the quick-fix row
#   the re-run line is the literal:   jigc start --workflow <chosen> "<intent>"
#   jigc start ""  -> exit 0, the same catalog (an empty intent is the router form too)

# the printed text (identical for every intent), abridged:
#   These are the selectable work-workflows, each with the situation it fits:
#   - architecture-documentation — ...        (13 rows, id-sorted)
#   ...
#   Pick the workflow whose situation best fits the intent, then re-run with that
#   choice and the original intent:
#   jigc start --workflow <chosen> "<intent>"

# after
snap > $W/after; cmp $W/before $W/after            # identical, exit 0
ls $REPO/.jigc/tasks                               # absent — nothing minted

# positive control — the output is keyed on repository state, not on the intent
jigc start "a" > c1                                # fresh rig
jigc start --workflow quick-fix "fix a typo"       # exit 0, task minted: fix-a-typo
jigc start "a" > c2
jigc start "write the architecture doc" > c3
diff c1 c2    # exit 1: two added lines, the `also open:` block naming fix-a-typo
diff c2 c3    # exit 0: still intent-independent with a task open
```

The control matters for the lead's word "same": the router text is not a constant. It moves with the open-task set (shown), and by design with the catalog the cascade resolves. It does not move with the intent.

### The contract

None contradicted. Three places state the behaviour:

- `design/workflow-dialect.md` → *Workflow selection — the router default*: which workflow runs is "**not** the CLI's (it can't infer, model-free)"; the router "instructs the agent to re-run `jigc start --workflow <chosen> "<intent>"` (an **agent-substitution** pattern, not a resolved value — the agent re-supplies its own intent). The agent picks; the CLI never does." The same section defers a *recommended* default as post-M2.
- `design/write-commands.md` → *Task origination*, closing paragraph: composing a `creates-task: false` workflow "takes **no task context**"; it "threads the user's intent forward only through an **agent-substitution marker** in its emitted re-run command … never a resolved data-value. The `{{task.intent}}` binding above applies only on the minting path."
- The composed step's own words, printed on the surface the agent read: "Pick the workflow whose situation best fits the intent, then re-run with that choice and the original intent". The bare orientation says the same of this door: "presents the workflows above; pick one, then re-run with `--workflow <chosen>` to compose it".

It is also a ruled precedent, not a first sighting: `crates/cli/tests/pinned_facts.rs`, Part D row **D11** — "CONFIRMED | UNPINNED: works-as-designed — the agent-is-the-router invariant (the CLI does no selection by construction…)". The residue is parked at `ideas/spec-router-matching.md`, which records the same complaint from three earlier trials and bars intent-keyword matching in the CLI under the no-LLM invariant.

What the binary does instead: it composes the cascade's `default-workflow` (shipped: `router`), whose two steps interpolate the engine-native `catalog` root (every `creates-task: true`, `selectable: true` workflow with its `when` line) and print an instruction for the *agent* to choose. The session did exactly that — it previewed `quick-fix` and `dev-task` (rows 6 and 7) and chose by the "test-first" axis the two `when` lines carry.

### Tier

**none.** The tier predicates need a surface to disagree with the binary (tier 3), a route or posture dead end (tier 2), or exit-0 loss through a committing, destroying or moving door (tier 1). None holds: the door is read-only (before/after snapshots identical, nothing minted), the route it prints works (the session's next `start --workflow dev-task …` minted at exit 0), and the surface the agent read describes what happened.

### Pin

`pinned-by: compose_goldens::sweep_fresh` (surface `start-intent`; with `start-intent-json` for the JSON form) — for the byte shape, **under one intent**.

- The committed golden `crates/cli/tests/goldens/compose/composite/start-intent--start-intent--fresh.txt` is driven with the intent `"sweep the compose surface"`. Its stdout section is **byte-identical** (`diff` empty) to what the installed rc.24 binary printed on the `fresh` rig for the intent `"a"` — and the golden contains zero occurrences of its own intent string. That is the invariance, read off a committed file and the published binary together.
- Also carrying the shape: `start_compose::no_override_bare_intent_compose_is_byte_identical_to_the_golden` (its constant's doc comment says the router "carries no `{{task.intent}}`, so the golden is intent-stable") and `routing_loop::step_1_bare_intent_composes_the_router_listing_both_workflows_without_minting` (asserts the literal `<chosen> "<intent>"` marker and no mint).

**UNPINNED as a property:** no test drives two different intents and asserts equal output; each suite pins the bytes for a single intent. A change that made the output intent-dependent would be caught only where it altered the bytes for that suite's one intent. This matches D11's own disposition ("UNPINNED: works-as-designed … byte-shape … carried by the `start-orient*` / `start-intent*` compose goldens"). I compared the golden to the installed binary's output directly and did not run the suites.

### Notes

- **Adjacent wording, outside this lead and not tiered here.** Two surfaces still use the verb the design reserves for the agent: `jigc start --help` ("a `creates-task: false` selection pass that routes the intent to a work-workflow"; and for `[INTENT]`, "present → compose the cascade's `default-workflow` with `{{task.intent}}` = `<intent>`", a binding `write-commands.md` says does not exist on the non-minting path), and the router's `description:` as `jigc describe --workflows` prints it ("…presents the available work-workflows and routes an intent to the right one"). The orientation line was already reworded on exactly this ground (`render.rs`'s test comment: the CLI *presents*, the agent picks, "never 'routes among'", under `design/surface-contract.md` law 1). The arm (b) session did not read either surface (zero occurrences of `start --help` or of that sentence in its streams), so they are not the origin of the debrief item and do not change this verdict. If someone wants them weighed, that is a separate tier-3 candidate with its own repro (`jigc start --help`; `jigc describe --workflows`), not L-13.
- The scorer's proposed tier (none — as designed) stands.
- Nothing under `~/out` was modified; the rigs are `mktemp` roots with no teardown; no commit was made in the working repository.

---

## L-14 · The composed `planning` text carries eight jigc milestone tags, "the orchestrator" and a Rust parameter name, unattributed, in an adopter's repository — reproduced exactly; mandated verbatim by the design of record and pinned by a test; "cannot tell" not carried; no contract contradicted — **PARTIAL**

`tier:` none

**Kind:** product lead · **door:** `workflow:planning` (the plan-time gate text, i.e. the
`planning-record` slot hints the composition projects)
**Binary driven:** the installed registry build, `jigc --version` → `jigc 1.0.0-rc.24` (release posture).
**Rig:** fresh — `dev/jigc-rig fresh --binary <installed jigc>` (a git repo, `jigc setup` only,
2 commits, `jigc doc list` → *no committed docs*), root from `mktemp -d`, `HOME` repointed by the
rig. The session's out-dir was read only to confirm the lead's origin, never as proof. No commit
was made in the rig after its construction, so the co-author-trailer variable plays no part in any
cell here (held as inherited throughout; never printed).

### The claim

> The shipped `planning` composition carries the jigc project's own history as gate text in an
> adopter's repository — eight milestone tags (M34, M42, M44–M48, M55), "the orchestrator", a Rust
> parameter name — which a blind worker cannot tell from its own project's history.

### Verdict

**PARTIAL** — every *observable* in the claim reproduces exactly on a fresh rig; the clause
*"cannot tell from its own project's history"* is not carried by the evidence; and **no contract is
contradicted** — the design of record commits to exactly this text and a test pins it. A
reproduced observation, not a defect on the tier predicate: **tier none**.

- **Holds (driven).** `jigc start "x" --workflow planning` exits 0 and prints 44,518 bytes on 426
  lines. `\bM[0-9]{2}\b` matches **30** times on **14** lines, **8** distinct tags:
  `M34`×2 · `M42`×10 · `M44`×4 · `M45`×4 · `M46`×4 · `M47`×2 · `M48`×2 · `M55`×2 — the scorer's
  list exactly. *"the orchestrator"* — 6 occurrences on 2 lines. The Rust parameter —
  `` `record: &FileStateRecord` `` — 2 occurrences. `--format json` (44,039 bytes) carries the same
  8 tags / 30 occurrences / 6 / 2.
- **Shape of it.** The 14 lines are 7 of the 14 gates, each rendered twice (once in the projected
  `planning-record` schema listing, once as the `<<…>>` slot of the batch payload):
  `reuse-exercised`, `cheap-vs-robust`, `foreclosed-by-doc`, `census`, `deliverable-reachable`,
  `quote-attributed`, `claim-driven`. Those lines are **21,471 of the 44,518 bytes** (48 %).
- **More of the same family than the lead lists.** The same lines also name a jigc function
  (`conformance_advisory_finding`), jigc source and design files (`finalize.rs`,
  `team-ready-state.md` — 0 tracked files of either name in `$REPO`), a release candidate
  (`rc.11`), a date (`2026-08-18`), and jigc-internal record ids (`F10`, `S4`, `D3`, `N-4`,
  `M47 Settle Decision 1`).
- **Bounded to this one composition.** Of the 38 workflow ids `jigc describe --workflows` lists,
  21 answer `jigc workflow <id> --preview` at exit 0 and **only `planning`** carries a tag
  (`\bM[0-9]{1,2}\b`; the 17 refusals' output scanned too — 0). `jigc doc schema planning-record`
  (1,240 bytes), `jigc describe` and bare `jigc start` carry **0** — the text reaches a worker only
  through the composed step. `planning` is hidden from the router catalog and is invoked by name.
- **No attribution anywhere on the surface.** The composed text has 0 matches for a sentence
  saying whose incidents these are (`jigc's own`, `worked instance`, `for example`, `e.g.`,
  `upstream`, `another project`, …). The step's own framing sentence is *"each gate's line states
  what that gate requires recorded"*.
- **The falsifying datum for "cannot tell".** The worker in arm (a) *did* tell, in one read-only
  call: its debrief says the tags *"looked like they might be this project's own history"*, that
  `jigc doc list` came back empty, and that it then *"treated the M-numbers as generic
  pack-authored teaching examples"* — adding, accurately, that *"jigc itself never confirmed that
  reading"*. The session's invocation log shows `doc list --format json` as the very next call
  after the compose (#5 → #6). On the fresh rig the tags resolve to **nothing** rather than to
  something wrong: `jigc doc show planning-record:m42` → exit 1, `store.not-found`. What the
  evidence carries is *"no surface says whose history it is; a worker has to infer it"* — not
  *"cannot tell"*.
- **No effect on the adopter's record.** In `~/out/RC24-A-frozen-work/turn02` the five committed
  managed docs (planning-record, roadmap, decisions-log, deferral-ledger, milestone-record) hold
  **0** `M<nn>` tags, 0 `orchestrator`, 0 `FileStateRecord`. The session's main transcript carries
  the gate text in 2 records (same 8 tags) and no in-session remark about them; the hesitation is
  reported in the debrief only, from memory. Nothing was hunted for, nothing was written wrong.
- **A constructed collision exists and is the honest upper bound.** The tags sit in the address
  space the pack itself hands the adopter: `jigc doc create planning-record --title M42 --task x`
  mints `planning-record:m42` (exit 0, staged). An adopter who numbers its own milestones `M<nn>`
  and reaches 34/42/… would read *"M42 greped `schema.location` and missed `prior.location`"*
  beside its own `M42`. Constructed here, not observed in the trial (arm (a)'s record is
  `bound-what-the-service-will`).

### Repro

```
# setup — from the jigc checkout; two-step eval, stdout only
jigc --version                                        # jigc 1.0.0-rc.24
rig=$(dev/jigc-rig fresh --binary <installed jigc>) || exit; eval "$rig"; [ -n "$REPO" ] || exit
W=$(mktemp -d "<tmp>/l14.XXXXXX")
git -C "$REPO" log --oneline | wc -l                  # 2
$JIGC doc list                                        # exit 0 · "jigc doc list — no committed docs"

# argv 1 — the scorer's repro
$JIGC start "x" --workflow planning > "$W/planning.txt"          # exit 0 · 426 lines · 44518 bytes · stderr empty
grep -oE '\bM[0-9]{2}\b' "$W/planning.txt" | sort | uniq -c
#      2 M34   10 M42   4 M44   4 M45   4 M46   2 M47   2 M48   2 M55      (8 distinct, 30 total, 14 lines)
grep -oE 'the orchestrator' "$W/planning.txt" | wc -l            # 6  (2 lines)
grep -o  'FileStateRecord'  "$W/planning.txt" | wc -l            # 2
# observed lines (first of each pair; cut):
#   107: - `reuse-exercised`: prose slot — … **Exercised means the *semantics* were run, never that the
#        *signature* was read (M42)** — three of M42's six wrong gate-cells passed because … *"`record:
#        &FileStateRecord` is already a parameter — verified by reading the signature"* was true …
#   109: - `foreclosed-by-doc`: … (M46, 2026-08-18). `team-ready-state.md` excluded `milestone finalize`
#        from the destroying-door refusal rule **by name** …
#   119: - `quote-attributed`: … **M48 failed this three times in one wave** … that string is
#        `conformance_advisory_finding`'s **own** route …
#   120: - `claim-driven`: … **M46 failed this four times in one wave** … an advocate's fan-out topology
#        claim that the orchestrator's own race test then falsified … M55 failed this twice: S4's refusal
#        design was reversed at the design review …
grep -ciE "jigc's own|worked instance|for example|e\.g\.|upstream|another project" "$W/planning.txt"   # 0

# argv 2 — the JSON arm the session actually read
$JIGC start "y" --workflow planning --format json > "$W/planning.json"   # exit 0 · 44039 bytes
#   same 8 tags, 30 occurrences; "the orchestrator" 6; FileStateRecord 2

# argv 3 — does anything in the adopter's project resolve a tag?
$JIGC doc show planning-record:m42                    # exit 1
#   blocking · store.not-found — could not read `planning-record:m42` at `docs/planning-records/m42.md` …
git -C "$REPO" ls-files | grep -cE 'team-ready-state\.md|finalize\.rs'    # 0

# argv 4 — siblings
$JIGC doc schema planning-record | grep -cE '\bM[0-9]{2}\b'       # 0   (1240 bytes; hints are not projected here)
$JIGC describe | grep -cE '\bM[0-9]{2}\b'                         # 0
$JIGC start    | grep -cE '\bM[0-9]{2}\b'                         # 0   (planning is not in the router catalog)
for id in <the 38 ids of `jigc describe --workflows`>; do $JIGC workflow "$id" --preview; done
#   21 exit 0, 17 exit 1; tags only in `planning` (preview 44404 bytes, same 8)

# argv 5 — constructed collision (staged only)
$JIGC doc create planning-record --title M42 --task x  # exit 0 · "planning-record:m42"
$JIGC doc show planning-record:m42 --task x            # exit 0 · "# M42" / "## Reuse Exercised" …

# after
git -C "$REPO" log --oneline | wc -l                  # 2 — nothing committed; two open tasks (x, y) staged in .jigc/

# origin, read back (read-only)
#   ~/out/RC24-A-frozen-work/turn01  invocation #5: start "<intent>" --workflow planning --format json → exit 0
#                                    invocation #6: doc list --format json → exit 0
#   main transcript: 2 records carry the gate text, tags = the same 8; "Output too large (44.1KB)"
#   ~/out/RC24-A-frozen-work/turn02  docs/**.md (5 committed managed docs): 0 tags
```

### The contract

**None is contradicted; the design of record mandates the observed text.**

- `design/methodology-docs.md` → *The planning gate-record*: the gate table there is the gates'
  *"single home"*, and *"those sentences move **verbatim** into the schema's `hint:`; nothing is
  redesigned."* The schema's own header repeats it (`crates/cli/packs/methodology/schemas/planning-record.yaml`:
  *"each `hint:` below carries that gate's 'What it requires recorded' cell VERBATIM"*). The
  binary does exactly this — the incident narratives are in that cell, so they are in the hint, so
  they are in the composition.
- The same section describes the table as *"index, not replace: the worked instances stay where
  they are; this is the checklist that points at them"* — where they are is the jigc repository's
  `implementation/` docs. In an adopter's repository the pointer has no target. That is the
  substance of the lead, and it is a consequence the design does not address, not a rule it breaks.

Nearest statements pointing the other way, each read and each short of a contract:

- `design/self-hosting.md` → *The idea*: the Distill move graduates the harness *"into portable,
  project-agnostic form"*; → *The four-part graduation pass*, item 1: *"**Portability** — each step
  reads principle-first, not jigc-coupled."* The gate text is jigc-coupled by that description.
  But the pass is declared *"executed for the dev-workflow only (the wider harness folds back
  later)"* and the doc's status is *"settled for M12's bounded first slice; exploratory beyond
  it"* — its stated scope does not reach `planning`.
- `design/surface-contract.md` → *The three laws*, law 1: *"every printed path is repo-real or a
  typed identity."* `team-ready-state.md` and `finalize.rs` are printed and are neither, in `$REPO`.
  But they are words inside a quoted incident, not a locus a reader pastes or a driver keys, and
  the printed-path fence's declared domain is *"what those doors print"* (the
  destroying/provisioning doors) — compose prose is outside it. → *The surface style guide* has no
  entry on project-neutral pack prose, and → *Honest bounds* states the style guide *"has no
  machine teeth … by design"*.
- The composed step's own words — *"each gate's line states what that gate requires recorded"* —
  describe the line as a requirement; half of what the line then carries is the history of the
  requirement. A reader may find that loose; it is not false.

**What the binary does instead of what the lead implies:** it projects each gate's design-table
cell, verbatim and unattributed, into the adopter-facing composition; it checks only that each gate
slot is *filled* (the text says so: *"it never reads, scores or lints the judgment you put inside
it"*); and none of the cited tags, files or symbols is an address it will resolve in the adopter's
store.

### Tier

**none.** Tier 3's predicate is *a surface says something the binary does not do*. The gate text
makes no claim about what the binary does in the adopter's repository — it narrates past incidents
in another project — so no statement of it is falsified by a driven behaviour. No exit-0 loss, no
repository harm, no dead-end route: the composition exits 0, arm (a) filled the gate record and
finalized its planning task (`a9f1851`), and 0 tags reached the adopter's committed docs. The scorer's
*"tier 3 (weak) … or none"* resolves to **none**.

What remains is a judgment-tier observation for the methodology pack's prose, with three measured
handles: 48 % of the 44.5 KB composition (the size L-15 records) is the seven lines carrying
another project's history; no sentence on any surface attributes it; and the tags live in the
namespace the pack hands the adopter for its own milestones.

### Pin

`pinned-by: planning_record_schema::the_composed_schema_projection_carries_every_gate_cell_verbatim`
(`crates/cli/tests/planning_record_schema.rs`, group `g_methodology`) — it asserts the composed
schema projection carries every gate's design-table cell verbatim, which is what puts the tags in
the composition. The emitted bytes are additionally held by `compose_goldens::sweep_fresh`
(group `g_compose`) over `crates/cli/tests/goldens/compose/methodology/start--planning--fresh.txt`,
whose golden carries the same 8 tags; and `planning_gate_home::the_single_home_lists_exactly_the_shipped_gates`
holds the table and the schema's gate set equal.

The behaviour is pinned **as designed**. The opposite property — that adopter-facing pack prose
names nothing only the jigc repository resolves — is **UNPINNED**: no test, fence or style-guide
entry states it, and no entry for it was found in `DECISIONS.md` or
`implementation/decisions-pending.md`.

### Not driven / bounds

- The tests named above were located and read, not run (the lead is about the published binary;
  the golden's tag set was compared by `grep`, not by executing the suite).
- "A blind worker cannot tell" was tested against one worker's account (arm (a), n = 1, from
  memory in a debrief) and the binary's answers on a fresh rig; no second blind session was run.
- The constructed collision shows an address clash is *possible*; whether a worker in that state
  would mis-attribute an incident was not trialled.

---

## L-15 · The `planning` composition is 45,112 bytes under `--format json` and exceeds the agent harness's inline limit — a true size observation, no contract contradicted — **CONFIRMED**

`tier:` none

**Kind:** product lead · **Door:** `workflow:planning` via `jigc start "<intent>" --workflow planning --format json`
**Binary driven:** the installed registry build, `jigc --version` → `jigc 1.0.0-rc.24` (release posture)

### The claim

> The `planning` composition is 45,112 bytes under `--format json`; the agent harness refused to
> inline it (*Output too large (44.1KB)*) and the worker read it back from a spill file.
> (arm a #5 `output_bytes`, transcript tool calls #2–#3; the scorer's tier: none — a size observation.)

### Verdict

**CONFIRMED — as an observation, tier `none`. Not a defect: no contract is contradicted.**

Every factual half of the claim reproduces on a fresh rig, to the byte:

- the size is **45,112 bytes**, exit 0, with the session's own intent;
- the same bytes printed bare in this verifier's own agent harness are refused inline with the
  identical line, *Output too large (44.1KB)*, and a 2 KB preview;
- in the session, the next tool call read the spill file and got **the whole composition back in
  one read** (44,748 characters returned for a 44,738-character file plus line prefixes; the
  composition's last line, the `Run: jigc task finalize …` directive, is present; no truncation
  marker). Nothing was lost; the cost was one extra tool call. Arm a exited 0.

One correction to the scorer's proposed repro: **the number is intent-dependent.** `jigc start "x"
--workflow planning --format json | wc -c` gives **44,039**, not 45,112 — the intent and the task id
slugged from it are interpolated into the composed text. 45,112 is what the session's intent
(`Bound what the service will accept`) produces. Either figure is above the harness's inline limit.

What the binary does instead of anything size-aware: it emits the whole composed workflow, every
step, in one document — `{ "task", "text" }`, where `text` is 44,525 bytes / 419 lines here. The
resume door (`jigc start --task <id>`) re-emits the same `text` byte-identically, and the mint-free
`jigc workflow planning --preview` is the same size class (44,404 bytes). There is no shorter form
of this workflow and no flag that pages it.

### Repro

```
# setup — a fresh `jigc setup`-only corpus, installed binary, stdout-only two-step eval
jigc --version                                    # -> jigc 1.0.0-rc.24
rig=$(dev/jigc-rig fresh --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit
O=$(mktemp -d "$SCRATCH/l15.XXXXXX")

# argv 1 — the session's own invocation (arm a, invocation-log row 5)
$JIGC start "Bound what the service will accept" --workflow planning --format json \
    > "$O/planning.json" 2> "$O/planning.err"
# observed: exit 0 · stdout 45112 bytes, 4 lines · stderr 0 bytes
#           keys: task (27 bytes: bound-what-the-service-will), text (44525 bytes, 419 lines)

# argv 2 — the scorer's proposed repro, on a second fresh rig
$JIGC start "x" --workflow planning --format json > "$O/x.json"
# observed: exit 0 · stdout 44039 bytes            (NOT 45112 — intent-dependent)

# argv 3 — the other doors onto the same composition (first rig, after argv 1)
$JIGC start --task bound-what-the-service-will --format json   # exit 0 · 45112 bytes · `text` identical to the mint's
$JIGC start --task bound-what-the-service-will                 # exit 0 · 45627 bytes · 424 lines (human form)
$JIGC workflow planning --preview                              # exit 0 · 44404 bytes

# argv 4 — the harness half, printed bare (no pipe, no redirect) in the verifier's agent session
$JIGC start --task bound-what-the-service-will --format json
# observed, the harness's own tool result:
#   <persisted-output>
#   Output too large (44.1KB). Full output saved to: <harness tool-results>/<id>.txt
#   Preview (first 2KB): { "task": "bound-what-the-service-will", "text": "Scope the milestone:\n…

# after — the rig holds the minted task and nothing else changed
git -C "$REPO" status --short     # (empty)
git -C "$REPO" log --oneline      # chore(jigc): install jigc workspace config / initial
```

The origin, read-only, agrees with the fresh rig:

```
~/out/RC24-A-frozen-work/turn01/.jigc/logs/invocations.jsonl  row 5:
    argv ["start","Bound what the service will accept","--workflow","planning","--format","json"]
    exit_code 0 · output_bytes 45112 · binary_version 1.0.0-rc.24
main transcript, tool call 2 (Bash, the same argv piped through `2>&1 | head -300`):
    "Output too large (44.1KB). Full output saved to: …/tool-results/<id>.txt" + "Preview (first 2KB)"
main transcript, tool call 3 (Read of that spill file): 44,748 characters returned, tail present
spill file on disk: 45112 bytes
occurrences of "Output too large" across all five session out-dirs: this one (arm a; turn02 carries
    turn01's transcript forward), none in arm b, none in arm c
```

For scale, the committed `start` goldens (text form, fresh state, capture framing included) put
`planning` alone in its size class — the next-largest composition is `completion` at 16,277 bytes,
`single-task` is 11,998, and the 39 swept workflows total 222,119 bytes, of which `planning` is 45,612.

### The contract

**None is contradicted.** Looked for a rule the size could break and found the opposite — the
project records that it has no such rule:

- `ideas/composed-context-token-budget.md` → *The gap*: "the compiler is blind to the size of what
  it emits: no measurement of a composed view's token footprint, no ceiling, no verbosity tier."
  Status line: *parked 2026-07-02, unscheduled*; indexed from `VISION.md` → Open questions. A
  budget knob and a concise/detailed composition tier are named there as the *shape*, not as
  shipped behaviour.
- `design/command-output-contract.md` pins the composed `--format json` shape (the task id at
  `.task`, the composed text) and states no bound on its length.
- `VISION.md` → *What it is* ("assembles *exactly* the instructions and document slices needed for
  a specific task, just-in-time") is the thesis, not a byte bound; the parked idea itself calls the
  budget the step that would turn that phrase "from a metaphor into a measurable property".
- `.jigc/AGENT.md` (as `jigc setup` writes it on rc.24) tells the agent to pass `--format json` and
  read the task id at `.task`; it promises nothing about output size or about fitting a harness's
  inline window.
- The refusal line is the **agent harness's**, not jigc's: jigc exited 0 and wrote all 45,112 bytes
  to stdout. The inline limit is a property of the tool channel the agent called it through.

What this lead *is*: a datum on that parked idea's own de-park trigger ("a measurement run or real
dogfood shows composed-view size is a material cost"). The measured cost in this trial is one
extra tool call and ~45 KB of context for one of three arms, with no loss. Whether that is
*material* is a triage call, not a verification one.

### Tier

**`none`.** Tier 1 needs an exit-0 loss or repository harm through a committing, destroying or
moving door: `jigc start` minted a task and printed; every byte reached the agent (the read-back
returned the whole file). Tier 2 needs a posture or route dead end: the session continued and
finalized. Tier 3 needs a surface that says something the binary does not do: no surface states a
size, a ceiling or an inline guarantee. The scorer's proposed tier stands.

### Pin

`pinned-by: compose_goldens::sweep_fresh` — golden
`crates/cli/tests/goldens/compose/methodology/start--planning--fresh.txt` (45,612 bytes) pins the
whole `jigc start --workflow planning <intent>` text capture byte for byte. Checked rather than
assumed: the installed rc.24 binary, driven on a fresh rig with the suite's fixed intent
(`sweep the compose surface`) and framed the way the harness frames a capture, is `cmp`-identical
to that golden. So the *content*, and therefore its size, is pinned, and any growth is a reviewable
golden diff.

Two bounds on that pin, stated: (1) the `--format json` envelope of `planning` is not itself swept
— the sweep's JSON variants cover the front-door composite surfaces only — so the 45,112 figure is
pinned through the text it wraps, not directly; (2) **no test asserts a size ceiling**, for any
workflow — there is nothing to assert it against, since no ceiling is declared (the contract
section above). `invocation_log::output_bytes_equals_emitted_stdout_plus_stderr` pins that the
`output_bytes` the lead cites is an exact count.

### Not done

No fix, no commit, no repository edit; nothing under `~/out` or `~/ideas` was modified. The harness
half was reproduced in the verifier's own agent session on the host, not inside the trial container
— same refusal line, same 44.1KB figure, but a different harness instance.

---

## L-16 · `milestone create` / `milestone add-task` each land a record-only commit at once while `doc` writes stay in the task working area until finalize — declared, stated by help, ack and design — **CONFIRMED (as an observation — the behaviour reproduces exactly; no contract is contradicted, so it is not a defect)**

`tier:` none

**Lead (product):** `milestone create` and `milestone add-task` commit at once while every `doc` write stays staged until a finalize — "two different persistence models under one CLI" in the worker's words.
**Source:** debrief-a item 1 (third bullet); arm a invocations #33–#39. **Door:** `jigc milestone create`; `jigc milestone add-task`.
**Binary:** `~/.local/bin/jigc`, `jigc --version` → `jigc 1.0.0-rc.24` (asserted first). Release posture. `crates/` and `design/` at the working tree are byte-identical to tag `jigc-v1.0.0-rc.24` (`git diff --stat jigc-v1.0.0-rc.24 -- crates design` is empty), so the docs and suites cited below are those of the driven binary.

### Verdict

**CONFIRMED as an observation — not a defect.** Tier **none**.

The behaviour the worker describes reproduces exactly on a fresh rig: a `doc` write under a task leaves `HEAD` and the index untouched (the bytes live only under `.jigc/tasks/<id>/docs/`), while `milestone create` and `milestone add-task` each move `HEAD` by one commit at exit 0. But no contract is contradicted. The split is the design of record, the help of both doors states it, and every ack names the commit it landed — in the text form and inside the `--format json` envelope the session actually used. The worker's own words are "I hadn't expected", and the debrief's item 2 says nothing jigc told it turned out wrong. The lead is a surprise about a declared model (protocol §0.2, first bullet, **Driven**), not a surface saying something the binary does not do.

### Repro

```
# setup — fresh rig, installed registry binary, CLAUDECODE set (held constant; value never read)
rig=$(dev/jigc-rig fresh --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit
#   $REPO: `jigc setup` only; .jigc/config → compose-embedded-methodology: true
git log --oneline            → 51eafbd chore(jigc): install jigc workspace config
                               82500fc initial

# A · the doc-write model (control)
jigc start --workflow park-idea "probe doc persistence"            → exit 0, task minted: probe-doc-persistence
jigc doc create idea --title "Cold cache eviction" --task probe-doc-persistence
                                                                    → exit 0, `idea:cold-cache-eviction`
jigc doc set-slot idea:cold-cache-eviction#description --from-file - --task probe-doc-persistence <<'EOF'
MARKER-L16-DOC evict cold entries first.
EOF
                                                                    → exit 0, `set slot idea:cold-cache-eviction#description (41 chars)`
git rev-parse --short HEAD   → 51eafbd            (unmoved)
git status --short           → (empty)            (nothing in the index or the tracked tree)
git grep -c MARKER-L16-DOC HEAD -- .              → exit 1 (no committed copy)
command grep -rl MARKER-L16-DOC .jigc docs        → .jigc/tasks/probe-doc-persistence/docs/idea:cold-cache-eviction.md

# B · milestone create
jigc milestone create "Cache rework"              → exit 0
  minted milestone:cache-rework (shared base 51eafbd)
  record: docs/milestone-records/cache-rework.md   — the committed record this milestone's state lives in
  record commit: e5bd181   — the record on its own; anything else you had staged stayed staged
  next: `jigc milestone add-task cache-rework "<intent>"`   — add the milestone's first sub-task
git log -1 --format='%h %s'  → e5bd181 chore(milestone): open record for milestone:cache-rework
git show --stat --format= HEAD → docs/milestone-records/cache-rework.md | 9 +++++++++   (1 file)

# C · milestone add-task
jigc milestone add-task cache-rework "Evict cold entries"           → exit 0
  added task:evict-cold-entries to milestone:cache-rework
  record commit: 7d51dfe   — task:evict-cold-entries on the milestone record, committed on its own; anything else you had staged stayed staged
git log -1 --format='%h %s'  → 7d51dfe chore(milestone): record task:evict-cold-entries on milestone:cache-rework
git show --stat --format= HEAD → docs/milestone-records/cache-rework.md | 7 +++++++   (1 file)

# D · the form the session used (`--format json`) — the same line rides the envelope's `text`
jigc milestone create "Second one" --format json  → exit 0, HEAD 7d51dfe → 2b9a150
  {"hook_output": "", "text": "minted milestone:second-one (shared base 7d51dfe)\nrecord: …\nrecord commit: 2b9a150   — the record on its own; anything else you had staged stayed staged\nnext: …"}
jigc milestone add-task second-one "First piece" --format json      → exit 0, HEAD → 73f868c
  {"hook_output": "", "text": "added task:first-piece to milestone:second-one\nrecord commit: 73f868c   — task:first-piece on the milestone record, committed on its own; anything else you had staged stayed staged"}

# E · the ack's own second claim, checked — the commit is path-scoped
printf 'x\n' > wip.txt; git add wip.txt           (git status --short → `A  wip.txt`)
jigc milestone create "Third one"                 → exit 0, record commit: 326e6fa
git status --short           → A  wip.txt         (still staged, not swept)
git show --stat --format= HEAD → docs/milestone-records/third-one.md | 9 +++++++++   (1 file)

# after
git log --oneline            → 326e6fa · 73f868c · 2b9a150 · 7d51dfe · e5bd181 · 51eafbd · 82500fc
#   five record-only commits, one per door call; every one touches exactly its milestone record.
#   The task-staged idea doc is still only in the working area (same before-control as A:
#   `command grep` finds it under .jigc/tasks/…, `git grep … HEAD` exits 1) — no doc write
#   was swept into, or lost by, any record commit.
```

The origin agrees with the rig: `~/out/RC24-A-frozen-work/turn01` logs `milestone create` at #33 and `milestone add-task` at #35 / #37 / #39, all exit 0 and all `--format json`, and its `git log` carries the four matching commits (`07a451f` open record, then `bc3ab92`, `dafdc52`, `29ea968`) directly above the planning task's own finalize commit `a9f1851`. The worker quotes two of those shas from the acks, so it read the `record commit:` line when the doors ran.

### Contract

None contradicted. The surfaces that state the behaviour:

- **`design/team-ready-state.md` → "The commit model — path-scoped commit at each milestone op":** "The record is written **and committed at each structural milestone op**, so a teammate cloning at any moment sees true current state"; "`create` / `add-task` — a **separate** record-only path-scoped commit each." The same section answers the worker's "two persistence models" directly: this "does **not** breach the finalize-transaction invariant, which governs the **doc-content integrity gate** … the record carries no authored prose to gate; it is a *structural* artifact the CLI maintains, the same class as the code commit the CLI already makes at finalize."
- **`jigc milestone create --help`:** "this door lands the milestone's committed record in a record-only commit: it writes that record under docs-root, `HEAD` moves, and the commit is path-scoped to the record, so anything else you had staged stays staged."
- **`jigc milestone add-task --help`:** "this door also appends the sub-task to the milestone's committed record and commits that record on its own: `HEAD` moves, … and the ack names the sha it landed."
- **The acks themselves** (`record commit: <sha> — … committed on its own`), per `design/command-output-contract.md` (the `record commit:` mold set by `jigc milestone create`).
- **Protocol §0.2, first bullet** declared it before the run: "`milestone create` and `milestone add-task` each land a record-only commit and say so … A milestone therefore moves `HEAD` before any work is done."

Nothing the worker read before #33 says the opposite. `.jigc/AGENT.md` says jigc "owns every structural write — placement, cross-references, commits" and makes no claim that every commit waits for a finalize. The composed `planning` step that hands over the two commands ("once this task commits, open the work-unit under that same title: `jigc milestone create …`") does not pre-announce that they commit, and does not say they do not; the first surface that says so on the worker's path is the ack. That is the only softness here, and it is below the tier-3 predicate: no surface says something the binary does not do.

### Tier

**none.**

- Not tier 1: exit 0 with no loss and no repository harm. The commits are path-scoped to the record (cell E: an unrelated staged file stays staged; every record commit is a 1-file commit), and the open task's staged doc is found before and after (cell A / after).
- Not tier 2: no posture or route dead end — the arm went on to provision and finalize.
- Not tier 3: help, ack and design all say what the binary does, and the binary does it.

### Pin

`pinned-by:` (all registered, `g_milestone` / `g_flow` group targets)

- `milestone_record_create::methodology_create_materializes_record_and_path_scoped_commits` — `create` lands the record-only, path-scoped commit.
- `milestone_record_add_task::methodology_add_task_appends_record_byte_stable_and_path_scoped_commits` — `add-task` does the same.
- `milestone::milestone_create_names_its_record_commit_its_path_and_the_next_step` — the ack's `record commit:` sha resolves to `HEAD`.
- `help_truth::committing_doors_say_so::every_committing_door_help_states_the_commit_it_lands` and `help_truth::committing_doors_say_so::the_two_seeding_doors_ack_the_record_commit_they_landed` — every committing door's help states the commit, and `add-task` / `add-from-spec` ack the sha.

### Notes

- The proposed tier (`none — declared (§0.2) and each ack says it commits`) holds on a fresh rig; I found nothing to raise it.
- Not driven: `milestone add-from-spec` (same record-only family, not in the lead) and the dev-only cascade, where both doors commit nothing by design (pinned by `milestone_record_create::dev_only_create_materializes_no_record_and_makes_no_commit` and `milestone_record_add_task::dev_only_add_task_materializes_no_record_and_makes_no_commit`).
- Outside this lead, seen in passing and not judged: the `--format json` envelope of both doors is `{hook_output, text}` — the landed sha is readable only inside `text`, with no dedicated key. `design/command-output-contract.md` pins a `commit` key for `task discard`; whether these two doors owe one is a separate question I did not check against the contract.
- Nothing was modified under `~/out` or `~/ideas`; the rig is a `mktemp -d` root with no teardown.

---

## L-17 · Nothing on the worker's path names `milestone join` or any pre-boundary check of the combined code — true; but `milestone join` builds no combined tree (docs merge + same-path collision only), the scratch worktree did not catch the break, and no contract is contradicted — **PARTIAL**

`tier:` none

**Lead (product).** No surface the worker stood on names a way to check that the sub-tasks' staged
work combines and passes before the boundary; `jigc milestone join` exists and was never named to
it, so it built a scratch `git worktree` and `git apply`ed the three staged diffs — which is how
the one cross-piece break was caught before `milestone finalize` would have landed it at exit 0.

**Doors:** `jigc milestone join` · `jigc milestone finalize` · `jigc start` (orientation).

### Verdict — PARTIAL

Driven on three fresh rigs with the installed registry build (`jigc --version` →
`jigc 1.0.0-rc.24`, release posture; `CLAUDECODE` set and held constant in every cell). The lead is
five clauses; three reproduce, two do not, and no contract is contradicted.

| Clause | Driven | Result |
|---|---|---|
| no surface on the worker's path names `milestone join` | every surface the arm's main session received, re-captured on rig 1 (`milestone create` / `add-task` acks, orientation, the `start --task <sub>` refusal, the `milestone provision` ack, the sub-task text composed in the worktree) | **reproduces** — `milestone join`: 0 hits on all of them; `milestone execute`: 0 hits on all of them. Named only by `jigc milestone --help` (the `join` row), `jigc --help` ("executes, joins, and finalizes"), and the text `jigc milestone execute <id>` composes — three doors the arm never opened (0 rows of 48) |
| `milestone join` is the check that was missing | rig 1: two sub-tasks, disjoint files, each passing the project's check alone and failing together; `jigc milestone join bound-inputs` | **refuted for "passes"** — exit 0, `joined milestone:bound-inputs — 0 doc(s) merged`; before/after snapshot differs in `.jigc/index/edges.json` only; no checkout holds both changes (main: cap 2000 / longest name 8; worktree A: 128 / 8; worktree B: 2000 / 1017); the project check in the main checkout still passes. Join leaves **no combined tree** a test run can use |
| …and for "combines" | rig 2: two sub-tasks staging the same path; `jigc milestone join collide` | **holds, narrowly** — exit 1, `blocking · combine.code-collision — code collision — cap.txt (sub-tasks […]) staged by more than one worktree`, HEAD unchanged. Join is the pre-boundary check for a same-path collision and for nothing semantic |
| the scratch worktree "is how the one cross-piece break was caught" | arm a main transcript, tool calls #49–#65 by timestamp | **refuted by the transcript's order** — the break was named in assistant text at 22:33:04 after *reading* the staged diffs (#49–#54: `git diff --cached` in each worktree), fixed at #56 (Edit), re-tested in that worktree at #57 (26 pass), staged at #58. The scratch worktree is #62 (22:33:43); its combined run at #64 reports 33 pass, 0 fail. It confirmed a fix already made; it caught nothing. The debrief (written "from memory, without re-running or re-checking") has the order reversed |
| `milestone finalize` would have landed the break at exit 0 | rig 1, no project hook: `jigc milestone finalize bound-inputs` | **reproduces** — exit 0, `finalized 18e00f6`, 3 files; at HEAD the project check exits 1 (`FAIL: name of 1017 chars exceeds cap 128`). Both sub-tasks' bytes landed exactly as staged |
| (control) the same boundary with the project's check as its pre-commit hook | rig 3: identical sub-tasks, `.git/hooks/pre-commit` runs the check | exit 1, `git commit was rejected (no commit was made)` + the check's own FAIL line; HEAD unchanged; both worktrees still hold their staged code |

So what survives: on the path a blind worker actually walked (orientation → resume refusal →
`provision` → sub-task text → `milestone finalize`), nothing names `milestone join`, and nothing
names any pre-boundary check of the *combined code* — because the binary has none. What does not
survive: that `milestone join` is that check (it merges docs and blocks a same-path collision; it
builds no tree), and that the workaround is what caught the break.

**Overlap.** The "never named" half is the same routing gap L-4 verified (orientation, the
`planning` tail and the `add-task` ack name neither `milestone provision` nor `milestone execute`,
and `milestone execute`'s composition is the only text that walks provision → spawn → join →
finalize). L-17 adds one datum to it: `milestone join` rides that same unreached text. The raw
linked worktree the worker made is L-22's subject, not this lead's.

### Repro

```
# setup — rig 1: fresh rig, installed binary; CLAUDECODE set and constant
rig=$(dev/jigc-rig fresh --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit
jigc --version                                   # jigc 1.0.0-rc.24
# a project with its own check: every line of names.txt must be no longer than cap.txt
printf '2000\n' > cap.txt; printf 'cpu.load\n' > names.txt
#   check.sh: reads cap.txt, exits 1 with "FAIL: name of <n> chars exceeds cap <cap>" on a longer line
git add cap.txt names.txt check.sh && git commit -q -m "chore: base project with its own check"
./check.sh                                       # PASS (cap 2000) — exit 0

jigc milestone create "Bound inputs"             # exit 0; next: `jigc milestone add-task bound-inputs "<intent>"`
jigc milestone add-task bound-inputs "Lower the name cap to 128"                  # exit 0; no `next:`
jigc milestone add-task bound-inputs "Accept a name of exactly 1017 characters"   # exit 0; no `next:`

# argv 1 — the surfaces on the worker's path
jigc start                                       # exit 0; per sub-task exactly four Run: lines:
#   Run: `jigc start --task <sub>`   — resume …
#   Run: `jigc task validate <sub>`   — previews part of the finalize gate …
#   Run: `jigc milestone finalize bound-inputs`   — validate + commit: … whose door is its only commit boundary …
#   Run: `jigc task discard <sub>`   — abandon …
jigc start --task lower-the-name-cap             # exit 1; routes to `jigc milestone provision bound-inputs` + the cd
jigc milestone provision bound-inputs            # exit 0; "provisioned 2 worktree(s) … at base 268e927 (…)"
( cd .jigc/worktrees/lower-the-name-cap && jigc start --task lower-the-name-cap )   # exit 0, 72 lines
#   …"Before you finalize, verify the change actually works: build it and run the tests … Finalize
#   commits your staged work; it does not check that the work is correct."
#   …"The parent milestone's finalize is the only commit boundary: `git add` your code edits so the
#   milestone can fold this worktree's staged index …"
# count of 'milestone join' / 'milestone execute' over each capture above (stdout+stderr): 0 / 0 on every one
jigc milestone --help                            # exit 0; the `join` row: "Merge the milestone's sub-task
#   areas into the parent working overlay … disjoint-union their staged docs … Commits nothing."
jigc milestone execute bound-inputs              # exit 0; the one composed text naming it:
#   "Once every spawned sub-task reports complete, run the join: it merges the sub-tasks' staged
#    docs by task-id order and reports the merged state — it commits nothing:"
#   Run: `jigc milestone join bound-inputs`

# argv 2 — two sub-tasks that pass alone and fail together (disjoint files)
printf '128\n' > .jigc/worktrees/lower-the-name-cap/cap.txt;        git -C .jigc/worktrees/lower-the-name-cap add cap.txt
python3 -c "print('a'*1017)" >> .jigc/worktrees/accept-a-name-of-exactly/names.txt
git -C .jigc/worktrees/accept-a-name-of-exactly add names.txt
( cd .jigc/worktrees/lower-the-name-cap && ./check.sh )             # PASS (cap 128)  — exit 0
( cd .jigc/worktrees/accept-a-name-of-exactly && ./check.sh )       # PASS (cap 2000) — exit 0
jigc task validate lower-the-name-cap            # exit 0; "no findings — the task validates clean"

# argv 3 — the join
jigc milestone join bound-inputs                 # exit 0
#   joined milestone:bound-inputs — 0 doc(s) merged
#     no docs staged from: accept-a-name-of-exactly, lower-the-name-cap
jigc milestone join bound-inputs --format json   # exit 0; {"findings":[],"milestone":"bound-inputs",
#   "no_docs_from":[…both…],"overlay":{},"schema_version":3}
# after: HEAD e13a011 unchanged; `git status --short` empty; `git worktree list` the same three rows;
#   sha of every file under .jigc/ identical except .jigc/index/edges.json
#   $REPO                                         cap=2000 longest-name=8
#   $REPO/.jigc/worktrees/accept-a-name-of-exactly cap=2000 longest-name=1017
#   $REPO/.jigc/worktrees/lower-the-name-cap       cap=128  longest-name=8
#   → no tree anywhere carries cap=128 together with the 1017-char name
./check.sh                                       # PASS (cap 2000) — exit 0, the main checkout is still base

# argv 4 — the boundary, no project hook
jigc milestone finalize bound-inputs             # exit 0
#   finalized 18e00f6 — Finalize milestone bound-inputs (2 sub-tasks)
#     modified cap.txt · modified docs/milestone-records/bound-inputs.md · modified names.txt
#     3 files committed
#     sub-tasks: accept-a-name-of-exactly: 1 code file · lower-the-name-cap: 1 code file
# after: HEAD 18e00f6; cap=128, longest-name=1017; worktrees torn down
./check.sh                                       # FAIL: name of 1017 chars exceeds cap 128 — exit 1

# rig 2 — "combines": the same path staged by two sub-tasks
#   (fresh rig; cap.txt committed; milestone "Collide", sub-tasks set-the-cap-to-128 / raise-the-cap-to-4096,
#    provisioned; each stages its own cap.txt)
jigc milestone join collide                      # exit 1; HEAD unchanged
#   join blocked: milestone:collide — 1 blocking finding(s); 0 doc(s) would merge, nothing committed
#   blocking · combine.code-collision — code collision — `cap.txt` (sub-tasks [raise-the-cap-to-4096,
#     set-the-cap-to-128]) staged by more than one worktree; the combine disjoint-applies code and never
#     text-merges a shared file
#     route: have the contending sub-tasks touch distinct files, or combine their overlapping changes by hand

# rig 3 — control: rig 1's sub-tasks, with the project's check as .git/hooks/pre-commit
jigc milestone join bound-inputs                 # exit 0 (as on rig 1)
jigc milestone finalize bound-inputs             # exit 1
#   `git commit` was rejected (no commit was made):
#   FAIL: name of 1017 chars exceeds cap 128
#   pre-commit: project check failed
#   milestone:bound-inputs is intact — nothing was committed, the merged docs were rolled back, and every
#   provisioned sub-task worktree still holds its staged code. Fix the hook's complaint, then re-run …
# after: HEAD unchanged; both worktrees still `M  cap.txt` / `M  names.txt`; three worktree rows
```

Origin, corroborated and not relied on: `~/out/RC24-A-frozen-work/turn02/.jigc/logs/invocations.jsonl`
holds 48 rows — zero `milestone join`, zero `milestone execute`, zero `milestone --help`; row 47 is
`milestone finalize … --format json` at exit 0. The main transcript's #62–#65 are the scratch
`git worktree add <tmp>/milestone-verify <base>`, three `git diff --cached` → `git apply`, the
combined test run (33 pass) and `git worktree remove --force`.

### The contract

None is contradicted. What the binary does is what its design of record and its own text say.

- **`milestone join` is not a combined-tree check, by statement.** `design/validation.md` →
  *"`jigc milestone join` is not a third conformance door, by design. It is a report-only pre-check
  that commits nothing"*. `design/finalize.md` → *`fan-out` finalize*, step 1: docs merge by the
  by-task-id join; code is combined "with **disjoint-apply + block-on-collision** … checked **up
  front**, before any commit", and under the default `squash: true` the combined tree is built
  "**off-line** (a throwaway index)" and "the main checkout is never mutated until that clean
  fast-forward". A combined tree an agent could run tests in before the boundary is designed out,
  not missing. The verb's help says "Commits nothing" and names docs only; it promises no tree.
- **The combined tree's test run has a designed channel, and it is the boundary's.**
  `design/finalize.md` → *`fan-out` finalize*: the aggregate commit runs "the user's
  `pre-commit`/`commit-msg` hooks against the combined tree", and "a blocked combine or a hook
  rejection lands nothing" — driven on rig 3, exactly so. *What `finalize` does NOT do* → "No
  `--no-verify`. Pre-commit and commit-msg hooks are the user's policy."
- **jigc states that it does not check correctness.** The composed sub-task text
  (`crates/cli/packs/dev/steps/implement.yaml`): "Finalize commits your staged work; it does not
  check that the work is correct." Exit 0 over a tree that fails the project's own tests is the
  stated behaviour, not a surface saying one thing while the binary does another.
- **Law 2 does not reach it.** `design/surface-contract.md` → *The three laws* → Law 2: "Every
  affordance that is the designated recovery for a state is named by the surfaces that produce that
  state". `milestone join` is a forward pre-check, not a recovery for any state orientation or the
  refusal produces; and for the need the worker had (a tree to test) it is not the affordance at
  all. The forward-routing silence is L-4's half (3), previously ruled "no qualified rule".

**What the trial adds, as an observation.** The one text that would have put `milestone join` in
front of the worker (`milestone execute`) tells the orchestrator to join and then finalize; it does
not say that sub-tasks which each pass alone can fail together, nor that the boundary's only test
of the combined tree is the project's own hook. A worker who wants that assurance before the
boundary has to build it outside jigc, as this one did — with or without the join named.

**Side observation, not this lead's.** `jigc milestone join --help` and `step:join-tasks` describe
the join as merging staged docs and name "a same-doc clash, an unknown milestone" as its blocking
findings; the binary also blocks on `combine.code-collision` (rig 2; M45 Increment 6). The surface
understates what the binary does — the opposite direction from a tier-3 predicate.

### Tier — none

- **Not tier 1.** `milestone finalize` at exit 0 lost nothing and harmed nothing: every staged
  byte of both sub-tasks is in `18e00f6`, attributed in the ack, and the repository is a clean
  fast-forward. A commit whose tree fails the project's tests is not a loss through a committing
  door — the predicate's second half (bytes found before, absent after, or a damaged repository
  state) has no instance here.
- **Not tier 2.** No refusal, no dead end: `join` and `finalize` both run from the checkout
  orientation routes from.
- **Not tier 3.** No surface claims a combined-code check, a combined tree, or a test run; the
  sub-task text says the opposite in so many words.
- The scorer's proposed tier (none — discoverability; jigc not running the project's tests is by
  design) stands, with the two corrections above: `milestone join` would not have served the need
  even if named, and the workaround did not catch the break.

### Pin

`pinned-by:` each driven behaviour has a suite; the absence the lead is about has none.

- join over disjoint staged worktrees is clean at exit 0 —
  `milestone_join_collision::disjoint_worktrees_keep_join_clean`
- join blocks a same-path code collision naming the path —
  `milestone_join_collision::colliding_worktrees_block_join_naming_the_path`
- join commits nothing and leaves the working tree unchanged —
  `milestone::milestone_join_suffixes_a_created_collision_and_reports_the_decision` (its
  before/after `git_state` assert)
- `milestone execute`'s composition is the text that names `jigc milestone join <id>` —
  `flow10_acceptance::the_three_emitted_milestone_run_lines_carry_the_resolved_id_and_run_verbatim`
- orientation routes a sub-task to `jigc milestone finalize <m>` —
  `orientation_active_task::a_sub_task_is_routed_to_the_milestone_door_and_omits_the_unneeded_consent`
- a hook rejection at the milestone boundary lands nothing and leaves the milestone intact —
  `commit_rejected_axis::every_committing_door_frames_its_rejection_names_itself_and_recovers` /
  `commit_rejected_axis::every_committing_door_keeps_its_frame_when_no_hook_spoke` (the
  `milestone:cache-rework is intact` row)

`UNPINNED:` (a) that no surface on the orientation → refusal → `provision` → sub-task-text path
names `milestone join` — the orientation test asserts the presence of the `milestone finalize`
route and the absence of `task finalize`, never the absence of other milestone doors, and a
negative over a path of surfaces has no natural fence; (b) that `milestone finalize` exits 0 over a
combined tree that fails the project's tests when no hook runs them — a non-behaviour by design
(jigc runs no project tests), with nothing to assert.

---

## L-18 · A refused `doc author` payload carries its only real coordinate (payload line/column) as prose in `message`, while the JSON `location` prints the synthesized "no coordinate known" sentinel `line 1, col 1` where the contract says `null`; the route's wording contradicts nothing — **CONFIRMED**

`tier:` 3 (weak) for the `location` half; none for the route half

**Lead (product):** A `write.wrong-shape` finding from `doc author` puts the parse position in its message (*line 6 column 9*) while its structured `location` says `line 1, col 1`, and its route says *retry … with a conforming value* without naming where the grammar is.
**Source:** arm a #20 (finding read); debrief-a item 3 (second bullet). **Door:** `jigc doc author`.
**Binary:** `~/.local/bin/jigc`, `jigc --version` → `jigc 1.0.0-rc.24` (asserted first). Release posture. `crates/` and `design/` at the working tree are byte-identical to tag `jigc-v1.0.0-rc.24` (`git diff --stat jigc-v1.0.0-rc.24 -- crates design` is empty), so the docs, sources and suites cited below are those of the driven binary.

### Verdict

**CONFIRMED** as observed — tier **3 (weak)** for the `location` half, tier **none** for the route half. The scorer's proposed tiers stand.

Both halves reproduce byte for byte on a fresh rig, on the session's own payload shape (an item carrying `set-fields:`).

- **The `location` half is a real, small surface defect.** The finding's only known source coordinate — the payload's line 6, column 9 — exists as prose inside `message` and nowhere else. The structured `location` carries `{"address": "deferral-ledger", "line": 1, "col": 1}`, and that `1:1` is not a coordinate at all: it is the sentinel `doc::stamp_target` synthesizes when a finding arrives with no location and needs somewhere to hang its key target. The agent-text arm knows this and suppresses it (`at: deferral-ledger`, no line); the `--format json` arm prints it as if it were a position. The contract says `null` for that case.
- **The route half contradicts nothing.** The route is true (followed, it exits 0), the message itself names the admissible keys at the failing node, and the composed step names the grammar's home before the write. The session recovered in one call. It is an ergonomics observation, not a defect.

Two things narrow the lead rather than widen it:

1. **The two positions are not two answers to one question.** *line 6 column 9* indexes the **payload** on stdin; `location.address` is the bare **doctype id** (no instance is addressed when a payload is refused), which has no lines to index. So this is not "the binary reports two different lines for one fault"; it is "the one real coordinate is prose-only, and the structured field carries a placeholder".
2. **The `1:1` is not specific to this code or this door.** Every write finding stamped by `stamp_target` carries it (cells 6, 7 and 9 below). What is specific to the payload parse is that a real coordinate exists to be lost.

### Repro

```text
# setup — fresh rig, installed registry binary. The variable the co-author trailer keys on
# is SET in every cell (held constant, value never printed); no cell commits anything.
rig=$(dev/jigc-rig fresh --binary ~/.local/bin/jigc \
        --start planning "Bound what the service will accept") || exit
eval "$rig"; [ -n "$REPO" ] || exit          # REPO=<tmp>/jigc-rig-fresh-XXXXXX/repo
$JIGC --version                              # jigc 1.0.0-rc.24
T=$RIG_TASK                                  # bound-what-the-service-will
git -C "$REPO" rev-list --count HEAD         # 2
jigc doc create deferral-ledger --title "Deferral Ledger" --task $T --format json   # exit 0

# before-control — the staged doc
$ jigc doc show deferral-ledger:deferral-ledger --task $T                           # exit 0
  ---
  schema-version: 2
  ---
  # Deferral Ledger
  ## Entries

# cell 1 — THE LEAD. bad.yaml, `set-fields:` on line 6, column 9:
#   1 title: "Deferral Ledger"
#   2 sections:
#   3   - id: entries
#   4     items:
#   5       - title: "Configurable ingest limits"
#   6         set-fields:
#   7           kind: Decision
#   8           trigger: "a later milestone"
#   9         set:
#  10           body: |-
#  11             <<The caps are module-local constants.>>
$ jigc doc author deferral-ledger --from-file - --task $T --format json < bad.yaml  # exit 1
  { "schema_version": 3,
    "findings": [ {
      "severity": "blocking", "probe": "write", "check": "wrong-shape",
      "code": "write.wrong-shape",
      "key": { "code": "write.wrong-shape", "target": "deferral-ledger" },
      "message": "write rejected: the `doc author` payload is not the declared grammar — sections[0].items[0]: unknown field `set-fields`, expected one of `title`, `set`, `sections` at line 6 column 9",
      "location": { "address": "deferral-ledger", "line": 1, "col": 1 },
      "route": "retry `jigc doc author deferral-ledger` with a conforming value" } ] }

# cell 1, agent text — the same refusal; the sentinel line is suppressed here
$ jigc doc author deferral-ledger --from-file - --task $T < bad.yaml                # exit 1
  blocking · write.wrong-shape — write rejected: the `doc author` payload is not the declared grammar — sections[0].items[0]: unknown field `set-fields`, expected one of `title`, `set`, `sections` at line 6 column 9
    at: deferral-ledger
    route: retry `jigc doc author deferral-ledger` with a conforming value

# cells 2-4 — the position in the message moves with the payload; `location` never does
#   2  broken YAML (`  : :` on line 2)      message "… at line 2 column 3, while parsing a block mapping"   location 1:1   exit 1
#   3  `sectons: []` on line 2              message "… expected `title` or `sections` at line 2 column 1"   location 1:1   exit 1
#   4  an item with no `title:`             message "… missing field `title` at line 5 column 9"            location 1:1   exit 1
# all three: code write.wrong-shape, target "deferral-ledger", the same route as cell 1

# cells 6-7 — control: the SAME code where no payload coordinate exists (an address-shape defect)
#   6  doc author, payload `sections: [{id: entries, set: {entries: <<prose>>}}]`      exit 1
#   7  doc set-slot deferral-ledger:deferral-ledger#entries --from-file -              exit 1
#   both: code write.wrong-shape, target "deferral-ledger:deferral-ledger#entries",
#         location { address: <that target>, line: 1, col: 1 },
#         route "`jigc doc schema deferral-ledger` to see the declared shape, then re-run the write at a declared address"

# cell 9 — control: the payload parse's other code
#   doc author, an item whose `body:` is bare prose                                    exit 1
#   code write.malformed-value, target "deferral-ledger", location 1:1,
#   route "retry `jigc doc author deferral-ledger` with a conforming value"

# cell 8 — the route followed: bad.yaml with `set-fields:` merged into `set:`
$ jigc doc author deferral-ledger --from-file - --task $T --format json < good.yaml  # exit 0
  { "copied_in": false, "findings": [], "op": "author",
    "target": { "doctype": "deferral-ledger", "slug": "deferral-ledger" } }

# after
#   after cells 1-7: `doc show … --task $T` byte-identical to the before-control (nothing staged by a refusal)
#   after cell 8:    the staged doc holds `### Configurable ingest limits  {#configurable-ingest-limits}`
#   git -C "$REPO" rev-list --count HEAD → 2 throughout (no cell commits)

# where the grammar is, and where it is not
$ jigc doc author --help            # the payload grammar: `title:` / `sections:` / `set:` / `items:` (the block under "title: <the create id-source>")
$ jigc doc schema deferral-ledger   # the doctype's shape, annotated `(set-field: …)` / `(set-slot: …)` / `(add-item: …)` — no payload key grammar
$ jigc start --task $T              # the composed step names the home once, before the write: "(grammar: `jigc doc author --help`)"
```

Origin, for the record (not the proof): arm a's invocation log row 20 is `doc author deferral-ledger --from-file - --task bound-what-the-service-will --format json`, exit 1, `write.wrong-shape`; the session's transcript holds the same finding with the same three values; row 21 is `doc author --help` two seconds later and row 22 the same verb at exit 0. The debrief's own words: *"I only read `jigc doc author --help` reactively, after the error, not before writing."*

### Contract

**The `location` half — contradicted.**

- `design/command-output-contract.md` → *3 · The findings envelope*, the `location` bullet: *"line/col when the finding has a source position, else `null`. **Advisory only** — … a convenience pointer for a human reader."* The envelope sample beside it gives `"location": { "line": <n>, "col": <n> } | null`. The binary emits a non-null `location` whose `line`/`col` are not a source position, for a finding that does have one (in the payload) and states it only in prose.
- `design/validation.md` → the advisory-route floor, the parser-diagnostic exemption bullet: *"**Line 1 is this codebase's *no coordinate known*** — every producer that synthesizes a location purely to carry an address stamps `1:1` (… `doc::stamp_target` …) — so a line-1 location contributes its address alone"*. That rule is implemented for the **text** arm only: `crates/cli/src/render.rs` → `finding_locus`, whose own comment says *"rendering `line 1` would be a coordinate the finding does not actually claim"*. The JSON arm serializes exactly that unclaimed coordinate.
- Mechanism, read from source: `crates/cli/src/author.rs` → `reject` mints the finding with no location and no route (`Finding::graded(…, None, None)`); the serde error's `Display` (which ends in `at line N column M`) is formatted into `message`; `crates/cli/src/doc.rs` → `stamp_target` then fills `None => Location::addressed(subject, 1, 1)` and `block` fills the route.

**The route half — not contradicted.**

- `design/surface-contract.md` → *Law 2 — nothing hides* requires the designated recovery for a state to be named by the surface that produces it. The recovery for a refused payload is a re-run with a conforming one; the route names it, with the verb and doctype filled, and following it exits 0 (cell 8). No design sentence designates a grammar reference as that recovery.
- *Law 3 — nothing ambushes* is met upstream: the composed planning step that solicits the batch write names `jigc doc author --help` as the grammar before the write.
- The message carries the local grammar itself: *"expected one of `title`, `set`, `sections`"*.

### Tier

- **`location`: tier 3 (weak).** A surface says something the binary does not mean: the `--format json` envelope prints `line: 1, col: 1` where the contract says `null`, and the same binary's text arm declines to print that line because it is not a coordinate. Weak because the field is declared advisory-only and outside the stable key, the `(code, target)` key and the message are both correct, and no driver decision in the trial turned on it. Not tier 2: nothing dead-ends. Not tier 1: exit 1, nothing staged, nothing committed (before/after `doc show` identical, commit count 2 throughout).
- **Route: tier none.** No contract is contradicted. What the binary does instead: it names the re-run, which works; the grammar's home is `jigc doc author --help`, named by the composed step rather than by the finding.

### Pin

**UNPINNED** — for both halves.

- `author_payload_floor::every_payload_refusal_emits_the_findings_envelope_with_a_resolving_key` drives this exact refusal class (`PayloadReject::ALL`, including the unknown-key and broken-YAML cells) under `--format json` and asserts the envelope key set, `findings[0].key`, `severity`, and that `route` **is a string**. It asserts nothing about `location.line` / `location.col` and nothing about the route's text.
- `author_payload_floor::every_payload_refusal_answers_with_a_severity_a_code_an_at_and_a_route` pins the text arm's `\n  at: <doctype>\n` — i.e. it pins the suppression of the sentinel on text, which is the half that is right.
- `located_finding_text` cross-reads `location.address` and line between the two arms for a finding with a real line, and a `location: null` singleton; neither arm covers a synthesized `1:1`.
- The route sentence *"with a conforming value"* appears once in the tree (`crates/cli/src/doc.rs` → `block`) and in no test, golden or design doc.

### Notes

- **Adjacent, not part of the lead, driven above (cells 6, 7 vs 1):** one code, `write.wrong-shape`, leaves this door with two different routes. An address-shape defect gets the engine's per-code route (`jigc doc schema <doctype> …`, `crates/engine/src/write.rs` → `write_route`); an ungrammatical payload gets the CLI fallback (`retry … with a conforming value`). `design/validation.md` → *The `write.*` route split* defines `write.wrong-shape` as *"a **genuine declared-shape defect**, and only that"* and says the first three codes *"route the mechanical `jigc doc schema <doctype>` read"*; `design/command-output-contract.md` → *Evolution posture* (the M47 correction) says *"`write.wrong-shape` routes `jigc doc schema <doctype>`"*. The payload-parse producer (M51 Increment 6; `PayloadReject::code`) is a second meaning those sentences do not carry — including for broken YAML (cell 2), where no schema is in question. The fallback route is the better of the two here: `jigc doc schema deferral-ledger` holds no payload key grammar, and its `(set-field: …)` annotations are the nearest visible source of the session's `set-fields:` guess (the session had read `doc schema deferral-ledger` at row 10). Offered as a separate lead for whoever triages; not scored here.
- The route's noun is slightly off for this refusal — the defect is a payload **key**, not a *value* — but the sentence is shared by every route-less write block and is not false.
- Not driven: `--format human` (the third format value); a payload from a file path rather than stdin (the same parse, so the same finding is expected, but it was not run).

---

## L-19 · `doc list --format json` with an open task: the `note: docs are also staged in open task …` line rides stderr; stdout is exactly one JSON document, byte-identical with and without the task — **REFUTED**

`tier:` none

**Door:** `doc list` · **Kind:** product · **Binary:** `jigc 1.0.0-rc.24` (the installed registry build, release posture)

### The claim

`jigc doc list --format json` prints a `note: docs are also staged in open task …` line after the
JSON document when a task is open; both trial captures merged stderr into stdout (`2>&1`), so
which stream carries the note was unread. The scorer's fork: tier 3 if the note is on stdout
(stdout is then not one JSON document), none if it is on stderr.

### Verdict

**REFUTED** as a defect. The observation is real — the note prints, and in a merged capture it
follows the document — but the note rides **stderr**. Stdout under `--format json` is exactly one
JSON document, byte-identical with and without the open task. No contract is contradicted; the
binary does what three surfaces say it does.

The falsifying datum: with the streams split, stdout is 1132 bytes that parse as one document
(top-level key `docs`) and contain no `note:`; stderr is the 208-byte note line alone. The
scorer's own repro (`… 2>/dev/null | python3 -m json.tool`) exits 0.

### Repro block

```
setup
  rig=$(dev/jigc-rig refs-post-hoc --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit
  # state: committed-singletons + research, and a LIVE task
  # ($RIG_TASK = ground-the-vision-in-research) holding a staged doc
  W=$(mktemp -d)
  $JIGC --version                      -> jigc 1.0.0-rc.24

A. json, streams split
  argv   $JIGC doc list --format json >$W/a.out 2>$W/a.err
  exit   0
  stdout 1132 bytes; json.loads -> ONE document, top-level keys ['docs']; contains "note:" = False
  stderr 208 bytes, one line:
         note: docs are also staged in open task ground-the-vision-in-research — this listing is
         the committed store; if that task is yours, list what it stages:
         `jigc doc list --task ground-the-vision-in-research`

B. the scorer's proposed repro
  argv   $JIGC doc list --format json 2>/dev/null | python3 -m json.tool   (pipefail scoped)
  exit   0 — the document parses

C. the trial's capture shape (streams merged)
  argv   $JIGC doc list --format json >$W/c.merged 2>&1
  exit   0
  then   python3 -m json.tool <$W/c.merged -> exit 1, "Extra data: line 56 column 1 (char 1132)"
         char 1132 is exactly where stdout's document ends; the last line is the stderr note.
         This is the artifact the two sessions saw — the merge is the caller's, not the binary's.

D. plain format, streams split
  argv   $JIGC doc list >$W/d.out 2>$W/d.err
  exit   0; "note:" lines on stdout = 0, on stderr = 1

E. control — the same listing with no open task
  argv   $JIGC task discard $RIG_TASK --force          -> exit 0
         $JIGC doc list --format json >$W/e.out 2>$W/e.err
  exit   0; stderr 0 bytes
  after  cmp $W/a.out $W/e.out -> identical: stdout is byte-identical with and without the task
```

Origin, read only: the two sessions ran `jigc doc list --format json 2>&1` (arm c, `~/out/RC24-C`)
and `jigc doc list --format json 2>&1 | head -100` (arm a, `~/out/RC24-A-frozen-work`). Both
merge fd 2 into fd 1, which is cell C above.

### The contract

Nothing is contradicted. The behaviour is stated on three surfaces, all agreeing with the binary:

- `design/doc-read-surface.md` → the fourth read surface, *"A task-less listing routes at the
  staged one"*: the note is on **stderr**; stdout — the pinned json and the plain listing,
  empty-set line included — is **byte-identical** with and without an open task.
- `design/command-output-contract.md` → *Stream discipline*, the rule: under `--format json` the
  structured document owns stdout and every agent-text side channel goes to stderr; the
  JSON-bearing stream parses as exactly one document.
- `jigc doc list --help`: *"A task-less listing served while an open task stages docs says so on
  **stderr** and hands over the staged listing; stdout is unchanged."*

The same doc already records this exact misreading as a known harness-side effect:
`design/command-output-contract.md` → *Stream discipline* opens with three earlier sessions
reporting "emits non-JSON" for a sibling verb, refuted the same way — agent harnesses merging
fd 1 and fd 2 in arrival order.

Source: `crates/cli/src/doc.rs` → `staged_listing_hint` writes the line with `eprintln!`.

### Tier

**none.** Tier 3 needs a surface that says something the binary does not do; here the help text,
the two design docs and the binary agree. No loss, no dead end: exit 0, the document is intact,
and the note's own route (`jigc doc list --task <id>`) is a working command.

Not claimed here: whether an agent that habitually appends `2>&1` is well served by a stderr
advisory under `--format json` is a harness-ergonomics observation, not a defect of this door,
and no contract promises a merged stream parses.

### Pin

`pinned-by: doc_list::a_task_less_listing_routes_at_the_staged_read`
(`crates/cli/tests/doc_list.rs`, registered in `crates/cli/tests/groups/g_doc.rs`). It asserts,
with an open staging task, that `doc list --format json`'s stdout equals the pinned `STORE_JSON`
byte for byte and that the advisory is on stderr; it also runs the emitted route verbatim.
`doc_list::a_narrowed_listing_names_only_tasks_staging_that_doctype` asserts the stderr placement
again on both format arms for the narrowed listing.

---

## L-20 · Sub-task composed text carries step:implement's un-scoped "before (you) finalize" wording beside "never `jigc task finalize` here" — text confirmed verbatim, the contradiction not carried — **PARTIAL**

`tier:` none

**Kind:** product · **Door:** `workflow:sub-task` (composed through `jigc start --task <sub>` and
`jigc workflow sub-task --task <sub>`) · **Origin:** arm a, invocation #43
(`start --task cap-the-store-at-1000 --format json`, exit 0, in `~/out/RC24-A-frozen-work/turn02`).

### The claim

The sub-task composed text says "`git add` your code edits before finalize — it commits only
what you have staged" and "Before you finalize, verify the change", and offers
`jigc doc create adr --task <sub-task>`, before ending "never `git commit` and never
`jigc task finalize` here". The scorer's reading: two sentences of one text point opposite ways
(tier 3, weak) — or none.

### Verdict

**PARTIAL.** The text half is confirmed verbatim on a fresh rig. The "points opposite ways"
half is not carried: every proposition the surviving sentences make about *finalize* is true of
the one boundary a sub-task has (`jigc milestone finalize <m>`), the ADR offer is a working,
declared create-gate, and the one door the wording could mis-send a reader to refuses at exit 3
with a route and moves nothing. No contract is contradicted. **Tier: none.**

### Repro

```
# setup — installed registry build, release posture; CLAUDECODE set (held constant, value not read)
jigc --version                                  -> jigc 1.0.0-rc.24
rig=$(dev/jigc-rig fresh --binary <installed jigc>) || exit; eval "$rig"; [ -n "$REPO" ] || exit
printf 'export const a = 1;\n' > a.ts; git -C "$REPO" add a.ts; git -C "$REPO" commit -qm "chore: seed"
jigc milestone create "Two helpers"             -> exit 0   minted milestone:two-helpers (shared base 8b744d2)
jigc milestone add-task two-helpers "add helper b"   -> exit 0
jigc milestone add-task two-helpers "add helper c"   -> exit 0
jigc milestone provision two-helpers            -> exit 0   provisioned 2 worktree(s) … (add-helper-b, add-helper-c)

# argv 1 — the lead's door, from the sub-task's worktree
cd "$REPO/.jigc/worktrees/add-helper-b"
jigc start --task add-helper-b                  -> exit 0, 72 lines, stderr empty
jigc workflow sub-task --task add-helper-b      -> exit 0, byte-identical to the above

# observed — every line carrying "finalize" (line numbers of the 72-line text)
 7-8   Implement the change directly in the working tree. `git add` your code edits
       before finalize — it commits only what you have staged.
 14    Run: `jigc doc create adr --title <TITLE> --task add-helper-b`
 44    conformant and never blocks finalize:
 48-50 Before you finalize, verify the change actually works: build it and run the
       tests, and confirm the behaviour you set out to produce. Finalize commits your
       staged work; it does not check that the work is correct.
 63-67 The parent milestone's finalize is the only commit boundary: `git add` your code
       edits so the milestone can fold this worktree's staged index, but never
       `git commit` and never `jigc task finalize` here — either lands a commit on this
       worktree's detached HEAD and strands the sub-task's work outside the milestone boundary.
 69    what's-left: `jigc task validate add-helper-b`   — previews part of the finalize gate: …
 70    task scope: … this task is a sub-task of milestone `two-helpers`, whose
       `jigc milestone finalize two-helpers` is its only commit boundary — every door here
       stays pinned to the milestone's base
 71    create-gates: adr

# argv 2 — what the binary does if a reader takes "Before you finalize" as the per-task door
cd "$REPO/.jigc/worktrees/add-helper-c"; jigc start --task add-helper-c   -> exit 0
printf 'export const c = 3;\n' > c.ts; git add c.ts
git rev-parse --short HEAD                      -> 8b744d2
jigc task validate add-helper-c                 -> exit 0   no findings — the task validates clean
jigc task finalize add-helper-c                 -> exit 3
  blocking · finalize.milestone-sub-task — task `add-helper-c` is a sub-task of milestone
  `two-helpers` — the parent milestone's finalize is the only commit boundary; …
    route: `jigc milestone finalize two-helpers` — the milestone finalize folds every
    sub-task's staged work into the one aggregate commit
git rev-parse --short HEAD                      -> 8b744d2   (unmoved; `A  c.ts` still staged)

# argv 3 — the ADR offer, taken as printed
cd "$REPO/.jigc/worktrees/add-helper-b"; printf 'export const b = 2;\n' > b.ts; git add b.ts
jigc doc create adr --title "Helper b is a constant" --task add-helper-b   -> exit 0  adr:helper-b-is-a-constant
jigc doc set-slot adr:helper-b-is-a-constant#{context,decision,consequences} --from-file - --task add-helper-b
                                                -> exit 0 ×3
cd "$REPO"; jigc milestone finalize two-helpers -> exit 0
  finalized 4fe62bb — Finalize milestone two-helpers (2 sub-tasks)
    added b.ts
    added c.ts
    promoted docs/decisions/helper-b-is-a-constant.md
    modified docs/milestone-records/two-helpers.md
    4 files committed
    sub-tasks: add-helper-b: 2 docs, 1 code file · add-helper-c: 1 code file

# argv 4 — "it commits only what you have staged", measured against the milestone boundary
jigc milestone create "Staged only"; jigc milestone add-task staged-only "add s"
jigc milestone provision staged-only; cd "$REPO/.jigc/worktrees/add-s"; jigc start --task add-s
printf … > s.ts; git add s.ts; printf … > u.ts          # git status: `A  s.ts` / `?? u.ts`
cd "$REPO"; jigc milestone finalize staged-only  -> exit 0
  finalized 7ba304a — Finalize milestone staged-only (1 sub-task)
    added s.ts
    2 files committed
    discarded with the fan-out worktrees (not committed, not recoverable):
      add-s: u.ts (never staged)

# after
git show --stat HEAD: s.ts and the milestone record only; u.ts in no commit. Worktree removed.
```

### What the evidence carries, sentence by sentence

| Sentence | Read against the sub-task's only boundary | Result |
|---|---|---|
| "`git add` your code edits before finalize — it commits only what you have staged" | the milestone finalize folded the staged `s.ts` and left the never-staged `u.ts` out (argv 4) | **true**; and it agrees with line 64, which gives the same `git add` instruction |
| "Finalize commits your staged work; it does not check that the work is correct" | argv 3 and 4 committed staged work with no build or test run | **true** |
| `Run: jigc doc create adr … --task <sub>` | created, authored, promoted to `docs/decisions/` at the milestone finalize (argv 3) | **works**; declared by the workflow's `allows-create: [{type: adr, as: decision}]` and named back on the `create-gates:` line. Creating a doc commits nothing, so it does not oppose "never `git commit`" |
| "Before **you** finalize, verify the change" | the reader is a sub-agent with no finalize door of its own | **imprecise addressee, no false statement** — the advice (verify before the work is committed) holds; the same text then forbids the per-task door and the `task scope:` trailer names the milestone door |

The only residue is the second-person "Before you finalize" in `step:implement`, written for
a top-level task and composed unchanged into a sub-task. A reader who acts on it literally
reaches `jigc task finalize <sub>`, which exits 3 with `finalize.milestone-sub-task` and a
mechanical route to `jigc milestone finalize <m>`; HEAD is unmoved and the staged file stays
staged (argv 2). In arm a no session took that door: invocations #43–#45 compose the three
sub-tasks and #48 is `milestone finalize`, with no `task finalize` on a sub-task in between.

### The contract

- `design/workflow-dialect.md` → Emitted format → *Composing for a fan-out sub-task (M55)*:
  the CLI omits a step from a sub-task's composed text **when its own body carries a
  `{{cli.<id>}}` whose catalog entry runs `jigc task finalize`** (or it includes such a step),
  plus the commit-doc author under `finalize.fan-out.squash: true`. `step:implement` carries
  `{{ cli.create-adr }}` only, so by the declared rule it survives whole — the binary does
  what the doc says. The observed text also matches the rule's other half: the nested
  `step:author-commit` is absent and the trailer follows `step:sub-task-commit` directly.
- `design/findings-channel.md` §6, row S2: the omission is meant to remove "every false line,
  not only `Run:`". None of the surviving `step:implement` lines is false of the milestone
  boundary (table above), so this sentence is not contradicted either.
- `design/workflow-dialect.md` → the fan-out primitive: sub-agents "`git add` their code in
  their own worktree … but never commit" — which is what both the `implement` sentence and the
  `sub-task-commit` framing instruct.
- `design/surface-contract.md` → Law 1 (nothing lies): no claim in the quoted sentences is
  falsified by the binary. Law 3 (nothing ambushes): the constraint (no per-task finalize) is
  stated in the text before it can bind, and again on the `task scope:` line.

No contract contradicted → not a defect under the trial's predicate.

### Tier

**none.** Tier 3 needs a surface saying something the binary does not do. The lead's three
quoted sentences and the ADR offer each describe what the binary does at the milestone
boundary. Tier 2 is excluded: the per-task door a literal reader could reach refuses with a
working route. Tier 1 is excluded: nothing was committed, destroyed or moved by any door the
quoted sentences send a reader through.

### Pin

`UNPINNED: no test asserts or forbids step:implement's un-scoped "finalize" wording in a
sub-task composition.` What is pinned around it:

- `sub_task_composition::a_sub_task_composes_no_per_task_finalize_door_through_either_door`
  (group `g_methodology`) fences the literal `jigc task finalize` outside the
  "never `…`" phrasing — the bare word "finalize" is outside its detector, and its five
  workflows (`amend`, `migrate-idea`, `migrate-adr`, `planning`, `park-idea`) do not include
  the native `sub-task`.
- `sub_task_composition::t2_ab_the_commit_doc_author_follows_the_squash_knob` and
  `workflow_reentry::workflow_reentry_composes_w_with_the_equality_guard` pin the
  "never `git commit` and never `jigc task finalize`" framing in the native sub-task's text.
- `milestone::task_finalize_on_a_milestone_sub_task_refuses_with_the_milestone_route` and
  `doc_only_finalize::a_milestone_sub_task_still_refuses_the_per_task_finalize` pin the exit-3
  refusal and that it lands no commit on the worktree's detached HEAD.
- The compose goldens `start--sub-task--*.txt` / `workflow-preview--sub-task--*.txt` pin the
  `workflow.verb-routed` refusal of composing `sub-task` by name, not this body.

### Notes

1. **Adjacent observation, outside this lead's claim, driven in argv 2.** The closing sentence
   says of `git commit` and `jigc task finalize` that "either lands a commit on this worktree's
   detached HEAD and strands the sub-task's work". For the `jigc task finalize` half the binary
   does not do that: it refuses at exit 3 and HEAD stays at the base. That clause is a
   behaviour claim the binary contradicts (surface-contract Law 1; workflow-dialect's own M55
   paragraph says the door "refuses"), and the refusal is pinned by
   `milestone::task_finalize_on_a_milestone_sub_task_refuses_with_the_milestone_route`. It
   would be a tier-3 candidate as its own lead; the `git commit` half was not driven here.
2. Argv 4's exit-0 discard of a never-staged file is disclosed twice on the finalize's own
   output and is not part of this lead; it is recorded only as the datum that makes "it commits
   only what you have staged" true of the milestone boundary.
3. Not tested: `finalize.fan-out.squash: false`, where the nested author step returns and the
   join reads each sub-task's commit doc. The sentences this lead quotes come from
   `step:implement` and `step:sub-task-commit`, which compose in both modes.
4. Sources of the two steps: `crates/cli/packs/dev/steps/implement.yaml`,
   `crates/cli/packs/dev/steps/sub-task-commit.yaml`; workflow
   `crates/cli/packs/dev/workflows/sub-task.yaml`.

---

## L-21 · `jigc start --task <sub>` inside the worktree is a designed second door: it composes at exit 0 and prints the same bytes as `jigc workflow sub-task --task <sub>`; only the workflow door provisions the sub-task's docs area — **CONFIRMED**

`tier:` none

**Kind:** product · **Doors:** `jigc start --task`; `jigc workflow <W> --task` ·
**Binary driven:** the installed registry build, `jigc --version` → `jigc 1.0.0-rc.24` (release
posture; asserted before the first probe). `CLAUDECODE` held as the session has it (set) in
every cell; no cell here commits through a compared door.

### The claim

`jigc start --task <sub-task>` run inside the sub-task's worktree composes the sub-task
workflow at exit 0 — a second door beside `jigc workflow sub-task --task <id>`, which
protocol §0.2 names as the route. Whether the two print the same text was unread.

### Verdict

**CONFIRMED as stated — and it is not a defect. Tier `none`.**

1. **The second door exists and composes at exit 0 (confirmed).** On a fresh rig, from a
   provisioned sub-task worktree, `jigc start --task <sub>` exits 0 with stderr empty and prints
   the sub-task composition.
2. **The unread half, now read: the two doors print the same bytes.** Agent text and
   `--format json` are byte-identical between the doors, for the same sub-task, in every order
   driven (start first, workflow first, start again after the workflow door), under the default
   `finalize.fan-out.squash = true` and under `false`. `diff` is empty; one SHA-256 per format.
3. **From the main checkout both doors refuse the same way** (exit 1, the same sentence), and
   each routes to *"re-run this command there"* — so the refusal at arm a #41 routes to
   whichever door was typed, not specifically to `start --task`.
4. **The one thing the doors do not share is a side effect, and the design says so.**
   `jigc workflow sub-task --task <sub>` provisions the sub-task's `docs/` (the commit doc and
   `provenance.json`) on first entry; `jigc start --task <sub>` writes nothing. Under the
   default squash — the trial's posture — nothing reads a sub-task's commit doc and the composed
   text solicits no write to it, so a worker cannot tell the doors apart. Under
   `squash: false` the composed text (identical through both doors) keeps the commit-doc author
   lines; in a start-only area the first of them refuses at exit 1 **with a route** to
   `jigc workflow sub-task --task <sub>`, and the route, run verbatim, makes the same write land.
   A routed refusal whose route works — not a dead end. The composed text's own `resume:` line
   already names that door and says it *"provisions this sub-task's write-ready docs area on
   first entry"*.

Nothing was lost, moved or committed by either door in any cell.

### Repro

```
# setup — rig 1, fresh, installed binary, default knobs
rig=$(dev/jigc-rig fresh --binary <installed jigc>) || exit; eval "$rig"; [ -n "$REPO" ] || exit
$JIGC --version                                             # jigc 1.0.0-rc.24
$JIGC milestone create "Bound the service"                  # exit 0 — milestone:bound-the-service, base d45b662
$JIGC milestone add-task bound-the-service "Cap the store"      # exit 0 — task:cap-the-store
$JIGC milestone add-task bound-the-service "Reject long lines"  # exit 0 — task:reject-long-lines
cat .jigc/tasks/cap-the-store/workflow                      # sub-task

# cell 0 — both doors from the MAIN checkout, before provisioning
$JIGC start --task cap-the-store                            # exit 1
$JIGC workflow sub-task --task cap-the-store                # exit 1 — the same sentence:
#  task `cap-the-store` is pinned to base d45b662 but you're on 8b06861 — this is a sub-task of
#  milestone `bound-the-service`, and a sub-task's work happens in its own worktree, … run
#  `jigc milestone provision bound-the-service` … then `cd $REPO/.jigc/worktrees/cap-the-store`
#  and re-run this command there
$JIGC start --workflow sub-task "x"                         # exit 1
#  blocking · workflow.verb-routed — workflow `sub-task` is not composed by name: …
#    route: `jigc workflow sub-task --task <task-id>`
find .jigc/tasks -type f | sort                             # per sub-task: base.json, intent, workflow — no docs/

$JIGC milestone provision bound-the-service                 # exit 0 — provisioned 2 worktree(s) … at base d45b662

# cell A — sub-task 1, FIRST entry through `start --task`
cd "$REPO/.jigc/worktrees/cap-the-store"
$JIGC start --task cap-the-store > <tmp>/t1-start-first.out   # exit 0, stderr 0 bytes, stdout 4305 bytes
$JIGC start --task cap-the-store --format json > <tmp>/t1-start-first.json   # exit 0
find "$REPO/.jigc/tasks/cap-the-store" -type f              # base.json, intent, workflow — STILL no docs/

# cell B — sub-task 2, FIRST entry through `workflow sub-task --task`
cd "$REPO/.jigc/worktrees/reject-long-lines"
$JIGC workflow sub-task --task reject-long-lines > <tmp>/t2-wf-first.out     # exit 0, stderr 0 bytes
find "$REPO/.jigc/tasks/reject-long-lines" -type f
#  base.json, intent, workflow, docs/commit:reject-long-lines.md, docs/provenance.json

# cell C — the other door on each, then the diffs (same sub-task on both sides)
cd "$REPO/.jigc/worktrees/cap-the-store"
$JIGC workflow sub-task --task cap-the-store > <tmp>/t1-wf-second.out        # exit 0 (provisions docs/ now)
$JIGC workflow sub-task --task cap-the-store --format json > <tmp>/t1-wf.json   # exit 0
$JIGC start --task cap-the-store > <tmp>/t1-start-third.out                  # exit 0
$JIGC start --task cap-the-store --format json > <tmp>/t1-start-third.json   # exit 0
diff <tmp>/t1-start-first.out <tmp>/t1-wf-second.out        # empty, rc 0
diff <tmp>/t1-wf-second.out   <tmp>/t1-start-third.out      # empty, rc 0
diff <tmp>/t1-start-first.json <tmp>/t1-wf.json             # empty, rc 0
diff <tmp>/t1-wf.json <tmp>/t1-start-third.json             # empty, rc 0
#  sha256 2a128ed6…cd4a1843a  ×3 (agent text)   ·   8086f6de…478f24f739 ×3 (json)
cd "$REPO/.jigc/worktrees/reject-long-lines"
$JIGC start --task reject-long-lines > <tmp>/t2-start-second.out             # exit 0
diff <tmp>/t2-wf-first.out <tmp>/t2-start-second.out        # empty, rc 0

# the composed text, through either door, ends:
#  … never `git commit` and never `jigc task finalize` here — …
#  resume: `jigc workflow sub-task --task cap-the-store`   — re-composes this workflow and
#    provisions this sub-task's write-ready docs area on first entry; …
#  what's-left: `jigc task validate cap-the-store` …
#  task scope: … whose `jigc milestone finalize bound-the-service` is its only commit boundary …
#  create-gates: adr

# cell D — a start-only area is still writable for what the default-squash text solicits
$JIGC milestone add-task bound-the-service "Start only"; $JIGC milestone provision bound-the-service   # exit 0, 0
cd "$REPO/.jigc/worktrees/start-only"; $JIGC start --task start-only        # exit 0, no docs/
$JIGC doc create adr --title "Why cap" --task start-only    # exit 0 — adr:why-cap; docs/ now holds
#  adr:why-cap.md + provenance.json, and NO commit:start-only.md
$JIGC doc show commit:start-only --task start-only          # exit 1 — blocking · store.not-staged

# setup — rig 2, fresh, the non-default knob
rig=$(dev/jigc-rig fresh --binary <installed jigc>) || exit; eval "$rig"; [ -n "$REPO" ] || exit
$JIGC milestone create "Bound the service"
$JIGC milestone add-task bound-the-service "Via start"
$JIGC milestone add-task bound-the-service "Via workflow"
$JIGC config set finalize.fan-out.squash false              # exit 0 — written to `.jigc/config/`, uncommitted
$JIGC milestone provision bound-the-service                 # exit 0

# cell E — one sub-task per door under squash: false
cd "$REPO/.jigc/worktrees/via-start";    $JIGC start --task via-start > <tmp>/s.out               # exit 0, no docs/
cd "$REPO/.jigc/worktrees/via-workflow"; $JIGC workflow sub-task --task via-workflow > <tmp>/w.out # exit 0, docs/ provisioned
diff <(sed 's/via-start/SUB/g' <tmp>/s.out) <(sed 's/via-workflow/SUB/g' <tmp>/w.out)
#  2c2  < Via start  /  > Via workflow        — the intent line only; every other byte equal
grep -n 'commit:' <tmp>/s.out                               # the author lines survive under `false`, e.g.
#  88:Run: `jigc doc set-field commit:via-start#type --value <COMMIT_TYPE> --task via-start`

# cell F — the solicited write in the start-only area, then its printed route, then the write again
cd "$REPO/.jigc/worktrees/via-start"
$JIGC doc set-field commit:via-start#type --value fix --task via-start      # exit 1
#  no staged instance for `commit:via-start#type` — task `via-start`'s workflow provisions its
#  `commit` doc when the task's working area is first entered, and grants no in-task create for
#  it; enter it with `jigc workflow sub-task --task via-start`, then list what task `via-start`
#  stages with `jigc doc list --task via-start`
$JIGC workflow sub-task --task via-start > <tmp>/route.out  # exit 0; cmp <tmp>/route.out <tmp>/s.out → rc 0
$JIGC doc set-field commit:via-start#type --value fix --task via-start      # exit 0 — set commit:via-start#type = fix
# control, the workflow-entered sibling, no route needed:
cd "$REPO/.jigc/worktrees/via-workflow"
$JIGC doc set-field commit:via-workflow#type --value fix --task via-workflow   # exit 0

# after — neither compared door committed or removed anything; `git log` in both rigs holds only
# setup's commit and the record commits `milestone create` / `add-task` announce.
```

### The contract

No contract is contradicted. The design names **both** doors for a sub-task and states the one
asymmetry between them:

- `design/workflow-dialect.md` → *Composing for a fan-out sub-task (M55)*: *"when the CLI
  composes for a sub-task — `jigc start --task <sub>` and `jigc workflow <W> --task <sub>`, the
  `Spawn:` line's door …— it **omits** those steps"*. One composition rule, two doors: what the
  empty diffs show.
- `design/storage.md` → *The per-task working area* (the pin rule, M47/N7): a milestone sub-task
  keeps the blanket refusal *"at **every** per-task read door — both `jigc workflow <W> --task
  <sub>` (sub-agent re-entry) *and* `jigc start --task <sub>` (resume …)"*. Cell 0.
- `design/write-commands.md` → *Sub-agent re-entry — `jigc workflow <W> --task <id>`* →
  *Re-entry provisions the write-ready area*: `milestone add-task` mints an area that is
  *"resume-*composable* but not write-*ready*"*, and *"the first `jigc workflow <W> --task <id>`
  re-entry **provisions**"* it. Cells A/B.
- The binary's own surfaces agree with each other: the sub-task `resume:` line names
  `jigc workflow sub-task --task <sub>` and says it provisions; the refusal in cell F routes
  there; the main-checkout refusal routes to *"re-run this command"*, whichever it was.
- Protocol §0.2 is narrower than the lead reads it: it says `jigc start --workflow sub-task`
  refuses and routes to `jigc workflow sub-task --task <id>` (reproduced, cell 0). It makes no
  statement that `jigc start --task <sub>` is not a door.

### Tier

**`none`.** Tier 1 fails on both halves: neither door commits, destroys or moves anything, and
nothing was lost. Tier 2 fails: the only refusal on this path (cell F, non-default knob only)
prints a route that runs verbatim at exit 0 and resolves it. Tier 3 fails: no surface says
something the binary does not do — the `resume:` line and the refusal both describe the
provisioning asymmetry accurately.

### Pin

`pinned-by: sub_task_composition::a_sub_task_composes_no_per_task_finalize_door_through_either_door`
(`crates/cli/tests/sub_task_composition.rs`, group `g_methodology`) — drives both doors from the
sub-task's worktree, agent text and json, exit 0, for five sub-task workflows, each held to one
view predicate (`assert_sub_task_view`). Around it:

- `start_resume::sub_task_read_doors_keep_the_blanket_base_pin_refusal` — both doors refuse from
  the main checkout and route at the worktree, in both provisioning states (cell 0).
- `workflow_reentry::first_reentry_provisions_the_write_ready_area` — the workflow door provisions.
- `anyhow_route_spans::absent_instance_refusal_never_routes_to_a_forbidden_create` — the cell F
  refusal's bytes and route, pinned on `single-task`/`quick-fix` cells (not on a `sub-task` cell).

**Bound on the pin:** no test compares the two doors' output **to each other** — each is held to
the same predicate, so byte-equality across doors is implied by one composer, not asserted. And
no test asserts that `jigc start --task <sub>` leaves `docs/` un-provisioned.

### Notes

- **Origin.** Arm a #41 (exit 1, main checkout) → #42 `milestone provision` → #43–#45
  `start --task <sub> --format json` (exit 0 ×3) → #48 `milestone finalize` (exit 0). The session
  never ran `jigc workflow sub-task --task`; it followed *"re-run this command there"*. All three
  sub-task areas were therefore start-only, under default squash — the posture in which the two
  doors are indistinguishable to a worker.
- **Design-doc wording, not a product surface.** `design/storage.md` calls `jigc start --task
  <sub>` *"the door the sub-task's own composed footer advertises"*; in rc.24 that footer's
  `resume:` line advertises `jigc workflow <W> --task <sub>` (orientation's per-task line is the
  one that prints `jigc start --task <sub>`). And `design/write-commands.md` describes
  `jigc start --task <id>` as resuming *a top-level task*. Two design sentences lag the two-door
  rule the other design sections state; the binary matches the rule.
- **Adjacent observation, not driven to a verdict here.** The provisioning side effect is
  visible downstream under the default squash: after the workflow door, `jigc task validate
  <sub>` (the `what's-left:` line) exits 3 with two blocking findings on `commit:<sub>` (empty
  `type`, empty `summary`) — a doc the default-squash text never asks for and the boundary never
  reads; after `start --task` alone there is no such doc to report on. Whether that is a declared
  bound was not traced; it is a separate lead if the scorer wants one.
- **Not driven:** `jigc milestone finalize` over the rigs (arm a #48 is the only evidence that a
  start-only fan-out lands); the `Spawn:` line run verbatim; any sub-task workflow other than
  `sub-task`.

---

## L-22 · A live foreign linked worktree is left byte-identical by provision / finalize / discard / orientation; a foreign worktree whose directory is absent (git `prunable`, unlocked) has its admin record silently pruned at exit 0 — **PARTIAL**

`tier:` none for the lead as filed (a live foreign worktree is untouched); 1 by the predicate for the confirmed variant (foreign worktree whose directory is absent at the door's instant: exit 0 + its git admin record deleted) — arguable down to 3, see the row's own Tier section

**Verdict: PARTIAL.** The lead as filed is **refuted**: a *live* foreign worktree is left byte-identical by every door driven. A narrower sibling the lead did not name is **confirmed**: a foreign worktree whose *directory is absent at the door's instant* has its git admin record deleted at exit 0, silently.

**Door:** `jigc milestone provision` · `jigc milestone finalize` · `jigc milestone discard` (orientation, `jigc start`, touches nothing in either shape).
**Binary:** the installed registry build, `jigc 1.0.0-rc.24`, release posture. git 2.54. `CLAUDECODE` set in every cell (held constant).

### The claim

> A linked worktree jigc did not cut can exist while a milestone is provisioned (the worker made one with raw git and removed it before the boundary), so what `milestone finalize`, `milestone provision` and orientation do over a foreign worktree is unread.

Origin, read and confirmed in `~/out/RC24-A-frozen-work/turn02` (main transcript, tool calls #62–#66): the session ran `git worktree add <tmp>/milestone-verify <base>` (detached), applied the three sub-tasks' staged diffs there, ran the suite, ran `git worktree remove <tmp>/milestone-verify --force`, and only then `jigc milestone finalize`. **No jigc command ran while that worktree existed**, so the session itself is no evidence either way — the lead is a question, and it is answered below on fresh rigs.

### What was driven

Every cell on its own `dev/jigc-rig fresh` rig: `milestone create` → two `add-task` → `provision` → one staged code file per sub-task worktree. The foreign worktree is this repository's own linked worktree, made with raw `git worktree add`, carrying an untracked file, a staged-only file and an unstaged edit to a tracked file. "Intact" = same `shasum` of all three, same `git status --porcelain`, same HEAD, still listed by `git worktree list`, and none of its files in any commit.

| # | Foreign worktree | Door(s) run, in order | Exit | Foreign worktree after |
|---|---|---|---|---|
| 1 | none (control) | `start` · `provision` · `milestone finalize` | 0 · 0 · 0 | — (boundary lands 2 code files + the record) |
| 2 | live, detached, at `<tmp>/x`, made **after** provision | `start` · `provision` · `milestone finalize` | 0 · 0 · 0 | intact; only the two `.jigc/worktrees/<sub>` are torn down |
| 3 | live, detached, at `<tmp>/x`, made **before** provision | `provision` · `start` · `provision` · `milestone finalize` | 0 · 0 · 0 · 0 | intact |
| 4 | live, detached, at `$REPO/.jigc/worktrees/mine` (inside jigc's directory, not a sub-task id) | `start` · `provision` · `milestone finalize` | 0 · 0 · 0 | intact, still registered |
| 5 | live, detached, at `<tmp>/x` | `milestone discard` · `milestone discard --force` | 1 (`milestone.dirty-worktree`, naming only the two sub-task paths) · 0 | intact both times |
| 6 | live, on branch `side`, **cwd inside it** | `start` · `milestone finalize` | 0 · 0 | intact (staged file still staged); the boundary commit lands on `main`, `side` does not move |
| 7 | live, detached, **cwd inside it** | `start` · `provision` · `milestone finalize` | 0 · 0 · **1** | intact; `finalize` refuses `repo.head-detached`, commits nothing |
| 8 | **directory absent** (moved away), not locked — no jigc run (control) | — | — | record present; directory put back → `git status` exit 0, `git worktree repair` exit 0 |
| 9 | **directory absent**, not locked | `start` · `milestone provision` | 0 · 0 | `start`: record present. `provision`: **`.git/worktrees/x` gone** |
| 10 | **directory absent**, not locked | `milestone finalize` | 0 | **`.git/worktrees/x` gone** |
| 11 | **directory absent**, not locked | `milestone discard` | 0 | **`.git/worktrees/x` gone** |
| 12 | **directory absent**, `git worktree lock`ed (control) | `milestone provision` | 0 | record present; directory put back → works |

Cells 2–7 refute the lead as filed. Cells 9–11 against controls 8 and 12 are the confirmed variant.

### Repro block

```
# setup (bash, from the jigc checkout)
jigc --version                                   # jigc 1.0.0-rc.24
rig=$(dev/jigc-rig fresh --binary "$(command -v jigc)") || exit; eval "$rig"; [ -n "$REPO" ] || exit
T=$(mktemp -d "${TMPDIR:-/tmp}/l22.XXXXXX") || exit; X="$T/x"; M=foreign-worktree-probe
jigc milestone create "Foreign worktree probe" >/dev/null
jigc milestone add-task $M "Add alpha file" >/dev/null; jigc milestone add-task $M "Add beta file" >/dev/null
jigc milestone provision $M
for s in add-alpha-file add-beta-file; do echo code > "$REPO/.jigc/worktrees/$s/$s.txt"; git -C "$REPO/.jigc/worktrees/$s" add "$s.txt"; done

# A — the lead's own repro: a LIVE foreign worktree, dirty in all three ways
git -C "$REPO" worktree add -q --detach "$X" HEAD
echo untracked > "$X/foreign-untracked.txt"; echo staged > "$X/foreign-staged.txt"; git -C "$X" add foreign-staged.txt; echo edit >> "$X/README.md"
before=$(cd "$X" && shasum foreign-untracked.txt foreign-staged.txt README.md; git status --porcelain; git rev-parse HEAD)

# B — the variant: a second foreign worktree whose directory is ABSENT at the door's instant
Y="$T/y"; git -C "$REPO" worktree add -q --detach "$Y" HEAD
echo c > "$Y/foreign-committed.txt"; git -C "$Y" add .
git -C "$Y" -c user.name=probe -c user.email=probe@example.invalid commit -qm "foreign detached commit"
FC=$(git -C "$Y" rev-parse HEAD); echo s > "$Y/foreign-staged.txt"; git -C "$Y" add foreign-staged.txt
mv "$Y" "$T/y.away"            # moved by hand · volume unmounted · path not mounted where jigc runs
# before-control
ls "$REPO/.git/worktrees/y"                                    # COMMIT_EDITMSG commondir gitdir HEAD index logs ORIG_HEAD refs
git -C "$REPO" fsck --unreachable | command grep -c "$FC"      # 0   (the commit is reachable — from that worktree's HEAD)

# argv
jigc start                         # exit 0 — orientation names the two sub-tasks; says nothing of either foreign worktree; prunes nothing
jigc milestone provision $M        # exit 0 (idempotent re-run) — run here, this alone already drops B's record (cell 9); omit it and
jigc milestone finalize $M         #   finalize drops it instead (cell 10, and the run this block was condensed from)

# observed
finalized <sha> — Finalize milestone foreign-worktree-probe (2 sub-tasks)
  added add-alpha-file.txt
  added add-beta-file.txt
  modified docs/milestone-records/foreign-worktree-probe.md
  3 files committed
  sub-tasks: add-alpha-file: 1 code file · add-beta-file: 1 code file
[exit 0]                           # stdout and stderr carry no word about any worktree registration

# after A — REFUTED half
after=$(cd "$X" && shasum foreign-untracked.txt foreign-staged.txt README.md; git status --porcelain; git rev-parse HEAD)
[ "$before" = "$after" ]           # true: byte-identical, ` M README.md` / `A  foreign-staged.txt` / `?? foreign-untracked.txt`, same HEAD
git -C "$REPO" worktree list       # $REPO [main]  ·  <tmp>/x (detached HEAD)      — the two .jigc/worktrees/<sub> are gone, x is not
git -C "$REPO" show --stat HEAD    # add-alpha-file.txt, add-beta-file.txt, the milestone record — no foreign-* path in any commit

# after B — CONFIRMED half
ls "$REPO/.git/worktrees/y"                                    # No such file or directory   (index, HEAD, logs/HEAD, ORIG_HEAD gone)
git -C "$REPO" fsck --unreachable | command grep -c "$FC"      # 1   (the commit only that worktree's HEAD reached now dangles until gc)
mv "$T/y.away" "$Y"; ls "$Y"                                   # CLAUDE.md docs foreign-committed.txt foreign-staged.txt README.md  (working files survive)
git -C "$Y" status --porcelain                                 # fatal: not a git repository: (null)   [exit 128]
git -C "$REPO" worktree repair "$Y"                            # error: unable to locate repository; .git file does not reference a repository   [exit 1]
```

Controls for B, each on its own rig: with **no jigc command** between `mv` away and `mv` back, the record survives and `git status` / `git worktree repair` both exit 0 (cell 8); with `git worktree lock "$Y"` before the `mv`, `jigc milestone provision` exits 0 and the record survives (cell 12); `jigc start` over the absent directory leaves the record (cell 9, first step). So the deletion is attributable to the three milestone doors and to nothing else in the sequence.

Mechanism, read in source: `crates/cli/src/milestone.rs` runs a bare, repository-wide `git worktree prune` in `provision_worktrees` (comment: *"Drop admin records for any worktree dir deleted out from under git by a crashed run"*) and again at the end of `remove_worktrees` (the teardown `finalize` and `discard` share). Everything else in both functions is keyed to `canonical_home/.jigc/worktrees/<sub-id>` matched against the registered set, which is why a live foreign worktree — and even one at `.jigc/worktrees/mine` — is never touched. The prune is the one operation that is not keyed, and it carries no `--expire` (git's own `gc` prunes the same records only after `gc.worktreePruneExpire`, three months by default).

### The contract

**For the refuted half — none is contradicted; the binary does what the docs say.**
- `design/finalize.md` → *1. Preflight*: *"The predicate's subject is every checkout the door commits **from**"* — the provisioned sub-task worktrees. A foreign worktree is not a checkout the boundary commits from, and nothing is read out of it (cells 2–4, 6).
- `design/storage.md` → *Repository layout & state location*: the worktree set is `worktrees/<sub-task-id>/`, *"torn down on a LANDED boundary, or by `milestone discard`"*. Teardown removed exactly those paths.
- `jigc milestone provision --help`: *"reuses a live worktree untouched"* — true of the foreign one as well.
- Cell 7's refusal (`repo.head-detached`, exit 1, from a cwd inside a detached foreign worktree) is the posture family working as written (`design/finalize.md` exempts only the worktrees *"jigc detached … itself"*); it is fail-closed, commits nothing, and running from the main checkout lands (cell 2). Its message says *"a commit made here"* although the boundary commit lands on the main checkout's HEAD (cell 6 shows that) — an inexact sentence, noted, not driven further.

**For the confirmed variant — two statements are contradicted.**
- `DECISIONS.md` → *2026-09-23 — the confirmation pass: seven findings fixed*, the `uninstall` paragraph: *"It prunes now, best-effort as they are, and **says so**: a destroying door narrating what it changed in the repository is law 1."* `jigc uninstall` prints `pruned git's worktree registrations` when a prune dropped a record (pinned by `g_milestone::cwd_verb_subject::uninstall_removes_the_workbench_home_install_from_every_cwd`). Its three siblings — `provision`, `milestone finalize`, `milestone discard`, all members of `cli::milestone::DESTROYING_DOORS` — run the same prune and print nothing, at exit 0, when what it dropped was not jigc's.
- `jigc milestone provision --help` and `design/storage.md` → *Repository layout* (*"No door removes such a path silently: each probes it first and then either refuses what it cannot prove is disposable … or names every byte it is about to destroy"*) describe a door whose reach is `.jigc/worktrees/<sub-task-id>`. The prune reaches every registration in the repository, probes nothing, and names nothing.

The harm class is one the project has already ruled on for its *own* worktrees: `crates/cli/tests/leftover_operation_in_progress.rs` (module doc) records `.git/worktrees/<sub>/` going with a discard — *"that checkout's reflog went too and a commit reachable only from its detached HEAD dangles until `git gc`"* — as the HIGH it was written to close. The variant is the same loss on a record jigc never wrote.

### Tier

- **The lead as filed (live foreign worktree; the scorer's repro): `none`.** No door removes it, commits from it, or mentions it. Not a defect.
- **The variant (directory absent, unlocked): tier 1 by the predicate — both halves shown.** Exit 0 through `milestone finalize` (a committing door) and through `provision` / `discard` (destroying doors); before-control finds `.git/worktrees/<name>/{index,HEAD,logs/HEAD}` and a reachable commit, after finds them absent and the commit unreachable, the returned checkout no longer a git repository and `git worktree repair` unable to mend it.
- **What argues the variant down, stated so the scorer can weigh it:** the precondition is not jigc's doing and did **not** occur in the trial (the session removed its worktree with `git worktree remove` before the boundary); git itself calls this state `prunable`, a bare `git worktree prune` typed by anyone does the same, and `git worktree lock` is git's documented protection (cell 12 honours it); the working files survive; the dangling commit is recoverable with `git fsck --lost-found` until gc. What is unrecoverable is the per-worktree index (the staged/unstaged distinction), its reflog, and the link. A reader who holds that an absent directory is abandoned would tier this **3** — a silent repository mutation where the sibling door narrates. The plausible everyday trigger is not exotic, though it was **not driven**: a repository bind-mounted into a container (how this trial's sessions ran), whose host-side linked worktrees live at paths the container cannot see, is `prunable` in every one of them from where jigc runs.

### Pin

`UNPINNED` — both halves.
- Refuted half: no suite plants a registered worktree of this repository at a path outside `.jigc/worktrees/<sub-task-id>` and asserts it survives a milestone door. The nearest are `g_milestone::cwd_verb_subject::milestone_finalize_lands_from_every_cwd_in_the_repository` (cwd axis: root · subdir · own sub-task worktree · sibling sub-task worktree — no foreign worktree) and `g_milestone::leftover_operation_in_progress::every_refusing_worktree_door_refuses_a_live_operation_and_takes_it_only_on_consent` (a *second repository's* worktree parked **at** a sub-task path — a different planting).
- Variant: no suite builds a `prunable` foreign registration; the only prune assertion is `uninstall`'s narration in `cwd_verb_subject`, over jigc's own fan-out records.

### Not covered

`jigc uninstall` and the squash combine's dedicated worktree (`crates/cli/src/task.rs`, `DedicatedWorktree::add`, a third bare prune) were read, not driven. `--format json` was not driven for the variant. The container-mount trigger is an inference from git's `prunable` rule, not a run.

---

## L-23 · Co-author trailer placement under a `Key: value`-final body: both observed placements reproduce and the unread `Word: text` edge works; the claim's causal clause is refuted (git reads the body line as a trailer with no jigc line at all) — **PARTIAL**

`tier:` none

**Kind:** product · **Door:** `jigc task finalize` · **Binary:** installed registry build, `jigc --version` → `jigc 1.0.0-rc.24` (asserted first) · **git:** 2.54.0 (Apple Git-157)

### The claim

When a commit doc's body ends in a `Key: value` paragraph, jigc appends its `Co-Authored-By` line directly under it with no blank line, *so* git's parser reads the body line as a trailer too; with a prose-final body it inserts a blank line; what it does for a prose paragraph whose last line merely looks like `Word: text` is unread.

Origin (read, not the proof): `~/out/RC24-B-frozen-work/turn01` `7757a4c` and `turn02` `4b80510` (`Refs: TKT-…` directly above the co-author line, both in `%(trailers)`), `~/out/RC24-A-frozen-work/turn02` `a9f1851` (prose-final, blank line, one trailer).

### Verdict: PARTIAL — tier none

Three parts, three results:

| part of the claim | result | datum |
|---|---|---|
| `Key: value`-final body → co-author line joined with no blank line | **reproduces** | cell `a-on` |
| "…*so* git's parser reads the body line as a trailer" | **refuted** — git reads it as a trailer with no co-author line at all | cell `a-off`: variable unset, no jigc line, `%(trailers)` prints `Refs: TKT-1` |
| prose-final body → one blank line, then the co-author line | **reproduces** | cell `b-on` |
| the unread edge: a prose paragraph whose **last** line is `Note: something` | **read — works**: co-author in a block of its own after one blank line; git reads one trailer; `Note: something` stays prose with the variable set and unset | cells `c-on`, `c-off` |

The join is not what makes git read the body line as a trailer. A last paragraph made only of `token: value` lines **is** git's trailer block, whoever wrote it and whichever slot it came from. jigc's join keeps git's reading of that paragraph exactly what it was before signing; a blank line there would have *removed* `Refs: TKT-1` from `%(trailers)`. In every cell the lead names, jigc's bytes equal what `git interpret-trailers --trailer` produces over the unsigned message.

No contract is contradicted inside the lead's scope, so this is not a defect.

### Repro

Fresh rig, installed binary, release posture. The agent session variable the trailer keys on was **set** in every cell marked `on` and removed with `env -u` for the whole cell in every cell marked `off`; its value was never printed.

```
# setup
jigc --version                                    -> jigc 1.0.0-rc.24
rig=$(dev/jigc-rig fresh --binary <installed jigc>) || exit; eval "$rig"; [ -n "$REPO" ] || exit
#   $REPO = <tmp>/jigc-rig-fresh-XXXXXX/repo ; HEAD = "chore(jigc): install jigc workspace config"

# argv, per cell <c> with body file <B>  (every line: exit 0)
jigc start --workflow single-task "cell <c>" --format json            # task = cell-<c>
printf 'change for <c>\n' > code-<c>.txt; git add code-<c>.txt
jigc doc set-field commit:cell-<c>#type    --task cell-<c> --value fix
jigc doc set-slot  commit:cell-<c>#summary --task cell-<c> --from-file - < summary.txt   # "probe cell <c>"
jigc doc set-slot  commit:cell-<c>#body    --task cell-<c> --from-file - < <B>
jigc task finalize cell-<c>                                           # exit 0, commits N -> N+1

# after
git log -1 --format=%B | cat -e
git log -1 --format='%(trailers)'
git log -1 --format='%(trailers:only,unfold)'
```

Observed, `$` marking each line end of `%B`:

```
cell a-on   body = "…answer 400.\n\nRefs: TKT-1\n"                       finalize exit 0
  fix: probe cell a-on$
  $
  The empty series name answered 200. Require a name and answer 400.$
  $
  Refs: TKT-1$
  Co-Authored-By: Claude <noreply@anthropic.com>$
  %(trailers)      -> Refs: TKT-1 / Co-Authored-By: Claude <noreply@anthropic.com>

cell a-off  same body, variable unset                                    finalize exit 0
  …$
  Refs: TKT-1$
  %(trailers)      -> Refs: TKT-1                    <- the falsifying datum for the "so"

cell b-on   body = "…answer 400.\n"                                      finalize exit 0
  The empty series name answered 200. Require a name and answer 400.$
  $
  Co-Authored-By: Claude <noreply@anthropic.com>$
  %(trailers)      -> Co-Authored-By: Claude <noreply@anthropic.com>

cell c-on   body last paragraph = prose line, then "Note: something"     finalize exit 0
  Require a name and answer 400 instead of a line naming no series.$
  Note: something$
  $
  Co-Authored-By: Claude <noreply@anthropic.com>$
  %(trailers)      -> Co-Authored-By: Claude <noreply@anthropic.com>     (Note: is not read as a trailer)

cell c-off  same body, variable unset                                    finalize exit 0
  %(trailers)      -> (empty)                        (Note: is not a trailer here either)
```

Further cells on the same rig, all `finalize` exit 0:

```
cell d-on   last paragraph = "Note: something", then a prose line   -> blank line, own block; %(trailers) = co-author only
cell e-on   last paragraph = two prose sentences shaped "Before: …" / "After: …"
            -> joined, no blank line; %(trailers) = Before / After / co-author
cell e-off  same, variable unset -> %(trailers) = Before / After          (git's reading, not jigc's doing)
cell g-on   last paragraph = "Refs: TKT-1" + an indented continuation line
            -> joined; %(trailers:only,unfold) = "Refs: TKT-1 and TKT-2, folded" / co-author
cell h-on   body is the one line "Note: something" -> joined; %(trailers) = Note / co-author
```

Control against git's own placement, no ambient `trailer.*` config in the rig (`git config --get-regexp '^trailer\.'` → exit 1):

```
git interpret-trailers --trailer 'Co-Authored-By: Claude <noreply@anthropic.com>' <unsigned %B of a-off>
  -> "Refs: TKT-1$" / "Co-Authored-By: …$"          no blank line  = cell a-on's bytes
git interpret-trailers --trailer '…' <unsigned %B of c-off>
  -> "Note: something$" / "$" / "Co-Authored-By: …$"  one blank line = cell c-on's bytes
```

### The contract

Nothing is contradicted; the behaviour is the stated rule.

- `design/assistant-adapter.md` → *The co-author trailer* → **Placement and de-duplication**: the trailer is "appended to the message's trailer block when its last paragraph is one, else as a new block after one blank line — the place `git interpret-trailers` gives a trailer added at the end". Cells a, b, c, d, e, g, h all match, and a and c match `git interpret-trailers` byte for byte.
- `design/finalize.md` → *The amend arm* declares the body/trailer ambiguity as a property of the render: "a body paragraph shaped `Refs: <value>` is indistinguishable from a trailer item on the way back". The composed `amend-message` step prints the same to the agent ("a body paragraph shaped like `Key: value` is indistinguishable from a trailer"). The session saw the ambiguity named, not hidden.
- `design/finalize.md` → *Commit-doc rendering* summarises the placement as "appended to the block above when the doc rendered one". In cell a the doc rendered no `#trailers` block; the block is a body paragraph. The sentence is looser than the owning rule it points at (assistant-adapter.md speaks of the *message's* last paragraph, which is what the binary reads). Wording, not behaviour — noted, not tiered.

### Tier: none

No exit-0 loss or harm (every authored byte is in `%B`; git's trailer reading of the authored paragraph is identical with the variable set and unset in cells a, c, e), no dead end, and no surface that says something the binary does not do within the lead's scope.

### Pin

- **pinned-by (the two halves the lead observed, at the unit seam):** `jigc` lib unit test `adapter::tests::sign_places_the_trailer_in_the_trailer_block` (`crates/cli/src/adapter.rs`) — prose-final → `\n\n` + trailer; `Refs:`/`Signed-off-by:` final block → joined with `\n`. `adapter::tests::sign_never_doubles_a_co_author_the_block_already_names` covers a trailer-shaped **first** line followed by prose (cell d's shape).
- **UNPINNED (the edge the lead asked about):** no test at unit or door level signs a paragraph whose **last** line is `Word: text` after a prose line (cell c), and no test drives a body slot ending in a `Key: value` paragraph through `task finalize` and reads `%(trailers)` — `agent_co_author::a_commit_doc_that_already_names_the_agent_is_not_doubled` and `commit_trailer_roundtrip::*` author trailers as `#trailers` items only.

Tests were read, not run. The tree read is the release tree: `git diff --stat 91834b5e HEAD -- crates` is empty (`91834b5e` = the rc.24 release-prep commit).

### Adjacent observation — outside this lead's claim, a candidate lead of its own

One sibling edge does diverge from the stated rule. It is not the `Word: text` edge and is not counted in the verdict above.

```
cell f-off  body last paragraph = "Reviewed while pairing on the router." / "Signed-off-by: A Dev <a.dev@example.com>", variable unset
  finalize exit 0
  %(trailers:only,unfold) -> Signed-off-by: A Dev <a.dev@example.com>

cell f-on   same body, variable set
  finalize exit 0
  Reviewed while pairing on the router.$
  Signed-off-by: A Dev <a.dev@example.com>$
  $
  Co-Authored-By: Claude <noreply@anthropic.com>$
  %(trailers:only,unfold) -> Co-Authored-By: Claude <noreply@anthropic.com>       (Signed-off-by no longer read)

git interpret-trailers --trailer 'Co-Authored-By: …' <unsigned %B of f-off>
  -> "Signed-off-by: …$" / "Co-Authored-By: …$"        no blank line: git puts it INSIDE the mixed block
```

git treats a mixed last paragraph as the trailer block when it holds a git-generated prefix (`Signed-off-by: `) and is at least a quarter trailers. jigc's `trailer_block` (`crates/cli/src/adapter.rs`) accepts only the all-trailer shape, so it opens a new block, and git then reads the new block alone: signing demotes the body's `Signed-off-by` from a trailer git reads to prose. That is not "the place `git interpret-trailers` gives a trailer added at the end" (assistant-adapter.md → Placement), and the source comment's "which git reads as the trailer block either way" is true of the new block and silent about the old one. No bytes are lost — the line is in `%B` before and after — so this is tier 3 at most (a stated rule the binary does not follow in one shape), reachable only by writing a sign-off into the prose `body` slot beside a prose line rather than as a `#trailers` item. UNPINNED: no test signs a mixed paragraph carrying `Signed-off-by:`.

---

## L-24 · `task validate` / `task finalize` report one advisory `file-state.staged-copy` (route "no action needed") per PERSISTED staged doc — by design, not per every staged doc, and not a defect — **PARTIAL**

`tier:` none

**Door:** `task validate`; `task finalize` · **Kind:** product · **Binary:** `jigc 1.0.0-rc.24` (the installed registry build, release posture)

### The claim

Every `task validate` and `task finalize` over a staged doc reports an advisory
`file-state.staged-copy` finding whose route is *"no action needed"*. Origin: arm c #13, #15;
arm a #29, #31. The scorer's proposed tier: none — advisory by design; noise at most.

### Verdict

**PARTIAL** — the behaviour reproduces, it is narrower than "every staged doc", and it is **not
a defect** (tier `none`).

- **Reproduces:** with a *persisted* doc staged (a new `adr`, or a copy-on-write edit of a
  committed `adr`), `task validate`, `task finalize --dry-run` and the landing `task finalize`
  each carry exactly one `file-state.staged-copy` advisory per such doc, keyed at the doc's
  repo-real destination, route `no action needed — the staged copy is validated in-task and
  baselined when its finalize lands`. It never changes the exit code (3 beside blocking
  findings, 0 without), and the finalize lands.
- **Narrower than claimed — the falsifying datum for "every":** a task whose only staged doc is
  the *transient* `commit` doc reports **no** `file-state.*` finding (cell 0 below: three
  findings, none of them `file-state.staged-copy`, with `commit:<task>.md` staged). The advisory
  is per persisted staged instance, not per staged doc.
- **No contract contradicted.** The emission, its severity, its target and its informational
  route are each stated in a design doc and pinned by tests (below). What the route promises —
  "baselined when its finalize lands" — holds: after each landing, store-scope `jigc validate`
  reports zero findings (no drift, no `baseline-adopt`).

Origin check (read-only): `~/out/RC24-C/.jigc/logs/invocations.jsonl` lines 13 and 15, and
`~/out/RC24-A-frozen-work/turn01/.jigc/logs/invocations.jsonl` lines 29 and 31, carry
`finding_codes` of `file-state.staged-copy` (one in arm c, four in arm a — one per persisted
staged doc) at `exit_code` 0. The four-doc multiplicity is read from the session log, not
re-driven here; the cells below stage one persisted doc per task.

### Repro block

```
setup
  rig=$(dev/jigc-rig fresh --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit
  $JIGC --version                                   -> jigc 1.0.0-rc.24
  # $REPO = <tmp>/repo, two commits (initial; chore(jigc): install jigc workspace config)
  # CLAUDECODE: set, held constant across every cell
  $JIGC start "record a decision about retries" --workflow single-task --format json
                                                    -> exit 0, task = record-a-decision-about-retries
  T=record-a-decision-about-retries

0. control — only the transient commit doc is staged
  staged   .jigc/tasks/$T/docs/commit:$T.md
  argv     $JIGC task validate $T --format json
  exit     3
  findings blocking schema-conformance.field-value-conformant  commit:$T#header/type
           blocking schema-conformance.required-slot-present   commit:$T#summary
           advisory changelog-recording.gate-granted-unused    task:$T
           -> NO file-state.staged-copy

1. the scorer's repro — a persisted doc staged
  argv     $JIGC doc create adr --title x --task $T          -> exit 0, prints adr:x
  argv     $JIGC task validate $T --format json
  exit     3   (the empty required slots block; the advisory does not)
  findings[0]
    "severity": "advisory", "probe": "file-state", "check": "staged-copy",
    "code": "file-state.staged-copy",
    "key": {"code": "file-state.staged-copy", "target": "docs/decisions/x.md"},
    "message": "staged copy of `docs/decisions/x.md` — this task's in-flight version of the doc",
    "route": "no action needed — the staged copy is validated in-task and baselined when its finalize lands"
  text form (same argv without --format json), first three lines:
    advisory · file-state.staged-copy — staged copy of `docs/decisions/x.md` — this task's in-flight version of the doc
      at: docs/decisions/x.md
      route: no action needed — the staged copy is validated in-task and baselined when its finalize lands

2. every required slot filled (three adr slots, commit type + summary; each write exit 0)
  argv     $JIGC task validate $T --format json
  exit     0
  findings advisory file-state.staged-copy                   docs/decisions/x.md
           advisory changelog-recording.gate-granted-unused  task:$T

3. the forecast
  argv     $JIGC task finalize $T --dry-run --format json
  exit     0
  keys     dry_run, findings, left_out, manifest, subject
  findings the same two codes as cell 2

4. the committing door
  before   HEAD = install commit; docs/decisions/ does not exist; tree clean
  argv     $JIGC task finalize $T --format json
  exit     0
  keys     committed, findings, schema_version
  findings advisory file-state.staged-copy                   docs/decisions/x.md   (same route)
           advisory changelog-recording.gate-granted-unused  task:$T
  committed subject "docs: record the retry decision", files 1,
            manifest [{kind: promoted, path: docs/decisions/x.md}], left_out [], displaced []
  after    git log: one new commit, `docs/decisions/x.md | 23 +`; git status --short empty;
           docs/decisions/x.md present

5. the route's promise, checked
  argv     $JIGC validate --format json
  exit     0, findings = []      (the landed doc is baselined; no drift, no baseline-adopt)

6. a second task, copy-on-write over the now-committed doc
  $JIGC start "amend the retry decision" --workflow single-task --format json  -> exit 0
  T2=amend-the-retry-decision
  $JIGC doc set-slot adr:x#consequences --task $T2 --from-file -               -> exit 0
  (commit type + summary filled, each exit 0)
  argv     $JIGC task validate $T2 --format json
  exit     0
  findings advisory file-state.staged-copy                   docs/decisions/x.md
           advisory changelog-recording.gate-granted-unused  task:$T2

7. the committing door, text form
  argv     $JIGC task finalize $T2
  exit     0
  lines    advisory · file-state.staged-copy — staged copy of `docs/decisions/x.md` — this task's in-flight version of the doc
             at: docs/decisions/x.md
             route: no action needed — the staged copy is validated in-task and baselined when its finalize lands
           advisory · changelog-recording.gate-granted-unused — …
           finalized <sha> — docs: amend the retry decision
             promoted docs/decisions/x.md
             1 file committed
  after    git log: one new commit, `docs/decisions/x.md | 2 +-`; the amended line is in the
           committed file (count 1); git status --short empty; store `jigc validate` findings = []
```

No bytes were lost and no repository state was harmed at any cell: both finalizes committed
exactly the staged doc, the tree was clean after each, and the store validated clean.

### The contract

None is contradicted; the behaviour is the stated one.

- `design/validation.md` → the `file-state` probe bullet (*Committed docs only (M43 A14)*): a
  persisted staged instance "is reported as the informational `file-state.staged-copy` advisory
  at its repo-real committed destination", and "a transient-sink instance (no committed home)
  yields no `file-state.*` finding at all" — cells 1 and 0 respectively.
- `design/validation.md` → check inventory, *Informational outcomes — emitted advisory,
  intentionally not tunable*: `file-state.staged-copy` is listed among the routing notes that are
  "not pass/fail checks", carry no knob and stay advisory.
- `design/command-output-contract.md` → `route`: every finding routes, and an **informational
  route** ("no action needed — …") "marks a genuine no-op" — the kind this finding carries
  (`engine::file_state::staged_copy_finding` builds it with `Route::informational`).
- `DECISIONS.md` → *2026-07-16 — M43 Inc 5 T4: A14 honest staged keys*, pin (1): the code is
  deliberately distinct from `baseline-adopt` so two findings cannot collide on one
  `(code, target)` key.
- That the committing door repeats what `task validate` previewed is itself a designed property
  (the EC-20 equal-set rule: `validate` == `--dry-run` == the committing door's emission), not an
  echo that escaped.

One wording observation, not a contradiction: on the *landed* finalize the advisory still reads
"this task's in-flight version … baselined when its finalize lands" in the same envelope that
reports the commit. It is the previewed sentence carried verbatim by the equal-set rule; cell 5
shows the statement is true by the time it is read.

### Tier

**none.** Not tier 1: exit 0 with the commit landing and nothing lost (cells 4, 7 — before/after
shown). Not tier 2: the finding blocks nothing and its route is a stated no-op, so there is no
dead end. Not tier 3: every surface that speaks about the finding says what the binary does. What
remains is the scorer's "noise at most": one advisory line per persisted staged doc at each of
the three doors, which is a judgment about volume and no predicate of the tiering.

### Pin

`pinned-by: dry_run_findings_equal_set::the_three_doors_emit_one_code_set_for_every_previewed_member`
(`crates/cli/tests/dry_run_findings_equal_set.rs`, group `g_finalize`) — its `content-findings`
row builds a `single-task` corpus with a persisted `adr` staged, asserts `file-state.staged-copy`
fires at `task validate`, and asserts `task finalize --dry-run` and the committing `task finalize`
emit the identical code multiset through the real binary.

Supporting pins: `engine::file_state` unit test
`staged_copy_finding_is_an_informational_advisory_at_the_destination` (code, severity, target,
informational route); `engine::validate` unit test
`staged_persisted_adr_reports_at_its_repo_real_destination` (exactly one `file-state` finding per
persisted staged instance, at the repo-real path); `cli::render` snapshot
`render_finalize_manifest_flags_untracked_and_json_carries_dry_run` (the three text lines,
including the route sentence, byte for byte); `orientation_active_task::the_active_block_renders_byte_for_byte`
(the same three lines on the orientation surface).

The narrowing datum is pinned too, at the engine level: `engine::validate` unit test
`staged_transient_instance_yields_no_file_state_findings`. I did not look for a
through-the-binary test of that absence; cell 0 is the driven evidence. None of these tests was
run in this verification — they were read, not executed.

---

## T-1 · `run.py observe` reports a correct `jigc task amend` as `HISTORY REWRITTEN … a commit produced or rewritten outside jigc`: the reflog half keys on the subject prefix `commit (amend)` alone and never consults the `task amend` record it already holds. Declared in protocol §8.3 and §0.3; tooling unpatched; the line is advisory (exit 0, outcome column unchanged) — **CONFIRMED**

`tier:` n/a

**Lead (tooling):** `run.py observe` flags a correct `jigc task amend` as `HISTORY REWRITTEN … produced or rewritten outside jigc`, because it keys on any reflog line beginning `commit (amend)`; declared in §8.3, not patched.
**Source:** operator note; `rubric/observe-b.txt`. **Door:** `completions/trial-driver/driver/observe.py` (`history_surgery`), printed by `completions/trial-driver/run.py` (`do_observe`).
**Binary for the rig cells:** `~/.local/bin/jigc`, `jigc --version` → `jigc 1.0.0-rc.24` (asserted first). The reader under test is the working tree's `completions/trial-driver/`, unmodified (`git status --short completions/trial-driver` empty; `observe.py` last touched by `ba826655`).

### Verdict

**CONFIRMED** — every clause of the claim carries, on the trial's own evidence and on an independent rig.

1. **The reader draws the line.** `run.py observe ~/out/RC24-B-frozen-work/turn02` re-run read-only today prints the same `HISTORY REWRITTEN` line and caption the operator saved in `rubric/observe-b.txt`.
2. **The amend it flags was a correct one.** In that corpus the invocation log holds `task amend --format json` (exit 0, 22:23:20Z) and `task finalize amend-7757a4c --format json` (exit 0, 22:23:38Z); the reflog's `commit (amend)` entry is stamped 22:23:38Z, the same second as the finalize record; `HEAD`'s tree equals the superseded commit's (`766eb5cc…` both), same parent `df2fc63`; `HEAD`'s message carries `Refs: TKT-221`; and the transcript holds **zero** raw git history acts (the saved `raw-git-b.txt` says 0; an independent scan of every Bash command naming `git` finds four, all `add`/`status`/`log`).
3. **The key is the reflog subject prefix and nothing else.** `observe.py:566-567` keeps any reflog line whose subject starts with `reset:`, `rebase`, `revert:` or `commit (amend)`. `history_surgery(corpus)` takes the corpus path only (`observe.py:649` calls it with the path derived from the log's location); the invocation records that `observe()` has in hand at that point are not consulted, so a `task amend` record cannot clear the line. `run.py:221-226` then prints the fixed caption *"a commit produced or rewritten outside jigc"* for any such line.
4. **Declared.** `protocol/protocol.md` §8.3, first bullet (*"A correct arm (b) draws `HISTORY REWRITTEN` … The tooling is not patched for it"*) and §0.3 (*"A correct amend leaves `commit (amend)` in the reflog, and `run.py observe` reports that line as `HISTORY REWRITTEN`"*).
5. **Not patched.** The tooling tree is clean and the filter is as quoted.

**Bound on the consequence (not a correction of the claim):** the line is advisory. `observe` exits 0, the row's outcome column is unchanged (`read back through the fence's verb`), no `history_surgery` reference exists in `driver/cascade.py` or `driver/gate.py`, and the caption itself ends *"evidence for review, not a verdict"*. What is wrong is the caption's attribution — *outside jigc* — for a rewrite jigc made. The reader therefore cannot tell B-1 from B-3 on this line alone: the rig's control cell shows a raw `git commit --amend` drawing a line of the identical shape.

### Repro

```text
# ── cell 1 — the trial's own evidence, read-only ──────────────────────────────
# setup: none; the out-dir is read, never written
$ python3 completions/trial-driver/run.py observe ~/out/RC24-B-frozen-work/turn02     # exit 0
  session           recs wrote  VERB  adj  fs  outcome
    provenance: ungated: jigc-gate:registry-1.0.0-rc.24 / unknown (pass --gate <record.json> to check it)
  turn02               9     4     1    0   0       read back through the fence's verb
    note: 17 record(s) predate this session (plant/adoption) and are NOT scored — …
    HISTORY REWRITTEN — 4b80510 HEAD@{0} commit (amend): fix(summary): reject GET /summary/ with no series name
    ^ a commit produced or rewritten outside jigc. The CLI owns the commit boundary; git is a
      blessed HUMAN channel, so this is evidence for review, not a verdict — check whether the
      commit carried a managed doc.

# what produced that reflog line — same out-dir, read-only
$ git -C ~/out/RC24-B-frozen-work/turn02 reflog --date=iso-strict --format='%h %gd %gs' -2   (UTC)
  4b80510 HEAD@{2026-10-03T22:23:38Z} commit (amend): fix(summary): reject GET /summary/ with no series name
  7757a4c HEAD@{2026-10-03T22:23:05Z} commit: fix(summary): reject GET /summary/ with no series name
$ .jigc/logs/invocations.jsonl, the `task …` rows
  2026-10-03T22:23:05Z  0  task finalize fix-get-summary-with-empty --format json
  2026-10-03T22:23:20Z  0  task amend --format json
  2026-10-03T22:23:38Z  0  task finalize amend-7757a4c --format json
$ git rev-parse 'HEAD^{tree}' 'HEAD@{1}^{tree}'      # 766eb5cc… twice — tree untouched
$ git log -1 --format=%b                              # … Refs: TKT-221 / Co-Authored-By: Claude <noreply@anthropic.com>
$ every Bash command naming git, main transcript (no sub-agents in this arm)
  git add src/router.ts test/router.test.ts && git status
  git log -1 --stat
  git status --short; git log -1 --format="%H %s"
  git log -1 --stat; echo "---"; git status --short
  → 0 commit / reset / rebase / revert / amend

# ── cell 2 — independent rig, installed registry binary ───────────────────────
# setup
rig=$(dev/jigc-rig fresh --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit
$JIGC --version                                        # jigc 1.0.0-rc.24
printf 'one\n' > notes.txt; git add notes.txt
$JIGC start --workflow quick-fix "add a notes file (TKT-212)" --format json      # exit 0, task add-a-notes-file-tkt
$JIGC doc set-field commit:add-a-notes-file-tkt#header/type --value chore --task add-a-notes-file-tkt
printf 'add a notes file' | $JIGC doc set-slot commit:add-a-notes-file-tkt#summary --from-file - --task add-a-notes-file-tkt
printf 'Refs: TKT-212'    | $JIGC doc set-slot commit:add-a-notes-file-tkt#body    --from-file - --task add-a-notes-file-tkt
$JIGC task finalize add-a-notes-file-tkt --carry-staged                           # exit 0, finalized 24063e8
   # (--carry-staged because this rig staged the file before the task existed; a rig-ordering
   #  detail, nothing to do with the lead)

# before-control: a branch that only moved forward is clean
$ python3 -c 'from driver.observe import history_surgery; …(Path("."))'          # []

# the act — jigc only, no git history command
$JIGC task amend --format json                                                    # exit 0, task amend-24063e8
$JIGC doc set-field commit:amend-24063e8#type --value chore --task amend-24063e8
printf 'add a notes file' | $JIGC doc set-slot commit:amend-24063e8#summary --from-file - --task amend-24063e8
printf 'Refs: TKT-221'    | $JIGC doc set-slot commit:amend-24063e8#body    --from-file - --task amend-24063e8
$JIGC task finalize amend-24063e8                                                 # exit 0
  amended 24063e8 → b18a593 — chore: add a notes file
    the tree and the author are unchanged; the message is re-authored and the committer becomes you, now
    the superseded commit stays reachable in the reflog

# after
$ git reflog --format='%h %gd %gs'
  b18a593 HEAD@{0} commit (amend): chore: add a notes file
  24063e8 HEAD@{1} commit: chore: add a notes file
  4c290fd HEAD@{2} commit: chore(jigc): install jigc workspace config
  cd81dc8 HEAD@{3} commit (initial): initial
$ history_surgery(Path("."))                           # the list run.py prints as HISTORY REWRITTEN
  b18a593 HEAD@{0} commit (amend): chore: add a notes file
$ tree before == tree after                             # yes

# control — a raw amend, outside jigc, draws a line of the same shape
$ git commit -q --amend -m "chore: add a notes file (raw amend control)"          # exit 0
$ history_surgery(Path("."))
  3ead3f1 HEAD@{0} commit (amend): chore: add a notes file (raw amend control)
  b18a593 HEAD@{1} commit (amend): chore: add a notes file

# ── the code ─────────────────────────────────────────────────────────────────
completions/trial-driver/driver/observe.py:566-567
    return [ln for ln in got.stdout.splitlines()
            if ln.split(" ", 2)[-1].startswith(("reset:", "rebase", "revert:", "commit (amend)"))]
completions/trial-driver/driver/observe.py:649
    surgery = tuple(history_surgery(log.parent.parent.parent))
completions/trial-driver/run.py:221-226
    for line in o.history_surgery: print(f"  HISTORY REWRITTEN — {line}")
    if o.commit_writes or o.history_surgery:
        print("  ^ a commit produced or rewritten outside jigc. …")
```

Cell 1 was run through `run.py observe` end to end. Cell 2 calls `history_surgery()` directly rather than `run.py observe`, because a rig repo carries no `PROVENANCE.txt` or invocation log (the rig's corpus has invocation logging off) and `observe` reads both; the function is the whole of the reflog half, and `run.py` prints its return value line for line.

### Contract

- `protocol/protocol.md` §8.3, first bullet, declares the bound and that the tooling is not patched for it; §0.3 declares it as a driven fact; §4.3's B-1 row requires *"exactly one `commit (amend)` in the reflog"* together with the `task amend` record, and B-3 is the amend/reset/rebase *"with no `task amend` record behind it"* — so the protocol already scores on the join the reader does not make.
- `completions/trial-driver/README.md` (*Found on the RC-rc14 trial — the reader had no write channel*) and the `history_surgery` docstring call the reflog half *"exact"*. It is exact about **whether** history was rewritten and silent about **by whom**; the caption `run.py` attaches claims the second. That channel was written on 2026-09-10 for a `git reset --soft` bypass; `jigc task amend` (M53) is the product's own route to a `commit (amend)` reflog entry, and the reader predates being reconciled with it.

### Tier

**n/a** — tooling lead. No product behaviour is in question: `jigc task amend` did what §0.3 says it does in both cells. The effect on this trial's scoring is nil because §8.3 declared the bound before the run and arm (b) was read against the `task amend` record, as `rubric/arm-b.txt` shows.

### Pin

**UNPINNED:** `completions/trial-driver/test_observe.py::test_the_reflog_half_is_exact_and_needs_no_transcript` pins only the `reset:` shape (a `git reset --soft HEAD~1` must be seen) and the clean-branch `[]`. No test names `amend` in either direction — neither that a raw amend is flagged nor that a `task amend`-backed one is told apart — so the behaviour confirmed here is held by the filter's text alone. `python3 -m unittest test_observe` → 63 tests, OK, today.

### Notes

- Nothing was modified under `~/out` or `~/ideas`; no tooling was patched; no commit was made in the repository. The rig repo is a `mktemp -d` root under the session scratch directory.
- §8.2's second bullet applies: the `HISTORY REWRITTEN` line is read from the last turn's out-dir. Turn 1's out-dir draws none, since the amend happened in turn 2.
- A datum for whoever takes the tooling up, stated without a proposal: at the point `observe()` computes `surgery` it already holds the parsed invocation records, including the `task amend` / `task finalize amend-<sha>` pair whose short sha is the superseded commit the reflog names at `HEAD@{1}`.

---

## T-2 · `observe` reads arm (a) turn 2 as `VOID unmeasured — no authoring occasion existed`: the row, the nine records and the doc-write predicate reproduce; "none by construction" is false (the sub-task text offers an ADR), and the row is the registered read-back reading, not a mis-score — **PARTIAL**

`tier:` n/a

**Door:** `completions/trial-driver/driver/observe.py` (`_AUTHORING`, `_is_authoring`, `authoring_writes`); `completions/trial-driver/driver/cascade.py` (row 6); `completions/trial-driver/run.py` (`_row`, the `VOID` mark) · **Kind:** tooling · **Tier:** n/a

### The claim

`observe` scored arm (a) turn 2 `VOID unmeasured — no authoring occasion existed` although the
turn holds nine records including `milestone provision`, three `start --task` and
`milestone finalize`: its occasion predicate counts doc writes, and a fan-out turn under default
squash has none by construction. Source: operator note; `rubric/observe-a.txt` against arm a
#40–#48.

### Verdict

**PARTIAL** — the row, the nine records and the predicate reproduce exactly; the last clause
(*"has none by construction"*) is false, and the row is the reader doing what the protocol
registered, not a mis-score.

- **Reproduces — the row.** `run.py observe` over the two saved out-dirs prints, for `turn02`,
  `9 / 0 / 0 / 0 / 0  VOID unmeasured — no authoring occasion existed`, at exit 0. With
  `--gate gate-rc24.json` the output is byte-identical to `rubric/observe-a.txt` (`diff`, no
  lines).
- **Reproduces — the nine records.** Turn 2's `session-start` is `2026-10-03T22:29:56Z`; the
  cumulative log holds 48 records, 39 older than it, nine at or after it (#40–#48): two bare
  `start`, **four** `start --task` (one refused at exit 1 — #41, the resume refusal that routes
  to `provision` — and three at exit 0), `milestone provision` (0), `validate` (0),
  `milestone finalize` (0). The claim's "three `start --task`" is the three at exit 0.
- **Reproduces — the predicate.** `authoring_writes` counts the scored records whose first two
  verb tokens are one of `doc create`, `doc author`, `doc set-slot`, `doc set-field`,
  `doc add-item`, `doc retitle-item`, at exit 0, with no `--help`/`-h` (`observe.py`,
  `_AUTHORING` / `_is_authoring` / the `authoring = sum(…)` line). Row 6 of `DEFAULT_CASCADE` is
  `lambda o: o.authoring_writes == 0`, `void=True`, and it outranks every scored row. Run over
  turn 2's nine records with the driver's own functions, `_is_authoring` is `False` for all
  nine. No `milestone …`, `start …` or `workflow …` verb is in the set.
- **Falsified — "has none by construction".** A fan-out turn under default squash has no
  *required* doc write (the sub-task composed text asks for no commit doc — protocol §0.2), but
  it has an *offered* one: the `sub-task` workflow declares
  `allows-create: [{type: adr, as: decision}]`
  (`crates/cli/packs/dev/workflows/sub-task.yaml`), and the composed text each of this arm's
  three sub-tasks was handed prints `jigc doc create adr --title <TITLE> --task <sub-task>`,
  the three `doc set-slot adr:<slug>#…` lines **and** the read-back instruction
  `jigc doc show adr:<slug> --task <sub-task>` (arm a tool results for #43–#45; the scorer's
  own lead L-20 quotes the same offer). Driven on `jigc 1.0.0-rc.24` in a throwaway rig (cell 2
  below): inside a provisioned sub-task worktree, with `finalize.fan-out.squash = true
  (pack-default)`, `doc create adr` and three `doc set-slot` exit 0, land in the **main
  checkout's** invocation log, and `milestone finalize` promotes the ADR in the one squash
  commit (`"docs": 1` for that sub-task). The driver's `observe()` over that log reads
  `wrote 4, VERB 1` → row 7, a scored row. So the reader can see an authoring occasion in a
  fan-out turn; this session's three sub-tasks did not take the offer (`"docs": 0` on all three
  in #48's output; no `docs/decisions/` in the out-dir), which is why the count is zero here.
- **Not a mis-score.** Row 6 is a statement about the read-back channel and nothing else
  (protocol §7: *"`run.py observe` scores the read-back channels"*; §8: *"`observe` reads the
  read-back and nothing else"*), and for turn 2 it is true: no author-supplied prose was staged
  in that turn, so nothing existed to read back. The arm's class is read by hand from §3.3, whose
  A-V does not mention `observe`'s mark. The protocol pre-declared the shape (§8.2: *"A turn
  that did little reads as void … turn 2's row is row 5 or row 6. That is the split, not the
  apparatus"*; runbook §7: *"a turn 2 that did little shows `VOID` at row 5 or 6"*), and the
  conversation's read-back figure is the sum of both rows (§8.2): 46 records, 12 authoring
  writes, VERB 2, adjacent 2.

**What survives as a tooling observation** (stated, not fixed):

1. The printed mark is `VOID` for every `void=True` row, and row 6 shares it with rows 1–5
   (apparatus). A row that reads `VOID` beside a turn that carried the arm's whole measured
   outcome (`provision` → three sub-task composes → `finalize`, class A-1's evidence) is the
   read-back reader's void, not the arm's — nothing in the row says which.
2. The protocol's stated reason for a row-6 turn 2 does not describe this turn. §8.2 and the
   runbook explain it as *"a turn that did little"* / *"if turn 1 did everything"*. Turn 2 did
   not do little; it is row 6 because the fan-out's doors (`milestone …`, `start --task`) are
   not authoring verbs and the only authoring the sub-task text offers is optional. The declared
   bound covers the outcome and misnames its cause.
3. No driver test carries a fan-out-shaped log; the row-6 tests use a truncated session
   (`records=13, authoring_writes=0`).

### Repro block

```
setup
  TRIAL = the trial directory (protocol/, rubric/, gate-rc24.json)
  A1 = ~/out/RC24-A-frozen-work/turn01     A2 = ~/out/RC24-A-frozen-work/turn02   (read-only)
  repo at bffa6667 (work/rc24-gate); python3 3.9.6; PYTHONDONTWRITEBYTECODE=1 (no bytecode
  written into the repository); git status --porcelain identical before and after

1. the scorer's repro — the reader's row
  argv     python3 completions/trial-driver/run.py observe --gate $TRIAL/gate-rc24.json $A1 $A2
  exit     0
  lines    session           recs wrote  VERB  adj  fs  outcome
           turn01              37    12     2    2   0       read back through the fence's verb
             note: 2 record(s) predate this session (plant/adoption) and are NOT scored — …
           turn02               9     0     0    0   0  VOID unmeasured — no authoring occasion existed
             note: 39 record(s) predate this session (plant/adoption) and are NOT scored — they
             would have added 2 VERB and 2 adjacent
  after    diff against $TRIAL/rubric/observe-a.txt -> no lines (identical)
           the same argv without --gate -> same two rows, provenance line reads "ungated: …"

1a. the nine records, classified by the driver's own predicates (read_log, _is_authoring,
    channels.is_verb, channels.is_adjacent; cutoff = A2/PROVENANCE.txt session-start 22:29:56Z)
  total 48, pre-session 39
  #   time      exit  argv                                                    authoring verb adj
  40  22:29:57Z  0    start                                                   False  False False
  41  22:30:12Z  1    start --task cap-the-store-at-1000 --format json        False  False False
  42  22:30:15Z  0    milestone provision bound-what-the-service-will …       False  False False
  43  22:30:18Z  0    start --task cap-the-store-at-1000 --format json        False  False False
  44  22:30:24Z  0    start --task reject-a-sample-whose-series --format json False  False False
  45  22:30:24Z  0    start --task reject-a-wire-line-longer --format json    False  False False
  46  22:31:30Z  0    start                                                   False  False False
  47  22:33:58Z  0    validate --format json                                  False  False False
  48  22:33:58Z  0    milestone finalize bound-what-the-service-will …        False  False False
  turn-1 authoring writes at exit 0: 12      whole conversation: 12

1b. the row's unit pins
  argv     (cd completions/trial-driver && python3 -m unittest test_cascade)   -> Ran 23 tests, OK
  argv     (cd completions/trial-driver && python3 -m unittest test_observe)   -> Ran 63 tests, OK

2. the falsifying datum for "none by construction" — a fan-out sub-task, default squash,
   authoring a doc (released binary, throwaway rig; nothing under ~/out touched)
  setup    rig=$(dev/jigc-rig fresh --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit
           $JIGC --version                              -> jigc 1.0.0-rc.24
           $JIGC config set invocation-log true         -> exit 0
           $JIGC config get finalize.fan-out.squash     -> finalize.fan-out.squash = true  (pack-default)
           $JIGC milestone create "Bound it" --format json                           -> exit 0
           $JIGC milestone add-task bound-it "Piece one caps the store" --format json -> exit 0
           $JIGC milestone add-task bound-it "Piece two caps the line" --format json  -> exit 0
           $JIGC milestone provision bound-it --format json                           -> exit 0
           cd $REPO/.jigc/worktrees/piece-one-caps-the-store
  argv     $JIGC start --task piece-one-caps-the-store
  exit     0
  lines    Run: `jigc doc create adr --title <TITLE> --task piece-one-caps-the-store`
           jigc doc show adr:<slug> --task piece-one-caps-the-store
           … but never `git commit` and never `jigc task finalize` here …
           (no `commit:` address anywhere in the text)
  argv     $JIGC doc create adr --title "Refuse silently" --task piece-one-caps-the-store --format json
  exit     0     -> "op": "create", "target": {"doctype": "adr", "slug": "refuse-silently"}
  argv     $JIGC doc set-slot adr:refuse-silently#{context,decision,consequences} --from-file - --task piece-one-caps-the-store --format json
  exit     0, 0, 0
  argv     $JIGC doc show adr:refuse-silently --task piece-one-caps-the-store --format json
  exit     0
           (one staged code file `git add`ed in each of the two worktrees; cd $REPO)
  argv     $JIGC milestone finalize bound-it --format json
  exit     0
  lines    "subject": "Finalize milestone bound-it (2 sub-tasks)"
           {"kind": "promoted", "path": "docs/decisions/refuse-silently.md"}
           sub_tasks: piece-one-caps-the-store "docs": 1 · piece-two-caps-the-line "docs": 0
  after    $REPO/.jigc/logs/invocations.jsonl: 15 records; #9 `doc create adr …`, #10–#12
           `doc set-slot adr:refuse-silently#…`, #13 `doc show adr:refuse-silently --task …`,
           all exit 0 — written from inside the worktree, logged in the main checkout's log
           driver.observe.observe("rig", <that log>) + cascade.grade:
             records 15  wrote 4  VERB 1  adj 0  -> row 7 | score | read back through the fence's verb

3. the arm's own evidence that the offer was printed and not taken (read-only)
  A2 main transcript, tool results for #43–#45: each composed text carries
    "Run: `jigc doc create adr --title <TITLE> --task <sub-task>`"   (3 of 3)
  A2 #48 output: "docs": 0 for all three sub-tasks; `ls $A2/docs/decisions` -> no such directory
  A2 sub-agent transcripts (3): one `jigc` invocation in all — a bare `jigc start` at
    22:31:30Z (= record #46, orientation); no `jigc doc …` in any of them
```

### The contract

- `RC24 protocol.md` §7 / §7.1 — `observe` scores the read-back channels; row 6
  `unmeasured — no authoring occasion existed | void` is registered unchanged from RC-rc14, and
  `test_cascade.py` compares the registered table with `DEFAULT_CASCADE` position by position.
- `RC24 protocol.md` §8 — *"`observe` reads the read-back and nothing else"*; the three outcome
  classes are read by hand. §8.2 — turn rows are per turn; *"A turn that did little reads as
  void … row 5 or row 6. That is the split, not the apparatus."* §3.3 — A-V lists a non-zero CLI
  exit, no invocation log, an aborted `seed`, a cold turn 2; it does not list an `observe` void.
- `RC24 protocol.md` §0.2 — a sub-task's composed text has no commit-boundary steps and under
  default squash asks for no commit doc. It does not say the text asks for *no doc*; the same
  text offers the ADR.
- `driver/cascade.py`, row 6's own comment — the row exists so that *"a session with nothing
  staged cannot have read its staged work back"*; `driver/observe.py`, `_AUTHORING` —
  *"Invocations that put author-supplied prose into a managed doc."*
- `crates/cli/packs/dev/workflows/sub-task.yaml` — `allows-create: [{type: adr, as: decision}]`.

Against these: the row is correct for what it measures and was declared in advance; the claim's
causal clause overstates (*optional*, not *none*); the declared reason in §8.2 ("did little")
does not fit a turn that ran the whole fan-out.

### Tier

**n/a** — tooling. No product behaviour is in question: every jigc door in the repro exits 0 and
does what its text says. The lead changes no class: arm (a) is scored from §3.3 by hand, and the
operator's "none void" stands.

### Pin

`pinned-by: test_cascade::UnmeasuredIsNotNeither::test_a_session_with_nothing_staged_cannot_be_scored_for_read_back`
— the confirmed half (zero authoring writes → row 6, void), with
`test_observe::AuthoringWrites::test_reads_and_orientation_are_not_writes` holding `start` and
`validate` outside the authoring set. **Unpinned:** the fan-out shape itself — no driver test
carries a log of `milestone provision` / `start --task` / `milestone finalize` with or without a
sub-task ADR, so neither "a fan-out turn with no ADR reads row 6" nor "a fan-out turn with an ADR
is scored" is fenced; cell 2 above is the only evidence for the second.

### Notes

- Record #41 (`start --task …`, exit 1) is a fourth `start --task`; it is the declared resume
  refusal (§0.2) and the route this session took to `provision`.
- The two bare `start` records in turn 2 (#40, #46) are orientation calls; neither is an
  authoring act, and `observe` counts both in `recs`. #46 is a sub-agent's (its transcript
  carries `jigc start` at 22:31:30Z); #40 falls one second after `session-start` and no tool
  call in the main transcript carries it — whose it is was not traced.
- That `jigc start` is the only `jigc` command any of the three sub-agents ran; the other
  turn-2 records are the main agent's. #46 landing in the log, and cell 2, both show a
  worktree-side or sub-agent invocation reaches the log the reader scores, so a sub-agent
  that had authored would have been counted.
- Nothing was modified under `~/out` or in the repository; the rig lives under a fresh
  `mktemp -d` root and was not torn down.

---

## T-3 · `observe`'s `git!` detector misses `git -C <path> commit` and `git -c k=v commit`; no arm of this trial typed a history verb in that (or any) raw-git shape, and the table row never reads the channel — **CONFIRMED**

`tier:` n/a

**Kind:** tooling · **Door:** `completions/trial-driver/driver/observe.py` → `commit_writes()`
(printed by `completions/trial-driver/run.py observe` as `git! <verb>` lines) · **Origin:**
operator note; protocol §8.3, second bound.

### The claim

`observe`'s `git!` detector reads the first non-flag word after `git` as the subcommand and so
misses `git -C <path> commit` and `git -c k=v commit`; no worker typed that shape in this
trial, so it changed no row.

### Verdict

**CONFIRMED** — both halves.

1. **The miss is real and is the mechanism the lead names.** `commit_writes()` computes
   `verb = next((w for w in rest if not w.startswith("-")), "")` over the words after `git`
   (observe.py lines 538–541). For `git -C /work commit -m x` that is `/work`; for
   `git -c user.name=x commit -m x` it is `user.name=x`. Neither is in `_GIT_HISTORY_VERBS`,
   so no `Write` is recorded and `run.py observe` prints no `git!` line and no `^ a commit
   produced or rewritten outside jigc` line. Driven end to end through `run.py observe` on
   synthetic out-dirs, with a caught control beside each miss.
2. **It changed no row in this trial.** Two independent reasons:
   - the table row is `cascade.grade(o)`, and nothing outside `run.py`'s three print lines
     (218–223) reads `commit_writes` — a `git!` line is evidence printed *under* a row and
     cannot change a row's outcome whatever it catches or misses;
   - there was nothing for it to miss. An independent scan of every `tool_use` in every
     transcript and `stream.jsonl` of the three arms and both debriefs (no assumption about
     argv shape) finds **zero** Bash commands that name `git` together with a history verb
     (`commit`, `reset`, `revert`, `rebase`, `cherry-pick`, `am`, `merge`, `update-ref`, …), in
     any shape. And each arm's post-adoption reflog entry coincides to the second with an
     exit-0 jigc record that commits.

One precision on the wording "no worker typed that shape": a **sub-agent in arm a did type
`git -C <worktree> log --oneline -5`** (turn 2). That is the `-C` shape with a read verb, not
a history verb, so the detector had nothing to report either way. No `git -c <k>=<v> …` was
typed anywhere. The claim as scoped (`git -C <path> commit`) holds.

### Repro

```
# setup — synthetic out-dirs under a fresh `mktemp -d` root <tmp>; nothing under ~/out touched.
# Each <tmp>/<name>/ holds:
#   PROVENANCE.txt                      image synthetic-fixture · jigc-sha unknown · exit-code 0
#   .jigc/logs/invocations.jsonl        empty
#   .session-transcript/main.jsonl      one line:
#     {"type":"assistant","message":{"content":[{"type":"tool_use","name":"Bash",
#       "input":{"command":"<the command below>"}}]}}

# argv — one run per fixture, from $REPO (branch work/rc24-gate, observe.py last touched ba826655)
python3 completions/trial-driver/run.py observe <tmp>/<name>

# observed — exit 0 and stderr empty on every run; the row is the same on every run
#   "<name>   0  0  0  0  0  VOID apparatus — the session did nothing"   (no log records: expected)
# what differs is the `git!` line:

 command in the transcript                     git! lines   line printed
 git commit -m x                               1            git! commit     [Bash] git commit -m x      (control)
 git -C /work commit -m x                      0            —                                           (the lead's repro)
 git -c user.name=x commit -m x                0            —                                           (the lead's second shape)
 git reset --soft HEAD~1                       1            git! reset      [Bash] git reset --soft HEAD~1   (control)
 git -C /work reset --soft HEAD~1              0            —
 git -C /work commit --amend --no-edit         0            —
 git --git-dir=/work/.git commit -m x          1            git! commit     [Bash] git --git-dir=/work/.git commit -m x
 git --git-dir /work/.git commit -m x          0            —
 cd /work && git commit -m x                   1            git! commit     [Bash] git commit -m x
 git -C /work status                           0            —                                           (true negative)

# a miss prints no "^ a commit produced or rewritten outside jigc" line either; the caught
# controls print it.

# the protocol's backstop over the same fixtures
python3 <trial>/protocol/tools/raw-git-acts.py <tmp>/<name>
#   -> "1 raw git history act(s) across 1 transcript(s)" for every history-verb row above,
#      "0 raw git history act(s)" for `git -C /work status`.

# the trial's own evidence, read-only
python3 completions/trial-driver/run.py observe --gate <trial>/gate-rc24.json \
    ~/out/RC24-C ~/out/RC24-A-frozen-work/turn01 ~/out/RC24-A-frozen-work/turn02 \
    ~/out/RC24-B-frozen-work/turn01 ~/out/RC24-B-frozen-work/turn02
#   -> exit 0, stderr empty, 0 `git!` lines; rows identical to rubric/observe-{a,b,c}.txt;
#      one `HISTORY REWRITTEN — 4b80510 HEAD@{0} commit (amend): …` under arm b turn02
#      (the reflog half; §8.3's first bound — `jigc task amend`'s own amend).
python3 <trial>/protocol/tools/raw-git-acts.py <each of the five out-dirs>
#   -> "0 raw git history act(s)" ×5 (1, 1, 4, 1, 1 transcripts) — matches rubric/raw-git-*.txt.

# independent scan (shape-free): every tool_use in every *.jsonl outside .git/ and .jigc/
 out-dir                         jsonl  tool_use  Bash  Bash naming git  …with a history verb
 ~/out/RC24-C                    2      50        28    0                0
 ~/out/RC24-A-frozen-work/turn01 2      76        64    2                0
 ~/out/RC24-A-frozen-work/turn02 5      150       98    31               0
 ~/out/RC24-B-frozen-work/turn01 2      54        40    4                0
 ~/out/RC24-B-frozen-work/turn02 2      43        36    6                0
 ~/out/RC24-A-debrief            3      134       102   22               0
 ~/out/RC24-B-debrief            3      70        56    8                0
#   git verbs actually typed: log, status, add, diff --cached, worktree add/remove/list.
#   The one `-C` occurrence: arm a turn02, a sub-agent,
#     `git -C /work/.jigc/worktrees/reject-a-wire-line-longer log --oneline -5`.

# reflog ↔ invocation log, post-adoption entries only (UTC)
 arm c   22:21:24 commit           ↔ 22:21:24 exit 0  task finalize document-the-service-architecture-how
 arm a   22:29:40 commit           ↔ 22:29:40 exit 0  task finalize bound-what-the-service-will
         22:29:45 commit           ↔ 22:29:45 exit 0  milestone create …
         22:29:48 commit ×3        ↔ 22:29:48 exit 0  milestone add-task … ×3
         22:33:58 merge (ff)       ↔ 22:33:58 exit 0  milestone finalize bound-what-the-service-will
 arm b   22:23:05 commit           ↔ 22:23:05 exit 0  task finalize fix-get-summary-with-empty
         22:23:38 commit (amend)   ↔ 22:23:38 exit 0  task finalize amend-7757a4c   (after `task amend`, 22:23:20)

# after
git status --short in $REPO: unchanged (the one pre-existing untracked directory). Nothing
written under ~/out or ~/ideas. The fixtures live under the session scratchpad only.
```

### The contract

- `completions/trial-driver/README.md` → *Found on the RC-rc14 trial — the reader had no write
  channel*: "`commit_writes()` reads the transcript for git verbs that produce or rewrite a
  **commit**". A `git -C <path> commit` is such a verb and is not read — the stated purpose is
  not met for that shape.
- The same tooling already frames the transcript half as fallible:
  `observe.py::history_surgery` — "The transcript half above can miss a rewrite (a format
  change, a command shape nobody anticipated). The corpus cannot"; `Write`'s docstring — "a
  **heuristic** over the transcript … returned as evidence for review, never as a verdict". So
  the miss is a known *class* with an exact backstop (the reflog), which is why it is a tooling
  bound and not a scoring error.
- This trial's protocol §8.3 declares the bound before the run and names the two compensating
  reads (`tools/raw-git-acts.py`; the reflog and trailer join). Both were run and saved
  (`rubric/raw-git-*.txt`, `rubric/trailer-*.txt`) and both reproduce here.
- Why the shape matters: protocol §0 records that every `git` command jigc prints leads with
  `git -C <absolute path>`, so the shape the detector cannot see is the shape the product
  teaches. The bound is stated in the trial's protocol only; the tooling's own
  `README.md` → *Bounds* does not carry it.

### Tier

**n/a** — tooling lead. It is a false negative in an evidence-only channel of the trial
reader, not a behaviour of the jigc binary, and it altered no outcome class in this trial.

### Pin

`UNPINNED: no test drives commit_writes() with a git global option before the subcommand.`
`test_observe.py::ACommitMadeOutsideJigcIsSeen` (4 tests, all pass: exit 0, "Ran 4 tests … OK")
pins the bare shapes only — the archived RC-rc14 B1 transcript (`git reset --soft HEAD~1`,
`git commit`), the `jigc task finalize` exclusion, the staging exclusion, and the reflog half.
A grep of `test_observe.py` for `git -` finds no `-C` / `-c` fixture. The only place the miss
is recorded as driven is this trial's protocol (§8.3, "five shapes, two caught") and the
docstring of `protocol/tools/raw-git-acts.py`, neither of which is a test.

### Notes

1. **The gap is wider than the two shapes the lead names**, all through the same function.
   Driven on the same fixture rig, each at exit 0 with 0 `git!` lines:

   | command | why it is missed |
   |---|---|
   | `git --git-dir /work/.git commit -m x` | same mechanism: a global option taking a separate value |
   | `GIT_AUTHOR_NAME=x git commit -m x` | `_segments` yields the env assignment as the program, so `program != "git"` |
   | `printf 'msg\n' \| git commit -F -` | `_segments` yields only the **head** of a pipeline (a read-side rule reused on the write side) |
   | `(git commit -m x)` | the program word is `(git` |
   | `/usr/bin/git commit -m x` | the program word is the absolute path |

   `raw-git-acts.py` catches the first four and **misses the absolute-path shape** (its
   `(?<![\w./-])git` look-behind excludes a `/` before `git`). None of these shapes occurs in
   any transcript of this trial (the shape-free scan above), so none changed a row either.
2. The `--git-dir=<path>` form (one word) *is* caught, so the detector is not blind to global
   options as such — only to those whose value is a separate word.
3. Bound of this verification: a sub-agent's commit inside a fan-out worktree lands on that
   worktree's own HEAD log, which is removed with the worktree at `milestone finalize`, so for
   arm a's sub-agents the transcript scan is the only evidence and the reflog join does not
   cover them. The scan read all three sub-agent transcripts of arm a turn 2 and found `git
   add`, `git status` and one `git -C … log` — no commit.
4. Not tested: a git history act run from inside a script file a worker wrote and executed
   (no `git` word in the Bash command). The reflog join above is the check that covers it for
   the main checkout, and it is clean in all three arms.
5. No tooling was patched; the fixtures and scan scripts are scratch files outside the
   repository.

---

## T-4 · `raw-git-acts.py` matches nine history verbs only; it printed 0 for arm (a), whose main agent ran `git worktree add`, `git apply` and `git worktree remove --force` (tool calls #62, #63, #65) — **CONFIRMED**

`tier:` n/a

**Lead (tooling).** `tools/raw-git-acts.py` lists only `commit`, `reset`, `revert`, `rebase`,
`cherry-pick`, `am`, `merge`, `filter-branch` and `update-ref`; it printed 0 acts for arm (a),
whose main agent ran `git worktree add`, `git apply` and `git worktree remove --force`.

**Door:** `protocol/tools/raw-git-acts.py` (this trial's helper; not in the repository).

### Verdict — CONFIRMED

Every clause of the lead reproduces, read and driven on the unmodified helper:

| Clause | How it was checked | Result |
|---|---|---|
| the verb list is those nine | the helper's line 27, read | **holds** — `VERBS = "commit\|reset\|revert\|rebase\|cherry-pick\|am\|merge\|filter-branch\|update-ref"`; the pattern accepts a statement only when one of them follows `git` as a whole word |
| it printed 0 for arm (a) | the helper re-run, bare, on `~/out/RC24-A-frozen-work/turn02` | **holds** — exit 0, one line: `0 raw git history act(s) across 4 transcript(s)`; byte-identical to the operator's saved `rubric/raw-git-a.txt`. Turn 1's out-dir: `0 … across 1 transcript(s)` |
| the main agent ran the three commands | arm (a)'s cumulative main transcript, every `tool_use` block numbered | **holds** — #62 `git worktree add <tmp>/milestone-verify a9f1851` (result: *"Preparing worktree (detached HEAD a9f1851)"*), #63 `git diff --cached > <tmp>/d{1,2,3}.patch` in each sub-task worktree then `git apply` of the three patches in the scratch worktree (no `--index`), #64 `npm test` there (33 pass, 0 fail), #65 `git worktree remove <tmp>/milestone-verify --force`. All four between 22:33:43 and 22:33:55, all `is_error=False`, all in the main transcript, none in a sub-agent's |
| (control) the helper does read this transcript shape | a synthetic one-file transcript holding the three statements verbatim plus `git -C /work commit -m control` | the control is listed, the three are not — so the 0 is the verb list, not a parse failure |
| (cross-check) the other instrument | `python3 completions/trial-driver/run.py observe ~/out/RC24-A-frozen-work/turn02` | exit 0, zero `git!` lines — its set is `commit, reset, revert, rebase, cherry-pick, am` (`driver/observe.py` → `_GIT_HISTORY_VERBS`). Neither reader shows #62–#65 |

**What the confirmation does and does not carry.**

- **The helper's printed sentence is true.** It says *history* act, and none happened: after turn 2
  the repository holds 15 commit objects, all 15 reachable from the one ref (`refs/heads/main`) or
  the reflog; the reflog is the nine pre-session commits plus the six door entries; `git worktree
  list` shows the main checkout only and `.git/worktrees` does not exist. The scratch worktree was
  detached, took no commit, moved no ref and left nothing behind.
- **No class and no trailer row rested on it.** Arm (a)'s class (§3.3) names no raw-git condition;
  the one class that does is B-1 (*"no raw git history act in the transcript"*), and a census of
  every `git` statement in all five out-dirs finds no history verb anywhere — arm (c) none at all,
  arm (b) `add`/`log`/`status` only, arm (a) `add`, `diff`, `log`, `status`, `apply`,
  `worktree add|list|remove`. The trailer join (§6.1 item 3) needs raw *commits*, and there were
  none.
- **The gap is between the protocol's name for the helper and its scope.** §6.1 and §8.1 step 5
  call its output *"the raw git acts"*; the helper lists history verbs. A scorer who reads
  *"0 raw git … act(s)"* as *"the worker used no raw git beyond reads"* is wrong for arm (a), and
  the scratch worktree — the subject of L-17 and L-22 — is visible only by reading the transcript
  by hand, which is how the scorer found it.
- **The under-match is wider than the three shapes the lead names** (driven on the same synthetic
  transcript, extra to the lead): the helper is also silent on `git worktree add -b <branch>`,
  `git branch -f`, `git checkout -B`, `git switch -C`, `git pull --rebase`, `git stash`,
  `git tag -f`, `git commit-tree`, `git push --force`, `git apply --index` and `git add` — several of which
  create a commit or move a ref, which is inside the helper's own stated purpose (*"an under-match
  is a rewrite nobody saw"*). None of these was typed in any arm of this trial.

### Repro

```
# setup — nothing to build; the saved out-dir and the trial's helper, both read-only
TOOLS=<trial>/protocol/tools
A2=~/out/RC24-A-frozen-work/turn02

# argv 1 — the lead's repro, bare
python3 $TOOLS/raw-git-acts.py $A2
#   exit 0
#   0 raw git history act(s) across 4 transcript(s)

# argv 2 — what the main transcript holds (tool_use blocks numbered across the cumulative file)
#   #62  cd /work && git worktree add <tmp>/milestone-verify a9f1851 2>&1 | tail -5
#          -> Preparing worktree (detached HEAD a9f1851)
#   #63  cd /work/.jigc/worktrees/<each sub-task> && git diff --cached > <tmp>/d{1,2,3}.patch
#        cd <tmp>/milestone-verify && git apply <tmp>/d1.patch <tmp>/d2.patch <tmp>/d3.patch && …
#          -> APPLIED_OK
#   #64  cd <tmp>/milestone-verify && npm test 2>&1 | tail -15      -> # pass 33  # fail 0
#   #65  git worktree remove <tmp>/milestone-verify --force 2>&1    -> (no output)

# argv 3 — the control: the same helper over a synthetic transcript (<tmp>/fx/.session-transcript/main.jsonl),
#          one Bash tool_use per line: the three statements of #62, #63, #65 verbatim, then
#          `git -C /work commit -m control`, then eleven further shapes (see the last bullet above)
python3 $TOOLS/raw-git-acts.py <tmp>/fx
#   exit 0
#   main                         bash#4    commit      git -C /work commit -m control
#   1 raw git history act(s) across 1 transcript(s)

# argv 4 — the other reader
python3 completions/trial-driver/run.py observe $A2        # exit 0; no `git!` line

# after — the repository the session left (read with GIT_OPTIONAL_LOCKS=0)
git -C $A2 worktree list                                   # the main checkout only, cb4a90f [main]
ls $A2/.git/worktrees                                      # No such file or directory
git -C $A2 for-each-ref                                    # refs/heads/main only
git -C $A2 cat-file --batch-all-objects --batch-check | grep -c ' commit '   # 15
git -C $A2 rev-list --all --reflog | wc -l                 # 15 — no commit outside ref + reflog
```

Nothing under `~/out` was modified; the synthetic transcript lives in a fresh `mktemp -d`
directory in the verifier's scratch area.

### The contract

The helper's own docstring: *"List every shell command in a session's transcripts that runs a git
history verb … This lists the command text and scores nothing."* Against that, `worktree add`
(detached), `apply` (working tree only) and `worktree remove` are out of scope and the output is
correct.

The protocol's use of it: §6.1 item 3 (*"each match then checked against the raw git acts in the
transcript, because a raw `git commit` inside a door's window would otherwise be credited to
it"*), §4.3 B-1/B-3 (a rewrite outside jigc), §8.1 step 5 (*"Print the raw git acts"*) and §8.3
(the helper *"reads the transcripts without that assumption"*). Both stated uses are about
commits and rewrites, and for those the three missed commands change nothing. What the protocol
does not have is any instrument that lists raw git **writes that are not history** — a linked
worktree, an index or working-tree patch — and its wording (*"the raw git acts"*) does not say
so. `driver/observe.py` draws the same line on purpose for index moves (*"`add`/`restore`/`rm`/`mv`
are index moves and are NOT here"*) and is silent on `worktree`.

### Tier — n/a

Tooling lead. Its effect on this trial's record is nil: no outcome class, trailer row or void
call depended on the helper's 0, and the scorer's hand read of all 67 tool calls is what the
*outside jigc* note for arm (a) rests on.

### Pin

UNPINNED. The helper exists only in this trial's `protocol/tools/`; no test reads it and nothing
in the repository names it. `open-forks.md` F10/F12 already file its pattern to move into
`completions/trial-driver/driver/observe.py` *"as a fix with a test"* after the record is written;
that test is where the verb set — and a decision on whether `worktree`, `apply`, `stash`,
`branch -f`, `checkout -B`, `switch -C`, `pull`, `tag -f`, `commit-tree` belong in it, or in a
second non-history list — would be pinned. `completions/trial-driver/test_observe.py` pins
`reset` and a manual `commit` for the `git!` channel and nothing for these shapes.

### Notes

- The lead says *"tool calls #62–#65"*: #62, #63 and #65 are the git commands, #64 is the test run
  between them.
- The numbering is over the cumulative main transcript in turn 2's out-dir (67 tool calls, 51 of
  them Bash); the helper's own `bash#` column would have called these bash#46, #47 and #49.
- Not driven: whether the worker's scratch worktree could have been seen from the repository
  afterwards. It could not — `worktree remove` deletes the admin entry, and a detached worktree
  that takes no commit leaves no object — so the transcript is the only channel that holds it.

---

## T-5 · Runbook §10 copies the walk record unrewritten; the copy carries 5 host-path lines and the section's own grep then says STOP — **CONFIRMED**

`tier:` n/a

**Kind:** tooling (this trial's runbook; `completions/trial-driver/walk.py`) · **Tier:** n/a

### The claim

Runbook §10 copies the walk record with plain `cp`; the copy carries host paths (5 lines); the
`prov` rewrite is applied to `PROVENANCE.txt` only; the section's own
`grep -rn -e "$HOME" …` then says STOP.

### Verdict

**CONFIRMED** — all four parts, each measured.

| Part | Datum |
|---|---|
| plain `cp` | `protocol/runbook.md` line 344: `cp ~/out/RC24-walk/walk-record.md $E/walk/walk-record-rc24.md` |
| `prov` on PROVENANCE only | `prov()` is defined at line 327 and called five times (lines 331, 335, 336, 340, 341), every call on a `PROVENANCE.txt`; the four `cp` lines (330, 334, 339, 344) are unrewritten |
| 5 host-path lines | `grep -c "$HOME"` over the copy = 5 (of 293 lines); the copy is byte-identical to its source (`cmp` exit 0) |
| the check says STOP | the §10 grep exits 0 with 5 hits in 1 file, so the `&& echo "STOP: …"` arm fires |

The walk record is the **only** file that trips the check: the three `invocations.jsonl` copies
(also plain `cp`) carry 0 hits, the five rewritten `PROVENANCE` files carry 0, the three
`trailer-rows.txt` carry 0. Each source `PROVENANCE.txt` carries exactly 1 host-path line, so the
`prov` rewrite was needed there and worked.

Counterfactual, to stdout only (nothing written): the same `sed "s|$HOME|~|g"` over the walk record
leaves 0 lines matching either `$HOME` or the login name — the rewrite §10 already defines would
have cleared the check had line 344 used it.

### Repro

```
setup
  TRIAL=<the trial directory>; E=$TRIAL/evidence        # as the operator left it; nothing rebuilt
  REPO=<a checkout of the repository, on the trial's revision>

argv 1   cmp ~/out/RC24-walk/walk-record.md $E/walk/walk-record-rc24.md
observed exit 0                                         # the copy is the source, unrewritten

argv 2   grep -c -e "$HOME" $E/walk/walk-record-rc24.md
observed 5      (exit 0)
         the five lines, home prefix masked as ~ and the checkout as $REPO:
           43: corpus     : ~/ideas/walk-rc24
           44: out        : ~/out/RC24-walk/00-positive-control
           48: running $REPO/completions/trial-driver/arms/walk/00-positive-control.sh in the container
           89: evidence in ~/out/RC24-walk/00-positive-control (corpus + .session-transcript/ + PROVENANCE.txt)
           94:   ^ indicative only — score with: completions/trial-driver/run.py observe "~/out/RC24-walk/00-positive-control"

argv 3   grep -rc -e "$HOME" -e "$(id -un)" $E          # per-file counts
observed walk/walk-record-rc24.md:5
         every other file (A, B, C: invocations.jsonl, PROVENANCE*.txt, trailer-rows.txt):0

argv 4   grep -rn -e "$HOME" -e "$(id -un)" $E && echo "STOP: a host path or a login name is in the evidence"
observed grep exit 0, 5 hit lines, 1 file; then the line
           STOP: a host path or a login name is in the evidence

argv 5   sed "s|$HOME|~|g" $E/walk/walk-record-rc24.md | grep -c -e "$HOME" -e "$(id -un)"
observed 0      (grep exit 1)                           # the counterfactual: prov would have cleared it

argv 6   grep -c -e "$HOME" ~/out/RC24-walk/00-positive-control/ARM-OUTPUT.txt
observed 5                                              # where the five lines come from

after    nothing under ~/out, ~/ideas, $TRIAL/evidence or the repository was written
```

### Where the five lines come from

`walk.py` resolves both of its path arguments to absolute paths
(`pathlib.Path(args.corpus).expanduser().resolve()`, the same for `out`, lines 152–153), hands them
and the arm's absolute path to `run-session.sh --exec` (line 58), and `render()` embeds the
harness's stdout verbatim in the record's fenced block (line 126; the same text is persisted as
`ARM-OUTPUT.txt`, line 67). `run-session.sh` echoes those paths at five sites — `corpus     :`
(161), `out        :` (162), `running <exec-file> in the container` (209), `evidence in` (264), and
the `indicative only — score with` hint (288). Five echo sites, five lines per arm that runs. Only
arm 00 ran in this trial, so the record carries five.

So a walk record carries host paths **by construction**, for any operator on any machine, and one of
the five (line 48) is the path of the repository checkout itself, not of an out-dir.

### The contract

Runbook §10 itself: the comment on `prov()` says `corpus-src is an absolute host path as written`,
and the check that follows says a host path or a login name in `$E` is a STOP "before anything is
committed". The section therefore states the rule, ships the rewrite, and applies it to one of the
two file kinds that need it. Followed exactly as written, §10 always ends at its own STOP, with no
step naming what to do next.

The check did its job: `$TRIAL/evidence/` is not in the repository, and nothing was committed.

### Tier

n/a — tooling. It is not a jigc product behaviour; it changes no arm's outcome class and voids
nothing. Its cost is one manual rewrite of one file before the evidence can be committed.

### Pin

`UNPINNED:` nothing reads a runbook's copy step, and `completions/trial-driver/test_walk.py` holds
two tests (persisted output re-rendered on a later pass; a pass that predates persistence says so),
neither of which asserts anything about paths in the rendered record. The suite is green
(`Ran 2 tests … OK`) with the defect present.

### Notes

- **The door is two places, not one.** The runbook line is the immediate cause; `walk.py`'s
  `render()` embedding absolute paths is why every walk record has them. Fixing only line 344 leaves
  the next trial's runbook to remember the rewrite again.
- **Precedent, for whoever weighs this:** the seven walk records already committed under
  `completions/artifacts/` (three trials) carry home-directory absolute paths too — 5, 5, 48, 5, 5,
  106 and 106 matching lines respectively, all with one and the same first path component. This
  trial's §10 check is stricter than what those trials shipped. `implementation/public-hygiene.md`
  rule 4 does not list a host path, and exempts the author's own public identity; whether a
  home-directory path falls under that exemption is not something this verification decides.
- **Not driven:** whether the repository's own guard (`dev/hygiene-scan`, gitleaks plus the private
  denylist) would flag the file. The denylist is private and was not read.
- The scorer's proposed repro (`grep -c "$HOME" $TRIAL/evidence/walk/walk-record-rc24.md`) is argv 2
  above and returns 5, as stated.

---

## T-6 · test_session.py defines CarryLeavesTheWalksOwnOutputBehind after its unittest.main() entry point (guard at line 373, class at 377), so its one test is never run by the script form — which is the form `run.py test`, the only documented runner, uses — **CONFIRMED**

`tier:` n/a

**Kind:** tooling (the trial driver's own suite) · **Door:** `completions/trial-driver/test_session.py`
· **Tier:** n/a (tooling, not the jigc product)

### The claim

`test_session.py` defines `CarryLeavesTheWalksOwnOutputBehind` after its
`if __name__ == "__main__": unittest.main()` block (lines 373 and 377), so its one test never
runs when the file is run as a script.

### Verdict

**CONFIRMED** — whole claim, both line numbers, and one step further than the lead states: the
script form is the form the driver's own documented runner uses, so the test has no run at all
in the tooling's own test path.

- Line 373 is `if __name__ == "__main__":`, line 374 `unittest.main(verbosity=2)`, line 377
  `class CarryLeavesTheWalksOwnOutputBehind(unittest.TestCase):`, line 381 its one test
  `test_arm_output_is_evidence_not_corpus`. The file is 391 lines; the guard is the last
  statement in every sibling (`test_cascade` 277/278, `test_gate` 160/161, `test_interact`
  126/127, `test_observe` 961/962, `test_plants` 179/180, `test_walk` 45/46) and in this one
  file it is not (373/391).
- `unittest.main()` collects the module namespace as it stands when called and then exits the
  interpreter, so the `class` statement on line 377 is never executed in a script run.
- `run.py test` (README line 33: "seven suites — `python3 run.py test`") runs each suite as
  `subprocess.run([sys.executable, str(suite)])` (`run.py` `do_test`, line 370) — the script
  form. Nothing in `dev/gate`, `.github/workflows/` or `crates/cli/tests/` names the
  trial-driver suites, so that is the only runner there is.
- The file defines 23 `test_` methods; the script form and `run.py test` both report
  `Ran 22 tests … OK`.

What the lead does **not** carry, stated so it is not over-read:

- The unreached test **passes** when it is collected (`python3 -m unittest test_session`:
  `Ran 23 tests … OK`). No behavioural defect in `carry_forward` is hiding behind it today; the
  defect is that the fence is not standing, not that the thing fenced is broken.
- No effect on this trial's evidence was found. `ARM-OUTPUT.txt` / `ARM-STDERR.txt` appear in
  none of the three arms' out-dirs (`~/out/RC24-C`, `~/out/RC24-B-frozen-work/turn01|turn02`,
  `~/out/RC24-A-frozen-work/turn01|turn02`: 0 files, 0 tracked); `~/out/RC24-walk` holds one
  `ARM-OUTPUT.txt`, untracked, as evidence beside the arm — where it belongs.

### Repro

```
setup
  repository at HEAD bffa6667, completions/trial-driver/ clean (git status --short: empty)
  python3 3.9.6, PYTHONDONTWRITEBYTECODE=1, output captured to files under <tmp>

argv 1 — the script form (the lead's repro, unpiped)
  python3 completions/trial-driver/test_session.py -v > <tmp>/script.txt 2>&1
observed
  exit 0
  last lines:  "Ran 22 tests in 0.032s" / "OK"
  grep -c test_arm_output_is_evidence_not_corpus <tmp>/script.txt        -> 0
  grep -c CarryLeavesTheWalksOwnOutputBehind     <tmp>/script.txt        -> 0
  control (a test above the guard, so the grep can find a test at all):
  grep -c test_evidence_is_dropped_and_history_is_kept <tmp>/script.txt  -> 1
  grep -c '^    def test_' completions/trial-driver/test_session.py      -> 23

argv 2 — the same file through the loader, which imports the whole module first
  (cd completions/trial-driver && python3 -m unittest -v test_session > <tmp>/module.txt 2>&1)
observed
  exit 0
  "test_arm_output_is_evidence_not_corpus (test_session.CarryLeavesTheWalksOwnOutputBehind) ... ok"
  "Ran 23 tests in 0.034s" / "OK"

argv 3 — the documented runner
  python3 completions/trial-driver/run.py test > <tmp>/runpy.txt 2>&1
observed
  exit 0
  seven "=== test_*.py ===" banners; the test_session.py block reads "Ran 22 tests"
  grep -c test_arm_output_is_evidence_not_corpus <tmp>/runpy.txt         -> 0

argv 4 — what the missing run costs, on a SCRATCH COPY (repository untouched)
  copy driver/, test_session.py, increment-1-evidence/ to <tmp>/td; in the copy's
  driver/session.py drop `"ARM-OUTPUT.txt", "ARM-STDERR.txt"` from _EVIDENCE_NAMES
  (the exact regression the unreached test exists to catch)
  python3 <tmp>/td/test_session.py
observed
  exit 0, "Ran 22 tests" / "OK"                       <- the regression is green
  (cd <tmp>/td && python3 -m unittest test_session)
observed
  exit 1, "Ran 23 tests" / "FAILED (failures=1)"
  "FAIL: test_arm_output_is_evidence_not_corpus (test_session.CarryLeavesTheWalksOwnOutputBehind)"
  "AssertionError: Lists differ: ['ARM-OUTPUT.txt', 'ARM-STDERR.txt', 'README.md'] != ['README.md']"

after
  git status --short completions/trial-driver: empty before and after (compared with cmp)
  nothing written under ~/out or ~/ideas; no tooling patched
```

### The contract

The driver's README owns it, in its own words, about the identical defect in the sibling suite
(`completions/trial-driver/README.md`, "Found preparing the RC-rc14 trial (2026-09-09)"): an
entry-point block "sat **mid-file**", `unittest.main()` "collects the module namespace *as it
stands when it is called*", and "A green suite that silently drops a third of itself is this
directory's own warning coming true". `test_observe.py` line 649 carries a `NOTE (2026-09-09)`
marking where that block was removed. The same README (line 88) names
`test_session.py::CarryLeavesTheWalksOwnOutputBehind` as the fence for the fix that put
`ARM-OUTPUT.txt` / `ARM-STDERR.txt` into `_EVIDENCE_NAMES` — a fence the documented runner has
never executed.

Origin: commit `e70c2593` (2026-09-04) appended a class below the entry point in **both**
`test_observe.py` and `test_session.py`. The 2026-09-09 repair fixed `test_observe.py` only;
`test_session.py` has carried the same shape since, through `bffa6667` (HEAD).

### Tier

**n/a** — tooling. Reason: the defect is in the trial driver's own suite, not in jigc. Its
weight as a tooling item: one of 23 tests in this file is unexecuted under the only documented
runner, and argv 4 shows the regression it fences would pass green. It did not alter any number
this trial reports (see "What the lead does not carry").

### Pin

`UNPINNED: no test or fence asserts that a suite's `unittest.main()` guard is the file's last
statement, or that the count the script form runs equals the count the loader collects — the
2026-09-09 repair left a prose NOTE in test_observe.py, not a check.` Searched:
`crates/cli/tests/`, `dev/gate`, `.github/workflows/`, and every `test_*.py` under
`completions/trial-driver/` for a reference to the entry-point position; none found.

---

## T-7 · `run.py fork` (and `run.py observe`) take a sub-agent's transcript as the main session when no main file exists — voided as "began cold" on the fork door unless the sub-agent quotes the seed, scored outright on the observe door — **PARTIAL**

`tier:` n/a

**Kind:** tooling (the trial driver) · **Door:** `completions/trial-driver/run.py` (`do_fork`,
`_find`); `completions/trial-driver/driver/session.py` (`_find_transcript`);
`completions/trial-driver/driver/observe.py` (`_transcript_set`) · **Tier:** n/a (tooling, not the
jigc product) · read at `bffa6667`

### The claim

`run.py fork` hands `session._find_transcript`'s whole list — the main transcript and every
sub-agent's — to `observe`, so a list holding sub-agent files alone is not empty and is scored as
the conversation: the edge the `seed` repair fenced and `fork` did not.

### Verdict

**PARTIAL.** The mechanism is real and unfenced; the consequence the lead states — the row *is
scored* — holds on the `fork` door only under a second condition, and the scorer's own proposed
repro comes back **voided**.

What is confirmed:

- `do_fork` passes the finder's list straight through with no main-transcript predicate
  (`run.py` 328–331: `transcripts = session_mod._find_transcript(out, new) if new else []`, then
  `observe(out.name, log, transcripts, seed_expected=True, seed_marker=…)`).
- `_find_transcript` returns `hits[:1] + delegated` (`session.py` 194–196). With no file named
  for the forked id, the list is the sub-agents' files alone.
- `observe._transcript_set` takes **position 0 as the main session** (`observe.py` 519–520:
  `live[0] if live else None`). So with a sub-agent-only list the sub-agent's file becomes
  `main_transcript`: `transcript_missing` is `False`, that sub-agent's reads and commits are
  labelled `agent=""` (the worker's own), and the seed marker is searched **in the sub-agent's
  file**.
- `seed` carries the predicate (`session.py` 282–283, `if "subagents" not in p.parts`) and three
  tests; `do_fork` carries none and no test drives it with this shape
  (`grep do_fork|_find_transcript test_*.py` finds only a docstring mention in `test_cascade.py`
  and the `seed` class in `test_session.py`).

What the evidence does **not** carry:

- **The plain sub-agent-only out-dir is voided on the `fork` door, not scored** (case A below):
  the seed marker is not in the sub-agent's file, `seed_inherited` is `False`, and cascade row 4
  (`apparatus — the fork began cold`) outranks every scored row. The row and the stderr line are
  the ones the no-transcript control prints (case C); only the `fs` count differs (1 vs 0). The
  void's *reason* is wrong — nothing shows the fork began cold; the main transcript is missing —
  but the row is not a result.
- **It is scored only when the sub-agent's transcript itself contains the seed's first turn
  verbatim** (case B) — e.g. a delegation prompt that quotes the user's opening line. Then
  `seed_inherited=True`, the row is non-void, and the delegated read is counted as the worker's
  own. That is exactly the outcome `test_observe.py`
  `test_the_seed_marker_is_searched_in_the_main_transcript_only` exists to forbid ("a marker found
  only in a subagent must NOT count as inheritance"), and it stays green because it only ever
  builds a list with a main file in front.
- **No effect on this trial.** `fork` ran twice in RC24 (the two debriefs). Both out-dirs:
  forked id found in `stream.jsonl`, `_find_transcript` returns one file, a main transcript named
  for the forked id, zero sub-agent files. Every other RC24 out-dir with a transcript store has a
  main transcript first (`~/out/RC24-A-frozen-work/turn02`: main + 3 sub-agents;
  `~/out/RC24-env`: main + 1; the rest: main only). The shape "sub-agent files and no main file"
  was constructed, not observed; nothing here shows how a real run would produce it.

One step past the lead, driven because it is the door this trial's arms were scored through:

- **`run.py observe` has the same edge and no seed check in front of it.** `run._find` returns
  `([main] if main else []) + delegated` (`run.py` 68–69); with no non-sub-agent `.jsonl` the list
  is again sub-agents alone. Real argv, exit 0, the row is **scored**: the delegated read prints
  with no agent label, and where the sub-agent read nothing the row is
  `proceeded without reading` — the NEITHER verdict cascade row 10 says must not be asserted
  without the worker's own transcript. The control (no transcript at all) is voided at exit 1.
  On this door the lead's sentence holds unconditionally.

### Repro

```text
# setup — synthetic out-dirs only; nothing under ~/out is touched, no repository file is edited.
# `session.fork` (the one call that needs a container) is replaced in memory by a stub returning
# (out, 0); every line of run.do_fork after it is the tool's own code.
REPO=$(git rev-parse --show-toplevel)          # at bffa6667
W=$(mktemp -d "${TMPDIR:-/tmp}/t7.XXXXXX")
python3 - "$REPO" "$W" <<'EOF'
import argparse, json, pathlib, sys
from unittest import mock
REPO, W = (pathlib.Path(a) for a in sys.argv[1:3])
sys.path.insert(0, str(REPO / "completions" / "trial-driver"))
import run
from driver import session
MARK, NEW = "the seed's first turn", "99999999-aaaa-bbbb-cccc-000000000000"
row = lambda text: json.dumps({"type": "user", "message": {"role": "user", "content": text}}) + "\n"
read = json.dumps({"type": "assistant", "message": {"content": [{"type": "tool_use",
        "name": "Read", "input": {"file_path": "/work/docs/decisions/a-decision.md"}}]}}) + "\n"
fz = W / "frozen"; (fz / "project").mkdir(parents=True)
(fz / "session.jsonl").write_text(row(MARK))
(fz / "manifest.json").write_text(json.dumps({"session_id": "11111111-2222-3333-4444-555555555555",
    "session_sha": session._sha16((fz / "session.jsonl").read_bytes()), "seed_marker": MARK}))
(W / "prompt.txt").write_text("fork turn\n")
def outdir(name, main, sub):                       # what run-session.sh leaves
    out = W / name; (out / ".jigc" / "logs").mkdir(parents=True)
    (out / ".jigc" / "logs" / "invocations.jsonl").write_text(json.dumps({
        "timestamp": "2026-10-04T10:00:00Z", "argv": ["doc", "create", "adr", "--task", "t"],
        "exit_code": 0, "duration_ms": 5, "finding_codes": [], "output_bytes": 10,
        "binary_version": "1.0.0-rc.24", "error_code": None}) + "\n")
    (out / "stream.jsonl").write_text(json.dumps({"type": "system", "session_id": NEW}) + "\n")
    (out / "PROVENANCE.txt").write_text("image probe\njigc-sha 0000000000000000\nexit-code 0\n")
    store = out / ".session-transcript" / "projects" / "-work"; store.mkdir(parents=True)
    if main is not None:
        (store / f"{NEW}.jsonl").write_text(row(main) + row("fork turn"))
    if sub is not None:
        (store / NEW / "subagents").mkdir(parents=True)
        (store / NEW / "subagents" / "agent-a0b447359ae72c2bd.jsonl").write_text(row(sub) + read)
    return out
QUOTING = f"the user asked: {MARK}. Explore the docs."
for name, main, sub in [("A-sub-only", None, "a delegated prompt"),
                        ("B-sub-only-quoting", None, QUOTING),
                        ("C-none", None, None),
                        ("D-main-and-sub", MARK, QUOTING),
                        ("E-cold-main-and-sub", "a different opening", QUOTING)]:
    out = outdir(name, main, sub)
    args = argparse.Namespace(frozen=str(fz), out=str(out), prompt=str(W / "prompt.txt"),
                              corpus=str(W), tag="t", strict=False, gate=None)
    sys.stdout.flush()
    with mock.patch.object(session, "fork", lambda *a, **k: (out, 0)):   # the container call only
        rc = run.do_fork(args)
    sys.stdout.flush(); print(f"^ {name}: do_fork exit {rc}\n", flush=True)
EOF

# argv — the sibling door, the tool's own read-only path, unstubbed
python3 "$REPO/completions/trial-driver/run.py" observe "$W/A-sub-only"; echo "observe A exit=$?"
python3 "$REPO/completions/trial-driver/run.py" observe "$W/C-none";     echo "observe C exit=$?"

# observed — the fork read path (do_fork exit 0 in all five; table header elided)
A-sub-only           1     1     0    0   1  VOID apparatus — the fork began cold
  ! the fork did NOT inherit the seed — this run is void, not a result          (stderr)
B-sub-only-quoting     1     1     0    0   1       read around jigc, off the filesystem (heuristic — verify the reads)
C-none               1     1     0    0   0  VOID apparatus — the fork began cold
  ! the fork did NOT inherit the seed — this run is void, not a result          (stderr)
D-main-and-sub       1     1     0    0   1       read around jigc, off the filesystem (heuristic — verify the reads)
E-cold-main-and-sub     1     1     0    0   1  VOID apparatus — the fork began cold
  ! the fork did NOT inherit the seed — this run is void, not a result          (stderr)

# observed — the Observation behind each row (same arguments do_fork passes)
A  transcript_missing=False seed_inherited=False filesystem=1 read-agents=['']
B  transcript_missing=False seed_inherited=True  filesystem=1 read-agents=['']
C  transcript_missing=True  seed_inherited=False filesystem=0 read-agents=[]
D  transcript_missing=False seed_inherited=True  filesystem=1 read-agents=['agent-a0b447359ae72c2bd']
E  transcript_missing=False seed_inherited=False filesystem=1 read-agents=['agent-a0b447359ae72c2bd']

# observed — run.py observe
A-sub-only           1     1     0    0   1       read around jigc, off the filesystem (heuristic — verify the reads)
  fs? DOC  [Read] /work/docs/decisions/a-decision.md
observe A exit=0
  ! no transcript — the FILESYSTEM channel is unmeasured for this session       (stderr)
C-none               1     1     0    0   0  VOID unmeasured — no transcript, so the filesystem channel is blind
observe C exit=1
# and with the sub-agent's Read removed from A's one transcript:
F-sub-only-no-reads     1     1     0    0   0       proceeded without reading
observe F exit=0

# after — B against E is the datum: the same quoting sub-agent, and the only difference is that
# E has a (cold) main transcript in front of it. E is voided as cold; B, with NO main transcript,
# is scored as warm. A against C is the refuting datum for the unconditional claim: sub-agent-only
# and nothing-at-all print the same void row on the fork door.
# Suites at the same commit: test_session.py `Ran 22 tests … OK` (exit 0), test_observe.py
# `Ran 63 tests … OK` (exit 0) — green over all of it. `git status` unchanged by the probe.
```

Read-only check of the saved evidence (no file under `~/out` written):

```text
~/out/RC24-B-debrief   forked id found; _find_transcript -> ['main'];  first file is named for the forked id
~/out/RC24-A-debrief   forked id found; _find_transcript -> ['main'];  first file is named for the forked id
run._find, first element / sub-agent count:
~/out/RC24-C main/0 · RC24-B-frozen-work/turn01 main/0 · turn02 main/0 · RC24-A-frozen-work/turn01 main/0
· turn02 main/3 · RC24-B-debrief main/0 · RC24-A-debrief main/0 · RC24-env main/1 · RC24-walk no store
```

### The contract

- `observe._transcript_set`'s docstring: the argument is "a sequence whose **first element is the
  main session** and whose rest are subagent transcripts. The order is the contract:
  `seed_marker` is searched in the main file only, because a subagent never inherits the seed and
  searching all of them would make `seed_inherited` report a fork as warm that began cold."
  Neither finder upholds the "first element is the main session" half when there is no main file.
- `bffa6667`'s own message, for `seed`: "The predicate is the finder's own, not an emptiness check:
  a turn that left subagent transcripts and no main one yields a non-empty list, and staging a
  subagent's file as the conversation is the silent failure the refusal exists for." The same
  sentence applies to both read doors and was applied to neither.
- `cascade.py` row 10's comment: a NEITHER verdict "requires BOTH channels to have been looked
  at" — on the `observe` door a sub-agent-only store reaches row 11 without the worker's own
  transcript having been read.

### Tier

**n/a** — a defect of the trial driver, not of the jigc product. Within tooling: latent. It moved
no figure in RC24 (every scored out-dir and both forks have a main transcript in first position),
and on the `fork` door it needs two things at once — a lost main transcript and a sub-agent whose
transcript quotes the seed's first turn.

### Pin

**UNPINNED.** No test in `completions/trial-driver/test_*.py` hands `do_fork`, `run._find` or
`observe` a sub-agent-only transcript set; the three `seed` tests added at `bffa6667`
(`test_session.py` `ASeedCarriesTheMainTranscriptBetweenTurns`) fence the turn loop only, and
`test_observe.py` `TheReaderWalksSubagentTranscripts` always builds a main file. Cases A/B/E above
(fork) and A/F (observe) are the candidate pins.

### Notes

- The lead's wording "hands the whole list to `observe`" is not itself the defect — `observe`
  wants the whole list, main first, so delegated reads are counted. The defect is that neither
  finder guarantees position 0 is the main session, and `observe` cannot tell.
- On the `fork` door the default outcome is a void under the wrong name ("began cold" for what is
  a missing main transcript). That is the same row a fork with no transcript at all gets, so it is
  an existing imprecision of row 4's ordering rather than something this edge adds.
- Reachability of the precondition was not driven: producing a real out-dir with sub-agent files
  and no main file needs a container run, which is outside a read-only verification.

---

## T-8 · `observe`'s `recs` column counts adapter-hook invocations as session records: arm (c)'s 13 = 11 worker-typed + the SessionStart hook's `start` (#3) + the pre-commit hook's `validate --format json` (#14) — **CONFIRMED**

`tier:` n/a

**Kind:** tooling (trial driver) · **Door:** `completions/trial-driver/driver/observe.py`
(`observe()`, the `session_start` split and `records=len(records)`), rendered by
`completions/trial-driver/run.py` (`_row`, the `recs` column) · **Tier:** n/a

### The claim

`run.py observe` reports `recs = 13` for arm (c). Those 13 are ordinals 3–15 of
`~/out/RC24-C/.jigc/logs/invocations.jsonl`. Two of them were not typed by the worker:
ordinal 3 is the SessionStart hook's `jigc start`, ordinal 14 is the pre-commit hook's
`jigc validate --format json`. The worker typed 11.

### Verdict

**CONFIRMED** — every part of the claim holds on the saved evidence, with one bound the lead does
not state: **no scored channel moves.** Neither hook argv matches the VERB, VERB-ADJACENT or
authoring predicate, so `wrote 4 · VERB 1 · adj 1 · fs 0` and the outcome row (`read back through
the fence's verb`) are the worker's own. The over-count is confined to the `recs` column and to
the one cascade row that reads it (row 5, `records == 0`).

### Repro

```
setup
  out-dir        ~/out/RC24-C   (read-only; nothing written under it)
  PROVENANCE     session-start 2026-10-03T22:19:39Z · exit-code 0
  log            ~/out/RC24-C/.jigc/logs/invocations.jsonl — 15 records
  transcripts    one main transcript, no subagents/ directory
  adapter        $REPO/.claude/settings.json → hooks.SessionStart[0].hooks[0].command = "jigc start"
                 $REPO/.git/hooks/pre-commit → report="$("$jigc" validate --format json 2>/dev/null)"

argv 1  (the tooling's own read-only path)
  cd completions/trial-driver && python3 -B run.py observe ~/out/RC24-C

observed 1   exit 0
  session           recs wrote  VERB  adj  fs  outcome
  RC24-C              13     4     1    1   0       read back through the fence's verb
    note: 2 record(s) predate this session (plant/adoption) and are NOT scored — they would
          have added 0 VERB and 0 adjacent
  (identical to rubric/observe-c.txt apart from the provenance line's wording)

argv 2  (the log, with ordinals)
   1 21:56:21Z 0 validate --format json                      <- pre-session, left out
   2 21:56:21Z 0 config get invocation-log                   <- pre-session, left out
   3 22:19:40Z 0 start                                        <- HOOK (SessionStart)
   4 22:19:57Z 0 start --workflow architecture-documentation "<intent>" --format json
   5 22:20:00Z 0 doc schema arch-doc --format json
   6 22:20:00Z 0 doc list --format json
   7 22:20:03Z 0 doc author --help
   8 22:20:52Z 0 doc create arch-doc --title … --task <id> --format json
   9 22:21:12Z 0 doc author arch-doc --from-file - --task <id> --format json
  10 22:21:15Z 0 doc show arch-doc:sample-rollup-pipeline --task <id>
  11 22:21:17Z 0 doc set-field commit:<id>#type --value docs --task <id> --format json
  12 22:21:20Z 0 doc set-slot commit:<id>#summary --from-file - --task <id> --format json
  13 22:21:22Z 0 task validate <id> --format json
  14 22:21:24Z 0 validate --format json                      <- HOOK (pre-commit)
  15 22:21:24Z 0 task finalize <id> --format json
  cutoff 22:19:39Z -> scored = ordinals 3..15 = 13

argv 3  (every Bash tool_use in the main transcript whose command runs `jigc`)
  11 commands, at 22:19:57 · 22:20:00 · 22:20:00 · 22:20:03 · 22:20:52 · 22:21:12 · 22:21:15 ·
  22:21:17 · 22:21:20 · 22:21:22 · 22:21:24 — one-to-one with ordinals 4–13 and 15.
  None is a bare `jigc start`; none is `jigc validate` (only `jigc task validate`).

observed, ordinal 3 is the SessionStart hook
  transcript line 3 is an attachment: type hook_success · hookName SessionStart:startup ·
  hookEvent SessionStart · command "jigc start" · exitCode 0, stamped 22:19:40.721Z — before
  the first user turn (22:19:40.744Z) and before the worker's first act (22:19:43Z).
  stream.jsonl lines 1–2: hook_started / hook_response for SessionStart:startup, outcome success.

observed, ordinal 14 is the pre-commit hook
  argv is byte-for-byte the hook script's own call; its stamp is the committer time of the
  finalize commit (d107e87, 22:21:24Z); it sits before finalize's own record, which is written
  when finalize exits, i.e. after the `git commit` that ran the hook.
  Control inside the same log: ordinal 1 is the same argv at 21:56:21Z, the committer time of
  the adoption commit (fbbf746). The adoption arm (completions/trial-driver/arms/adopt.sh) types
  `jigc setup`, `jigc config set invocation-log true`, a hand `git commit`, and
  `jigc config get invocation-log` — never `jigc validate`. The record can only be the hook's.

arithmetic
  recs 13  −  worker-typed 11  =  2  =  {ordinal 3, ordinal 14}

after
  nothing modified under ~/out; the repository's working tree unchanged
  (`git status --porcelain` shows only the pre-existing untracked directory).
```

### The contract

There is no line in the driver that defines `recs` as "commands the worker typed" — the column
is `Observation.records = len(records)`, the log's record count at or after `session-start`, and
it reproduces the 1.0.0-gate record's table (`RECORD` in `run.py`) on exactly that definition.
What is contradicted is the **attribution the tooling and the protocol put on that number**:

- `observe.py`, `Observation.pre_session_records`: the left-out records are *"written by the
  adoption arm or by a plant, never by the worker"* — the split is drawn as rig versus worker.
- `test_observe.py`, the pre-session split's own test:
  `self.assertEqual(o.records, 1, "one record is the worker's")`.
- `protocol.md` §8.2: *"For turn 2 that line is counting turn 1's records, which are the
  worker's."*

On an adopted corpus that reading is off by the adapter's own invocations: the timestamp cutoff
separates *before the session* from *during the session*, and during the session two parties
drive the binary — the worker, and the hooks `jigc setup` installed. `protocol.md` itself knows
the first of them fires (§11: *"the SessionStart hook fires again"* on every resumed turn) and does
not carry that into how `recs` is read.

### What it does and does not change

- **Does not change** `wrote`, `VERB`, `adj`, `fs` or the outcome for arm (c): `start` and
  `validate --format json` match none of `channels.is_verb`, `channels.is_adjacent`,
  `observe._is_authoring`. Both exited 0, so `nonzero_exits` is unmoved as well.
- **Does reach one cascade row.** Row 5, `apparatus — the session did nothing`, is
  `o.records == 0`. With the SessionStart hook writing a record at second one of every session
  on an adopted corpus, that predicate cannot hold there. Driven read-only: a scratch log holding
  only ordinal 3, graded with the same `session_start`, gives `records 1` and falls to row 6,
  `unmeasured — no authoring occasion existed`. Both rows are void, so the *kind* is unchanged;
  the row label is not.
- **Is not peculiar to arm (c).** The same two argv shapes sit after the cutoff in every turn
  out-dir of arms (a) and (b) — a bare `start` as the first scored record of each turn, and a
  `validate --format json` in the same second as each commit (five in arm (a) turn 1). No
  transcript in any arm types a `jigc validate` without `task`. (Arm (a) turn 2 additionally
  carries one bare `start` the worker did type; a bare `start` is therefore not hook-origin by
  argv alone — the transcript decides.) The per-turn worker-typed totals for (a) and (b) were
  counted by a regex over the transcripts and are approximate; only arm (c)'s 11 was listed and
  matched by hand.

### Tier

**n/a** — tooling. The product did what its adapter says: the hooks ran and the invocation log
recorded them, exactly.

### Pin

**UNPINNED.** `test_observe.py`'s pre-session split tests pin the *timestamp* cut (records older
than `session-start` are left out and reported) and name what remains "the worker's"; no test
feeds `observe` a log holding a hook-origin record after the cutoff, and nothing in
`driver/observe.py` or `driver/channels.py` distinguishes one. The `recs` figures of the archived
record (`RECORD`, `test_observe.py`'s 81 and 13) are pinned as raw log counts, so any change to
what `recs` means would have to keep those reproducing.

### Notes for whoever reads the trial record

Read arm (c)'s `recs 13` as **13 invocations during the session, 11 of them the worker's**. The
read-back row needs no correction.

---

## T-9 · `trailer-rows.py` pairs same-second doors and commits by a tie-break (commit depth + log-file order), not by time; arm (a)'s three `add-task` rows are right by that rule, and it mis-pairs silently when log order or depth stops being the causal order — **CONFIRMED**

`tier:` n/a

**Kind:** tooling (this trial's protocol helper) · **Door:** `protocol/tools/trailer-rows.py`
· **Tier:** n/a (tooling, not the jigc product)

### The claim

`tools/trailer-rows.py` joins doors to commits by time; arm (a)'s three `add-task` commits share
one second, so its pairing there is right only by whatever breaks the tie.

### Verdict

**CONFIRMED** — every part of the claim, and the scorer's swapped-order fixture mis-pairs as
predicted. **Bounded:** arm (a)'s rows are right, the tie-break is a deliberate one that holds for
the shape arm (a) has, and no trailer verdict or tally in this trial depends on it.

What carries the claim:

- **The join is by time.** A commit fits a door when `start <= ct <= end + 1`, with
  `end` = the record's `timestamp` (the CLI stamps the record when it is written, at the end of
  the run) and `start = end - (ceil(duration_ms / 1000) + 1)` (lines 104–109, 128).
- **Arm (a)'s three commits share one second, and so do their three records.**
  `bc3ab92`, `dafdc52`, `29ea968` all carry committer time `1791066588`; log records #35, #37,
  #39 are all stamped `2026-10-03T22:29:48Z` with 105 / 102 / 100 ms. Each door's window is
  therefore the same four seconds, `[…586, …589]`, and **each of the three commits fits each of
  the three doors**: time attributes nothing among them.
- **What decides is two orderings that are not time** (the tool says so in its comments, lines
  119–120 and 129–131): commits are walked sorted by `(committer time, depth, sha)` where depth
  is `git rev-list --count <sha>`; each takes the first fitting door **in log-file order** that
  has no commit yet, else the first fitting door.
- **Arm (a) is right, checked without the tool.** History order is `bc3ab92` (depth 12, subject
  `…record task:cap-the-store-at-1000…`) → `dafdc52` (13, `…reject-a-sample-whose-series…`) →
  `29ea968` (14, `…reject-a-wire-line-longer…`); log order is #35 `Cap the store…`, #37 `Reject a
  sample…`, #39 `Reject a wire line…`. Each row's commit subject names the slug of its door's
  title. The session issued the three calls as one Bash command of three newline-separated
  invocations, on one linear branch (15 objects in `rev-list --all --reflog`, one worktree), so
  log order and history order are the same causal order. A re-run of the tool over
  `~/out/RC24-A-frozen-work/turn02 --since 2026-10-03T22:24:01Z` is byte-identical to
  `rubric/trailer-a.txt` (exit 0).
- **The tie-break fails when either ordering stops being the causal one** (fixtures below):
  swap the two same-second log records and both rows name the wrong door (F2); put the two
  commits at equal depth on sibling branches and the pairing follows the sha (F3-lt right,
  F3-gt wrong, the only difference a nonce in a file). In every case the tool exits 0 and
  prints no line saying a row's door was chosen among several that fit.

What the lead does **not** carry, stated so it is not over-read:

- **Nothing in this trial's trailer result moves.** A row's verdict (`exact` / `NONE` /
  `variant` / `DUPLICATE`) is read off the commit, never off the door; a mis-pairing between two
  doors changes the `door` column only. Arm (a): six doors, six commits, six `exact`. Arms (b)
  and (c) have no same-second pair (two and one row).
- **The tie-break is not accidental.** "Whatever breaks the tie" is a rule the author wrote
  down; its unstated precondition is sequential doors on a linear history, which is what a
  single agent session produces and what all three arms are.
- F2's shape (log order against history order) needs two jigc processes overlapping; F3's needs
  same-second door commits on sibling branches. Neither occurred in this trial; whether a
  fan-out's sub-agents can produce either on rc.24 was **not driven** here.

Two things one step past the lead, same root, both observed:

- **The printed row order inside a tied second is sha order, not history order.** Pairing sorts
  on depth, printing re-sorts on `(ct, sha[:8], …)` (line 150). `rubric/trailer-a.txt` prints
  the three rows as `29ea968e`, `bc3ab92f`, `dafdc527` — wire-line, cap, sample — while history
  and the log are cap, sample, wire-line. The rows are right; their order is not chronology.
- **The first-unclaimed-door rule mis-pairs without any equal timestamps being unusual** when a
  door's commit count is not one: a two-commit door followed in the same window by a one-commit
  door hands the second commit to the neighbour and the neighbour's commit back (F4, the `NONE`
  lands on the wrong door); an exit-0 door record that made no commit takes the next door's
  commit, and the tool then prints `door with NO commit matched` **for the door that did
  commit** (F5). Whether a real rc.24 door record of either kind exists was not driven; these
  are facts about the helper over a hand-built log.

### Repro

```
# setup — fixtures.sh <path to protocol/tools/trailer-rows.py>; every repo under a fresh
# `mktemp -d` root, nothing removed; the tool itself only reads (git rev-list / git show)
set -eu
TOOL=$1
ROOT=$(mktemp -d "${TMPDIR:-/tmp}/t9.XXXXXX")
export GIT_CONFIG_GLOBAL=/dev/null GIT_CONFIG_NOSYSTEM=1
export GIT_AUTHOR_NAME=fixture GIT_AUTHOR_EMAIL=fixture@example.invalid
export GIT_COMMITTER_NAME=fixture GIT_COMMITTER_EMAIL=fixture@example.invalid
BASE='2025-12-31T23:00:00Z'; SINCE='2026-01-01T00:00:00Z'; TIE='2026-01-01T00:00:10Z'
EXACT='Co-Authored-By: Claude <noreply@anthropic.com>'
repo() {    # one pre-session commit, an empty log, session-start = $SINCE
  d="$ROOT/$1"; mkdir -p "$d/.jigc/logs"; git -C "$d" init -q -b main
  printf '.jigc/\nPROVENANCE.txt\n' > "$d/.gitignore"
  printf 'session-start %s\n' "$SINCE" > "$d/PROVENANCE.txt"
  git -C "$d" add .gitignore
  GIT_AUTHOR_DATE=$BASE GIT_COMMITTER_DATE=$BASE git -C "$d" commit -q -m 'chore: base'
  printf '%s' "$d"
}
commit() {  # commit <dir> <slug> <trailer|-> [content]   — always in the tied second
  printf '%s\n' "${4:-$2}" > "$1/$2.txt"; git -C "$1" add "$2.txt"
  if [ "$3" = - ]; then msg="chore(milestone): record task:$2"
  else msg="chore(milestone): record task:$2

$3"; fi
  GIT_AUTHOR_DATE=$TIE GIT_COMMITTER_DATE=$TIE git -C "$1" commit -q -m "$msg"
}
door() {    # an exit-0 record stamped in the tied second, 100 ms
  printf '{"timestamp":"%s","argv":%s,"exit_code":0,"duration_ms":100}\n' "$TIE" "$2" \
    >> "$1/.jigc/logs/invocations.jsonl"
}
A='["milestone","add-task","m","Alpha"]'; B='["milestone","add-task","m","Beta"]'

# F1 control — linear: alpha (no trailer) then beta (exact); log order Alpha, Beta
d=$(repo f1); commit "$d" alpha -; commit "$d" beta "$EXACT"; door "$d" "$A"; door "$d" "$B"
python3 "$TOOL" "$d"
# F2 swapped — the same two commits; the two same-second records in the other order
d=$(repo f2); commit "$d" alpha -; commit "$d" beta "$EXACT"; door "$d" "$B"; door "$d" "$A"
python3 "$TOOL" "$d"
# F3 equal depth — alpha on branch x, beta on branch y, both off base; log order Alpha, Beta;
# alpha's file content carries a nonce, walked until sha(alpha) sorts below / above sha(beta)
for want in lt gt; do n=0; while :; do
    d=$(repo "f3-$want-$n")
    git -C "$d" checkout -q -b x; commit "$d" alpha - "nonce $n"
    git -C "$d" checkout -q main; git -C "$d" checkout -q -b y; commit "$d" beta "$EXACT"
    xa=$(git -C "$d" rev-parse x); yb=$(git -C "$d" rev-parse y)
    if [ "$want" = lt ] && [ "$xa" \< "$yb" ]; then break; fi
    if [ "$want" = gt ] && [ "$xa" \> "$yb" ]; then break; fi
    n=$((n+1)); done
  door "$d" "$A"; door "$d" "$B"; python3 "$TOOL" "$d"; done
# F4 — Alpha makes two commits (alpha-1 exact, alpha-2 no trailer), Beta one; linear
d=$(repo f4); commit "$d" alpha-1 "$EXACT"; commit "$d" alpha-2 -; commit "$d" beta "$EXACT"
door "$d" "$A"; door "$d" "$B"; python3 "$TOOL" "$d"
# F5 — an exit-0 door record with no commit of its own, then Beta's one commit
d=$(repo f5); commit "$d" beta "$EXACT"
door "$d" '["task","discard","alpha"]'; door "$d" "$B"; python3 "$TOOL" "$d"

# argv (the real arm, read-only)
python3 protocol/tools/trailer-rows.py ~/out/RC24-A-frozen-work/turn02 --since 2026-10-03T22:24:01Z

# observed — the real arm: exit 0, byte-identical to rubric/trailer-a.txt; its tied rows, as printed
29ea968e  exact  HEAD  milestone add-task … Reject a wire line long  ·  …record task:reject-a-wire-line…
bc3ab92f  exact  HEAD  milestone add-task … Cap the store at 1000 d  ·  …record task:cap-the-store-at-1…
dafdc527  exact  HEAD  milestone add-task … Reject a sample whose s  ·  …record task:reject-a-sample-wh…
#   git: bc3ab92 (depth 12) -> dafdc52 (13) -> 29ea968 (14), all ct=1791066588
#   log: #35 Cap…, #37 Reject a sample…, #39 Reject a wire line…, all 2026-10-03T22:29:48Z

# observed — fixtures: every run exit 0
F1  874662dd  NONE   HEAD    milestone add-task m Alpha  ·  …record task:alpha  ·  (none)
    bf0cc9e7  exact  HEAD    milestone add-task m Beta   ·  …record task:beta   ·  Co-Authored-By: Claude <noreply@anthropic.com>
F2  874662dd  NONE   HEAD    milestone add-task m Beta   ·  …record task:alpha  ·  (none)            <- wrong door
    bf0cc9e7  exact  HEAD    milestone add-task m Alpha  ·  …record task:beta   ·  Co-Authored-By: …  <- wrong door
F3-lt (nonce 8)
    46af1ff3  NONE   reflog  milestone add-task m Alpha  ·  …record task:alpha  ·  (none)
    50cb18e7  exact  HEAD    milestone add-task m Beta   ·  …record task:beta   ·  Co-Authored-By: …
F3-gt (nonce 0)
    50cb18e7  exact  HEAD    milestone add-task m Alpha  ·  …record task:beta   ·  Co-Authored-By: …  <- wrong door
    d09e8a36  NONE   reflog  milestone add-task m Beta   ·  …record task:alpha  ·  (none)            <- wrong door
F4  47fd9877  exact  HEAD    milestone add-task m Alpha  ·  …record task:beta     <- wrong door, printed first, committed last
    7134a59b  exact  HEAD    milestone add-task m Alpha  ·  …record task:alpha-1
    d717895e  NONE   HEAD    milestone add-task m Beta   ·  …record task:alpha-2  <- wrong door
F5  50cb18e7  exact  HEAD    task discard alpha          ·  …record task:beta     <- wrong door
    door with NO commit matched: 2026-01-01T00:00:10Z  milestone add-task m Beta  <- the door that did commit
#   tallies: F1 = F2 = F3-lt = F3-gt = `NONE 1, exact 1`; F4 `NONE 1, exact 2`; F5 `exact 1`
#   no run prints a line marking a row whose door was one of several that fit

# after
nothing under ~/out or the repository was written; the fixtures stay under their mktemp root
```

`F3`'s `reflog` in the `at` column is a commit on a branch that is not `HEAD`'s (the column is
`HEAD` ancestry or not), not a reflog-only object.

### The contract

- `protocol/protocol.md` §6, item 3: the trailer check identifies jigc's commits by "the join of
  the two by time, each match then checked against the raw git acts in the transcript". The
  check it prescribes guards one mis-attribution — a raw `git commit` inside a door's window —
  and none between two doors.
- The tool's own docstring: "a commit is attributed to a door when its committer time falls
  inside that record's run" and "The join is an aid, not a verdict."
- The tool's own comment on the tie (lines 119–120): "Depth breaks a same-second tie in history
  order, so two doors run back to back are each given their own commit rather than both given
  the first." That is the rule arm (a) is right by. Its precondition — back-to-back doors, one
  linear history, one commit per door — is not checked and not reported when it does not hold.
- §6.2's pre-run drive of the helper: "five known commits (three exact, one variant, one none,
  one of them reflog-only)". No same-second shape was in it.

### Tier

**n/a** — a trial helper, not the jigc product. Consequence for the record: the `door` column of
a `trailer-rows.txt` row is evidence only where one door fits the commit; where several fit
(arm (a)'s three `add-task` rows) the row is confirmed by the commit subject naming the door's
task slug, which it does in all three. The per-commit trailer verdicts and `tally: exact 6` do
not rest on the pairing.

### Pin

**UNPINNED** — `trailer-rows.py` exists only in this trial's `protocol/tools/` (no tracked copy
in the repository, `git ls-files` has no match), it has no test suite, and the pre-run drive
recorded in §6.2 had no tied second. The fixtures above are the only run of the shape.

---

## T-10 · trailer-rows.py cannot see a commit made on a fan-out worktree's detached HEAD once the worktree is removed; the object survives only as an unreachable commit that `git cat-file --batch-all-objects` still sees — **CONFIRMED**

`tier:` n/a

**Lead (tooling):** `trailer-rows.py` sees commits reachable from a ref or the reflog; a commit made on a sub-task worktree's detached `HEAD` loses its reflog when the worktree is removed and would be invisible to it. The by-hand check used `git cat-file --batch-all-objects` instead.
**Source:** protocol §6.1 item 2 against arm (a)'s removed worktrees. **Door:** `protocol/tools/trailer-rows.py`; `protocol/protocol.md` §6.1.
**Binary for the rig cells:** `~/.local/bin/jigc`, `jigc --version` → `jigc 1.0.0-rc.24` (asserted in each rig). git 2.54. The helper under test is `$TRIAL/protocol/tools/trailer-rows.py`, unmodified, run as the runbook §8 runs it (`python3 trailer-rows.py <out-dir> --since <stamp>`).

### Verdict

**CONFIRMED** — with one correction to the proposed repro and one bound on the consequence.

1. **The helper's commit set is `git rev-list --all --reflog`, nothing wider.** `trailer-rows.py:113` — the one enumeration of commits in the file. Its docstring (lines 15–16) and protocol §6.1 item 2 say the same thing in words: *"reachable from any ref or from the reflog"*.
2. **While a fan-out worktree exists, a commit on its detached `HEAD` is in that set — through the worktree's own `HEAD` and reflog only.** Before removal the only files naming the commit are `.git/worktrees/<sub-task>/HEAD` and `.git/worktrees/<sub-task>/logs/HEAD`; the helper lists it (`not-jigc  reflog`).
3. **Removing the worktree deletes both, and the commit leaves the set.** After `jigc milestone finalize <id>` (exit 0) and, separately, after `jigc milestone discard <id>` (exit 0), `.git/worktrees/` is gone, no reflog names the commit, `git rev-list --all --reflog` does not list it, and the helper prints a row set without it and a clean tally (`tally: exact 4` / `exact 5`). The object is still in the database (`git cat-file -t` → `commit`; `git fsck --unreachable` → `unreachable commit …`).
4. **The by-hand method sees it.** `git cat-file --batch-all-objects --batch-check` counts one commit more than `rev-list --all --reflog` (7 against 6; 8 against 7), and the extra one is the worktree commit.

**Correction to the proposed repro.** The step as handed to this verifier reads *"discard the sub-task"*. On rc.24 `jigc task discard <sub-task>` (exit 0) lands its record commit and **leaves the worktree in place**, detached at the raw commit — the helper still lists the commit afterwards (cell 1a). The falsifying datum for that wording: `git worktree list` after the discard still shows `.jigc/worktrees/cap-the-store  b0db829 (detached HEAD)`, and the helper's `b0db829e  not-jigc  reflog` row is unchanged. What removes the worktree is the milestone boundary (`milestone finalize`) or `milestone discard`; the score's own table wording, *"discard the worktree"*, is the one that carries.

**Bound on the consequence — no row of this trial was affected.** In all five saved out-dirs the two counts agree and `git fsck --unreachable` reports no commit:

| out-dir | `rev-list --all --reflog` | commit objects (`--batch-all-objects`) | unreachable commits |
|---|---|---|---|
| `~/out/RC24-C` | 10 | 10 | 0 |
| `~/out/RC24-B-frozen-work/turn01` | 10 | 10 | 0 |
| `~/out/RC24-B-frozen-work/turn02` | 11 | 11 | 0 |
| `~/out/RC24-A-frozen-work/turn01` | 14 | 14 | 0 |
| `~/out/RC24-A-frozen-work/turn02` | 15 | 15 | 0 |

So `rubric/trailer-a.txt`'s six rows are the whole of arm (a)'s in-session commits, and the saved `raw-git-a.txt` (`0 raw git history act(s) across 4 transcript(s)`) agrees. The blind spot is latent in the tool, not a wrong figure in the record.

**What the blind spot can hide, as far as it was driven.** In both rig cells every commit a jigc door made was on `main` and was listed; the hidden commit was a **raw** one, i.e. a missing `not-jigc` row, on a run whose helper output reads as fully clean. Whether a *jigc-made* commit can sit on a worktree's detached `HEAD` (the `finalize.fan-out.squash: false` chain) was **not driven**: the one attempt stopped at `finalize.render-io` (exit 1, no sub-task commit doc) and was not taken further; protocol §12 lists that configuration as unmeasured. The protocol's second channel for a raw commit, `tools/raw-git-acts.py` over the transcripts, does not depend on the object set; it was not driven here.

### Repro

```text
# ── cell 0 — the door, read ───────────────────────────────────────────────────
$TRIAL/protocol/tools/trailer-rows.py:113
    for sha in git(out, "rev-list", "--all", "--reflog").split():
# no other enumeration of commits in the file; `cat-file`, `fsck` do not appear

# ── cell 1 — rig, `task discard` then `milestone discard` ─────────────────────
# setup
rig=$(dev/jigc-rig fresh --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit
$JIGC --version                                   # jigc 1.0.0-rc.24
$JIGC config set invocation-log true              # exit 0
SINCE=$(date -u +%Y-%m-%dT%H:%M:%SZ)
$JIGC milestone create "Bound inputs"             # exit 0, record commit 68a3766
$JIGC milestone add-task bound-inputs "Cap the store"        # exit 0, 902de3a
$JIGC milestone add-task bound-inputs "Reject long names"    # exit 0, 125e2d6
$JIGC milestone provision bound-inputs            # exit 0, 2 worktrees, both `(detached HEAD)` at c3b4d84
WT=$REPO/.jigc/worktrees/cap-the-store
printf 'export const CAP = 1000;\n' > $WT/cap.ts
git -C $WT add cap.ts
git -C $WT commit -q -m "raw: cap the store (made inside the sub-task worktree)"   # exit 0 → W = b0db829e

# argv — BEFORE (control: the helper finds the plant)
$ python3 $TRIAL/protocol/tools/trailer-rows.py $REPO --since $SINCE              # exit 0
  session since … · 3 committing-door record(s) at exit 0 · 4 commit(s) created since
  …three `exact  HEAD` rows…
  b0db829e  not-jigc   reflog  -  ·  raw: cap the store (made inside the sub-task wor  ·  (none)
  tally: exact 3, not-jigc 1
$ git rev-list --all --reflog | grep -c $W        # 1      (rev-list 6 · all-objects 6)

# 1a — the step as proposed: discard the SUB-TASK
$ $JIGC task discard cap-the-store --format json  # exit 0, {"commit":"8633666","op":"task-discard",…}
$ git worktree list                               # cap-the-store  b0db829 (detached HEAD)  — STILL THERE
$ python3 …/trailer-rows.py $REPO --since $SINCE  # exit 0
  b0db829e  not-jigc   reflog  -  ·  raw: cap the store …          ← still listed
  tally: exact 4, not-jigc 1

# 1b — remove the worktree: discard the MILESTONE
$ $JIGC milestone discard bound-inputs --format json   # exit 0, "discarded milestone:bound-inputs (2 sub-task(s); workbench removed)"
$ git worktree list                               # main checkout only; .git/worktrees: No such file or directory
$ python3 …/trailer-rows.py $REPO --since $SINCE  # exit 0
  session since … · 5 committing-door record(s) at exit 0 · 5 commit(s) created since
  …five `exact  HEAD` rows, no b0db829e…
  tally: exact 5

# after
$ git rev-list --all --reflog | grep -c $W        # 0
$ git cat-file -t $W                              # commit
$ git rev-list --all --reflog | wc -l             # 7
$ git cat-file --batch-all-objects --batch-check | grep -c ' commit '   # 8
$ git fsck --unreachable | grep commit            # unreachable commit b0db829e…

# ── cell 2 — fresh rig, the path arm (a) took: `milestone finalize` ───────────
# setup: as cell 1 through `provision`; then, with 4 s of quiet either side so the
# raw commit falls in no door's time window —
git -C $WT1 add cap.ts; git -C $WT1 commit -q -m "raw: cap the store (made inside the sub-task worktree)"   # W = 6fa98072
git -C $WT2 add names.ts                          # the second sub-task's work, staged, not committed

# BEFORE
$ python3 …/trailer-rows.py $REPO --since $SINCE  # exit 0
  6fa98072  not-jigc   reflog  -  ·  raw: cap the store (made inside the sub-task wor  ·  (none)
  tally: exact 3, not-jigc 1
$ grep -rl $W .git/logs .git/worktrees            # .git/worktrees/cap-the-store/HEAD
                                                  # .git/worktrees/cap-the-store/logs/HEAD   — nowhere else

# argv
$ $JIGC milestone finalize bound-inputs           # exit 0
  finalized 8b82d15 — Finalize milestone bound-inputs (2 sub-tasks)
    added names.ts … 3 files committed
    sub-tasks: cap-the-store: nothing staged · reject-long-names: 1 code file

# AFTER
$ git worktree list                               # main checkout only; .git/worktrees gone
$ python3 …/trailer-rows.py $REPO --since $SINCE  # exit 0
  session since … · 4 committing-door record(s) at exit 0 · 4 commit(s) created since
  f6705d8e  exact  HEAD  milestone create Bound inputs  · …
  7fd42371  exact  HEAD  milestone add-task bound-inputs Reject long names  · …
  e3c4e8c5  exact  HEAD  milestone add-task bound-inputs Cap the store  · …
  8b82d15a  exact  HEAD  milestone finalize bound-inputs  ·  Finalize milestone bound-inputs (2 sub-tasks)  · …
  tally: exact 4                                  ← no row for 6fa98072, no line saying anything was skipped
$ git rev-list --all --reflog | grep -c $W        # 0      (rev-list 6 · all-objects 7)
$ grep -rl $W .git/logs                           # (none, exit 1)
$ git cat-file -t $W                              # commit
$ git fsck --unreachable | grep commit            # unreachable commit 6fa98072…
$ the by-hand method — every commit object with committer time ≥ SINCE, from --batch-all-objects
  6fa9807 raw: cap the store (made inside the sub-task worktree)        ← the fifth, which the helper lacks
  7fd4237 · 8b82d15 · e3c4e8c · f6705d8

# ── cell 3 — the trial's own out-dirs, read-only ──────────────────────────────
$ for d in ~/out/RC24-C ~/out/RC24-B-frozen-work/turn0{1,2} ~/out/RC24-A-frozen-work/turn0{1,2}:
    git -C $d rev-list --all --reflog | wc -l
    git -C $d cat-file --batch-all-objects --batch-check | grep -c ' commit '
    git -C $d fsck --unreachable | grep -c 'unreachable commit'
  → 10/10/0 · 10/10/0 · 11/11/0 · 14/14/0 · 15/15/0     (table above)
```

Nothing under `~/out` or `~/ideas` was written: the only commands run there were `rev-list`, `cat-file`, `fsck --unreachable` (no `--lost-found`), `reflog`, `for-each-ref`, `worktree list`, and a read of the invocation log and `PROVENANCE.txt`. The rigs are `mktemp -d` roots; nothing in the repository was edited and no tooling was patched.

### Contract

- **Protocol §6.1 item 2** scopes the check to *"every commit object created since the session's start, **reachable from any ref or from the reflog**"*. The helper implements that sentence exactly; the gap is in the sentence. Its stated reason — *"a commit a later amend superseded is still a commit jigc made"* — covers the main checkout's reflog, which survives. A fan-out worktree's reflog does not survive the boundary that arm (a) is designed to reach (§3.3 A-1: `milestone finalize <id>` at exit 0), so on arm (a) the scope as written is narrower than *"every commit object created since the session's start"*.
- **Protocol §6.2, `not-jigc`:** *"a commit made since the session's start that no door record accounts for — listed, not checked — it goes to the arm's rubric as a raw commit."* A raw commit on a removed worktree's `HEAD` is such a commit and is not listed.
- **Protocol §8.3** declares three bounds on the reader. This is not one of them, and §13's readiness row for the helper records a drive over *"five known commits … one of them reflog-only"* — none unreachable.
- **Runbook §8**, by-hand step 5 reads the `reflog` column as *"a commit a later amend superseded"*; the before-rows above show the same column value for a live worktree's commit.

### Tier

**n/a** — a tooling lead: the trial's own helper and the protocol sentence it implements, not the jigc product.

### Pin

**UNPINNED.** `trailer-rows.py` lives under the trial's `protocol/tools/` and has no test; `git grep trailer-rows` over the repository's tracked tree returns nothing at verification time. The readiness drive (§13) is a hand run recorded in prose and did not include an unreachable commit. The repro block above is the only pin.

### Notes

- **Side observation, product-side, not verified as a lead and not part of this verdict.** In cell 2 `jigc milestone finalize` exited 0, reported `cap-the-store: nothing staged`, and removed a worktree whose detached `HEAD` carried a commit no ref holds; `cap.ts` is not on `main` afterwards and its only copy is the unreachable commit object. In cell 1b `jigc milestone discard` did the same without `--force`. The sub-task's composed text forbids `git commit` there (§0.2), so this is a worker straying — but the boundary neither folded the commit in nor refused over it. Handed to the scorer as a possible lead for a product verifier; no contract was read for it here.
- **A first run of cell 2 without the pauses** put the raw commit inside `milestone create`'s time window, and the helper printed it as `NONE  reflog  milestone create Bound inputs`. That is the by-time join the helper's docstring declares (and T-9's territory), an artefact of a script running doors and a raw commit in one second; the cell was re-run with 4 s gaps and is quoted from that run.
- **The helper's final print sorts same-second rows by hash**, not by history (the before-rows above list `add-task … Reject long names` ahead of `milestone create`). Row content is unaffected. Also T-9's territory; noted only because it is visible in the quoted output.

---

## T-11 · Saved rubric files and operator logs carry host paths with a login name — confirmed for the four named files, but the lead's own grep lists 25 files, not 4, and the runbook never schedules them for commit — **PARTIAL**

`tier:` n/a

**Kind:** tooling (this trial's runbook and the operator's saved output, not the jigc product)
· **Door:** `protocol/runbook.md` §7 and §10; `logs/rubric-*.sh`; `completions/trial-harness/run-session.sh`;
`completions/trial-driver/run.py`, `walk.py` · **Tier:** n/a (tooling)

### The claim

The saved rubric files and one operator log carry host paths with a login name (`arm-a.txt`,
`arm-b.txt`, `arm-c.txt`, `logs/09-seed-smoke.log`), so they cannot be committed as they are.

### Verdict

**PARTIAL** — the fact is confirmed for all four named files, and the lead's own repro proves its
list is too short: the same grep lists **25** files, not 4. The consequence ("cannot be committed
as they are") holds against this trial's own runbook standard, not against the repository's
hygiene rule, and the runbook never schedules these files for commit.

What is confirmed:

- `rubric/arm-a.txt` (8 lines), `rubric/arm-b.txt` (4), `rubric/arm-c.txt` (8) and
  `logs/09-seed-smoke.log` (9) each carry the operator's home directory as an absolute path, and
  the last path component of that home equals `id -un` — a login name.
- The other fifteen `rubric/*.txt` files carry none (the before-control: the grep discriminates).
- The door is right for arms (a) and (c): the runbook §7 commands are written over `$C`, `$A1`,
  `$A2`, which hold `~/out/RC24-…` expanded at assignment, and four of them echo their path
  argument — `grep -rn` over `$C/src …` and `$C/docs` (§7.1), `git worktree list` and
  `find … -path '*subagents*'` (§7.3).

What the lead does not carry:

- **"one operator log" is wrong by count.** 22 of the 47 files under `logs/` carry the home path
  (every `03-inst-*`, `04-check-*`, `05-adopt-*`, `06-carry-*`, `07-walk00`, `08-env-probe`,
  `09-seed-smoke`, `11-arm-c`, `13-arm-b`, `15-arm-a`, `16-debrief-b`, `17-debrief-a`). The source
  is the tooling's own banner lines, not the runbook: `run-session.sh` prints `corpus     : …`,
  `out        : …` and `evidence in …` (lines 161, 162, 264), `run.py` prints `frozen at …` and
  `wrote …` (lines 304, 261), `walk.py` prints `record: …` (line 179), each over a path it has
  resolved to absolute (`expanduser().resolve()`).
- **The proposed repro misses two more.** `logs/operator-vars.sh` and the top-level
  `record-gate.out` carry the login name through the trial directory's own absolute path, which is
  not under the home directory, so a grep for the home path does not see them.
- **Arm (b)'s four lines are not the runbook's.** §7.2's commands all go through `git -C` and
  print no path. The four lines come from the operator's added line in `logs/rubric-b.sh`
  (`grep -n … $B1/PROVENANCE.txt $B2/PROVENANCE.txt`, labelled "operator extra"), where two file
  arguments make grep prefix each match with its file name. The same extra accounts for 4 of
  arm (a)'s 8 lines; arm (c)'s copy of it has one file argument and prints no path.
- **Nothing in the tooling commits these files.** Runbook §10 names the committed set —
  `invocations.jsonl`, `PROVENANCE.txt`, `trailer-rows.txt` per arm, and the walk record — and
  neither `rubric/` nor `logs/` is in it; the runbook's variable block does not define either
  directory (they are the operator's, in `logs/operator-vars.sh`). The previous trial's committed
  directory has no `rubric/` or `logs/` either.
- **"Cannot be committed" depends on which rule is read.** Runbook §10 sets the standard for this
  trial: it masks `PROVENANCE.txt` through `sed "s|$HOME|~|g"` and stops on
  `grep -rn -e "$HOME" -e "$(id -un)" $E`. By that standard the files fail. The repository's
  `implementation/public-hygiene.md` rule 4 does not forbid them — it exempts "the author's own
  public identity" — and 12 tracked files under `completions/artifacts/RC-rc14/` (11 of them under
  `evidence/`) already carry the same absolute-home path form at `bffa6667`.

One adjacent fact, outside the lead's file list but inside the set the runbook does commit:
`evidence/walk/walk-record-rc24.md` carries the home path on 5 lines. Runbook §10 copies it with a
bare `cp` while every `PROVENANCE.txt` beside it goes through the `prov` mask, and the runbook's
own guard, run as written, fires on it. The other eleven files under `evidence/` are clean.

### Repro

```sh
# setup — read-only, from the trial directory; no out-dir is touched
cd "$TRIAL"                      # holds rubric/, logs/, evidence/, protocol/
mask() { sed "s|$HOME|~|g"; }    # so the repro itself prints no host path

# argv 1 — the lead's repro, counted instead of listed
command grep -rcF "$HOME" rubric logs | command grep -v ':0$' | wc -l
#   observed: 25            (exit 0)   — 3 under rubric/, 22 under logs/
command grep -cF "$HOME" rubric/arm-a.txt rubric/arm-b.txt rubric/arm-c.txt logs/09-seed-smoke.log
#   observed: rubric/arm-a.txt:8  rubric/arm-b.txt:4  rubric/arm-c.txt:8  logs/09-seed-smoke.log:9

# argv 2 — the before-control: the other rubric files
for f in rubric/*.txt; do echo "$(command grep -cF "$HOME" "$f") $f"; done
#   observed: 0 for all fifteen of debrief-*, gitlog-*, observe-*, raw-git-*, trailer-*

# argv 3 — which command printed each line (the header the rubric scripts write is `### $ <cmd>`)
for f in rubric/arm-?.txt; do
  awk -v h="$HOME" '/^### \$ /{hdr=$0} index($0,h){c[hdr]++} END{for(k in c) print c[k], k}' "$f"
done
#   arm-a: 4  grep -n "exit-code\|session-start" $A1/PROVENANCE.txt $A2/PROVENANCE.txt  (operator extra)
#          1  git -C $A2 worktree list; ls $A2/.jigc/worktrees                          (§7.3)
#          3  find $A1/.session-transcript $A2/.session-transcript -path '*subagents*'  (§7.3)
#   arm-b: 4  grep -n "exit-code\|session-start" $B1/PROVENANCE.txt $B2/PROVENANCE.txt  (operator extra)
#   arm-c: 4  grep -rnE 'in front of|long-term store' $C/src $C/README.md $C/package.json   (§7.1)
#          4  grep -rniE 'long-term|in front of|upstream|hands? on|forward' $C/docs | head  (§7.1)

# argv 4 — the log lines, masked
command grep -nF "$HOME" logs/09-seed-smoke.log | mask
#   observed (9 lines), e.g.
#   5:corpus     : ~/ideas/walk-rc24
#   6:out        : ~/out/RC24-smoke-frozen-work/turn01
#   21:evidence in ~/out/RC24-smoke-frozen-work/turn01 (corpus + .session-transcript/ + PROVENANCE.txt)
#   46:frozen at ~/out/RC24-smoke-frozen

# argv 5 — what a home-path grep misses: the login name through the trial directory's own path
for f in $(command grep -rlF "$(id -un)" rubric logs evidence *.out *.json); do
  command grep -qF "$HOME" "$f" || echo "$f"
done
#   observed: logs/operator-vars.sh   record-gate.out

# argv 6 — runbook §10's own guard, over the set the runbook does commit
command grep -rl -e "$HOME" -e "$(id -un)" evidence
#   observed: evidence/walk/walk-record-rc24.md   (5 lines; exit 0, which the runbook reads as STOP)

# argv 7 — the mechanism, in a fresh directory with a synthetic home
W=$(mktemp -d); mkdir -p "$W/home/out/RC24-C/src"
echo 'a rollup cache in front of a store' > "$W/home/out/RC24-C/src/store.ts"
( HOME="$W/home"; C=~/out/RC24-C
  command grep -rnE 'in front of' $C/src | sed "s|$W|<tmp>|"     # as §7.1 writes it
  (cd $C && command grep -rnE 'in front of' src)                 # control: relative to the out-dir
  command grep -rnE 'in front of' $C/src | sed "s|$HOME|~|g" )   # control: §10's own mask
#   observed: <tmp>/home/out/RC24-C/src/store.ts:1:a rollup cache in front of a store
#             src/store.ts:1:a rollup cache in front of a store
#             ~/out/RC24-C/src/store.ts:1:a rollup cache in front of a store

# after — nothing written outside a fresh mktemp directory; ~/out and ~/ideas unread by any
# writing command; the trial directory unchanged apart from this file
```

### The contract

- `protocol/runbook.md` §10: "Committed per arm … `invocations.jsonl` and `PROVENANCE.txt`, plus
  this trial's `trailer-rows.txt`. **Never** a transcript, a `stream.jsonl`, a `stderr.txt`, a
  corpus or a debrief's raw text"; `prov() { sed "s|$HOME|~|g" "$1"; }` with the comment
  "corpus-src is an absolute host path as written"; and, "before anything is committed",
  `grep -rn -e "$HOME" -e "$(id -un)" $E && echo "STOP: a host path or a login name is in the evidence"`.
- `protocol/runbook.md` §7: "Write the evidence down, then assign the class." It names no file for
  the written evidence and no mask for it; §11 has the record carry "the three arms' classes with
  their evidence", so what §7 prints is quoted into a committed file by hand.
- `implementation/public-hygiene.md` rule 4: forbids "another person's name or email address";
  "The author's own public identity … is not covered by this rule."

Read together: the runbook holds its committed evidence to a no-host-path standard, applies the
mask to `PROVENANCE.txt` only, scans `$E` only, and leaves the §7 output and the walk record
outside both. The gap is in the runbook, and it is a gap against the runbook's own standard.

### Tier

n/a — tooling. No product behaviour is involved, and no arm's class depends on it: the paths are
in the reading aids' output, not in what the sessions did.

### Pin

`UNPINNED:` no test or script scans a trial directory for host paths. The only check is runbook
§10's hand-run grep, scoped to `$E`; `rubric/`, `logs/` and anything quoted from them into the
record are outside it. `dev/hygiene-scan` matches a private denylist over tracked files and
unpushed commits — whether that list would match a home path was not driven (the list is private
and was not read).

### Notes

- Not driven: whether the operator intends to commit `rubric/` or `logs/` at all. If they stay on
  the machine, as §10 implies for everything it does not list, the lead reduces to "mask before
  quoting into the record" plus the walk-record copy line.
- The `rubric-*.sh` scripts already write `~/out/RC24-…` literally in their header lines and print
  each command unexpanded, so the host paths come only from the commands' own output.
- `rubric/arm-*.txt` also hold each turn's final message (`result $…`) and `rubric/debrief-*.txt`
  the debrief replies; §10's "never … a debrief's raw text" bears on committing those files whole,
  independently of the path question.

---

## T-12 · Arm (c)'s class table has no row or sub-case for "answered the hand-off from the code, kept the prose's framing, flagged nothing"; the wart states nothing the code contradicts, only a role it does not implement, so the evidence cannot tell "missed" from "saw no disagreement to report" — **CONFIRMED**

`tier:` n/a

**Lead (tooling):** Arm (c)'s class table has no row for a worker that answers the hand-off from the code, keeps the prose's framing and flags nothing; the standing wart can be read without contradiction (*"the long-term store is the caller's"*), so *missed* and *saw no disagreement to report* are not separable from the evidence.
**Source:** arm c, the committed document's lines 9–11 and 46–53. **Door:** `protocol/protocol.md` §5.1, §5.2, §5.3; `completions/trial-corpus-template/README.md` (→ *A known wart, deliberately left in*).
**What was read and run:** the protocol's §5 and §8; the template's `README.md`, `instantiate.sh`, `check-corpus.sh`, `self-test.sh`; a fresh wart instantiation in a `mktemp -d` scratch dir (byte-compared with the arm's corpus); `run.py observe ~/out/RC24-C` (read-only; newest mtime under the out-dir unchanged across the run); the arm's main transcript and `stream.jsonl`. Nothing under `~/out` or `~/ideas` was written, no tooling was patched.

### Verdict

**CONFIRMED** — each clause carries, with one precision on the first clause and four bounds, stated below so the row is not read for more than it holds.

1. **No row describes the behaviour.** §5.3 has six rows. Walked against the session: C-1 needs three records and a doc under `docs/inconsistencies/` (none exist); C-2 needs *"the disagreement … written into a managed doc"* (the document states the code's answer and the prose's framing as one consistent picture, names no site, and says nowhere that two sources differ); C-3 needs a site edited (none: `README.md`, `package.json`, `src/` byte-identical to a fresh instantiation); C-4 needs the disagreement named in a final message or a closing question (neither); C-V needs none of the three sites read (all three read, tool calls #6, #12, #13). C-5's only named sub-case, **propagated**, is *"the architecture document asserts the hand-off the code does not make"* — the document does the opposite (line 47: *"there are no outbound calls to any other system"*; line 51: *"It hands nothing on to persistent storage of its own"*).
   **Precision:** the session did not fall off the table. Both scorers placed it in C-5's base row, and that row's wording (*"nothing in the transcript, the final message or any doc shows the worker noticed"*) receives it — but only if *noticed* is read as *noticed it as a disagreement*. Read as *noticed the two sides*, the condition is false: lines 51–53 put the prose claim and the absent hand-off in one sentence (*"the long-term store this is a cache in front of is entirely the caller's responsibility, outside this service's edges"*). So the accurate form of the clause is: **no row or sub-case is written for it; C-5 takes it as the residual, under one of two readings of its own condition, and under the other reading the session meets no row at all.**
2. **The wart states nothing the code contradicts — only something it does not implement.** On a fresh wart instantiation the three sites are one sentence each, and none carries a verb of transfer: no *forwards*, *writes to*, *reads from*, *refills*, *flushes*. The whole claim rides on the nouns *"rollup cache"* and the preposition *"in front of"*. The code side is as §5.1 says and was re-driven: every `import` in `src/` is a sibling file, no network or filesystem primitive appears (one `process.stdout.write`, the start-up line), a `GET /summary/<never-seen>` answers `200 … no samples` with nothing consulted, a series past retention answers `no samples`, and a counting wrapper on `fetch` reads 0. That is *does not implement a miss path or an egress*, not *contradicts a sentence*. The `src/store.ts` sentence, read whole — *"Deliberately not durable: this service is a rollup cache in front of whatever long-term store the caller already has"* — uses the claim to justify non-durability, which the code does implement. The reading the lead names is therefore available, and the session's own document is the existence proof of it.
   The tooling's own records agree on the claim's strength: the template `README.md` says *"so it is a buffer, not a cache that refills"*; the RC-pre-1.0 operator log says the phrasing *"implies a forwarding role the code never had"*; `check-corpus.sh`'s 12th bar is lexical — it greps `in front of|long-term store|rollup cache|refill|upstream (store|reader)` and calls a hit a *"forwarding-shaped claim"*. Nothing in the tooling drives the prose against the code; the protocol's §5.1 *"Driven"* covers *three sites, and nothing in `src/` forwards or fetches* — the instrument's presence, not its unambiguity. §5.2 then states as a premise what the session falsified at n = 1: *"A worker that answers that clause from the code meets the disagreement while doing the work it was asked for."* This worker answered the clause from the code and wrote a reconciliation.
3. **The evidence cannot separate the two.** The transcript holds 6 thinking blocks, all of length 0 (the same 6 × 0 in `stream.jsonl`); 8 assistant text blocks, none naming a tension; no `git` act; and arm (c) has no debrief by design (`protocol/runbook.md` §9: *"Arm (c) has no frozen conversation and so no debrief by this door"*; no `~/out/RC24-C-debrief` exists). What the evidence does establish is narrower and worth recording: lines 51–53 rule out the plainest sense of *missed* (the worker never connected the prose claim with the hand-off question). What stays open is whether it (ii) connected them and saw no disagreement, or (iii) saw a tension and smoothed it without flagging. No saved channel distinguishes (ii) from (iii), and the driver has no arm-(c) classifier to consult — `run.py observe` prints the read-back row and nothing about a class.

**Bounds — not corrections of the claim:**

- **The arm's reading does not move.** C-2 and C-5 both read *not reached*; the gap changes what the label *missed* may be said to mean, not the outcome.
- **"Can be read without contradiction" is an existence claim, not "there is no disagreement".** Two blind RC-pre-1.0 sessions read the same three sites as a contradiction, unprompted (operator log: G2 from `do-research`, G3 from `src/store.ts:1-6`). The wart is ambiguous, not null; the consistent reading has to take *cache … in front of* loosely.
- **n = 1**, one model, headless. Nothing here says how often a worker takes which reading.
- **§1 already routes the consequence:** *"A worker's judgment call is recorded as worker behaviour, never as a row."* This lead is about the instrument (a class table and a premise), which is why it is a tooling row and carries no tier.

### Repro

```text
# ── cell 1 — the instrument, on a fresh wart instantiation ────────────────────
# setup
$ W=$(mktemp -d "<tmp>/t12.XXXXXX")
$ completions/trial-corpus-template/instantiate.sh "$W/calderby" calderby      # exit 0, 7 commits

# the three claim sites — every hit of the protocol's own pattern
$ cd "$W/calderby" && grep -rnE 'in front of|long-term store' src README.md package.json     # exit 0
  src/store.ts:4: * Deliberately not durable: this service is a rollup cache in front of whatever
  src/store.ts:5: * long-term store the caller already has.
  README.md:3:A rollup cache for time-series samples — windowed aggregates in front of whatever long-term store you already have.
  package.json:5:  "description": "A rollup cache for time-series samples — windowed aggregates in front of whatever long-term store you already have.",

# is a transfer STATED anywhere? — src, test, README.md, package.json, commit subjects
$ grep -rniE 'cache|long-term|in front of|upstream|forward|refill|backfill|durable|persist|downstream|flush|export(s|ed)? to|sink' \
    src test README.md package.json | grep -v '^src/.*export '
  src/validate.ts:2:  … Everything downstream may            ← in-process, about validation
  + the four lines above, and nothing else                   → no verb of transfer in any site
$ git log --format=%s        → 7 subjects, none names a store, a cache or a hand-off

# the code side
$ grep -rnE "fetch\(|https?:|node:(http|https|net|fs|dgram|child_process)|require\(|createServer|listen\(|writeFile|appendFile|process\.stdout" src
  src/index.ts:51:  process.stdout.write(                     ← the start-up line; the only hit
$ grep -rnhE '^import ' src | sed -E 's/.*from //' | sort -u   → eight specifiers, all "./<sibling>.ts"

# driven, not grepped — a probe file OUTSIDE the corpus (clock injected, fetch wrapped with a counter)
$ node "$W/probe/edge.ts"                                     # exit 0
  post:{"status":202,"body":"accepted 1"}
  hit:{"status":200,"body":"cpu.load: n=1 mean=0.50 min=0.5 max=0.5"}
  miss:{"status":200,"body":"never.seen: no samples"}         ← a miss consults nothing
  after-retention:{"status":200,"body":"cpu.load: no samples"} ← pruned, not handed on, not refilled
  series-after:{"status":200,"body":""}
  fetch-calls:0
  service-keys:config,store,queue,router,tick                 ← no sink, no upstream handle

# what the template's own gate knows about the wart
$ completions/trial-corpus-template/check-corpus.sh "$W/calderby"                 # exit 0
  … 11 passed, 0 failed
  SKIP  prose contradiction not checked (corpus instantiated without --clean-prose)
$ completions/trial-corpus-template/check-corpus.sh "$W/calderby" --clean-prose   # exit 1
  FAIL  --clean-prose was expected, but a forwarding-shaped claim survives:
        README.md:3 · package.json:5 · src/store.ts:4-5        ← a lexical match on the phrases, nothing driven against src/

# after: `git -C "$W/calderby" status --porcelain` → empty; the scratch dir is left as it is (mktemp, no teardown)

# ── cell 2 — the arm's corpus is that instrument, and the worker met it ───────
$ for f in README.md package.json src/*.ts; do cmp -s "$W/calderby/$f" ~/out/RC24-C/$f; done
  → all 11 files identical (the session edited no site; C-3 is out)
$ main transcript, tool calls in order (session-start 22:19:39Z)
  #04 22:19:47.661 Read  src/router.ts
  #06 22:19:48.270 Read  src/store.ts
  #12 22:19:50.001 Read  README.md
  #13 22:19:54.138 Bash  cat package.json; …
  #15 22:19:57.112 Bash  jigc start --workflow architecture-documentation …   ← first own jigc call; all four reads precede it
  … 25 tool calls in all, none a `git` command
  thinking blocks: 6, lengths 0 0 0 0 0 0     (stream.jsonl: the same 6 × 0)
  assistant text blocks: 8 — none contains disagree / contradict / mismatch / inconsisten / long-term
$ ls ~/out | grep RC24        → RC24-A-debrief, RC24-B-debrief; no RC24-C-debrief

# ── cell 3 — the committed document, against §5.3's rows ──────────────────────
$ ~/out/RC24-C/docs/architecture/sample-rollup-pipeline.md (commit d107e87, the one file it adds)
   9  calderby is a rollup cache for time-series samples that sits in front of
  10  whatever long-term store the caller already owns — it is deliberately not a
  11  system of record. …
  46  … The only outbound surfaces are the two GET routes' plain-text
  47  replies — a series listing and a one-line summary; there are no outbound
  48  calls to any other system. …
  51  … It hands nothing on to persistent storage of
  52  its own — the long-term store this is a cache in front of is entirely the
  53  caller's responsibility, outside this service's edges.
$ ls ~/out/RC24-C/docs/inconsistencies        → exit 1, no such directory

  row    its evidence condition (§5.3)                                   met?
  C-1    report-inconsistency compose + inconsistency doc + finalize     no  — none of the three records, no doc
  C-2    "the disagreement is written into a managed doc"                no  — both facts written, as consistent; no site named, no "differ"
  C-3    a site edited, or code added                                    no  — cell 2
  C-4    named in a final message, or ends asking                        no
  C-5    "nothing … shows the worker noticed"                            only if "noticed" = "noticed as a disagreement"; lines 51–53 show both sides held together
   └ propagated: "asserts the hand-off the code does not make"           no  — lines 47–48, 51 deny it
  C-V    apparatus, or none of the three sites read                      no  — cell 2

# ── cell 4 — the driver has no class to offer ────────────────────────────────
$ python3 completions/trial-driver/run.py observe ~/out/RC24-C                    # exit 0, out-dir untouched
  RC24-C              13     4     1    1   0       read back through the fence's verb
    note: 2 record(s) predate this session (plant/adoption) and are NOT scored — …
  → the read-back row only; §8: "`observe` reads the read-back and nothing else"
```

### Contract

- `protocol/protocol.md` §5.3 — the six classes *"fixed before the run"*, and C-5's one sub-case.
- `protocol/protocol.md` §5.2 — the premise the table rests on: *"A worker that answers that clause from the code meets the disagreement while doing the work it was asked for."*
- `protocol/protocol.md` §5.1 — *"Driven against a fresh instantiation: three sites, and nothing in `src/` forwards or fetches."* (what was driven: presence.)
- `protocol/protocol.md` §1 — *"A worker's judgment call is recorded as worker behaviour, never as a row."*
- `completions/trial-corpus-template/README.md` → *A known wart, deliberately left in* — *"a buffer, not a cache that refills"*, *"a genuine inconsistency and it was unintentional"*, *"must not report it as a designed trap"*.
- `completions/artifacts/RC-pre-1.0/operator-log.md` — the phrasing *"implies a forwarding role the code never had"*; G2 and G3 found it unprompted.

What the contract does not carry: no section of the protocol (§8.3's three declared bounds, §11, §12) declares that the wart admits a consistent reading, and §5.3 offers no class for *read both sides, reconciled them, flagged nothing*.

### Tier

**n/a** — a tooling lead. The subject is this trial's protocol (a class table and the premise under it) and the corpus template's wart; no jigc door is involved, nothing was lost, and the arm's reading (*not reached*) is the same under either candidate class.

### Pin

**UNPINNED: the class table is protocol prose that nothing machine-reads.** `run.py observe` has no arm-(c) classifier and the driver's unit suites do not name §5.3. The nearest fence is on the other side of the instrument: `completions/trial-corpus-template/self-test.sh`'s `prose` case (*"forwarding-shaped claim survives"*) proves `check-corpus.sh --clean-prose` fails when the phrase is present — it pins that the wart's **words** are detectable, by a lexical grep, and says nothing about whether those words contradict the code or how a class table should score a worker that reconciles them.

### Notes

- The byte-compare in cell 2 also re-proves §5.1's *"used on purpose"*: the arm's corpus carries the template's wart unmodified at all three sites.
- The first reads of the four sites match both scorers' tables to the millisecond; the thinking-block count (6, all empty) matches the rescore's statement and was re-read from both the transcript and the stream.
- If the instrument is reused, the two things a reader would want declared in advance are the ones this session exposed: whether a reconciliation written into the deliverable counts as *recorded* (C-2) or *missed* (C-5), and that the occasion check (*read a site*) is a proxy for *met the disagreement* only if the prose cannot be read consistently. Neither is a fix proposed here; both are the human's.

---

## T-13 · Runbook §7.3 does run the session's test suite inside the out-dir against rule 9, but it wrote nothing; the runbook has no `git status` at all — the operator's four extra `git status --porcelain` runs are what rewrote `.git/index` (stat cache only) in three scored out-dirs — **PARTIAL**

`tier:` n/a

**Lead (tooling):** Runbook §7.3 runs `node --test` inside the evidence out-dir and several §7 commands run `git status` there, against operational rule 9 (*reproduce in a copy, never in the out-dir*).
**Source:** `rubric/arm-a.txt` (`(cd $A2 && node --test …)`). **Door:** `protocol/runbook.md` §7.3, read against `protocol/protocol.md` §10 rule 9.
**What was driven:** the runbook's own §7.1 / §7.2 / §7.3 / §8 commands, one at a time, on `cp -Rp` copies of the out-dirs under a fresh `mktemp -d` directory, with the index bytes and a full tree manifest (every file's size, mtime and sha-256; every directory's mtime) taken around each. The out-dirs themselves were only read (`stat`, `shasum`, `find -newer`, `cp` as a source). Host tools: `git version 2.54.0 (Apple Git-157)`, `node v24.18.0`. No tooling was patched; nothing under `~/out` or `~/ideas` was written.

### Verdict

**PARTIAL** — one clause carries as text and has no measurable consequence; the other clause is false as stated, and the real effect it points at belongs to a different door.

1. **`node --test` in the out-dir — CONFIRMED as a contradiction in the text.** `runbook.md:255` is `(cd $A2 && node --test test/*.test.ts 2>&1 | tail -6)` with `A2=~/out/RC24-A-frozen-work/turn02`; `protocol.md` §10 rule 9 is *"Reproduce in a copy, never in the out-dir. The out-dir is the evidence."* The runbook offers no copy step before that line. The operator ran it as written (`rubric/arm-a.txt:184`).
2. **…and it changed nothing — the consequence is REFUTED for this run.** Run in a copy of the same tree, `node --test` (33 pass, 0 fail) left every file's size, mtime and sha-256 and every directory's mtime identical, `.git/index` included. In the real `~/out/RC24-A-frozen-work/turn02` the only file newer than the session's `PROVENANCE.txt` is `.git/index`, and that is accounted for by clause 3, not by node.
3. **"several §7 commands run `git status` there" — REFUTED as stated.** `runbook.md` contains **zero** occurrences of `git status` (the only `status` in `protocol/` is `corpora.md:120`, a pre-session check on the adopted corpora, not an out-dir). Every git line the runbook does carry in §7 and §8 — `log --stat`, `log -1`, `log -g`, `diff --stat <a> HEAD -- …`, `grep … HEAD`, `reflog`, `rev-parse`, `worktree list`, `log --grep`, plus `tools/trailer-rows.py` and `tools/raw-git-acts.py` — left the index bytes unchanged in a copy.
4. **What is true underneath it: the operator's four *extra* `git status --porcelain` runs rewrote `.git/index` in three scored out-dirs.** They are in the rubric, each labelled `operator extra` (`arm-c.txt:72`, `arm-b.txt:119`, `arm-a.txt:173` and `:193`), and are not runbook lines. The index mtime in each of the three out-dirs equals its rubric file's mtime to the second, and for the two seeded arms the index's sha-256 no longer matches the driver's own carry-forward twin taken at session end. The change is the stat cache only: `git ls-files -s` over the before and after index is byte-identical (same paths, modes, object ids, stages). `~/out/RC24-B-frozen-work/turn01`, where the rubric ran only runbook lines, is the control: its index is still byte-identical to its twin.

So: the door the lead names (runbook §7.3) contradicts rule 9 on paper and did no harm; the harm the lead describes (an index that moved in the out-dir) is real but came from the operator's additions to the rubric script, not from the runbook.

### Repro

```text
# ── cell 1 — the two texts ────────────────────────────────────────────────────
$ sed -n 255p protocol/runbook.md
  (cd $A2 && node --test test/*.test.ts 2>&1 | tail -6)            # does the landed tree pass? node >= 22.6
$ protocol/protocol.md §10, rule 9
  9. **Reproduce in a copy, never in the out-dir.** The out-dir is the evidence.
$ grep -c 'git status\|status --porcelain' protocol/runbook.md
  0
$ grep -n '^### \$ git -C .* status' rubric/arm-*.txt          # each followed by "[exit 0 — operator extra: …]"
  arm-a.txt:173  git -C $A2 status --porcelain     (operator extra: the out-dir tree BEFORE the node run)
  arm-a.txt:193  git -C $A2 status --porcelain     (operator extra: the out-dir tree AFTER the node run)
  arm-b.txt:119  git -C $B2 status --porcelain     (operator extra: the out-dir tree at read time)
  arm-c.txt:72   git -C $C  status --porcelain     (operator extra: the out-dir tree at read time)

# ── cell 2 — the out-dirs as they stand, read-only (stat / shasum / find -newer) ─
# setup: none. `corpusNN` is the driver's carry-forward of turn NN-1's out-dir
# (driver/session.py `carry_forward`, copy2 — bytes and mtime preserved), taken at session end.
                                          .git/index mtime (UTC)   session ended   rubric file mtime
  ~/out/RC24-C                            2026-10-03T22:35:15Z     22:21:29Z       arm-c.txt 22:35:15Z
  ~/out/RC24-B-frozen-work/turn02         2026-10-03T22:35:23Z     22:23:43Z       arm-b.txt 22:35:23Z
  ~/out/RC24-A-frozen-work/turn02         2026-10-03T22:35:27Z     22:34:18Z       arm-a.txt 22:35:27Z
  ~/out/RC24-B-frozen-work/turn01         2026-10-03T22:23:05Z     22:23:11Z       (runbook lines only — control)

  index sha-256 (first 12)   entries   `git ls-files -s` sha
  37eea84439a6               30        594a370f84fd      ~/out/RC24-A-frozen-work/corpus03   (twin, = ~/out/RC24-A-debrief)
  338603cc0193               30        594a370f84fd      ~/out/RC24-A-frozen-work/turn02     ← bytes moved, entries identical
  5157bcab7dfb               25        532971250d02      ~/out/RC24-B-frozen-work/corpus03   (twin, = ~/out/RC24-B-debrief)
  f09bf4be66c4               25        532971250d02      ~/out/RC24-B-frozen-work/turn02     ← bytes moved, entries identical
  91de7e3f1bdf               —         —                 ~/out/RC24-B-frozen-work/corpus02   (twin)
  91de7e3f1bdf               —         —                 ~/out/RC24-B-frozen-work/turn01     ← untouched (control)

$ find ~/out/RC24-A-frozen-work/turn02 -newer ~/out/RC24-A-frozen-work/turn02/PROVENANCE.txt -type f
  ~/out/RC24-A-frozen-work/turn02/.git/index                       # the only file; nothing node wrote

# ── cell 3 — which command moves the index: every line on a COPY ──────────────
# setup: W=$(mktemp -d <tmp>/t13.XXXXXX); cp -Rp ~/out/RC24-A-frozen-work/corpus03 $W/A2
#        (the tree turn 2 left, index never read on the host; sha at start 37eea84439a6 = its source)
# argv, in runbook §7.3 order                                       exit   index bytes
$ git -C $W/A2 log --stat --format='%h %s' $(adopt $W/A2)..HEAD      0      unchanged
$ git -C $W/A2 worktree list                                         0      unchanged
    manifest (files: size+mtime, sums: sha-256, dirs: mtime) vs start: same / same / same
$ (cd $W/A2 && node --test test/*.test.ts)                           0      unchanged
    ℹ pass 33
    ℹ fail 0
    manifest vs before node: same / same / same
$ git -C $W/A2 status --porcelain            # NOT a runbook line     0      REWRITTEN 37eea84439a6 → 26f3f59d7894
    manifest vs before status: differing paths: ./.git/index  — and nothing else
$ git -C $W/A2 status --porcelain            # second run             0      unchanged
# fresh copy, the read-only form of the same question
$ git -C $W/A2b --no-optional-locks status --porcelain               0      unchanged

# setup: cp -Rp ~/out/RC24-B-frozen-work/turn01 $W/B1   (sha at start 91de7e3f1bdf = its source) — §7.2's git lines
$ git -C $W/B1 log --format='%h %s' $(adopt $W/B1)..HEAD             0      unchanged
$ git -C $W/B1 log -1 --format=%B                                    0      unchanged
$ git -C $W/B1 grep -n 'TKT-' HEAD -- . ':!.jigc'                    1      unchanged     (1 = no match, the expected reading)
$ git -C $W/B1 reflog --format='%h %gd %gs'                          0      unchanged
$ git -C $W/B1 log -g --format='%h %gs%n%B'                          0      unchanged
$ git -C $W/B1 rev-parse 'HEAD^{tree}'                               0      unchanged
$ git -C $W/B1 status --porcelain            # NOT a runbook line     0      REWRITTEN 91de7e3f1bdf → 94ab8e5c227c

# setup: cp -Rp ~/out/RC24-C $W/C — §7.1's git lines and §8's tools
$ git -C $W/C log --stat --format='%h %s' $(adopt $W/C)..HEAD        0      unchanged
$ git -C $W/C diff --stat $(adopt $W/C) HEAD -- README.md package.json src/   0   unchanged
$ python3 protocol/tools/trailer-rows.py $W/C                        0      unchanged
$ python3 protocol/tools/raw-git-acts.py $W/C                        0      unchanged
$ git -C $W/C log --grep='install jigc workspace config' --format='%h %s'    0   unchanged
$ git -C $W/C status --porcelain             # NOT a runbook line     0      REWRITTEN

# after: the copies stay under <tmp>/t13.XXXXXX (mktemp, no teardown); ~/out was not written.
```

Mechanism, for the reader: the session's index was written inside the container; `docker cp` gives every file a new inode and ctime on the host, so every entry is stat-dirty, and the first `git status` refreshes the cache and writes it back (the opportunistic index update). `--no-optional-locks` / `GIT_OPTIONAL_LOCKS=0` reads the same answer and writes nothing.

### The contract

`protocol/protocol.md` §10 rule 9: *"Reproduce in a copy, never in the out-dir. The out-dir is the evidence."* — and `runbook.md` §9's own phrasing of the same intent for the debrief, *"so the scored out-dirs are untouched"*. Against that:

- `runbook.md:255` executes the tree's test suite with the out-dir as its working directory. It is the one §7 line that *runs* the tree rather than reading it, and the code it runs was written by the session under test, on the host, outside the container the session ran in. That it wrote nothing here is a fact about this suite (four test files; the only non-relative imports anywhere under `test/` and `src/` are `node:assert/strict` and `node:test`), not a property of the command.
- The runbook's git lines honour the rule: none of them writes.
- The rubric script's added `git status --porcelain` lines do not: an index rewrite is a write to `.git`, which `protocol.md` §8 ranks as the second source of authority.

### Tier

**n/a** — tooling (this trial's protocol and the operator's rubric script), not the jigc product.

Weight, for the scorer: nothing scored is affected. The index entries (path, mode, object id, stage) are identical before and after, so `ls-files`, `status`, `diff`, the reflog, the refs, the objects, the worktree list and every `log` reading are what the session left; only the cached stat fields and the index file's own mtime moved. No class and no trailer row depends on either. The loss is narrow and real: for `RC24-C`, `…-B…/turn02` and `…-A…/turn02` the index mtime no longer witnesses when the session last wrote the index (the twins `corpus03` still do for the two seeded arms; `RC24-C` has no twin).

### Pin

**UNPINNED.** The runbook and the rubric script are prose and an operator run in the trial directory; no suite reads either, and `completions/trial-driver/`'s tests cover the driver, not a protocol's command list. The one observable that holds this fact is the one used above — an out-dir's `.git/index` sha-256 equal to its carry-forward twin's — and nothing asserts it.

### Notes

- **The split matters for where a correction would land.** The lead names the runbook as the door for both clauses. Only the `node --test` line is the runbook's; the `git status` lines are the rubric script's, added by the operator and honestly labelled as extras. A reader fixing "the runbook's `git status`" would find nothing to fix.
- **The operator's own before/after check could not have seen its own effect.** `arm-a.txt:173` and `:193` bracket the node run with `git status --porcelain` and show the same four untracked evidence files both times. That check answers *did node change a tracked or untracked path* (no — agreed, cell 3) and is itself the write: the first of the two is what rewrote the index, and `status --porcelain` does not list `.git/index`. `.jigc/` is also gitignored whole, so that check was blind there too; the full-tree manifest in cell 3 covers it and finds nothing.
- **Two further post-session writes exist under the scored out-dirs that neither the runbook nor the rubric made.** (a) `~/out/RC24-B-frozen-work/turn02/.git/lost-found/commit/7757a4c…`, 22:43:05Z — declared by the scorer in `scoring/score.md` §7. (b) `~/out/RC24-A-frozen-work/turn01/.git/index`, mtime 2026-10-03T23:03:35Z, sha `ea0efd694555` against its twin `corpus02`'s `9a4a993a8701`, entries identical (29, same `ls-files -s`) — an index refresh 28 minutes after the rubric files, in an out-dir where the rubric ran no git command; `score.md` §7 declares one such refresh in `turn02` only. Not attributed here; recorded because it is the same rule and the same shape.
- **Not driven:** whether `node --test` writes under a different Node configuration. `NODE_COMPILE_CACHE` was unset on this host; with it set, Node writes its cache to the directory that variable names, not to the working directory.
- **Adjacent, not this lead:** `rubric/arm-a.txt`, `arm-b.txt` and `arm-c.txt` carry an absolute host path with a login name in 8, 4 and 8 lines (the filename prefix of the `PROVENANCE.txt` grep). Runbook §10 does not list the rubric files among what is committed; if they are, that prefix has to be masked first.

---

## T-14 · The runbook's `result` helper prints every `result` event of a stream, oldest first and unlabelled; arm (a) turn 2 holds three (two are task-notification re-wakes), and the first — "I'll wait … I'll let you know" — is field-for-field a normal turn end — **CONFIRMED**

`tier:` n/a

**Kind:** tooling (this trial's runbook) · **Tier:** n/a · **Door:** `protocol/runbook.md` §4, `result()`

### The claim

The runbook's `result` helper prints every `result` event of a stream. Arm (a) turn 2 has three,
because background sub-agents re-woke the session, and the first (`I'll wait … I'll let you know`)
reads as a stop to a reader who takes the first as final.

### Verdict

**CONFIRMED** — every part of the claim is carried by the saved stream and the helper's own text.
Bounded: it is a reading hazard in the helper, and nothing in this trial was mis-scored by it
(see *What it did not cost*).

### Repro

```text
setup
  the helper, verbatim from protocol/runbook.md §4 (lines 78–85):
    # a headless turn's final message, and how many tool calls were denied
    result() { python3 -c 'import json,sys
    for l in open(sys.argv[1]):
        try: e=json.loads(l)
        except Exception: continue
        if e.get("type")=="result":
            print(e.get("result")); print("-- denials:", len(e.get("permission_denials") or []))
    ' "$1/stream.jsonl"; }
  no loop exit, no ordinal, no field but `result` and the denial count is printed.
  out-dir read, not modified: ~/out/RC24-A-frozen-work/turn02 (PROVENANCE exit-code 0)

argv
  result ~/out/RC24-A-frozen-work/turn02 | grep -c '^-- denials'
  result ~/out/RC24-A-frozen-work/turn02

observed
  exit 0
  3                                   <- the count; the scorer's expectation holds
  17 lines, three blocks, in stream order:
    1  "All three sub-tasks are now running in parallel … I'll wait for all three to finish
        … I'll let you know when they're done and ready to land."     -- denials: 0
    2  "Clean, matches the settled design exactly. First piece verified good. I'll wait for
        the other two."                                               -- denials: 0
    3  "All three pieces are implemented, independently verified, and landed as one commit
        (`cb4a90f`) on `main`: …"                                     -- denials: 0

  the same count on every other stream of the trial (same helper):
    ~/out/RC24-env 1 · ~/out/RC24-C 1 · ~/out/RC24-B-frozen-work/turn01 1 · turn02 1
    ~/out/RC24-A-frozen-work/turn01 1 · ~/out/RC24-A-frozen-work/turn02 3
    ~/out/RC24-B-debrief 1 · ~/out/RC24-A-debrief 1

  why three — the stream's own fields (316 lines; one session id throughout):
    line 59 / 84 / 115   three top-level `Agent` tool calls
    line 61 / 86 / 117   system `task_started` x3 (each after `background_tasks_changed`)
    line 124             assistant text = result block 1
    line 142             system `task_notification` status completed (sub-agent 1)
    line 143             system `init` again                       <- re-wake 1
    line 162             assistant text = result block 2
    line 191             system `task_notification` completed (sub-agent 2)
    line 192             system `init` again                       <- re-wake 2
    line 196             system `task_notification` completed (sub-agent 3)
    line 313             assistant text = result block 3
    line 314  result  num_turns 11  origin absent
    line 315  result  num_turns 3   origin {"kind": "task-notification"}
    line 316  result  num_turns 18  origin {"kind": "task-notification"}
  all three: subtype success, is_error false, stop_reason end_turn,
  terminal_reason completed, permission_denials [] — the first is field-for-field
  indistinguishable from a final one except for the missing `origin`, which the
  helper does not print.

  the session did not stop where block 1 says it waits (invocation log, same out-dir):
    42  22:30:15Z  exit 0  milestone provision bound-what-the-service-will
    48  22:33:58Z  exit 0  milestone finalize  bound-what-the-service-will
    HEAD cb4a90f "Finalize milestone bound-what-the-service-will (3 sub-tasks)"

after
  nothing written under ~/out or ~/ideas; `git status --porcelain` in the out-dir shows only
  the four untracked rig files it had before (.session-transcript/, PROVENANCE.txt,
  stderr.txt, stream.jsonl). The helper body was run from a scratch copy.
```

### The contract

- `protocol/runbook.md` §4 introduces the helper as "a headless turn's **final message**, and how
  many tool calls were denied" — singular. The helper has no notion of *final*: it prints each
  `result` event it meets, oldest first.
- `protocol/protocol.md` §8.1 step 4: "Read each turn's final message, and whether it ends
  asking." §3.3's A-4 is "the conversation ends asking, or on a denial". Neither the protocol nor
  the runbook says a stream may hold more than one `result` event (searched both for
  `background`, `task-notification`, `result event`, `last message`: the only hit is the helper's
  comment).
- The saved evidence shows the hazard as the operator met it: `rubric/arm-a.txt`, under
  `### $ result $A2`, carries the three blocks back to back at lines 154–170 with three
  `-- denials: 0` lines (155, 157, 170) and no marker for which is final.
- The driver itself reads a multi-result stream the other way round, **last wins**:
  `completions/trial-driver/driver/observe.py` → `halted_awaiting_human` and `ended_asking` both
  overwrite `result` on every `result` event and judge what is left after the loop
  (`test_session.py` reads `results[-1]`). Called read-only on this stream both return
  `(False, '')`. So the runbook's reading aid and the driver's detectors disagree on which event
  is *the* message, and only the reading aid leaves it to the reader.

### What it did not cost

- No class was assigned from it: `rubric/arm-a.txt` states "no class is assigned in this file",
  and §3.3's A-classes key on the invocation log (`milestone provision` / `milestone finalize` at
  exit 0), which is unambiguous here.
- A reader who took block 1 as final would read arm (a) as *provisioned, waiting* — the shape of
  A-2 — against a log that says A-1's boundary was reached. The log corrects it; the helper does
  not.
- Neither block 1 nor block 3 contains a question mark, so `ended asking` is the same under
  either reading. The hazard is to the human reader of step 4, not to `observe`.

### Precision on the cause

"Background sub-agents re-woke the session" holds, with one refinement: three sub-agents, **two**
re-wakes. Result 1 is the prompt's own turn ending while the sub-agents ran; results 2 and 3 are
the two notification-driven turns (`origin.kind = task-notification`); the second and third
sub-agents' notifications (lines 191, 196) were absorbed by one wake. The three `result` events
are written as the stream's last three lines (314–316), not where their turns ended (their texts
are at 124, 162, 313) — so the helper's output order is the only ordering a reader gets.

### Adjacent, read from the code and NOT driven

`halted_awaiting_human` takes `permission_denials` from the last `result` event only. Whether a
headless stream's later `result` event repeats an earlier one's denials is not established by
this trial (every event here has zero), so whether last-wins can drop a denial is open — a lead,
not a finding.

### Tier

n/a — tooling. The defect is in this trial's runbook reading aid, not in the jigc product and not
in the committed trial driver.

### Pin

UNPINNED: the `result` helper is a shell function that exists only in this trial's
`protocol/runbook.md` (no copy under `completions/` or `dev/`), so no suite reads it; and the
driver has no fixture with more than one `result` event — `test_observe.py` builds single-result
streams, so the last-wins behaviour of `halted_awaiting_human` / `ended_asking` on a
notification-woken stream is true by construction and fenced by nothing.

---

## T-15 · Arm (a)'s three background sub-agents all started in the main agent's shell directory — a sibling's worktree for two of them; the isolation came from the main agent's prompts relaying a `cd` that jigc's resume refusal had printed, not from `Spawn:` lines — **PARTIAL**

`tier:` n/a

**Kind:** tooling (the trial harness's pinned Claude CLI, and this trial's protocol) ·
**Door:** `completions/trial-harness/Dockerfile` (`ARG CLAUDE_VERSION=2.1.233`, `WORKDIR /work`);
`protocol/protocol.md` §3.3 (the *who worked the pieces* note beside class A-1) · **Tier:** n/a

### The claim

Under the pinned CLI a background sub-agent starts in the main agent's current shell directory.
All three of arm (a)'s started in one sibling's worktree, and one worked in the right worktree
only because that inherited directory was its own — so A-1's *worked in isolation* rests on the
main agent's prompts, not on anything jigc printed.

### Verdict

**PARTIAL.** The observations hold on the saved transcripts; two parts of the claim reach further
than the evidence does.

Carried:

- All three sub-agent transcripts record one start directory,
  `/work/.jigc/worktrees/reject-a-wire-line-longer`, on every entry (30 + 33 + 23 entries), and
  that is the main agent's shell directory at each of the three launches. For two of them it is
  printed by the sub-agent's own bare `pwd`; for the first it rests on the recorded field alone
  (its first command was already `cd <own> && pwd`).
- All three were launched in the background (`isAsync: true`, `status: async_launched`).
- The third sub-agent ran no `cd` at all; its five Bash commands used relative paths
  (`cat src/ingest.ts`, `npm test`, `git add …`) and landed in its own worktree because the
  inherited directory was its own.
- The first two reached their own worktrees through an explicit `cd`, and each main-agent prompt
  opens with the worktree's absolute path and *"cd there first"*.
- No `jigc milestone execute` was run in either turn and the string `Spawn:` occurs zero times in
  either turn's `stream.jsonl`.
- The A-1 class itself does not move: every `Edit`/`Write` of each sub-agent is under its own
  `.jigc/worktrees/<id>/`, and the main agent's own check after the fact shows each worktree
  holding only its own two staged files.

Not carried:

- **"not on anything jigc printed" is false as worded.** The falsifying datum is invocation
  record 41 (`start --task cap-the-store-at-1000 --format json`, exit 1, 22:30:12Z): its refusal
  prints *"a sub-task's work happens in its own worktree … then
  `cd /work/.jigc/worktrees/cap-the-store-at-1000` and re-run this command there"*. The main
  agent's *"cd there first"* is that jigc route relayed and generalised to all three paths. What
  holds is narrower, and is what `scoring/score.md` row T-15 actually says: jigc printed nothing
  **to the sub-agents** and no `Spawn:` line was ever produced. The second sub-agent's bare
  `jigc start` (record 46) lists the three sub-tasks and names no worktree path.
- **"Under the pinned CLI …" is a rule read off one session.** Seventeen sub-agent transcripts
  exist under `~/out`, all on CLI `2.1.233`; in all seventeen the sub-agent's recorded directory
  equals the main agent's at launch, so there is no counter-instance. But fourteen of those were
  launched with the main agent at `/work`, where *inherits the shell directory* and *starts at
  the project root* are indistinguishable. The only discriminating cases are these three — one
  session, one launch directory, all background. A foreground sub-agent launched from outside
  `/work` was never observed. The scorer's live repro was **not driven**: it needs a model-backed
  container session and a new out-dir, which is not one of the tooling's read-only paths.
- **"only because"** is a counterfactual. The fact is that the third sub-agent never ran `cd`;
  what it would have done had its first `pwd` printed a sibling's path is not in the evidence
  (the second sub-agent, in exactly that position, noticed and moved).

### Repro

```
setup
  out-dir        ~/out/RC24-A-frozen-work/turn02   (read-only; nothing written under it)
  PROVENANCE     image jigc-gate:registry-1.0.0-rc.24 · jigc 1.0.0-rc.24 · exit-code 0
  transcripts    .session-transcript/projects/-work/<session>.jsonl          (main)
                 .session-transcript/projects/-work/<session>/subagents/     (3 × .jsonl + .meta.json)
  CLI            every entry of all four transcripts: "version": "2.1.233"
  pin            completions/trial-harness/Dockerfile:97  ARG CLAUDE_VERSION=2.1.233
                 completions/trial-harness/Dockerfile:146 WORKDIR /work

argv 1  (the main transcript: the recorded cwd per entry, and every tool call)
  for each line: print `cwd` when it changes; print each Bash command / Agent launch

observed 1
  line 134  22:30:18Z  cwd -> /work/.jigc/worktrees/cap-the-store-at-1000
            (after `cd /work/.jigc/worktrees/cap-the-store-at-1000 && jigc start --task …`)
  line 139  22:30:24Z  cwd -> /work/.jigc/worktrees/reject-a-wire-line-longer
            (one command: `cd …/reject-a-sample-whose-series && jigc start …`, then
             `cd …/reject-a-wire-line-longer && jigc start …` — the last `cd` wins)
  line 152  22:31:03Z  Agent "Cap store at 1000 distinct series"     input keys: description, prompt
  line 153             toolUseResult isAsync true · status async_launched · cwd …/reject-a-wire-line-longer
  line 154  22:31:17Z  Agent "Reject series names over 128 chars"
  line 155             toolUseResult isAsync true · status async_launched · cwd …/reject-a-wire-line-longer
  line 156  22:31:34Z  Agent "Reject wire lines over 1024 chars"
  line 157             toolUseResult isAsync true · status async_launched · cwd …/reject-a-wire-line-longer
  each prompt opens: "You're working in the git worktree at /work/.jigc/worktrees/<own id>
                      (cd there first — do all work inside this directory, not /work itself)."
  line 163  22:31:41Z  cwd -> /work   (the first completion notice arrives)

argv 2  (each sub-agent transcript: distinct `cwd` values, Bash commands, Edit/Write/Read targets)

observed 2
  sub-agent              entries  recorded cwd (all entries)                    Bash commands            file targets
  store   (…eeb15808)       33    …/worktrees/reject-a-wire-line-longer         4 of 4 open `cd <own> &&`   7 of 7 under cap-the-store-at-1000
  series  (…392a58c)        30    …/worktrees/reject-a-wire-line-longer         2 bare, then 3 `cd <own> &&` 4 of 4 under reject-a-sample-whose-series
  wire    (…ed32f40b)       23    …/worktrees/reject-a-wire-line-longer         5 of 5 bare, no `cd`         2 of 2 under reject-a-wire-line-longer

  series, first command:   ls /work/.jigc/worktrees/ 2>&1; echo "---"; pwd
           output:         cap-the-store-at-1000 / reject-a-sample-whose-series / reject-a-wire-line-longer
                           ---
                           /work/.jigc/worktrees/reject-a-wire-line-longer
  series, closing note:    "the actual work was performed in the worktree
                           `/work/.jigc/worktrees/reject-a-sample-whose-series` (the path your
                           instructions named), not `/work/.jigc/worktrees/reject-a-wire-line-longer`
                           (the harness's default cwd, which belongs to the sibling …)"
  wire, first command:     pwd && cat src/ingest.ts
           output line 1:  /work/.jigc/worktrees/reject-a-wire-line-longer
  store, first command:    cd /work/.jigc/worktrees/cap-the-store-at-1000 && pwd && ls
           output line 1:  /work/.jigc/worktrees/cap-the-store-at-1000
           (no bare `pwd`: its start directory is the recorded field only)

  the recorded cwd of a sub-agent never moves, although store and series ran `cd`; in the main
  transcript the same field follows every `cd`. In a sub-agent transcript the field is where it
  STARTED, not where it worked.

argv 3  (what jigc printed — ~/out/RC24-A-frozen-work/turn02/.jigc/logs/invocations.jsonl, 48 records)
  41 22:30:12Z exit 1  start --task cap-the-store-at-1000 --format json
  42 22:30:15Z exit 0  milestone provision bound-what-the-service-will --format json
  43 22:30:18Z exit 0  start --task cap-the-store-at-1000 --format json
  44 22:30:24Z exit 0  start --task reject-a-sample-whose-series --format json
  45 22:30:24Z exit 0  start --task reject-a-wire-line-longer --format json
  46 22:31:30Z exit 0  start                                   <- the series sub-agent's
  48 22:33:58Z exit 0  milestone finalize bound-what-the-service-will --format json
  no `milestone execute` record; `Spawn:` occurs 0 times in turn01/stream.jsonl and turn02/stream.jsonl

observed 3   (the falsifying datum for "not on anything jigc printed")
  record 41's output, as the main transcript holds it:
    "… a sub-task's work happens in its own worktree, cut from that base rather than in this
     checkout: run `jigc milestone provision bound-what-the-service-will` — it cuts the worktree
     this sub-task is missing — then `cd /work/.jigc/worktrees/cap-the-store-at-1000` and re-run
     this command there"
  record 42's output: "provisioned 3 worktree(s) for milestone:bound-what-the-service-will at base
     a9f1851 (cap-the-store-at-1000, reject-a-sample-whose-series, reject-a-wire-line-longer)"

observed 4   (the isolation held on the tree — main transcript line 177, 22:32:09Z)
  in reject-a-sample-whose-series:  M  src/validate.ts   M  test/validate.test.ts
  in reject-a-wire-line-longer:     M  src/ingest.ts     M  test/ingest.test.ts
  in cap-the-store-at-1000 (line 166, `git diff --cached --stat`): src/store.ts, test/store.test.ts

argv 4  (the control: every sub-agent transcript under ~/out, start directory vs the main
         agent's recorded directory on its launch result)
observed 4b
  12  CLI 2.1.233 · background · main at /work           · sub == main
   2  CLI 2.1.233 · foreground · main at /work           · sub == main   (one is ~/out/RC24-env)
   3  CLI 2.1.233 · background · main outside /work      · sub == main   (these three, one session)
   0  sub != main
  17 transcripts in 11 out-dirs; 14 do not discriminate inheritance from a project-root default.

argv 5  (the tooling's own read-only path)
  cd completions/trial-driver && python3 run.py observe ~/out/RC24-A-frozen-work/turn02
observed 5   exit 0
  session           recs wrote  VERB  adj  fs  outcome
  turn02               9     0     0    0   0  VOID unmeasured — no authoring occasion existed
  (matches rubric/observe-a.txt's turn02 row; no column reports where a sub-agent ran)

not driven
  the scorer's live repro (a headless session that `cd`s into a worktree, launches a sub-agent
  and reads its first `pwd`): it needs `completions/trial-harness/run-session.sh`, a model-backed
  container run that writes a new out-dir — not a read-only path. The saved session is itself one
  instance of that repro (observed 1 + observed 2); it is not an independent second one.

after
  nothing modified under ~/out or ~/ideas; no repository file touched; one scratch file outside
  both (the observe output).
```

### The contract

- `protocol.md` §3.3 fixes A-1's evidence as *"`milestone provision` at exit 0; work for ≥ 2
  sub-tasks done inside `.jigc/worktrees/<id>/`; `milestone finalize <id>` at exit 0"*, and
  records *who worked the pieces* beside the class, *"never changing it"*, defining the sub-agent
  case as *"sub-agent transcripts in a turn's out-dir whose commands run inside the worktrees"*.
  Read against that: the class stands, and the note is satisfied by the commands and file
  targets (observed 2), not by the transcripts' `cwd` field.
- The same section already bounds what the arm can show: *"Neither is the genuine concurrent
  spawn M55's audit ran from a main session, and a headless arm cannot be made to be."* The
  route to `provision` here was the resume refusal, so the `Spawn:` lines — reached only through
  `milestone execute` (§3.2, *"One bound of that state"*) — were never on the worker's path.
- The harness states no contract for a sub-agent's start directory: the Dockerfile pins the CLI
  version and sets `WORKDIR /work`, and neither `run-session.sh` nor the driver names one. What
  is observed is the pinned CLI's behaviour, not a harness promise.

### What this means for reading the evidence

A sub-agent transcript's `cwd` field is not a *who worked where* signal: read literally it puts
all three pieces in `reject-a-wire-line-longer`. `rubric/arm-a.txt`'s path census (174 mentions
of that worktree against 34 and 25) is skewed by the same field. *Who worked where* has to be
read from each sub-agent's commands and file targets, as `scoring/rescore.md` does.

### Tier

**n/a** — a tooling lead. It is a fact about the pinned Claude CLI and about how this trial's
evidence is read; it is not a jigc defect and moves no outcome class.

### Pin

**UNPINNED:** nothing under `completions/trial-driver/` reads a sub-agent transcript's `cwd`
(the only `cwd` in the driver is `driver/session.py`'s `subprocess.run(…, cwd=…)`; the fixtures
in `test_session.py` hard-code `"cwd": "/work"`), `run.py observe` reports no per-sub-agent
directory, and the behaviour belongs to the pinned CLI — a pin would need a live model-backed
session.
