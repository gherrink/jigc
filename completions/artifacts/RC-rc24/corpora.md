# The corpora — one per arm, and how each is built

**Order is fixed: instantiate → gate → adopt → carry.** The gate runs on the naive corpus, while
its *no jigc residue* bar still means something. Nothing is planted in this trial, so there is no
fifth step.

All four come from `completions/trial-corpus-template/`. Each blind corpus has a product-plausible
name with **no `rc`, `trial`, `probe` or `gate` token**, unused by any earlier trial. The walk
corpus is not blind and may say what it is.

| corpus | product | arm | `instantiate.sh` | gate | adopted |
|---|---|---|---|---|---|
| `fenwick` | fenwick | **(a)** the fan-out | `--clean-prose` | 12 passed | yes |
| `halloway` | halloway | **(b)** the correction | `--clean-prose` | 12 passed | yes |
| `calderby` | calderby | **(c)** the disagreement | **no flag — the standing wart** | 11 passed, the prose bar **skipped** | yes |
| `walk-rc24` | walkfield | walk 00 · the environment probe · the seed smoke | `--clean-prose` | 12 passed | **no** — arm 00 adopts it itself |

Commands are run from the repository root. `TAG=jigc-gate:registry-1.0.0-rc.24`.

## Why (a) and (b) are clean and (c) is not

`calderby` is instantiated **without** `--clean-prose`, so it carries the template's standing
disagreement: `README.md`, `package.json` and the header of `src/store.ts` call the service *a
rollup cache … in front of whatever long-term store* the caller has, and `src/router.ts` has no
egress and no upstream reader. That disagreement is arm (c)'s whole instrument
([protocol.md](protocol.md) §5.1) and it is an accident the template kept, not a trap this trial
designed.

`fenwick` and `halloway` are instantiated clean so that arms (a) and (b) each measure one thing. A
worker that met the disagreement there would spend its budget on it, and a result in arm (a) or
(b) could not be told from a result of arm (c).

## 1 · Instantiate

A neutral git identity is set for the template's seven commits. `instantiate.sh` commits with
whatever identity the shell has, and a blind worker reads `git log`; a real name there is a
person's name in front of the worker and, if it is ever quoted into an intent, in the invocation
log this trial commits. The variables override git's configuration for these commands only; the
template tooling is not changed.

```sh
export GIT_AUTHOR_NAME='Corpus Owner'    GIT_AUTHOR_EMAIL='owner@example.invalid'
export GIT_COMMITTER_NAME='Corpus Owner' GIT_COMMITTER_EMAIL='owner@example.invalid'

T=completions/trial-corpus-template
$T/instantiate.sh --clean-prose ~/ideas/fenwick   fenwick
$T/instantiate.sh --clean-prose ~/ideas/halloway  halloway
$T/instantiate.sh               ~/ideas/calderby  calderby
$T/instantiate.sh --clean-prose ~/ideas/walk-rc24 walkfield

unset GIT_AUTHOR_NAME GIT_AUTHOR_EMAIL GIT_COMMITTER_NAME GIT_COMMITTER_EMAIL
```

`instantiate.sh` refuses an existing destination. The flag comes **first**; in any other position
it is refused rather than ignored.

## 2 · Gate

```sh
$T/check-corpus.sh ~/ideas/fenwick   --clean-prose      # 12 passed, 0 failed
$T/check-corpus.sh ~/ideas/halloway  --clean-prose      # 12 passed, 0 failed
$T/check-corpus.sh ~/ideas/walk-rc24 --clean-prose      # 12 passed, 0 failed
$T/check-corpus.sh ~/ideas/calderby                     # 11 passed, 0 failed, 1 SKIP
```

`calderby`'s twelfth bar prints `SKIP  prose contradiction not checked (corpus instantiated
without --clean-prose)`. That is the gate saying what it did not look at, and it is the expected
line for this corpus. Driven on a fresh instantiation: 11 passed, 0 failed, that skip.

**The wart is then asserted, not assumed.** A template edit that reworded any of the three sites
would hand arm (c) a corpus with no disagreement in it, and the gate would not notice:

```sh
grep -rnE 'in front of|long-term store' ~/ideas/calderby/src ~/ideas/calderby/README.md \
    ~/ideas/calderby/package.json
# expect exactly 4 lines: src/store.ts:4, src/store.ts:5, README.md:3, package.json:5
```

Any corpus that fails a bar is not a trial corpus: delete it and instantiate again.

## 3 · Adopt, through the container's own binary

```sh
A=completions/trial-driver/arms/adopt.sh
for c in fenwick halloway calderby; do
  ./completions/trial-harness/run-session.sh --exec "$A" ~/ideas/$c ~/out/RC24-adopt-$c "$TAG"
done
```

`adopt.sh` runs `jigc setup`, `jigc config set invocation-log true` and one `git commit`, and ends
`ADOPT-OK`. The log is switched on by the arm, not by the prompt, so the channel the rubric reads
does not depend on the worker running a command and the prompt carries no operator instruction.

Expected, and driven on the host registry binary: **7 template commits + setup's install commit +
the adoption commit = 9**, a clean tree, `CLAUDE.md` and `.jigc/AGENT.md` present.

**Neither adoption commit carries a co-author trailer, and neither should.** The script runs
outside Claude Code, so `CLAUDECODE` is not set: the install commit is jigc's and lands without
one by design, and the adoption commit is raw git. Both predate every session and are out of the
trailer check's scope; the install commit is its **control** row ([protocol.md](protocol.md) §6.2).

## 4 · Carry

`run-session.sh` writes its own evidence into the out-dir. Handing an out-dir on as a corpus plants
the rig's files in front of the worker, so each is carried:

```sh
for c in fenwick halloway calderby; do
  python3 completions/trial-driver/run.py carry ~/out/RC24-adopt-$c ~/ideas/$c-adopted
done
```

`carry` **replaces** its destination if one exists. The three `-adopted` names are unused.

## 5 · The frozen state, checked on each adopted corpus

```sh
for c in fenwick halloway calderby; do
  d=~/ideas/$c-adopted
  echo "$c: $(git -C $d rev-list --count HEAD) commits, $(git -C $d status --porcelain | wc -l | tr -d ' ') dirty"
  ls $d/PROVENANCE.txt $d/.session-transcript 2>/dev/null     # expect: nothing
  git -C $d log -2 --format='%s | %(trailers:key=Co-Authored-By,valueonly,separator=%x2C)'
done
```

Expect, for each: **9 commits, 0 dirty**, no rig evidence, and the two newest subjects
`chore: adopt jigc for document management` and `chore(jigc): install jigc workspace config`, each
followed by an empty trailer column.

**The invocation log is not empty at freeze, and that is expected.** The adoption commit fires the
pre-commit hook `jigc setup` installed, and the hook runs `jigc validate --format json` with the
log already on (driven: one record). `observe` leaves out everything older than the session's
start and says how many records that was.

Each corpus is used by exactly one arm, once. A re-run of an arm takes a new corpus under a new
name, and the record says why.

## What a worker sees

Inside the container the corpus is always `/work`; the host directory name never reaches the
worker. What does is the product name in `package.json` and `README.md`, the nine-commit history,
and the adapter `jigc setup` installed — `CLAUDE.md`, `.jigc/AGENT.md`, `.claude/settings.json`
with the allowlist, the deny floor and the SessionStart hook, and the guide at
`.claude/skills/jigc/SKILL.md`.
