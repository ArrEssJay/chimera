"""Functional pipeline for the Chimera Raman Whisper modulation demo."""
from __future__ import annotations

from dataclasses import dataclass, field
from typing import Dict, Iterable, List, Optional, Sequence

import math

import numpy as np

try:  # Optional dependency: SciPy signal processing helpers
    from scipy.signal import butter, filtfilt, lfilter
except ImportError:  # pragma: no cover - handled at runtime when the feature is used
    butter = filtfilt = lfilter = None  # type: ignore

try:  # Optional dependency: LDPC construction/decoding
    from pyldpc import decode as ldpc_decode
    from pyldpc import make_ldpc
except ImportError:  # pragma: no cover - handled at runtime when the feature is used
    make_ldpc = ldpc_decode = None  # type: ignore

try:  # Optional dependency: Numba for acceleration
    import numba
except ImportError:  # pragma: no cover - numba is optional
    numba = None

from .config import LDPCConfig, ProtocolConfig, SimulationConfig
from .utils import LogCollector, bits_to_str, hex_to_bitstream, int_to_bitstream, string_to_bitstream


@dataclass
class LDPCMatrices:
    """Container for LDPC parity-check and generator matrices."""

    H: np.ndarray
    G: np.ndarray
    message_bits: int
    codeword_bits: int


@dataclass
class EncodingResult:
    """Artifacts produced by the modulation stage."""

    noisy_signal: np.ndarray
    clean_signal: np.ndarray
    qpsk_bitstream: np.ndarray
    payload_bits: np.ndarray
    qpsk_phase_map: np.ndarray
    total_frames: int
    duration_seconds: float
    num_samples: int
    logs: List[str] = field(default_factory=list)


@dataclass
class DemodulationDiagnostics:
    """Intermediate arrays captured during demodulation for plotting/debugging."""

    received_symbols_i: np.ndarray
    received_symbols_q: np.ndarray
    timing_error: np.ndarray
    nco_freq_offset: np.ndarray


@dataclass
class DemodulationResult:
    """Outputs produced by the demodulation + decoding stage."""

    demodulated_bitstream: np.ndarray
    decoded_bitstream: np.ndarray
    recovered_message: str
    pre_fec_errors: int
    pre_fec_ber: float
    post_fec_errors: int
    post_fec_ber: float
    diagnostics: DemodulationDiagnostics
    logs: List[str] = field(default_factory=list)


@dataclass
class SimulationResult:
    """Aggregated outputs across encoding and decoding phases."""

    encoding: EncodingResult
    demodulation: DemodulationResult
    matrices: LDPCMatrices
    logs: List[str]


def create_ldpc_matrices(protocol: ProtocolConfig, config: LDPCConfig, *, verbose: bool = False) -> LDPCMatrices:
    """Create LDPC parity-check and generator matrices for the given protocol layout."""

    if make_ldpc is None:
        raise ImportError("pyldpc is required to construct LDPC matrices. Install it via `pip install pyldpc`.")

    layout = protocol.frame_layout
    k_message_bits = layout.message_bits
    n_codeword_bits = layout.codeword_bits

    if verbose:
        print(
            "Generating LDPC matrices with parameters: "
            f"k={k_message_bits}, n={n_codeword_bits}, dv={config.dv}, dc={config.dc}, seed={config.seed}"
        )

    H, _ = make_ldpc(
        n_codeword_bits,
        config.dv,
        config.dc,
        systematic=True,
        seed=config.seed,
    )

    P = H[:, :k_message_bits].T
    I_k = np.identity(k_message_bits, dtype=int)
    G = np.concatenate((I_k, P), axis=1, dtype=int) % 2

    if G.shape != (k_message_bits, n_codeword_bits):
        raise ValueError(
            "Generator matrix construction failed: "
            f"shape={G.shape}, expected=({k_message_bits}, {n_codeword_bits})"
        )

    return LDPCMatrices(H=H.astype(np.uint8), G=G.astype(np.uint8), message_bits=k_message_bits, codeword_bits=n_codeword_bits)


def build_full_bitstream(
    payload_bits: np.ndarray,
    protocol: ProtocolConfig,
    matrices: LDPCMatrices,
    *,
    verbose: bool = False,
    logger: Optional[LogCollector] = None,
) -> tuple[np.ndarray, int]:
    """Construct the full framed bitstream ready for transmission."""

    if logger is None:
        logger = LogCollector()

    layout = protocol.frame_layout
    total_frames = math.ceil(len(payload_bits) / matrices.message_bits) if len(payload_bits) else 1

    if total_frames > protocol.max_frames:
        raise ValueError(f"Message too long for protocol (requires {total_frames} frames, max is {protocol.max_frames}).")

    logger.emit(f"Payload requires {total_frames} frame(s).", verbose)

    sync_bits = hex_to_bitstream(protocol.sync_sequence_hex, layout.sync_symbols * 2)
    target_id_bits = hex_to_bitstream(protocol.target_id_hex, layout.target_id_symbols * 2)

    frames: List[np.ndarray] = []
    for frame_idx in range(total_frames):
        command_type = protocol.command_opcode | (frame_idx << protocol.current_frame_shift) | (total_frames << protocol.total_frames_shift)
        command_bits = int_to_bitstream(command_type, layout.command_type_symbols * 2)

        start, end = frame_idx * matrices.message_bits, (frame_idx + 1) * matrices.message_bits
        message_chunk = payload_bits[start:end]
        if len(message_chunk) < matrices.message_bits:
            message_chunk = np.pad(message_chunk, (0, matrices.message_bits - len(message_chunk)))
        codeword = (message_chunk @ matrices.G % 2).astype(np.uint8)
        payload, ecc = codeword[: matrices.message_bits], codeword[matrices.message_bits :]

        if frame_idx < 3:
            logger.emit(
                f"[TX] Frame {frame_idx + 1}/{total_frames}\n"
                f"  > Command : 0x{command_type:08X}\n"
                f"  > Payload : {bits_to_str(payload)}...\n"
                f"  > ECC     : {bits_to_str(ecc)}",
                verbose,
            )

        frame_bits = np.concatenate((sync_bits, target_id_bits, command_bits, payload, ecc))
        frames.append(frame_bits)

    full_stream = np.concatenate(frames) if frames else np.array([], dtype=np.uint8)
    return full_stream, total_frames


def _ensure_signal_backend() -> None:
    if butter is None or filtfilt is None or lfilter is None:
        raise ImportError("scipy is required for modulation/demodulation. Install it via `pip install scipy`.")


def generate_modulated_signal(
    sim_config: SimulationConfig,
    protocol: ProtocolConfig,
    matrices: LDPCMatrices,
    *,
    plaintext: Optional[str] = None,
    rng: Optional[np.random.Generator] = None,
    verbose: bool = False,
) -> EncodingResult:
    """Encode a plaintext string into a noisy modulated audio stream."""

    _ensure_signal_backend()

    logger = LogCollector()

    if plaintext is None:
        plaintext = sim_config.plaintext_source

    payload_bits = string_to_bitstream(plaintext)
    logger.emit(f"Source plaintext length: {len(plaintext)} characters ({len(payload_bits)} bits).", verbose)

    qpsk_bitstream, total_frames = build_full_bitstream(payload_bits, protocol, matrices, verbose=verbose, logger=logger)

    layout = protocol.frame_layout
    samples_per_symbol = max(1, int(round(sim_config.sample_rate / protocol.qpsk_symbol_rate)))
    samples_per_bit = max(1, int(round(sim_config.sample_rate / protocol.fsk_bit_rate)))
    duration_seconds = max(1, total_frames * layout.total_symbols / protocol.qpsk_symbol_rate)
    num_samples = int(duration_seconds * sim_config.sample_rate)

    logger.emit(f"Calculated signal duration: {duration_seconds:.2f} s with {num_samples} samples.", verbose)

    fsk_bits_needed = int(math.ceil(duration_seconds * protocol.fsk_bit_rate))
    if len(payload_bits) == 0:
        fsk_source_bits = np.zeros(fsk_bits_needed, dtype=np.uint8)
    else:
        repeats = math.ceil(fsk_bits_needed / len(payload_bits))
        fsk_source_bits = np.tile(payload_bits, repeats)[:fsk_bits_needed]

    fsk_freq_offsets = (fsk_source_bits * 2 - 1) * protocol.fsk_freq_deviation_hz
    base_freq = protocol.carrier_freq_hz + np.repeat(fsk_freq_offsets, samples_per_bit)[:num_samples]

    qpsk_pairs = qpsk_bitstream.reshape(-1, 2)
    qpsk_symbol_map = {
        (0, 0): 0,
        (0, 1): 1,
        (1, 1): 2,
        (1, 0): 3,
    }
    qpsk_symbols = np.array([qpsk_symbol_map[tuple(pair)] for pair in qpsk_pairs])
    qpsk_phase_map = (np.array([1, 0, 2, 3]) * np.pi / 2) + (np.pi / 4)
    qpsk_phases = qpsk_phase_map[qpsk_symbols]
    phase_qpsk_raw = np.repeat(qpsk_phases, samples_per_symbol)
    if len(phase_qpsk_raw) < num_samples:
        pad = num_samples - len(phase_qpsk_raw)
        phase_qpsk_raw = np.pad(phase_qpsk_raw, (0, pad))
    else:
        phase_qpsk_raw = phase_qpsk_raw[:num_samples]

    b, a = butter(4, protocol.qpsk_bandwidth_hz / (0.5 * sim_config.sample_rate), "low")
    phase_qpsk_smoothed = np.arctan2(
        filtfilt(b, a, np.sin(phase_qpsk_raw)),
        filtfilt(b, a, np.cos(phase_qpsk_raw)),
    )

    phase_fsk = 2 * np.pi * np.cumsum(base_freq) / sim_config.sample_rate
    clean_signal = np.sin(phase_fsk + phase_qpsk_smoothed)

    logger.emit(f"Simulating AWGN channel with SNR = {sim_config.snr_db} dB.", verbose)

    signal_power = np.mean(clean_signal**2)
    snr_linear = 10 ** (sim_config.snr_db / 10)
    noise_power = signal_power / snr_linear if snr_linear > 0 else 0.0

    rng = rng or np.random.default_rng(sim_config.rng_seed)
    noise = rng.normal(0.0, math.sqrt(noise_power), num_samples)
    noisy_signal = clean_signal + noise

    logger.emit("Modulation complete; generated noisy signal for transmission.", verbose)

    return EncodingResult(
        noisy_signal=noisy_signal,
        clean_signal=clean_signal,
        qpsk_bitstream=qpsk_bitstream,
        payload_bits=payload_bits,
        qpsk_phase_map=qpsk_phase_map,
        total_frames=total_frames,
        duration_seconds=duration_seconds,
        num_samples=num_samples,
        logs=logger.entries,
    )


def _timing_and_carrier_recovery_impl(
    baseband_signal: np.ndarray,
    samples_per_symbol: float,
    sample_rate: int,
    kp_carrier: float = 0.000005,
    ki_carrier: float = 0.000005**2 / 4.0,
    kp_timing: float = 0.0001,
    ki_timing: float = 0.000001,
) -> tuple[np.ndarray, np.ndarray, np.ndarray]:
    """Pure Python implementation of the joint timing/carrier loop."""

    nco_phase = 0.0
    nco_freq_rad = 0.0
    integrator_carrier = 0.0

    timing_error = 0.0
    integrator_timing = 0.0

    out_symbols: List[complex] = []
    out_nco_freq: List[float] = []
    out_timing_error: List[float] = []

    i_in = samples_per_symbol
    signal_len = len(baseband_signal)

    while i_in < signal_len - samples_per_symbol - 1:
        idx = int(np.floor(i_in))
        frac = i_in - idx
        if idx < 1 or idx + 1 >= signal_len:
            break

        mid_interp = baseband_signal[idx] + frac * (baseband_signal[idx + 1] - baseband_signal[idx])

        half_idx = int(np.floor(i_in - samples_per_symbol / 2))
        half_frac = i_in - samples_per_symbol / 2 - half_idx
        if half_idx < 1 or half_idx + 1 >= signal_len:
            break
        half_interp = baseband_signal[half_idx] + half_frac * (baseband_signal[half_idx + 1] - baseband_signal[half_idx])

        nco_output = np.exp(-1j * nco_phase)
        corrected_mid = mid_interp * nco_output
        corrected_half = half_interp * nco_output

        if out_symbols:
            prev_mid = out_symbols[-1]
            timing_error = np.real(corrected_half) * (np.real(corrected_mid) - np.real(prev_mid)) + np.imag(corrected_half) * (
                np.imag(corrected_mid) - np.imag(prev_mid)
            )

        integrator_timing += ki_timing * timing_error
        i_in += samples_per_symbol - (kp_timing * timing_error + integrator_timing)

        i_corrected = np.real(corrected_mid)
        q_corrected = np.imag(corrected_mid)
        phase_error = np.arctan2(q_corrected, i_corrected)

        integrator_carrier += ki_carrier * phase_error
        nco_freq_rad += kp_carrier * phase_error + integrator_carrier
        nco_phase += nco_freq_rad

        out_symbols.append(corrected_mid)
        out_nco_freq.append(nco_freq_rad * sample_rate / (2 * np.pi))
        out_timing_error.append(timing_error)

    return np.array(out_symbols), np.array(out_timing_error), np.array(out_nco_freq)


def timing_and_carrier_recovery(
    baseband_signal: np.ndarray,
    samples_per_symbol: float,
    sample_rate: int,
) -> tuple[np.ndarray, np.ndarray, np.ndarray]:
    """Joint symbol timing and carrier recovery loop (Numba accelerated when available)."""

    if numba is not None:
        compiled = numba.jit(nopython=True)(_timing_and_carrier_recovery_impl)  # type: ignore[arg-type]
        return compiled(baseband_signal, samples_per_symbol, sample_rate)
    return _timing_and_carrier_recovery_impl(baseband_signal, samples_per_symbol, sample_rate)


def demodulate_and_decode(
    encoding: EncodingResult,
    matrices: LDPCMatrices,
    sim_config: SimulationConfig,
    protocol: ProtocolConfig,
    *,
    plot: bool = False,
    verbose: bool = False,
) -> DemodulationResult:
    """Demodulate the noisy signal and attempt to recover the original payload."""

    if ldpc_decode is None:
        raise ImportError("pyldpc is required for decoding. Install it via `pip install pyldpc`.")
    _ensure_signal_backend()

    logger = LogCollector()

    noisy_signal = encoding.noisy_signal
    t = np.arange(len(noisy_signal)) / sim_config.sample_rate
    complex_baseband_raw = noisy_signal * np.exp(-1j * 2 * np.pi * protocol.carrier_freq_hz * t)

    samples_per_symbol = sim_config.sample_rate / protocol.qpsk_symbol_rate
    num_taps = 101
    beta = 0.35
    t_rrc = np.arange(-num_taps // 2, num_taps // 2 + 1) / samples_per_symbol
    rrc_filter_taps = np.sinc(t_rrc) * np.cos(np.pi * beta * t_rrc) / (1 - (2 * beta * t_rrc) ** 2)
    rrc_filter_taps[np.isnan(rrc_filter_taps)] = 0  # guard singularities
    rrc_filter_taps /= np.linalg.norm(rrc_filter_taps)

    complex_baseband_filtered = lfilter(rrc_filter_taps, 1, complex_baseband_raw)

    logger.emit("Performing timing and carrier recovery...", verbose)
    received_symbols, timing_error, nco_freq_offset = timing_and_carrier_recovery(
        complex_baseband_filtered,
        samples_per_symbol,
        sim_config.sample_rate,
    )

    diagnostics = DemodulationDiagnostics(
        received_symbols_i=np.real(received_symbols),
        received_symbols_q=np.imag(received_symbols),
        timing_error=timing_error,
        nco_freq_offset=nco_freq_offset,
    )

    if plot:
        plot_demodulator_diagnostics(diagnostics, encoding.qpsk_phase_map, protocol, sim_config.snr_db)

    received_phases = np.angle(received_symbols)
    qpsk_reverse_bit_map: Dict[int, List[int]] = {
        0: [0, 1],
        1: [0, 0],
        2: [1, 1],
        3: [1, 0],
    }

    demodulated_bits: List[int] = []
    for phase in received_phases:
        phase_diff = np.angle(np.exp(1j * (phase - encoding.qpsk_phase_map)))
        symbol_index = int(np.argmin(np.abs(phase_diff)))
        demodulated_bits.extend(qpsk_reverse_bit_map[symbol_index])

    demodulated_bitstream = np.array(demodulated_bits, dtype=np.uint8)
    if len(demodulated_bitstream) < len(encoding.qpsk_bitstream):
        padding = len(encoding.qpsk_bitstream) - len(demodulated_bitstream)
        demodulated_bitstream = np.pad(demodulated_bitstream, (0, padding), 'constant')
    else:
        demodulated_bitstream = demodulated_bitstream[:len(encoding.qpsk_bitstream)]

    pre_fec_errors = int(np.sum(encoding.qpsk_bitstream != demodulated_bitstream))
    pre_fec_ber = pre_fec_errors / len(encoding.qpsk_bitstream)
    logger.emit(f"Pre-FEC BER: {pre_fec_ber:.6f} ({pre_fec_errors} errors).", verbose)

    sync_bits = hex_to_bitstream(protocol.sync_sequence_hex, protocol.frame_layout.sync_symbols * 2)
    frame_size_bits = protocol.frame_layout.frame_bits

    sync_location = -1
    limit = len(demodulated_bitstream) - len(sync_bits)
    for idx in range(max(limit, 0)):
        if np.array_equal(demodulated_bitstream[idx : idx + len(sync_bits)], sync_bits):
            sync_location = idx
            break

    if sync_location == -1:
        raise RuntimeError("Frame sync sequence not found. Decoding failed.")

    aligned_bitstream = demodulated_bitstream[sync_location:]
    num_frames = len(aligned_bitstream) // frame_size_bits

    decoded_payload_bits: List[int] = []
    for frame_idx in range(num_frames):
        start, end = frame_idx * frame_size_bits, (frame_idx + 1) * frame_size_bits
        frame = aligned_bitstream[start:end]
        payload_start = (
            (protocol.frame_layout.sync_symbols + protocol.frame_layout.target_id_symbols + protocol.frame_layout.command_type_symbols) * 2
        )
        payload_end = payload_start + matrices.codeword_bits
        if len(frame) < payload_end:
            continue
        noisy_codeword = frame[payload_start:payload_end]

        decoded_message = ldpc_decode(matrices.H, noisy_codeword, sim_config.snr_db)

        if frame_idx < 3:
            payload = noisy_codeword[: matrices.message_bits]
            ecc = noisy_codeword[matrices.message_bits :]
            logger.emit(
                f"[RX] Frame {frame_idx + 1}/{num_frames}\n"
                f"  > Noisy Payload : {bits_to_str(payload)}...\n"
                f"  > Noisy ECC     : {bits_to_str(ecc)}\n"
                f"  > Corrected Msg : {bits_to_str(decoded_message)}...",
                verbose,
            )

        decoded_payload_bits.extend(map(int, decoded_message))

    decoded_bitstream = np.array(decoded_payload_bits, dtype=np.uint8)
    trimmed_decoded = decoded_bitstream[: len(encoding.payload_bits)]

    post_fec_errors = int(np.sum(encoding.payload_bits != trimmed_decoded))
    post_fec_ber = post_fec_errors / len(encoding.payload_bits)
    logger.emit(f"Post-FEC BER: {post_fec_ber:.6f} ({post_fec_errors} errors).", verbose)

    recovered_bytes = np.packbits(trimmed_decoded)
    recovered_message = recovered_bytes.tobytes().decode("utf-8", "ignore").rstrip("\x00")

    return DemodulationResult(
        demodulated_bitstream=demodulated_bitstream,
        decoded_bitstream=trimmed_decoded,
        recovered_message=recovered_message,
        pre_fec_errors=pre_fec_errors,
        pre_fec_ber=pre_fec_ber,
        post_fec_errors=post_fec_errors,
        post_fec_ber=post_fec_ber,
        diagnostics=diagnostics,
        logs=logger.entries,
    )


def plot_demodulator_diagnostics(
    diag: DemodulationDiagnostics,
    ideal_phases: Sequence[float],
    protocol: ProtocolConfig,
    snr_db: float,
) -> None:
    """Visualize demodulator diagnostics using matplotlib."""

    print("\n--- DEMODULATOR DIAGNOSTICS ---")
    slice_len = min(200, len(diag.nco_freq_offset))

    print("\n--- Corrected Baseband I/Q Symbols (First 200) ---")
    for i in range(slice_len):
        print(f"Symbol {i:3d}: I={diag.received_symbols_i[i]:.4f}, Q={diag.received_symbols_q[i]:.4f}")

    print("\n--- Gardner Timing Error (First 200 Symbols) ---")
    for i in range(slice_len):
        print(f"Symbol {i:3d}: Error={diag.timing_error[i]:.4f}")

    print("\n--- Tracked NCO Frequency Offset (First 200 Symbols) ---")
    for i in range(slice_len):
        print(f"Symbol {i:3d}: Offset={diag.nco_freq_offset[i]:.4f} Hz")


def run_simulation(
    sim_config: Optional[SimulationConfig] = None,
    protocol: Optional[ProtocolConfig] = None,
    ldpc_config: Optional[LDPCConfig] = None,
    *,
    plaintext: Optional[str] = None,
    verbose: bool = False,
    rng: Optional[np.random.Generator] = None,
) -> SimulationResult:
    """Convenience wrapper that executes an end-to-end encode → transmit → decode pipeline."""

    sim_config = sim_config or SimulationConfig()
    protocol = protocol or ProtocolConfig()
    ldpc_config = ldpc_config or LDPCConfig()

    matrices = create_ldpc_matrices(protocol, ldpc_config, verbose=verbose)
    encoding = generate_modulated_signal(sim_config, protocol, matrices, plaintext=plaintext, rng=rng, verbose=verbose)
    demodulation = demodulate_and_decode(
        encoding,
        matrices,
        sim_config,
        protocol,
        plot=sim_config.generate_plots,
        verbose=verbose,
    )

    combined_logs = encoding.logs + demodulation.logs

    return SimulationResult(encoding=encoding, demodulation=demodulation, matrices=matrices, logs=combined_logs)

</final_file_content>

Now that you have the latest state of the file, try the operation again with fewer, more precise SEARCH blocks. For large files especially, it may be prudent to try to limit yourself to <5 SEARCH/REPLACE blocks at a time, then wait for the user to respond with the result of the operation before following up with another replace_in_file call to make additional edits.
(If you run into this error 3 times in a row, you may use the write_to_file tool as a fallback.)
</error><environment_details>
# VSCode Visible Files
chimera/pipeline.py

# VSCode Open Tabs
README.md
.git/COMMIT_EDITMSG
chimera/utils.py
../../Library/Application Support/Code/User/globalStorage/saoudrizwan.claude-dev/settings/cline_mcp_settings.json
chimera/config.py
pyproject.toml
tune_gains.py
chimera/pipeline.py
main.py

# Current Time
02/10/2025, 8:08:09 am (Australia/Melbourne, UTC+10:00)

# Context Window Usage
345,666 / 1,048.576K tokens used (33%)

# Current Mode
ACT MODE
</environment_details>
