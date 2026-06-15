#!/usr/bin/env bash
# M23 inc-4 T2 — migration-quality corpus driver.
# Drives each reproduced foreign CHANGELOG through the re-pinned HEAD `jigc`
# (from PATH) migrate -> author -> review -> approve -> adopt path, capturing
# evidence per file. Framing A: this script IS the agent rewriting the foreign
# prose through the write verbs (it places nothing; the CLI owns placement).
set -u

CORPUS=/home/maurice/Projects/gherrink-jigc/completions/artifacts/M23/corpus
EVID=/home/maurice/Projects/gherrink-jigc/completions/artifacts/M23/evidence
TASK=changelog

git_q() { git -C "$1" "${@:2}" >/dev/null 2>&1; }

# author helpers operate on $REPO (a global set per file)
create_doc() {
  jigc doc create changelog --title Changelog --task "$TASK" >/dev/null 2>&1 \
    || { echo "  create FAILED"; return 1; }
}

# add_release <version> <date> [link] -> echoes release address
add_release() {
  local ver="$1" date="$2" link="${3:-}"
  local addr
  addr=$(jigc doc add-item changelog:changelog#releases --title "$ver" --task "$TASK" 2>/dev/null)
  jigc doc set-field "$addr/date" --value "$date" --task "$TASK" >/dev/null 2>&1
  if [ -n "$link" ]; then
    jigc doc set-field "$addr/link" --value "$link" --task "$TASK" >/dev/null 2>&1
  fi
  echo "$addr"
}

# add_group <release_addr> <Category> <notes...>
add_group() {
  local parent="$1" cat="$2" notes="$3"
  local g
  g=$(jigc doc add-item "$parent/changes" --title "$cat" --task "$TASK" 2>/dev/null)
  printf '%s' "$notes" | jigc doc set-slot "$g/notes" --from-file - --task "$TASK" >/dev/null 2>&1
}

make_commit_conformant() {
  jigc doc set-field "commit:$TASK#type" --value feat --task "$TASK" >/dev/null 2>&1
  jigc doc set-field "commit:$TASK#scope" --value changelog --task "$TASK" >/dev/null 2>&1
  printf 'adopt the migrated changelog\n' | jigc doc set-slot "commit:$TASK#summary" --from-file - --task "$TASK" >/dev/null 2>&1
  printf 'Migrate the foreign CHANGELOG.md into managed shape.\n' | jigc doc set-slot "commit:$TASK#body" --from-file - --task "$TASK" >/dev/null 2>&1
}

# scaffold <slug> <corpus-file> <author-fn>
scaffold() {
  local slug="$1" cf="$2" author="$3"
  REPO=$(mktemp -d "/tmp/m23-corpus/repo-$slug-XXXX")
  local log="$EVID/$slug.log"
  : > "$log"
  cd "$REPO" || return 1

  git init -q
  git config user.email t@example.com
  git config user.name Tester
  printf 'placeholder\n' > README.md
  git add . ; git commit -qm initial
  mkdir -p .jigc/config

  cp "$CORPUS/$cf" "$REPO/CHANGELOG.md"
  git add CHANGELOG.md ; git commit -qm "track foreign changelog"
  local foreign_sha
  foreign_sha=$(git hash-object CHANGELOG.md)

  jigc setup >/dev/null 2>&1

  {
    echo "### FILE: $cf  (slug=$slug)"
    echo "foreign CHANGELOG.md git-blob: $foreign_sha"
    echo
    echo "--- migrate (composed view, head) ---"
  } >> "$log"
  jigc migrate CHANGELOG.md --as changelog >>"$log" 2>&1

  create_doc
  "$author"
  make_commit_conformant

  {
    echo
    echo "--- finalize WITHOUT --approve (review gate) ---"
  } >> "$log"
  jigc task finalize "$TASK" >>"$log" 2>&1
  local block_exit=$?
  echo "BLOCK_EXIT=$block_exit" >> "$log"

  {
    echo
    echo "--- finalize --approve ---"
  } >> "$log"
  jigc task finalize "$TASK" --approve >>"$log" 2>&1
  local approve_exit=$?
  echo "APPROVE_EXIT=$approve_exit" >> "$log"

  {
    echo
    echo "--- canonical changelog/changelog.md on disk ---"
  } >> "$log"
  if [ -f changelog/changelog.md ]; then
    cat changelog/changelog.md >> "$log"
    cp changelog/changelog.md "$EVID/$slug.canonical.md"
  else
    echo "(no canonical doc produced)" >> "$log"
  fi

  {
    echo
    echo "--- foreign original present after approve? ---"
    if [ -f CHANGELOG.md ]; then echo "PRESENT (not retired)"; else echo "GONE (retired)"; fi
    echo
    echo "--- ingest ---"
  } >> "$log"
  jigc ingest >>"$log" 2>&1

  {
    echo
    echo "--- HEAD commit name-status ---"
  } >> "$log"
  git show --name-status --format='%H %s' HEAD >> "$log" 2>&1

  cd /tmp || true
}

# ===== per-file authoring (Framing A: the agent's category mapping) =====

author_keepachangelog() {
  # clean KaC, multi-release. Author oldest-to-newest. All categories map.
  local r
  r=$(add_release "1.1.0" "2019-02-15" "https://github.com/olivierlacan/keep-a-changelog/compare/v1.0.0...v1.1.0")
  add_group "$r" "Added" $'- Danish translation (#297).\n- Georgian translation from (#337).\n- Changelog inconsistency section in Bad Practices.\n'
  add_group "$r" "Fixed" $'- Italian translation (#332).\n- Indonesian translation (#336).\n'

  r=$(add_release "1.1.1" "2023-03-05" "https://github.com/olivierlacan/keep-a-changelog/compare/v1.1.0...v1.1.1")
  add_group "$r" "Added" $'- Arabic translation (#444).\n- v1.1 French translation.\n- v1.1 Dutch translation (#371).\n'
  add_group "$r" "Fixed" $'- Improve French translation (#377).\n- Improve id-ID translation (#416).\n'
  add_group "$r" "Changed" $'- Use frontmatter title & description in each language version template.\n'
  add_group "$r" "Removed" $'- Trademark sign previously shown after the project description in version 0.3.0.\n'
}

author_express() {
  # loosely-structured, no foreign categories -> the agent assigns the enum.
  local r
  r=$(add_release "4.18.0" "2022-04-25")
  add_group "$r" "Added" $'- `res.download` support for object with all optional properties.\n'
  add_group "$r" "Changed" $'- `res.sendFile` to use `send@0.18.0`.\n- deps: body-parser@1.20.0.\n'
  add_group "$r" "Removed" $'- deprecated leading colon in `name` for `app.param(name, fn)`.\n'

  r=$(add_release "4.18.1" "2022-04-29")
  add_group "$r" "Fixed" $'- hanging on large stack of sync routes.\n'

  r=$(add_release "4.18.2" "2022-10-08")
  add_group "$r" "Fixed" $'- regression routing a large stack in a single route.\n- `req.resume()` called after `res.pipe()`.\n'
  add_group "$r" "Changed" $'- deps: body-parser@1.20.1 (deps: qs@6.11.0).\n- deps: qs@6.11.0.\n'
}

author_axios() {
  # non-KaC categories: Bug Fixes->fixed, Features->added,
  # Performance Improvements->changed (no perf enum member; agent remap).
  local r
  r=$(add_release "1.6.1" "2023-11-08" "https://github.com/axios/axios/compare/v1.6.0...v1.6.1")
  add_group "$r" "Fixed" $'- **formdata:** informative error handling for browser-only multipart payloads ([#6053]).\n'
  add_group "$r" "Changed" $'- **trim:** lifted regexp-based trimming for a small startup gain ([#6034]). (foreign category: Performance Improvements)\n'

  r=$(add_release "1.6.2" "2023-11-14" "https://github.com/axios/axios/compare/v1.6.1...v1.6.2")
  add_group "$r" "Fixed" $'- **formdata:** content-type header normalization for non-standard browser environments ([#6056]).\n- **dns:** lookup function decoration to support all signatures ([#6011]).\n'
  add_group "$r" "Added" $'- **withXSRFToken:** option as a workaround to support the old withCredentials-coupled behavior ([#6046]).\n'
}

author_commander() {
  # multi-release stress (5 releases). All categories map (incl. Deprecated).
  local r
  r=$(add_release "10.0.1" "2023-04-15")
  add_group "$r" "Fixed" $'- export the `Help.visibleGlobalOptions()` helper missing from the type definitions ([#1896]).\n'

  r=$(add_release "11.0.0" "2023-06-20")
  add_group "$r" "Changed" $'- **Breaking:** Commander now requires Node.js v16 or higher ([#1929]).\n'
  add_group "$r" "Fixed" $'- subcommand help now respects a configured output width ([#1918]).\n'

  r=$(add_release "11.1.0" "2023-10-13")
  add_group "$r" "Added" $'- allow using `InvalidArgumentError` from custom argument processing ([#1973]).\n'
  add_group "$r" "Fixed" $'- TypeScript: widen the `Option.argChoices` typing to accept readonly arrays ([#1948]).\n'

  r=$(add_release "12.0.0" "2024-02-03")
  add_group "$r" "Added" $'- add `.saveStateBeforeParse()` and `.restoreStateBeforeParse()` for reuse of a configured command ([#2057]).\n'
  add_group "$r" "Changed" $'- **Breaking:** Commander now requires Node.js v18 or higher ([#2027]).\n- **Breaking:** `.parse()` and `.parseAsync()` now accept options as the second parameter ([#2098]).\n'
  add_group "$r" "Deprecated" $'- the trailing-comma form of `.option()` flag lists is deprecated in favour of explicit arrays ([#2104]).\n'
  add_group "$r" "Removed" $'- **Breaking:** removed the long-deprecated `.command(\x27*\x27)` default-command form ([#2061]).\n'

  r=$(add_release "12.1.0" "2024-05-18")
  add_group "$r" "Added" $'- TypeScript: add `startup` to the `AddHelpTextPosition` type used by `.addHelpText()` ([#2197]).\n'
  add_group "$r" "Changed" $'- update package-lock to address `braces` advisory ([#2208]).\n'
}

scaffold kac        01-keep-a-changelog.md         author_keepachangelog
scaffold express    02-express-history.md          author_express
scaffold axios      03-axios.md                    author_axios
scaffold commander  04-commander-multi-release.md  author_commander

echo "ALL DONE"
