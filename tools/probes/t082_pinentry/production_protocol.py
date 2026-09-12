"""Bounded Assuan reader for a dummy-only, uninstrumented candidate."""
import os
import selectors
import subprocess
import time

class ProtocolFailure(Exception):
    pass

def encode_argument(text):
    # Encode bytes, not Unicode code points. Keep generated command lines ASCII.
    return "".join(chr(b) if 32 <= b < 127 and b != 37 else f"%{b:02X}"
                   for b in text.encode("utf-8")).encode("ascii")

def decode_data(data):
    result = bytearray()
    index = 0
    while index < len(data):
        value = data[index]
        if value == 37:
            pair = data[index + 1:index + 3]
            if len(pair) != 2 or any(c not in b"0123456789abcdefABCDEF" for c in pair):
                raise ProtocolFailure("invalid_percent_escape")
            result.append(int(pair, 16))
            index += 3
        else:
            if value in (10, 13):
                raise ProtocolFailure("unescaped_line_break")
            result.append(value)
            index += 1
    return bytes(result)

def stop(child):
    if child.poll() is None:
        child.terminate()
        try:
            child.wait(timeout=1)
        except subprocess.TimeoutExpired:
            child.kill()
            child.wait(timeout=2)

class Exchange:
    def __init__(self, child, deadline):
        self.child = child
        self.deadline = deadline
        self.selector = selectors.DefaultSelector()
        self.pending = b""
        self.stderr_present = False
        self.byte_budget = 65_536
        self.line_budget = 512
        self.stdout_eof = False
        for stream, kind in ((child.stdout, "stdout"), (child.stderr, "stderr")):
            os.set_blocking(stream.fileno(), False)
            self.selector.register(stream, selectors.EVENT_READ, kind)
        os.set_blocking(child.stdin.fileno(), False)

    def close(self):
        self.selector.close()

    def pump(self, deadline):
        if time.monotonic() >= min(deadline, self.deadline):
            raise ProtocolFailure("deadline")
        for key, _ in self.selector.select(min(.05, max(0, deadline - time.monotonic()))):
            chunk = os.read(key.fileobj.fileno(), 4096)
            if not chunk:
                self.selector.unregister(key.fileobj)
                self.stdout_eof |= key.data == "stdout"
                continue
            self.byte_budget -= len(chunk)
            if self.byte_budget < 0:
                raise ProtocolFailure("output_budget_exhausted")
            if key.data == "stderr":
                self.stderr_present = True
            else:
                self.pending += chunk
                if b"\n" not in self.pending and len(self.pending) > 1000:
                    raise ProtocolFailure("line_budget_exceeded")

    def line(self, deadline):
        while b"\n" not in self.pending:
            if self.stdout_eof:
                raise ProtocolFailure("unexpected_eof")
            self.pump(deadline)
        raw, self.pending = self.pending.split(b"\n", 1)
        self.line_budget -= 1
        if len(raw) + 1 > 1000 or self.line_budget < 0:
            raise ProtocolFailure("line_budget_exceeded")
        return raw.removesuffix(b"\r")

    def send(self, command):
        if not command or len(command) + 1 > 1000 or b"\n" in command or b"\r" in command:
            raise ProtocolFailure("invalid_command")
        try:
            if os.write(self.child.stdin.fileno(), command + b"\n") != len(command) + 1:
                raise ProtocolFailure("partial_command_write")
        except (BrokenPipeError, BlockingIOError):
            raise ProtocolFailure("request_pipe_unavailable") from None

    def response(self, seconds):
        deadline = min(self.deadline, time.monotonic() + seconds)
        data, data_seen = bytearray(), False
        while True:
            line = self.line(deadline)
            if not line or line.startswith(b"#") or line.startswith(b"S "):
                continue  # Never retain arbitrary comments/status text.
            if line.startswith(b"D "):
                data_seen = True
                data.extend(decode_data(line[2:]))
                if len(data) > 4096:
                    raise ProtocolFailure("data_budget_exceeded")
                continue
            if line == b"OK" or line.startswith(b"OK "):
                kind, code = "ok", None
            elif line.startswith(b"ERR "):
                token = line[4:].split(b" ", 1)[0]
                if not token.isdigit() or len(token) > 10:
                    raise ProtocolFailure("invalid_error_code")
                kind, code = "error", int(token)
            else:
                raise ProtocolFailure("unexpected_record")
            if self.pending:
                raise ProtocolFailure("unsolicited_trailing_record")
            return kind, code, bytes(data), data_seen

    def ok(self, command=None, seconds=5):
        if command is not None:
            self.send(command)
        kind, _, _, data_seen = self.response(seconds)
        if kind != "ok" or data_seen:
            raise ProtocolFailure("expected_plain_ok")

    def finish(self, seconds=3):
        self.ok(b"BYE", seconds)
        self.child.stdin.close()
        deadline = min(self.deadline, time.monotonic() + seconds)
        while self.child.poll() is None or self.selector.get_map():
            self.pump(deadline)
            if self.pending:
                raise ProtocolFailure("trailing_output_after_bye")
        if self.child.returncode != 0:
            raise ProtocolFailure("nonzero_exit")
