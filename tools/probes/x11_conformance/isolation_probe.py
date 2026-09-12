"""Fixture child; endpoints are fabricated by test_isolation, never ambient."""
import argparse
import json
import os
import socket

from isolation import validate_entry


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('activation', type=int)
    parser.add_argument('--socket')
    parser.add_argument('--delegated', type=int)
    args = parser.parse_args()
    delegated = validate_entry(args.activation)
    result = {'validated': True, 'environment': dict(os.environ), 'reachable': False}
    if args.socket:
        with socket.socket(socket.AF_UNIX) as peer:
            peer.settimeout(1)
            try:
                peer.connect(args.socket)
                result['reachable'] = True
            except OSError as error:
                result['connect_errno'] = error.errno
    if args.delegated is not None:
        if args.delegated not in delegated:
            raise RuntimeError('capability not delegated')
        os.write(args.delegated, b'authorized instance capability')
        result['delegated'] = True
    print(json.dumps(result))


if __name__ == '__main__':
    main()
