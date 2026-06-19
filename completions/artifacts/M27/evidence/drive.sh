#!/usr/bin/env bash
# M27 inc-4 T4 — the F7 recorded-alongside measure: run the FOUR measured facts of
# worked-examples flow 29 on the ACTUALLY-PINNED, `cargo install`-built binary (NOT
# cargo test). The installed `jigc` is taken from PATH with the EMBEDDED dev pack
# (NO JIGC_PACK_DIR) and the `doc-code` probe resolved as the sibling beside the
# installed binary (NO JIGC_DOC_CODE_PROBE) — the production probe-resolution path a
# real install hits, exercising the M27 multi-grammar set.
#
# Mirrors flow 29 (crates/cli/tests/flow29_acceptance.rs) on a polyglot repo:
#   component A → a real TypeScript symbol (src/api.ts#RateRouter, a class_declaration)
#   component B → a real Python symbol    (services/limiter.py#TokenLimiter, a class_definition)
# Four arms over the SAME authored arch-doc fixture (the per-anchor pass<->block flip
# is the masking guard — a resolving doc-code check emits no finding):
#   BLOCK-TS   delete A's TS symbol, B's Python valid  -> block names A's addr, never B's
#   BLOCK-PY   delete B's Python symbol, A's TS valid  -> block names B's addr, never A's
#   PASS       both present                            -> one docs(arch-doc): commit, promoted
#   DISAMBIG   == BLOCK-TS (per-item disambiguation across two grammars; A blocks, B silent)
set -u

EVID="$(cd "$(dirname "$0")" && pwd)"

# Guard the production path: NO pack/probe overrides reach the installed binary.
unset JIGC_PACK_DIR
unset JIGC_DOC_CODE_PROBE

JIGC=$(command -v jigc)
echo "### binary under test"
echo "which jigc       : $JIGC"
echo "which doc-code   : $(command -v doc-code)"
echo "jigc sha256      : $(sha256sum "$JIGC" | cut -d' ' -f1)"
echo "doc-code sha256  : $(sha256sum "$(command -v doc-code)" | cut -d' ' -f1)"
echo

git_q() { git -C "$1" "${@:2}"; }

# Stand up a fresh polyglot scratch repo with the flow-29 fixture, set up + the cited adr
# committed. Echoes the repo path. $1 = HOME for the run, $2 = a-source, $3 = b-source.
make_repo() {
  local home="$1" a_src="$2" b_src="$3"
  local repo
  repo=$(mktemp -d /tmp/jigc-m27-flow29-XXXXXX)
  git -C "$repo" init -q
  git -C "$repo" config user.email t@e.com
  git -C "$repo" config user.name T
  mkdir -p "$repo/src" "$repo/services"
  printf '%s' "$a_src" > "$repo/src/api.ts"
  printf '%s' "$b_src" > "$repo/services/limiter.py"
  git -C "$repo" add -A
  git -C "$repo" commit -q -m initial

  ( cd "$repo" && HOME="$home" jigc setup >/dev/null 2>&1 )

  # The committed adr the arch-doc cites (committed-first ordering).
  ( cd "$repo" && HOME="$home" jigc start --workflow single-task "decide the cache strategy" >/dev/null 2>&1
    HOME="$home" jigc doc create adr --title "Use a cache" >/dev/null 2>&1
    HOME="$home" jigc doc set-slot adr:use-a-cache#context     --from-file - <<<"Lookups must stay fast." >/dev/null 2>&1
    HOME="$home" jigc doc set-slot adr:use-a-cache#decision    --from-file - <<<"Cache the index."        >/dev/null 2>&1
    HOME="$home" jigc doc set-slot adr:use-a-cache#consequences --from-file - <<<"A cold node re-warms."   >/dev/null 2>&1
    HOME="$home" jigc doc set-field commit:decide-the-cache-strategy#type  --value docs   >/dev/null 2>&1
    HOME="$home" jigc doc set-field commit:decide-the-cache-strategy#scope --value adr    >/dev/null 2>&1
    HOME="$home" jigc doc set-slot  commit:decide-the-cache-strategy#summary --from-file - <<<"decide the cache strategy" >/dev/null 2>&1
    HOME="$home" jigc doc set-slot  commit:decide-the-cache-strategy#body    --from-file - <<<"Pick the cache strategy."   >/dev/null 2>&1
    HOME="$home" jigc task finalize decide-the-cache-strategy >/dev/null 2>&1 )

  echo "$repo"
}

# Author the two-component arch-doc (A->TS symbol, B->Python symbol) in $1=repo, $2=HOME.
author_arch_doc() {
  local repo="$1" home="$2"
  ( cd "$repo"
    HOME="$home" jigc start --workflow architecture-documentation "document the gateway" >/dev/null 2>&1
    HOME="$home" jigc doc create arch-doc --title "Gateway" >/dev/null 2>&1
    HOME="$home" jigc doc set-slot  arch-doc:gateway#overview --from-file - <<<"The gateway routes requests and limits per-client volume." >/dev/null 2>&1
    HOME="$home" jigc doc set-field arch-doc:gateway#cites --value adr:use-a-cache >/dev/null 2>&1
    HOME="$home" jigc doc add-item  arch-doc:gateway#components --title "Edge router" >/dev/null 2>&1
    HOME="$home" jigc doc set-slot  arch-doc:gateway#components/edge-router/description --from-file - <<<"Routes requests at the edge." >/dev/null 2>&1
    HOME="$home" jigc doc set-field arch-doc:gateway#components/edge-router/implemented-by --value "src/api.ts#RateRouter" >/dev/null 2>&1
    HOME="$home" jigc doc add-item  arch-doc:gateway#components --title "Token limiter" >/dev/null 2>&1
    HOME="$home" jigc doc set-slot  arch-doc:gateway#components/token-limiter/description --from-file - <<<"Limits per-client token volume." >/dev/null 2>&1
    HOME="$home" jigc doc set-field arch-doc:gateway#components/token-limiter/implemented-by --value "services/limiter.py#TokenLimiter" >/dev/null 2>&1
    HOME="$home" jigc doc set-field commit:document-the-gateway#type  --value docs     >/dev/null 2>&1
    HOME="$home" jigc doc set-field commit:document-the-gateway#scope --value arch-doc >/dev/null 2>&1
    HOME="$home" jigc doc set-slot  commit:document-the-gateway#summary --from-file - <<<"document the gateway" >/dev/null 2>&1
    HOME="$home" jigc doc set-slot  commit:document-the-gateway#body    --from-file - <<<"Living architecture doc for the gateway." >/dev/null 2>&1 )
}

# --- TS symbol present / renamed-away ----------------------------------------
A_PRESENT=$'export class RateRouter {\n  route(): number {\n    return 0;\n  }\n}\n'
A_DELETED=$'export class RouterRenamed {\n  route(): number {\n    return 0;\n  }\n}\n'
# --- Python symbol present / renamed-away ------------------------------------
B_PRESENT=$'class TokenLimiter:\n    def allow(self) -> bool:\n        return True\n'
B_DELETED=$'class LimiterRenamed:\n    def allow(self) -> bool:\n        return True\n'

run_arm() {
  local tag="$1" a_src="$2" b_src="$3"
  local log="$EVID/$tag.log"
  local home repo
  home=$(mktemp -d /tmp/jigc-m27-home-XXXXXX)
  repo=$(make_repo "$home" "$a_src" "$b_src")
  author_arch_doc "$repo" "$home"
  local before
  before=$(git -C "$repo" rev-list --count HEAD)
  {
    echo "=================================================================="
    echo "### ARM: $tag"
    echo "repo: $repo   HEAD-before: $before"
    echo "src/api.ts head:        $(head -1 "$repo/src/api.ts")"
    echo "services/limiter.py head: $(head -1 "$repo/services/limiter.py")"
    echo
    echo "--- jigc --format json task finalize document-the-gateway ---"
  } >> "$log"
  ( cd "$repo" && HOME="$home" jigc --format json task finalize document-the-gateway ) >> "$log" 2>&1
  local exit=$?
  local after
  after=$(git -C "$repo" rev-list --count HEAD)
  {
    echo
    echo "FINALIZE_EXIT=$exit"
    echo "HEAD-after: $after   (delta $((after - before)))"
    if [ -f "$repo/docs/architecture/gateway.md" ]; then
      echo "PROMOTED: docs/architecture/gateway.md PRESENT"
      echo "--- git log -1 --format=%s ---"
      git -C "$repo" log -1 --format=%s
    else
      echo "PROMOTED: docs/architecture/gateway.md ABSENT (nothing promoted)"
    fi
  } >> "$log"
  echo "  $tag: exit=$exit  head-delta=$((after - before))  log=$log"
  rm -rf "$repo" "$home"
}

echo "### running the four arms (installed binary, no overrides)"
run_arm block-ts "$A_DELETED" "$B_PRESENT"   # BLOCK-TS  + DISAMBIG (A blocks, B silent)
run_arm block-py "$A_PRESENT" "$B_DELETED"   # BLOCK-PY  (B blocks, A silent)
run_arm pass     "$A_PRESENT" "$B_PRESENT"   # PASS      (one commit, promoted)
echo "### done"
