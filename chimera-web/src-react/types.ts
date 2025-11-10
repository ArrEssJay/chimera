/**
 * TypeScript types matching Rust backend structures
 */

export interface SimulationConfig {
  plaintext_source: string;
  snr_db: number;
  link_loss_db: number;
  rng_seed?: number;
}

export interface FrameLayout {
  total_symbols: number;
  sync_symbols: number;
  target_id_symbols: number;
  command_type_symbols: number;
  data_payload_symbols: number;
  ecc_symbols: number;
}

export interface ProtocolConfig {
  carrier_freq_hz: number;
  qpsk_symbol_rate: number;
  qpsk_bandwidth_hz: number;
  fsk_bit_rate: number;
  fsk_freq_zero_hz: number;
  fsk_freq_one_hz: number;
  command_opcode: number;
  frame_layout: FrameLayout;
  sync_sequence_hex: string;
  target_id_hex: string;
  max_frames: number;
  current_frame_shift: number;
  total_frames_shift: number;
}

export interface LDPCConfig {
  dv: number;
  dc: number;
  seed?: number;
}

export interface FrameDescriptor {
  frame_index: number;
  bit_offset: number;
  bit_length: number;
  preamble_offset: number;
  sync_offset: number;
  header_offset: number;
  payload_offset: number;
}

export interface SymbolDecision {
  symbol_index: number;
  received_i: number;
  received_q: number;
  decided_i: number;
  decided_q: number;
  distance: number;
  bits: number[];
}

export interface DemodulationDiagnostics {
  symbol_decisions: SymbolDecision[];
  mean_evm: number;
  mean_distance: number;
}

export interface SimulationReport {
  pre_fec_ber: number;
  post_fec_ber: number;
  pre_fec_errors: number;
  post_fec_errors: number;
  total_bits: number;
  recovered_message: string;
  sync_found: boolean;
  sync_position: number;
}

export interface FSKState {
  current_frequency_hz: number;
  frequency_deviation_hz: number;
  current_bit: number;
  bit_index: number;
  bit_history: number[];
  symbols_per_bit: number;
  bit_rate_hz: number;
}

export interface EncodingResult {
  qpsk_bitstream: Uint8Array;
  payload_bits: Uint8Array;
  total_frames: number;
  samples_per_symbol: number;
  sample_rate: number;
  logs: string[];
  frame_descriptors: FrameDescriptor[];
}

export interface DemodulationResult {
  demodulated_bitstream: Uint8Array;
  decoded_bitstream: Uint8Array;
  recovered_message: string;
  diagnostics: DemodulationDiagnostics;
  report: SimulationReport;
  logs: string[];
}

export interface ProcessingResult {
  encoding: EncodingResult;
  demodulation: DemodulationResult;
  clean_signal: Float64Array;
  noisy_signal: Float64Array;
  qpsk_symbols: Array<{ re: number; im: number }>;
}

// FramePreset matches Rust enum in presets.rs
export type FramePresetKey = 'raman-whisper' | 'burst-telemetry' | 'deep-space-probe';

export interface FramePresetInfo {
  key: FramePresetKey;
  displayName: string;
  description: string;
}

export const FRAME_PRESETS: FramePresetInfo[] = [
  {
    key: 'raman-whisper',
    displayName: 'Raman Whisper',
    description: 'Baseline frame for terrestrial operations with balanced payload and ECC.',
  },
  {
    key: 'burst-telemetry',
    displayName: 'Burst Telemetry',
    description: 'High-rate bursts with tighter sync and extended payload for short-lived windows.',
  },
  {
    key: 'deep-space-probe',
    displayName: 'Deep-Space Probe',
    description: 'Long-haul frames with heavy redundancy and relaxed bandwidth for deep-space links.',
  },
];

// Full configuration bundle
export interface ConfigBundle {
  simulation: SimulationConfig;
  protocol: ProtocolConfig;
  ldpc: LDPCConfig;
}
