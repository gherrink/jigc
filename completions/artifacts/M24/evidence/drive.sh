#!/usr/bin/env bash
# M24 inc-7 T4 — the FULL project-delta + project-gamma live-migration driver (the done-bar).
# Drives each real, dateless foreign CHANGELOG through the re-pinned HEAD `jigc`
# (from PATH, EMBEDDED dev pack — no JIGC_PACK_DIR) over a THROWAWAY clone:
#   setup -> migrate -> ONE `doc author --from` batch -> review-gate block ->
#   --approve -> adopt.
# Framing A: build-payload.py is the agent rewriting the foreign prose into ONE
# declarative payload; this script places nothing — the CLI applies every leaf.
set -u

EVID=/home/maurice/Projects/gherrink-jigc/completions/artifacts/M24/evidence
TASK=migrate-changelog

# drive <slug> <source-repo>
drive() {
  local slug="$1" srcrepo="$2"
  local log="$EVID/$slug.log"
  local payload="$EVID/$slug.payload.yaml"
  local clone="/tmp/jigc-m24-$slug"
  : > "$log"

  rm -rf "$clone"
  git clone -q "$srcrepo" "$clone"
  cd "$clone" || return 1
  git config user.email t@example.com
  git config user.name Tester
  # Detach so we never touch the real repo's branch; clean to just the changelog test.
  git checkout -q --detach HEAD

  local foreign_sha
  foreign_sha=$(git hash-object CHANGELOG.md)

  {
    echo "### REPO: $srcrepo  (slug=$slug)"
    echo "clone: $clone"
    echo "foreign CHANGELOG.md git-blob: $foreign_sha"
    echo "foreign line count: $(wc -l < CHANGELOG.md)"
    echo
    echo "--- jigc setup ---"
  } >> "$log"
  jigc setup >>"$log" 2>&1

  {
    echo
    echo "--- jigc migrate CHANGELOG.md --as changelog (composed view) ---"
  } >> "$log"
  jigc migrate CHANGELOG.md --as changelog >>"$log" 2>&1

  {
    echo
    echo "--- jigc doc author changelog --from <payload> (ONE batch) ---"
  } >> "$log"
  local author_out
  author_out=$(jigc doc author changelog --from - --task "$TASK" < "$payload" 2>>"$log")
  echo "AUTHOR_STDOUT=$author_out" >> "$log"

  {
    echo
    echo "--- jigc task finalize $TASK  (review gate, NO --approve) ---"
  } >> "$log"
  jigc task finalize "$TASK" >>"$log" 2>&1
  local block_exit=$?
  echo "BLOCK_EXIT=$block_exit" >> "$log"

  {
    echo
    echo "--- jigc task finalize $TASK --approve ---"
  } >> "$log"
  jigc task finalize "$TASK" --approve >>"$log" 2>&1
  local approve_exit=$?
  echo "APPROVE_EXIT=$approve_exit" >> "$log"

  {
    echo
    echo "--- foreign original present after approve? ---"
    if [ -f CHANGELOG.md ]; then echo "PRESENT (NOT retired)"; else echo "GONE (retired)"; fi
    echo
    echo "--- canonical changelog/changelog.md (head + tail) ---"
  } >> "$log"
  if [ -f changelog/changelog.md ]; then
    cp changelog/changelog.md "$EVID/$slug.canonical.md"
    {
      head -20 changelog/changelog.md
      echo "    [...]"
      tail -8 changelog/changelog.md
      echo
      echo "canonical line count: $(wc -l < changelog/changelog.md)"
      echo "canonical '### ' release-heading count: $(grep -c '^### ' changelog/changelog.md)"
      echo "canonical '#### ' change-group count: $(grep -c '^#### ' changelog/changelog.md)"
      echo "canonical 'date:' line count: $(grep -c 'date:' changelog/changelog.md)"
    } >> "$log"
  else
    echo "(no canonical doc produced)" >> "$log"
  fi

  {
    echo
    echo "--- jigc ingest (round-trip conformance: adopted?) ---"
  } >> "$log"
  jigc ingest >>"$log" 2>&1

  {
    echo
    echo "--- jigc ingest AGAIN (idempotent — file-state hash stable) ---"
  } >> "$log"
  jigc ingest >>"$log" 2>&1

  {
    echo
    echo "--- HEAD commit subject + name-status (the migration set) ---"
  } >> "$log"
  git show --name-status --format='%H%n%s%n%b' HEAD >> "$log" 2>&1

  cd /tmp || true
  echo "$slug: BLOCK_EXIT=$block_exit APPROVE_EXIT=$approve_exit"
}

drive project-delta /home/maurice/Projects/project-delta
drive project-gamma      /home/maurice/Projects/project-gamma
echo "ALL DONE"
