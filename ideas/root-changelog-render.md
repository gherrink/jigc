# Root CHANGELOG.md render — the managed changelog's platform-legible sink

**Status: parked 2026-07-02, unscheduled.** From the KB/research comparison (the one genuine KB↔jigc placement contradiction, adjudicated in [DECISIONS.md](../DECISIONS.md) → 2026-07-02 harvest record). Indexed from [VISION.md](../VISION.md) → Open questions.

## The shape

Root `CHANGELOG.md` is the platform-detected idiom (keepachangelog; GitHub, release tooling, humans all look there), and `jigc migrate` currently makes an adopted repo *less* platform-legible (the foreign root file is retired; truth moves to `docs/changelog/changelog.md`). The reconciliation that keeps both records right: the managed doc **stays the source of truth** where jigc keeps it, and root `CHANGELOG.md` becomes a **deterministic CLI-rendered artifact** — a render target like the commit doctype's git-message sink, regenerated at finalize. The KB's own floor says a CHANGELOG is "always generated"; jigc's version of "generated" is *generated structure, curated prose* — the determinism boundary applied to changelogs. No schema change: `location:` stays untouched (the schema-move variant was consciously rejected — a T2 version-bump for a contested preference).

Bump-computation facet: since jigc owns commit type tokens, the render *could* also deterministically compute the SemVer bump from the recorded changes — same sink, same machinery, optional.

## Trigger

Adoption feedback: an adopter (or the RC trials) surfaces root-changelog legibility as a real friction — tooling or humans failing to find the changelog.
