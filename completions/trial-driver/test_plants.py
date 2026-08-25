"""The poller, against the real plant in a real container.

These are integration tests and they need docker plus `jigc-gate:rc11`; they skip
cleanly without either. They need **no API spend**, which is the point — the step
they cover is the one that voided session B3a of the 1.0.0-gate trial
(`chmod: … Operation not permitted`), and it was previously exercised only by a
human typing `docker exec` into a live blind session.

Both directions are asserted, because a plant that cannot refuse is not a plant:
`plants/README.md` states it as *"a plant assumed to fire is not a plant."*

Run: python3 completions/trial-driver/test_plants.py
"""
from __future__ import annotations

import pathlib
import shutil
import subprocess
import sys
import unittest

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))

from driver import plants

REPO = pathlib.Path(__file__).resolve().parents[2]
PLANT = REPO / "completions" / "artifacts" / "RC-1.0-gate" / "plants" / "b3-foreign-adr.sh"
TEMPLATE = REPO / "completions" / "trial-corpus-template"
TAG = "jigc-gate:rc11"

#: The B3 plant's own precondition, expressed as a state over the corpus history:
#: at least two commits since jigc's install commit, i.e. the worker's first
#: finalize has landed. Lifted from the plant rather than invented.
SINCE_INSTALL_GE_2 = (
    'cd /work && [ "$(git rev-list --count '
    '$(git log --format=%H --grep="install jigc workspace config" -n1)..HEAD)" -ge 2 ]'
)


def _docker_ready() -> bool:
    if shutil.which("docker") is None:
        return False
    if subprocess.run(["docker", "info"], capture_output=True).returncode != 0:
        return False
    return subprocess.run(["docker", "image", "inspect", TAG],
                          capture_output=True).returncode == 0


class PlantPoller(unittest.TestCase):
    """One container per test, adopted corpus inside, destroyed in tearDown."""

    cid: str = ""

    @classmethod
    def setUpClass(cls) -> None:
        if not _docker_ready():
            raise unittest.SkipTest(f"docker or {TAG} unavailable")
        if not PLANT.is_file():
            raise unittest.SkipTest("the B3 plant is not present")

    def setUp(self) -> None:
        self.cid = subprocess.run(
            ["docker", "create", "--entrypoint", "/usr/local/bin/session-entry",
             TAG, "sleep", "300"],
            capture_output=True, text=True, check=True).stdout.strip()
        subprocess.run(["docker", "start", self.cid], capture_output=True, check=True)
        # Built as `node`, never as root: `/work` already belongs to node in the
        # image, so a root `git init` there dies on git's dubious-ownership check
        # (rc 128) and any chown that was meant to follow never runs.
        #
        # The fixture satisfies every bar the plant checks BEFORE the one under
        # test, which is why it carries a `.jigc/` and a commit adding CLAUDE.md:
        # the plant refuses in its own declared order, and a fixture that trips an
        # earlier bar tests that bar instead of the intended one.
        self._exec_node(
            'cd /work && git init -q -b main && '
            'echo x > a.txt && git add -A && '
            'git -c user.name=T -c user.email=t@x commit -q -m "chore: project skeleton" && '
            'mkdir -p docs/decisions .jigc && echo "# CLAUDE" > CLAUDE.md && '
            'git add -A && '
            'git -c user.name=T -c user.email=t@x '
            'commit -q -m "chore(jigc): install jigc workspace config" && '
            # The adopt commit. It matters: the plant counts commits SINCE install
            # and wants >= 2, so on a real corpus the worker's first finalize is the
            # second one. A fixture without it makes the plant wait for a commit that
            # in the real flow has already happened.
            'echo "invocation-log = true" > .jigc/config && git add -A && '
            'git -c user.name=T -c user.email=t@x '
            'commit -q -m "chore: adopt jigc for document management"')

    def _exec_node(self, cmd: str) -> subprocess.CompletedProcess:
        got = subprocess.run(["docker", "exec", "-u", "node", self.cid, "bash", "-lc", cmd],
                             capture_output=True, text=True)
        if got.returncode != 0:
            raise AssertionError(f"fixture setup failed ({got.returncode}): "
                                 f"{got.stderr.strip()[:300]}")
        return got

    def tearDown(self) -> None:
        if self.cid:
            subprocess.run(["docker", "rm", "-f", self.cid], capture_output=True)

    def _plant(self) -> plants.Plant:
        return plants.Plant(name="b3-foreign-adr", when=SINCE_INSTALL_GE_2, script=PLANT)

    def _land_a_worker_commit(self) -> None:
        """What the plant is waiting for: the worker's first finalize landing."""
        self._exec_node(
            'cd /work && echo z >> a.txt && git add -A && '
            'git -c user.name=W -c user.email=w@x commit -q -m "feat: a worker commit"')

    # ------------------------------------------------------------------ the state

    def test_the_state_is_false_before_the_first_finalize(self) -> None:
        self.assertFalse(plants.state_holds(self.cid, SINCE_INSTALL_GE_2))

    def test_the_state_becomes_true_when_the_commit_lands(self) -> None:
        """A state, not a moment: true continuously from then on."""
        self._land_a_worker_commit()
        self.assertTrue(plants.state_holds(self.cid, SINCE_INSTALL_GE_2))
        self.assertTrue(plants.state_holds(self.cid, SINCE_INSTALL_GE_2),
                        "still true on a second look — that is what makes it a state")

    # ---------------------------------------------------------------- the firing

    def test_firing_early_is_refused_loudly_by_the_plant_itself(self) -> None:
        """`a plant assumed to fire is not a plant` — so the refusal is asserted."""
        got = plants.fire(self.cid, self._plant())
        self.assertFalse(got.fired)
        self.assertTrue(got.refused)
        self.assertIn("refusing:", got.stderr)
        self.assertIn("first finalize has not landed", got.stderr)

    def test_the_plant_lands_once_the_state_holds(self) -> None:
        self._land_a_worker_commit()
        got = plants.watch_and_fire(self._cid_file(), self._plant(),
                                    poll_s=0.5, timeout_s=30)
        self.assertTrue(got.fired, got.reason + got.stderr)
        self.assertEqual(got.reason, "planted")
        landed = subprocess.run(
            ["docker", "exec", "-u", "node", self.cid, "bash", "-lc",
             "ls /work/docs/decisions/"], capture_output=True, text=True)
        self.assertIn("keep-the-sample-store-in-memory", landed.stdout)

    def test_the_sibling_body_file_travels_with_the_script(self) -> None:
        """The plant reads a body resolved from its own dirname.

        Copying the script alone makes it die on a file it never mentions in its
        usage, so the poller copies the directory — the incantation the trial's own
        runbook documents.
        """
        self._land_a_worker_commit()
        got = plants.fire(self.cid, self._plant())
        self.assertTrue(got.fired, got.stderr)
        self.assertIn("b3-foreign-adr", got.stdout)

    # ------------------------------------------------------------ no silent branch

    def test_a_dead_container_reports_rather_than_hangs(self) -> None:
        subprocess.run(["docker", "rm", "-f", self.cid], capture_output=True)
        got = plants.watch_and_fire(self._cid_file(), self._plant(),
                                    poll_s=0.2, timeout_s=10)
        self.assertFalse(got.fired)
        self.assertIn("ended before", got.reason)

    def test_a_missing_container_id_is_an_outcome_not_a_hang(self) -> None:
        got = plants.watch_and_fire(pathlib.Path("/nonexistent/cid.txt"),
                                    self._plant(), poll_s=0.2, timeout_s=1)
        self.assertFalse(got.fired)
        self.assertIn("no container id", got.reason)

    def _cid_file(self) -> pathlib.Path:
        import tempfile
        p = pathlib.Path(tempfile.mkstemp()[1])
        p.write_text(self.cid + "\n")
        return p


if __name__ == "__main__":
    unittest.main(verbosity=2)
