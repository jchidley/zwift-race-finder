"""Radio-free regression checks for explicitly supplied ANT network keys."""
import contextlib
import io
import runpy
import sys
from pathlib import Path
from unittest.mock import patch


def main():
    receiver = Path(__file__).with_name("ant_power_receive.py")
    module = runpy.run_path(str(receiver))
    parse = module["network_key_from_environment"]
    for value in (None, "", "not hex", "00" * 7, "00" * 9):
        env = {} if value is None else {"ANT_NETWORK_KEY": value}
        with patch.dict("os.environ", env, clear=True):
            try:
                parse()
            except ValueError as error:
                assert "ANT_NETWORK_KEY" in str(error)
                assert str(error) in (
                    "Set ANT_NETWORK_KEY from an authorized source before receiver use",
                    "ANT_NETWORK_KEY must encode exactly eight bytes in hexadecimal",
                )
            else:
                raise AssertionError("Invalid key accepted")

    with patch.dict("os.environ", {"ANT_NETWORK_KEY": "00" * 8}, clear=True):
        assert parse() == bytes(8)

    # No key must fail before platform-specific USB imports or hardware access.
    for env in ({}, {"ANT_NETWORK_KEY": "not hex"}):
        with (
            patch.dict("os.environ", env, clear=True),
            patch.object(sys, "argv", ["ant_power_receive.py"]),
            contextlib.redirect_stderr(io.StringIO()),
        ):
            try:
                module["main"]()
            except SystemExit as error:
                assert error.code == 2
            else:
                raise AssertionError("Invalid key did not stop receiver startup")

    print("PASS: synthetic key parsing, value-free errors and rejection before USB access")


if __name__ == "__main__":
    main()
