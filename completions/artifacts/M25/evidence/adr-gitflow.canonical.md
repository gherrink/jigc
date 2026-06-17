---
status: superseded
date: 2024-09-26
---

# transition-to-simplified-git-flow

## Context

The existing GitHub Flow lacked clarity for stable releases and was overly manual, making it hard to delegate and streamline contributions. A more structured yet simple workflow was needed — automating releases, letting work merge quickly without breaking the stable branch while beta testers try new features, easing delegation to new maintainers, and clearly separating ongoing development from stable releases.

## Decision

Adopt Simplified Git Flow: a `develop` branch for ongoing work and a `stable` branch for releases, with automated canary/beta releases from `develop`. It adds structure without the full complexity of release-branch Git Flow.

## Consequences

Positive: clear separation of development and stable branches, automated beta releases for early testing, easier delegation, and streamlined versioning via Conventional Commits. Negative: contributors must open PRs against `develop` instead of `stable`, and there is a small learning curve for Conventional Commits.
