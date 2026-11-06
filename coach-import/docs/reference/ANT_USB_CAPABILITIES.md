# ANT USB version and capability check

## Verified result

Queried the owner's newly attached sticks over direct USB on Debian.
These are device-reported capabilities, not an ANT+ profile interoperability,
radio-range, channel-load or Zwift test.

| Host | USB identity | Device version | Maximum channels | Maximum networks |
|---|---|---|---|---|
| WYK82Q | 0fcf:1009, Dynastream ANT USB-m Stick | AJK2.07RAF | 8 | 8 |
| WYKH32J | 0fcf:1008, Dynastream ANT USBStick2 | AJK1.04RAF | 8 | 3 |

Both report standard-option byte 0x00: none of the basic receive/transmit/message
restriction flags is set. Both are candidates for transmit and receive use.
Eight channels is the reported total, **not eight transmit plus eight receive**.
Networks are logical ANT network/key contexts, not a rider count or extra radios.
More networks does not increase the eight-channel limit.

For the current bridge, this does not require swapping sticks or buying hardware.
WYK82Q remains the first bridge host. Actual cycling-power/stride encoding,
scheduling, permissions for normal operation and receiver compatibility still
need qualification.

## Raw evidence

Version message ID 0x3e, capabilities message ID 0x54. Probe validated ANT framing
and XOR checksums before reporting responses.

- WYK82Q version payload:
  `41 4a 4b 32 2e 30 37 52 41 46 00`
- WYK82Q capabilities payload:
  `08 08 00 ba 36 00 df 04`
- WYKH32J version payload:
  `41 4a 4b 31 2e 30 34 52 41 46 00`
- WYKH32J capabilities payload:
  `08 03 00 ba 36 00`
- Both reset/startup messages (0x6f) had payload `00`.
- Both used bulk IN endpoint 0x81 and OUT endpoint 0x01.

Retain the complete capability bytes. The USB-m returns additional optional
fields; they have not all been decoded/qualified here. Do not infer unsupported
features from missing optional fields or turn advertised features into tested
behavior. The inspected OpenANT advanced-options enums have overlapping bit
definitions; do not blindly use them as authoritative interpretation.

## Access preparation and effects

The owner approved installing query prerequisites, elevated direct USB access,
temporary generic-driver detachment/restoration and device reset.

Installed, with no package upgrades/removals:
- WYK82Q: `libusb-1.0-0` version `2:1.0.28-1`.
- WYKH32J: `libusb-1.0-0-dev` version `2:1.0.28-1`; its compiler and
  libusb runtime were already installed.

No Python environment, OpenANT installation, udev rule, group change, Bluetooth
package/service, persistent driver change, reboot or network configuration
change was needed.

Earlier attempts through the generic serial endpoint on WYKH32J returned no
response. Serial settings were restored and verified. That failed transport
attempt is superseded by successful direct USB queries; it is not evidence
of a faulty stick.

## Reproducible diagnostic

Source: [scripts/ant_usb_info.c](../../scripts/ant_usb_info.c).
This is a diagnostic utility, not the application runtime or a bridge scaffold.

Build on a machine with a C compiler and libusb headers:

```bash
cc -std=c11 -Wall -Wextra -Werror -O2 scripts/ant_usb_info.c -lusb-1.0 -o /tmp/ant-usb-info
```

The executable takes exactly `1008` or `1009`. It requires explicit approved
USB access, resets the selected ANT device and temporarily detaches its driver.
Do not run against an in-use stick. It refuses multiple identical matching
devices rather than guessing. It does not set network keys, assign channels,
open/search channels or send radio data. Only reset and version/capability
request commands are allowed by its command guard.

Validation performed:
- Compiled on WYKH32J with `-Wall -Wextra -Werror`.
- Invalid-argument test returned exit status 2 without accessing USB.
- Live version/capability query succeeded on each stick with checked framing.
- Transferred the same x86-64 binary to WYK82Q; SHA-256 verified before execution:
  `b21666997b94720b2e1a1cb2c0f5cb4202eaa44257ced37ba2aaf968a121709e`.
- Program verified reattachment; separate sysfs reads confirmed both original
  `usb_serial_simple` driver bindings after execution.

Temporary query artifacts were left in:
- WYKH32J: `/tmp/ant-usb-query.LSmkwe/` (source and binary).
- WYK82Q: `/tmp/ant-usb-query.JwDkQO/` (binary).

No ANT channel was assigned or opened, no sensor search was run and no
sensor-data broadcast was made. USB communication and ANT device resets
**were** performed; this was not a purely passive inventory check.
