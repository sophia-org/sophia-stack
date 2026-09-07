"""The Qt gate must reject plausible-looking pixels with a failed or absent grab."""
import importlib.util
from pathlib import Path
import unittest

SPEC = importlib.util.spec_from_file_location(
    "qt_popup_probe", Path(__file__).resolve().parents[1] / "run_qt_popup_probe.py")
PROBE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(PROBE)


def passing_log():
    lines = []
    for index, stage in enumerate(("first", "reopen", "nested", "dismiss"), 20):
        lines.append(f"qt_popup capture={stage} xid={index} result=pass")
        if stage != "nested":
            lines.append(f"qt_popup grab stage={stage} xid={index} api=xi2 sequence=100 mapped_before=1 status=0")
    for outcome in ("selection=first", "selection=nested", "dismiss=escape", "complete"):
        lines.append(f"qt_popup {outcome} result=pass")
    lines.extend((
        "qt_popup grab stage=draw-after-grab xid=30 api=core sequence=120 mapped_before=1 status=0",
        "qt_popup raw_grab=pass drew_after_success=1",
        "qt_popup raw_pixels=pass mapped=1 owned=1",
        "sophia_live_session schema=17 status=bounded_complete cpu_max_nonzero_pixel_bytes=10 cpu_nonzero_frames=40",
        "status=exited id=terminal source=startup exit_status=exit status: 0",
        "sophia_live_session_health schema=1 status=clean protocol_errors=0",
        "sophia_live_session_cleanup schema=1 status=clean"))
    return "\n".join(lines)


class QtPopupVerifier(unittest.TestCase):
    def test_complete_success(self):
        self.assertTrue(PROBE.verify(passing_log(), 0)["passed"])

    def test_failed_first_grab_with_valid_pixels_is_rejected(self):
        records = passing_log().replace("mapped_before=1 status=0", "mapped_before=1 status=3", 1)
        self.assertFalse(PROBE.verify(records, 0)["passed"])

    def test_unobserved_grab_is_not_success(self):
        records = "\n".join(line for line in passing_log().splitlines()
                            if not line.startswith("qt_popup grab stage=first"))
        self.assertFalse(PROBE.verify(records, 0)["passed"])

    def test_grab_on_wrong_window_is_not_popup_success(self):
        records = passing_log().replace("grab stage=first xid=20", "grab stage=first xid=99")
        self.assertFalse(PROBE.verify(records, 0)["passed"])

    def test_missing_map_order_is_rejected(self):
        records = passing_log().replace("mapped_before=1", "mapped_before=0", 1)
        self.assertFalse(PROBE.verify(records, 0)["passed"])

    def test_empty_compositor_cannot_pass_frontend_pixels(self):
        records = passing_log().replace("cpu_nonzero_frames=40", "cpu_nonzero_frames=0")
        self.assertFalse(PROBE.verify(records, 0)["passed"])

    def test_draw_before_grab_success_is_not_accepted(self):
        records = passing_log().replace("drew_after_success=1", "drew_after_success=0")
        self.assertFalse(PROBE.verify(records, 0)["passed"])

    def test_client_failure_cannot_pass_session_exit(self):
        records = passing_log().replace("qt_popup complete result=pass", "qt_popup complete result=fail")
        self.assertFalse(PROBE.verify(records, 0)["passed"])


if __name__ == "__main__":
    unittest.main()
