#!/usr/bin/env bash
# 10-upgrade-continue-on-rc12.sh — protocol.md §5 arm 03, SECOND HALF.
#
# Numbered 10 so it sorts last: it is the only arm that must run AFTER another
# arm, on a DIFFERENT binary, over the corpus that one left behind. See
# `03-upgrade-author-on-rc11.sh` for the full three-command sequence.
#
# WHAT THIS HALF IS FOR: the real 1.0.0 upgrade path — a corpus authored by an
# older binary, continued by the new one. Nothing else in the suite covers it,
# because every fixture builder constructs its states by driving the CURRENT
# binary, so no test has ever seen a corpus an older one wrote.
#
# It compares against MEASUREMENTS, not memory: the first half wrote the
# pre-change behaviour of both declared changes into `.upgrade-baseline`, and this
# half asserts the flip against that file rather than against a remembered claim.
set -uo pipefail
cd /work

say()  { printf '\n=== %s\n' "$1"; }
FAIL=0
step() { printf '\n$ %s\n' "$*"; "$@" 2>&1; printf '[exit %s]\n' "$?"; }
bar()  { if eval "$2" >/dev/null 2>&1; then echo "  OK    $1"; else echo "  FAIL  $1"; FAIL=1; fi; }
newtask() { jigc task list | awk '/^  [a-z]/{print $1; exit}'; }

if [ ! -f /work/.upgrade-baseline ]; then
  cat <<'SKIP'

=== SKIPPED — no baseline from the first half in this corpus

This arm continues the corpus `03-upgrade-author-on-rc11.sh` left behind. Run:

  python3 walk.py <corpus>   <out-a> --tag jigc-gate:rc11 --only 03
  python3 run.py  carry <out-a>/03-upgrade-author-on-rc11 <corpus-b>
  python3 walk.py <corpus-b> <out-b> --tag jigc-gate:rc12 --only 10
SKIP
  echo "ARM 10 SKIPPED (no .upgrade-baseline — the rc.11 half has not run)"
  exit 0
fi

say "0 · we are the NEW binary, over a corpus the OLD one authored"
step jigc --version
step cat /work/.upgrade-baseline
bar "this half really is running rc.12"  "jigc --version | grep -q 'rc.12'"
bar "the corpus really was authored earlier" \
    "test \$(git ls-files docs/decisions/ | wc -l | tr -d ' ') -ge 2"
bar "the tree arrived clean, with no rig evidence carried in" \
    "test -z \"\$(git status --porcelain=v1 | grep -v '^?? .upgrade-baseline')\""
bar "…and specifically no PROVENANCE.txt from the first half" "test ! -e PROVENANCE.txt"

say "1 · the version mismatch is announced, not silent"
step jigc validate
V="$(jigc validate 2>&1)"
bar "the store says it was last written by the older binary" \
    "printf '%s' \"\$V\" | grep -q 'store-version.binary-mismatch'"
bar "…and names both versions" \
    "printf '%s' \"\$V\" | grep -q 'rc.11' && printf '%s' \"\$V\" | grep -q 'rc.12'"

say "2 · BOTH declared changes flip, measured against the first half's baseline"
jigc validate >/dev/null 2>&1; VRC=$?
OLD_VRC="$(sed -n 's/^rc11-validate-exit=//p' /work/.upgrade-baseline)"
bar "§0.1 · the squatter validated 0 on rc.11 …" "test \"$OLD_VRC\" = 0"
bar "§0.1 · … and is non-zero on rc.12"          "test $VRC -ne 0"
bar "§0.1 · with the code that flipped it"       "printf '%s' \"\$V\" | grep -q 'schema-conformance.unadopted-instance'"

jigc start --workflow single-task "add a second rate limiter" >/dev/null 2>&1
TS="$(newtask)"
jigc doc set-field "commit:$TS#header/type"  --value feat   --task "$TS" >/dev/null 2>&1
jigc doc set-field "commit:$TS#header/scope" --value ingest --task "$TS" >/dev/null 2>&1
printf 'add a second rate limiter\n' | jigc doc set-slot "commit:$TS#summary" --from-file - --task "$TS" >/dev/null 2>&1
step jigc task validate "$TS"
TV="$(jigc task validate "$TS" 2>&1)"
OLD_CLEAN="$(sed -n 's/^rc11-task-validate-clean=//p' /work/.upgrade-baseline)"
bar "§0.2 · the same shape said 'no findings' on rc.11 …" "test \"$OLD_CLEAN\" -ge 1"
bar "§0.2 · … and draws the changelog advisory on rc.12" \
    "printf '%s' \"\$TV\" | grep -q 'changelog-recording.gate-granted-unused'"
bar "§0.2 · and it is still exit 0 — an advisory, not a new gate" \
    "jigc task validate '$TS' >/dev/null 2>&1"

say "3 · the corpus still WORKS on the new binary — reads, writes, and a commit boundary"
ADR="$(git ls-files docs/decisions/ | head -1 | sed 's|docs/decisions/||; s|\.md$||')"
step jigc doc show "adr:$ADR"
bar "a doc the OLD binary wrote reads back through the new one" \
    "jigc doc show 'adr:$ADR' | grep -q '^# '"
step jigc doc list
bar "…and appears in the index read as managed" \
    "jigc doc list | grep -q 'adr:$ADR'"

# The task minted above is carried to a real commit: an upgrade that reads but
# cannot land is not an upgrade.
#
# It deliberately does NOT write a changelog entry. The first half planted a
# foreign, non-conformant CHANGELOG.md at the changelog home for the §0.1
# baseline, so a `doc create changelog` here copies those bytes in and the
# boundary refuses them — correctly, and for a reason that has nothing to do with
# the upgrade. That refusal cost a run of this arm; asserting it as an upgrade
# failure would have been a false finding about the wrong subject.
#
# Landing WITHOUT the entry is also the better cell: it shows the §0.2 advisory
# is an advisory — the boundary commits over it.
# `single-task` is a code-changing workflow, so it needs a code change staged —
# `finalize.nothing-staged` is the correct refusal otherwise, and its route says
# exactly this. Following the printed route rather than working around it.
printf '\nexport const RATE_LIMIT = 1000;\n' >> src/config.ts
git add src/config.ts

step jigc task finalize "$TS"
# Assert the COMMIT, not an empty task list: the first half left a milestone
# sub-task open, so `jigc task list` is legitimately non-empty and the emptiness
# predicate fails on a finalize that plainly succeeded.
bar "a task minted on the new binary lands over the old corpus" \
    "git log -1 --format=%s | grep -q 'second rate limiter'"
bar "…and the code change is in that commit" \
    "git show --stat --format= HEAD | grep -q 'src/config.ts'"
bar "…and the §0.2 advisory did NOT gate the boundary" \
    "git log -1 --format=%s | grep -q 'rate limiter'"

say "4 · setup installs the guide artifact onto a corpus that predates it"
# NOT `jigc upgrade` — that re-checks config deltas and is report-only by its own
# help. An executor who drives it here sees no install and records a false finding.
step jigc setup
bar "the guide artifact is installed" "test -f .claude/skills/jigc/SKILL.md"
bar "…and is stamped with this build" "grep -q 'rc.12' .claude/skills/jigc/SKILL.md"

say "5 · migrate-corpus over the older corpus"
step jigc migrate-corpus --dry-run
bar "migrate-corpus reports rather than refusing over the upgraded corpus" \
    "jigc migrate-corpus --dry-run >/dev/null 2>&1"

say "SUMMARY"
echo "  the real 1.0.0 upgrade path: a corpus authored on rc.11, read, written,"
echo "  committed and migrated on rc.12, with both declared changes asserted"
echo "  against the first half's MEASURED baseline rather than against memory."
if [ "$FAIL" -eq 0 ]; then echo "ARM 10 PASS"; else echo "ARM 10 FAIL"; fi
exit "$FAIL"
