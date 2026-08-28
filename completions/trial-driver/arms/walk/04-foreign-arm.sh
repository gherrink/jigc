#!/usr/bin/env bash
# 04-foreign-arm.sh — protocol.md §5 arm 04 (M46 Increment 3).
#
# WHICH SET THIS ARM ITERATES: the **declared behaviour change** of protocol §0.1,
# over the two doors M46 made agree about one file. Not a registry — a pair of
# doors that used to tell two stories about the same bytes:
#
#   `jigc validate`        said "adopt it"  (advisory, route: jigc ingest)
#   `jigc migrate-corpus`  said "author the prose, then re-run" (blocking, exit 1)
#                          over a file jigc never wrote — a route that, followed
#                          exactly, changed nothing.
#
# THIS ARM CARRIES §5's STANDARD FOR §0.1, which §1 row 2 defers to. A declared
# change reads as designed only if all four hold: (a) it names what it objects to;
# (b) it says what the consequence is; (c) it names the route; (d) the route, run
# verbatim, works. Any one missing and it reads as obstruction — a finding.
#
# Phase B is the **positive-control-shaped** half protocol §0.1 says the trial gets
# for free: a session that adopts its foreign docs should watch the exit go to 0.
# It matters because an exit that only ever goes one way is not evidence that the
# gate is measuring anything.
set -uo pipefail
cd /work

say()  { printf '\n=== %s\n' "$1"; }
FAIL=0
step() { printf '\n$ %s\n' "$*"; "$@" 2>&1; printf '[exit %s]\n' "$?"; }
bar()  { if eval "$2" >/dev/null 2>&1; then echo "  OK    $1"; else echo "  FAIL  $1"; FAIL=1; fi; }
commit_as() { git -c user.name='Corpus Owner' -c user.email='owner@example.invalid' commit -q "$@"; }

say "0 · adopt jigc onto a repo that already has documents (the adopter's day one)"
jigc setup >/dev/null 2>&1
jigc config set invocation-log true >/dev/null 2>&1
jigc --version

# A stock brownfield corpus: a real Keep-a-Changelog and a Nygard-shaped decision
# record, both NON-conformant, both sitting at homes jigc manages. Through rc.11
# this validated at exit 0.
printf '# Changelog\n\n## [0.1.0] - 2026-01-04\n### Added\n- first release\n' > CHANGELOG.md
mkdir -p docs/decisions
printf '# Use an in-memory buffer\n\n## Status\nAccepted\n\n## Context\nWe need speed.\n' \
  > docs/decisions/use-an-in-memory-buffer.md
git add -A && commit_as -m 'docs: changelog and a decision record'

# ---------------------------------------------------------------------------
say "A · the store sweep — protocol §0.1's declared change"
step jigc validate

V="$(jigc validate 2>&1)"; VRC=$?
bar "(a) it names the file it objects to" \
    "printf '%s' \"\$V\" | grep -q 'CHANGELOG.md'"
bar "(a) …and the home that makes it jigc's business" \
    "printf '%s' \"\$V\" | grep -q 'sits at the .changelog. home'"
bar "(b) it says what the consequence is — why the sweep itself exits non-zero" \
    "printf '%s' \"\$V\" | grep -q 'exits non-zero rather than report a green'"
bar "(c) it names a route" \
    "printf '%s' \"\$V\" | grep -q 'jigc ingest'"
bar "the exit flipped — this is the declared change" "test \$VRC -ne 0"
bar "the finding itself is untouched: still advisory, not blocking" \
    "printf '%s' \"\$V\" | grep -q 'advisory · schema-conformance.unadopted-instance'"

say "A · the paired door — migrate-corpus stops claiming a file that is not its subject"
step jigc migrate-corpus --dry-run --format json

jigc migrate-corpus --dry-run --format json > /tmp/mc.json 2>&1; MRC=$?
bar "migrate-corpus exits 0 over a brownfield repo (rc.11 exited 1 here)" "test $MRC -eq 0"
bar "the foreign files ride the DECLARED 'unadopted' key" \
    "node -e 'const o=require(\"/tmp/mc.json\");process.exit(Array.isArray(o.unadopted)&&o.unadopted.length===2?0:1)'"
bar "…and are NOT in 'blocked' — emptiness of blocked IS the exit rule" \
    "node -e 'const o=require(\"/tmp/mc.json\");process.exit(o.blocked.length===0?0:1)'"
bar "both doors emit the same code from the one producer" \
    "node -e 'const o=require(\"/tmp/mc.json\");process.exit(o.unadopted.every(f=>f.code===\"schema-conformance.unadopted-instance\")?0:1)'"

say "A · (d) the route, run verbatim, WORKS — it routes rather than dead-ends"
step jigc ingest
I="$(jigc ingest 2>&1)"
bar "(d) ingest routes each non-conformant file onward, naming the verb" \
    "printf '%s' \"\$I\" | grep -q 'jigc migrate CHANGELOG.md --as changelog'"
bar "(d) …and classifies rather than silently skipping" \
    "printf '%s' \"\$I\" | grep -q 'needs-reconcile'"

# ---------------------------------------------------------------------------
say "B · THE POSITIVE CONTROL: the exit must be able to go back to 0"
# An exit that only ever goes one way is not evidence the gate measures anything.
# A conformant foreign doc — right shape, no schema-version stamp, because jigc
# never wrote it — is the case `ingest` adopts register-only.
rm -f CHANGELOG.md docs/decisions/use-an-in-memory-buffer.md
cat > docs/decisions/prefer-a-bounded-buffer.md <<'DOC'
---
status: accepted
date: 2026-02-11
---

# Prefer a bounded buffer

## Context

Unbounded buffering let a slow drain consume all memory.

## Options

Grow without limit, or bound the buffer and shed on overflow.

## Decision

Bound it.

## Consequences

Overflow becomes a policy decision rather than an OOM.
DOC
git add -A && commit_as -m 'docs: a conformant decision record, hand-written'

# The first run of this arm expected `ingest` alone to green the sweep. It does
# not, and the product is right: adopting a STAMP-LESS conformant file makes it a
# MANAGED doc, and a managed doc below the current schema-version draws a
# DIFFERENT exit-flipping member — `schema-conformance.schema-version-current`,
# routed at `jigc migrate-corpus`. So the adopter's path has three steps, not two,
# and the two exit-flip members hand off to each other, each carrying its own
# route. That hand-off is the better demonstration and is now what this phase
# asserts.
jigc validate >/dev/null 2>&1; BEFORE=$?
step jigc ingest
jigc validate >/dev/null 2>&1; MID=$?
step jigc validate
MIDTEXT="$(jigc validate 2>&1)"
step jigc migrate-corpus
jigc validate >/dev/null 2>&1; AFTER=$?
step jigc validate

bar "before adoption the sweep refuses (non-zero)" "test $BEFORE -ne 0"
bar "the conformant foreign doc is ADOPTED register-only, no file moved" \
    "jigc ingest 2>&1 | grep -q 'adopted — indexed + baselined, no file moved'"
bar "adoption hands off to the OTHER exit-flip member, not to green" \
    "test $MID -ne 0"
bar "…and that member is the schema-version one, with its own route" \
    "printf '%s' \"\$MIDTEXT\" | grep -q 'schema-conformance.schema-version-current'"
bar "…routed at migrate-corpus, not back at ingest" \
    "printf '%s' \"\$MIDTEXT\" | grep -q 'run .jigc migrate-corpus. to upgrade it'"
bar "…and it no longer says the file is unadopted — the discriminator moved it" \
    "! printf '%s' \"\$MIDTEXT\" | grep -q 'unadopted-instance'"
bar "AFTER the full adopter path the sweep goes GREEN — the exit is not stuck" \
    "test $AFTER -eq 0"
bar "the file was never moved by any of it" "test -f docs/decisions/prefer-a-bounded-buffer.md"
bar "and it now carries the stamp that made it current" \
    "grep -q '^schema-version: 2$' docs/decisions/prefer-a-bounded-buffer.md"

say "SUMMARY"
echo "  §5 standard for the declared change §0.1: (a) names it · (b) says the consequence"
echo "  · (c) names the route · (d) the route runs and the exit returns to 0"
if [ "$FAIL" -eq 0 ]; then echo "ARM 04 PASS"; else echo "ARM 04 FAIL"; fi
exit "$FAIL"
