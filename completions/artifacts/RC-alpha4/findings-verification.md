# Findings verification — the project-alpha-4.0 rc.9 trial vs the rc.9 code (verified 2026-08-02)

Every distinct claim from the trial's five feedback reports ([trial-record.md](trial-record.md); verbatim in `feedback-P1..P5.md`) adversarially verified against **`jigc 1.0.0-rc.9` rebuilt from `e07637a`** (the rc.9 release source; `--version` confirmed) with **live repros in throwaway repos** — never by code inspection alone. **Provenance caveat, declared up front:** this verification ran **2026-08-02 on a new machine**, after the trial machine's drive failure destroyed the corpus, the 504-record invocation log, and the original analysis session (trial-record.md → Honest bounds). It therefore verifies the *feedback claims* against the *binary's behavior* — which survives intact, since rc.9 is deterministic and rebuildable — but cannot cross-check any claim against the lost log. Four independent adversarial verifier passes, one per theme: **Part A** the validate/finalize contract (5 claims) · **Part B** the write surface (7) · **Part C** the read surface (7) · **Part D** routing & misleading messages (12). Verdicts: **CONFIRMED** / **PARTIAL** (real behavior, claim needs correction) / **REFUTED** (working alternative shown) — repro blocks for all three alike (pinning.md §3).

**Joint scoreboard (31 claims):** **20 CONFIRMED** (+1 confirmed-as-decided: D7, the M42 fork-6 advisory, value contested on its 3rd corpus) · **6 PARTIAL** · **4 REFUTED** (A2 finalize's json stdout is pure — the human lines ride stderr, the trial saw a merged-stream view; B2 the one-shot author rule does not exist — the tool's own help states a refusal the binary doesn't have; B3 `set-field --unset` shipped in M41 and clears the exact field P1 fought; C1 address-scoped `doc show` works at every documented depth — the trial's address elided the section segment, and the same elision fails identically on the write side). Two sub-claims refuted inside confirmed findings (D8: the live-collision arm refuses loudly, and `jigc start --slug` exists).

**The verified through-line — the routing-followability lens.** No printed surface was caught inventing state; two are **stale prose contradicting real behavior** (B1+B2 — the pair that forced the trial's only real quality loss, the inverted roadmap, defending against a duplicate that the atomic `write.already-present` reject makes impossible; A3 — AGENT.md's report-only/exit-0 claim, four days staler than the deliberate exit flip it contradicts). Everything else is a **true statement that cannot be followed from where the agent stands**: a preview verb that doesn't run two of finalize's probe families while four surfaces call it finalize's preview (A1); a block route that steers directly into a second block (D2); a half-failed verb whose re-run reports done and whose store sweep then **false-greens** (A5 — the High sibling of the U2/V1 class); routes that name three repairs but not the shipped fourth (B3); wrong-address dead ends one hop from the answer the write side already gives (C1); capabilities reachable only through `--help` pages no composed surface points at (B7, C3, D8). The known-hole lens **did not** land: zero regressions, zero data loss, zero invented state — the carryover gate, the ahead-stamp detector, the item-slot guard, the stream discipline, and the atomic author reject all **held under their first unseeded field provocation**.

---

# Part A — the validate/finalize contract

Five claims from the project-alpha-4.0 feedback reports (P1/P2/P4/P5) verified against the installed **`jigc 1.0.0-rc.9`** (confirmed `jigc --version`; the trial's own binary per `protocol.md`). Every verdict rests on a **live repro** in a throwaway repo (`git init` → seed commit → `jigc setup` → the QUICKSTART loop), driven through the real binary under the scratchpad (`repo-a1b`, `repo-a1a` with the methodology compose marker, `repo-a5`); source cites locate behavior only. Verdicts: **CONFIRMED** / **PARTIAL** (real behavior, claim needs correction) / **REFUTED** (working alternative shown).

**Scoreboard: 3 CONFIRMED (A1 · A3 · A5, A5 sharpened worse than reported) · 1 PARTIAL (A4) · 1 REFUTED as stated (A2 — stdout is pure JSON on rc.9; the human lines ride stderr).**

---

## A1 — `jigc task validate` does not preview what finalize gates on, despite saying it does — **CONFIRMED, both arms** (High — the reporter's "broken contract" framing holds)

Two finalize-only probe families never run at `task validate`: the **carryover gate** (`finalize.carried-staged`, P2 §2) and the **owner-artifact presence probe** (`owner-artifact.present`, P1 §2). Both arms reproduced end-to-end; in each, `task validate` returned clean/benign (exit 0) and the immediately following `finalize` blocked (exit 3) on findings validate never mentioned.

**What the surface claims.** The preview claim is made on at least four surfaces, none qualified:

- The composed workflow text (single-task and completion alike), twice per compose:
  > `To see what's left before committing, run `jigc task validate <id>` — it previews the findings finalize will gate on, without committing anything.`
  > `what's-left: `jigc task validate <id>`   — previews the findings finalize will gate on`
- `QUICKSTART.md` (repo root, lines 138–143): "You can preview what finalize will gate on at any time: `jigc task validate <id>`".
- `jigc describe`'s command hint: `validate-task — Preview validation findings without committing.`

And the same composed text *itself* warns about the carryover refusal ("Anything still staged from BEFORE this task was minted makes finalize refuse too") — the workflow prose knows the gate its own preview verb doesn't run. P2 avoided the refusal only by reading that prose; an agent trusting validate's claim hits it at finalize.

**Repro block (b) — the carryover arm (priority), `repo-a1b`:**

```
# stage a modification + a deletion + an untracked-then-staged file BEFORE minting
$ echo "new line" > will-modify.txt && git add will-modify.txt
$ git rm -q will-delete.txt
$ echo "fresh" > new-untracked.txt && git add new-untracked.txt
$ git status --short
A  new-untracked.txt
D  will-delete.txt
M  will-modify.txt
$ jigc start --workflow single-task "carryover gate probe"
task minted: carryover-gate-probe
# author the commit doc so the carried set is the only difference
$ jigc doc set-field commit:carryover-gate-probe#type --value chore --task carryover-gate-probe
$ printf 'probe the carryover gate' | jigc doc set-slot commit:carryover-gate-probe#summary --from-file - --task ...

$ jigc task validate carryover-gate-probe
no findings — the task validates clean
VALIDATE EXIT=0
$ jigc task validate carryover-gate-probe --format json
{ "schema_version": 2, "findings": [] }          # zero findings — nothing truncated

$ jigc task finalize carryover-gate-probe
blocking · finalize.carried-staged — `new-untracked.txt` was already staged before this task existed …
blocking · finalize.carried-staged — `will-delete.txt` was already staged for deletion before this task existed …
blocking · finalize.carried-staged — `will-modify.txt` was already staged before this task existed …
FINALIZE EXIT=3
```

Exactly one blocking finding per carried path, all three axes (modification / deletion / untracked-then-staged), none previewed. Matches P2 §2 verbatim.

**Repro block (a) — the owner-artifact arm, `repo-a1a`:** driven in full, not just "as far as feasible". The methodology pack was composed via the setup marker (`.jigc/config/packs.yaml` → `compose-embedded-methodology: true` — `pack.rs:702` `read_compose_marker`; the knob is not `config set`-table), then the real `completion` workflow minted (`jigc start --workflow completion "close milestone v1"`), the `completion-record` authored to schema (verdict, findings item, all required fields), `owner-artifact` set to `completions/artifacts/v1/audit.md` **with no file at that path** — P1's exact state.

```
$ jigc task validate close-milestone-v1
advisory · file-state.staged-copy — staged copy of `docs/completions/v1.md` — this task's in-flight version of the doc
  route: no action needed — …
VALIDATE EXIT=0                                   # only the benign staged-copy advisory — P1's words exactly

$ jigc task finalize close-milestone-v1
blocking · owner-artifact.present — owner-artifact `owner-artifact` in section `meta` of `docs/completions/v1.md`:
  `completions/artifacts/v1/audit.md` names no file under the repository
  route: place the owned artifact at the recorded path, or correct the field with `jigc doc set-field` …
FINALIZE EXIT=3
# placing the file and re-running finalize lands clean (0a51710) — the probe is presence-at-finalize only
```

P1 hit this through `migrate-completion-record` + `--approve`; the plain `completion` workflow exercises the identical probe, and it reproduces on plain finalize — the `--approve` detail is incidental. Nothing about the completion path was undrivable.

**Notes.** The gap is structural, not cosmetic: both probe families key on state (`the staged set at mint` / `a repo path outside the doc`) that the task-validate pass simply does not consult. P2's disjunction stands as written: either validate runs these probes, or the four surfaces stop advertising it as finalize's preview. Note `--dry-run` on finalize is *also* not a substitute a reader is pointed to by the preview claim — the claim names `task validate`.

---

## A2 — "`task finalize --format json` emitted three lines of human text before the JSON object" — **REFUTED as stated on rc.9: stdout is pure JSON; the human lines ride stderr** (the residue is a documentation gap, not impure output)

**Repro block (`repo-a1b`, a landed finalize with unstaged + untracked files present — P2's exact scenario, `{"committed": …}` success path):**

```
$ git status --short
M  README.md            # staged (the task's work)
 D will-delete.txt      # unstaged deletion
 M will-modify.txt      # unstaged modification
?? new-untracked.txt    # untracked
$ jigc task finalize carryover-gate-probe --format json > /tmp/fin.json 2> /tmp/fin.err; echo EXIT=$?
EXIT=0
$ head -c 20 /tmp/fin.json | od -c
0000000    {  \n           "   c   o   m   m   i   t   t   e   d   "   :     # stdout byte 1 is `{`
$ jq . /tmp/fin.json && echo PARSES
PARSES                                             # naive `| jq .` succeeds
$ cat /tmp/fin.err
finalize — committing the index; leaving out:
  left-out (unstaged/untracked — git add to include):
    will-delete.txt
    will-modify.txt
    new-untracked.txt                              # ← P2's "three lines" — they are on STDERR
$ cat /tmp/fin.err /tmp/fin.json | jq . ; echo $?
jq: FAILS                                          # merged streams reproduce P2's picture exactly
```

Re-verified on a second finalize (`--carry-staged` success with both a left-out block *and* a carried-over block on stderr): stdout pure, `| jq` clean, merged fails. The blocked path (`carried-staged`, exit 3) emits a pure JSON findings envelope on stdout too.

**The working alternative:** `jigc task finalize <id> --format json | jq .` works as AGENT.md instructs — **provided the caller does not merge streams**. P2's transcript (human lines immediately "before" the JSON object) is a merged-stream capture: `2>&1`, or an agent harness that interleaves stdout and stderr into one view — under which the text does appear exactly as reported, and a merged-stream `jq` does fail. M45's stream-discipline increment is what put the human block on stderr; on the trial's own binary the claim's mechanism ("the structured result is prefixed by those lines") is wrong.

**Residue (real, fix-shaped):** nothing tells the caller the split exists. `.jigc/AGENT.md` says "pass `--format json` and parse the structured result — do not scrape the human-readable lines" and mentions streams **nowhere** (grep of the installed AGENT.md: zero hits for stderr/stdout/stream). An agent whose execution surface shows combined output cannot follow the instruction without knowing to separate the streams — and the human block *is* still emitted alongside `--format json` (stderr), with content redundant to `.committed.left_out`, exactly as P2 noted. One AGENT.md sentence ("human progress rides stderr; stdout is the JSON") or suppressing the stderr block under `--format json` closes it.

---

## A3 — AGENT.md says store-scope `jigc validate` is report-only/exit-0; with a schema-version-ahead finding it exits 1 — **CONFIRMED: the behavior is by design, the AGENT.md prose was never updated and is now false as written** (Low-Medium, law-1-shaped)

**What the surface claims.** The installed `.jigc/AGENT.md` (generated from `adapter.rs:1086`, `BOOTSTRAP_OUTPUT_CONTRACT`), unqualified:

> "A store-scope `jigc validate` is report-only — it exits 0 even when it surfaces findings."

**The intended behavior** is the deliberate exception: DECISIONS.md → 2026-07-24 confidence-audit fix 1 minted `schema-conformance.schema-version-ahead` with "the exit flips on the same untrustworthy-sweep criterion (the doc was written to a schema this binary does not know)" — deliberately un-keyed, undemotable. The trial protocol *planted* this exact state (`schema-version: 99`, expected signal (c): "store sweep exits non-zero"). P5 hit the designed trap and correctly caught that the docs never followed.

**Repro block (`repo-a1a`, committed completion-record, stamp hand-edited out-of-band and committed):**

```
$ sed -i '' 's/^schema-version: 1$/schema-version: 99/' docs/completions/v1.md
$ git add docs/completions/v1.md && git commit -qm "oob: stamp 99"
$ jigc validate; echo VALIDATE EXIT=$?
blocking (gates at finalize) · file-state.hash-matches — on-disk content of `docs/completions/v1.md` differs …
blocking · schema-conformance.schema-version-ahead — `docs/completions/v1.md`: field `schema-version` is
  schema-version 99, above the current schema-version 1 — the doc was written to a schema this jigc build does not know
  route: ahead — … either a newer jigc wrote it (upgrade jigc) or the stamp was edited out-of-band (restore it
  from git history) — `jigc migrate-corpus` cannot fix a future stamp
a committed doc is stamped above this build's schema-version — … the sweep could not adjudicate it and exits non-zero;
upgrade jigc, or restore the stamp from git history, then re-validate.
VALIDATE EXIT=1
```

**Notes.** The binary is honest at the point of contradiction — its trailer *states* "exits non-zero" and why — so the lie is confined to AGENT.md's preloaded contract, which is precisely the surface an agent trusts without re-checking. P5's framing ("the docs are wrong, or schema-version-ahead is an undocumented exception") is exactly right: it is a documented-in-DECISIONS, undocumented-in-AGENT.md exception. Fix is one clause on the `BOOTSTRAP_OUTPUT_CONTRACT` sentence (e.g. "… exits 0 even when it surfaces findings — with one exception: a doc stamped above this build's schema-version flips the exit, because the sweep cannot adjudicate it"). Note the M43 tier-2 exit-code registry / `AGENT.md` machine-output paragraph (M44) both predate the 2026-07-24 exit-flip — the stale-prose window is four days of one wave, but it shipped in rc.9's installed adapter.

---

## A4 — "Couldn't tell whether `jigc task validate <id>` gates; had to run it twice to read `$?`" — **PARTIAL: the exit semantics are stated in the verb's own `--help` first line; no output line states them, and the store/task label asymmetry actively feeds the confusion** (Low-Medium, discoverability)

**The measured facts (all three states, live):**

| state | output | exit |
|---|---|---|
| (i) no findings (`repo-a1b`, fully-authored task) | `no findings — the task validates clean` | **0** |
| (ii) advisory-only (`repo-a1a`, staged-copy advisory) | `advisory · file-state.staged-copy — … route: no action needed …` | **0** |
| (iii) blocking (`repo-a1b`, fresh task, empty commit doc) | `blocking · schema-conformance.field-value-conformant — …` + `blocking · schema-conformance.required-slot-present — …` | **3** |

So: exit is `0` unless a blocking finding exists, then `3` — `task validate` **does** gate its exit on blocking findings (it is the task-scope preview *and* it reports gate status through `$?`).

**Where that is stated.** It IS stated, once, in the verb's own help — `jigc task validate --help`, first line:

> "Run `validate(task)` and render the findings; **exit non-zero iff any blocks**"

and AGENT.md's exit table ("3 blocking findings at a task-scope gate") implies it. **It is stated nowhere in the command's output**: the clean line says "validates clean" (which does answer P4's "does clean mean no findings" — it says `no findings`), but a findings-bearing run prints bare `blocking ·` / `advisory ·` labels with no trailer about this invocation's exit or gate. Contrast the **store** sweep, which annotates in-band — `blocking (gates at finalize) · file-state.hash-matches` (A3 repro above) and P1's quoted "report-only at store scope (exit 0); each gates nowhere" trailer. The task-scope verb, the one whose exit actually flips, is the one that says nothing — that asymmetry is the verified root of P4's double-run. The claim is PARTIAL because the answer was one `--help` away (P4 reports reading AGENT.md, not the verb help); the output-side absence is real and one trailer line fixes it.

---

## A5 — `migrate-corpus` has no route for its own half-failure (hook-rejected commit, writes staged, re-run says done) — **CONFIRMED, and sharpened: `jigc validate` then reports the store clean while HEAD still holds the unmigrated copy** (High — a false green over a half-landed migration, V1/U2's sibling)

**Repro block (`repo-a5`: committed ADR hand-downgraded to v1 shape — stamp `schema-version: 1`, `## Options` removed, committed; rejecting pre-commit hook standing in for P1's husky/lint-staged):**

```
$ jigc migrate-corpus --dry-run
corpus migration (dry run — nothing written): 1 would migrate, 0 already current, 0 blocked

$ printf '#!/bin/sh\necho "lint-staged: FAIL" >&2\nexit 1\n' > .git/hooks/pre-commit && chmod +x .git/hooks/pre-commit
$ jigc migrate-corpus; echo EXIT=$?
`git commit` was rejected (no commit was made):
lint-staged: FAIL
EXIT=1                                             # plain error text: no finding, no route, no --retry-commit
$ git status --short; git log --oneline -1
M  docs/decisions/use-sqlite.md                    # the migration write LANDED and is STAGED
218e952 simulate v1-era adr                        # …and nothing was committed

$ jigc migrate-corpus; echo EXIT=$?                # re-run (hook still rejecting — same with hook removed)
corpus migration: 0 migrated, 1 already current, 0 blocked
  current    docs/decisions/use-sqlite.md
EXIT=0                                             # P1's dead end, verbatim: "done", nothing committed

$ git show HEAD:docs/decisions/use-sqlite.md | head -4     # committed copy: schema-version: 1
$ head -4 docs/decisions/use-sqlite.md                     # on-disk copy:   schema-version: 2

$ jigc validate; echo EXIT=$?
no findings — the committed store validates clean
EXIT=0                                             # ← the sharpener: a false green — the sweep adjudicated the
                                                   #   on-disk bytes; the message claims the COMMITTED store,
                                                   #   whose copy is still stamped 1
$ rm .git/hooks/pre-commit && jigc migrate-corpus  # recovery attempt: it never re-attempts its own commit
corpus migration: 0 migrated, 1 already current, 0 blocked
```

**Does ANY surface route the staged-but-uncommitted state?** Swept all three named surfaces plus the next-task path:

- **`migrate-corpus` re-run:** no — "already current", exit 0, hook present or removed; it keys "current" off the on-disk stamp and never re-attempts the commit it owes.
- **`jigc validate`:** no — worse, the false green above. (Contrast A3: the *ahead* stamp flips the exit; the *stranded-at-index* state produces silence.)
- **`finalize`:** no direct route — but the **next task's carryover gate** does eventually trip on it: `blocking · finalize.carried-staged — docs/decisions/use-sqlite.md was already staged before this task existed` (exit 3, live-verified). Its route, though, is **wrong for this state**: "unstage it (`git restore --staged -- …`) if it is not this task's work, or `--carry-staged`" — unstaging strands the migration further; the correct recovery (commit the pathspec-limited migration) is named by no surface. Riding it out with `--carry-staged` would fold a corpus migration into an unrelated task's commit.

So the only clean recovery is a raw hand `git commit` of a jigc-owned schema-version write — exactly the workaround P1 recorded as "the one I'm least comfortable with" (a9731ef). **Notes.** The M42 finalize surface got the survivable-hook-rejection frame ("a hook-rejected finalize identifiable in the invocation log"); `migrate-corpus`'s own commit path visibly never got the sibling treatment — its rejection surface is a bare stderr passthrough with no finding key, and its idempotence check ("already current") is stamp-based, not committed-state-based, which is what converts a transient hook failure into a permanent silent dead end. Fix-shaped on the complete-fix contract's own terms: the half-failure axis here is the *verb-owned commit* family (finalize had it hardened at F7/M45; `migrate-corpus`'s and `rename`'s own commits are the un-swept siblings).

---

## Scoreboard

| # | claim | verdict |
|---|---|---|
| A1 | `task validate` doesn't preview finalize's gates despite saying so (owner-artifact + carried-staged arms) | **CONFIRMED** (both arms, live) |
| A2 | `finalize --format json` impure — human lines before the JSON | **REFUTED as stated** (stdout pure, `\| jq` clean; the lines are stderr; merged-stream capture reproduces the report; AGENT.md never states the split) |
| A3 | AGENT.md's report-only/exit-0 claim vs the exit-1 `schema-version-ahead` sweep | **CONFIRMED** (exit-flip by design; AGENT.md prose stale and unqualified) |
| A4 | can't tell whether `task validate` gates | **PARTIAL** (0/0/3 measured; stated in `--help` "exit non-zero iff any blocks"; absent from output; store/task label asymmetry real) |
| A5 | `migrate-corpus` half-failure: staged writes, "already current" dead end, no route | **CONFIRMED + sharpened** (`jigc validate` false-greens the half-state; only accidental surface is the next task's carryover gate with a wrong route; recovery is a raw git commit) |

---

# Part B — the write surface

Seven write-surface claims from the trial's feedback reports (P1/P3/P4/P5) adversarially verified against the installed `jigc 1.0.0-rc.9` (confirmed `--version`) by **live repros in a throwaway repo** (`git init` → `jigc setup` → record-change / single-task / architecture-documentation chains through the real binary), with source cites to locate — never to substitute for — the observed behavior. Verdicts: **CONFIRMED** / **PARTIAL** (real behavior, claim needs correction) / **REFUTED** (the working alternative shown).

**Scoreboard:** 3 CONFIRMED (B1 the contradiction — and the live behavior matches *neither* text · B4 no item ordering · B5 no slot append) · 2 REFUTED (B2 the one-shot author rule does not exist on rc.9 · B3 `set-field --unset` exists and clears `implemented-by` end-to-end through finalize) · 2 PARTIAL (B6 `jigc task diff` exists and shows the staged changeset — P3 right, P1 half-right; the doc half is a full copy, not a delta, and no author dry-run exists · B7 the three slug ids reproduce deterministically and the word-cap *is* stated on the soliciting help surfaces — the renormalization + edge-stopword steps that produce "except when it isn't" are stated nowhere).

**The through-line, again the discoverability lens:** four of the seven claims dissolve into *the capability exists; the surface the tester was on never named it* (`--unset` in `set-field --help` but not in the blocking route; `task diff` in `task --help` but not in any workflow text; the second-author freedom contradicted by the tool's own stale one-shot prose; the slug cap in `add-item --help` but not at the composed `add-item` call site). The one defect class that is *prose being wrong about the tool* (B1/B2) is a law-1 violation on two surfaces at once.

---

## B1 — The append-vs-update contradiction (P3 §2) — **CONFIRMED: both texts verbatim on rc.9, and the live behavior is a third thing neither text states** (High — this drove a real quality loss)

**(i) Both texts exist verbatim on rc.9 surfaces.**

```
$ jigc workflow planning --preview | grep -n -B2 "existing entries are untouched"
83  stdin (grammar: `jigc doc author --help`) and places every entry in a single
84  write. Over an already-committed roadmap it copies the committed doc in and
85  appends the new milestone — existing entries are untouched:

$ jigc doc author --help | grep -B3 "committed doc is fine"
Use `create` then `set-slot`/`set-field`/`add-item` for incremental,
one-leaf-at-a-time authoring instead. Run `author` INSTEAD of those verbs,
never after them — a doc already staged by `create` rejects the second
create `author` implies. (A committed doc is fine: `author` copies it in
and updates it.)
```

Sources: `packs/methodology/steps/author-roadmap.yaml:21` (the preview text; the sibling steps `author-ledger.yaml:22`, `author-decisions.yaml:19`, `author-decision.yaml:24` carry the same "existing entries are untouched" clause) and `crates/cli/src/doc.rs:199` (the help text).

**(ii) What author actually does over a committed doc with a matching item title: REJECTS, atomically.** Neither duplicate-append nor in-place update. Repro (dev-pack changelog; release `1.0.0` committed through a full record-change finalize first):

```
$ jigc doc author changelog --from-file payload2.yaml --task back-fill-the-changelog-history
# payload = release "1.0.0" (matches committed) + release "1.1.0" (new)
blocking · write.already-present — write rejected: item "1-0-0" in section "releases" is already present
  route: the target already exists — edit it in place (`set-field`/`set-slot`) instead of re-creating it
EXIT=1

$ jigc task diff back-fill-the-changelog-history       # nothing staged for changelog — the reject is whole-payload atomic
# staged docs
--- commit:back-fill-the-changelog-history.md          # (only the mint-time commit doc)
```

The full behavior map, characterized live:

- **New items append; existing entries genuinely untouched** — a `1.1.0`-only payload landed `### 1.1.0` after the committed `### 1.0.0`, byte-untouched above it.
- **Existing leaves update in place** — an author payload over the committed arch-doc replaced the committed `overview` slot prose (`REVISED overview prose…` served back by `doc show --task`). So "updates it" is true for `set:` leaves.
- **A payload item whose title matches any existing item (committed or staged) rejects the whole payload atomically** — `write.already-present`, nothing staged, route to `set-field`/`set-slot`.

**The consequence for P3's session:** the feared failure mode — "that same payload duplicates v1.0" — **cannot happen**. The chronological all-six-milestones payload P3 declined to try would have been *rejected whole, safely, with nothing staged*. The defensive append-only ordering (and the inverted roadmap it produced) was forced by documentation, not by behavior: the preview text is right about the no-collision case, the help's "updates it" is right about doc-level leaves, and *neither* states the actual matching-item contract (reject + route). A law-1 fix needs one sentence on both surfaces: *new items append; existing leaves update; a matching item title rejects — edit it in place.*

## B2 — "author may be run only once per task" (P3 §2) — **REFUTED: no one-shot rule exists on rc.9; a second `doc author` succeeded on every path tried** (the help text is stale prose)

The claim is the tool's own help ("Run `author` INSTEAD of those verbs, never after them — a doc already staged by `create` rejects the second create `author` implies", `doc.rs:196-199`, quoted in the composed once-note too). Live, the stated refusal **fires on no path**:

```
# path 1 — author twice over a committed doc, same task (new items each time)
$ jigc doc author changelog --from-file payload3.yaml --task back-fill…   # adds 1.1.0
changelog:changelog  EXIT=0
$ jigc doc author changelog --from-file payload4.yaml --task back-fill…   # adds 1.2.0
changelog:changelog  EXIT=0
$ jigc doc show changelog --task back-fill… | grep '^### '
### 1.0.0 / ### 1.1.0 / ### 1.2.0      # both authors landed; nothing lost

# path 2 — create first, then author (the exact case the help says rejects)
$ jigc doc create adr --title "Use widgets for the frobnicator" --task probe…
adr:use-widgets-for-the-frobnicator
$ jigc doc author adr --from-file adr-payload.yaml --task probe…           # fresh doc staged by create
adr:use-widgets-for-the-frobnicator  EXIT=0

# path 3 — the identical author payload run twice on that same fresh doc
$ jigc doc author adr --from-file adr-payload.yaml --task probe…
adr:use-widgets-for-the-frobnicator  EXIT=0

# path 4 — a second, differently-titled adr authored in the same task
$ … --task second-adr-in-one-task    # "Use widgets…" then "Choose sprockets…"
adr:use-widgets-for-the-frobnicator  EXIT=0
adr:choose-sprockets-instead-of-cogs EXIT=0     # both staged (task diff shows both)
```

The only refusal in this territory is **item-level**, not run-level: `write.already-present` when a payload re-adds an existing item (B1) — atomic, nothing staged, route "edit it in place". Recovery from a genuinely wrong payload is `jigc task discard <id>` (live: "discarded task … — dropped staged edits to: changelog:changelog, commit:… (transient)"), which P3 knew.

**Verdict on the session impact:** the no-retry rule that (combined with B1's contradiction) made P3 refuse to experiment **does not exist** — experimentation was safe on every axis: colliding payloads reject atomically, non-colliding re-runs merge, and discard drops the lot. The help sentence is a stale claim about behavior the binary does not have — the same "nothing lies" class M43 fenced, on the one surface (long help) the fences don't generate.

## B3 — "No way to clear an optional field; set-field but no unset/--clear" (P1 §3) — **REFUTED: `set-field --unset` shipped in M41, is documented in `set-field --help`, and clears arch-doc `implemented-by` end-to-end through a green finalize** (discoverability residue is real)

```
$ jigc doc set-field --help
  --unset   Clear the field entirely — remove its line/bullet (an optional field
            re-conforms absent). Refused for author-required / defaulted / CLI-`set:` fields

# the exact field P1 fought:
$ jigc doc set-field 'arch-doc:widget-subsystem#components/widgetloader/implemented-by' \
      --value 'src/widget.js#WidgetLoader' --task describe-the-widget-subsystem
set arch-doc:widget-subsystem#components/widgetloader/implemented-by = src/widget.js#WidgetLoader
$ jigc doc set-field 'arch-doc:widget-subsystem#components/widgetloader/implemented-by' \
      --unset --task describe-the-widget-subsystem
unset arch-doc:widget-subsystem#components/widgetloader/implemented-by   EXIT=0
```

After the unset: the staged item carries no `implemented-by` line, `task validate` raises **no** finding for its absence, and the task **finalized green** (`finalized 4c9abe1 … promoted docs/architecture/widget-subsystem.md`) — the committed component has heading + description only. So a code-anchor field is *not* excluded: it is unset-eligible, and absent it re-conforms through the full gate. The refusal boundary is exactly what the help states, captured live:

```
$ … 'adr:probe-unset-boundaries#status' --unset       # defaulted field
blocking · write.unset-ineligible — field "status" is required (or defaulted) and cannot be unset
$ … 'adr:probe-unset-boundaries#date' --unset          # set: on-create stamp
blocking · write.unset-ineligible — field "date" is CLI-derived (`set: on-create`) and cannot be unset
```

**The discoverability residue (why P1 missed it):** the blocking finding P1 was actually staring at routes elsewhere. Live capture of `doc-code.symbol-exists`:

> route: if the cited code is on disk but unstaged, `git add` it — finalize adjudicates the staged index, not the working tree; otherwise update the citation to match the renamed/moved code, or restore the cited symbol (e.g. revert the change)

Three repairs named; *removing the anchor* (`set-field <addr> --unset`) is not one of them. The capability exists one `--help` away; the route at the point of pain doesn't name it — the same push/pull pattern the rc.7 rerun named. P1's workaround (`remove-item` + `add-item` + re-author) was three verbs where one flag sufficed. Fix-shaped: add the fourth arm to the `symbol-exists` (and `title-names-symbol`) routes.

## B4 — No position argument on add-item (P3 §3) — **CONFIRMED: append-only, no `--before`/`--after`/`--position`, and no reorder verb anywhere on the doc surface**

`jigc doc add-item --help` (captured in full): arguments are `<ADDR>` + `--title` + `--task` + `--format`, nothing else; the summary itself declares the contract — "the CLI mints the `{#id}` anchor + **appends** the item block". The full `jigc doc` verb list (`create · add-item · remove-item · retitle-item · set-field · set-slot · author · show · schema · list`) contains no move/reorder/insert-at verb, and `doc author` items land in payload order *after* existing items (B1), so the batch verb is no side door. Live: every added release appended at section tail. P3's structural point stands sharpened by the design's own vocabulary: the architectural invariant is "ordering lives in a separate ordered list" (stable IDs, never positions — CLAUDE.md/VISION), but **no write verb addresses that ordered list**; for `roadmap` ("the running milestone spine") the order is expressible only by authoring in final order the first time. P1 §3 independently hit the compounding case: `remove-item` + re-`add-item` lands at the tail with no recourse. Genuine gap, correctly reported.

## B5 — No way to append to a slot; set-slot replaces (P3 §3) — **CONFIRMED live: last write wins, no `--append`, no partial-edit verb**

`jigc doc set-slot --help` (captured in full): `<ADDR>` + `--from-file` + `--task` + `--format` only. Replacement proven live:

```
$ printf 'first line of prose' | jigc doc set-slot adr:slot-replace-probe#context --from-file - --task …
$ printf 'second write'        | jigc doc set-slot adr:slot-replace-probe#context --from-file - --task …
$ jigc doc show adr:slot-replace-probe#context --task …
second write                                    # the first write is gone
```

The only route to add one bullet is the read-modify-write cycle P3 (and P1 §4, twice, via Python string-replace) actually ran: `doc show <leaf>` → edit externally → `set-slot --from-file`. Consistent with the determinism boundary (the CLI owns placement, the LLM owns the prose *whole*), so an `--append` is a design addition, not a bug — but this is now the **second trial** (P1's "no partial-edit verb for slot prose" + P3's `--append`) demanding a within-slot edit affordance, with the round-trip's cleanliness resting on observed behavior, not a documented guarantee (P3's exact caveat).

## B6 — Dry-run/diff for doc writes (multi-session) — **PARTIAL, settled: `jigc task diff` exists and covers the pre-finalize changeset (P3 right; P1's "no equivalent" wrong and self-corrected); the doc half renders full copies, not deltas; no render-without-staging exists for an author payload**

```
$ jigc task --help | grep diff
  diff      Show the working changeset vs base (code diff + staged managed docs)

$ jigc task diff probe-task-diff-rendering
# code changes vs base 07eeb03
diff --git a/probe.js b/probe.js               # ← real unified git diff for code
+console.log("hi")
# staged docs
--- changelog:changelog.md                     # ← FULL staged copy, not a diff
--- commit:probe-task-diff-rendering.md
```

Adjudication of the two claims: **P3 §4 is correct** — "task diff would have shown me the staged changeset before each finalize" (it does, and P3 admitted not using it). **P1 §3's "non-migration tasks appeared to have no equivalent" is refuted as written** — `task diff` plus `task finalize --dry-run` (live help: "Print the pre-commit manifest … and stop — commit nothing") together answer "what will this task change" before the commit. What P1/P5 wanted *beyond* that is real, though: the staged-docs half prints whole documents — the *delta* against the committed store must be eyeballed (P5's "I approximated with doc show --task and read it by eye"), where the code half gets a genuine `diff`. The name `task diff` overpromises for docs by exactly that margin.

**Render-without-staging for an author payload: CONFIRMED absent.** `doc author --help` carries only `--format/--from-file/--task`; live, `--dry-run` is rejected (`error: unexpected argument '--dry-run' found`). `jigc workflow <id> --preview` previews *step text*, never a payload render. Three sessions (P3/P4/P5) asked for it independently. The mitigating facts, proven above, shrink the risk the dry-run would fence: a bad payload rejects **atomically** (nothing staged, B1), a good one is inspectable post-write via `doc show --task`/`task diff`, and `task discard` drops everything — so the want is an ergonomics gap, not a safety hole. But note the compounding: the *stated* one-shot rule (B2) is what made post-write inspection feel unusable — fix B2's prose and half of the dry-run demand evaporates.

## B7 — Item-id minting "roughly five tokens, except when it isn't" (P5 §1) — **CONFIRMED as reported, with the mechanism named: all three ids reproduce exactly; the rule is deterministic; the word-cap IS stated on the soliciting help surfaces, the renormalization + edge-stopword steps are stated nowhere**

All three of P5's table rows reproduce byte-exact on rc.9 via `add-item`:

```
$ jigc doc add-item 'changelog:changelog#releases' --title "Test against MariaDB rather than port the views to portable SQL" --task …
changelog:changelog#releases/test-against-mariadb-rather-than
$ … --title "Scope v1.1 to the toolchain, not to a green suite"
changelog:changelog#releases/scope-v1-1
$ … --title "PHPStan baseline has drifted from the code"
changelog:changelog#releases/phpstan-baseline-has-drifted-from
```

The actual rule (`crates/engine/src/slug.rs`, slug-rule generation 2): renormalize (case-fold, map ` `/`_`/`/`/`.` → `-`, collapse) → cap at the first **5 words** (`MAX_WORDS`, `slug.rs:76`) → **50-char** word-boundary backstop (`MAX_CHARS`, `slug.rs:86`) → drop **edge** stopwords (`a/an/the/of/to/in/on/at/by/for`; medial untouched). That explains every "except": `v1.1` renormalizes to `v1-1` = **two** words, so the 5-word window of "Scope v1.1 to the toolchain…" is `scope-v1-1-to-the`, whose trailing `the` then `to` are edge-stopword-dropped → `scope-v1-1`. `than` and `from` are not in the stopword set, so rows 1 and 3 keep them — 5 plain words each. Deterministic throughout; "unpredictable" is precisely *unstated*, not random.

**What the surface states (this half corrects the claim):** `jigc doc add-item --help` says, seam-generated from the enforcing constants themselves (`slug.rs::mint_statement`, the M43 stated-at fence): "slugged lowercase-kebab into the item `{#id}` anchor, **capped at the first 5 words / 50 chars**" — `doc create --help` likewise. So a surface *does* state the cap. **What no surface states:** the renormalization (that `.` splits a token — the entire `scope-v1-1` surprise) and the edge-stopword drop; grep of help strings, `describe`, `doc schema`, orientation, and the composed planning/record-change texts finds neither. And the composed call sites the agent actually works from (`author-roadmap.yaml`: "The item id is minted from the title you give") carry no cap statement at all — the rule lives one `--help` away from where the guess happens, the same pull-tier shape as B3.

**The riders, both verified live:** (a) **no `--id`/`--slug` on add-item — CONFIRMED** (`error: unexpected argument '--id' found`, same for `--slug`; `--slug` exists at *doc* mint on `create`/`start`/`migrate`/`rename`, but no verb overrides an *item* anchor — `retitle-item` explicitly freezes it). (b) **add-item returns the minted id on stdout — CONFIRMED, and it is the full address**: plain format prints `changelog:changelog#releases/<id>` (directly pasteable into the follow-up `set-slot`), `--format json` carries it structured (`"op": "add-item", "target": {"item": "json-ack-probe", "section": "releases", …}`). The sanctioned batch pattern P5 wanted exists: capture stdout, never guess. Nothing in the composed step text says so — the capture path is as undocumented as the slug rule it would route around.

---

# The fix-shaped residue, ranked

1. **B1+B2 — the author prose pass (law 1):** one corrected paragraph on both surfaces (`doc.rs` long help + the four methodology `author-*` step once-notes): *author copies a committed doc in; new items append; existing leaves update in place; a matching item title rejects the whole payload atomically — edit it in place; author may be re-run in the same task.* Deletes the stated-but-nonexistent one-shot rule. This pair caused the trial's only real quality loss (the inverted roadmap) and both halves are pure prose.
2. **B3 — the `symbol-exists`/`title-names-symbol` routes name the fourth repair** (`set-field <addr> --unset` to drop the anchor) — one route string; the capability is proven end-to-end.
3. **B7 — state the whole slug rule where it binds:** extend `mint_statement` with the two unstated steps (dot-splits-token + edge-stopword drop), and render it at the composed `add-item` call sites, not only in `--help`; state the stdout-capture pattern in the batch step text.
4. **B4 — item ordering:** a genuine surface gap against the design's own "ordering lives in a separate ordered list" invariant; needs a design decision (insert-position flag vs a reorder verb), not a wording fix. Second trial demanding it (P1's remove/re-add tail landing compounds it).
5. **B6 — the doc-delta half of `task diff`** (render staged-vs-committed as a diff, matching the code half) and/or an author `--dry-run`; third consecutive trial asking. The atomic-reject + discard facts bound it to ergonomics, and fixing item 1 removes the fear that drove half the demand.
6. **B5 — slot append/partial-edit:** design-addition-sized (the determinism boundary owns placement; whole-slot prose is the current contract); second trial demanding it — park with the batch-authoring/slot-ergonomics ideas rather than patch.

---

# Part C — the read surface

Seven read-surface claims from the trial's feedback reports (P2/P3/P4/P5, `feedback-P1..P5.md`) adversarially verified against the installed `jigc 1.0.0-rc.9` (confirmed `--version`) by **live repros** in a throwaway repo (`git init` → `jigc setup` → two full task arcs — `record-decision` → committed ADR with slots + fields; `planning` → committed `deferral-ledger` with two repeatable entries, one minting the trial's exact item id `creator-wizard-state-lives`), plus code/design citation where it locates the behavior. The setup composes the methodology pack by default, mirroring the trial corpus. Stream probes use explicit `>out 2>err` file captures (zsh MULTIOS falsifies `2>&1 >/dev/null` one-liners — noted because the trial's own stream evidence has the same failure mode).

**Scoreboard:** 5 CONFIRMED (C2 · C4 · C5 · C6 · C7 — C7 with its mechanism corrected) · 1 REFUTED-as-capability / CONFIRMED-as-discoverability (C1) · 1 PARTIAL (C3). **The through-line, again the discoverability lens:** every capability the testers wanted on the read side except two (cross-corpus search, historical read) exists and worked first-try under the documented address grammar — what failed was the *route from the failure to the grammar*: the read-side wrong-address routes name no next command, the `{#id}`-anchor-is-the-id fact lives only on write-verb long help, and the footer terminates the same stream as the diagnostic it follows.

---

## C1 — "`doc show` dumps the whole document; the set-slot-symmetric address doesn't work" (P2 §3) — **REFUTED as a capability gap; CONFIRMED as an address-grammar discoverability defect** (Medium)

**The capability exists at every documented depth, live on rc.9.** Section, fields-only-section, field-leaf, item, item-leaf, and bare-singleton addresses all resolve (M42/M43, `design/doc-read-surface.md` → The slice grammar):

```
$ jigc doc show 'deferral-ledger:deferral-ledger#entries/creator-wizard-state-lives'
### Creator wizard state lives in the session  {#creator-wizard-state-lives}
whether wizard state stays in the session or moves to the db is deferred until the rework
<!-- fields --> - date: … - kind: Decision - trigger: …          # exit 0

$ jigc doc show 'deferral-ledger:deferral-ledger#entries/creator-wizard-state-lives/body'
whether wizard state stays in the session or moves to the db …   # exit 0 (leaf)
$ jigc doc show 'adr:use-sqlite-for-session-storage#context'      # slot section — exit 0
$ jigc doc show 'adr:use-sqlite-for-session-storage#status'       # fields-only section — exit 0
$ jigc doc show 'adr:use-sqlite-for-session-storage#status/status'
accepted                                                          # field leaf — exit 0
$ jigc doc show 'deferral-ledger#entries/creator-wizard-state-lives'  # bare singleton + fragment — exit 0
```

**What P2 tried fails because it elides the section segment, not because the read side lacks addressing.** `deferral-ledger:deferral-ledger#creator-wizard-state-lives` puts the *item id* in the *section* position — the grammar is `#<section>/<item>` (`#entries/creator-wizard-state-lives`):

```
$ jigc doc show 'deferral-ledger:deferral-ledger#creator-wizard-state-lives'
blocking · store.no-such-section — … names no section `creator-wizard-state-lives`
  route: name a section that exists in the committed doc          # exit 1
```

**The claimed write/read asymmetry does not exist** — the identical elided address fails on the write side too, so "symmetric with set-slot addressing" was never the form P2 used for set-slot (whose address is `#entries/<id>/body`):

```
$ echo x | jigc doc set-slot 'deferral-ledger:deferral-ledger#creator-wizard-state-lives' --task <t> --from-file -
no slot addressed by `deferral-ledger:deferral-ledger#creator-wizard-state-lives`   # exit 1
```

**The confirmed defect is the failure's route.** `store.no-such-section` says "name a section that exists in the committed doc" and `store.no-such-item` says "name an item that exists in the committed doc" — neither enumerates the sections, names the `#<section>/<item>` grammar, nor points at `jigc doc schema <doctype>` / `jigc doc show <doc>`. Contrast the **write-side** wrong-shape route, which does: `` `jigc doc schema <doctype>` to see the declared shape, then re-run the write at a declared address ``. An agent that guesses a read address wrong is dead-ended one hop from the answer; P2 concluded "no address-scoped read exists" and fell back to `| sed -n '290,320p'` — the wrong window on the first try, exactly as reported. ("Dumps the whole document" is true only of the bare `<type>:<slug>` address, by design.) *Fix-shaped:* the read-side no-such-section/item routes name `jigc doc schema <doctype>` (or enumerate the live section ids — the doc is already parsed at that point).

## C2 — No cross-corpus search; nothing between `doc list` and `doc show` (P2 §3 + P4 §3) — **CONFIRMED** (High — the verified driver of the trial's real AGENT.md violations)

Full verb surface enumerated live (`jigc --help`, `jigc doc --help`): the doc verbs are exactly `create · add-item · remove-item · retitle-item · set-field · set-slot · author · show · schema · list`. No `grep`/`search`/`find` verb exists anywhere on the surface. `jigc doc list` takes only an optional `[DOCTYPE]` — no string filter, no path filter. `jigc describe` is a static prose menu of workflows/doctypes ("don't parse it") — no per-corpus content. Nothing reads more than one doc's content per invocation.

**Is there ANY sanctioned path to "which managed docs mention string X"? Yes, exactly one, and it is O(N) invocations:** `jigc doc list --format json` → loop `jigc doc show <id> --format json` per doc → search client-side over the *output*. That composition is sanctioned (AGENT.md forbids reading the *files* — "the files are storage, not your interface"; grepping `doc show` output is reading through the interface) but no surface teaches it, and at the trial's corpus size (19–45+ docs) both P2 and P4 judged it non-viable and ran `grep -rn … docs/ CHANGELOG.md` raw — P2 names it "the real violation", P4 "a real deviation". **There is no single-verb path; the violation is the predictable cost of the gap.** Adjacent tracked home: `ideas/doc-search.md` (the lacon A5 nuance — `rename`'s prose-mention scan is the only repo-wide text scan in the binary, inseparable from executing a rename). This is now the second consecutive trial where the search gap drove the headline boundary violation.

## C3 — Item-address enumeration: "`{#anchor}`s turn out to be the ids, but nothing says so" (P4 §3 + P5 §3) — **PARTIAL: every mechanism shipped; the connecting sentence exists only on write-verb help** (Medium)

- **(i) CONFIRMED — `doc show` renders `{#id}` anchors on item headings.** Live: `### Creator wizard state lives in the session  {#creator-wizard-state-lives}` (the M42 anchor restoration, `design/doc-read-surface.md` → the retired byte-exactness claim).
- **(ii) CONFIRMED as the gap — no read-surface or bootstrap statement connects the anchor to the address component.** The fact *is* stated, but only on **write-verb long help**: `add-item --help` ("the CLI mints the `{#id}` anchor"; `--title` is "slugged to the `{#id}` anchor"), `retitle-item --help` ("its `{#id}` anchor stays frozen"), `remove-item --help` (the `#<section>/<id>` address forms). It appears **nowhere the reader stands**: not in `doc show --help` (which names `#section/<id>` slices but never mentions the rendered anchor), not in the show output itself, not in `.jigc/AGENT.md` (read in full — no mention of anchors or item ids), and `jigc doc schema` prints `<id>` placeholders (`deferral-ledger:<slug>#entries/<id>/kind`) without saying where a real `<id>` comes from. P4's "I didn't trust it" and P5's "by inference — nothing told me the address" are the expected outcome.
- **(iii) CONFIRMED — the pinned JSON carries the item `id`** (M42, live): whole-doc `--format json` → each item object carries `"id": "creator-wizard-state-lives"`; the section slice `doc show '<doc>#entries' --format json` returns the item array with ids — so `… --format json | jq -r '.[] | .id + " | " + .title'` **is** the item-listing P4 scripted Python for, in one call.
- **(iv) CONFIRMED absent — no `--items`/`--addresses`:** both rejected as `error: unexpected argument` (clap, exit 2). The `doc schema --addresses`-shaped projection remains the fix-shaped residual it was at lacon B6 — now demanded by a third trial.

## C4 — No way to read a managed doc at a past revision (P5 §3) — **CONFIRMED** (Medium; a genuine gap, correctly self-diagnosed)

`jigc doc show` takes exactly `--format` and `--task` (help + live: `--at`/`--rev` rejected as unexpected arguments). No verb on the enumerated surface reads git history — `doc show` serves committed-or-staged only (`design/doc-read-surface.md` → What it reads: committed by default, the staged copy on `--task`, nothing else). The workaround P5 used is the only path and it is raw git:

```
$ git show HEAD~1:docs/decisions/use-sqlite-for-session-storage.md   # works; no jigc equivalent
```

P5's own framing was already exact: "jigc has no equivalent; this is a genuine gap, not convenience." Note the boundary consequence: AGENT.md's read rule ("never read managed docs directly … read one with `jigc doc show`") has no carve-out for historical reads, so tracing any committed stamp/regression (P5's `schema-version: 99` hunt via `git show`/`git log -S`) *necessarily* goes around the interface. Design-revision-sized, not fix-shaped (a `doc show --at <rev>` engages the committed-by-default record and the parse path's source selection).

## C5 — `doc list --format json` has no schema-version (P5 §3) — **CONFIRMED, keys captured verbatim** (Low-Medium)

Live against the committed corpus:

```
$ jigc doc list --format json
{"docs": [
  {"id": "adr:use-sqlite-for-session-storage", "path": "docs/decisions/use-sqlite-for-session-storage.md", "state": "managed", "item-count": 0},
  {"id": "deferral-ledger:deferral-ledger",    "path": "docs/deferral-ledger.md",                          "state": "managed", "item-count": 2}]}
```

Row keys are exactly `id · path · state · item-count` — P5's report is byte-accurate; `schema-version` is absent. **Nuance:** the per-instance stamp *is* readable sanctioned — `jigc doc show <id> --format json` carries it in `fields` (live: `"schema-version": "2"`) — but only one doc per call, so P5's actual question ("are *other* docs also stamped 99?") has no bulk surface and the corpus grep was the only viable move. Given the confidence-audit wave made `schema-conformance.schema-version-current`/`-ahead` exit-flipping, the stamp is now gate-relevant state with no index-level read; a `schema-version` key on the `doc list` row is additive-window material (the pin explicitly reserves pre-1.0 additive keys; `item-count` landed under exactly that rule at M44).

## C6 — `doc show` on a leaf emits clean content, no footer — "observed behavior, not a documented guarantee" (P3 §4) — **CONFIRMED on both halves** (Low as behavior; the documentation half feeds C7)

**Behavior, live with split streams:** the leaf read emits the prose + one trailing `\n` on stdout (hexdump-verified: no banner bytes), **stderr 0 bytes**, no footer. Same for whole-doc, `doc list`, `doc schema`, and every successful write-ack probed (`set-field`/`set-slot`/`create`/`add-item`). The footer rides composed output (`jigc start`), `task validate`, and every findings envelope — not the successful read path. So P3's shell round-trip (`doc show <leaf> > f; cat >> f; set-slot --from-file f`) is safe, as they verified empirically with `tail -c 200`.

**Documented guarantee: no, on any surface the agent is sanctioned to consult.** `doc show --help` says "plain text is the canonical render" (the closest statement — it implies, never states, banner-free stdout). `.jigc/AGENT.md` (read in full) says nothing about the footer or streams. The real statements live only in jigc's own design tree, which the trial agent never has (and AGENT.md explicitly redirects from: "ask the installed binary … never a checked-out jigc or pack source tree"): `design/command-output-contract.md` → Stream discipline pins the `--format json` split (M45); `design/bootstrap.md`/`workflow-dialect.md` scope the routing footer to composed/orientation/validate agent-text output and "JSON carries no footer". **One documentation defect found en route:** `bootstrap.md:108` ("Routing footer on every CLI output") and `assistant-adapter.md:44` ("the routing footer the CLI appends to every agent-facing output") overstate the footer's coverage — live, the successful read/write-ack surfaces carry none. The overstatement is also the seed of P5's "essentially every invocation" model (C7). *Fix-shaped:* one sentence in `doc show`'s long help ("stdout is the addressed content only — no banner; diagnostics ride stderr") turns P3's observed behavior into the stated guarantee their round-trip already depends on.

## C7 — "jigc appends a footer to essentially every invocation, so `| tail -1` shows the banner and eats the diagnostic" (P5 §2) — **CONFIRMED as a hazard, with the mechanism corrected on both counts** (Medium)

**Which stream: the footer rides the *same stream as the diagnostic it follows* — there is no stream split to rescue `tail`.** Live, split-stream (agent format): a **reject** (blocked write, read block) emits finding + route + footer all on **stderr**, stdout **empty**; an **adjudication** (`task validate` findings, composed output) emits them all on **stdout** — matching the M45 outcome-class rule (`design/command-output-contract.md` → Stream discipline), with the footer as the terminal line of whichever stream carries the text.

**The tail hazard, reproduced live both ways** on P5's exact scenario (a wrong-item-id `set-field`, the guessed-id failure from their §1):

```
$ jigc doc set-field 'deferral-ledger:deferral-ledger#entries/wrong-id/kind' --value Decision --task <t> | tail -1
blocking · write.wrong-shape — write rejected: item "wrong-id" in section "entries" not present
  route: `jigc doc schema <doctype>` …
— jigc · run `jigc start` for orientation; all writes through `jigc`.
   # stdout-only pipe: stderr BYPASSES tail — the diagnostic prints in full. Nothing eaten.

$ jigc doc set-field '…#entries/wrong-id/kind' --value Decision --task <t> 2>&1 | tail -1
— jigc · run `jigc start` for orientation; all writes through `jigc`.
   # merged pipe: ONLY the banner survives. The diagnostic is eaten — P5's exact observation.
```

**So the claim is REFUTED in its letter and CONFIRMED in its substance.** (a) The footer is *not* on "essentially every invocation" — successful `doc show`/`list`/`schema` and every write-ack carry none (C6); it terminates composed output, validate output, and **every findings envelope**, which is every output an agent tails when something went wrong. (b) A plain stdout `| tail -1` does **not** eat a write diagnostic (rejects leave stdout empty; stderr passes through) — the eaten-diagnostic state requires a **merged** view (`2>&1 |`, or a harness that folds fd 2 into fd 1, which is what the trial worker's environment presents) — and P5's own caveat ("the error was probably printed and I truncated it") was right. In a merged view the hazard is fully real and stream discipline cannot defuse it, because within one stream the banner is always last. Compounding: the pipe also destroys the exit code (`tail` exits 0), defeating AGENT.md's "a non-zero exit means stop" rule in the same motion. **The robust alternative already exists and AGENT.md already names it:** `--format json` — the reject envelope is a single JSON document on stderr with **no footer** (live-verified: `{"schema_version":2,"findings":[…]}`, stdout empty), and the stream-discipline predicate (parse stdout; if empty, parse stderr) is tail-proof. P5's self-diagnosis ("I should have used --format json for writes as AGENT.md suggests") is the correct route; the residual fix-shaped question is whether the banner belongs on findings envelopes at all — the routing footer's stated purpose is compaction-resilient *orientation*, and a findings envelope already carries its own route line.

---

## The fix-shaped residue (read-surface slice)

1. **Read-side wrong-address routes name the next command** — `store.no-such-section`/`no-such-item` point at `jigc doc schema <doctype>` or enumerate the live section/item ids (C1; the write side already does).
2. **State the anchor-is-the-id fact where the reader stands** — one sentence in `doc show --help` + the AGENT.md read paragraph ("the `{#id}` on an item heading is the item's address id"); the `--addresses` projection is the fuller answer, now third-trial-demanded (C3, lacon B6).
3. **A search surface** — the gap that drives real AGENT.md violations two trials running; at minimum name the sanctioned O(N) composition (`doc list` → `doc show --format json` loop) in AGENT.md until a `doc grep`-shaped verb is designed (C2; `ideas/doc-search.md`).
4. **`schema-version` on the `doc list` row** — additive-window key, gate-relevant since the ahead/current exit flips (C5).
5. **Pin C6's observed guarantee** — `doc show` long help states banner-free stdout; correct `bootstrap.md`/`assistant-adapter.md`'s "every output" overstatement (C6).
6. **The footer on findings envelopes** — reconsider, or at least keep it off the last-line position no `tail` survives; historical reads (C4) stay design-revision-sized.

---

# Part D — routing & misleading messages

Twelve claims from the five feedback reports ([feedback-P1..P5.md](./)) adversarially verified against the installed `jigc 1.0.0-rc.9` (confirmed `--version`) by **live repros in throwaway repos** (`git init` → `jigc setup` → mint/author/finalize chains, incl. a completion-record migration driven to both of P1's finalize blocks, a husky-`core.hooksPath` install, an OOB-edit absorb cycle, and a task-id collision arc), plus source cites to locate behavior (never as the verdict). Repos: `scratchpad/repos/r1` (dev+methodology composed — the default `jigc setup` posture on rc.9) and `r2` (husky). Each confirmed claim is classified against the three surface-contract laws ([design/surface-contract.md](../../../design/surface-contract.md): **nothing lies / nothing hides / nothing ambushes**).

**Scoreboard:** 9 CONFIRMED · 2 PARTIAL (D5 real behavior, wrong verb attributed; D10 the projection half holds, the claimed rejection is refuted live) · 1 CONFIRMED-as-decided on value not correctness (D7). Two sub-claims refuted inside confirmed findings (D8: the live-collision case is loud, and `start --slug` exists). **The theme's through-line: no printed line was caught factually inventing state it can see — every defect here is a true statement mis-scoped (D1, D12), mis-rooted (D2), mis-framed as event-detection (D3), or printed at the wrong place/time (D4, D6, D9).** That is the discoverability lens again, now on the *message* tier.

---

## D1 — `--preview` advertised generally, refuses on `creates-task: false` workflows — **CONFIRMED** (law 1 — the advertising surfaces overstate the affordance's scope) (Low harm, trust erosion)

All three advertising surfaces verified live, none scoped:

- **SessionStart banner** = orientation (`.claude/settings.json` hook is literally `command: "jigc start"`): ``Preview: `jigc workflow <id> --preview`   — read a workflow's step text without minting a task`` — printed **directly above** ``Run: `jigc start --workflow ingest-existing` `` (P1's exact trap layout).
- **`jigc describe`**: "To read any one's full step text before you commit to running it, run `jigc workflow <id> --preview`" — attached to the full catalog, minting and non-minting alike.
- **`.jigc/AGENT.md`**: no preview mention (grep = zero) — not an advertising surface, contra none claimed.

The refusal, verbatim (exit 1): `workflow 'ingest-existing' mints no task, so there is nothing to preview — run it directly: jigc start --workflow ingest-existing` (identical shape for `increment`). **The fallback is followable and honest:** the direct run of `ingest-existing` exited 0, printed the full step text, and created no `.jigc/tasks/` entry — for a `creates-task: false` workflow the direct run *is* the preview, so the refusal's route is correct. The correctly-scoped statement exists exactly once, in `jigc workflow --help`: "`--preview` composes a `creates-task: true` `<W>`". So the defect is pure law 1 at the two composed surfaces: `<id>` in the banner/describe line quantifies over the whole catalog, and **nothing in the catalog marks which workflows mint**, so a reader cannot even predict which ids the verb will refuse. One-line fix per surface ("preview a *minting* workflow; a non-minting one runs directly with no side effects").

```sh
git init t && cd t && git add -A; git commit -m init; jigc setup
jigc start                                  # banner: unscoped Preview: line
jigc workflow ingest-existing --preview     # exit 1, "nothing to preview"
jigc start --workflow ingest-existing       # exit 0, prints steps, mints nothing
```

## D2 — the owner-artifact home never states its root; P1 paid two finalize blocks — **CONFIRMED** (law 3 — the constraint is stated nowhere before it fails; the block route then compounds it) (Medium-High, the trial's costliest single message)

The ladder text on rc.9 (`packs/methodology/steps/author-migration-completion-record.yaml:28-51`, and the sibling `author-completion-record.yaml:8`): "MUST live under the owned artifact home `completions/artifacts/<milestone>/`" + literal `mkdir -p completions/artifacts/<milestone>` — **no surface says repo-root-relative**: not the ladder, not `jigc doc schema completion-record` (projects the field as bare `owner-artifact: owned-location`), not either block message. Grep of the whole methodology pack: the only "repo-root" statements attach to *other* doctypes' comments. In a corpus where every managed path lives under `docs/`, P1's read was the natural one.

**Both blocks reproduced live** on a real `jigc migrate <foreign> --as completion-record` with the file at `docs/completions/artifacts/v1.0/source.md`:

1. Field `completions/artifacts/v1.0/source.md` → `blocking · owner-artifact.present — … names no file under the repository` / `route: place the owned artifact at the recorded path, or correct the field with jigc doc set-field to where the file really lives`.
2. Following that route (field → `docs/completions/artifacts/v1.0/source.md`) → `blocking · owner-artifact.present — … is not under the owned artifact home completions/artifacts/<milestone>/` — **same route text again**, which is now doubly wrong: the file *is* at the recorded path and the field *is* where the file really lives; the actual repair (move the file under repo-root `completions/`) is named by neither rung nor route.

So block 1's route actively steers into block 2 — P1's two-block arc is not bad luck, it is the routed path. The root statement only becomes inferable at block 2, by contrast with the rejected `docs/`-prefixed value. Fix-shaped: one absolute-from-repo-root example in the ladder (P5's suggestion) + the block-2 route naming the move.

```sh
printf '# v1 close\n\nShipped; audit clean.\n' > notes/v1-close.md; git add notes; git commit -m x
jigc migrate notes/v1-close.md --as completion-record          # mints migrate-…-<hash>
mkdir -p docs/completions/artifacts/v1.0 && cp notes/v1-close.md docs/completions/artifacts/v1.0/source.md && git add docs
# author payload: verdict green, owner-artifact: completions/artifacts/v1.0/source.md
jigc task finalize <id> --approve --carry-staged                # block 1: "names no file under the repository"
jigc doc set-field 'completion-record:v1-0#meta/owner-artifact' --value docs/completions/artifacts/v1.0/source.md --task <id>
jigc task finalize <id> --approve --carry-staged                # block 2: "is not under the owned artifact home"
```

## D3 — `title-names-symbol` misdescribes a capitalized-token heuristic as rename/removal detection — **CONFIRMED** (law 1 — the finding asserts an event that did not happen) (Low-Medium, sent P1 hunting a phantom code change)

Reproduced byte-for-byte P1's shape. Component titled `WordPress.org public APIs`, anchor `apps/api/index.js#CmsGlobalClient` (symbol present and resolving, nothing ever renamed or removed):

> `advisory · doc-code.title-names-symbol — component title `WordPress.org public APIs` names symbol(s) ["WordPress", "APIs"] but its anchor implements `CmsGlobalClient` (…) — the heading still names a renamed/removed symbol; update the title to match the code`

Three lies in one line, all law 1: (a) "names symbol(s)" — `WordPress` and `APIs` are capitalized prose tokens, not symbols in any grammar; (b) "the heading **still** names a **renamed/removed** symbol" asserts a history event on a freshly-created doc in its first task; (c) "update the title to match the code" — the code never diverged. The check's *design intent* is real (the M13 rationale in `crates/cli/pack/schemas/arch-doc.yaml:8-13`: a model fixes the anchor and leaves the title stale) and the route is good (names `retitle-item`, notes a descriptive title also clears it) — but the message narrates the one scenario the check was built for as if it were the scenario detected. Fires identically at task validate and store scope (drove both). Sharpener en route: **the check is symbol-presence-gated** — the same mismatched title over a *bare-path* anchor raises nothing (see D10), so the advisory appears exactly when the author does the more diligent thing (anchoring a symbol). Fix is pure wording: state the heuristic ("title contains capitalized token(s) X not matching the anchored symbol Y").

```sh
jigc doc add-item 'arch-doc:api-services#components' --title "WordPress.org public APIs" --task <t>
jigc doc set-field 'arch-doc:api-services#components/wordpress-org-public-apis/implemented-by' --value 'apps/api/index.js#CmsGlobalClient' --task <t>
jigc task validate <t>       # advisory fires with the rename/removal phrasing
```

## D4 — `jigc setup` reports `pre-commit hook → .git/hooks/pre-commit` under `core.hooksPath` — **CONFIRMED** (law 1 — a printed path that is not repo-real) (Low; behavior correct, message wrong)

Live in a repo with `core.hooksPath=.husky/_` and an existing husky `pre-commit` configured **before** setup:

- Setup output: `- pre-commit hook → .git/hooks/pre-commit   (warn-only doc↔code drift backstop)`.
- Reality: `.git/hooks/pre-commit` **does not exist** (`ls` exit 2); the jigc hook body landed inside `.husky/_/pre-commit`, with the pre-existing content preserved (my repro: appended after the jigc block; the existing hook demonstrably still ran — its `lint-staged-runs` line printed during setup's own install commit). So the install honors `core.hooksPath` and chains correctly, exactly as P1 found — and the message hard-codes the default path. Textbook A14-class law-1 miss: the fix is printing the resolved install path (setup knows it — it just wrote there).

```sh
git init t && cd t && mkdir -p .husky/_
printf '#!/bin/sh\necho lint-staged-runs\n' > .husky/_/pre-commit && chmod +x .husky/_/pre-commit
git config core.hooksPath .husky/_; git add -A; git commit -m init
jigc setup                        # prints ".git/hooks/pre-commit"
ls .git/hooks/pre-commit          # exit 2 — no such file
grep -c jigc .husky/_/pre-commit  # the hook actually landed here
```

## D5 — the stale-copy warning names the reader's own task — **PARTIAL: the self-naming note is real and reproduced; it is emitted by task-less `doc show`, not by `doc create` as claimed** (Low, momentary alarm; style-guide tier)

The exact message exists once in the binary — the M43 stale-read hint, `crates/cli/src/doc.rs:2175-2207`, printed on **stderr** by **task-less `jigc doc show`** only. Live, with the agent's own task the sole stager:

> `note: `changelog:changelog` is also staged in open task extend-the-rate-limiter-with — the committed copy served here may be stale; staged read: `jigc doc show changelog:changelog --task extend-the-rate-limiter-with``

**The claimed verb is wrong:** `jigc doc create changelog --task <own-task>` prints only the ack (`changelog:changelog` / `… (already existed — copied in for update)`) — stdout and stderr both captured, no note, twice. P3's quote also drops the `staged read:` tail the real note carries. What survives of the claim: the hint is an **existence check by declared design** ("no content read" — the rustdoc), so when the only staging task is *yours* and the staged copy was just copied from committed (necessarily identical), the note fires anyway and reads as a third-party warning about yourself. Technically true (law 1 holds), practically noise — a style-guide case: one `is_own_task` branch ("staged in **this** task — reading the committed copy; your staged edits are at `--task <id>`") would keep the truth and drop the alarm.

```sh
jigc doc create changelog --title Changelog --task <t>   # ack only — no note (2>&1 captured)
jigc doc show changelog:changelog                        # stderr: the note, naming <t> itself
```

## D6 — store-level `reconciliation.absorb` rides inside the per-task finding list — **CONFIRMED** (style guide / law-1-adjacent — placement implies causation; nothing marks scope) (Medium as attention cost)

Live: committed ADR hand-edited conformantly out-of-band (+ committed); then an **unrelated** task (touching only `apps/api/burst.js` + its commit doc) ran `task validate` and `task finalize`. Both printed, inside the task's own finding list, indistinguishable in form from the task-caused findings around it:

> `advisory · reconciliation.absorb — external edit absorbed: `docs/decisions/use-sqlite-for-storage.md`` / `route: no action needed — the external edit was absorbed into the baseline`

The finding text never claims the task caused it ("external edit" is accurate — law 1 holds), but the *list* is task-scoped output of a task-scoped verb, and nothing carries a `store-level`/`not-this-task` marker — P4's "I spent real attention deciding whether I'd caused it" is the designed reading of that placement. Mechanism: task validate routes the committed store through the reconciler (`file_state::reconcile_committed_store`), so store news surfaces wherever the next task-scope sweep happens to run — first-task-to-validate gets the blame-shaped line. Same class as lacon B9 (the no-op-route habituation, 4th trial), sharpened here to *mis-attribution by placement*. Fix-shaped: a scope prefix on reconciler findings in task-scope renders (`store · reconciliation.absorb …`), or a separate "store reconciliation" stanza.

```sh
sed -i '' 's/simple./simple. Needs no server./' docs/decisions/use-sqlite-for-storage.md
git add -A; git commit -m "oob edit"
jigc task validate <unrelated-task>    # absorb advisory inside the task's list
jigc task finalize <unrelated-task>    # same, above the commit ack
```

## D7 — `changelog-recording.gate-granted-unused` fires on every non-user-facing task — **CONFIRMED as designed; the value critique stands with one new datum** (works-as-decided — M42 fork 6; no law breach)

Live on a `refactor`-typed, zero-user-facing task from `single-task`:

> `advisory · changelog-recording.gate-granted-unused — workflow `single-task` grants the `changelog` create-gate and this task recorded no changelog entry` / route: `if the change is user-facing, record it — jigc start --workflow record-change "<what changed>" (or, before finalize, in-task: …); if it is not user-facing, no action is needed`

P4's structural point verified: the advisory keys on gate-granted-and-unused only — the commit `#type` sitting one document away (here `refactor`; in the sibling repro `feat`) is never consulted, so a correct skip and an oversight print identically, every time. The text is fair-minded exactly as P4 grants, and the route's `record-change` reference is followable (`jigc workflow record-change --preview` works even though the catalog doesn't list it). One new datum for the fork-6 revisit: **the advisory also fired on a task that had the changelog staged in-task** (copied in, no entry added) *and* on a `feat` commit where recording arguably was owed — so as shipped it distinguishes neither the innocent case nor the guilty one. This is the lacon B10 verdict recurring on its 3rd corpus; the `#type == docs`-suppression revision remains Settle-fork material against the M42 fork-6 record, not a silent fix.

```sh
jigc start --workflow single-task "rename internal helper variable for clarity"
# stage a pure-internal edit; set commit type=refactor, summary; then:
jigc task finalize rename-internal-helper-variable   # advisory fires
```

## D8 — task-id collision at mint: same first ~5 words → byte-identical id, no warning — **CONFIRMED for the finalized-reuse arm; the live-collision arm is loudly guarded, and two escape hatches the testers missed exist** (Medium; law-2-adjacent — the disambiguators are real but unnamed at mint)

Three arms driven live (`quick-fix`, intents sharing "back-fill the pre v1 …"):

1. **Reuse after finalize (P3's exact case): silent.** First task minted `back-fill-the-pre-v1`, finalized clean; a second, semantically different intent then minted **the byte-identical id** — output `task minted: back-fill-the-pre-v1`, zero mention that the id belonged to a finalized task ten seconds earlier. The finalize wipes `.jigc/tasks/<id>/`, so nothing structural collides — but every id-keyed record (invocation log, task-attributed findings, the human's memory) now aliases two unrelated pieces of work. No pre-mint check, exactly as P3 reported.
2. **Collision while LIVE: refuses, exit 1, routed** — `` task `back-fill-the-pre-v1` is already active / route: resume with `jigc start --task …` or abandon with `jigc task discard …` `` — no silent resume, no clobber. Note the message frames it as *re-minting the same task*; for a genuinely different intent that framing is the only wobble (it can't know), but the state outcome is safe. **This half of the exposure is closed.**
3. **Refuted sub-claim:** P3's "there's no `--id` flag … I can't avoid this" — `jigc start --slug <id>` exists (M39) and verified live: `--slug back-fill-pre-v1-docs` minted the named id. It appears in `--help` only; no composed surface names it at mint — the pull-tier lens again, not a missing capability.

M44's path-hash confirmed migration-only, live in the same session: the D2 migrate task minted `migrate-completion-record-notes-v1-close-faaba72f53e7` (blake3 suffix); no work-workflow id carries one. Also verified: commit messages carry no task id (type+summary only), so post-hoc disambiguation of arm-1 aliasing has no committed record either. Fix-shaped: a one-line mint ack rider when the minted id matches a finalized task's id in the invocation log, + the mint surfaces naming `--slug`.

```sh
jigc start --workflow quick-fix "back-fill the pre v1 zero release notes"   # task minted: back-fill-the-pre-v1
# …finalize it clean…
jigc start --workflow quick-fix "back-fill the pre v1 documentation for the unreleased work"
                                          # task minted: back-fill-the-pre-v1 — same id, no warning
jigc start --workflow quick-fix "back-fill the pre v1 completely different thing"   # (while live) exit 1, resume/discard route
jigc start --workflow quick-fix --slug my-own-id "back-fill the pre v1 x"            # collision avoidable, undiscovered
```

## D9 — the commit doc is invisible until it fails — **CONFIRMED** (law 3 — a mandatory author-required contract whose first appearance is the block message) (Medium)

The full composed `record-decision` on rc.9 captured (57 lines). Every "commit" token audited: line 15 "commits you to" (prose), lines 40-46 the finalize/staging paragraph ("Validate and commit the task as one logical commit… finalize commits only the staged set **plus the docs it manages**"), lines 48-53 the validate/resume footer. **Nothing names `commit:<task>`, nor that a commit doc exists, nor that its `type` and `summary` are author-required** — the footer's `create-gates: adr` doesn't count the commit doc (it's intrinsic, not gated), and "the docs it manages" is the only, fully opaque, allusion. The workflow narrates ADR authoring verb-by-verb, then jumps to finalize — P2's description is exact.

The discovery-by-failure blockers, live at `task validate` (exit 3 — same findings the finalize gates on):

> `blocking · schema-conformance.field-value-conformant — `commit:record-the-choice-to-use`: field `type` in section `header`: "" is not a member of enum "type" (allowed: feat, fix, docs, style, refactor, perf, test, build, ci, chore, revert)`
> `blocking · schema-conformance.required-slot-present — `commit:record-the-choice-to-use`: required slot in section `summary` is empty`

The routes are followable (exact `set-field`/`set-slot` commands — law 2 holds at the block), and lacon B11 already established the vocabulary is discoverable *after* contact. The ambush is that a workflow which walks every other required write never mentions this one: two sentences in the finalize paragraph ("finalize also renders `commit:<task>` into the commit message; set its `type` + `summary` first: …") close it. Checked the sibling: `single-task` composes explicit `set-commit-type`/`set-commit-summary` steps — so the gap is `record-decision`-shaped (and by P2's session, its cost lands on precisely the workflow sold as "no code to write").

```sh
jigc start --workflow record-decision "record the choice to use sqlite for storage" | grep -in commit
# author the ADR per the composed text, then:
jigc task validate record-the-choice-to-use    # exit 3 — the two commit-doc blockers, first mention anywhere
```

## D10 — two code-anchor fields, two accepted shapes, indistinguishable in `doc schema` — **PARTIAL: the indistinguishable-projection half is CONFIRMED; the "implemented-by rejects a bare path" half is REFUTED live** (Low-Medium; the real defect is an unstated *check-activation* difference, law 2)

**Projection half — confirmed.** Both render as bare `code-anchor`, no shape, no check, no distinction:
- `jigc doc schema adr`: `- cites-code: code-anchor (section: status) (set-field: adr:<slug>#status/cites-code)`
- `jigc doc schema arch-doc`: `- implemented-by: code-anchor (set-field: arch-doc:<slug>#components/<id>/implemented-by)`

**Rejection half — refuted at every gate.** Live: `implemented-by = apps/api/index.js` (bare path) was accepted at write (exit 0), passed `task validate`, passed `finalize` (doc promoted, exit 0), and passed store-scope `jigc validate` — even a bare *directory* (`apps/api`) passed. Same for adr `cites-code` (P1's half that already matched). Source agrees: both fields are the same pack type (`crates/cli/pack/config/field-types.yaml:11` — one `code-anchor`, adjudicator `doc-code`, check `symbol-exists`); the engine treats "a bare path (no `#symbol`)" as "itself the file" (`crates/engine/src/validate.rs:8202`), so `symbol-exists` degrades to file-existence. On rc.9 there is **one accepted grammar, everywhere** — no gate wants `path#symbol`. P1's remembered rejection did not reproduce and no code path shaped like it was found (their block was plausibly D2's `owner-artifact.present` arc or a dangling-path case, which *does* block — different finding).

**What is real underneath the claim (law 2):** the *checking* differs silently by value shape — a symbol-bearing anchor buys `symbol-exists` + (on arch-doc) `title-names-symbol`; a bare path buys file-existence only, and `title-names-symbol` never evaluates (verified: the D3 mismatch fired only after the `#symbol` was added). No surface states this: the schema projection can't express "prefer `path#symbol`; a bare path weakens the check to file-existence", and `arch-doc`'s own YAML rationale (anchor-the-symbol) is invisible to users. Fix-shaped: the `doc schema` type cell gains the accepted grammar + check (`code-anchor (path[#symbol]; bare path = existence-check only)`).

```sh
jigc doc set-field 'adr:use-sqlite-for-storage#status/cites-code' --value apps/api/index.js --task <t>       # accepted, finalizes clean
jigc doc set-field 'arch-doc:api-services#components/<id>/implemented-by' --value apps/api/index.js --task <t> # accepted, finalizes clean, store-validates clean
jigc doc schema adr; jigc doc schema arch-doc    # both: bare "code-anchor"
```

## D11 — bare `jigc start "<intent>"` is a no-op round trip repeating the SessionStart list — **CONFIRMED** (works-as-designed — the agent-is-the-router invariant — but the claim's facts all hold) (Low, one call per task)

Captured and diffed both outputs in one repo. The **twelve workflow one-liners are byte-identical** between orientation (what the SessionStart hook prints — the hook is literally `jigc start`) and the with-intent router output. The router output contains **zero intent-specific content**: the intent string appears 0 times (grep), no reordering, no filtering, and the re-run line prints the literal placeholder `jigc start --workflow <chosen> "<intent>"` — it does not even echo the user's own intent back into the copy-ready line. Deltas are framing only (orientation adds the pack/config header, the Preview line, and the planning/ingest footer; the router adds "Pick the workflow whose situation best fits the intent"). So within one session the with-intent call adds nothing the agent doesn't already have — P2's "mandatory no-op round trip" is exact. The design answer is on record (lacon B1: the CLI does no selection by invariant; `ideas/spec-router-matching.md` is the residue) — this is its 2nd trial recurrence, plus one strictly mechanical papercut that engages no invariant: **substituting the real intent into the re-run line** (the composer already holds it — `{{task.intent}}` is engine-native) would at least make the round trip emit something copy-runnable.

```sh
jigc start > a.txt; jigc start "split the docker image build into two stages" > b.txt
diff a.txt b.txt          # one-liners identical; framing-only deltas
grep -c docker b.txt      # 0
```

## D12 — "advisory" is scope-dependent; only trailer lines distinguish never-gates from gates-later — **CONFIRMED, and sharpened: the mixed trailer counts but does not name, and its gate-count claim is false for cascade-demoted codes** (Medium; law 1 residual inside the M42-corrected trailer)

All three trailer forms located (`store_trailer`, `crates/cli/src/render.rs:698-755`), two captured live:

- **Pure never-gates** (r2, adoption advisories only): `2 finding(s) — report-only at store scope (exit 0); each gates nowhere — a store-scope advisory, actionable through its own route above.`
- **Mixed** (r1, `doc-code.title-names-symbol` + `schema-conformance.repeatable-populated`): `2 finding(s) — report-only at store scope (exit 0); 1 of them gate at `jigc task validate` / `jigc task finalize`; the rest are store-scope advisories that gate nowhere — follow each finding's route above.`
- All-gate form ("these gate at …") per source, same function.

P1's claim confirmed as stated: both findings in the mixed run render with the **identical `advisory ·` label** — the severity vocabulary carries zero gate information; the trailer is the only discriminator, and **the mixed form does not say which** finding is the gating one (the per-finding `(gates at finalize)` label exists but only for blocking-class findings, M42 Inc 12). Sharpened defect found under it: the counted "gating" finding here is `title-names-symbol`, whose cascade severity is **advisory at task scope too** (the M40 demotion) — driven live, it sat in a `task validate` that exited **0**. `gates_at_task` (`render.rs:632-647`) asks "does a task-scope path emit this code" and never consults cascade severity, so the trailer told this store that 1 finding "gates at `jigc task validate` / `jigc task finalize`" when nothing in the corpus can block anything — the exact false-gate-claim class M42's fix ("the trailer must not claim a gate that does not exist") was built to close, surviving through the severity axis. Fix-shaped: `gates_at_task` intersects with resolved cascade severity, and the mixed trailer names codes.

```sh
# r1: commit arch-doc w/ mismatched symbol title; leave changelog releases empty
jigc validate           # mixed trailer: "1 of them gate at …" — but that finding is advisory at task scope
jigc task validate <t>  # exit 0 with the same finding present — the claimed gate never fires
# r2: conformant hand-authored CHANGELOG.md only
jigc validate           # pure form: "each gates nowhere"
```

---

# The through-line, against the laws

- **Law 1 (nothing lies):** D3 (heuristic narrated as event detection), D4 (a path that isn't there), D1 (an affordance quantified over workflows it refuses), D12 (a gate claim false under the severity axis) — four true-mechanism/false-sentence findings.
- **Law 2 (nothing hides):** D8 (`--slug` + the loud live-collision guard exist; nothing at mint names them), D10 (the check-activation difference between bare path and `path#symbol` is stated nowhere).
- **Law 3 (nothing ambushes):** D2 (the root of the artifact home first inferable from the second block) and D9 (the commit doc's contract first printed as its own block) — both are constraints that bind at finalize and are stated in no soliciting surface.
- **Works-as-decided, value contested:** D7 (M42 fork 6, 3rd corpus), D11 (the no-LLM router invariant, 2nd corpus).
- **Message-tier honesty holds where it was engineered:** every block in every repro carried a parseable route (the M41/M43 floors held under all of this session's provocation); D2's is the one route observed steering *into* a second block.
