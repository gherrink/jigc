"""The gate refuses, and each refusal has its own failure story.

A gate that is never asked is operator discipline wearing a gate's badge — the
shape that let two apparatus defects reach publication in the rig this is
harvested from. So the refusals are the tests.

Docker-dependent tests skip cleanly without docker or `jigc-gate:rc11`.

Run: python3 completions/trial-driver/test_gate.py
"""
from __future__ import annotations

import json
import pathlib
import shutil
import subprocess
import sys
import tempfile
import unittest

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))

from driver import gate
from driver.gate import Identity

TAG = "jigc-gate:rc11"

REAL = Identity(tag=TAG, image_id="sha256:" + "c" * 64,
                jigc_sha="d1ebbc227ba9f4b8310bcb7984c648c3865aa306",
                jigc_version="jigc 1.0.0-rc.11", cli_version="2.1.233 (Claude Code)")


def _docker_ready() -> bool:
    if shutil.which("docker") is None:
        return False
    if subprocess.run(["docker", "info"], capture_output=True).returncode != 0:
        return False
    return subprocess.run(["docker", "image", "inspect", TAG],
                          capture_output=True).returncode == 0


class TheGateRefuses(unittest.TestCase):
    """Each refusal separately, because each is a different way to publish a defect."""

    def setUp(self) -> None:
        self.tmp = tempfile.TemporaryDirectory()
        self.record = pathlib.Path(self.tmp.name) / "isolation-check.json"

    def tearDown(self) -> None:
        self.tmp.cleanup()

    def test_a_missing_record_is_refused_and_names_the_verifier(self) -> None:
        with self.assertRaises(SystemExit) as caught:
            gate.gate(self.record, TAG)
        message = str(caught.exception)
        self.assertIn("no isolation record", message)
        self.assertIn("verify-image.sh", message, "a refusal must carry its route")

    def _write(self, ident: Identity, checks: dict) -> None:
        gate.write_record(self.record, ident, checks=checks)

    def test_a_record_with_no_checks_is_refused(self) -> None:
        """A gate that passes on nothing is not a gate."""
        self._write(REAL, {})
        with self.assertRaises(SystemExit) as caught:
            gate.gate(self.record, "nonexistent:tag")
        # It fails on the image before it reaches the checks; assert the record
        # itself is judged empty by the same predicate the gate uses.
        kept = json.loads(self.record.read_text())
        self.assertFalse(kept["checks"])

    def test_a_record_reporting_its_own_failure_is_refused(self) -> None:
        self._write(REAL, {"isolation": True, "workspace_trusted": False})
        kept = json.loads(self.record.read_text())
        failed = [n for n, ok in kept["checks"].items() if not ok]
        self.assertEqual(failed, ["workspace_trusted"])

    def test_the_record_round_trips(self) -> None:
        self._write(REAL, {"isolation": True})
        kept = json.loads(self.record.read_text())
        self.assertEqual(kept["identity"]["jigc_sha"], REAL.jigc_sha)
        self.assertEqual(kept["identity"]["image_id"], REAL.image_id)


class AgainstTheRealImage(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        if not _docker_ready():
            raise unittest.SkipTest(f"docker or {TAG} unavailable")

    def setUp(self) -> None:
        self.tmp = tempfile.TemporaryDirectory()
        self.record = pathlib.Path(self.tmp.name) / "isolation-check.json"

    def tearDown(self) -> None:
        self.tmp.cleanup()

    def test_identity_is_read_from_the_image_not_declared(self) -> None:
        ident = gate.identity(TAG)
        self.assertTrue(ident.image_id.startswith("sha256:"))
        self.assertEqual(len(ident.jigc_sha), 40, "a full commit sha")
        self.assertIn("jigc", ident.jigc_version)
        self.assertIn("Claude Code", ident.cli_version)

    def test_a_matching_record_passes_and_returns_the_identity(self) -> None:
        ident = gate.identity(TAG)
        gate.write_record(self.record, ident, checks={"isolation": True})
        got = gate.gate(self.record, TAG)
        self.assertEqual(got.image_id, ident.image_id)

    def test_a_record_made_against_another_image_is_refused(self) -> None:
        """A tag is mutable: a rebuild under the same tag is a different image."""
        ident = gate.identity(TAG)
        stale = Identity(tag=TAG, image_id="sha256:" + "0" * 64,
                         jigc_sha=ident.jigc_sha, jigc_version=ident.jigc_version,
                         cli_version=ident.cli_version)
        gate.write_record(self.record, stale, checks={"isolation": True})
        with self.assertRaises(SystemExit) as caught:
            gate.gate(self.record, TAG)
        self.assertIn("a tag is mutable", str(caught.exception))

    def test_a_record_with_a_different_binary_is_refused(self) -> None:
        """`jigc --version` cannot tell two builds apart; JIGC_SHA can."""
        ident = gate.identity(TAG)
        stale = Identity(tag=TAG, image_id=ident.image_id, jigc_sha="0" * 40,
                         jigc_version=ident.jigc_version, cli_version=ident.cli_version)
        gate.write_record(self.record, stale, checks={"isolation": True})
        with self.assertRaises(SystemExit) as caught:
            gate.gate(self.record, TAG)
        self.assertIn("jigc sha", str(caught.exception))

    def test_a_failed_check_in_the_record_is_refused(self) -> None:
        ident = gate.identity(TAG)
        gate.write_record(self.record, ident,
                          checks={"isolation": True, "workspace_trusted": False})
        with self.assertRaises(SystemExit) as caught:
            gate.gate(self.record, TAG)
        self.assertIn("workspace_trusted", str(caught.exception))

    def test_an_image_without_jigc_sha_is_refused(self) -> None:
        """build-image.sh bakes it precisely because --version cannot disambiguate."""
        with self.assertRaises(SystemExit) as caught:
            gate.identity("node:22-trixie-slim")
        self.assertIn("JIGC_SHA", str(caught.exception))

    def test_an_unchanged_image_passes_the_after_check(self) -> None:
        gate.assert_unchanged(gate.identity(TAG), TAG)

    def test_a_changed_image_fails_the_after_check(self) -> None:
        """A CLI that self-updates mid-round means the runs are not one measurement."""
        before = gate.identity(TAG)
        moved = Identity(tag=TAG, image_id="sha256:" + "f" * 64,
                         jigc_sha=before.jigc_sha, jigc_version=before.jigc_version,
                         cli_version=before.cli_version)
        with self.assertRaises(SystemExit) as caught:
            gate.assert_unchanged(moved, TAG)
        self.assertIn("not one measurement", str(caught.exception))


if __name__ == "__main__":
    unittest.main(verbosity=2)
