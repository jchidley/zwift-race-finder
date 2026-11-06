# Historical device diagnostics

These files support the dated reports in [docs/reference](../../docs/README.md). They are not the coaching runtime, maintained Fitness Bridge components or permission to operate a device.

| File | Role and effects |
|---|---|
| ant_usb_info.c | USB capability diagnostic; detaches/restores a driver and resets the selected stick. Requires separately approved device access |
| ant_power_receive.py | Windows ANT receiver; resets the stick and opens a receive channel. `--open-only` resets/queries without opening the channel. `--watchdog-test-host` also sends **positive synthetic power over UDP**; it is not a passive test |
| gymnasticon_bot_input.cjs | Original input validation/expiry helper, no I/O |
| patch_gymnasticon_bot.cjs | Generates a candidate from an explicitly supplied compiled Gymnasticon 1.4.0 bot; writes a new file, never deploys it |
| test_gymnasticon_bot_input.cjs | Radio-free synthetic helper tests |
| test_gymnasticon_bot_adapter.cjs | Radio-free adapter tests; executes an explicitly supplied candidate with stubbed dependencies |
| test_ant_receiver_config.py | Radio-free synthetic configuration tests for the receiver's publication-time key removal |

## Publication-time changes — 2026-10-31

The receiver originally embedded an ANT network key. The published version requires `ANT_NETWORK_KEY` as hexadecimal encoding of eight bytes, supplied from an authorized source in the consuming process environment. It never embeds or logs the value and rejects missing/invalid input before USB imports or access. No automatic credential retrieval or redistribution right is assumed. Do not put keys in command arguments, Git or plaintext configuration.

The parser `--self-test` and USB `--open-only` path do not require the key. Only the former is radio-free. This source change has not been deployed or hardware-qualified; the historical bench receipts describe the earlier script.

The historical Gymnasticon UDP endpoint listens on `0.0.0.0:3000` without authentication. Do not expose it to an untrusted network. Generated bot candidates and device recordings belong outside this checkout.

Radio-free checks, using existing runtimes only:

```bash
uv run --offline --no-project --managed-python python tools/historical-coach/ant_power_receive.py --self-test
uv run --offline --no-project --managed-python python tools/historical-coach/test_ant_receiver_config.py
node --check tools/historical-coach/patch_gymnasticon_bot.cjs
node tools/historical-coach/test_gymnasticon_bot_input.cjs
```

These are diagnostic regressions, not tests of a coaching application or the new Rust bridge.

## Upstream attribution

The patch generator contains small match anchors from [ptx2/gymnasticon](https://github.com/ptx2/gymnasticon), copyright 2020 ptx2, MIT licensed. They target the inspected installed compiled 1.4.0 bot. The separate local source reference at `e142f539c518d6d4ced3abdd587a825b918a9367` is 1.4.1, not the installed artifact.

[LICENSE-GYMNASTICON](LICENSE-GYMNASTICON) retains the upstream notice. The complete upstream bot is not included; a generated derivative retains upstream obligations and must not be published under only this project's licence.
