# Sub-agent brief — sub-task `report-the-eviction-disagreement` (`report-inconsistency`)

You are one of three sub-agents a milestone fanned out. The orchestrator spawned
you; two siblings are filing their own findings at the same time. Your job is to
file exactly ONE finding through jigc, with the exact content below, and stop.

## Your environment — every Bash call, no exceptions

Your shell does not keep state between Bash calls, so EVERY command you run must
start with this prefix (it points `HOME` at the rig's home and puts the jigc
under test first on `PATH`):

~~~sh
. '<RIG>/agent-env.sh' && cd '<RIG>/repo/.jigc/worktrees/report-the-eviction-disagreement' && <command>
~~~

Your working directory is your sub-task's worktree: `<RIG>/repo/.jigc/worktrees/report-the-eviction-disagreement`

First call — check the binary, then run your `Spawn:` line VERBATIM (the part
after the prefix is exactly what `jigc milestone execute` printed):

~~~sh
. '<RIG>/agent-env.sh' && command -v jigc && jigc --version
. '<RIG>/agent-env.sh' && cd <RIG>/repo/.jigc/worktrees/report-the-eviction-disagreement && jigc workflow report-inconsistency --task report-the-eviction-disagreement
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
  read files either: `jigc doc schema`, `jigc doc list … --task report-the-eviction-disagreement` and
  `jigc doc show … --task report-the-eviction-disagreement` are how you look.
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

### 1. Create the record

~~~sh
jigc doc create inconsistency --title "Cache eviction disagrees" --task report-the-eviction-disagreement
~~~

It acks the new doc's address, `inconsistency:<slug>`. Expect
`inconsistency:cache-eviction-disagrees`; use whatever slug the ack actually
prints as `<slug>` below (and report it if it differs).

### 2. Set the kind

~~~sh
jigc doc set-field inconsistency:<slug>#meta/kind --value code-doc --task report-the-eviction-disagreement
~~~

### 3. Add the three sides, IN THIS ORDER, each followed by what it says

Each side is added with a short `--slug` (as the composed text tells you), then
its `says` slot is authored — one line each.

~~~sh
jigc doc add-item inconsistency:<slug>#sides --title "src/cache.rs" --slug code --task report-the-eviction-disagreement
jigc doc set-slot inconsistency:<slug>#sides/code/says --from-file - --task report-the-eviction-disagreement <<'EOF'
The cache evicts the oldest entry.
EOF
~~~

~~~sh
jigc doc add-item inconsistency:<slug>#sides --title "adr:cache-strategy#decision" --slug adr --task report-the-eviction-disagreement
jigc doc set-slot inconsistency:<slug>#sides/adr/says --from-file - --task report-the-eviction-disagreement <<'EOF'
The cache evicts the least-recently-used entry.
EOF
~~~

~~~sh
jigc doc add-item inconsistency:<slug>#sides --title "docs/cache.md" --slug guide --task report-the-eviction-disagreement
jigc doc set-slot inconsistency:<slug>#sides/guide/says --from-file - --task report-the-eviction-disagreement <<'EOF'
The cache never evicts.
EOF
~~~

(The side titles are a path and an address that name the disagreeing artifacts;
none of them exists in this repo and you must not go looking for them.)

### 4. Author the description — one line

~~~sh
jigc doc set-slot inconsistency:<slug>#description --from-file - --task report-the-eviction-disagreement <<'EOF'
The code, the decision and the guide each state a different eviction rule.
EOF
~~~

### 5. Author the evidence — one line

~~~sh
jigc doc set-slot inconsistency:<slug>#evidence --from-file - --task report-the-eviction-disagreement <<'EOF'
Read all three side by side.
EOF
~~~

### 6. Read it back

~~~sh
jigc doc show inconsistency:<slug> --task report-the-eviction-disagreement
~~~

It must show the title `Cache eviction disagrees`, `kind: code-doc`, `status: open`, a
`date:`, the three sides in the order code · adr · guide with their lines, the
description and the evidence exactly as above. Then stop.

## Report back (your final message)

1. Every command you ran (after the prefix), each with its exit code.
2. The address the create acked, and the full output of your final
   `jigc doc show <address> --task report-the-eviction-disagreement`.
3. One explicit sentence: whether you reached state only through `jigc` calls,
   whether you read or listed any path outside your worktree (name it if so),
   and whether you edited any file directly or ran any `git` command.
4. Any refusal, finding, warning or surprise, quoted verbatim. If a command
   exits non-zero, STOP there and report — do not retry with different values,
   do not work around it.
