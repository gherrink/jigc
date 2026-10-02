# feedback web service — jigc feedback leaves the adopter's repository

**Status: parked 2026-10-02.** Parked by the M55 Settle, S2 and S9 ([DECISIONS.md](../DECISIONS.md) → 2026-10-02 M55 settled; design of record [design/findings-channel.md](../design/findings-channel.md) → 1, 2). Indexed from [VISION.md](../VISION.md) → Open questions.

## The gap

M55 ships the `jigc-feedback` doctype — bugs, inconveniences and feedback **about jigc** — but a report lands in the reporter's own repository, because the CLI makes no network calls. In this repository that is the point (the port files its findings here). In an adopter's repository it is a dead end: the maintainers never see it. So the `report-jigc-feedback` workflow ships **hidden** (`selectable: false` + `suppressed`, the `record-dogfood` precedent), callable by name by this repository's agents, and S3 keys its opening to this idea: *open it up when the feedback interface exists*.

## The direction

A service that receives `jigc-feedback` submissions from adopters, and a submission path from the CLI to it. Two shapes, cheapest first:

1. **The CLI writes a submission bundle; a human sends it.** `jigc` renders a filed feedback doc into a self-contained payload (a file, or a prefilled issue body) and never opens a socket. The determinism boundary and the no-network posture stay intact; the human is the transport.
2. **An explicit submit verb.** The CLI's first outbound network call — opt-in, per submission, never automatic, never at finalize. A large posture change to weigh at pickup: today the only network statement in the design is the probe sandbox's prohibition ([validation.md](../design/validation.md) → the pack-probe determinism contract).

Either way the router opens `report-jigc-feedback` to adopters once there is somewhere for it to go.

## Submissions from other people's private repositories (S9)

The human's addition: feedback will come from repositories we cannot access, and a submission must be safe to send. Revision 1's fields split in two:

| Shareable as-is | Local context — may carry private names, paths or code |
|---|---|
| `kind`, `jigc-version`, `about`, `tier`, `description` | `found-in` (a task id, a milestone slug, a review label), `repro` (argv, paths, file contents), and on `inconsistency` the `sides` (paths and quoted text) |

So a submission needs a **scrub/consent step**: show the exact payload, let the reporter redact or replace the local fields, and require an explicit yes before anything leaves. The anonymization vocabulary this repository already enforces on itself — `acme`, `project-alpha`…, `TKT-<n>` ([public-hygiene.md](../implementation/public-hygiene.md) → The rules) — is the natural replacement map. To weigh at pickup: `description` is prose and can name a client as easily as `repro` can, so "shareable as-is" is a default for the field, not a guarantee about its content. No schema change was made for this at M55.

## What it would cost

- Hosting, authentication and abuse handling for a public endpoint; a privacy statement.
- A **versioned submission format** — a new machine contract, pinned like the read surfaces ([doc-read-surface.md](../design/doc-read-surface.md)).
- De-duplication across adopters: `duplicate-of` is a ref inside one repository's store and cannot point across them.
- Opening the hidden workflow: the `selectable` flip and the router-visible compose-golden blast.
- Shape 2 only: the network posture change, its failure modes (offline, proxy, timeouts) and its tests.

## Trigger

The first external adopter who wants to report something about jigc and has no channel but a public issue — or the post-1.0 adoption volume at which the maintainers learn of problems only by chance. Related: [issue-tracker-integration](issue-tracker-integration.md) (external trackers as task *origination*, the inbound direction).
