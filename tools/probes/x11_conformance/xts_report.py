"""Strict selected-purpose TET journal accounting; never intersect away missing cases."""
import argparse
import json
from pathlib import Path

VERDICTS = {0: 'PASS', 1: 'FAIL', 2: 'UNRESOLVED', 3: 'NOTINUSE',
            4: 'UNSUPPORTED', 5: 'UNTESTED', 6: 'UNINITIATED', 7: 'NORESULT'}


def parse_journal(text):
    activities, started, results = {}, set(), {}
    for line in text.splitlines():
        fields = line.split('|')
        if fields[0] not in ('10', '200', '220'):
            continue
        if len(fields) != 3:
            raise ValueError(f'malformed journal record: {line}')
        values = fields[1].split()
        if fields[0] == '10':
            activity, case = values[:2]
            if activity in activities:
                raise ValueError('duplicate activity identity')
            activities[activity] = case
            continue
        activity, purpose = values[:2]
        if activity not in activities:
            raise ValueError('purpose has no test-case start')
        key = (activities[activity], purpose)
        if fields[0] == '200':
            if key in started:
                raise ValueError('duplicate purpose start')
            started.add(key)
        else:
            if key not in started or key in results:
                raise ValueError('unstarted or duplicate purpose result')
            status = VERDICTS.get(int(values[2]))
            if status is None or status != fields[2].strip():
                raise ValueError('numeric and textual verdict disagree')
            results[key] = status
    if not started:
        raise ValueError('empty journal: no purposes started')
    return started, results


def evaluate_journal(expected, journal, process_status=0):
    keys = [(row['case'], str(row['purpose'])) for row in expected]
    if not keys or len(keys) != len(set(keys)):
        raise ValueError('expected purposes must be nonempty and unique')
    required = set(keys)
    failures = []
    if process_status != 0:
        failures.append(f'XTS process exit {process_status} (124 means TIMEOUT)')
    started, results = parse_journal(journal)
    for key in sorted(required | started):
        if key not in required:
            failures.append(f'{key}: unmanifested purpose')
        elif key not in started:
            failures.append(f'{key}: MISSING purpose')
        elif results.get(key) != 'PASS':
            failures.append(f'{key}: {results.get(key, "NORESULT")}')
    return {'status': 'FAIL' if failures else 'PASS', 'failures': failures,
            'required': len(required), 'started': len(started), 'completed': len(results)}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--expected', type=Path, required=True)
    parser.add_argument('--journal', type=Path, required=True)
    parser.add_argument('--process-status', type=int, default=0)
    args = parser.parse_args()
    try:
        report = evaluate_journal(json.loads(args.expected.read_text()), args.journal.read_text(),
                                  args.process_status)
    except (ValueError, KeyError, IndexError, OSError) as error:
        report = {'status': 'FAIL', 'failures': [str(error)]}
    print(json.dumps(report, indent=2))
    return 0 if report['status'] == 'PASS' else 1


if __name__ == '__main__':
    raise SystemExit(main())
