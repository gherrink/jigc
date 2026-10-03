# committed-doc edit gate — `allows-edit:` for the docs a workflow may change

**Status: parked 2026-10-02.** Parked by the M55 Settle, S5 ([DECISIONS.md](../DECISIONS.md) → 2026-10-02 M55 settled; design of record [design/findings-channel.md](../design/findings-channel.md) → 1.5). Indexed from [VISION.md](../VISION.md) → Open questions.

## The gap

Creation is gated: a workflow's `allows-create:` front-matter names the doctypes a task may create ([write-commands.md](../design/write-commands.md) → The create-gate). Editing is not. Any task can `set-field`, `set-slot`, `add-item` or `remove-item` on **any** committed doc, whatever its workflow allows — only `doc create` and `doc author` consult the gate (the M55 baseline, C7; [planning-findings.md](../completions/artifacts/M55/planning-findings.md) → F14). For most doctypes that is the intended freedom. For a record that is meant to be append-only — a finding, a research record, a decision — it means the rule lives only in prose.

M55 met it with the findings channel: both doctypes are **append-only by convention** — after filing only `status` and the resolution fields change — and the convention is stated in their `usage` and their design, with the engine not enforcing it.

## The direction

A workflow front-matter knob, `allows-edit:`, the create-gate's sibling: the doctypes (and possibly the leaves) a task composed from this workflow may change on a **committed** doc, checked by every doc-write verb before it moves bytes, cascade-overridable at the same key family as `allows-create`. Field-level granularity — *this workflow may set `status` and `resolution` on a `jigc-feedback`, nothing else* — is what would turn the findings convention into a rule.

## Why parked

Not cheaper now than later, and it blocks nothing: the report workflows only create, doc-level append-only is already free (no doc-delete verb), and the field-level convention holds as long as the agents follow their steps. The gap itself is recorded as a `jigc-feedback` row in the M55 seed.

## What it would cost

- A default that does not break the shipped workflows: an empty default would refuse every edit planning, completion and the milestone ops make today, so either the default is permissive (and the gate opt-in per doctype) or every shipped workflow declares its edits — an audit of both packs and a compose-golden blast.
- A check at every write verb's address-resolution point ([write-commands.md](../design/write-commands.md) → Every write resolves its address before it moves bytes), with its refusal code and routes.
- Leaf-level granularity is a second axis of the same knob, and its own design.
- The milestone-record's machine-maintained leaves are already refused to the `jigc doc` verbs ([team-ready-state.md](../design/team-ready-state.md) → The record is not writable through the `jigc doc` verbs); the gate would have to agree with that rule, not restate it.

## Trigger

The first observed out-of-workflow edit that destroyed a record's content (a finding's description rewritten after filing, a research record's findings replaced), or M57's triage asking for append-only to be enforced rather than conventional.
