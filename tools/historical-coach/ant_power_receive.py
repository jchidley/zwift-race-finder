"""Bounded native-Windows ANT+ Bicycle Power receiver; no ANT data TX.

Run using uv-managed Windows Python with pyusb==1.3.1. Uses the existing
Dynastream libusb0 driver; does not install/replace drivers. A single USBStick2
must be present and unused by other applications. Resets that stick, opens
one receive-only ANT channel, then closes/unassigns it and releases USB.
Receiver mode requires ANT_NETWORK_KEY from an authorized source; no key is
embedded or logged. --self-test and --open-only do not require that key.
--watchdog-test-host additionally sends positive synthetic power over UDP.
"""
import argparse
import json
import os
import socket
import time
from collections import deque
from datetime import UTC, datetime
from pathlib import Path


def frame(message_id, data):
    body = bytes([0xA4, len(data), message_id, *data])
    checksum = 0
    for value in body:
        checksum ^= value
    return body + bytes([checksum])


class Frames:
    def __init__(self):
        self.pending = bytearray()

    def feed(self, chunk):
        self.pending.extend(chunk)
        result = []
        while self.pending:
            if self.pending[0] != 0xA4:
                del self.pending[0]
                continue
            if len(self.pending) < 4:
                break
            length = self.pending[1] + 4
            if len(self.pending) < length:
                break
            packet = self.pending[:length]
            del self.pending[:length]
            checksum = 0
            for value in packet:
                checksum ^= value
            if checksum:
                raise ValueError("Invalid ANT frame checksum")
            result.append((packet[2], bytes(packet[3:-1])))
        return result


def decode_power(data):
    if len(data) != 8 or data[0] != 0x10:
        raise ValueError("Expected eight-byte standard power page")
    power = int.from_bytes(data[6:8], "little")
    return {
        "event_count": data[1],
        "cadence_rpm": None if data[3] == 0xFF else data[3],
        "accumulated_power": int.from_bytes(data[4:6], "little"),
        "power_w": None if power == 0xFFFF else power,
        "raw": data.hex(),
    }


def self_test():
    packet = frame(0x4D, [0, 0x3E])
    assert packet == bytes.fromhex("a4024d003ed5")
    parser = Frames()
    assert parser.feed(b"noise" + packet[:2]) == []
    assert parser.feed(packet[2:] + packet) == [
        (0x4D, bytes([0, 0x3E])), (0x4D, bytes([0, 0x3E]))
    ]
    corrupt = bytearray(packet)
    corrupt[-1] ^= 1
    try:
        Frames().feed(corrupt)
    except ValueError:
        pass
    else:
        raise AssertionError("Checksum corruption was accepted")
    sample = decode_power(bytes([0x10, 255, 255, 85, 0x34, 0x12, 180, 0]))
    assert sample["power_w"] == 180 and sample["cadence_rpm"] == 85
    assert sample["accumulated_power"] == 0x1234
    sample = decode_power(bytes([0x10, 0, 255, 255, 0, 0, 255, 255]))
    assert sample["power_w"] is None and sample["cadence_rpm"] is None
    sample = decode_power(bytes([0x10, 0, 255, 0, 0, 0, 0, 0]))
    assert sample["power_w"] == 0 and sample["cadence_rpm"] == 0
    print("PASS: framing, fragmentation, multiple frames, checksum rejection, power/cadence, zero/unavailable")


def network_key_from_environment():
    """Read an explicitly supplied key without embedding or logging it."""
    value = os.environ.get("ANT_NETWORK_KEY")
    if value is None:
        raise ValueError("Set ANT_NETWORK_KEY from an authorized source before receiver use")
    try:
        key = bytes.fromhex(value)
    except ValueError:
        raise ValueError("ANT_NETWORK_KEY must encode exactly eight bytes in hexadecimal") from None
    if len(key) != 8:
        raise ValueError("ANT_NETWORK_KEY must encode exactly eight bytes in hexadecimal")
    return key


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--self-test", action="store_true")
    parser.add_argument("--open-only", action="store_true")
    parser.add_argument("--seconds", type=int, default=12)
    parser.add_argument("--device-number", type=int, default=11234)
    parser.add_argument("--watchdog-test-host", help="Explicit controlled UDP watchdog exercise on this Gymnasticon host")
    args = parser.parse_args()
    if args.self_test:
        self_test()
        return
    if not 1 <= args.seconds <= 120 or not 1 <= args.device_number <= 65535:
        parser.error("seconds must be 1..120; device-number must be 1..65535")

    if args.watchdog_test_host and args.seconds < 27:
        parser.error("watchdog test requires at least 27 seconds")

    network_key = None
    if not args.open_only:
        try:
            network_key = network_key_from_environment()
        except ValueError as error:
            parser.error(str(error))

    import usb.backend.libusb0
    import usb.core
    import usb.util

    dll = Path(os.environ["WINDIR"]) / "System32" / "libusb0.dll"
    if not dll.is_file():
        raise RuntimeError("Existing 64-bit libusb0.dll not found; no driver changes attempted")
    backend = usb.backend.libusb0.get_backend(find_library=lambda _: str(dll))
    if backend is None:
        raise RuntimeError("Could not load existing libusb0 backend")
    devices = list(usb.core.find(find_all=True, idVendor=0x0FCF, idProduct=0x1008, backend=backend))
    if len(devices) != 1:
        raise RuntimeError(f"Expected exactly one Windows USBStick2, found {len(devices)}")
    device = devices[0]
    frames = Frames()
    queue = deque()
    assigned = opened = claimed = False
    samples = []
    producer = None
    producer_address = None

    def send(message_id, data):
        # Deliberately no broadcast/acknowledged/burst data messages.
        if message_id not in (0x4A, 0x4D, 0x46, 0x42, 0x51, 0x43, 0x45, 0x44, 0x4B, 0x4C, 0x41):
            raise ValueError("Refused non-receiver command")
        if message_id == 0x42 and data != [0, 0, 0]:
            raise ValueError("Only channel 0 receive mode on network 0 is allowed")
        packet = frame(message_id, data)
        if device.write(0x01, packet, timeout=1000) != len(packet):
            raise RuntimeError("Short USB write")

    def read(deadline):
        while time.monotonic() < deadline:
            if queue:
                return queue.popleft()
            try:
                chunk = device.read(0x81, 4096, timeout=250)
            except usb.core.USBError as error:
                # Legacy Windows libusb0 reports ETIMEDOUT without errno.
                if isinstance(error, usb.core.USBTimeoutError) or error.backend_error_code == -116:
                    continue
                raise
            queue.extend(frames.feed(chunk))
        return None

    def wait(message_id, request=None, seconds=3):
        deadline = time.monotonic() + seconds
        while (packet := read(deadline)) is not None:
            mid, data = packet
            if mid != message_id:
                continue
            if request is not None:
                if len(data) < 3 or data[0] != 0 or data[1] != request:
                    continue
                if data[2] != 0:
                    raise RuntimeError(f"ANT command {request:02x} rejected: {data[2]:02x}")
            return data
        raise RuntimeError(f"Timeout waiting for ANT message {message_id:02x}, request={request}")

    def command(mid, data):
        send(mid, data)
        wait(0x40, request=mid)

    try:
        # Existing active configuration only; do not reset/replace Windows drivers.
        usb.util.claim_interface(device, 0)
        claimed = True
        send(0x4A, [0])
        wait(0x6F)
        send(0x4D, [0, 0x3E])
        version = wait(0x3E).rstrip(b"\0").decode("ascii")
        send(0x4D, [0, 0x54])
        capabilities = wait(0x54)
        print(json.dumps({"usb": "0fcf:1008", "version": version, "capabilities": capabilities.hex()}), flush=True)
        if args.open_only:
            return
        # Key supplied before any USB access; never embed it in published source.
        command(0x46, [0, *network_key])
        command(0x42, [0, 0, 0])
        assigned = True
        command(0x51, [0, args.device_number & 255, args.device_number >> 8, 11, 1])
        command(0x43, [0, 8182 & 255, 8182 >> 8])
        command(0x45, [0, 57])
        command(0x44, [0, 12])
        command(0x4B, [0])
        opened = True
        print(json.dumps({"receiver": "ready", "device_number": args.device_number, "seconds": args.seconds}), flush=True)
        started = time.monotonic()
        deadline = started + args.seconds
        schedule = deque()
        exercise_started = not args.watchdog_test_host
        if args.watchdog_test_host:
            producer = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
            producer_address = (socket.gethostbyname(args.watchdog_test_host), 3000)
            zero = b'{"power":0,"cadence":0}'
            producer.sendto(zero, producer_address)
            deadline = started + 20  # Bounded acquisition before exercising the bot.
            schedule.extend([
                (0, zero),
                (3, b'{"power":180,"cadence":85}'),
                (4, b'{bad'),
                (5, b'{"power":999}'),
                (8, b'{"power":220,"cadence":90}'),
                (9, b'null'),
                (10, b'{"power":220,"cadence":255}'),
            ])
            schedule.extend((13 + i * 0.5, b'{"power":150,"cadence":80}') for i in range(9))
            schedule.extend([
                (22, b'{"power":120,"cadence":70}'),
                (23, zero),
                (26, zero),
            ])
        while time.monotonic() < deadline:
            while exercise_started and schedule and time.monotonic() >= started + schedule[0][0]:
                _, payload = schedule.popleft()
                sent_at = time.monotonic()
                producer.sendto(payload, producer_address)
                print(json.dumps({"command_sent": payload.decode("ascii"), "monotonic_s": sent_at}), flush=True)
            next_read = min(deadline, started + schedule[0][0]) if exercise_started and schedule else deadline
            packet = read(next_read)
            if packet is None:
                continue
            mid, data = packet
            if mid != 0x4E or len(data) < 9 or data[0] != 0 or data[1] != 0x10:
                continue
            sample = decode_power(data[1:9])
            sample["utc"] = datetime.now(UTC).isoformat()
            sample["monotonic_s"] = time.monotonic()
            samples.append(sample)
            print(json.dumps(sample), flush=True)
            if not exercise_started:
                started = time.monotonic()
                deadline = started + args.seconds
                exercise_started = True
                print(json.dumps({"exercise_started_after_acquisition": started}), flush=True)
        print(json.dumps({"summary": {
            "samples": len(samples),
            "values": sorted({(s["power_w"], s["cadence_rpm"]) for s in samples}, key=str),
            "duration_s": samples[-1]["monotonic_s"] - samples[0]["monotonic_s"] if len(samples) > 1 else 0,
        }}), flush=True)
        if not samples:
            raise RuntimeError("No standard power pages received from the selected sensor")
    finally:
        producer_error = None
        if producer is not None:
            try:
                producer.sendto(b'{"power":0,"cadence":0}', producer_address)
            except OSError as error:
                producer_error = error
            finally:
                producer.close()
        try:
            if opened:
                command(0x4C, [0])
                # Wait for the channel-closed event before unassigning.
                deadline = time.monotonic() + 2
                while (packet := read(deadline)) is not None:
                    if packet == (0x40, bytes([0, 1, 7])):
                        break
            if assigned:
                command(0x41, [0])
        finally:
            if claimed:
                usb.util.release_interface(device, 0)
            usb.util.dispose_resources(device)
            print("USB resources released; receiver closed.", flush=True)
        if producer_error is not None:
            raise producer_error


if __name__ == "__main__":
    main()
