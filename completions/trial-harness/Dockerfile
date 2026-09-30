# The isolation the 1.0.0-gate trial's blind sessions run in.
#
# Not a convenience. Every jigc RC trial to date ran as a plain interactive session on
# the host, which loads `~/.claude/CLAUDE.md` — so the measured conduct was jigc *plus*
# whatever global instructions the operator keeps, and nothing separated the two.
#
# A redirected config dir does NOT fix it. That was tried by this project's own M17
# pilot (completions/artifacts/M17/pilot/runbook.md) and recorded as insufficient: the
# global file loaded anyway from the shared claude env. Only full filesystem isolation
# defeats it.
#
# The CLI is installed rather than mounted from the host: this host is macOS/arm64 and
# a Mach-O binary cannot execute in a Linux container. Same reason jigc is BUILT here
# from a pinned source tree rather than copied from ~/.local/bin.
#
# That build is also the sha pin. `build-image.sh <sha>` feeds a `git archive` of that
# exact commit, so the rc.10 arm cannot accidentally be built from the rc.10-STAMPED
# tree that already contains M48 (protocol.md §5 arm 2). The trap becomes unreachable
# rather than merely documented.

# ---- stage 1: build jigc for linux from the pinned tree, or install a published one --
FROM rust:1.95-slim-trixie AS build

# build-essential because the doc-code probe links tree-sitter grammars (C) — beside jigc
# on an old tree, inside it on a new one; either way a C compiler is needed.
# git because the build embeds nothing from it but cargo may consult it for deps.
RUN apt-get update && apt-get install -y --no-install-recommends \
      build-essential git ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /src
COPY src/ /src/

# Set only by `build-image.sh --registry <version>` (M54 S14), whose build context carries
# an empty src/. Empty selects source mode, below.
ARG JIGC_REGISTRY=
ARG JIGC_SHA=unknown

# Registry mode installs that exact published version from crates.io — a bare version is
# cargo's exact match — and builds nothing of ours: what an adopter can install is what
# the session runs. A published jigc carries the probe inside it (M54 put it there before
# the first publish), so the layout is single-binary. /stage/share/source records where
# the binary came from, in both modes.
#
# Source mode keys on the ARCHIVED TREE, never on a version: M54 moved the doc-code probe
# inside jigc (one binary, run by self-exec as `jigc __probe doc-code --build <v>`), and
# this harness keeps building shas from both sides of that change.
#
#   crates/cli/probes/doc-code/ present -> separate-probe: the probe lives OUTSIDE the
#     workspace by design (root Cargo.toml -> exclude), so it is a second, separate build.
#     Without it `doc-code` probes cannot bind and a whole validation family is silently
#     absent from the run.
#   absent                              -> single-binary: jigc alone carries the probe.
#
# --locked: the lockfile is committed, and a build that silently resolves a different
# dependency set is not the binary the trial claims to be testing.
#
# Every executable built is staged to /stage/bin, the ONE directory the session image
# copies, and /stage/share records what was built: `layout` (read by verify-image.sh
# check 2), `executables` (the list the ldd gate below runs over) and `source` (`sha <sha>`
# or `registry jigc <version>`).
RUN set -e; \
    mkdir -p /stage/bin /stage/share; \
    if [ -n "$JIGC_REGISTRY" ]; then \
      cargo install jigc --version "$JIGC_REGISTRY" --locked --root /stage; \
      echo single-binary > /stage/share/layout; \
      echo "registry jigc $JIGC_REGISTRY" > /stage/share/source; \
    else \
      cargo build --release --locked; \
      cp target/release/jigc /stage/bin/jigc; \
      if [ -d crates/cli/probes/doc-code ]; then \
        (cd crates/cli/probes/doc-code && cargo build --release --locked); \
        cp crates/cli/probes/doc-code/target/release/doc-code /stage/bin/doc-code; \
        echo separate-probe > /stage/share/layout; \
      else \
        echo single-binary > /stage/share/layout; \
      fi; \
      echo "sha $JIGC_SHA" > /stage/share/source; \
    fi; \
    ls /stage/bin > /stage/share/executables

# ---- stage 2: the session image -----------------------------------------------------
# trixie, matching the build stage. A bookworm runtime links glibc 2.36 and the binary
# built on trixie needs 2.39 — it builds clean, ships, and dies at exec with a loader
# error. The pilot's own Dockerfile said `node:20-trixie-slim` for this reason; the
# reason had to be rediscovered because only the tag recorded it.
FROM node:22-trixie-slim

# ripgrep because an agent reaches for it constantly; its absence would change conduct.
# gosu for the uid normalisation below.
RUN apt-get update && apt-get install -y --no-install-recommends \
      git ripgrep ca-certificates gosu \
    && rm -rf /var/lib/apt/lists/*

# Pinned, not `latest`. A trial whose CLI version drifts under it cannot be compared to
# its own other sessions, which is the only comparison these sessions are for.
ARG CLAUDE_VERSION=2.1.233
RUN npm install -g @anthropic-ai/claude-code@${CLAUDE_VERSION}

ARG JIGC_SHA=unknown
ENV JIGC_SHA=${JIGC_SHA}

COPY --from=build /stage/bin/ /usr/local/bin/
COPY --from=build /stage/share/ /usr/local/share/jigc-image/

# Fail the BUILD, not the trial. A binary that cannot resolve its libc still copies in
# cleanly and only dies when a session runs it — where a dead probe reads as a whole
# validation family finding nothing, which this trial would report as a result about
# jigc. Assert every binary the build stage staged actually loads, here, once.
RUN set -e; \
    for n in $(cat /usr/local/share/jigc-image/executables); do \
      b="/usr/local/bin/$n"; \
      if ldd "$b" | grep -q "not found"; then \
        echo "unresolved libraries in $b:"; ldd "$b"; exit 1; \
      fi; \
    done; \
    echo "layout: $(cat /usr/local/share/jigc-image/layout)"; \
    echo "source: $(cat /usr/local/share/jigc-image/source)"; \
    /usr/local/bin/jigc --version

# Interactive mode needs onboarding state that `-p` does not: without it the session
# opens on the authentication/setup screen even though the token is valid. Seed the
# minimum — onboarding done, /work trusted — and nothing else.
#
# This file carries NO instructions, NO memory and NO project history. It is the
# opposite of the thing being isolated against: it exists so the session starts, not
# so it behaves differently. `verify-image.sh` check 3 asserts the distinction by
# looking for instruction files rather than for an empty home.
RUN printf '%s\n' '{"hasCompletedOnboarding":true,"numStartups":5,"projects":{"/work":{"hasTrustDialogAccepted":true,"hasCompletedProjectOnboarding":true,"allowedTools":[],"mcpServers":{},"mcpContextUris":[],"enabledMcpjsonServers":[],"disabledMcpjsonServers":[]}}}' \
      > /home/node/.claude.json \
 && chown node:node /home/node/.claude.json

# A worker commits. Without an identity git refuses, and the session would spend its
# attention on a git config problem this trial is not measuring.
RUN printf '[user]\n\temail = worker@example.com\n\tname = Worker\n[init]\n\tdefaultBranch = main\n[safe]\n\tdirectory = /work\n' \
      > /home/node/.gitconfig \
 && chown node:node /home/node/.gitconfig

# The corpus arrives by `docker cp`, which preserves the HOST uid (501). The container
# runs as node (1000) and would fail to modify any of it — silently producing a session
# that "changed nothing", which this trial would otherwise read as a finding about jigc.
RUN printf '#!/bin/sh\nset -e\nchown -R node:node /work 2>/dev/null || true\nexec gosu node "$@"\n' \
      > /usr/local/bin/session-entry \
 && chmod +x /usr/local/bin/session-entry

WORKDIR /work
ENTRYPOINT ["/usr/local/bin/session-entry"]
CMD ["bash", "-l"]
