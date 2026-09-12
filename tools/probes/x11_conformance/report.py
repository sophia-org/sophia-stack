"""Strict obligation accounting. An absent verdict is never a passing verdict."""
import json


def expected(manifest):
    assert manifest['schema'] == 1
    assert manifest['byte_orders'] == ['little', 'big']
    cases = manifest['cases']
    ids = [case['id'] for case in cases]
    assert ids and len(ids) == len(set(ids)), 'empty or duplicate manifest cases'
    for item in manifest['features']:
        assert item['cases'], f"feature has no behavioral obligations: {item['name']}"
        assert set(item['cases']) <= set(ids), f"unknown feature case: {item['name']}"
    for name, extension in manifest['extensions'].items():
        assert extension['mandatory_cases'], f'extension has no obligations: {name}'
        assert set(extension['mandatory_cases']) <= set(ids), f'unknown extension case: {name}'
        assert all(case['mandatory'] for case in cases
                   if case['id'] in extension['mandatory_cases']), f'nonmandatory extension case: {name}'
    return {(case['id'], order) for case in cases if case['mandatory']
            for order in manifest['byte_orders']}


def evaluate(manifest, results):
    required = expected(manifest)
    seen, failures = {}, []
    for result in results:
        key = (result['case'], result['byte_order'])
        if key not in required:
            failures.append(f'unexpected result {key}')
        if key in seen:
            failures.append(f'duplicate result {key}')
        seen[key] = result
        if result['status'] != 'PASS':
            failures.append(f'{key}: {result["status"]}: {result.get("detail", "")}')
    for key in sorted(required - seen.keys()):
        failures.append(f'{key}: MISSING mandatory result')
    return {'status': 'FAIL' if failures else 'PASS', 'required': len(required),
            'executed': len(seen), 'failures': failures, 'results': results}


def load_manifest(path):
    manifest = json.loads(path.read_text())
    expected(manifest)
    return manifest
