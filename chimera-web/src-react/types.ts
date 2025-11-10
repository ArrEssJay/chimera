/**
 * TypeScript types matching Rust backend structures
 */

export type BitDepth = 'Pcm16' | 'Pcm24' | 'Pcm32' | 'Float32';

export interface SimulationConfig {
  plaintext_source: string;
  snr_db: number;
  link_loss_db: number;
  sample_rate: number;
  bit_depth: BitDepth;
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

// Preset configurations
export interface SimulationPreset {
  name: string;
  description: string;
  config: SimulationConfig;
}

export const SIMULATION_PRESETS: SimulationPreset[] = [
  {
    name: 'Default',
    description: 'Standard test configuration',
    config: {
      plaintext_source: 'Hello CHIMERA',
      snr_db: 10,
      link_loss_db: 0,
      sample_rate: 48000,
      bit_depth: 'Float32' as BitDepth,
    },
  },
  {
    name: 'High SNR',
    description: 'Clean channel, minimal noise',
    config: {
      plaintext_source: 'Testing high SNR scenario',
      snr_db: 20,
      link_loss_db: 0,
      sample_rate: 48000,
      bit_depth: 'Float32' as BitDepth,
    },
  },
  {
    name: 'Low SNR',
    description: 'Noisy channel stress test',
    config: {
      plaintext_source: 'Low SNR test',
      snr_db: 3,
      link_loss_db: 5,
      sample_rate: 48000,
      bit_depth: 'Float32' as BitDepth,
    },
  },
  {
    name: 'Long Message',
    description: 'Test multi-frame encoding',
    config: {
      plaintext_source: 'This is a longer message to test multi-frame encoding and decoding capabilities of the CHIMERA system.',
      snr_db: 10,
      link_loss_db: 0,
      sample_rate: 48000,
      bit_depth: 'Float32' as BitDepth,
    },
  },
];
