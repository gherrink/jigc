#!/usr/bin/env bash
# Adopt a naive corpus, using the CONTAINER's own jigc binary.
#
# This is manual step 6 of the 1.0.0-gate burden — `corpora.md` describes it in
# prose ("drive `jigc setup` inside the image rather than with the host binary,
# then copy /work back out") and nothing performed it. Run through
# `run-session.sh --exec`, so it rides the identical copy-in / copy-out /
# provenance path a blind session rides, which is that flag's stated purpose.
#
# Why the container's binary and not the host's: the state under test must be
# produced by the exact build the sessions run.
#
# Expected end state (corpora.md): 7 template commits + setup's install commit
# + this adopt commit = 9, clean tree, invocation log on.
set -euo pipefail
cd /work

echo "=== jigc version ==="
jigc --version

echo "=== before ==="
git rev-list --count HEAD

echo "=== jigc setup ==="
jigc setup

echo "=== enable the invocation log ==="
# Pre-enabled on a B2/B3-shape corpus so §3.3's primary channel does not depend
# on the worker running it, and the blind prompt loses a line that is operator
# instruction rather than task.
jigc config set invocation-log true

echo "=== commit the adoption ==="
git add -A
git -c user.name='Corpus Owner' -c user.email='owner@example.invalid' \
    commit -q -m 'chore: adopt jigc for document management' || echo "(nothing to commit)"

echo "=== after ==="
git rev-list --count HEAD
git status --porcelain=v1
echo "=== adapter surfaces present? ==="
for f in CLAUDE.md .jigc/AGENT.md; do
  [ -f "$f" ] && echo "  present: $f" || echo "  MISSING: $f"
done
echo "=== invocation log setting ==="
jigc config get invocation-log || true
echo "ADOPT-OK"
