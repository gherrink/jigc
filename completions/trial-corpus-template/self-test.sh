#!/usr/bin/env bash
# self-test.sh [--keep]
#
# Prove each of check-corpus.sh's bars can FAIL.
#
# Why this exists. check-corpus.sh was written because RC-pre-1.0 stated its
# checklist as prose and nothing ever checked it — "a plant assumed to fire is not
# a plant", applied to the corpus. This applies the same sentence to the gate
# itself: a bar that has silently stopped working reports PASS, and a green
# check-corpus run is then indistinguishable from an unchecked corpus.
#
# The eval rig this idea is harvested from states the general form: "a check that
# has stopped working shows up as a failure rather than as a clean run", and, more
# bluntly, "every apparatus failure found so far was found by running a check
# somewhere it had not been run, never by reading".
#
# How it works: instantiate one pristine corpus, then for each mutation below copy
# it, break exactly one thing, and require BOTH that the gate exits non-zero AND
# that the named bar is the one reporting FAIL. Requiring the *named* bar matters —
# a mutation that trips some other bar would otherwise look like a pass while the
# bar under test is dead.
set -uo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
KEEP=0
[ "${1:-}" = "--keep" ] && KEEP=1

WORK="$(mktemp -d)"
cleanup() { [ "$KEEP" = 1 ] || rm -rf "$WORK"; }
trap cleanup EXIT

PASS=0; FAIL=0
ok()  { echo "  PASS  $1"; PASS=$((PASS+1)); }
bad() { echo "  FAIL  $1"; FAIL=$((FAIL+1)); }

echo "building a pristine corpus"
"$HERE/instantiate.sh" --clean-prose "$WORK/pristine" selftestsvc >/dev/null 2>&1 || {
  echo "could not instantiate a corpus" >&2; exit 2; }

# The gate must pass on the pristine corpus, or every failure below is meaningless.
if "$HERE/check-corpus.sh" "$WORK/pristine" --clean-prose >"$WORK/pristine.out" 2>&1; then
  ok "the gate passes on a pristine corpus (the positive control)"
else
  echo "  FAIL  the gate does not pass on a pristine corpus — nothing below is readable"
  sed 's/^/        /' "$WORK/pristine.out"
  exit 1
fi

# name | expected bar substring | mutation (run inside the copy)
run_case() {
  local name="$1" want="$2" mutation="$3"
  local dir="$WORK/case-$name"
  rm -rf "$dir"; cp -R "$WORK/pristine" "$dir"
  ( cd "$dir" && eval "$mutation" ) >/dev/null 2>&1
  "$HERE/check-corpus.sh" "$dir" --clean-prose >"$dir.out" 2>&1
  local rc=$?
  if [ "$rc" -eq 0 ]; then
    bad "$name: the gate still passed — this bar is dead"
    return
  fi
  if grep -q "FAIL  .*$want" "$dir.out"; then
    ok "$name → '$want' fired"
  else
    bad "$name: the gate failed, but not at '$want' — a different bar caught it"
    grep "FAIL" "$dir.out" | sed 's/^/        /'
  fi
}

echo "mutating one thing at a time"
# The expected string is the bar's FAILURE text, which is not its PASS label — the
# gate says "core.hooksPath unset" when happy and "core.hooksPath is set to '…'"
# when not. Matching the pass label silently matched nothing and reported every bar
# dead, which is this script's own first finding about itself.
#
# A mutation may trip OTHER bars too (an extra commit moves the count and the
# reflog). That is fine and expected: the assertion is that the named bar fired,
# never that it fired alone.
run_case commits        "expected 7 commits"          'git commit -q --allow-empty -m "extra"'
run_case dirty          "working tree dirty"          'echo x >> README.md'
run_case jigc-residue   "corpus is not naive"         'mkdir -p .jigc && touch .jigc/x'
run_case hook-residue   "corpus is not naive"         'printf "#!/bin/sh\nexit 0\n" > "$(git rev-parse --git-path hooks)/pre-commit"'
run_case remote         "corpus has a real remote"    'git remote add origin https://example.invalid/x.git'
run_case hookspath      "core.hooksPath is set"       'git config core.hooksPath .githooks'
run_case reflog         "this corpus has been worked in" 'git commit -q --allow-empty -m "worked"'
run_case branch         "expected main"               'git branch -m main other'
run_case stray-md       "tracked .md files are"       'echo "# x" > NOTES.md && git add -A && git -c user.name=t -c user.email=t@x commit -q -m n'
run_case suite          "node --test: expected"       'f="$(ls test/*.js test/*.ts 2>/dev/null | head -1)"; printf "\nthrow new Error(\"boom\");\n" >> "$f"'
run_case anchor         "missing symbols"             'perl -pi -e "s/class MemoryStore/class RenamedStore/" src/store.ts'
# PT-D's bar, proven able to fail. The mutation is the DEFECT ITSELF as it stood for
# three trials — the router writing straight to the store — so this case is a
# regression test on the corpus's own history, not a synthetic break. Note it leaves
# every one of the 24 unit tests passing except the one wiring test, which is exactly
# why presence-greps and a green suite were both insufficient.
run_case reach          "a plant symbol is unreachable" 'perl -pi -e "s/this\.queue\.push\(/this.store.put(/" src/router.ts'
run_case prose          "forwarding-shaped claim survives" 'printf "\nA cache in front of whatever long-term store you already have.\n" >> README.md'

echo
echo "$PASS passed, $FAIL failed"
[ "$KEEP" = 1 ] && echo "(kept: $WORK)"
[ "$FAIL" -eq 0 ] || exit 1
