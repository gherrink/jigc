#!/bin/sh
# Emit the M35 RENAME-STUDY SEED CORPUS into <target>/docs/ as plain, conformant managed
# docs. The SAME bytes seed every arm (build-templates-*.sh): the jigc arm runs `jigc
# ingest` to adopt them (managed/baselined); the static/plain arms use them as-is (plain
# files). Confirmed against the real binary 2026-06-28: conformant `adr` / `arch-doc` files
# carrying the M34 `schema-version: 1` stamp under docs/decisions/ + docs/architecture/,
# `jigc setup`+`jigc ingest` adopts them, and `jigc validate` reports **0 findings** on
# this graph (every forward edge resolves AND every doc is schema-version-stamped — the
# clean-seed bar; the un-stamped cross-doc seed now trips field-value-conformant, so the
# stamp is mandatory here).
#
# This is the cross-doc seed re-cut for the rename cost study: same session-subsystem
# decision graph, PLUS one decision built to be the **deliberately non-greppable referrer**
# (sequence.json edit 4). The decision-history graph (8 forward edges; 1 never-touched
# CONTROL):
#
#   supersedes chain:    redis-session-cache    --> in-memory-session-cache
#                        redis-cluster-sessions --> redis-session-cache
#   supersedes (CONTROL, never edited, never cited):
#                        stateless-jwt-sessions --> server-side-session-store
#   arch-doc `session-management` cites:
#                        redis-cluster-sessions, csrf-double-submit-cookie,
#                        ip-rate-limiting, append-only-audit-log, sticky-lb-affinity
#
# The NON-GREPPABLE target = `sticky-lb-affinity` (titled "Sticky load-balancer affinity"),
# cited by the arch-doc (a structured managed referrer, 2 hops). Its rename prompt
# (prompts/edit-4.txt) is a SEMANTIC TICKET that names the decision only by behaviour and
# NEVER prints the slug words "sticky", "lb", or "affinity" — so a plain/static agent that
# never derived the old slug has nothing to grep, while `jigc rename` resolves the slug and
# walks the edge index to repoint the arch-doc cite. (A transitive supersedes chain is
# greppable — the old slug still appears textually — and is NOT the construction;
# ideas/cli-owned-rename.md -> Acceptance.) check-seed.sh enforces the slug never leaks
# into the prompt.
set -eu
T="${1:?usage: build-seed.sh <target-repo-dir>}"
D="$T/docs/decisions"
A="$T/docs/architecture"
mkdir -p "$D" "$A"

adr() { # adr <slug> <title> [supersedes-value]
  slug="$1"; title="$2"; sup="${3:-}"
  {
    echo '---'
    echo 'schema-version: 1'
    echo 'status: accepted'
    echo 'date: 2026-06-28'
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

# --- supersedes chain (renamed across the sequence) ---
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
# --- the deliberately NON-GREPPABLE target (cited by the arch-doc; renamed at edit 4 via a
#     semantic ticket that withholds this slug) ---
adr sticky-lb-affinity        "Sticky load-balancer affinity"

# --- the arch-doc (empty `components` => no code anchors; cross-doc refs only) ---
{
  echo '---'
  echo 'schema-version: 1'
  echo 'cites: [adr:redis-cluster-sessions, adr:csrf-double-submit-cookie, adr:ip-rate-limiting, adr:append-only-audit-log, adr:sticky-lb-affinity]'
  echo '---'
  echo
  echo '# Session management'
  echo
  echo '## Overview'
  echo 'Session management spans the caching strategy, CSRF protection, rate limiting,'
  echo 'request routing, and auditing. The decisions cited above govern how each concern is'
  echo 'currently handled.'
  echo
  echo '## Components'
} > "$A/session-management.md"

echo "seed written to $T/docs (9 adr + 1 arch-doc; 8 forward edges, 1 control; schema-version: 1 stamped)"
