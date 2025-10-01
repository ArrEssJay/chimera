"""Utility helpers for Chimera modulation pipeline."""
from __future__ import annotations

from dataclasses import dataclass, field
from typing import Iterable, List, Sequence

import numpy as np


def int_to_bitstream(value: int, num_bits: int) -> np.ndarray:
    """Convert an integer into a fixed-length big-endian bit array."""
    if num_bits <= 0:
        raise ValueError("num_bits must be positive")
    if value >= 2 ** num_bits:
        raise ValueError(f"Value {value} won't fit into {num_bits} bits")
    return np.array(list(format(value, f"0{num_bits}b")), dtype=np.uint8)


def hex_to_bitstream(hex_string: str, expected_bits: int) -> np.ndarray:
    """Convert a hexadecimal string into a bit array of the expected width."""
    if expected_bits % 8 != 0:
        raise ValueError("expected_bits must be a multiple of 8 for hex conversion")
    padded = hex_string.zfill(expected_bits // 4)
    raw = bytes.fromhex(padded)
    return np.unpackbits(np.frombuffer(raw, dtype=np.uint8))


def string_to_bitstream(text: str) -> np.ndarray:
    """Encode a UTF-8 string into a bit array."""
    return np.unpackbits(np.frombuffer(text.encode("utf-8"), dtype=np.uint8))


def bits_to_str(bits: Sequence[int], limit: int = 32) -> str:
    """Render a bit sequence as a compact printable string."""
    slice_end = limit if limit is not None else len(bits)
    return "".join(str(int(b)) for b in bits[:slice_end])


def chunk_array(array: np.ndarray, chunk_size: int) -> List[np.ndarray]:
    """Split an array into equal-sized chunks (the last chunk may be shorter)."""
    if chunk_size <= 0:
        raise ValueError("chunk_size must be positive")
    return [array[i : i + chunk_size] for i in range(0, len(array), chunk_size)]


@dataclass
class LogCollector:
    """Small helper to capture verbose trace messages."""

    entries: List[str] = field(default_factory=list)

    def log(self, message: str) -> None:
        self.entries.append(message)

    def extend(self, lines: Iterable[str]) -> None:
        for line in lines:
            self.log(line)

    def emit(self, message: str, verbose: bool = False) -> None:
        self.log(message)
        if verbose:
            print(message)
