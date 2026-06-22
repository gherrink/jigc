# Runbook — re-running the differentiator pilot

How to rebuild the harness and re-run the matrix from scratch (or with variations).
Everything lives in the throwaway workspace `~/diff-pilot/` (not in any managed
repo). The exported `harness/` dir here is a copy of the build files for reference.

## Prerequisites (verify first — these were the gotchas)

| Thing | Why it bit us | Check |
|---|---|---|
| **Docker** running | the isolation *is* the experiment | `docker ps` |
| **glibc ≥ 2.39 base** | `jigc`/`doc-code` need GLIBC_2.39 → use `node:20-trixie-slim` (glibc 2.41), **not** bookworm (2.36) | `docker run --rm node:20-trixie-slim ldd --version` |
| **Run as non-root** | Claude refuses `bypassPermissions` as root | images end `USER node`, `HOME=/home/node` |
| **OAuth creds mount** | container auth | mount `~/.claude/.credentials.json` read-only into the run user's `.claude/` |
| **No `/etc/claude-code/`** | org policy can't be disabled, would hit all arms | `ls /etc/claude-code/` → absent |
| **`--model` honored** | a silent fallback ruins the tiers | in-container modelUsage = the pinned id only (host adds an aux haiku call — ignore) |
| **`--permission-mode bypassPermissions` REQUIRED** | without it, headless `claude -p` permission-gates every tool — `jigc` and `Edit`/`Write` get **blocked**, the agent fumbles ("needs approval"/"catch-22"), and the run is garbage (looks like the agent "couldn't drive jigc" when really it was denied). Bit two one-off stream-json probes. | every headless run must pass `--permission-mode bypassPermissions`; if a transcript mentions "approval"/"blocked"/"catch-22", the flag is missing |
| **Pass the prompt via `-e PILOT_PROMPT`, never inline** | the task prompts contain backticks (`` `CommentBlockParser` ``); inlining them into a `docker ... bash -c '…'` lets the **container's** shell command-substitute them ("CommentBlockParser: command not found") → mangled prompt. Bit a one-off probe. | always `-e PILOT_PROMPT="$(cat prompt)"` + `claude -p "$PILOT_PROMPT"` inside; **for ANY ad-hoc run, copy `run-rep.sh`'s docker invocation verbatim** rather than hand-rolling |

## 1. Baseline twins

Baseline = `gherrink-ui-doc @ 542b3206` (pre-AI HEAD; its AI-era `docs/` doesn't exist).
Three **separate clones** (never worktrees — auto-memory is per-repo and leaks across worktrees):

```
mkdir -p ~/diff-pilot/clones
for arm in jigc plain gsd; do
  git clone -q ~/Projects/gherrink-ui-doc ~/diff-pilot/clones/$arm
  (cd ~/diff-pilot/clones/$arm && git checkout -q 542b3206 -b pilot-base)
done
```

## 2. Author the managed arch-doc on the jigc twin (the differentiator)

jigc cannot add a doctype via the project layer (the binary loads doctypes only from
its embedded pack), so use the **shipped `arch-doc`** doctype:

```
cd ~/diff-pilot/clones/jigc
jigc setup
jigc start --workflow architecture-documentation "Document the core public API surface"
jigc doc create arch-doc --title "Core Public API"
printf '<overview prose>' | jigc doc set-slot "arch-doc:core-public-api#overview" --from-file -
# per component:
ADDR=$(jigc doc add-item "arch-doc:core-public-api#components" --title "CommentBlockParser")
printf '<desc>' | jigc doc set-slot "$ADDR/description" --from-file -
jigc doc set-field "$ADDR/implemented-by" --value "packages/core/src/CommentBlockParser.ts#CommentBlockParser"
# fill the commit doc, then finalize (validates anchors, promotes, commits):
jigc doc set-field "commit:<task>#type" --value docs
jigc doc set-field "commit:<task>#scope" --value arch-doc
printf 'document core public API' | jigc doc set-slot "commit:<task>#summary" --from-file -
jigc task finalize <task>
```

Then copy the **byte-identical** promoted `docs/architecture/core-public-api.md`
into the plain + gsd twins and commit it there as a plain file (the static twin
derives from the plain image). This makes the doc surface identical across arms;
only the jigc twin's copy is *managed/validated*.

## 3. Build the four images

```
cd ~/diff-pilot
docker build -f docker/Dockerfile.toolchain -t pilot-toolchain .   # node20-trixie + git + pnpm
docker build -f docker/Dockerfile.deps      -t pilot-deps .        # = plain (baseline + node_modules), USER node
docker build -f docker/Dockerfile.jigc      -t pilot-jigc .        # + jigc & doc-code binaries (staged in docker/bin/)
docker build -f docker/Dockerfile.static    -t pilot-static .      # deps + static-methodology CLAUDE.md
docker build -f docker/Dockerfile.gsd       -t pilot-gsd .         # deps + `npx @opengsd/gsd-core@latest --claude --global --portable-hooks`
```

Smoke-test each: `docker run --rm -v <claude>:/usr/local/bin/claude:ro -v <creds>:/home/node/.claude/.credentials.json:ro <img> bash -c 'claude -p "create SMOKE.txt with OK" --model claude-haiku-4-5-20251001 --permission-mode bypassPermissions --output-format json; cat SMOKE.txt'`

## 4. Run the matrix

`run-arm.sh <arm> <model> <task> <prompt-file>` runs one isolated cell and captures
`transcript.json`, `changes.diff`, `README.after.md`, `arch-doc.after.md`,
`commits.txt`, `jigc-validate.after.txt`. `run-all.sh` drives all 16 (concurrency 2).

```
cd ~/diff-pilot && bash run-all.sh > runs/run-all.log 2>&1
```

Prompts (`prompts/task{1,2}.txt`) are byte-identical, neutral refactor tickets — no
doc/test hints, so each *method* determines completeness.

## 5. Measure + judge

- **Mechanical (objective):** grep each cell's final `README.after.md` +
  `arch-doc.after.md` for the OLD symbol name; any hit = doc↔code drift. Separate
  the arch-doc anchor / `### title` / prose surfaces (see `results.md` generator).
- **Blind judge:** build `judge/blind_input.txt` (final docs, arm-coded by a fixed
  shuffle in `judge/mapping.json`), feed to a judge that sees no arm labels.
  - Cross-model (preferred): `codex exec "$(cat judge_prompt.md)" < blind_input.txt`
    — **needs Codex quota** (was exhausted on this run).
  - Fallback: a fresh same-family subagent with only the blind input.

## 6. Export

Copy `pre-registration.md`, `VERDICT.md`, `results.md`, `judge/`, `arms/`,
`prompts/`, `harness/`, and every `cells/<task>/<arm>-<model>/` into
`completions/artifacts/<pilot>/`; regenerate `MANIFEST.sha256`.

## Cleanup (reversibility)

```
docker rmi pilot-gsd pilot-jigc pilot-static pilot-deps pilot-toolchain
rm -rf ~/diff-pilot
```
The exported artifacts are self-contained; the workspace + images are disposable.
