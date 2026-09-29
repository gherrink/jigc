# Findings verification — the project-alpha-4.0 rc.9 trial vs the rc.9 code (2026-07-25)

Every distinct claim from the trial's five feedback reports ([trial-record.md](trial-record.md)) adversarially verified against HEAD `96c318e` (code byte-identical to the installed `jigc 1.0.0-rc.9` except Cargo.toml metadata; confirmed `--version`) by code inspection (file:line cites) plus **live repros** in throwaway environments: a fresh repo (`git init` → `jigc setup` → planted staged state → mint → validate → finalize chains), a fresh repo with `core.hooksPath=.husky/_` and a rejecting pre-commit hook, and a **scratch copy of the trial corpus itself** (rsync'd, runtime dirs excluded — the real corpus is untouched). Verdicts: **CONFIRMED** / **PARTIAL** (real behavior, claim needs correction) / **REFUTED**. Per the delivery-format rule ([milestone-completion-workflow.md](../../../implementation/milestone-completion-workflow.md) → Audit; block schema [pinning.md](../../../implementation/pinning.md) → §3), **CONFIRMED and REFUTED verdicts alike ship repro blocks**; `pinned-by:` names the standing test where one exists, else `UNPINNED` with the reason.

**Scoreboard:** 18 CONFIRMED · 6 PARTIAL · **4 REFUTED** (P1-10 `--unset` exists and is eligible · P1-14 `task diff` exists · P2-5 finalize JSON is pure on stdout — the observation was fd-merging, a class already pinned by `pinned_facts/finalize_json.rs` · P2-6 address-scoped reads exist and resolve, and the log shows the worker never attempted one). **The verified through-line, fifth trial running: the capability usually exists; the surface that should hand it over doesn't** — plus this trial's own signature: **the preview/validate contract is the only place two workers independently hit the same broken promise.**

---

# Part 1 — the cross-session headliners

## H1 — `jigc task validate` does not preview what finalize gates on — **CONFIRMED** (the trial's strongest defect; hit independently by P1 and P2)

The claim appears verbatim on four surfaces: `crates/cli/pack/steps/finalize.yaml:12-13` ("it previews the findings finalize will gate on, without committing anything" — mirrored in `packs/methodology/steps/finalize.yaml:37`), `QUICKSTART.md:138`, `cli.rs:1315`, `task.rs:354-355`. What validate actually runs is `TaskArea::validate` → `engine::validate::validate_task` alone (`task.rs:358-386`); finalize layers **eleven** gates on top that validate never sees (`task.rs:935-1345`), among them the two the trial hit:

- **(a) the carryover gate** — `decide_carryover` is called from exactly two places, `task.rs:1128` (finalize) and `milestone.rs:1751` (join); never from `validate_task`. Trial evidence: P2's `task validate` at log record 288 returned zero carried-path findings while three operator-planted paths were staged; the worker avoided the finalize refusal only by reading the workflow prose and unstaging via raw git.
- **(b) `owner-artifact.present`** — **deliberately relocated out of validate at M45** (`engine/src/validate.rs:276-289`: "the validate-0 / finalize-3 split", DECISIONS → 2026-07-23 Decision 6); it runs post-stage at `task.rs:1908/1921`. Trial evidence: P1's two consecutive finalize `--approve` blocks (log records 62/64) after clean-looking validates.

So half of this finding is a defect (carried-staged was simply never included) and half is **a recorded decision whose surface prose was never scoped** — the M45 Settle moved the gate but left four surfaces still promising a full preview. Either way the promise is now a law-1 lie: the verb whose purpose is "don't find out at finalize" is exactly where two blind workers found out at finalize. The record is rebuttable and this is the re-open data.

```yaml
claim: "task validate previews the findings finalize will gate on (carryover half)"
verdict: CONFIRMED (validate omits the carryover probe; the finalize gate itself works)
setup:
  - git init; commit two files; jigc setup; commit
  - plant: edit+`git add` one file · `git rm` a second · stage one untracked file
  - ["jigc", "start", "--workflow", "single-task", "carryover probe task"]
repro:
  - ["jigc", "task", "validate", "carryover-probe-task", "--format", "json"]
expect:
  exit: 3
  stdout_json: findings = commit-doc blockers only; ZERO finalize.carried-staged
repro2:                       # after authoring the commit doc
  - ["jigc", "task", "finalize", "carryover-probe-task", "--format", "json"]
expect2:
  exit: 3
  stdout_json: exactly three finalize.carried-staged, one per path, the deletion
    with deletion-specific wording ("staged for deletion before this task existed")
repro3:
  - ["jigc", "task", "finalize", "carryover-probe-task", "--carry-staged", "--format", "json"]
expect3:
  exit: 0
  assert: all three paths labeled carried-over in the manifest (JSON + stderr render)
pinned-by: "UNPINNED — the carryover gate has finalize-side tests; the *validate-omits-it* fact and the preview-claim parity have no fence; this block is the red test for whichever fix M46 picks (run the probe at validate, or scope the promise)"
```

## H2 — the migrate-corpus hook-rejection dead end — **CONFIRMED** (forced the trial's only raw commit of jigc-owned writes)

P1: the real husky hook rejected migrate-corpus's commit; the stamp write stayed staged; re-run said "0 would migrate, 1 already current"; no route out. Code (agent-verified): `commit_migration` (`migrate_corpus.rs:323-369`) writes bytes and advances the **file-state baseline per-doc before any commit** (`:759-766`), stages, then commits via `git_commit_capture` — but unlike finalize (`task.rs:1336-1339`, which downcasts `CommitRejected`, prints the survivable frame, and logs `finalize.commit-rejected`), migrate-corpus lets the typed error bubble into the **generic operational-error arm** (`migrate_corpus.rs:212-215`): exit 1, no route, no error code — `ERROR_CODE_REGISTRY` (`invocation_log.rs:71`) has no migrate-corpus member, so the failure is log-anonymous (trial log record 9: exit 1, `error_code: null`). On re-run the stamped docs read `already-current` (`migrate_corpus.rs:441,529`) → `touched` empty → early return, **exit 0, nothing committed, index still loaded**. The M42 survivable-hook-rejection frame was an incomplete fix over the commit-door axis: task finalize got it; the pack's *other* committing door didn't. (M45's complete-fix contract names this exact shape.)

```yaml
claim: "migrate-corpus has no route for its own half-failure under a rejecting hook"
verdict: CONFIRMED
setup:
  - git init; core.hooksPath=.husky/_; jigc setup; commit
  - write a conformant, schema-version-less adr at docs/decisions/foreign-adr.md; commit
  - install a rejecting pre-commit hook (echo "HOOK REJECTS"; exit 1)
repro:
  - ["jigc", "migrate-corpus"]
expect:
  exit: 1
  stderr_contains: "`git commit` was rejected (no commit was made):"
  assert: no route line, no error_code in the invocation log record
  git: the stamped doc is STAGED (M), HEAD unmoved
repro2:
  - ["jigc", "migrate-corpus", "--dry-run"]
expect2:
  exit: 0
  stdout_contains: "0 would migrate, 1 already current"   # done-while-uncommitted
pinned-by: "UNPINNED — this block is the fix's red test (route + error code + a re-run that notices the staged-but-uncommitted state)"
```

## H3 — the AGENT.md validate exit-0 absolute — **CONFIRMED** (a law-1 lie on the preload tier, contradicted by the binary's own trailer)

P5 measured store-scope `jigc validate` exiting 1 (the planted ahead stamp) against `.jigc/AGENT.md`'s "A store-scope `jigc validate` is report-only — it exits 0 even when it surfaces findings." The sentence is the shipped constant `BOOTSTRAP_OUTPUT_CONTRACT` (`adapter.rs:1086`); the exit in fact flips for four classes (`render.rs:538-545`: pack-probe-integrity · reconciliation.rename · schema-version-current · schema-version-ahead; consumed at `cli.rs:771-776`). The doc-comment two lines above the constant even has the correct narrowing ("reports blocking **content** findings at exit 0", `adapter.rs:1074-1076`) — the qualifier never made it into the shipped sentence. Sharpener: **the adapter golden pins the lie** (`adapter.rs:1445,1481` asserts `body.contains("is report-only")`), so the fix must move the golden in the same motion. The binary's own trailer ("the sweep could not adjudicate it and exits non-zero") is correct — preload and output disagree, and the preload is the machine-trusted tier.

```yaml
claim: "AGENT.md: store validate 'exits 0 even when it surfaces findings'"
verdict: CONFIRMED stale (the binary correctly exits 1 on four finding classes)
setup:
  - any corpus with an ahead-stamped managed doc (scratch copy of the trial corpus)
repro:
  - ["jigc", "validate"]
  - ["grep", "-c", "report-only — it exits 0", ".jigc/AGENT.md"]
expect:
  exit: 1            # validate
  grep: 1            # the stale absolute is present in the shipped preload
pinned-by: "the adapter render golden (adapter.rs:1445) — currently pinning the WRONG sentence; regen rides the wording fix"
```

## H4 — the ahead-stamp surface, completed by repro — **the confidence-audit headline fix holds on every door**

Trial-confirmed: blocking `schema-conformance.schema-version-ahead` + exit flip in three sessions (log 372, 412, 427, 498-502), traced by P5 to the plant commit via `git log -S`, never force-edited, never false-greened. The two doors no worker walked, driven on the scratch corpus copy: **`migrate-corpus` blocks** — "1 blocked … stamped schema-version 99, above the current `arch-doc` schema-version 1 … route: upgrade jigc … or restore the stamp from git history" (dry-run and real, exit 1, never "current") — and **`set-field` refuses the forge**: `write.machine-maintained-field`, route "…advanced only by `jigc migrate-corpus`, never a manual write". Also probed and **REFUTED as a fear**: `reconciliation.absorb` of the planted doc (P4's task-scope sweep) cannot mask the ahead detection — the store-scope check re-parses committed bytes, never the file-state record (`validate.rs:891-895`; absorb re-baselines the *drift* hash only, `file_state.rs:213-219`).

```yaml
claim: "the planted ahead stamp is blocked on every tool route (P4 signal c, unexercised arms)"
verdict: CONFIRMED (field + repro union)
setup:
  - scratch copy of the trial corpus (docs/architecture/testing-infrastructure.md stamped 99)
repro:
  - ["jigc", "validate"]                          # exit 1, schema-version-ahead blocking
  - ["jigc", "migrate-corpus", "--dry-run"]       # exit 1, "1 blocked", names the doc + restore route
  - ["jigc", "doc", "set-field", "arch-doc:testing-infrastructure#schema-version", "--value", "1", "--task", "<t>"]
expect:
  assert: write.machine-maintained-field refusal; no door reports the doc current
pinned-by: "schema-version-ahead + settability suites (confidence-audit wave / M45 inc 3); the migrate-corpus block arm pinned at the wave close"
```

---

# Part 2 — P1, the unseeded migration

## P1-1 — owner-artifact home ambiguity (two finalize blocks) — **CONFIRMED** (root stated on one surface, absent on the one the worker was driven by; 2 sessions hit it)

The migrate ladder (`packs/methodology/steps/author-migration-completion-record.yaml:27-51`) prints literal `mkdir -p completions/artifacts/<milestone>` shell with **no statement of which root** and no absolute example — in a corpus where every managed path lives under `docs/`, doc-root-relative is the natural read (the worker's exact failure, twice: "names no file under the repository" → "is not under the owned artifact home", log 62/64). The completion-workflow sibling **does** say "repo-relative" (`author-completion-record.yaml:6-11`) — P5 still reported the ambiguity from the workflow text and resolved it only by finding the v1.0 artifact. The engine finding names the home constant but not the root; the route (`validate.rs:2748-2751`) names neither. Fix-shaped: one absolute-from-repo-root example in the ladder + the route naming the root. *(The validate-blindness half of this pain is H1(b).)*

## P1-2 — "two code-anchor fields, two accepted shapes" — **PARTIAL: the indistinguishability is real; the shape demand did not reproduce**

Confirmed half: `jigc doc schema` labels both `adr#status/cites-code` and `arch-doc#components/<id>/implemented-by` bare `code-anchor` — no per-field predicate visibility, so the schema surface genuinely cannot tell you what a field will demand at validate. Refuted half: live on the corpus copy, `set-field …/implemented-by --value apps/dashboard/app` (bare path, no symbol) was **accepted and validated clean** — no shape demand exists on rc.9. What the worker actually hit (log 189) was `doc-code.symbol-exists` on a `path#symbol` anchor whose symbol didn't resolve mid-migration — a value error, generalized into a shape rule. The actionable residue: project the anchor predicate (symbol-checked vs path-checked) into `doc schema`.

```yaml
claim: "arch-doc implemented-by requires path#symbol (bare path rejected)"
verdict: REFUTED as stated (bare path accepted + task validate clean); the schema-surface indistinguishability is the CONFIRMED residue
setup: [scratch corpus copy, any active task]
repro:
  - ["jigc", "doc", "set-field", "arch-doc:codebase-structure#components/app-directory-by-responsibility/implemented-by", "--value", "apps/dashboard/app", "--task", "<t>"]
  - ["jigc", "task", "validate", "<t>"]
expect: { write: accepted, validate: no doc-code finding on that field }
pinned-by: "UNPINNED — pin as a refuted-claim fact (bare-path code-anchor accepted on both doctypes) in pinned_facts/"
```

## P1-5 — `--preview` advertised as general, refuses non-minting workflows — **CONFIRMED** (hit independently in P1 and P5)

Orientation (`render.rs:119`) and `describe` (`render.rs:2536` — "any one's") advertise preview unqualified; `start.rs:783-787` refuses the four `creates-task: false` members (`router`, `milestone-execution`, `ingest-existing`, `increment`) with "run it directly: jigc start --workflow <id>" — for `ingest-existing` that is the very next line orientation prints (log record 1, the trial's second invocation, exit 1; again at 414). The refusal's advice is sound (running a non-minting workflow *is* the safe read) but the advertising is a law-2 miss: the surface offers an affordance a quarter of the catalog rejects. Fix-shaped: qualify the two advertising lines, or make preview degrade to the direct compose for non-minting members.

```yaml
claim: "--preview refuses workflows the orientation just offered it for"
verdict: CONFIRMED
setup: [any jigc repo]
repro: [["jigc", "workflow", "ingest-existing", "--preview"]]
expect: { exit: 1, stderr_contains: "mints no task, so there is nothing to preview" }
pinned-by: "compose-golden sweep (preview refusals are goldened stderr+exit — pinning[.md] §1); the *advertising* line parity is UNPINNED"
```

## P1-6 — `title-names-symbol` misdescribes its finding — **CONFIRMED** (verbatim)

`target_surface.rs:455-464`: "…the heading still names a renamed/removed symbol; update the title to match the code", fired off a pure capitalization heuristic (`is_compound_identifier`, `:532-541` — camel-hump or acronym-then-word); nothing is ever resolved against code history, so "renamed/removed" is asserted, not detected. The in-code comments already concede the false-positive class (`:433-440` — `WebSocket`, `WordPress`). The trial's instance ("APIs" flagged as a symbol) is exactly the acronym arm. Advisory severity (M40 demotion) held — cost was attention, not a block — but the wording sent the worker hunting a code change that never happened. Pure law-1 wording fix at the message site.

## P1-7 — setup names a hook path it didn't use — **CONFIRMED** (install right, message wrong, golden pins the wrong message)

`install_precommit_hook` resolves the real hooks dir through `git rev-parse --git-path hooks` honoring `core.hooksPath` (`setup.rs:270-273, 477-489`) — and the renderer prints a **hardcoded** `.git/hooks/pre-commit` (`render.rs:1517`); `SetupSummary` carries no hook-path field to interpolate (`setup.rs:650-659`), and a setup golden pins the hardcoded string (`render.rs:4034`). Live repro (fresh repo, `core.hooksPath=.husky/_`): message says `.git/hooks/pre-commit`; the file exists only at `.husky/_/pre-commit`. The worker checked the printed path, found nothing, and briefly concluded the backstop was inert — the exact trust-erosion shape of rc.6's A14.

```yaml
claim: "setup reports pre-commit -> .git/hooks/pre-commit under core.hooksPath=.husky/_"
verdict: CONFIRMED (jigc installs to the right place and reports the wrong one)
setup: [git init; mkdir -p .husky/_; git config core.hooksPath .husky/_; seed commit]
repro: [["jigc", "setup"]]
expect:
  stdout_contains: "pre-commit hook → .git/hooks/pre-commit"
  fs: .husky/_/pre-commit exists; .git/hooks/pre-commit does not
pinned-by: "setup golden render.rs:4034 — currently pinning the lie; fix threads the resolved path through SetupSummary and regens"
```

## P1-8 — the drift hook cries wolf — **CONFIRMED**

The installed hook greps the validate JSON for `"probe": "doc-code"` only (`setup.rs:194-196`) — severity is never inspected, so three cosmetic `title-names-symbol` advisories print the same "doc<->code drift detected in committed docs" as a genuine blocking anchor break; it fired on four consecutive P1 commits. Fix-shaped: match blocking severity within the doc-code probe (one grep clause).

## P1-10 — "no way to clear an optional field" — **REFUTED** (the flag exists, is documented in `--help`, and is eligible for exactly that field)

`set-field --unset` shipped at M41 (`doc.rs:159-166`; help: "Clear the field entirely — … Refused for author-required / defaulted / CLI-`set:` fields"). The item-field arm exists (`apply_unset_target` → `unset_item_field_validated`, `doc.rs:609-635`); the eligibility predicate (`write.rs:5755-5784`) refuses only `set:`-stamped, author-required, or defaulted fields — `implemented-by` (`arch-doc.yaml:41`) is none of those, so the exact operation the worker wanted (and worked around with remove-item + re-add + re-author) was one flag away. The shipped acceptance test clears the same field class (`set_field_unset.rs:151`, over `adr#status/cites-code`). Discoverability residue: no workflow/step text names `--unset`, and the worker never opened `set-field --help`. *(Live sharpener: `--unset` on an item whose field-group is absent fires `write.not-present` **with** the M44 containing-section route — the good route the wrong-item-id path lacks; see P5-4.)*

```yaml
claim: "there's set-field but no unset/--clear"
verdict: REFUTED
setup: [scratch corpus copy, active task]
repro:
  - ["jigc", "doc", "set-field", "--help"]                    # documents --unset
  - ["jigc", "doc", "set-field", "adr:keep-creator-wizard-state#status/cites-code", "--unset", "--task", "<t>"]
expect: { exit: 0, ack: "unset" }
pinned-by: set_field_unset::unset_clears_an_optional_scalar_byte_stable_and_reconforms
```

## P1-11 / P3-4 — no item ordering surface — **CONFIRMED** (2 sessions; append-only by construction)

`add-item` takes exactly `addr` + `--title` + `--task` (`doc.rs:104-115`) — no position flag; no reorder verb exists in the whole command tree; insertion is append-after-last by construction (`write.rs:2756-2758`). For the roadmap — whose own description says "running milestone spine" — P3 is right that a spine has an order the surface can't express; P1's remove+re-add landed at the end by luck. Capability-shaped (the invariant "ordering lives in a separate ordered list" makes a reorder op cheap in principle); M46 Settle material.

## P1-12 / P3-2-adjacent — no idempotent re-author / no dry-run — **CONFIRMED as capability gaps** (the underlying safety is better than the workers believed — see P3-2)

`doc author` has no `--dry-run`; a committed-doc author payload is add-only (item-title collision → whole-payload atomic reject, below); post-write inspection exists (`task diff`, `doc show --task`) but nothing pre-stages. Known-adjacent: [ideas/batch-authoring-ergonomics.md](../../../ideas/batch-authoring-ergonomics.md).

## P1-14 — "no task-scope diff for non-migration tasks" — **REFUTED** (P3 self-corrected the same day)

`jigc task diff <id>` exists (`task.rs:199-203`): git diff of the worktree vs the task's pinned base + full dump of staged managed docs. P3's report even concedes "the pre-commit case was covered and I missed it." Caveats worth carrying (from the trace): it ignores `--format json` (the one task verb outside the envelope), and part 2 is a dump, not a diff-vs-committed — which is P5-8's residue ask.

```yaml
claim: "non-migration tasks have no what-will-this-change surface"
verdict: REFUTED
setup: [any repo, active task with a staged doc edit]
repro: [["jigc", "task", "diff", "<t>"]]
expect: { exit: 0, stdout_contains: ["# code changes vs base", "# staged docs"] }
pinned-by: "UNPINNED — and the format-json bypass is a machine_output completeness gap worth a row in the M46 fix"
```

---

# Part 3 — P2, the carryover session

*(P2's headline — the validate hole — is H1; the gate's own conduct is trial-record → P2 row.)*

## P2-1 — the commit doc is invisible until it fails — **CONFIRMED**

`record-decision` composes `author-adr` + `superseded-context` + dev `finalize` (`record-decision.yaml:7-9`); none of the three steps mentions `commit:<task>` or its author-required `type`/`summary` (`commit.yaml:18,21`) — the worker met them as two blocking findings at validate. Sharp edge found by the trace: the methodology pack's *own* finalize step **does** carry the commit-doc line ("finalize renders the commit doc; it does not fill it, so set its header and prose first", `packs/methodology/steps/finalize.yaml:12`) — but body references resolve per-owning-pack (`pack.rs:1106-1113`), so dev-pack workflows get the dev finalize step, which lacks it. An axis miss in pack-text space: the statement exists in one pack's step and not its sibling's. Fix-shaped: the dev finalize step gains the same line (or the `{{schema:commit}}` seam).

## P2-3 — `start "<intent>"` is a mandatory no-op round trip — **CONFIRMED, by design** (lacon B1 stands; 3rd trial)

The router is deliberately model-free (the CLI-makes-no-LLM-calls invariant); the menu print *is* the routing surface. The papercut remains real in session-hook contexts (the SessionStart hook already printed the same catalog). Standing re-open data for [ideas/spec-router-matching.md](../../../ideas/spec-router-matching.md); no new scope.

## P2-5 — "`--format json` output is not pure JSON" — **REFUTED** (fd-merging in the reader, not a contract breach; the class is already pinned)

The left-out block goes to **stderr** under `--format json` by explicit branch (`emit_left_out_advisory`, `task.rs:2593-2602`: "the structured envelope owns stdout and must not be corrupted"); same discipline for the carried advisory and hook relay; the branch has existed since the print was introduced (M42, `87285ec`). Live repro (H1's repro3 state, non-empty left-out AND carried sets): stdout parsed as exactly one JSON document with `committed.left_out` populated; every human line was on stderr. There is a standing pinned-fact suite for **exactly this claim class** — `pinned_facts/finalize_json.rs`: "'finalize `--format json` emits non-JSON' — REFUTED as an fd-routing bug… agent harnesses merging the two streams." The P2 worker read a merged terminal view (their own transcript shows the lines then the JSON — a tty view) and inferred stdout. One honest bound: `machine_output.rs:228`'s finalize fixture leaves nothing out, so the non-empty-left-out arm rode this repro, not a standing test.

```yaml
claim: "task finalize --format json emits human lines before the JSON; naive | jq fails"
verdict: REFUTED (stdout pure; the lines are stderr)
setup: [repo with an active task, unstaged extra files present]
repro:
  - sh: jigc task finalize <t> --carry-staged --format json >out.json 2>err.txt
expect:
  stdout: parses as exactly one JSON document (committed.left_out populated)
  stderr_contains: "finalize — committing the index; leaving out:"
pinned-by: "pinned_facts/finalize_json.rs (the class); the non-empty-left-out arm UNPINNED — worth adding to machine_output.rs"
```

## P2-6 — "no address-scoped reads; the write side has addressing, the read side has none" — **REFUTED** (the reads exist; the log shows zero attempts; the miss's error route is the real finding)

`doc show` resolves `#section`, `#section/<item-id>`, and `#section/<item-id>/<leaf>` (grammar `address.rs:6-12`, slicing `store.rs:373-482` — M40 A2 "the write-grammar address accepted on show", M42 field-leaf + bare-singleton). Live on the corpus copy, the exact read the worker wanted — the deferral-ledger entry — resolves at `deferral-ledger:deferral-ledger#entries/creator-wizard-state-lives`. The P2 log segment (records 278–296) contains **no `doc show` with a fragment at all**: the worker guessed the grammar would be the bare item id, never ran it, declared the capability absent, and fell back to `sed` ranges. The half that IS fix-shaped: the bare-fragment miss fires `store.no-such-section` with route "name a section that exists in the committed doc" (`store.rs:381-388`) — it neither teaches the `#section/<item>` form nor enumerates the real section ids, so the one probe that would have corrected the guess teaches nothing (the same route-quality gap as P5-4).

```yaml
claim: "doc show cannot address a section/item; only whole-doc dumps"
verdict: REFUTED (both forms resolve); the miss-route quality is the CONFIRMED residue
setup: [scratch corpus copy]
repro:
  - ["jigc", "doc", "show", "deferral-ledger:deferral-ledger#entries/creator-wizard-state-lives"]
  - ["jigc", "doc", "show", "deferral-ledger:deferral-ledger#creator-wizard-state-lives"]
expect:
  first: { exit: 0, stdout_contains: "{#creator-wizard-state-lives}" }
  second: { nonzero exit, finding: store.no-such-section, route_teaches_grammar: false }
pinned-by: "doc_read_surface.rs address-grammar suite (the resolving forms); the miss-route wording UNPINNED"
```

## P2-7 — no search across managed docs — **CONFIRMED, known** (now demanded in three sessions of one trial: P2 §3, P4 §3/§4 — where it drove the trial's largest sanctioned-bypass cluster — and P5's bulk stamp-grep)

No `doc grep`/search verb (command tree, B-trace). Tracked: [ideas/doc-search.md](../../../ideas/doc-search.md) — this trial is its strongest demand yet: P4's AGENT.md-violating `grep -rn docs/` was *caused* by the gap ("reading nineteen docs one at a time wasn't viable"), i.e. the missing verb is now producing adapter-rule violations in otherwise-compliant workers.

## P2-8 — no deferral↔ADR settlement path; record-decision never prompts about contradicted deferrals — **CONFIRMED** (capability-shaped, M46 Settle input)

`supersedes` is adr→adr; no resolved-by relation exists; nothing in record-decision sweeps open ledger entries that the new decision contradicts. The worker's framing is exactly right: record-decision is the workflow most likely to invalidate a deferral. Adjacent tracked idea: [ideas/adr-lifecycle-extensions.md](../../../ideas/adr-lifecycle-extensions.md).

---

# Part 4 — P3, the batch-author session

## P3-1 — task ids are a silent function of the intent prefix — **CONFIRMED** (dead-id reuse silent; the live-collision fear REFUTED)

Trial log: the planning task at record 310 and the record-change task at 321 both minted `back-fill-the-pre-v1` from different intents sharing five words — silent reuse, exactly as reported (the first was finalized, so the dir was gone; `state.rs:512-517` checks only `dir.exists()`; no retired-id ledger, no warning). The worker's stated fear — "I could not tell whether I had… collided with a live one" — is refuted for the live case: a live collision hard-rejects (`task.serial-collision`, "task `X` is already active" + resume/discard route, live-reproduced). M44's path-hash fixed migration ids only; intent ids remain 5-word slugs. `--slug` on start exists (M39) — one more pull-tier miss — but no pre-mint warning that an id was recently used. Fix-shaped residue: a "previously used by a finalized task" note at mint, or fold a disambiguator on collision-with-history.

```yaml
claim: "minting over an existing id is silent"
verdict: PARTIAL (live id: loud reject; finalized id: silent reuse, no note)
setup: [any repo]
repro:
  - ["jigc", "start", "--workflow", "quick-fix", "probe id collision one alpha extra"]
  - ["jigc", "start", "--workflow", "quick-fix", "probe id collision one alpha bravo"]
expect:
  second: { blocking: task.serial-collision, route_contains: ["start --task", "task discard"] }
pinned-by: "state.rs serial-collision contract (doc'd state.rs:495-498); the dead-id-reuse silence UNPINNED (it is the claim)"
```

## P3-2 — the append-vs-update contradiction — **CONFIRMED as prose; the feared hazard does not exist** (this is the trial's cleanest "two surfaces, two stories" instance)

Five pack steps say author over a committed doc "appends — existing entries are untouched" (`author-roadmap.yaml:20-21`, `author-change.yaml:35-36`, + ledger/decisions/decision siblings); `doc author --help` says "copies it in and updates it" (`doc.rs:199-200`); a third spelling rides the create ack ("copied in for update"). The actual semantics, live-reproduced: a payload item whose slug collides with a committed item **hard-rejects the entire author atomically** (`write.already-present`, engine `write.rs:2765`, whole-batch rollback `doc.rs:2046-2050`) with an edit-in-place route — so neither story is right where they conflict: items neither append duplicates nor update in place, and doc-level `set:` leaves *do* overwrite. The worker's defensive append-only ordering (and the inverted roadmap it produced) was caused by prose, not by real risk — the experiment they declined was safe *and* the reject would have answered them. The "author only once per task" reading is PARTIAL: a second author against the same staged doc rejects, but iteration continues freely through set-slot/set-field/add-item.

```yaml
claim: "cannot tell whether author over a committed roadmap appends or updates; retry forbidden"
verdict: CONFIRMED (prose contradiction); behavior = add-only with atomic already-present reject
setup: [scratch corpus copy; planning task]
repro:
  - author payload re-listing committed title "v1.1 Build & Test Foundation"
  - ["jigc", "doc", "author", "roadmap", "--from-file", "payload.yaml", "--task", "<t>"]
expect:
  exit: 1
  finding: write.already-present ("edit it in place (`set-field`/`set-slot`) instead")
  assert: no partial staging (whole payload rolled back)
pinned-by: "engine write.rs:4365 (add_item collision); the author-batch rollback + the PROSE parity UNPINNED — the wording fix regen rides the compose goldens"
```

## P3-3 — the stale-copy note named my own task — **PARTIAL** (real note, wrong verb attributed, and it can name the reader)

The string exists once, fires only on **task-less `doc show`** (`stale_read_hint`, `doc.rs:2196-2199`, called at `doc.rs:2148`; the `--task` branch returns before it) — never on `doc create` as reported (create's only extra ack is "already existed — copied in for update"). The note enumerates **all** active staging tasks with no self-awareness (there is no requester identity on a task-less read to exclude), so a worker mid-task reading committed state is warned about themselves — technically true, includes the correct staged-read command, and still reads as a third-party alarm. Fix-shaped: when exactly one active task stages the doc, phrase it as "your open task <T> stages a newer copy — read it with `doc show … --task <T>`".

## P3-7 — a human-gated checkpoint walked through with no record — **CONFIRMED, known-tracked** (the checkpoint/gate-record deferral's next independent demand — 4th trial)

Planning's Settle is prose-only; nothing records that it was walked or skipped (project-alpha-3.0 session 3 + lacon + rc.5 hit the same). Tracked at [implementation/decisions-pending.md](../../../implementation/decisions-pending.md) (checkpoint/gate-record mechanism); this trial adds the sharpest instance yet — a back-fill with genuinely nothing to settle still *silently* passes the phase whose entire purpose is being a gate.

## P3-5 / P3-8 / P3-9 — small confirmations

`set-slot --append` doesn't exist (replace-only; shell round-trip is the only path) — capability papercut, batch-ergonomics family. `set-slot` outside the repo correctly errors but could resolve the repo from `--task` — trivial, by-design today. `doc show` on a leaf emitting clean, footer-free bytes is real and **documented only in code** (`store.rs:365-372` "declared slot prose byte-for-byte") — worth one sentence in `doc show --help` so round-trip scripting rests on a stated guarantee.

---

# Part 5 — P4, the designed implementation probe

## P4-3 — `reconciliation.absorb` of an untouched doc rides the task's findings — **CONFIRMED mechanics, by design; the presentation is the finding**

`validate_task` sweeps the **whole committed store** for OOB drift (`reconcile_committed_store`, store-wide iteration `file_state.rs:350-410`; `task_touched` is computed per-doc but gates nothing about inclusion), so the operator's plant appeared in P4's task-scope output as an absorb the worker spent real attention disowning. Durability is finalize-only (`advance_file_state`, `task.rs:2843-2846` — an absorbed baseline the commit never touched persists at the landed finalize). The sub-fear this analysis probed — absorb masking the ahead stamp — is **REFUTED** (H4). Fix-shaped: label store-news findings as not-of-this-task (or partition the render), per the worker's exact ask.

## P4-6 — editing a committed doc from a task stages it silently — **CONFIRMED**

Every edit verb copies a committed doc into the task on first touch (`read_or_copy_in`, `doc.rs:3542-3572`, six call sites) and the ack says only `set <addr> = <value>` (`render.rs:1214`; JSON `render.rs:1155-1158` — no copy-in signal, no promotion notice); only `doc create` states it (`existed: true`, M43 law-1 fix, `render.rs:1228-1232`). Live-reproduced on the corpus copy. The M43 statement was an incomplete fix over the verb axis: the create door tells the truth; the five edit doors stay silent about the same effect. The worker learned their task had promoted a committed ADR from the finalize output.

## P4-5 / P4-7 / P4-8 — the enumeration/labeling papercut cluster — **CONFIRMED** (lacon B6's residue, re-demanded)

`{#id}` anchors ARE the item ids (M42) but no surface says so; no `--addresses`/`--items` projection exists (the worker wrote Python over `--format json`); `doc list`'s three columns are headerless (`doc.rs:2447-2449`; `item-count` silently JSON-only) and `create-gates:` appears in the footer defined nowhere (`render.rs:284-289`). All small, all law-2-flavored, all in the "one `--addresses` projection + three sentences" fix family.

## P4-2 — "touches no documented code" undefined — **CONFIRMED** (wording; the worker's inference — grep the docs for the path — is exactly what the missing search verb P2-7 would answer)

---

# Part 6 — P5, the natural milestone

## P5-4 — wrong item-id fires `write.wrong-shape` with a route that cannot answer it — **CONFIRMED, and it is pinned as intended** (the re-open data is the trial)

`set-field` on a nonexistent item converts the miss to `WrongShape` (`insert_item_field`, `write.rs:3194-3197` — deliberately, "so the two absence cases stay distinguishable") and the generic route sends you to **type-level** `jigc doc schema` (`write.rs:5149-5153`), which cannot reveal that the real id was `phpstan-baseline-has-drifted-from`. M44's containing-section enrichment covers set-slot/remove-item only (`enrich_not_present_route`, `doc.rs:669-686`) — and `write_not_present_route.rs:20-22` **pins the exclusion as intended**. The trial is the field evidence the pinned intent is wrong for this case: the worker guessed an id, got the schema route, and (having truncated the error — their own caveat) burned the round; the recovery that worked was exactly the containing-section read the other path's route names. Live sharpener: the *field-group*-absent miss on the same verb DOES carry the good route — one verb, two miss shapes, two route qualities.

```yaml
claim: "wrong-id set-field appeared to silently no-op"
verdict: PARTIAL (never silent — exit 1 + printed finding, eaten by the worker's `tail -1`); the route-quality half CONFIRMED
setup: [scratch corpus copy, active task]
repro:
  - ["jigc", "doc", "set-field", "deferral-ledger:deferral-ledger#entries/nonexistent-item-xyz/kind", "--value", "Idea", "--task", "<t>", "--format", "json"]
expect:
  exit: 1
  finding: write.wrong-shape, message contains "not present"
  route: "jigc doc schema <doctype>"   # type-level — cannot reveal instance ids
pinned-by: "write_not_present_route.rs:491-499 — pinning the CURRENT route; the fix flips that arm to the containing-section route and moves the pin"
```

## P5-1 — AGENT.md exit-0 lie → **H3**. P5-2 — preview refusal → **P1-5**. P5-9 — owner-artifact root → **P1-1**.

## P5-3 — item-id minting unpredictable, no `--id` on add-item — **CONFIRMED** (slug-rule discoverability, lacon A1's exact re-demand under a fresh worker; the batch pattern sharpener is new)

The rule (5-word cap + stopword drop, slug-rule-version 2) still appears on no user-facing surface; `add-item` has no `--id` (B10); the mitigation that exists — add-item echoes the minted full address — is defeated by the documented batch pattern (add-item + set-field in one script forces capture-or-guess). Tracked: [ideas/slug-minting-ergonomics.md](../../../ideas/slug-minting-ergonomics.md) + [ideas/batch-authoring-ergonomics.md](../../../ideas/batch-authoring-ergonomics.md); the `-333` suffix case in the same session shows even observed ids don't teach the rule.

## P5-5 — the trailing footer makes `| tail` hazardous — **CONFIRMED as ergonomics, no defect** (the diagnostics were printed; the worker's pipe ate them — their own adjudication, verified by log exits 485/486 = 1)

The footer rides stdout on essentially every invocation by design (the routing floor). The worker's derived rule is correct: `--format json` for writes, never `tail -n`. Possible cheap mitigation for M46 consideration: suppress the footer under `--format json` (it already is) and on nonzero exits keep the finding as the last line. No repro block — behavior confirmed from the log's exit codes plus H-class repros above.

## P5-6 / P5-7 / P5-8 — read-surface capability asks — **CONFIRMED** (`doc list` JSON carries `id/path/state/item-count` only — `doc.rs:2476-2489`, no schema-version; no historical read — `git show <sha>:` was the only path for tracing the plant, used by P4 and P5 both; no doc-vs-committed diff — `task diff` dumps staged bodies whole and ignores `--format json`, `task.rs:326-350`). All three are read-surface completeness items for the M46 Settle; the historical read now has two-session demand and was load-bearing for the trial's own forensic task (tracing the plant).

## P5-11 — the truncation-induced changelog gap — **CONFIRMED, worker-attributed** (and the safety net fired)

The worker piped composed output through `head`/`tail` at least six places, missed `single-task`'s record-changelog step, and shipped v1.1 without changelog entries. The gate-granted-unused advisory **did fire** at both single-task finalizes (log records confirm 2 firings in P5) — and was truncated away too. The push tier cannot survive a reader that discards output; this is adapter-preload material (AGENT.md already says "Read every command's output") plus the standing acknowledged-findings-ledger deferral — noise habituation and truncation are the same failure through different doors.

---

# Part 7 — expected signals that did not fire (the protocol's own audit)

- **`finalize.carried-staged`: zero firings in 504 records.** Designed signal, dodged by a prose-warned worker; supplied by H1's repro. The *reason* it was dodged is itself finding H1.
- **The P4 anchor gate never blocked** — the worker repointed through jigc before finalize. Averted-not-fired; the gate's live block stands on the rc.5 fire + flow suites. No new gap.
- **`migrate-corpus` never ran post-plant** — the block arm was unexercised by workers; supplied by H4's repro.
- **`doc-code.symbol-exists` fired once (P1), zero in P4** — P4's split staged the new files and repointed the anchor in the same task, so the anchor resolved at finalize. Consistent with design; no gap.
- **Record 343 is not a worker record** (operator post-plant check) — honesty statement item 2 in the trial record.

# The lens verdict (input to triage)

**Zero data-loss or corruption defects. Zero regressions of prior-wave fixes.** The confirmed set splits: **(a) contract/parity defects on the tool's own surfaces** — the validate preview promise (H1), the migrate-corpus commit door (H2), the preload exit-code absolute (H3), the setup hook-path and title-names-symbol wordings (P1-6/7), the append-vs-update prose pair (P3-2), the silent copy-in acks (P4-6) — every one an *incomplete-sweep sibling of a shipped fix* (the M45 lens, landing on prose/route/ack axes this time instead of write paths); and **(b) capability-shaped asks with accumulating demand** — managed-doc search (3 sessions this trial), item ordering (2), historical reads (2), deferral↔ADR settlement, `--addresses`/`--id`, dry-run — all M46 Settle material, none 1.0-gating on the correctness axis. The four REFUTED rows are all pull-tier failures over shipped capability — the fifth consecutive trial with that signature, now with the sharpest instance yet (a capability declared absent with zero attempts logged).
