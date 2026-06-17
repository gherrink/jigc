---
status: accepted
date: 2024-12-17
supersedes: [adr:transition-to-simplified-git-flow]
---

# switch-back-to-github-flow

## Context

The Simplified Git Flow (a `develop` plus `stable` branch) aimed to separate development from releases and enable automated beta releases, but Lerna — which manages the monorepo — does not integrate well with a two-branch Git Flow: it cannot publish a stable release from a branch other than the main one.

## Decision

Switch back to GitHub Flow while keeping the automated beta releases, rather than patching Lerna. Pull requests still merge quickly without cutting a stable release, the automated beta release is preserved, and the stable release becomes a manual script run by maintainers — which makes Lerna work again and meets all the requirements.

## Consequences

Lerna publishes stable releases again from the main branch; the benefits of the prior workflow (fast merges, beta testing, easier delegation, clear dev/stable separation) are retained, at the cost of a manual maintainer-run stable-release step.
