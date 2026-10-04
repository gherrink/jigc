# The turn files

Verbatim. Nothing here is edited between the screen and the run.

| file | what it is | how it is delivered |
|---|---|---|
| `a-turns.txt` | arm (a), **blind** — two turns | `run.py seed`, one turn per line |
| `b-turns.txt` | arm (b), **blind** — two turns | `run.py seed`, one turn per line |
| `c-prompt.txt` | arm (c), **blind** — one prompt | `run-session.sh --headless --prompt-file` |
| `env-probe.txt` | apparatus, **not blind, not scored** — is `CLAUDECODE` set for the agent and a sub-agent | `run-session.sh --headless`, on the walk corpus |
| `seed-smoke-turns.txt` | apparatus, **not blind, not scored** — does turn 2 resume turn 1 | `run.py seed`, on the walk corpus |
| `debrief-prompt.txt` | the debrief, **only if it is taken**, and only after an arm is scored | `run.py fork` |

**The shape `run.py seed` takes:** it reads the file's non-blank lines, and each line is one turn.
A line break inside a turn makes two turns.

**A seed's first line carries no `"` and no `\`.** `seed` proves the frozen conversation is the
right one by finding turn 1's text in the transcript, where both characters are JSON-escaped. The
check runs after the whole conversation has, so a stray quote costs the arm.

**Screened 2026-10-03, all five blind turns clean on each count:**

- `driver/interact.py`'s `FORBIDDEN` list, by `interact.contaminates()` — plain substring,
  case-folded, deliberately over-refusing;
- the blindness list — *fan out*, *amend*, *inconsisten-*, *disagree*, *contradict*, *worktree*,
  *provision*, *sub-task*, *spawn*, *parallel*, any `--flag`, any `jigc <verb>`;
- no `"` and no `\` in a seed's first turn;
- no `rc`, `trial`, `probe` or `gate` token.

`debrief-prompt.txt` fails the first screen on purpose: it contains *read*. It is a debrief, never
a reply into a measured turn. `env-probe.txt` names an environment variable and prints `SET` or
`UNSET`, never a value.

Words left in on purpose, and argued in `../open-forks.md` → F2: *milestone*, *finalize*,
*architecture document*, and the one channel statement naming `jigc`.
