import unittest
import clipboard_check
import runner

def records():
    stages = [("process_start", 0), ("clipboard_window", 123),
              ("clipboard_drop_enter", 123), ("clipboard_join_enter", 0),
              ("clipboard_destroy_notify", 123), ("clipboard_join_return", 1),
              ("clipboard_drop_complete", 0), ("process_exit", 0)]
    return [dict(seq=i, stage=stage, value=value, dropped=0, thread="ThreadId(1)")
            for i, (stage, value) in enumerate(stages)]

class ClipboardTests(unittest.TestCase):
    def test_requires_notification_exact_identity_join_and_clean_exit(self):
        events = records()
        self.assertEqual(clipboard_check.evaluate(events, 0, False)["status"], "PASS")
        for index in (4, 5, 6, 7):
            changed = records()
            changed[index]["stage"] = "heartbeat"
            self.assertEqual(clipboard_check.evaluate(changed, 0, False)["status"], "FAIL")
        changed = records()
        changed[4]["value"] = 999
        self.assertEqual(clipboard_check.evaluate(changed, 0, False)["status"], "FAIL")
        self.assertEqual(clipboard_check.evaluate(events, -15, True)["status"], "FAIL")

    def test_reordered_or_duplicate_terminal_evidence_fails(self):
        changed = records()
        changed[4]["stage"], changed[5]["stage"] = changed[5]["stage"], changed[4]["stage"]
        changed[4]["value"], changed[5]["value"] = 1, 123
        self.assertEqual(clipboard_check.evaluate(changed, 0, False)["status"], "FAIL")
        changed = records()
        changed[3]["stage"], changed[3]["value"] = "clipboard_destroy_notify", 123
        self.assertEqual(clipboard_check.evaluate(changed, 0, False)["status"], "FAIL")

    def test_lost_or_invalid_diagnostics_cannot_pass(self):
        changed = records()
        changed[4]["dropped"] = 1
        self.assertEqual(clipboard_check.evaluate(changed, 0, False)["status"], "FAIL")
        self.assertEqual(clipboard_check.evaluate(records(), 0, False, True)["status"], "FAIL")
        self.assertEqual(clipboard_check.evaluate(records()[1:], 0, False)["status"], "FAIL")

    def test_wait_is_distinct_from_destroy_call_and_metadata_is_admitted(self):
        events = records()[:4]
        self.assertEqual(runner.last_open_span(events), "clipboard_join_enter")
        for event in records():
            runner.parse_trace(f't082 {event["seq"]} 123 ThreadId(1) 1 1 {event["stage"]} {event["value"]} 0'.encode(), 123)

if __name__ == "__main__":
    unittest.main()
