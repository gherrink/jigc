# The six script-side tasks of the core, reviewed for three dangers

Reviewed at `bbe873eb`, in a clone of its own with no remote, on macOS with git 2.54,
python 3.9.6 and gitleaks 8.30.1. The unit is six tasks — K1 `e76ef470`, K2 `b34fb073`,
K4 `3c6a889d`, K5/K6 `8f13ea7b` `b36b8c93`, K9 `c2169e19`, K10 `bbe873eb` — and the question
is three dangers only: publish something unscanned, record or compute something false,
strand state. A finding that only halts on the safe side is a line of the appendix.

## Verdict

**No finding is graded BLOCKS.** Nothing an ordinary run or an ordinary failure does
publishes unscanned bytes, and no arrival state I drove is left unnamed. The six tasks are
fit for the small canary as they stand.

For the first real run they are fit by the grading this review was handed. Two rows are
nevertheless the irreversible danger and cost a few lines each, and I would land them
first: **F1** (the push is by name, after a vet of a list read earlier) and **F2** (the
range is read off remote-tracking refs the remote may not bear out — and that also
switches off K2's foreign-commit refusal). **F3** needs a ruling before `K3` wires `close`:
every ordinary re-run records its results on a commit that is not the round's candidate,
and nothing holds or names that.

Eight rows, nine leads, seven appendix lines. Of the turned fences, one is weakened.

## How each count was reached

Every row below is **an instance** unless it says how its class was bounded. Where a class
is bounded, the read is named.

## Repro preamble (P0)

Every block starts from a fresh P0 unless it says it continues. It needs git, python3 and
a real `gitleaks` on `PATH`. The denylist is one synthetic term.

```sh
S=<scratch>                                   # a directory outside every repository
R=$(mktemp -d "$S/core.XXXXXX")
git clone --no-hardlinks <this repository> "$R/repo"
git -C "$R/repo" checkout --detach bbe873eb && git -C "$R/repo" remote remove origin
printf 'zzqforbidden\n' > "$R/denylist"; export JIGC_DENYLIST_FILE="$R/denylist"
rig() {  # rig <run> [--items gate]  -> prints the rig's root
  root=$(mktemp -d "$R/canary.XXXXXX")
  ( cd "$R/repo" && dev/stabilize-canary setup --root "$root" --commit HEAD --run "$@" \
      --previous 1.0.0-rc.24 --previous-commit "$(git rev-parse HEAD~40)" ) >/dev/null || return
  echo "$root"
}
gs() {   # gs <run>: the read every stage starts from, in the rig's clone
  dev/stabilize-step git-state --stage test --loop fix/$1 --rounds fix/$1-r \
    --run-dir completions/artifacts/$1 --product crates; echo " exit=$?"
}
```

## Rows

### F1 — a commit that lands between the vet and the push is published unvetted (K1, `e76ef470`)

`dev/stabilize-step:878` lists what the remote lacks, `:880` vets that range, and `:886`
pushes **the branch by name**. Whatever the branch stands at when `git push` runs is what
is published; nothing compares it with what was vetted, and the read-back at `:889-891`
compares the remote with the branch as it is *then*. K2's `foreign-commit` refusal
(`:829-837`) is computed before the vet and is passed by the same commit. The line that
comes back says `vetted: [A]` and `remote_head: B`.

K2's own table has `head-moved` because somebody commits on the loop branch while a stage
runs; the window here is the two scanners' run, a few seconds, at each push. What lands
in it would otherwise be pushed by its author with plain git, so the added exposure is
small — but it is the one act the tool says it never does.

The fix is to push the commit that was vetted (`<sha>:refs/heads/<branch>`); the suite's
`CHANGING` table (`tooling-tests/dev_stabilize_step.rs:5119`) pins the shape `push origin
<branch>` and has to be turned with it.

**Class:** instance, bounded to one site — the tool has one push, in one helper (the
suite's own assertion at `dev_stabilize_step.rs:5669-5678`, read), so `git-state`, `push`,
`land` and `carry` all share it.

```sh
# F1 — after P0
C=$(rig core-a); cd "$C/clone"; D=completions/artifacts/core-a
mkdir -p "$R/racebin"; REAL=$(command -v gitleaks)
cat > "$R/racebin/gitleaks" <<EOS
#!/bin/sh
# stands in for a second committer: a commit lands on the branch while the range is scanned
case " \$* " in *" --log-opts "*) if [ ! -e "$R/raced" ]; then : > "$R/raced"
  ( cd "$C/clone" && mkdir -p docs-x && printf 'zzqforbidden raced\n' > docs-x/raced.md \
    && git add docs-x/raced.md && git commit -q -m "landed while the vet ran" ) >/dev/null 2>&1
fi ;; esac
exec "$REAL" "\$@"
EOS
chmod +x "$R/racebin/gitleaks"
printf 'a note\n' > $D/note.md; git add $D/note.md; git commit -q -m "a record-shaped commit"
PATH="$R/racebin:$PATH" gs core-a
#   -> exit=0, status ready, "vetted": [<the note's commit>], "remote_head": <another commit>
git --git-dir="$C/origin.git" show fix/core-a:docs-x/raced.md
#   -> zzqforbidden raced      (outside the run's directory, denylisted, vetted by nothing)
```

### F2 — a remote-tracking ref the remote does not bear out switches off the vet and the foreign-commit refusal (K1 `e76ef470`, K2 `b34fb073`)

Both reads are `<branch> --not --remotes=origin` (`dev/stabilize-step:834-835`, `:878`).
With one ref under `refs/remotes/origin/` standing at a local commit, `records_only` finds
nothing foreign, `push` finds nothing to vet, and `git push` publishes everything. The
line says `found: [unpushed]`, `finished: [unpushed]`, `vetted: []`.

`DECISIONS.md` (the entry of K1, *Left open, and declared*) names the stale ref and gives
the reason not to check it: *the commits were published once*. That holds for a ref a
push or a fetch of **this** remote moved. It does not hold for a ref set by hand, or for a
checkout whose `origin` was pointed at another repository after the ref was written — and
the entry does not say that K2's refusal goes with it. The tool has what it needs to
notice: `remote_head` differs from the local head while the range is empty.

**Class:** instance, bounded to the two reads above — `--remotes=origin` stands in two
functions of the tool (grep of `dev/stabilize-step`: `records_only`, `push`; the header and
two refusal texts repeat it) and once in `vet --range`'s caller-given range.

```sh
# F2 — after P0
C=$(rig core-a); cd "$C/clone"
mkdir -p docs-x; printf 'zzqforbidden outside\n' > docs-x/leak.md
git add docs-x/leak.md; git commit -q -m "a task's commit"
gs core-a                                   # control -> exit=26, refused foreign-commit, nothing pushed
git update-ref refs/remotes/origin/fix/gone-branch HEAD   # a ref of a branch the remote does not hold
gs core-a                                   # -> exit=0, ready, finished [unpushed], "vetted": []
git --git-dir="$C/origin.git" show fix/core-a:docs-x/leak.md     # -> zzqforbidden outside
```

### F3 — the commit of a result is held to nothing once the round has its candidate, and `close` is computed over it (K4, `3c6a889d`)

`round-set` refuses a candidate while the round's results name another commit
(`tested_on`, `dev/stabilize-record:2831-2839`: *a round tests one candidate*).
`result-set` (`:2473-2576`) holds its `--commit` to nothing after that: a re-run, and the
first run of an item added since, is taken on any commit. `judged` (`:3077-3145`) never
reads a result's commit, so every clause turns green and `next` is `close` with
`forbids_close: []`, `candidate.commit` the round's and `current: true`, while no green is
of that commit.

This is reached by the harness as committed, on every re-run: it records a re-run's
results with the branch's head (`.claude/workflows/stabilize.js:2543`, `:2670`), which is
the candidate plus the round's record commits — where the same file hands the verifiers
the round's own candidate on purpose (`:2526-2529`). In an ordinary run the two differ in
no product path, which is why this is a row; a commit outside the product paths between
them makes the gate's green one of another tree. K4's claim is that the inputs of `close`
are held where they are written, and its list of what is *named or a bound* has no word
on this one.

**Class:** instance. The writers of a result are one (`result-set`; grep of `WRITERS`),
and the two paths into it after the candidate are the re-run and a new item's first run —
both driven below.

```sh
# F3 — after P0
C=$(rig core-b --items gate); cd "$C/clone"; REC=dev/stabilize-record
CAND=$(git rev-parse HEAD~1); OTHER=$(git rev-parse HEAD~5)     # the rig's candidate, and an older ancestor
echo '{"included":[{"door":"jigc doc list","registry":"-","derivation":"named by the caller"}],"excluded":[]}' \
  | $REC scope-set --run core-b --round 1
echo '[{"item":"gate","outcome":"void","reason":"the gate was cut off"}]' | $REC result-set --run core-b --round 1 --commit $CAND
echo "{\"candidate\":\"$CAND\"}" | $REC round-set --run core-b --round 1
echo '[{"key":"canary-seeded-claim","grade":"no-break"}]' | $REC triage-set --run core-b --round 1
echo '[{"item":"chk-a","kind":"check","clause":"no-lost-files","runs":"every-candidate","brief":"a check"},
       {"item":"chk-b","kind":"check","clause":"usable-by-agents","runs":"every-candidate","brief":"a check"},
       {"item":"chk-c","kind":"check","clause":"migration-works","runs":"every-candidate","brief":"a check"}]' \
  | $REC item-set --run core-b
echo '{"go":true}' | $REC round-set --run core-b --round 1          # next is now `retest`
echo '[{"item":"gate","outcome":"green"},{"item":"chk-a","outcome":"green"},{"item":"chk-b","outcome":"green"},{"item":"chk-c","outcome":"green"}]' \
  | $REC result-set --run core-b --round 1 --commit $OTHER; echo "exit=$?"     # -> exit=0
$REC state --run core-b | python3 -c "
import json,sys; d=json.load(sys.stdin)
print(d['next'], d['candidate'], d['forbids_close'], [(c['clause'],c['status'],c['commit'][:8]) for c in d['clauses']])"
#   -> close {'round': 1, 'commit': <CAND>, 'current': True} [] [four clauses, each green, each on <OTHER>]
```

### F4 — `discard` takes back a batch whose commit is pushed, on the tool's own advice (K2, `b34fb073`)

The step's `discard` refuses a committed batch only where no file of it was altered
(`dev/stabilize-step:1392`: `not batch["altered"] and …`). The arrival row `altered`
(`:679`) tells its reader to run exactly that act. So a batch that is in HEAD — and on
the remote — and one of whose files a hand then touched is taken back: the round's tables
leave the tree, the journal is gone, and `dev/stabilize-record state` answers `next: test`
with no candidate for a round whose record the remote holds. `git-state` names what is
left `dirty`, the human's, and no commit is lost — it halts — but the road in is the
command the refusal spells.

Reaching it needs a commit step killed between its commit and its settling (the arrival
state `committed`), then a push that does not reconcile — `push --branch`, which reads no
journal (`push_branch`, `:1182-1189`), and which is what the harness composes today
(`stabilize.js:1193`, no `--run-dir`) — then a hand in a table.

**Class:** instance.

```sh
# F4 — after P0
C=$(rig core-c --items gate); cd "$C/clone"; RUN=core-c; D=completions/artifacts/$RUN
CAND=$(git rev-parse HEAD~1); SC="$C/scratch"
dev/stabilize-step begin --run $RUN --round 1 --stage test --attempt 1 --reporter attempt --commit $CAND --scratch "$SC" >/dev/null
echo '{"included":[{"door":"jigc doc list","registry":"-","derivation":"named by the caller"}],"excluded":[]}' \
  | dev/stabilize-record scope-set --run $RUN --round 1 >/dev/null
python3 - > "$SC/batch.json" <<EOS
import json
print(json.dumps([
 {"argv":["result-set","--run","$RUN","--round","1","--commit","$CAND"],"stdin":json.dumps([{"item":"gate","outcome":"green"}])},
 {"argv":["round-set","--run","$RUN","--round","1"],"stdin":json.dumps({"candidate":"$CAND"})}]))
EOS
dev/stabilize-record apply --run $RUN --round 1 --subject "docs(record): core-c r1" < "$SC/batch.json" >/dev/null
# what a commit step killed between its commit and its kept answer leaves: the batch in HEAD, not settled
git add -- $D/r1 $D/clauses.md; git commit -q -m "docs(record): core-c r1"
dev/stabilize-step push --branch fix/$RUN >/dev/null; echo "push exit=$?"      # -> 0: the record is on the remote
printf '\n<!-- a hand was here -->\n' >> $D/r1/results.md
gs core-c          # -> exit=4, found [altered]; the recommendation spells `dev/stabilize-step discard …`
dev/stabilize-step discard --branch fix/$RUN --run-dir $D; echo " exit=$?"      # -> 0, discarded: three files
git status --short                                 # ->  M clauses.md   D r1/results.md   D r1/round.md
dev/stabilize-record state --run $RUN | python3 -c "
import json,sys; d=json.load(sys.stdin); print(d['next'], d['candidate'])"
#   -> test {'round': None, 'commit': None, 'current': False}
```

### F5 — an added line that opens with two plus signs and a space is read by no denylist scan of a range (K1's push vet; the defect is older, in `dev/hygiene-scan`)

`dev/hygiene-scan:144` takes every diff line that opens `+++ ` for a file header and
moves on. An added line whose own text opens `++ ` is printed by `git log -p` as `+++ …`
and is never matched — at a push, and in CI's `hygiene` job, which runs the same script.
The writers and the commit's vet are not blind to it: they scan a tree (`--tree`,
`dev/stabilize-record:1673`). So it needs a commit made by hand — the case K1 says the
push closes. A shell trace (`set -x` prints `++ …` for a nested command) is where such a
line comes from.

**Class:** bounded by reading the scanner's three skip rules (`dev/hygiene-scan:143-145`):
the commit marker and the hunk header cannot be opened by an added line, which always
carries its `+`; this is the one shape that can. Driven directly too: the script over the
one commit answers `denylist clean`, exit 0.

```sh
# F5 — after P0
C=$(rig core-a); cd "$C/clone"; D=completions/artifacts/core-a
printf 'a note\nzzqforbidden here\n' > $D/notes.md; git add $D/notes.md; git commit -q -m "a note"
gs core-a                                   # control -> exit=22, refused unvetted (hygiene)
git reset -q --hard HEAD~1                  # in the rig: the commit taken back
printf 'a note\n++ zzqforbidden here\n' > $D/notes.md; git add $D/notes.md; git commit -q -m "a note"
gs core-a                                   # -> exit=0, ready, finished [unpushed]
git --git-dir="$C/origin.git" show fix/core-a:$D/notes.md      # -> the line, on the remote
dev/hygiene-scan "$R/denylist" HEAD^..HEAD; echo "exit=$?"      # -> denylist clean, exit=0
```

### F6 — a secret-shaped string in a commit's message passes every push (K1, `e76ef470`)

gitleaks reads added lines and never a message. At the commit boundary the record script
makes up for that: the subject is a file of the scan's throwaway repository
(`dev/stabilize-record:4171`). At the push it does not: `range_scan` (`:4245-4259`) is
gitleaks over the range, and the message check of `vet_range` (`:4310-4319`) is the host
path alone. So the one check of the writers that a push does not repeat is the secret
scan of what a commit says — in a subject or a body. CI is blind in the same place. A
record's own commit carries the subject that was scanned; a commit made by hand, and —
in the `fix` half — every commit a fixer makes, does not.

The suite's arm *in each place a commit publishes* (`dev_stabilize_step.rs:2944-2981`)
drives the denylisted term in a file, a subject and a body, and no credential in a
message; the stand-in for gitleaks reads a tree, so it could not show this.

**Class:** instance; the two scanners are two (`scan_tools`, read), and only one reads
messages.

```sh
# F6 — after P0
C=$(rig core-a); cd "$C/clone"; D=completions/artifacts/core-a
# built here, never spelled: a token of a shape gitleaks' default rules name
TOK="ghp_$(python3 -c "import random,string; random.seed(7); print(''.join(random.choice(string.ascii_letters+string.digits) for _ in range(36)))")"
printf 'a note\ntoken = %s\n' "$TOK" > $D/n3.md; git add $D/n3.md; git commit -q -m "note three"
gs core-a                                   # control -> exit=22, refused unvetted (hygiene)
git reset -q --hard HEAD~1
printf 'a note\n' > $D/n3.md; git add $D/n3.md; git commit -q -m "note three" -m "token = $TOK"
gs core-a                                   # -> exit=0, ready, finished [unpushed]: the message is on the remote
```

### F7 — the fence on the files the tool opens no longer reddens for a file opened in the tool's own way (K10, `bbe873eb`)

Before K10 every file the tool opened was a line with `open(` and the scan counted them
(six at K9). K10 added two helpers — `whole(found)` reads any path, `kept_once(at, text)`
writes any path — and the table `OPENS` lists the helpers' own two lines, each once. A
new caller of either is no new line with `open(`: the scan is green over a file written
inside the repository, and over any file read. The commit calls the table *stricter than
the count*; for a direct `open(` it is, and the planted offender is one.

**Class:** bounded — the tool's helpers that open a path handed to them are these two
(grep of `def ` bodies holding `open(`: `whole`, `kept_once`; every other site names its
own file).

```sh
# F7 — after P0; needs the tooling group built once:
#   ( cd "$R/repo" && CARGO_TARGET_DIR="$R/target" cargo test -p jigc --test g_tooling --no-run )
cd "$R/repo"
python3 - <<'EOS'
p="dev/stabilize-step"; s=open(p,encoding="utf-8").read()
a="def push_branch(args, facts):\n"
assert s.count(a)==1
open(p,"w",encoding="utf-8").write(s.replace(a, a+'    kept_once(os.path.join(ROOT, "planted.txt"), "x")\n'))
EOS
CARGO_TARGET_DIR="$R/target" cargo test -p jigc --test g_tooling -- \
  dev_stabilize_step::the_tool_runs_no_git_command_an_agent_may_not \
  dev_stabilize_step::a_planted_offender_reddens_the_scan \
  dev_stabilize_step::the_table_names_every_act_and_every_arrival_and_each_is_driven
#   -> 3 passed
git checkout -- dev/stabilize-step
```

### F8 — two of the four things the commit step's kept answer is believed by are held by no test of its suite (K2, `b34fb073`)

`record` asked again answers `recorded` from `<gate file>.recorded.json` only where
`--head` is passed, the calls and checks are the ones asked, the branch and **the commit
are the ones the file names**, and HEAD's parent is `--head`
(`dev/stabilize-step:1299`). The code is right: a kept answer for a commit that is not on
the branch is answered `no-batch`. But with the commit's equality dropped, and with the
calls-and-checks equality dropped, the whole step suite stays green — 67 of 67, both
mutants at once. The parent's equality is held (`a_second_identical_call_finds_what_is_done`
is red without it).

**Class:** bounded to the four conjuncts of that one condition, each mutated alone: two
red or structural (`--head` present, the parent), two green.

```sh
# F8 — after P0, with the tooling group built (F7)
cd "$R/repo"
python3 - <<'EOS'
p="dev/stabilize-step"; s=open(p,encoding="utf-8").read(); o=s
s=s.replace('(kept.get("branch"), kept.get("commit")) == (current, head)','kept.get("branch") == current',1)
s=s.replace('args.head is not None and applied == (int(args.calls), int(args.checks)) and','args.head is not None and applied is not None and',1)
assert s!=o; open(p,"w",encoding="utf-8").write(s)
EOS
CARGO_TARGET_DIR="$R/target" cargo test -p jigc --test g_tooling -- dev_stabilize_step::
#   -> 67 passed; 0 failed
git checkout -- dev/stabilize-step
```

## The fences that were turned

Each was put to a mutant in the clone, applied alone, the four fence tests run, the clone
restored. *Red* is the fence failing.

| Fence | Task | A new unlisted member | A listed member that is gone | Verdict |
|---|---|---|---|---|
| `NO_STAGES`, the acts no stage asks for | K2 | an act `sweep` added to the tool's parser: red | `hash` taken out of the parser: red; a listed act the script asks for is refused by an assertion I read and did not mutate (`stabilize_harness_fence.rs:1689-1693`) | **holds both ways** |
| four more rows of `NO_STAGES` | K10 | as above | as above; three rows say *until K11*, and the second direction is what makes K11 take them out | **holds both ways** |
| the files the tool keeps (`table`) | K9, K10 | a row added to `KEPT`: red | a row removed is red by the same equality | **holds both ways — as an equality of two written lists.** It is tied to no write: with the supervisor no longer writing `exit.json`, and with a new file written through `kept_once`, this fence is green. It was that before the turn; the turn did not weaken it |
| the files the source opens | K9 → K10 | a direct `open(`: red (planted). A file opened through `whole` or `kept_once`: **green** | a listed site that is gone: red | **weakened** by K10 — row F7 |
| the roads to a process, and one fork | K10 | `subprocess.Popen`, `os.fork()`, `os.setsid()`: red (planted). `os.posix_spawn(…)` and `os.forkpty()`: green | `streamed` starting nothing: red | **holds both ways for the spellings it lists.** The two green ones were green before the turn too; *one fork* is as wide as the text `os.fork(` |
| the programs on the roads | K10 | a fifth `return` in `held_command`: red | — | **holds**; a constant pointed elsewhere (`GATE = "/bin/sh"`) is green at the fence, and I did not run the behaviour arms under it |
| the git subcommands (`archive`) | K10 | `archive` with an unlisted flag: red | the `archive` call removed: red | **holds both ways** |

## Leads — noticed, not pursued to a verdict

- **A held gate names no tree.** `hold-start --kind gate` takes `--run` alone
  (`dev/stabilize-step:660`); a start is answered by name with the job that is there
  (`:1727-1734`), so under one name a second tree is answered with the first tree's
  verdict; and `held_result` holds a verdict to the call's commit only where `asked` has
  `candidate` or `commit` (`dev/stabilize-record:2604`), which a gate's has not. Declared
  in part (*of the call's commit where the command was asked for one*). Read, not driven:
  the name is all that separates two gates, and it is `K11`'s to choose.
- **A held regression verdict is not held to the run's previous release.** `held_result`
  compares the candidate and nothing else; `previous-commit` is a fact of the run and is
  not asked. The committed list makes a wrong previous hard — its `commit:` rows are held
  to *not in the previous release* (driven: the candidate's parent as previous is refused
  `list`) — so this needs another list as well.
- **An item's entry in the state's digest drops its five standing fields where they are
  null.** Absent and null are one thing in the document; a harness that compares with
  `=== null` reads absent as something else. `K3`'s.
- **`unfit` is not a refusal.** A field that does not fit is printed null and named
  without its index (`state.items.why`). A reader that does not stop on a non-empty
  `unfit` reads a null it cannot tell from a real one. `K3`'s.
- **`forbids_close.candidate` is a flag in the digest** where the document has the word
  and the round. Whether the harness acts on the word I did not census.
- **gitleaks reads no merge's own lines** (no remerge diff), where the denylist scan
  does. A landing's merge is the tool's own and its conflicts are the logs'; CI is the
  same. Not driven.
- **Attributes kept outside the tree** (`info/attributes` of the git directory, marking
  paths `-diff`) would blind both scanners over a range and show in no `git status`. Not
  driven; a deliberate hand.
- **`git-state` making the owed push while a batch is applied** — the guard removed, four
  tests I ran stay green. Not run suite-wide, and what it would push is an earlier
  record, vetted.
- **`dev/hygiene-scan` prints up to twelve characters of a message line that opens with
  the byte 0x01** as if they were a commit's id — a hit, so the safe side, but by what
  and not by where. Driven once; older than these tasks.

## Appendix — real, and only halts on the safe side

- A link at a report's path makes `dev/stabilize-record pending` refuse `outside`, so
  every act that reads the pending writes — `git-state`, `record`, `discard`, the report
  check — halts `record` and K1's first exit (set aside, reporter `missing`) is not
  reached. Driven.
- A `verdict.json` that is empty — a wait killed between making the file and writing it —
  is refused `missing` by every later read, and the job's verdict is lost with its
  thirty-five minutes. Driven with an emptied file.
- A start killed between its fork and its `job.json` leaves a command running under a
  name every act answers `missing` for. Read (`dev/stabilize-step:1746-1751`).
- Two commit steps at once over one batch: one records, one halts `git`; the run ends
  recorded and pushed. Driven.
- A kept verdict is believed by a hash of itself, and an item's kind can be corrected at
  any time: a caller can compose a verdict, and can turn a held check into a plain one
  and hand it a word. Both declared (K10's entry, item 9; K4's). The second driven:
  refused before the row, taken after it.
- The probe's kind, held without the probe begun, is `void` (`no-verdict`). Driven; as
  it should be.
- A supervisor killed: `dead`, `no-exit`, and a start under the same name answers `dead`
  again. Driven.

## What was checked and held

- **Reverting one decision reddens its test**, seven of seven: the push's vet;
  `git-state`'s foreign-commit read; a counted item's row; a candidate for a round whose
  results are of another commit; a held verdict of another commit; a gate verdict its
  exit does not bear out; a gitleaks that could not run, in a range.
- A link committed by hand under the run's directory is refused at the push (`outside`).
- A denylisted line and a secret-shaped line in a file of a hand-made commit are each
  refused at the push — with the real gitleaks, over the real range.
- The regression tool refuses a list by an absolute path, by a path that leaves the
  repository, a file only the working tree holds, and one commit named twice; over this
  commit and the committed list's own previous release it reads 33 rows.
- A started job is answered again by name; other arguments under the name are `taken`.

## What I did not check

- No real `dev/gate` and no real `dev/regression-set run` were started; the gate,
  regression and build kinds were read and their recorded fixtures trusted.
- No stage was run and no agent launched; the harness was read only where it hands the
  two scripts their arguments.
- No kill inside a git child. No Linux run.
- Whether the remote-tracking refs of the repository the real run will stand in are
  borne out by its remote: this clone has none, by instruction. It is one read before
  the opening (`git ls-remote origin` beside `git for-each-ref refs/remotes/origin`).
- The record suite, the simulation, the canary's and the probe's suites were not run
  whole; the `fix` half's acts were not attacked beyond the one push they share.

<!-- end of report -->
