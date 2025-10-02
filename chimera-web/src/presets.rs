use std::fmt;

use chimera_core::config::{BitDepth, FrameLayout, LDPCConfig, ProtocolConfig, SimulationConfig};
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum FramePreset {
    RamanWhisper,
    BurstTelemetry,
    DeepSpaceProbe,
}

#[derive(Debug, Clone)]
pub struct PresetBundle {
    pub protocol: ProtocolConfig,
    pub simulation: SimulationConfig,
    pub ldpc: LDPCConfig,
}

impl FramePreset {
    pub const ALL: [FramePreset; 3] = [
        FramePreset::RamanWhisper,
        FramePreset::BurstTelemetry,
        FramePreset::DeepSpaceProbe,
    ];

    pub fn all() -> &'static [FramePreset] {
        &Self::ALL
    }

    pub fn key(&self) -> &'static str {
        match self {
            FramePreset::RamanWhisper => "raman-whisper",
            FramePreset::BurstTelemetry => "burst-telemetry",
            FramePreset::DeepSpaceProbe => "deep-space-probe",
        }
    }

    pub fn from_key(key: &str) -> Option<Self> {
        Self::all()
            .iter()
            .copied()
            .find(|preset| preset.key() == key)
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            FramePreset::RamanWhisper => "Raman Whisper",
            FramePreset::BurstTelemetry => "Burst Telemetry",
            FramePreset::DeepSpaceProbe => "Deep-Space Probe",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            FramePreset::RamanWhisper => {
                "Baseline frame for terrestrial operations with balanced payload and ECC."
            }
            FramePreset::BurstTelemetry => {
                "High-rate bursts with tighter sync and extended payload for short-lived windows."
            }
            FramePreset::DeepSpaceProbe => {
                "Long-haul frames with heavy redundancy and relaxed bandwidth for deep-space links."
            }
        }
    }

    pub fn bundle(&self) -> PresetBundle {
        let mut protocol = ProtocolConfig::default();
        let mut simulation = SimulationConfig::default();
        let mut ldpc = LDPCConfig::default();

        match self {
            FramePreset::RamanWhisper => {
                // Keep defaults but ensure descriptive plaintext.
                simulation.plaintext_source = "This is a longer message demonstrating the protocol-compliant, rate-4/5 LDPC error correction.".into();
            }
            FramePreset::BurstTelemetry => {
                protocol.qpsk_symbol_rate = 32;
                protocol.qpsk_bandwidth_hz = 28.0;
                protocol.frame_layout = FrameLayout {
                    total_symbols: 96,
                    sync_symbols: 24,
                    target_id_symbols: 16,
                    command_type_symbols: 8,
                    data_payload_symbols: 40,
                    ecc_symbols: 8,
                };
                protocol.sync_sequence_hex = "7E7E7E7E".into();
                protocol.target_id_hex = "CAFEBABE".into();
                protocol.command_opcode = 0x00F1;
                protocol.max_frames = 64;
                protocol.current_frame_shift = 12;
                protocol.total_frames_shift = 18;

                simulation.sample_rate = 96_000;
                simulation.snr_db = 6.0;
                simulation.bit_depth = BitDepth::Pcm24;
                simulation.plaintext_source =
                    "Telemetry burst payload with accelerated downlink cadence.".into();

                ldpc.dv = 3;
                ldpc.dc = 9;
            }
            FramePreset::DeepSpaceProbe => {
                protocol.qpsk_symbol_rate = 12;
                protocol.qpsk_bandwidth_hz = 18.0;
                protocol.fsk_bit_rate = 0.5;
                protocol.fsk_freq_zero_hz = 11_998.4;
                protocol.fsk_freq_one_hz = 12_001.6;
                protocol.frame_layout = FrameLayout {
                    total_symbols: 256,
                    sync_symbols: 32,
                    target_id_symbols: 32,
                    command_type_symbols: 32,
                    data_payload_symbols: 96,
                    ecc_symbols: 64,
                };
                protocol.sync_sequence_hex = "55AA55AA".into();
                protocol.target_id_hex = "0D15EA5E".into();
                protocol.command_opcode = 0x0D11;
                protocol.max_frames = 32;
                protocol.current_frame_shift = 20;
                protocol.total_frames_shift = 28;

                simulation.sample_rate = 48_000;
                simulation.snr_db = 1.5;
                simulation.bit_depth = BitDepth::Float32;
                simulation.plaintext_source =
                    "Deep-space probe telemetry with reinforced parity blocks.".into();

                ldpc.dv = 2;
                ldpc.dc = 12;
                ldpc.seed = Some(1337);
            }
        }

        simulation.sample_rate = 48_000;

        PresetBundle {
            protocol,
            simulation,
            ldpc,
        }
    }

    pub fn protocol_config(&self) -> ProtocolConfig {
        self.bundle().protocol
    }

    pub fn simulation_config(&self) -> SimulationConfig {
        self.bundle().simulation
    }

    pub fn ldpc_config(&self) -> LDPCConfig {
        self.bundle().ldpc
    }
}

impl fmt::Display for FramePreset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.display_name())
    }
}
