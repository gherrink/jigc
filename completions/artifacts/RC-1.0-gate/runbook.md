# Operator runbook — the commands, in order

Everything you paste is a file under `paste/`. **`cat` it, copy the output, paste it. Never retype.**

Run every command from the repo root:

```sh
cd /Users/maurice/projects/gherrink-jigc
```

`run-session.sh` takes **two arguments — corpus, out-dir** — and you run it **once per session**. On
exit it copies the evidence out and destroys the container **by itself**. There is nothing to run
afterwards.

---

## Status

| | session | state |
|---|---|---|
| ✅ | **Arm 0** control | PASS, in the rig — evidence in `~/ideas/rc11-control-out/` |
| ✅ | **B1** `harborlight` | done, archived in `~/ideas/harborlight-out/` (26 log records, transcript, provenance) |
| ▶ | **B2** `pinegrove` | next |
| | **B3** `stonefly` | after B2 |
| | **walk** `rc11-walk` | last |

---

## B2 — do this

**1. Start it.**

```sh
./completions/trial-harness/run-session.sh ~/ideas/pinegrove ~/ideas/pinegrove-out
```

Write down the **container id** it prints (`container  : <id>`).

**2. Paste the prompt.**

```sh
cat completions/artifacts/RC-1.0-gate/paste/b2-prompt.txt
```

**3. Wait for the trigger: the 3rd `set slot research:…` ack.**

**4. Before pasting the correction, check the slug will actually change:**

```sh
cat completions/artifacts/RC-1.0-gate/paste/b2-correction.txt
```

Use PRIMARY. Use ALTERNATE only if the primary's title would derive the slug the doc already has —
if it does, `doc rename` acks *"the id is unchanged"* at exit 0 and the arm silently no-ops.

**5. Paste it. Then immediately tell me the time and the container id** — I record the channel counts
at that moment; reconstructed ones are not evidence.

**6. If the worker asks you anything**, tell me before answering. Replies come from
`answer-key.md`; improvising one is how a session gets contaminated.

**7. At the end, while the session is still open:**

```sh
cat completions/artifacts/RC-1.0-gate/paste/feedback-prompt.txt
```

Paste it, then give me the worker's answer.

**8. Exit the session.** That is all — copy-out is automatic.

**B2 has no plants.** Nothing to land, nothing to release.

---

## B3 — do this

**1. Start it.**

```sh
./completions/trial-harness/run-session.sh ~/ideas/stonefly ~/ideas/stonefly-out
```

**2. Paste the prompt.**

```sh
cat completions/artifacts/RC-1.0-gate/paste/b3-prompt.txt
```

**3. The plant lands MID-SESSION — after the worker's first `finalize` lands, not before.** Tell me
when you see the first finalize succeed and I will run it, or run it yourself with the container id:

```sh
./completions/artifacts/RC-1.0-gate/plants/b3-foreign-adr.sh <container-id>
```

**4. Trigger: the first `spec:<slug>#criteria/<id>` ack** (a `doc add-item`). Then:

```sh
cat completions/artifacts/RC-1.0-gate/paste/b3-correction.txt
```

Same slug check as B2 step 4.

**5–8.** As B2: tell me the time, ask me before answering questions, paste the feedback prompt while
the session is open, exit.

---

## The walk — after all three blind sessions

```sh
./completions/trial-harness/verify-pair.sh          # must pass before arm 2
./completions/trial-harness/run-session.sh --shell ~/ideas/rc11-walk ~/ideas/rc11-walk-out
```

`--shell` gives a plain shell instead of a Claude session. The arms are in
[protocol.md](protocol.md) §5; I will walk you through them command by command when you get there.

---

## The two rules that void a measurement

1. **Never say anything to a worker that names reading, checking or verifying** — or a synonym. That
   voids the session's read-back measurement. If it happens, tell me; a logged void is recoverable, a
   hidden one poisons the result.
2. **Paste from `paste/`, verbatim.** Do not summarise, do not add context, do not answer the
   worker's questions on your own.

---

## Owed at close (mine, not yours)

`trial-record.md` · `findings-verification.md` with live repros · the coverage table · the PT-1
disposition recheck ([pre-trial-findings.md](pre-trial-findings.md)).
