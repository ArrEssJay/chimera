"""Configuration dataclasses for the Chimera modulation pipeline."""
from __future__ import annotations

from dataclasses import dataclass, field


@dataclass(frozen=True)
class FrameLayout:
    """Defines the symbol layout for a single frame in the protocol."""

    total_symbols: int = 128
    sync_symbols: int = 16
    target_id_symbols: int = 16
    command_type_symbols: int = 16
    data_payload_symbols: int = 64
    ecc_symbols: int = 16

    @property
    def message_bits(self) -> int:
        return self.data_payload_symbols * 2

    @property
    def ecc_bits(self) -> int:
        return self.ecc_symbols * 2

    @property
    def codeword_bits(self) -> int:
        return self.message_bits + self.ecc_bits

    @property
    def frame_bits(self) -> int:
        return self.total_symbols * 2


@dataclass(frozen=True)
class ProtocolConfig:
    """Encapsulates the Raman Whisper Modulation Protocol constants."""

    carrier_freq_hz: float = 12_000.0
    qpsk_symbol_rate: int = 16
    qpsk_bandwidth_hz: float = 20.0
    fsk_bit_rate: float = 1.0
    fsk_freq_zero_hz: float = 11_999.0
    fsk_freq_one_hz: float = 12_001.0
    command_opcode: int = 0x0001
    frame_layout: FrameLayout = field(default_factory=FrameLayout)
    sync_sequence_hex: str = "A5A5A5A5"
    target_id_hex: str = "DEADBEEF"
    max_frames: int = 256
    current_frame_shift: int = 16
    total_frames_shift: int = 24

    @property
    def fsk_freq_deviation_hz(self) -> float:
        return self.fsk_freq_one_hz - self.carrier_freq_hz


@dataclass(frozen=True)
class LDPCConfig:
    """Configuration for the LDPC code used in the pipeline."""

    dv: int = 2
    dc: int = 10
    seed: int | None = 42


@dataclass(frozen=True)
class SimulationConfig:
    """User-facing configuration for running an end-to-end simulation."""

    sample_rate: int = 48_000
    bit_depth: str = "FLOAT"
    snr_db: float = 3.0
    plaintext_source: str = (
        "This is a longer message demonstrating the protocol-compliant, rate-4/5 LDPC error "
        "correction. This signal simulates reception through a physically accurate AWGN channel "
        "where noise is added post-modulation. The decoder will now attempt to recover this exact "
        "message."
    )
    rng_seed: int | None = None
