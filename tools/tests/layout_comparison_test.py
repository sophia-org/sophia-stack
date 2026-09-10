import importlib.util
from pathlib import Path
import tempfile
import unittest


SPEC = importlib.util.spec_from_file_location(
    "layout_comparison", Path(__file__).resolve().parents[1] / "verify_layout_comparison.py"
)
VERIFIER = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(VERIFIER)

PAIR = "output=2 format=875713112 original_modifier=123 alternative_modifier=0"
TESTED = (
    f"sophia_live_layout_probe schema=1 status=Tested {PAIR} scene_generation=91 "
    "source_image=817 original_status=Rejected original_errno=22 "
    "alternative_status=Submitted alternative_errno=none\n"
)
RETIRED = (
    f"sophia_live_layout_probe schema=2 status=RetiredCopy {PAIR} scene_generation=91 "
    "source_image=817 transaction=71 native_generation=9\n"
)
MATCHED = (
    f"sophia_live_layout_probe schema=2 status=PreferenceMatched {PAIR} "
    "transaction=71 native_generation=9 preference_generation=17\n"
)
FEEDBACK = (
    "sophia_live_session_present_feedback schema=1 kind=complete transaction=71 "
    "routed=true ust=10000 msc=23\n"
)
ALLOCATED = "dri3_layout event=allocation buffer=0 format=875713112 modifier=123\n"
SUBMITTED = "dri3_layout event=submit serial=3 pixmap=42 buffer=0 format=875713112 modifier=123\n"
COPY = "dri3_layout event=complete serial=3 pixmap=42 mode=copy ust=10000 msc=23\n"
FINISHED = "dri3_layout event=finished result=pass\n"
SESSION = [TESTED, RETIRED, MATCHED, FEEDBACK]
PROBE = [ALLOCATED, SUBMITTED, COPY, FINISHED]


class LayoutComparisonTests(unittest.TestCase):
    def verify(self, session=None, probe=None, transaction=None, expected_mode="copy"):
        return VERIFIER.verify(
            SESSION if session is None else session,
            PROBE if probe is None else probe,
            transaction,
            expected_mode,
        )

    def test_suboptimal_requires_explicit_opt_in_and_matching_delivered_mode(self):
        feedback = FEEDBACK.replace("routed=true", "routed=true mode=SuboptimalCopy")
        submit = SUBMITTED.rstrip() + " suboptimal=1\n"
        complete = COPY.replace("mode=copy", "mode=suboptimal")
        session = [TESTED, RETIRED, MATCHED, feedback]
        probe = [ALLOCATED, submit, complete, FINISHED]
        result = self.verify(session, probe, expected_mode="suboptimal")
        self.assertEqual(result["completion_mode"], "suboptimal")
        with self.assertRaises(VERIFIER.EvidenceError):
            self.verify(session, probe)
        for bad in (FEEDBACK, feedback.replace("SuboptimalCopy", "Copy"),
                    feedback.replace("SuboptimalCopy", "Flip")):
            with self.subTest(feedback=bad), self.assertRaises(VERIFIER.EvidenceError):
                self.verify([TESTED, RETIRED, MATCHED, bad], probe, expected_mode="suboptimal")
        for bad in (SUBMITTED, submit.replace("suboptimal=1", "suboptimal=0"),
                    submit.replace("suboptimal=1", "suboptimal=2")):
            with self.subTest(submit=bad), self.assertRaises(VERIFIER.EvidenceError):
                self.verify(session, [ALLOCATED, bad, complete, FINISHED], expected_mode="suboptimal")
        for mode in ("copy", "flip", "skip"):
            with self.subTest(mode=mode), self.assertRaises(VERIFIER.EvidenceError):
                self.verify(session, [ALLOCATED, submit, complete.replace("mode=suboptimal", f"mode={mode}"), FINISHED], expected_mode="suboptimal")

    def test_complete_chain_requires_real_copy_and_reports_owned_identity(self):
        result = self.verify()
        self.assertEqual(result["transaction"], 71)
        self.assertEqual(result["source_image"], 817)
        self.assertEqual(result["native_generation"], 9)
        self.assertEqual(result["preference_generation"], 17)
        self.assertEqual(result["probe_serial"], 3)
        self.assertEqual(result["original_stage"], "Atomic")
        self.assertNotIn("task_complete", result)
        for format_ in (875713112, 875713089):
            with self.subTest(format=format_):
                rows = [r.replace("875713112", str(format_)) for r in SESSION]
                probe = [r.replace("875713112", str(format_)) for r in PROBE]
                self.assertEqual(self.verify(rows, probe)["format"], format_)

    def test_capture_order_and_unrelated_records_are_not_ownership(self):
        rows = [FEEDBACK, MATCHED, "unrelated event=anything\n", RETIRED, TESTED]
        self.assertEqual(self.verify(rows), self.verify())
        # Identical scene labels are allowed only when the source identity differs.
        rows.append(TESTED.replace("source_image=817", "source_image=818"))
        self.assertEqual(self.verify(rows), self.verify())

    def test_explicit_original_stages_preserve_the_owned_evidence_chain(self):
        for stage in ("Atomic", "Framebuffer"):
            tested = TESTED.replace("schema=1", f"schema=3 original_stage={stage}")
            session = [tested, *SESSION[1:]]
            with self.subTest(stage=stage):
                expected = dict(self.verify(), original_stage=stage)
                self.assertEqual(self.verify(session), expected)
                for old, new in (
                    ("original_errno=22", "original_errno=16"),
                    ("original_status=Rejected", "original_status=Unknown"),
                    ("alternative_status=Submitted", "alternative_status=Rejected"),
                    ("alternative_errno=none", "alternative_errno=22"),
                    ("source_image=817", "source_image=818"),
                    ("scene_generation=91", "scene_generation=92"),
                ):
                    with self.subTest(change=new), self.assertRaises(VERIFIER.EvidenceError):
                        self.verify([tested.replace(old, new), *SESSION[1:]])
                with self.assertRaises(VERIFIER.EvidenceError):
                    self.verify([TESTED, *session])

    def test_unknown_or_missing_stage_never_becomes_a_framebuffer_proof(self):
        for replacement in (
            "schema=3", "schema=3 original_stage=Unknown",
            "schema=3 original_stage=Prime", "schema=3 original_stage=framebuffer",
            "schema=1 original_stage=Framebuffer", "schema=1 original_stage=Unknown",
            "schema=2 original_stage=Atomic", "schema=4 original_stage=Framebuffer",
        ):
            with self.subTest(replacement=replacement), self.assertRaises(VERIFIER.EvidenceError):
                self.verify([TESTED.replace("schema=1", replacement), *SESSION[1:]])

    def test_every_chain_edge_is_required(self):
        for index in range(len(SESSION)):
            with self.subTest(session_edge=index), self.assertRaises(VERIFIER.EvidenceError):
                self.verify(SESSION[:index] + SESSION[index + 1:])
        for index in range(len(PROBE)):
            with self.subTest(probe_edge=index), self.assertRaises(VERIFIER.EvidenceError):
                self.verify(probe=PROBE[:index] + PROBE[index + 1:])

    def test_test_results_must_prove_the_specific_counterfactual(self):
        changes = [
            ("original_status=Rejected", "original_status=Submitted"),
            ("original_errno=22", "original_errno=16"),
            ("alternative_status=Submitted", "alternative_status=Rejected"),
            ("alternative_errno=none", "alternative_errno=22"),
            ("source_image=817", "source_image=818"),
            ("scene_generation=91", "scene_generation=92"),
            ("format=875713112", "format=875713089"),
            ("output=2", "output=3"),
        ]
        for old, new in changes:
            with self.subTest(change=new), self.assertRaises(VERIFIER.EvidenceError):
                self.verify([TESTED.replace(old, new), *SESSION[1:]])

    def test_retirement_and_current_state_cannot_be_substituted(self):
        changes = [
            ("native_generation=9", "native_generation=10"),
            ("transaction=71", "transaction=72"),
            ("original_modifier=123", "original_modifier=124"),
            ("alternative_modifier=0", "alternative_modifier=1"),
            ("source_image=817", "source_image=0"),
            ("schema=2", "schema=1"),
        ]
        for old, new in changes:
            with self.subTest(change=new), self.assertRaises(VERIFIER.EvidenceError):
                self.verify([TESTED, RETIRED.replace(old, new), MATCHED, FEEDBACK])

    def test_implicit_equal_or_unknown_format_is_not_layout_proof(self):
        changes = [
            ("original_modifier=123", f"original_modifier={VERIFIER.IMPLICIT}"),
            ("alternative_modifier=0", f"alternative_modifier={VERIFIER.U64_MAX}"),
            ("original_modifier=123", "original_modifier=0"),
            ("format=875713112", "format=42"),
        ]
        for old, new in changes:
            with self.subTest(change=new), self.assertRaises(VERIFIER.EvidenceError):
                self.verify([r.replace(old, new) for r in SESSION])

    def test_duplicate_attempts_or_shared_completion_clocks_are_ambiguous(self):
        for duplicate in SESSION:
            with self.subTest(duplicate=duplicate), self.assertRaises(VERIFIER.EvidenceError):
                self.verify([*SESSION, duplicate])
        with self.assertRaises(VERIFIER.EvidenceError):
            self.verify([*SESSION, FEEDBACK.replace("transaction=71", "transaction=72")])
        with self.assertRaises(VERIFIER.EvidenceError):
            self.verify(probe=[*PROBE, COPY.replace("serial=3", "serial=4")])

    def test_matched_without_delivery_or_received_copy_is_insufficient(self):
        for old, new in [("routed=true", "routed=false"), ("ust=10000", "ust=10001")]:
            with self.subTest(change=new), self.assertRaises(VERIFIER.EvidenceError):
                self.verify([*SESSION[:3], FEEDBACK.replace(old, new)])
        with self.assertRaises(VERIFIER.EvidenceError):
            self.verify([*SESSION[:3], FEEDBACK.rstrip() + " mode=Flip\n"])
        for mode in ("flip", "skip", "suboptimal"):
            with self.subTest(mode=mode), self.assertRaises(VERIFIER.EvidenceError):
                self.verify(probe=[ALLOCATED, SUBMITTED, COPY.replace("mode=copy", f"mode={mode}"), FINISHED])
        with self.assertRaises(VERIFIER.EvidenceError):
            self.verify(probe=[ALLOCATED, SUBMITTED, COPY, FINISHED.replace("result=pass", "result=failed")])

    def test_probe_allocation_submission_and_completion_must_agree(self):
        changes = [
            (0, "modifier=123", "modifier=124"),
            (0, "format=875713112", "format=875713089"),
            (0, "buffer=0", "buffer=1"),
            (1, "modifier=123", "modifier=124"),
            (1, "pixmap=42", "pixmap=43"),
            (1, "serial=3", "serial=4"),
        ]
        for index, old, new in changes:
            probe = list(PROBE)
            probe[index] = probe[index].replace(old, new)
            with self.subTest(change=new), self.assertRaises(VERIFIER.EvidenceError):
                self.verify(probe=probe)

    def test_narrowing_selects_existing_identity_and_never_invents_it(self):
        self.assertEqual(self.verify(transaction=71), self.verify())
        with self.assertRaises(VERIFIER.EvidenceError):
            self.verify(transaction=72)
        with self.assertRaises(VERIFIER.EvidenceError):
            self.verify([*SESSION, MATCHED.replace("transaction=71", "transaction=72")])
        self.assertEqual(
            self.verify([*SESSION, MATCHED.replace("transaction=71", "transaction=72")], transaction=71),
            self.verify(),
        )

    def test_malformed_and_unbounded_inputs_are_refused(self):
        for value in ("-1", "+1", "0x1", str(1 << 64), "secret"):
            with self.subTest(value=value), self.assertRaises(VERIFIER.EvidenceError):
                self.verify([TESTED.replace("source_image=817", f"source_image={value}"), *SESSION[1:]])
        with self.assertRaises(VERIFIER.EvidenceError):
            self.verify([TESTED.rstrip() + " source_image=817\n", *SESSION[1:]])
        with self.assertRaises(VERIFIER.EvidenceError):
            self.verify(["x" * (VERIFIER.MAX_LINE + 1)])
        with self.assertRaises(VERIFIER.EvidenceError):
            self.verify(("ignored\n" for _ in range(VERIFIER.MAX_RECORDS + 1)))
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "log"
            path.write_text("x" * (VERIFIER.MAX_LINE + 1))
            with self.assertRaises(VERIFIER.EvidenceError):
                list(VERIFIER.read_lines(path))


if __name__ == "__main__":
    unittest.main()
