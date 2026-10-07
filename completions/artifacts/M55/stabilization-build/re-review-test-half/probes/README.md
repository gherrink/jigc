# The runtime probes — what they showed

**Run 2026-10-07 by the orchestrating session, from the main checkout, at `1d21024e` (task `K0`).** Four invocations of the stabilization harness that work on no run, no branch and no remote, each observing one fact about the Workflow runtime that nobody had observed; a fifth fact turned up beside them. The probes' design is [the plan](../plan.md) → §5; what they changed in it is its *What the probes showed* there, and `DECISIONS.md` → *2026-10-07 — The runtime probes, run*. This file states the results and restates neither.

**What is committed here.** The four `result.json` files, one per probe, as `dev/stabilize-probe verdict` wrote them — byte-identical copies, renamed after their probe — **and a fifth, [relay-digest.json](relay-digest.json): the relay probe run again on the digest**, a byte-identical copy too, named to tell it from the first relay run. Each holds the cases as the script judged them: how every try of a call ended, the byte counts and hashes the harness computed, and the verdict. They hold no host path and no id. **What is not committed:** the probes' scratch roots and the workflow journals, which hold what each agent said; the few words quoted below are from those journals, and nothing else of them is copied.

| file | probe | what was asked | what came back |
|---|---|---|---|
| [relay.json](relay.json) | `relay` | a git step runs `dev/stabilize-step state` on a throwaway run and relays its one line; four sizes, three times each | **never whole.** 1 ledger row (3,345 bytes sent): `altered` ×3. 20 rows (25,672): `altered` ×3. 60 rows (80,682): `missing` ×2, `altered` ×1. 120 rows (159,194): `missing` ×3 |
| [payload.json](payload.json) | `payload` | the harness composes a record's batch of 20, 60, 120 and 300 entries; the executor writes it from its prompt and returns the file's hash | **whole, four of four** — 8,412 to 120,204 bytes; the hash on disk equal to the expected one, the relayed one agreeing |
| [required.json](required.json) | `required` | two reviewers under a schema that requires the hash: (a) told to leave it out; (b) a source pass that drives nothing | (a) **`threw`, three tries of three** — nothing came back, not even the fields that were there. (b) `returned`, first try, **with the hash it was handed** |
| [hold.json](hold.json) | `hold` | a command of 2,100 seconds: (a) held by one agent inside its turn; (b) started detached by one agent and asked for by others | (a) **`held`**, 2,100.06 s. (b) **`outlived`** its starter: 22 reads, the last `done`, 2,100.03 s |
| [relay-digest.json](relay-digest.json) | `relay`, **run again on the digest** — after the plan's `K9`, at `c2169e19`; the result file is dated 2026-10-07 on the machine it ran on, and the orchestrator's brief for `K10` names the run as of 2026-10-08 | a git step runs `dev/stabilize-step state --digest` on a throwaway run and relays its one line — the state's **digest**, printable ASCII by construction; four sizes (1, 10, 30 and 60 items over the same four ledgers), three times each | **whole, twelve of twelve**, each on its first try — 1,343 · 2,682 · 5,645 · 10,090 bytes sent, and the same bytes back |

## What each result means

**The relay fails at every size, for two reasons.** The orchestrator diffed the smallest case against what was sent:

1. **The line's escapes do not survive a structured return.** The step tool prints every character that is not ASCII as a `\uXXXX` escape; the agent's return decodes them to the characters; the harness hashes the bytes. The 27 bytes the smallest line lost are nine such escapes become nine three-byte characters. So a line fails its hash **whenever a record holds a dash, an arrow or a typographic quote** — at 3 KB as at 30.
2. **From about 80 KB the line does not come back at all.** The command's output is too large for the agent to be handed whole — its tool result says so and saves it to a file — and of the calls that tried to copy it, two were stopped by a safety classifier, which the runtime reported as the call having failed (`nothing`), and the others returned no line: *"I will not return a shortened or reconstructed line, because the harness holds the line to its hash."*

So the re-review's `R-H1` is not an inference about 75 KB any more. **The first state read of any real stage fails**, and so does any other line of the step tool that carries a commit's subject or a refusal's prose with one such character in it. The simulation could not show it: its stand-ins copy bytes.

**The write direction needs no change.** A payload of 120 KB composed by the harness reaches the disk as composed.

**A return that omits a required field is a call that failed** — the runtime's words, *"subagent completed without calling StructuredOutput (after in-conversation nudge)"* — three times, and each try is a whole run of that agent. So no field whose absence should cost one item may be `required`: it would cost three runs and a count toward the breaker. One sample says a reviewer that drives nothing returns the hash unasked.

**A command of thirty-five minutes can be held either way** — but (b) cost 26 agents and about 835,000 tokens for one wait, because every read was an agent of its own. One agent that asks again is the cheap form; that the process outlives its starter is what lets another agent take a wait over.

## Three things no probe was built for

- **A workflow invoked by its name runs the copy the session loaded first.** The self-test counted 373 checks by name and 421 by `scriptPath` after `K0` had landed in the same session. Every invocation in a session that has edited the harness goes by `scriptPath`.
- **The hold probe asked the human for a permission.** The harness tells an agent to start a long command *in the background* and wait *in slices*, and the agent composes the waiting shell itself — a command no allow rule can name. The human's ruling on seeing it: *"Hold used an script bash command that needed my permission that should not happen because then it can not run autonomous there should be tooling for this."*
- **The reviewer of case (b) reviewed what it was handed**, which was `dev/stabilize-probe` itself, and left four LOW findings and a lead on it: a failure that is no refusal prints a traceback and exits 1, where the header promises one line of JSON whatever the exit (`SP-1`); only the scratch root is resolved, so a link planted at `<scratch>/probe` carries the probe's writes elsewhere (`SP-2`); two verdicts of `required` are judged on the last try only (`SP-3`); the header names three of the four files a throwaway run copies (`SP-4`); and a `held` asked in the microseconds between a hold's record being created and written would meet `SP-1`'s traceback (lead). Each is a row of the plan's §7.

## What they leave unsettled

- ~~Whether a line that is pure ASCII and a few kilobytes long is relayed whole.~~ **Settled by the run on the digest** ([relay-digest.json](relay-digest.json), the last row of the table): the digest was relayed whole twelve times of twelve, from 1.3 to 10.1 KB. That result is what the start of the harness task (the plan's `K3`, which builds the harness on the digest) rested on; the plan's condition for it was *whole, three of three, at each of the four sizes*. What it does not show: a digest above 10 KB, and any line of the step tool other than the state's.
- A structured return of 26 KB arrived; of 80 KB, one in three. **A triage's return grows with the findings it is handed**, and nothing measured one.
- How an agent that *dies* reaches the script. Case (a) of `required` shows a call that fails by its own doing; nothing here killed an agent.
- Whether an allow rule stops the prompt the hold probe met: shown only once the held-command acts exist and the rule is in place.
- A payload written with an agent's file tool rather than by a here-document, which is what the plan's held-command task changes.
