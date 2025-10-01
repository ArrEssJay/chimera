"""Smoke tests for Chimera utility helpers."""
import numpy as np

from chimera.utils import bits_to_str, hex_to_bitstream, int_to_bitstream, string_to_bitstream


def test_string_roundtrip():
    text = "Chimera"
    bits = string_to_bitstream(text)
    assert bits.dtype == np.uint8
    assert len(bits) == len(text) * 8
    recovered = np.packbits(bits).tobytes().decode("utf-8")
    assert recovered == text


def test_int_to_bitstream_fixed_width():
    value = 0xAB
    bits = int_to_bitstream(value, 12)
    assert bits.tolist() == [0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1]


def test_hex_to_bitstream_expected_width():
    bits = hex_to_bitstream("A5A5", 16)
    assert bits.shape == (16,)
    assert bits[:8].tolist() == [1, 0, 1, 0, 0, 1, 0, 1]
    assert bits_to_str(bits, limit=16) == "1010010110100101"
