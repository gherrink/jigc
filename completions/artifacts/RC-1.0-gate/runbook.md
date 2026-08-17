# Operator runbook — the order of operations

The execution sheet. Design lives in [protocol.md](protocol.md), triggers in
[cue-cards.md](cue-cards.md), the texts in [blind-prompts.md](blind-prompts.md), the replies in
[answer-key.md](answer-key.md). This file is only *what to do, in what order*.

**Standing rules for every session:** paste prompts and utterances **verbatim**; log every operator
utterance in `operator-log.md` **as it happens**, with its timestamp; never answer a question about
how jigc behaves; no mid-trial fixes.

---

## Done

- **Arm 0 · the positive control — PASS, in the rig** (2026-08-16). The VERB channel records, greps,
  and survives copy-out. §3.4's *"control did not fire"* row is closed, so a null in a blind session
  is readable. Evidence: `~/ideas/rc11-control-out/`.
- **B1's corpus is planted and verified** (2026-08-17). See below.

---

## B1 · `~/ideas/harborlight` — cold start, both plants

### State at freeze, verified

```
staged (the carryover plant)   A  scripts/retention-sweep.sh
                               M  src/router.ts
core.hooksPath                 .githooks
hook                           .githooks/pre-commit — tracked, committed f97cb5f (back-dated)
release marker                 .git/docs-approved — ABSENT, i.e. rejecting
commits                        8   (7 template + the hook commit)
```

**Verified to survive the copy into the container, and to fire there** — the staged set, the
`core.hooksPath` setting and the executable hook all arrive intact, and a `docs/` commit inside the
container exits **1** with HEAD unmoved. A plant that fires on the host and not through the transport
is not a plant.

### 1 · start the session

```sh
cd /Users/maurice/projects/gherrink-jigc
./completions/trial-harness/run-session.sh ~/ideas/harborlight ~/ideas/harborlight-out
```

Note the container id it prints — the release step needs it.

### 2 · paste B1's prompt

Verbatim from [blind-prompts.md](blind-prompts.md) §1. Nothing added, nothing explained.

### 3 · watch for two things

| What you see | What it means | What you do |
|---|---|---|
| `jigc start --workflow decided-task …` | **Card B1 is dead** — that gate creates the `decisions-log` singleton, which `doc rename` refuses. | Nothing. Record the session as **unmeasured** for read-back (§3.4), let it run, keep collecting everything else. |
| the **3rd** `set slot adr:…` ack | Card B1's trigger | Go to step 4. |

### 4 · fire the cue card

Run [cue-cards.md](cue-cards.md) §0.7's pre-send check first — **if the replacement title derives the
slug the doc already has, the rename acks "the id is unchanged" at exit 0 and the arm silently
no-ops.** Use the card's alternate title if so.

Then paste the utterance verbatim, and log it immediately with its timestamp **and both §3.3 channel
counts at that moment** — not reconstructed later.

### 5 · the hook rejection, when it comes

The worker's doc-promoting `finalize` will be refused (exit 1, HEAD unmoved). It will likely ask how
the sign-off should happen — that is [answer-key.md](answer-key.md) **Card E**, and Card E's ordering
matters: **release the gate first, then reply.**

```sh
docker exec -it -u node <cid> bash -lc 'touch /work/.git/docs-approved'
```

`-u node` is not cosmetic: `docker exec` bypasses the entrypoint's gosu and lands as root, after
which every git call in `/work` dies on *dubious ownership*.

Do **not** tell the worker to create the marker itself. It lives inside `.git/`, deliberately outside
the worktree, so it cannot be swept into a finalize — RC-pre-1.0's reply pointed at a worktree path
and would put the worker's hand back exactly where this design moved it out of.

### 6 · at the end of the session, in the session

Paste the closing feedback prompt — **verbatim, from [blind-prompts.md](blind-prompts.md) §4** —
while the session is still open. Save the answer as `feedback-B1.md` **here, never into the corpus**
(a tracked `.md` there becomes an ingest candidate), and never feed it to a later worker.

### 7 · after the session

`run-session.sh` prints the counts and copies everything to `~/ideas/harborlight-out`. Record from the
**log and the transcript**, never from the worker's own account.

---

## Then, in order

- **B2 · `~/ideas/pinegrove`** — already adopted, invocation log already on, so its prompt carries
  neither. No plants. Correction targets a **research doc** (never the vision: it is a singleton and
  `doc rename` refuses it outright).
- **B3 · `~/ideas/stonefly`** — already adopted. The foreign-ADR plant lands **mid-stream**, after the
  worker's first finalize, with `plants/b3-foreign-adr.sh` — via `docker exec -it -u node`, and the
  script picks the clean-prose body for these corpora.
- **The walk · `~/ideas/rc11-walk`** — arms 1–4, with `run-session.sh --shell`. Arm 2 needs
  `jigc-gate:rc10`; run `verify-pair.sh` first, which refuses unless the old image is genuinely the
  pre-M48 tree.

## Owed at close

`trial-record.md` · `findings-verification.md` (every claim with a live repro, CONFIRMED **and**
REFUTED) · `operator-log.md` kept contemporaneously · the coverage table · per-session feedback ·
and the **recheck of PT-1's provisional disposition** ([pre-trial-findings.md](pre-trial-findings.md)).
