"""Fire a plant on a state, from outside the session, without a human watching.

The 1.0.0-gate trial proved which instrument shape works. From
`cue-card-postmortem.md` §4, comparing the plants that fired against the cue card
that did not:

| property | plant | cue card |
|---|---|---|
| trigger is a state, not a moment | the corpus holds the file or it does not | a string in a scrolling transcript |
| firing is a command, not a vigil | a script with a precondition bar | watch, judge, `cat`, copy, paste, in 14 s |
| failure is loud | prints `refusing: …` and plants nothing | nothing is printed, nothing is written |
| the product re-raises it | every `validate` and `finalize` until adopted | jigc never knows a correction was owed |

**The plants were already the right shape; the operator was the weak part.**
`b3-foreign-adr.sh` fires on `git rev-list --count "$INSTALL..HEAD" >= 2` — a state
— and refuses loudly below it. What the trial had no mechanism for was *noticing*
that the state had arrived: a human watched for the first `finalize` and then ran
`docker exec -it -u node <cid>` by hand, which is the step that killed session B3a
on `chmod: … Operation not permitted`.

This module is that missing poller and nothing more. It does not replace a plant,
change one, or decide when one *should* fire; it asks the plant's own precondition,
in the plant's own terms, on a clock.

**`-u node` is required, not cosmetic.** `docker exec` bypasses the image's
ENTRYPOINT, so it lands as root, and every `git` call in `/work` then dies on
"detected dubious ownership".

**Gate the corpus first, plant second** — `plants/README.md`. A planted corpus
deliberately breaks `check-corpus.sh` bars, so the gate must run while it still
means something. That ordering is the caller's to keep; this module fires only.
"""
from __future__ import annotations

import dataclasses
import pathlib
import subprocess
import time
from typing import Optional


@dataclasses.dataclass(frozen=True)
class Plant:
    """One plant: a state to wait for, and a script to run when it holds.

    `when` is a shell predicate evaluated **inside the live container**, against
    the corpus at `/work`, exactly as the plant's own precondition bar would be.
    Exit 0 means the state holds.

    `script` is a path on the host — an existing plant under
    `artifacts/<trial>/plants/`, unchanged.
    """
    name: str
    when: str
    script: pathlib.Path
    argument: str = "/work"


@dataclasses.dataclass(frozen=True)
class Firing:
    """What happened, in enough detail to put in a record.

    There is no "nothing happened" value: a plant that never fired says so with
    `fired=False` and a reason, because an instrument with a silent branch can
    return no data and a null is unreadable.
    """
    plant: str
    fired: bool
    reason: str
    stdout: str = ""
    stderr: str = ""
    returncode: Optional[int] = None
    waited_s: float = 0.0

    @property
    def refused(self) -> bool:
        """The plant ran its own precondition bar and declined. Loud, and expected."""
        return self.fired is False and self.returncode not in (None, 0)


def _exec(cid: str, command: str, *, user: str = "node") -> subprocess.CompletedProcess:
    return subprocess.run(
        ["docker", "exec", "-u", user, cid, "bash", "-lc", command],
        capture_output=True, text=True)


def container_alive(cid: str) -> bool:
    got = subprocess.run(["docker", "inspect", "-f", "{{.State.Running}}", cid],
                         capture_output=True, text=True)
    return got.returncode == 0 and got.stdout.strip() == "true"


def state_holds(cid: str, when: str) -> bool:
    """Ask the plant's own precondition, inside the container it is about."""
    return _exec(cid, when).returncode == 0


def fire(cid: str, plant: Plant) -> Firing:
    """Copy the plant's whole directory in and run it. Its refusal is kept verbatim.

    The **directory**, not the script — `b3-foreign-adr.sh` reads a sibling body file
    (`b3-foreign-adr-clean-prose.md`) resolved from `$BASH_SOURCE`'s own dirname, so a
    script copied alone dies on a missing file it never mentions in its usage. This is
    also the incantation the trial's own runbook documents:

        docker cp <plants-dir> <cid>:/tmp/plants
        docker exec -it -u node <cid> bash -lc '/tmp/plants/<plant>.sh /work'
    """
    src = plant.script.parent
    dest_dir = f"/tmp/{src.name}"
    _exec(cid, f"rm -rf {dest_dir}", user="root")
    copied = subprocess.run(["docker", "cp", str(src), f"{cid}:{dest_dir}"],
                            capture_output=True, text=True)
    if copied.returncode != 0:
        return Firing(plant.name, False,
                      f"could not copy the plant in: {copied.stderr.strip()[:200]}")
    # `docker cp` preserves the HOST uid, and `docker exec` lands as root unless told
    # otherwise — so the copied tree is unreadable-or-unwritable by `node` exactly as
    # a staged transcript was. Same class, same fix, one directory over.
    _exec(cid, f"chmod -R a+rX {dest_dir}", user="root")
    dest = f"{dest_dir}/{plant.script.name}"
    _exec(cid, f"chmod +x {dest}", user="root")
    got = _exec(cid, f"{dest} {plant.argument}")
    if got.returncode != 0:
        return Firing(plant.name, False,
                      f"the plant refused (exit {got.returncode})",
                      got.stdout, got.stderr, got.returncode)
    return Firing(plant.name, True, "planted", got.stdout, got.stderr, 0)


def watch_and_fire(cid_file: pathlib.Path, plant: Plant, *,
                   poll_s: float = 5.0, timeout_s: float = 3600.0) -> Firing:
    """Wait for the plant's state, then fire once. Every exit is a recorded outcome.

    Polls from **outside** the session, so nothing about the worker's pace matters
    and there is no window to miss — the property that separates a plant from a cue
    card. If the session ends before the state arrives, that is reported as such and
    is a finding about the arm, not a silent non-event.
    """
    started = time.monotonic()
    while not cid_file.is_file():
        if time.monotonic() - started > timeout_s:
            return Firing(plant.name, False, "no container id was ever written",
                          waited_s=time.monotonic() - started)
        time.sleep(0.5)
    cid = cid_file.read_text().strip()

    while True:
        waited = time.monotonic() - started
        if state_holds(cid, plant.when):
            got = fire(cid, plant)
            return dataclasses.replace(got, waited_s=waited)
        if not container_alive(cid):
            return Firing(plant.name, False,
                          "the session ended before the plant's state arrived",
                          waited_s=waited)
        if waited > timeout_s:
            return Firing(plant.name, False,
                          f"timed out after {timeout_s:.0f}s waiting for the state",
                          waited_s=waited)
        time.sleep(poll_s)
