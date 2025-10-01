"""Public package API for the Chimera modulation demo library."""
from .config import FrameLayout, LDPCConfig, ProtocolConfig, SimulationConfig
from .pipeline import (
    DemodulationDiagnostics,
    DemodulationResult,
    EncodingResult,
    LDPCMatrices,
    SimulationResult,
    build_full_bitstream,
    create_ldpc_matrices,
    demodulate_and_decode,
    generate_modulated_signal,
    plot_demodulator_diagnostics,
    run_simulation,
)

__all__ = [
    "FrameLayout",
    "SimulationConfig",
    "ProtocolConfig",
    "LDPCConfig",
    "LDPCMatrices",
    "EncodingResult",
    "DemodulationDiagnostics",
    "DemodulationResult",
    "SimulationResult",
    "build_full_bitstream",
    "create_ldpc_matrices",
    "generate_modulated_signal",
    "demodulate_and_decode",
    "plot_demodulator_diagnostics",
    "run_simulation",
]
