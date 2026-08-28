# Walk arms

One script per arm, driven by `walk.py` through `run-session.sh --exec` — the same
copy-in / copy-out / provenance chain a blind session rides. The record carries a
block per arm **including the ones that did not run**, which is the whole point:

> a narrative walk lost a chartered probe. Make each arm a line with a paste command
> and a captured output block, so an unrun arm is visibly blank rather than quietly
> absent. — `cue-card-postmortem.md` §6 step 4

## Conventions

- Named `NN-<slug>.sh`; the number is the order and the label.
- **Arm `00` is the positive control.** If it fails, `walk.py` stops and refuses to
  present the rest as readable (protocol §3.4).
- Every step goes through the `step()` helper, which echoes the command, its
  **combined** output and its **exit code**. That is not decoration: on this
  directory's first real run a step printed nothing and the record could not say
  whether it had succeeded quietly or failed quietly. It had in fact emitted a
  full blocking finding — on stderr, which the arm was not capturing.
- **The runtime image is node-based and carries no `python3`.** Arm 01 died on
  `python3: command not found` the first time it ran; parse JSON with `node -e`.
- An arm states its pass condition **before** the run, and prefers capturing
  output to asserting a string its author guessed.

## Arms

| arm | what it drives | the set it iterates |
|---|---|---|
| `00-positive-control.sh` | mint → author → `doc show … --task`; proves §3.3's channel fires and is countable | — (a control) |
| `01-destination-occupancy.sh` | the identity surfaces at a bound role | — |
| `02-destroying-doors.sh` | the four destroying doors: 3 refusal cells + narration at all four, incl. the `--ignored` axis | a **code-side registry** (`DESTROYING_DOORS`) |
| `03-upgrade-author-on-rc11.sh` | authors a mixed corpus on the OLD binary and measures both declared changes BEFORE | — (pairs with `10`) |
| `04-foreign-arm.sh` | protocol §0.1: `validate`'s exit flip, `migrate-corpus`'s `unadopted` key, and the exit returning to 0 | the declared change, over its two doors |
| `05-milestone-boundary.sh` | the boundary over own / `cp -R` / `mv` linkage, the last split on whether anything else lands | the class's **defining case-set** |
| `06-pre-guard-repair.sh` | `item-heading-unanchored`'s in-message repair; **declares** the two cells it cannot reach | a derivation, **with a stated hole** |
| `07-changelog-gate.sh` | protocol §0.2: the advisory at `task validate`, write-touch suppression, the promoted exit 3 | the declared change, over 2 doors × 2 severities |
| `08-identity-refusals.sh` | T10 committed-identity · T11 singleton · destination occupancy · the idempotent no-op | the postmortem's T7–T11 refusal set |
| `09-increment-8-doors.sh` | six of Increment 8's seven doors; T6 re-measured against the binary | a **hand-enumerated** set, said to be one |
| `10-upgrade-continue-on-rc12.sh` | the real 1.0.0 upgrade path, asserted against `03`'s measured baseline | — (pairs with `03`) |

## The upgrade pair needs three commands, not one

`03` and `10` are the only two-binary arms, so a normal walk pass cannot drive
them. **Each skips loudly** when handed the wrong binary or a corpus without the
other half's baseline — an arm that cannot run must not look like an arm that
found nothing.

```sh
python3 walk.py <corpus>   <out-a> --tag jigc-gate:rc11 --only 03
python3 run.py  carry <out-a>/03-upgrade-author-on-rc11 <corpus-b>
python3 walk.py <corpus-b> <out-b> --tag jigc-gate:rc12 --only 10
```

`run.py carry` between the halves is load-bearing: `run-session.sh` writes its own
evidence into the out-dir, and handing that on plants the rig's droppings in the
corpus the second half reads.

## Declared bound — arm 01 does not reach the two-doc collision

§6 step 5 asks for *"two `doc rename`s onto one identity, in both homes."* Arm 01
reaches the identity surfaces and records what each says, but **not** that:

`record-decision` declares **one** `allows-create` adr role, so a second
`doc create adr` in the same task is refused outright — `write.identity-change`,
exit 1, with an argv-complete route — rather than producing a second document to
collide with. A one-role workflow cannot hold two docs of that type, so the
two-doc case needs a workflow with two same-type create gates, or two tasks.

That is a finding about the **probe**, not about the guard, and it is recorded here
rather than left as a quiet gap — which is the failure §6 step 4 exists to prevent.
What arm 01 *does* establish on `1.0.0-rc.11`, verbatim in the record:

- a second create at a bound role is **blocked** and routed to `doc rename`;
- a same-title `doc rename` acks *"the id is unchanged"* at exit 0 — the documented
  no-op that `cue-cards.md` §0.7 identified as a silent-arm hazard;
- a top-level `jigc rename` against a **staged** doc reports no such managed doc,
  because its subject is the committed store.
