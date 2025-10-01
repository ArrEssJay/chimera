//! LDPC matrix generation and decoding abstractions.
use serde::{Deserialize, Serialize};

use crate::config::{FrameLayout, LDPCConfig};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LDPCMatrices {
    pub h: Vec<Vec<u8>>,
    pub g: Vec<Vec<u8>>,
    pub message_bits: usize,
    pub codeword_bits: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LDPCSuite {
    pub matrices: LDPCMatrices,
}

impl LDPCSuite {
    pub fn default_stub() -> Self {
        Self {
            matrices: LDPCMatrices {
                h: Vec::new(),
                g: Vec::new(),
                message_bits: 0,
                codeword_bits: 0,
            },
        }
    }
}

pub fn build_ldpc_matrices(layout: &FrameLayout, cfg: &LDPCConfig) -> LDPCMatrices {
    let _ = (layout, cfg);
    // TODO: Integrate sparse-ldpc once the API surface is validated.
    unimplemented!("LDPC generation not yet implemented")
}

pub fn decode_ldpc(_matrices: &LDPCMatrices, _noisy_codeword: &[u8], _snr_db: f64) -> Vec<u8> {
    // TODO: hook into LDPC decoder once available.
    unimplemented!("LDPC decoding not yet implemented")
}
