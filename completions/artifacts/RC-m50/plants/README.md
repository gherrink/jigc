# The plants this trial uses — by reference, not by copy

Every plant is reused **byte-identical** from the trial that built and rehearsed it. Copying them
here would make two files that can drift; the record instead names each source with its hash as
used on 2026-09-04, so a later reader can check nothing moved.

| plant | source | md5 | used on |
|---|---|---|---|
| **E** — the abandoned task (wrong title · `status: superseded` · back-dated) | [RC-1.0-final/plants/e-abandoned-task.sh](../../RC-1.0-final/plants/e-abandoned-task.sh) | `e8bcbee6ad930eeb65a2f9869c738add` | quillon · ashgrove · brackenmoor · r3-corpus |
| **hook** — the rejecting docs-gate `pre-commit` under `core.hooksPath` | [RC-1.0-gate/plants/b1-hook.sh](../../RC-1.0-gate/plants/b1-hook.sh) | `ae2939138cac3b02bc51c060eb597c5e` | larkspur · r4-corpus |
| **carryover** — two paths staged before the first mint | [RC-1.0-gate/plants/b1-staged.sh](../../RC-1.0-gate/plants/b1-staged.sh) | `37f0a34325328db709adc1756ce1fe76` | larkspur · r4-corpus |
| **foreign-ADR** — landed mid-session by the poller on a polled state | [RC-1.0-gate/plants/b3-foreign-adr.sh](../../RC-1.0-gate/plants/b3-foreign-adr.sh) with [b3-foreign-adr-clean-prose.md](../../RC-1.0-gate/plants/b3-foreign-adr-clean-prose.md) | `e9352173b40a9aaba10c5c61020c5e49` / `bd9a0b25f565ee4b85efc7d3ee588e24` | ashgrove · brackenmoor (B3, B3-h2) |
| **F** — the retitle correction into the hook's pause | [RC-1.0-final/plant-f-correction.md](../../RC-1.0-final/plant-f-correction.md) (a template, completed live) | — | larkspur (B1) |

Plant E's rehearsal on this binary is **R3**; plant F's stop is **R4** — both recorded in
[protocol.md](../protocol.md) §10 once run.
