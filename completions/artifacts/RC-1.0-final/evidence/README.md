# Evidence

**Archived because the 1.0.0-gate trial's evidence previously existed on one machine only**, and
this project has already lost a machine mid-wave. Committed here so a claim in the record can be
re-checked without the rig, the images, or the corpora.

## What is here, and what is not

| file | what it is |
|---|---|
| `<session>-invocations.jsonl` | the **scored channel**, verbatim — a product surface with a pinned record shape |
| `<session>-result.json` | the CLI's final `result` event: `subtype`, `is_error`, `num_turns`, `permission_denials`, and the worker's own summary |
| `<session>-PROVENANCE.txt` | image, jigc version + sha, model, permission mode, corpus source, **session-start**, exit code |
| `walk-record*.md` | the operator walk, one block per arm **including the ones that did not run** |

**Full session transcripts are deliberately not archived.** They are ~1 MB each of tool traffic,
and the channels scored from them are the FILESYSTEM heuristic's hits — which
[protocol.md](../protocol.md) §3.4 already returns as *evidence for review* rather than as a
number, and which the record quotes in full where they mattered. The two exact channels come from
the invocation logs, which are here.

## Re-scoring from this directory

`run.py observe` reads a run directory, not this one. To re-score a session from the archive,
point it at a reconstructed layout, or read the numbers straight out of the logs — the channel
predicates are `driver/channels.py`, and they are the registration.

**The reader's own control lives elsewhere and still passes:**

```sh
python3 completions/trial-driver/run.py observe --archive   # reproduces the 1.0.0-gate table
```

That control is what licenses believing anything this reader says about a fresh session. It
reproduces the four archived 1.0.0-gate sessions exactly, including B1's corrected 3.

## The session-start stamp matters here

`PROVENANCE.txt` carries `session-start`, and it is load-bearing rather than decorative: the
invocation log lives **inside the corpus**, so records written before the session — by the
adoption arm, or by a plant — sit in the same file. Scoring without that split inflated a
rehearsal's headline **threefold** ([rehearsal-R1.md](../rehearsal-R1.md)). Any re-score from
these logs must apply it.
