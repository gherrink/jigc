# Sub-agent brief — sub-task `report-the-sweep-again` (`report-jigc-feedback`)

You are one of three sub-agents a milestone fanned out. The orchestrator spawned
you; two siblings are filing their own findings at the same time. Your job is to
file exactly ONE finding through jigc, with the exact content below, and stop.

## Your environment — every Bash call, no exceptions

Your shell does not keep state between Bash calls, so EVERY command you run must
start with this prefix (it points `HOME` at the rig's home and puts the jigc
under test first on `PATH`):

~~~sh
. '<RIG>/agent-env.sh' && cd '<RIG>/repo/.jigc/worktrees/report-the-sweep-again' && <command>
~~~

Your working directory is your sub-task's worktree: `<RIG>/repo/.jigc/worktrees/report-the-sweep-again`

First call — check the binary, then run your `Spawn:` line VERBATIM (the part
after the prefix is exactly what `jigc milestone execute` printed):

~~~sh
. '<RIG>/agent-env.sh' && command -v jigc && jigc --version
. '<RIG>/agent-env.sh' && cd <RIG>/repo/.jigc/worktrees/report-the-sweep-again && jigc workflow report-jigc-feedback --task report-the-sweep-again
~~~

`command -v jigc` must print `<RIG>/shim-bin/jigc` and the version must be
`jigc 1.0.0-rc.22`. If either differs, STOP and report — do not continue.

The `Spawn:` line prints this sub-task's composed workflow. Read it; it is your
instruction set, and the commands below are its emitted lines with every
placeholder filled. The CONTENT values — titles, field values, slot text — come
from this brief and nowhere else, even where the composed text suggests a source
for one (it says `jigc --version` prints the `jigc-version`; this fixture
files a fixed value instead). A disagreement about a COMMAND — a verb, flag or
address form the composed text emits differently from what is below — is what
you STOP and report, verbatim. Do not improvise.

## The blackboard rule — what you may and may not touch

- You reach state ONLY through `jigc` calls (plus the `Spawn:` line above).
- Never read, list, grep or cat any file outside your own worktree — not a
  sibling's worktree under `.jigc/worktrees/`, not `.jigc/tasks/`, not the main
  checkout at `<RIG>/repo`, not `<RIG>`. Inside your worktree you have no reason to
  read files either: `jigc doc schema`, `jigc doc list … --task report-the-sweep-again` and
  `jigc doc show … --task report-the-sweep-again` are how you look.
- Never edit, create or delete a file directly (no Write/Edit tool, no `>`
  redirection into a file). Every write goes through a `jigc doc` verb.
- Run no `git` command at all — this sub-task has no code to stage.
- Do NOT finalize: never run `jigc task finalize`, `jigc milestone join` or
  `jigc milestone finalize`. Your composed text's `task scope:` line says why:
  `jigc milestone finalize file-the-findings-together` is the only commit boundary, and the
  orchestrator runs it after every sibling returns.
- Do not use `jigc doc author` (the batch alternative) — use the per-leaf verbs
  below, one call each, in the order given.
- Set nothing the brief does not list: no `status`, no `date` (both CLI-set),
  no `resolution`, no `pinned-by`, no `duplicate-of`, no `--slug` on the create.

## Exact content — byte-for-byte

Text is case-, punctuation- and whitespace-exact. Each slot payload goes in as a
heredoc attached directly to the `jigc` command (`<<'EOF'` … `EOF`), never
through a pipe, holding exactly the lines shown — no extra blank line, no
leading or trailing spaces, no added punctuation.

### 1. Create the finding

~~~sh
jigc doc create jigc-feedback --title "Finalize sweeps a staged path!" --task report-the-sweep-again
~~~

It acks the new doc's address, `jigc-feedback:<slug>`. Expect
`jigc-feedback:finalize-sweeps-a-staged-path`; use whatever slug the ack
actually prints as `<slug>` below (and report it if it differs).

The sibling sub-task `report-the-staged-sweep` files a finding whose title differs from
yours only by your trailing `!`, so this create mints the SAME slug as theirs,
in your own isolated area — it is not refused, because nothing has landed yet.
That collision is the point of this fixture: the join gives the bare slug to
the lower task id and `-2` to the higher. Keep the `!`, do not pass `--slug`,
and do not look at the sibling.

### 2. Set the four fields, in this order

~~~sh
jigc doc set-field jigc-feedback:<slug>#meta/kind --value bug --task report-the-sweep-again
jigc doc set-field jigc-feedback:<slug>#meta/found-in --value "milestone:findings-channel" --task report-the-sweep-again
jigc doc set-field jigc-feedback:<slug>#meta/jigc-version --value 1.0.0-rc.23 --task report-the-sweep-again
jigc doc set-field jigc-feedback:<slug>#meta/about --value "jigc task finalize" --task report-the-sweep-again
~~~

(`jigc-version` is deliberately `1.0.0-rc.23`, the value the automated fixture
files — not the `--version` you checked. Use it as given.)

### 3. Author the description — one line

~~~sh
jigc doc set-slot jigc-feedback:<slug>#description --from-file - --task report-the-sweep-again <<'EOF'
The report's finalize committed a path another task had staged.
EOF
~~~

### 4. Author the repro — a fenced block, four lines

~~~sh
jigc doc set-slot jigc-feedback:<slug>#repro --from-file - --task report-the-sweep-again <<'EOF'
```sh
# stage a file, then finalize the report
git add src/foreign.rs
```
EOF
~~~

(These four lines are the repro's TEXT. You are not running `git add`.)

### 5. Read it back

~~~sh
jigc doc show jigc-feedback:<slug> --task report-the-sweep-again
~~~

It must show the title `Finalize sweeps a staged path!`, `kind: bug`, `found-in: milestone:findings-channel`,
`about: jigc task finalize`, `jigc-version: 1.0.0-rc.23`, `status: open`, a `date:`,
and the description and repro exactly as above. Then stop.

## Report back (your final message)

1. Every command you ran (after the prefix), each with its exit code.
2. The address the create acked, and the full output of your final
   `jigc doc show <address> --task report-the-sweep-again`.
3. One explicit sentence: whether you reached state only through `jigc` calls,
   whether you read or listed any path outside your worktree (name it if so),
   and whether you edited any file directly or ran any `git` command.
4. Any refusal, finding, warning or surprise, quoted verbatim. If a command
   exits non-zero, STOP there and report — do not retry with different values,
   do not work around it.
