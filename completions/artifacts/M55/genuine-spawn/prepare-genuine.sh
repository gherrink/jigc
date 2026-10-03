#!/usr/bin/env bash
# M55 genuine-spawn — build the rig the REAL sub-agents run in, stopped right after
# `jigc milestone execute`, and write one brief per sub-task. Writes:
#   genuine.env          RIG / REPO / RIG_HOME / SHIM_BIN for after.sh
#   execute-output.txt   the verbatim `milestone execute` output (the Spawn lines)
#   brief-<task>.md      the self-contained brief for each sub-agent
# Re-running builds a NEW rig and overwrites these files (the old rig is left as is).
set -u
. <scratchpad>/spawn/common.sh

prep_rig || exit 1
printf '%s\n' "$EXECUTED" > "$SPAWN_DIR/execute-output.txt"
cat > "$SPAWN_DIR/genuine.env" <<ENV
RIG='$RIG'
REPO='$REPO'
RIG_HOME='$RIG_HOME'
SHIM_BIN='$SHIM_BIN'
ENV

# --- the brief, common frame ----------------------------------------------------
brief_head() { # $1 sub  $2 workflow
    local sub=$1 wf=$2 wt span
    wt="$REPO/.jigc/worktrees/$sub"
    span=$(spawn_span "$sub")
    cat <<BRIEF
# Sub-agent brief — sub-task \`$sub\` (\`$wf\`)

You are one of three sub-agents a milestone fanned out. The orchestrator spawned
you; two siblings are filing their own findings at the same time. Your job is to
file exactly ONE finding through jigc, with the exact content below, and stop.

## Your environment — every Bash call, no exceptions

Your shell does not keep state between Bash calls, so EVERY command you run must
start with this prefix (it points \`HOME\` at the rig's home and puts the jigc
under test first on \`PATH\`):

~~~sh
. '$RIG/agent-env.sh' && cd '$wt' && <command>
~~~

Your working directory is your sub-task's worktree: \`$wt\`

First call — check the binary, then run your \`Spawn:\` line VERBATIM (the part
after the prefix is exactly what \`jigc milestone execute\` printed):

~~~sh
. '$RIG/agent-env.sh' && command -v jigc && jigc --version
. '$RIG/agent-env.sh' && $span
~~~

\`command -v jigc\` must print \`$SHIM_BIN/jigc\` and the version must be
\`jigc 1.0.0-rc.22\`. If either differs, STOP and report — do not continue.

The \`Spawn:\` line prints this sub-task's composed workflow. Read it; it is your
instruction set, and the commands below are its emitted lines with every
placeholder filled. The CONTENT values — titles, field values, slot text — come
from this brief and nowhere else, even where the composed text suggests a source
for one (it says \`jigc --version\` prints the \`jigc-version\`; this fixture
files a fixed value instead). A disagreement about a COMMAND — a verb, flag or
address form the composed text emits differently from what is below — is what
you STOP and report, verbatim. Do not improvise.

## The blackboard rule — what you may and may not touch

- You reach state ONLY through \`jigc\` calls (plus the \`Spawn:\` line above).
- Never read, list, grep or cat any file outside your own worktree — not a
  sibling's worktree under \`.jigc/worktrees/\`, not \`.jigc/tasks/\`, not the main
  checkout at \`$REPO\`, not \`$RIG\`. Inside your worktree you have no reason to
  read files either: \`jigc doc schema\`, \`jigc doc list … --task $sub\` and
  \`jigc doc show … --task $sub\` are how you look.
- Never edit, create or delete a file directly (no Write/Edit tool, no \`>\`
  redirection into a file). Every write goes through a \`jigc doc\` verb.
- Run no \`git\` command at all — this sub-task has no code to stage.
- Do NOT finalize: never run \`jigc task finalize\`, \`jigc milestone join\` or
  \`jigc milestone finalize\`. Your composed text's \`task scope:\` line says why:
  \`jigc milestone finalize $MILESTONE\` is the only commit boundary, and the
  orchestrator runs it after every sibling returns.
- Do not use \`jigc doc author\` (the batch alternative) — use the per-leaf verbs
  below, one call each, in the order given.
- Set nothing the brief does not list: no \`status\`, no \`date\` (both CLI-set),
  no \`resolution\`, no \`pinned-by\`, no \`duplicate-of\`, no \`--slug\` on the create.

## Exact content — byte-for-byte

Text is case-, punctuation- and whitespace-exact. Each slot payload goes in as a
heredoc attached directly to the \`jigc\` command (\`<<'EOF'\` … \`EOF\`), never
through a pipe, holding exactly the lines shown — no extra blank line, no
leading or trailing spaces, no added punctuation.

BRIEF
}

brief_tail() { # $1 sub
    cat <<BRIEF

## Report back (your final message)

1. Every command you ran (after the prefix), each with its exit code.
2. The address the create acked, and the full output of your final
   \`jigc doc show <address> --task $1\`.
3. One explicit sentence: whether you reached state only through \`jigc\` calls,
   whether you read or listed any path outside your worktree (name it if so),
   and whether you edited any file directly or ran any \`git\` command.
4. Any refusal, finding, warning or surprise, quoted verbatim. If a command
   exits non-zero, STOP there and report — do not retry with different values,
   do not work around it.
BRIEF
}

feedback_body() { # $1 sub  $2 title  $3 sibling note
    local sub=$1
    cat <<BRIEF
### 1. Create the finding

~~~sh
jigc doc create jigc-feedback --title "$2" --task $sub
~~~

It acks the new doc's address, \`jigc-feedback:<slug>\`. Expect
\`jigc-feedback:finalize-sweeps-a-staged-path\`; use whatever slug the ack
actually prints as \`<slug>\` below (and report it if it differs).
$3

### 2. Set the four fields, in this order

~~~sh
jigc doc set-field jigc-feedback:<slug>#meta/kind --value $FB_KIND --task $sub
jigc doc set-field jigc-feedback:<slug>#meta/found-in --value "$FB_FOUND_IN" --task $sub
jigc doc set-field jigc-feedback:<slug>#meta/jigc-version --value $FB_VERSION --task $sub
jigc doc set-field jigc-feedback:<slug>#meta/about --value "$FB_ABOUT" --task $sub
~~~

(\`jigc-version\` is deliberately \`$FB_VERSION\`, the value the automated fixture
files — not the \`--version\` you checked. Use it as given.)

### 3. Author the description — one line

~~~sh
jigc doc set-slot jigc-feedback:<slug>#description --from-file - --task $sub <<'EOF'
$FB_DESCRIPTION
EOF
~~~

### 4. Author the repro — a fenced block, four lines

~~~sh
jigc doc set-slot jigc-feedback:<slug>#repro --from-file - --task $sub <<'EOF'
\`\`\`sh
# stage a file, then finalize the report
git add src/foreign.rs
\`\`\`
EOF
~~~

(These four lines are the repro's TEXT. You are not running \`git add\`.)

### 5. Read it back

~~~sh
jigc doc show jigc-feedback:<slug> --task $sub
~~~

It must show the title \`$2\`, \`kind: bug\`, \`found-in: $FB_FOUND_IN\`,
\`about: $FB_ABOUT\`, \`jigc-version: $FB_VERSION\`, \`status: open\`, a \`date:\`,
and the description and repro exactly as above. Then stop.
BRIEF
}

# --- the three briefs ----------------------------------------------------------
note_fb='
The sibling sub-task `'"$SUB_FB2"'` files a finding whose title differs from
yours only by a trailing `!`, so it mints the same slug in its own isolated
area. That is intended: the join gives the bare slug to the lower task id and
`-2` to the higher. Do not pass `--slug`, and do not look at the sibling.'
note_fb2='
The sibling sub-task `'"$SUB_FB1"'` files a finding whose title differs from
yours only by your trailing `!`, so this create mints the SAME slug as theirs,
in your own isolated area — it is not refused, because nothing has landed yet.
That collision is the point of this fixture: the join gives the bare slug to
the lower task id and `-2` to the higher. Keep the `!`, do not pass `--slug`,
and do not look at the sibling.'

{ brief_head "$SUB_FB1" report-jigc-feedback
  feedback_body "$SUB_FB1" "$TITLE_FB1" "$note_fb"
  brief_tail "$SUB_FB1"; } > "$SPAWN_DIR/brief-$SUB_FB1.md"

{ brief_head "$SUB_FB2" report-jigc-feedback
  feedback_body "$SUB_FB2" "$TITLE_FB2" "$note_fb2"
  brief_tail "$SUB_FB2"; } > "$SPAWN_DIR/brief-$SUB_FB2.md"

{ brief_head "$SUB_INC" report-inconsistency
  cat <<BRIEF
### 1. Create the record

~~~sh
jigc doc create inconsistency --title "$TITLE_INC" --task $SUB_INC
~~~

It acks the new doc's address, \`inconsistency:<slug>\`. Expect
\`inconsistency:cache-eviction-disagrees\`; use whatever slug the ack actually
prints as \`<slug>\` below (and report it if it differs).

### 2. Set the kind

~~~sh
jigc doc set-field inconsistency:<slug>#meta/kind --value $INC_KIND --task $SUB_INC
~~~

### 3. Add the three sides, IN THIS ORDER, each followed by what it says

Each side is added with a short \`--slug\` (as the composed text tells you), then
its \`says\` slot is authored — one line each.

~~~sh
jigc doc add-item inconsistency:<slug>#sides --title "src/cache.rs" --slug code --task $SUB_INC
jigc doc set-slot inconsistency:<slug>#sides/code/says --from-file - --task $SUB_INC <<'EOF'
The cache evicts the oldest entry.
EOF
~~~

~~~sh
jigc doc add-item inconsistency:<slug>#sides --title "adr:cache-strategy#decision" --slug adr --task $SUB_INC
jigc doc set-slot inconsistency:<slug>#sides/adr/says --from-file - --task $SUB_INC <<'EOF'
The cache evicts the least-recently-used entry.
EOF
~~~

~~~sh
jigc doc add-item inconsistency:<slug>#sides --title "docs/cache.md" --slug guide --task $SUB_INC
jigc doc set-slot inconsistency:<slug>#sides/guide/says --from-file - --task $SUB_INC <<'EOF'
The cache never evicts.
EOF
~~~

(The side titles are a path and an address that name the disagreeing artifacts;
none of them exists in this repo and you must not go looking for them.)

### 4. Author the description — one line

~~~sh
jigc doc set-slot inconsistency:<slug>#description --from-file - --task $SUB_INC <<'EOF'
$INC_DESCRIPTION
EOF
~~~

### 5. Author the evidence — one line

~~~sh
jigc doc set-slot inconsistency:<slug>#evidence --from-file - --task $SUB_INC <<'EOF'
$INC_EVIDENCE
EOF
~~~

### 6. Read it back

~~~sh
jigc doc show inconsistency:<slug> --task $SUB_INC
~~~

It must show the title \`$TITLE_INC\`, \`kind: $INC_KIND\`, \`status: open\`, a
\`date:\`, the three sides in the order code · adr · guide with their lines, the
description and the evidence exactly as above. Then stop.
BRIEF
  brief_tail "$SUB_INC"; } > "$SPAWN_DIR/brief-$SUB_INC.md"

echo "genuine rig: $RIG" >&2
printf '%s\n' "$EXECUTED" | grep '^Spawn:' >&2

