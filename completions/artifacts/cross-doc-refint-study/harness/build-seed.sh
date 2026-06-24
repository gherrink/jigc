#!/bin/sh
# Emit the cross-doc study SEED CORPUS into <target>/docs/ as plain, conformant managed
# docs. The SAME bytes seed every arm: arm A then runs `jigc ingest` to adopt them
# (managed); the static/plain arms use them as-is (plain files the rule says to keep
# consistent). Confirmed against the real binary 2026-06-24: conformant `adr` /`arch-doc`
# files under docs/decisions/ + docs/architecture/, `jigc ingest` adopts them, and
# `jigc validate` reports 0 ref-resolves on this graph (all forward edges resolve).
#
# The decision-history graph (7 forward edges; 1 is the never-touched CONTROL):
#
#   supersedes chain A:  redis-session-cache --> in-memory-session-cache
#                        redis-cluster-sessions --> redis-session-cache
#   supersedes (CONTROL, never edited, never cited):
#                        stateless-jwt-sessions --> server-side-session-store
#   arch-doc `session-management` cites:
#                        redis-cluster-sessions, csrf-double-submit-cookie,
#                        ip-rate-limiting, append-only-audit-log
#
# Every edit in sequence.json breaks at least one NON-control edge 1-3 hops from the edit
# site (the supersedes dangles are 1 hop; the arch-doc `cites` dangles are 2 hops — the
# far case a static rule is least likely to make a cold agent catch). The control edge
# must resolve at every edit on every arm (a sanity tripwire).
set -eu
T="${1:?usage: build-seed.sh <target-repo-dir>}"
D="$T/docs/decisions"
A="$T/docs/architecture"
mkdir -p "$D" "$A"

adr() { # adr <slug> <title> [supersedes-value]
  slug="$1"; title="$2"; sup="${3:-}"
  {
    echo '---'
    echo 'status: accepted'
    echo 'date: 2026-06-24'
    [ -n "$sup" ] && echo "supersedes: $sup"
    echo '---'
    echo
    echo "# $title"
    echo
    echo '## Context'
    echo "The session subsystem needed a decision on ${title}."
    echo
    echo '## Decision'
    echo "We adopted ${title} for the reasons recorded in the context."
    echo
    echo '## Consequences'
    echo 'The choice carries the usual tradeoffs; revisit if the load profile shifts.'
  } > "$D/$slug.md"
}

# --- supersedes chain A (edited) ---
adr in-memory-session-cache   "In-memory session cache"
adr redis-session-cache       "Redis session cache"        "adr:in-memory-session-cache"
adr redis-cluster-sessions    "Redis cluster sessions"     "adr:redis-session-cache"
# --- control lineage (NEVER edited, NEVER cited) ---
adr server-side-session-store "Server-side session store"
adr stateless-jwt-sessions    "Stateless JWT sessions"     "adr:server-side-session-store"
# --- standalone current decisions (cited by the arch-doc) ---
adr csrf-double-submit-cookie "CSRF double-submit cookie"
adr ip-rate-limiting          "IP rate limiting"
adr append-only-audit-log     "Append-only audit log"

# --- the arch-doc (empty `components` => no code anchors; cross-doc refs only) ---
{
  echo '---'
  echo 'cites: [adr:redis-cluster-sessions, adr:csrf-double-submit-cookie, adr:ip-rate-limiting, adr:append-only-audit-log]'
  echo '---'
  echo
  echo '# Session management'
  echo
  echo '## Overview'
  echo 'Session management spans the caching strategy, CSRF protection, rate limiting, and'
  echo 'auditing. The decisions cited above govern how each concern is currently handled.'
  echo
  echo '## Components'
} > "$A/session-management.md"

echo "seed written to $T/docs (8 adr + 1 arch-doc; 7 forward edges, 1 control)"
