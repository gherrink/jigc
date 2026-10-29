# Review of `a7c2d66a` — the opening gathers its facts first

Read-only review of `186bb9e7..a7c2d66a` on `fix/rc24-tier1` (4 files: the new `.claude/agents/stabilize-opening.md`, `DECISIONS.md`, `dev/stabilize-record` one header line, `implementation/stabilization-workflow.md` → *Opening a run* and → *Orchestration*). Read against the brief's ruling of 2026-10-27, `implementation/working-principles.md`, planning's Settle (`implementation/milestone-planning-workflow.md:39`), the record script's `--help`, the step tool, the write guard and its suite, and the harness source.

**Verdict.** The ruling is recorded faithfully in all three places, the quote of planning's rule is exact, and the claim about where the sheet goes holds: I reproduced it myself (below). Three holes are left. (H1) The sheet's outline does not cover every decision the opening takes. (H2) One sentence of the definition lets a gatherer remove an option by calling it "impossible". (H3) The write guard refuses any subagent that places the sheet once `opening.md` is on disk, and the doc does not say who places the sheet or in what order.

---

## Holes

### H1 — The sheet's outline leaves out opening decisions, so the human can be asked about them without facts

- **Location.** `.claude/agents/stabilize-opening.md:12` (*"one section per decision the opening takes, in the doc's order — the closing condition …, the candidate …, the test set …, the declared bounds, the stop mode with the bound across rounds and the default scope, and the ledger's opening rows"*). The list it copies is `implementation/stabilization-workflow.md:88-95`.
- **Evidence.** The ruling asks for *one section per decision the opening takes*. The definition puts that into practice as the six bullets of the doc's *What it decides*. But `dev/stabilize-record --help` → `run-set` (header lines 761-866) accepts more facts than those bullets cover:
  - `hand-on` is "written ONCE (exists)" at the opening.
  - `order` "is written at the opening, before the ledger's first row that names no kind". It is "the human's to set, and this script ships none".
  - `budget` is "written at the run's opening".

  None of these three appears in the doc's bullets, in its *What goes in as data* table (lines 101-107), or in the definition. They are live for this run: the doc's line 701 says the reshaped `test` stage with its queue "is owed before the run's opening". The run's slug and its loop branch are also decided at the opening ("chosen together with the loop branch", line 97). Yet the definition takes both as given in its prompt and **halts when the loop branch does not resolve** (line 29). For a run whose loop branch is new, that decision therefore has to be taken before the sheet exists, which contradicts "no question … before the sheet is back".
- **How the count was reached.** I enumerated the 12 fields `run-set --help` names (stop, rounds, cycles, previous, previous-commit, scope, clauses, condition, budget, hand-on, order, queued) and checked each against the six bullets and the five table rows. The bullets or the table cover 8. `queued` is a ruling taken at any step, so it is not an opening decision. That leaves **3 fields with no section (budget, hand-on, order)**, plus the slug and loop branch from line 97. This class is bounded by the `run-set` enumeration. `item-set`, `bound-set` and `ledger-add` are each covered by a bullet.
- **Smallest correct fix.** The doc's list is the gap, so the doc changes (working-principles: whichever is wrong changes):
  - Add the queue's three facts (where the run has a queue) to *What it decides* and to the data table.
  - Say whether the slug and loop branch come before the gather (handed to the gatherer and outside the rule) or are a section of the sheet.
  - Replace the copied list in the definition with a pointer: "one section per item of the doc's *What it decides* and per fact of its data table". Then a later addition to the doc reaches the sheet without a second edit (see note 2).

### H2 — "Impossible" lets the gatherer remove an option, which is a recommendation by elimination

- **Location.** `.claude/agents/stabilize-opening.md:19`: *"Where a fact makes one option impossible (a tool that does not exist cannot be a clause's instrument today), say that it is impossible and why — that is a fact."*
- **Evidence.** The definition's own example is a judgment, not a fact. A clause whose instrument has no tool can still be decided in several ways: build the tool before round 1, declare a bound, or reshape the clause. Each of those is a choice the human makes. The ruling's own reason, planning's Settle (`milestone-planning-workflow.md:39`), exists because options get settled before the human sees them ("a fork is framed on the options the orchestrator happened to list"). Calling an option impossible removes it from the frame the human answers inside. Item 2 of the same definition (line 15) already records the underlying fact as a fact: *"What does not exist yet … Stated as a fact, never as a plan."* Line 19 then turns that fact into a verdict on an option. **Instance**: one sentence. `command grep -n impossible` over the definition gives 1 hit, at line 19.
- **Smallest correct fix.** Delete the sentence. If anything replaces it: "what does not exist is stated under item 2; whether an option survives it is the main session's to say, and the human's to decide."

### H3 — Once `opening.md` exists, the write guard refuses any subagent that places the sheet, and the doc does not say who places it or when

- **Location.** `implementation/stabilization-workflow.md:84`, *"Forks go to the human one at a time; reading, checking and writing go to subagents (ruling 17)"*, and line 86, *"The sheet is committed beside the opening record as `opening-facts.md`"*. Neither line says who moves the sheet from the gatherer's scratch root into the run's directory, or in what order relative to `opening.md`.
- **Evidence.** `dev/stabilize-guard:18-26` and `:217`: the guard binds when a run's loop branch is checked out **and** `completions/artifacts/<run>/opening.md` is a file on disk, and it fences every subagent except `build-git` and `build-fixer`. I drove the hook in a throwaway clone (repro below). With `opening.md` present on `fix/rc24-tier1`, a `Write` of `completions/artifacts/rc24-tier1/opening-facts.md` was refused for `agent_type` `stabilize-opening` and for `general-purpose` (`permissionDecision: deny`, `refused repository`). The session's own call (no `agent_type`) passed. So a writer subagent that follows ruling 17 and writes `opening.md` first cannot then place the sheet. A `cp` from such a subagent runs under the write fence and is denied too.
- **How the count was reached.** The class is every hand-placed file in the run directory that a fenced subagent writes after `opening.md` exists. The `--help` file listing (header lines 28-31) names exactly two files as "never this script's": `opening.md` (its later revisions) and `opening-facts.md`. Every other file is written by `dev/stabilize-record` through one plain invocation, which the guard lets through unfenced. So the class is 2 files, and `opening.md`'s revisions predate this commit.
- **Smallest correct fix.** One sentence in *Opening a run*: the sheet is placed in the run's directory by the main session, or by a subagent before `opening.md` is first written, because the write guard binds from that file on (→ The write guard).

---

## Notes

1. **The ruling is recorded as given.** All three places carry it: `DECISIONS.md` (top entry, dated 2026-10-27), *Opening a run* (line 86) and *Orchestration* (line 477). Each includes the read-only gatherer, one section per decision, source and how-known, no recommendation, no question before the sheet is back, the recommendation from the main session with each question, the step's general reach, this run as first use, and the exception flagged *may re-open once the sheet is back*. The planning quote matches `milestone-planning-workflow.md:39` byte for byte. Nothing is widened. One small narrowing: planning's exception continues *"widening that one request into settling other forks early is the failure, not the exception"*, and the stabilization doc's *"The one exception is planning's too"* leaves that clause out. It points at planning, so it is arguably carried, but a reader of the stabilization doc alone does not see it.
2. **The definition restates the doc's list** (line 12) instead of only pointing at it. Line 10 already says the doc's list "is the outline of your sheet", so line 12 is a second copy that no fence holds to the first. This copy is how H1 reached the definition.
3. **Option ordering and framing (item 4, line 17).** Item 4 has no rule on the order in which values are listed, and none requiring each value to get the same kinds of fact. A gatherer could list its favourite first, or give consequences for one option and not the others. Smallest addition: "each value in the order its source lists it, each with the same kinds of fact."
4. **The placeholder vocabulary does not match the script's.** Line 23 lists `<repo>`, `<scratch>`, `<tmp>`. The record script's vocabulary (`dev/stabilize-record` header lines 2179-2183) writes a repository path as repository-relative (`.` for the root) and a home path as `~`. It has no `<repo>`. Harmless: `vet` passed a sheet containing `<repo>` (repro below) and refused a real host path with `host-path`. But the definition teaches a form the run's other records do not use, and it omits `~`.
5. **`cargo nextest list` is not read-only in effect.** It compiles every test binary into the shared `target/`, takes cargo's lock (which a concurrent gate also holds), and costs minutes. If the gatherer ever runs after `opening.md` exists (a second gather), the guard runs it under the write fence, which denies writes under the repository, so the listing fails. Suggest either reading the list a test set needs from its committed source (for example `dev/regression-set`'s list), or saying that the listing uses a private target directory.
6. **The read-only bound is an instruction, not a tool restriction.** The definition carries `tools: Read, Grep, Glob, Bash, Write`, the same set as `stabilize-scope` and `finding-triage`. Before the run is opened the guard does not bind, so nothing technical stops a repository write; the instruction is the whole bound. That is consistent with its siblings, but worth knowing.
7. **The halt contract asks for a `recommendation` field** (line 29) in a definition whose description says it "recommends nothing". It is the standard halt field (`stabilize-scope` has it too) and is scoped to what unblocks the halt. Wording it as "what would unblock it" would remove the ambiguity.
8. **The *Never* paragraph grants push and merge acts to a read-only role.** These are the fenced bytes of `release_pipeline_fence` arm (q), which every definition must carry, so the paragraph cannot be trimmed here. Lines 22-23 make the read-only scope clear.
9. **Paths, pronouns and hygiene are clean.**
   - All seven link targets in the diff resolve (checked with `test -e`).
   - `.claude/agents/` is not one of `doc_link_fence`'s `HOMES` (`tooling-tests/doc_link_fence.rs:198`), so the definition's links are held by no fence. I checked them by hand.
   - No added line contains he/him/his: `command grep -nE '\b(he|him|his|himself)\b'` over the added lines gave 0 hits.
   - No added line contains a host path or an id: a grep for `/Users/`, `/private/`, `/var/folders`, `session_`, `agent_id` gave 0 hits.
10. **Nothing else that reads a run's directory refuses the sheet.**
    - Every `os.listdir` and `os.walk` in `dev/stabilize-record` (12 sites, enumerated by grep) reads a subdirectory: `r<N>/reports…`, `r<N>/verdicts`, `numbered()`'s `r<N>` match, or the temporaries regex `\.stabilize-record\..*\.tmp`.
    - `dev/stabilize-step`'s only walk (`listed_files`) is over git's directories.
    - The harness source names only `opening.md` (`stabilize.js:2665`, `:4835`, `:5213`).
    - No fence walks `completions/`. `doc_link_fence` excludes it, and `install_line` is applied by `vet`.
    - Stage roles are told only of `opening.md` (`opening(v)`, `stabilize.js:2664`), so no stage agent reads the sheet. That is consistent with "prose … the record script neither … reads".
11. **The `--help` line was added and is printed.** `dev/stabilize-record --help` prints it at line 30 of its output. No fence holds that text.

---

## Questions for the human

1. **May the gatherer name an option that the doc, the script and the code do not frame?** The ruling's own reason is planning's M54 fork: the gap pass found self-exec, an option nobody had listed, and the decision had to re-open. Item 4 of the definition (line 17) says the decision is listed "only as the doc frames it … never invented". The gatherer can therefore never surface the kind of option that motivated the ruling; only the main session can. **My recommendation:** keep the gatherer to facts, and add one section, *"what is possible beyond the doc's framing"*. It would hold capabilities that exist, as facts with their sources (for example a tool that exists but is not a listed instrument). It would never rank them or recommend.

---

## Repro — the sheet beside `opening.md` (verifies the builder's claim)

On a throwaway clone at `a7c2d66a`, under the session scratchpad, with a bare remote:

```text
W=$(mktemp -d "<scratch>/clone.XXXXXX"); git clone -q --no-hardlinks <repo> "$W/repo"; git init -q --bare "$W/remote.git"
cd "$W/repo"; git checkout -q fix/rc24-tier1; git remote set-url origin "$W/remote.git"; git push -q origin fix/rc24-tier1
D=completions/artifacts/rc24-tier1   # the run name whose loop branch is fix/rc24-tier1 (branchName)
# write $D/opening.md and $D/opening-facts.md (the sheet holding `<repo>`, a commit sha, sections)
python3 dev/stabilize-record vet --run rc24-tier1 --subject "chore(stabilize): open probe-run" -- $D/opening.md $D/opening-facts.md
  -> {"ok": true, ... "refused": []}
# a host path appended to the sheet:
  -> {"ok": false, "refused": [{"why": "host-path", ...}]}   (vet=24); sheet restored
git add $D/opening.md; git commit   # sheet left untracked
python3 dev/stabilize-step git-state --stage test --loop fix/rc24-tier1 --rounds fix/rc24-tier1-r --run-dir $D --product Cargo.toml --product Cargo.lock --product crates --look
  -> refused dirty: ... ?? completions/artifacts/rc24-tier1/opening-facts.md   (rc=4)
git add $D/opening-facts.md; git commit
python3 dev/stabilize-record state --run rc24-tier1   -> opened true, next not-ready (owes clause, stop, previous, scope)
python3 dev/stabilize-record pending --run rc24-tier1 -> once [], batch null
python3 dev/stabilize-record run-set --run rc24-tier1 <facts.json  -> run.md + clauses.md written; state: not_ready [], next test
git reset --soft HEAD~2; git add $D; git commit -m "chore(stabilize): open the run rc24-tier1 (probe)"   # the opening's one commit, 4 files
python3 dev/stabilize-record vet --range -- fix/rc24-tier1 --not --remotes=origin   -> {"ok": true, "refused": []}
git push -q origin fix/rc24-tier1
python3 dev/stabilize-step git-state --stage test --loop fix/rc24-tier1 --rounds fix/rc24-tier1-r --run-dir $D --product Cargo.toml --product Cargo.lock --product crates
  -> {"status": "ready", "opening": "<the opening commit>", "untracked": [], "pending": null}   (rc=0)
python3 dev/stabilize-step begin --run rc24-tier1 --round 1 --stage test --attempt 1 --reporter stage-test --commit <head> --scratch "$W/scr"  -> begun
python3 dev/stabilize-step check-reports --run rc24-tier1 --round 1 --stage test --attempt 1 --scratch "$W/scr"  -> checked, aside []
```

## Repro — H3, the guard on a placement of the sheet

The same clone, with `opening.md` committed on `fix/rc24-tier1`. A PreToolUse `Write` event for `<clone>/completions/artifacts/rc24-tier1/opening-facts.md`, fed to `dev/stabilize-guard` on stdin with `CLAUDE_PROJECT_DIR` set to the clone:

```text
agent_type stabilize-opening -> permissionDecision deny, "refused repository: stabilize-opening may not write …"
agent_type general-purpose   -> permissionDecision deny, "refused repository: …"
no agent_type (the session)  -> no output, rc 0 (allowed)
```

<!-- end of report -->
