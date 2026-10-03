# M55 genuine-spawn — shared fixture (sourced by sim.sh, prepare-genuine.sh, after.sh).
#
# The fixture is flow 58 (D) — crates/cli/tests/flow58_several_reporters.rs —
# `several_reporters_land_through_one_join_by_task_id_in_either_filing_order`:
# milestone "File the findings together", three report sub-tasks (two
# report-jigc-feedback whose titles slug onto ONE id, one report-inconsistency),
# filed with the exact values of findings_workflows.rs `file_jigc_feedback` /
# `file_inconsistency`. Nothing here is a new fixture.
#
# Determinism pins:
#  * the milestone record carries `base: <sha>`, so every commit made while the
#    rig is prepared runs under a fixed GIT_AUTHOR_DATE/GIT_COMMITTER_DATE — the
#    base sha is then identical in every rig built from this file. The sub-agents
#    commit nothing; the join commit's date moves the commit, never the tree.
#  * the docs' `date:` field is `set: on-create` from the CLI clock (UTC day,
#    crates/cli/src/doc.rs today_iso, no override). It is NOT pinnable — the
#    golden and the genuine run must file on the same UTC day; after.sh re-derives
#    the golden at comparison time for exactly that reason.

SPAWN_DIR=<scratchpad>/spawn
# A COPY of target/debug/jigc as `cargo build` linked it at HEAD 72f54033, pinned here
# because a `cargo test` in the repo relinks target/debug/jigc to a different binary
# (dev-dependency feature unification) mid-run. Both builds land the same tree.
JIGC_BIN=$SPAWN_DIR/bin/jigc
JIGC_SRC=<repo>

PIN_DATE='2026-10-01T12:00:00+0000'

MILESTONE_TITLE='File the findings together'
MILESTONE='file-the-findings-together'

# (sub-task id, workflow, intent, title) — flow58 REPORTERS; ids are the ones
# `milestone add-task` mints from the intents (checked at prep time).
SUB_FB1=report-the-staged-sweep
SUB_FB2=report-the-sweep-again
SUB_INC=report-the-eviction-disagreement
INTENT_FB1='Report the staged sweep'
INTENT_FB2='Report the sweep again'
INTENT_INC='Report the eviction disagreement'
TITLE_FB1='Finalize sweeps a staged path'
TITLE_FB2='Finalize sweeps a staged path!'
TITLE_INC='Cache eviction disagrees'

# jigc-feedback values (findings_workflows.rs file_jigc_feedback)
FB_KIND=bug
FB_FOUND_IN='milestone:findings-channel'
FB_VERSION='1.0.0-rc.23'
FB_ABOUT='jigc task finalize'
FB_DESCRIPTION="The report's finalize committed a path another task had staged."

# inconsistency values (findings_workflows.rs file_inconsistency / SIDES)
INC_KIND=code-doc
INC_DESCRIPTION='The code, the decision and the guide each state a different eviction rule.'
INC_EVIDENCE='Read all three side by side.'

# Build a fresh rig (dev/jigc-rig fresh) and drive it to just after
# `milestone execute`. Sets RIG REPO RIG_HOME SHIM_BIN EXECUTED (the execute
# output). Every commit in here runs under the pinned date.
prep_rig() {
    local rig out
    export GIT_AUTHOR_DATE="$PIN_DATE" GIT_COMMITTER_DATE="$PIN_DATE"
    rig=$(cd "$JIGC_SRC" && dev/jigc-rig fresh --binary "$JIGC_BIN" 2>/dev/null) || { echo "prep: rig failed" >&2; return 1; }
    eval "$rig"
    [ -n "${REPO:-}" ] && [ -d "$REPO/.jigc" ] || { echo "prep: REPO not set" >&2; return 1; }
    cd "$REPO" || return 1
    SHIM_BIN=$RIG/shim-bin
    mkdir -p "$SHIM_BIN"
    printf '#!/bin/sh\nexec %s "$@"\n' "$JIGC_BIN" > "$SHIM_BIN/jigc"
    chmod 755 "$SHIM_BIN/jigc"
    "$JIGC_BIN" milestone create "$MILESTONE_TITLE" >/dev/null || return 1
    out=$("$JIGC_BIN" milestone add-task "$MILESTONE" "$INTENT_FB1" --workflow report-jigc-feedback) || return 1
    case $out in *"task:$SUB_FB1 "*) ;; *) echo "prep: unexpected id: $out" >&2; return 1;; esac
    out=$("$JIGC_BIN" milestone add-task "$MILESTONE" "$INTENT_FB2" --workflow report-jigc-feedback) || return 1
    case $out in *"task:$SUB_FB2 "*) ;; *) echo "prep: unexpected id: $out" >&2; return 1;; esac
    out=$("$JIGC_BIN" milestone add-task "$MILESTONE" "$INTENT_INC" --workflow report-inconsistency) || return 1
    case $out in *"task:$SUB_INC "*) ;; *) echo "prep: unexpected id: $out" >&2; return 1;; esac
    "$JIGC_BIN" milestone provision "$MILESTONE" >/dev/null || return 1
    EXECUTED=$("$JIGC_BIN" milestone execute "$MILESTONE") || return 1
    unset GIT_AUTHOR_DATE GIT_COMMITTER_DATE
    # One env file per rig: what every sub-agent Bash call sources first.
    cat > "$RIG/agent-env.sh" <<ENV
export HOME='$RIG_HOME'
export PATH='$SHIM_BIN':\$PATH
unset JIGC_PACK_DIR
ENV
}

# The verbatim `Spawn:` span for one sub-task, read off the execute output.
spawn_span() {
    printf '%s\n' "$EXECUTED" | sed -n "s/^Spawn: \`\\(.*--task $1\\)\`\$/\\1/p"
}

# Run a sub-task's Spawn span through `sh -c` with the shim first on PATH (as
# sub_task_composition::run_span does), then file its doc from its worktree.
# $2 = slot-payload form: "exact" (no trailing newline — what the Rust test
# feeds) or "heredoc" (trailing newline — what an agent following the composed
# text's heredoc instruction sends).
sim_sub() {
    local sub=$1 form=$2 span wt
    span=$(spawn_span "$sub")
    [ -n "$span" ] || { echo "sim: no Spawn line for $sub" >&2; return 1; }
    ( export HOME="$RIG_HOME" PATH="$SHIM_BIN:$PATH"; unset JIGC_PACK_DIR; sh -c "$span" ) > "$RIG/composed-$sub.txt" \
        || { echo "sim: span failed for $sub" >&2; return 1; }
    wt="$REPO/.jigc/worktrees/$sub"
    ( cd "$wt" && export HOME="$RIG_HOME" PATH="$SHIM_BIN:$PATH" && unset JIGC_PACK_DIR &&
      case $sub in
        "$SUB_FB1") file_feedback "$sub" "$TITLE_FB1" "$form" ;;
        "$SUB_FB2") file_feedback "$sub" "$TITLE_FB2" "$form" ;;
        "$SUB_INC") file_inconsistency "$sub" "$form" ;;
      esac )
}

payload() { if [ "$1" = heredoc ]; then printf '%s\n' "$2"; else printf '%s' "$2"; fi; }

file_feedback() {
    local sub=$1 title=$2 form=$3 addr slug
    addr=$(jigc doc create jigc-feedback --title "$title" --task "$sub") || return 1
    addr=$(printf '%s' "$addr" | tr -d '[:space:]')
    slug=${addr#jigc-feedback:}
    jigc doc set-field "jigc-feedback:$slug#meta/kind" --value "$FB_KIND" --task "$sub" >/dev/null || return 1
    jigc doc set-field "jigc-feedback:$slug#meta/found-in" --value "$FB_FOUND_IN" --task "$sub" >/dev/null || return 1
    jigc doc set-field "jigc-feedback:$slug#meta/jigc-version" --value "$FB_VERSION" --task "$sub" >/dev/null || return 1
    jigc doc set-field "jigc-feedback:$slug#meta/about" --value "$FB_ABOUT" --task "$sub" >/dev/null || return 1
    payload "$3" "$FB_DESCRIPTION" | jigc doc set-slot "jigc-feedback:$slug#description" --from-file - --task "$sub" >/dev/null || return 1
    # REPRO ends in a newline in the test constant itself, so both forms send it so.
    printf '```sh\n# stage a file, then finalize the report\ngit add src/foreign.rs\n```\n' \
        | jigc doc set-slot "jigc-feedback:$slug#repro" --from-file - --task "$sub" >/dev/null || return 1
    jigc doc show "jigc-feedback:$slug" --task "$sub" >/dev/null || return 1
}

file_inconsistency() {
    local sub=$1 form=$2 addr slug
    addr=$(jigc doc create inconsistency --title "$TITLE_INC" --task "$sub") || return 1
    addr=$(printf '%s' "$addr" | tr -d '[:space:]')
    slug=${addr#inconsistency:}
    jigc doc set-field "inconsistency:$slug#meta/kind" --value "$INC_KIND" --task "$sub" >/dev/null || return 1
    jigc doc add-item "inconsistency:$slug#sides" --title 'src/cache.rs' --slug code --task "$sub" >/dev/null || return 1
    payload "$form" 'The cache evicts the oldest entry.' | jigc doc set-slot "inconsistency:$slug#sides/code/says" --from-file - --task "$sub" >/dev/null || return 1
    jigc doc add-item "inconsistency:$slug#sides" --title 'adr:cache-strategy#decision' --slug adr --task "$sub" >/dev/null || return 1
    payload "$form" 'The cache evicts the least-recently-used entry.' | jigc doc set-slot "inconsistency:$slug#sides/adr/says" --from-file - --task "$sub" >/dev/null || return 1
    jigc doc add-item "inconsistency:$slug#sides" --title 'docs/cache.md' --slug guide --task "$sub" >/dev/null || return 1
    payload "$form" 'The cache never evicts.' | jigc doc set-slot "inconsistency:$slug#sides/guide/says" --from-file - --task "$sub" >/dev/null || return 1
    payload "$form" "$INC_DESCRIPTION" | jigc doc set-slot "inconsistency:$slug#description" --from-file - --task "$sub" >/dev/null || return 1
    payload "$form" "$INC_EVIDENCE" | jigc doc set-slot "inconsistency:$slug#evidence" --from-file - --task "$sub" >/dev/null || return 1
}

# The join + the one commit. Prints nothing; leaves JOIN_OUT / FINALIZE_OUT.
join_and_finalize() {
    JOIN_OUT=$(cd "$REPO" && HOME="$RIG_HOME" "$JIGC_BIN" milestone join "$MILESTONE" 2>&1) || { printf '%s\n' "$JOIN_OUT" >&2; return 1; }
    FINALIZE_OUT=$(cd "$REPO" && HOME="$RIG_HOME" "$JIGC_BIN" milestone finalize "$MILESTONE" 2>&1) || { printf '%s\n' "$FINALIZE_OUT" >&2; return 1; }
}
