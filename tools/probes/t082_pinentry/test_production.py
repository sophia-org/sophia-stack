import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch
import production
from production_protocol import ProtocolFailure, decode_data, encode_argument

FAKE = r'''
import os,signal,sys,time
assert not any(k.startswith("T082_") for k in os.environ)
assert "EGUI_INSPECTION" not in os.environ
mode=sys.argv[1]
description=b""
def emit(raw):
 sys.stdout.buffer.write(raw+b"\n");sys.stdout.buffer.flush()
emit(b"OK ready")
for raw in sys.stdin.buffer:
 cmd,_,arg=raw.rstrip(b"\n").partition(b" ")
 if cmd==b"GETINFO":
  emit(b"D "+str(os.getpid() if mode!="badpid" else 1).encode());emit(b"OK")
 elif cmd==b"SETDESC":
  description=arg;emit(b"OK")
 elif cmd in (b"RESET",b"SETTITLE",b"SETPROMPT"):
  emit(b"ERR 42 refused" if mode=="setup-error" else b"OK")
 elif cmd in (b"GETPIN",b"CONFIRM"):
  if mode in ("stall","ignore-term"):
   if mode=="ignore-term":signal.signal(signal.SIGTERM,signal.SIG_IGN)
   time.sleep(10)
  if mode=="flood":
   emit(b"x"*4096);time.sleep(10)
  cancel=any(word in description for word in (b"Cancel",b"Escape",b"window-manager"))
  if cancel:
   if mode=="cancel-data":emit(b"D PRIVATE_CANARY")
   emit(b"ERR 83886179 Operation cancelled")
  elif cmd==b"CONFIRM":
   if mode=="confirm-data":emit(b"D PRIVATE_CANARY")
   emit(b"OK")
  else:
   data=("café🔑%25".encode("utf-8") if b"caf%C3%A9" in description else b"test")
   if mode=="wrong":data=b"PRIVATE_CANARY"
   if mode=="mojibake":data="cafÃ©".encode("utf-8")
   if mode=="split":
    emit(b"D caf%C3%A9");emit(b"D %F0%9F%94%91%25")
   else:emit(b"D "+data)
   emit(b"OK")
 elif cmd==b"BYE":
  emit(b"OK closing")
  if mode=="hang-exit":time.sleep(10)
  if mode=="bad-exit":sys.exit(2)
  break
 else:emit(b"OK")
'''

class ProductionTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory(prefix="t082-production-test-")
        self.root = Path(self.tmp.name)
        self.fake = self.root / "fake.py"
        self.fake.write_text(FAKE)

    def tearDown(self):
        self.tmp.cleanup()

    def run_case(self, mode="normal", name="enter", observer=None, **kwargs):
        case = next(c for c in production.MATRIX if c["id"] == name)
        output = self.root / ("result-" + name + "-" + mode)
        with patch.dict(os.environ, {"T082_TRACE_FD": "123", "T082_DUMMY_PROBE": "1", "EGUI_INSPECTION": "1"}):
            result = production.run_case([sys.executable, str(self.fake), mode], output, case,
                                         observer, dialog_timeout=.3, setup_timeout=.3,
                                         exit_timeout=.3, **kwargs)
        return result, output

    def test_utf8_bytes_percent_and_multibyte_characters(self):
        raw = "café🔑%".encode("utf-8")
        self.assertEqual(decode_data(raw[:-1] + b"%25"), raw)
        self.assertEqual(decode_data(b"caf%C3%A9%F0%9F%94%91%25"), raw)
        self.assertEqual(encode_argument("café🔑%"), b"caf%C3%A9%F0%9F%94%91%25")
        for value in (b"%", b"%x0", b"%0", b"raw\r"):
            with self.assertRaises(ProtocolFailure):
                decode_data(value)

    def test_protocol_success_is_not_unattended_acceptance(self):
        result, _ = self.run_case()
        self.assertEqual(result["protocol_result"], "PASS")
        self.assertEqual(result["acceptance"], "NOT_ACCEPTED")
        self.assertIsNone(result["submission_latency_ms"])
        self.assertEqual(result["placement_evidence"], "not_recorded")
        self.assertFalse(result["harness_termination"])

    def test_attestation_and_successful_child_exit_are_required(self):
        seen = []
        def observe(spec):
            seen.append(spec["id"])
            return "confirmed"
        result, _ = self.run_case(observer=observe)
        self.assertEqual(result["acceptance"], "PASS")
        self.assertEqual(seen, ["enter"])
        with self.assertRaises(ProcessLookupError):
            os.kill(result["pid"], 0)

    def test_all_named_actions_and_confirm_responses(self):
        for name in ("ok", "cancel", "escape", "wm-close", "confirm-ok", "confirm-cancel"):
            result, _ = self.run_case(name=name)
            self.assertEqual(result["protocol_result"], "PASS", name)

    def test_unicode_split_data_and_corruption(self):
        result, _ = self.run_case("split", "unicode")
        self.assertEqual(result["protocol_result"], "PASS")
        result, _ = self.run_case("mojibake", "unicode")
        self.assertEqual(result["protocol_result"], "FAIL")

    def test_consecutive_dialogs_reuse_one_process(self):
        result, _ = self.run_case(name="consecutive")
        self.assertEqual(result["protocol_result"], "PASS")
        self.assertEqual([d["requested_command"] for d in result["dialogs"]],
                         ["GETPIN", "CONFIRM", "GETPIN"])
        self.assertEqual(len(result["dialogs"]), 3)

    def test_wrong_data_is_never_logged(self):
        result, output = self.run_case("wrong")
        self.assertEqual(result["protocol_result"], "FAIL")
        for file in output.iterdir():
            self.assertNotIn("PRIVATE_CANARY", file.read_text())

    def test_cancel_and_confirm_must_not_return_data(self):
        for mode, name in (("cancel-data", "cancel"), ("confirm-data", "confirm-ok")):
            result, _ = self.run_case(mode, name)
            self.assertEqual(result["protocol_result"], "FAIL")

    def test_pid_and_setup_failure_do_not_become_dialog_passes(self):
        for mode in ("badpid", "setup-error"):
            result, _ = self.run_case(mode)
            self.assertEqual(result["protocol_result"], "FAIL")
            self.assertEqual(result["dialogs"], [])

    def test_output_deadlines_and_exit_status_are_required(self):
        healthy = subprocess.Popen([sys.executable, "-c", "import time;time.sleep(15)"])
        try:
            for mode in ("stall", "ignore-term", "hang-exit", "bad-exit", "flood"):
                result, _ = self.run_case(mode)
                self.assertEqual(result["protocol_result"], "FAIL", mode)
                with self.assertRaises(ProcessLookupError):
                    os.kill(result["pid"], 0)
                self.assertIsNone(healthy.poll())
        finally:
            healthy.terminate()
            healthy.wait(timeout=2)

    def test_hint_parser_requires_exact_pid_and_atom(self):
        from production_hints import parse_properties
        text = "_NET_WM_PID(CARDINAL) = 123\n_NET_WM_WINDOW_TYPE(ATOM) = _NET_WM_WINDOW_TYPE_DIALOG\nWM_TRANSIENT_FOR:  not found.\n"
        self.assertIsNone(parse_properties(text, 12, "0x100"))
        found = parse_properties(text, 123, "0x100")
        self.assertTrue(found["dialog"] and found["transient_absent"])
        self.assertEqual(found["xid"], "0x100")
        self.assertFalse(parse_properties(text.replace("_DIALOG", "_DIALOG_FAKE"), 123, "0x100")["dialog"])

    def test_missing_case_or_placement_cannot_pass_matrix(self):
        results = [dict(case=c["id"], acceptance="PASS", placement_evidence="confirmed",
                        unicode_label_evidence="confirmed", dialogs=[dict(live_hints=dict(dialog=True, transient_absent=True))]) for c in production.MATRIX]
        self.assertEqual(production.verdicts(results)["acceptance"], "PASS")
        self.assertNotEqual(production.verdicts(results[:-1])["acceptance"], "PASS")
        results[0]["placement_evidence"] = "not_recorded"
        self.assertEqual(production.verdicts(results)["action_acceptance"], "PASS")
        self.assertNotEqual(production.verdicts(results)["acceptance"], "PASS")

    def test_production_wrapper_is_foreground_and_snapshots_identity(self):
        import prepare
        binary = self.root / "candidate"
        binary.write_bytes(b"dummy")
        binary.chmod(0o700)
        manifest = self.root / "manifest.json"
        manifest.write_text(json.dumps(dict(schema=1, instrumented=False,
            source_commit="a" * 40, sha256=production.digest(binary), binary_path=str(binary))))
        capture = self.root / "wrapper"
        capture.mkdir()
        prepare.prepare(Path(__file__).resolve().parents[3], capture, manifest)
        rc = (capture / "terminal.rc").read_text()
        self.assertIn("production.py", rc)
        self.assertNotIn(" &", rc)
        self.assertNotIn("runner.py", rc)
        identity = json.loads((capture / "production-candidate.json").read_text())
        self.assertEqual(Path(identity["binary_path"]).read_bytes(), b"dummy")
        self.assertNotEqual(identity["binary_path"], str(binary))

    def test_candidate_identity_is_explicit_and_snapshotted(self):
        binary = self.root / "candidate"
        binary.write_bytes(b"public dummy executable")
        binary.chmod(0o700)
        manifest = dict(schema=1, instrumented=False, source_commit="a"*40,
                        binary_path=str(binary), sha256=hashlib.sha256(binary.read_bytes()).hexdigest())
        path = self.root / "candidate.json"
        path.write_text(json.dumps(manifest))
        copied, identity = production.stage_candidate(path, self.root / "staged")
        binary.write_bytes(b"changed")
        self.assertEqual(copied.read_bytes(), b"public dummy executable")
        self.assertEqual(identity["candidate_kind"], "uninstrumented-production")
        with self.assertRaises(ValueError):
            production.stage_candidate(path, self.root / "wrong")
        manifest["instrumented"] = True
        path.write_text(json.dumps(manifest))
        with self.assertRaises(ValueError):
            production.stage_candidate(path, self.root / "traced")

if __name__ == "__main__":
    unittest.main()
