# Stream T — what binds every builder of a tooling task

You build ONE task of the committed plan `completions/artifacts/M55/stabilization-build/reshape/plan.md` in the main working tree of `~/projects/gherrink-jigc`, branch `fix/rc24-tier1`. The tree is clean and nobody else works in it (another builder works in a separate worktree on another branch; read nothing from it, write nothing there). One commit, not pushed.

**RED FIRST.** Write the task's arms before the code, run them, and bring the red output back in your return. Where a test had to follow its code, say which, show the suite red against the parent commit, and kill the task's mutants.

## Read these yourself before you write anything

1. The plan — the top (abbreviations for write-sets), §2 and §3, §4 from its start through the migration order and its table (the three phases and the rule *a task carries the assertions it breaks*), **your task's paragraph** (its *Writes*, *What*, *Done when*, *Must not*), and the paragraphs of the task before and the task after yours, so that you know what is NOT yours. §5 where your task is a `K…` task. §6 and §12 where your paragraph cites them.
2. `DECISIONS.md` — the entries of this build, newest first, back to *2026-10-10 — The reshaped `test` stage, corrected by a count the same day*; among them *2026-10-11 — The reshaped `test` stage's three open questions, ruled* (two records per stage; a verifier hands on at most N = 2; only a queued route item holds the close; how often a code or a site is named is recorded and printed, and nothing is ranked, promoted or merged on it). **Read the entry of the task before yours whole**: it says what that builder chose, what it left to you, and what is not established.
3. `implementation/decisions-pending.md` → the rows your paragraph names.
4. `implementation/dev-workflow.md` (the loop and → Gate), `implementation/stabilization-workflow.md`, `implementation/public-hygiene.md`.
5. Every file of your write-set, whole, and its suite — the suite's rig, how an arm drives a whole stage's calls, how a run is opened.

## Rules

- **The phase's condition is the task.** In Phase A: a run whose opening holds no `order` answers every call as it does today and leaves a run directory with no byte it did not hold; the record suite's arm `a_run_without_an_order_holds_the_bytes_it_held` stays green **unmodified** — if your task cannot be built without changing that arm's reference, that is a halt.
- Your write-set is your paragraph's *Writes* and carried assertions only. A task carries the assertions it breaks: run the whole `g_tooling` target before you are done, and where a text you moved is pinned by another suite, change that assertion and nothing else in that file, and name each in the commit. Nothing under `crates/`; nothing under `.claude/`, not `CLAUDE.md` (those are the orchestrating session's steps, the plan's `O…`).
- Never `git stash`. Never push, never touch `main`, never amend. Commit with `git commit -F <file>`, never `-m`.
- The full `dev/gate` before the commit, on exactly the tree you commit (take the staged diff's hash before and check it after), in the background, read unpiped — never read an exit code through a pipe. No path is gate-exempt. The machine is loaded (another lane's gate runs beside yours): a test that hits the runner's time bound and passes alone is named in your return, and the whole gate is run again — never commit on a red gate or a partial one.
- Never `rm -rf` a path built from variables; `mktemp -d` under the session scratchpad `<scratch>`.
- Never kill a process by a pattern; only by pid, and only one whose rig directory is gone.
- The repository is public: no absolute host path, no agent or session id, no person's name, no environment variable's value in any file or commit message; placeholders `<scratch>`, `<tmp>`, `<repo>`. (The commit trailer below is the one exception, and it is the project's.)
- A halt is right when the paragraph cannot be built as worded or the phase's condition cannot be met: stop, commit nothing, return what you found with the lines that show it. A small decision the plan left open is yours: take it, mark it in the code, and say it first in your return with what you rejected.
- One new top entry in `DECISIONS.md`, in the style of the entries above it: what and why, what you chose where the plan left it to you, what the next task owes because of it, what is not established. Write it with the Write tool to a scratch file and insert it with `python3` and `io.open(..., encoding='utf-8')`.
- Commit trailer, exactly these two lines at the end of the message:
  ```
  Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>
  Claude-Session: <session>
  ```

## Return (compact; the detail lives in the entry and the commit)

Decisions you took, first · the commit's sha · the red output of each arm, briefly · the mutants and what killed each · every carried assertion by suite and name · the gate's totals line · what the next task owes · what is not established · anything you saw and did not touch.
