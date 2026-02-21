#!/usr/bin/env python3
"""
Verify whether an NRO appears to export/contain a symbol.

Usage:
  python verify_nro_symbol.py "E:\\path\\to\\plugin.nro"
  python verify_nro_symbol.py "E:\\path\\to\\plugin.nro" --symbol ssbusync_external_disabler
"""

from __future__ import annotations

import argparse
import pathlib
import shutil
import subprocess
import sys
from typing import List, Tuple


def run_tool(cmd: List[str]) -> Tuple[bool, str]:
    try:
        proc = subprocess.run(
            cmd,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            check=False,
        )
    except OSError as exc:
        return False, f"{cmd[0]} failed to execute: {exc}"

    if proc.returncode != 0:
        msg = proc.stderr.strip() or proc.stdout.strip() or f"exit code {proc.returncode}"
        return False, msg

    return True, proc.stdout


def symbol_in_tools(path: pathlib.Path, symbol: str) -> Tuple[bool, List[str]]:
    checks = [
        ["llvm-nm", "-C", "-g", str(path)],
        ["nm", "-C", "-g", str(path)],
        ["aarch64-none-elf-nm", "-C", "-g", str(path)],
        ["llvm-readelf", "-Ws", str(path)],
        ["readelf", "-Ws", str(path)],
    ]

    logs: List[str] = []
    found = False

    for cmd in checks:
        tool = cmd[0]
        if shutil.which(tool) is None:
            logs.append(f"- {tool}: not found")
            continue

        ok, out = run_tool(cmd)
        if not ok:
            logs.append(f"- {' '.join(cmd)}: failed ({out})")
            continue

        if symbol in out:
            logs.append(f"- {' '.join(cmd)}: FOUND")
            found = True
        else:
            logs.append(f"- {' '.join(cmd)}: not found")

    return found, logs


def symbol_in_binary(path: pathlib.Path, symbol: str) -> bool:
    blob = path.read_bytes()
    sym = symbol.encode("ascii", "ignore")
    return (sym in blob) or ((sym + b"\x00") in blob)


def main() -> int:
    parser = argparse.ArgumentParser(description="Verify symbol presence in a Skyline NRO file.")
    parser.add_argument("nro_path", help="Absolute path to .nro")
    parser.add_argument(
        "--symbol",
        default="ssbusync_external_disabler",
        help="Symbol name to verify (default: ssbusync_external_disabler)",
    )
    args = parser.parse_args()

    path = pathlib.Path(args.nro_path)
    symbol = args.symbol.strip()

    if not path.is_absolute():
        print(f"[warn] path is not absolute: {path}")
    if not path.exists():
        print(f"[error] file does not exist: {path}")
        return 2
    if path.suffix.lower() != ".nro":
        print(f"[warn] file extension is not .nro: {path.name}")

    print(f"[info] NRO: {path}")
    print(f"[info] symbol: {symbol}")

    tool_found, logs = symbol_in_tools(path, symbol)
    for line in logs:
        print(line)

    binary_found = symbol_in_binary(path, symbol)
    print(f"- raw binary scan: {'FOUND' if binary_found else 'not found'}")

    if tool_found or binary_found:
        print(f"[ok] symbol appears present: {symbol}")
        return 0

    print(f"[fail] symbol not detected: {symbol}")
    return 1


if __name__ == "__main__":
    sys.exit(main())
