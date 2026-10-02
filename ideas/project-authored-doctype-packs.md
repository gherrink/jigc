# project-authored doctype packs — a project defines its own doctypes, ergonomically

**Status: parked 2026-10-02.** Parked by the M55 Settle, S3 option (b) ([DECISIONS.md](../DECISIONS.md) → 2026-10-02 M55 settled; design of record [design/findings-channel.md](../design/findings-channel.md)). Indexed from [VISION.md](../VISION.md) → Open questions. Sits behind the *"not yet a public pack platform"* non-goal ([CLAUDE.md](../CLAUDE.md) → Non-goals).

## The gap

A project that uses jigc should be able to create its own doctypes. Mechanically it can: since M49 a **listed project pack** composes on top of the built-in ones ([multi-pack.md](../design/multi-pack.md) → The pack-set). In practice the M55 planning measured how much it costs (the baseline and gap probes; [planning-findings.md](../completions/artifacts/M55/planning-findings.md) → F13, F16, F17):

- **It must copy, not include.** A listed pack's workflow cannot reach the built-in steps and commands it needs, so it copies the methodology pack's `commands.yaml` and the `finalize`, `author-commit` and `migration-finalize` steps — copies that go stale silently on every jigc upgrade.
- **A same-id built-in demotes it without a word** — a listed-pack doctype that collides with a built-in id is shadowed with no warning, and `doc schema` shows the built-in shape (F13).
- **It cannot be prototyped in the rig composed with methodology** — `JIGC_PACK_DIR` disables the embedded methodology composition (`crates/cli/src/pack.rs` → `make_pack_from_marker`), so a doctype meant to live beside the methodology pack is tried out without it (F17).
- **A project-layer workflow shadow** is a whole-file copy with no recorded base, so it too goes stale silently (F16).

There is no stable authoring surface to point a project at.

## The direction

Make a project pack a first-class, upgrade-safe thing: an **include across packs** for built-in steps and command refs (today body references resolve pack-locally — [multi-pack.md](../design/multi-pack.md) → Pack-local body-reference resolution, a correctness rule this would have to extend, not break); a **loud collision** when a listed pack's id meets a built-in one; rig support for a listed pack composed with methodology; and a documented, versioned schema-authoring surface — the start of the pack platform the non-goal defers.

## Why parked

S3 took option (a) for the findings channel — ship in the methodology pack, workflow hidden — rather than (b), a project pack in this repository, which would also have run against the port's rule that *inventing a doctype to fit our own corpus is the exact inversion*. Making project packs ergonomic is new mechanism with no adopter asking for it yet.

## What it would cost

- Cross-pack include resolution with a stated precedence and a recorded base, so a project pack tracks the built-in steps it uses across upgrades (the delta invariant, applied to pack content).
- A collision finding at pack-load, and a decision on whether a project may deliberately shadow a built-in doctype.
- Version-gating for project-authored doctypes — a project manifest, or an explicit freeze-exempt posture — so their corpus migrates like the built-in ones.
- A stabilized authoring reference: what a schema may declare, what is frozen, what is not.

## Trigger

The first adopter that needs a doctype jigc does not ship and tries to author one — or the pack-platform non-goal's own release condition. Related: [methodology-kb-pack](methodology-kb-pack.md) (a candidate first tenant of the same surface), [pack-doctype-visibility](pack-doctype-visibility.md) (moving built-in content into packs of its own).
