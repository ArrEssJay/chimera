//! LDPC matrix generation and decoding abstractions.
use ndarray::{Array2, Axis};
use serde::{Deserialize, Serialize};

use crate::config::{FrameLayout, LDPCConfig};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LDPCMatrices {
    pub h: Array2<u8>,
    pub g: Array2<u8>,
    pub message_bits: usize,
    pub codeword_bits: usize,
    pub parity_bits: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LDPCSuite {
    pub matrices: LDPCMatrices,
}

impl LDPCSuite {
    pub fn new(layout: &FrameLayout, cfg: &LDPCConfig) -> Self {
        Self {
            matrices: build_ldpc_matrices(layout, cfg),
        }
    }
}

/// Construct a simple systematic generator/parity suite.
///
/// The current implementation produces an identity generator matrix augmented
/// with parity identity columns. This acts as a stub until a production LDPC
/// backend is wired in, while still exercising the framing logic and ensuring
/// consistent dimensions.
pub fn build_ldpc_matrices(layout: &FrameLayout, _cfg: &LDPCConfig) -> LDPCMatrices {
    let message_bits = layout.message_bits();
    let parity_bits = layout.ecc_bits();
    let codeword_bits = layout.codeword_bits();

    let mut g = Array2::<u8>::zeros((message_bits, codeword_bits));
    for (row, mut row_view) in g.axis_iter_mut(Axis(0)).enumerate() {
        row_view[row] = 1;
    }

    let mut h = Array2::<u8>::zeros((parity_bits, codeword_bits));
    for (idx, mut row) in h.axis_iter_mut(Axis(0)).enumerate() {
        let col = message_bits + idx;
        if col < codeword_bits {
            row[col] = 1;
        }
    }

    LDPCMatrices {
        h,
        g,
        message_bits,
        codeword_bits,
        parity_bits,
    }
}

/// Decode by stripping parity bits (no correction yet).
pub fn decode_ldpc(matrices: &LDPCMatrices, noisy_codeword: &[u8], _snr_db: f64) -> Vec<u8> {
    debug_assert_eq!(noisy_codeword.len(), matrices.codeword_bits);
    noisy_codeword[..matrices.message_bits].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrices_have_expected_shapes() {
        let layout = FrameLayout::default();
        let cfg = LDPCConfig::default();
        let matrices = build_ldpc_matrices(&layout, &cfg);

        assert_eq!(matrices.g.nrows(), layout.message_bits());
        assert_eq!(matrices.g.ncols(), layout.codeword_bits());
        assert_eq!(matrices.h.nrows(), layout.ecc_bits());
        assert_eq!(matrices.h.ncols(), layout.codeword_bits());
    }

    #[test]
    fn decode_ldpc_returns_systematic_bits() {
        let layout = FrameLayout::default();
        let cfg = LDPCConfig::default();
        let matrices = build_ldpc_matrices(&layout, &cfg);

        let mut codeword = vec![0u8; layout.codeword_bits()];
        for i in 0..layout.message_bits() {
            codeword[i] = (i % 2) as u8;
        }
        for i in layout.message_bits()..layout.codeword_bits() {
            codeword[i] = 1;
        }

        let decoded = decode_ldpc(&matrices, &codeword, 0.0);
        assert_eq!(decoded.len(), layout.message_bits());
        assert_eq!(decoded, codeword[..layout.message_bits()].to_vec());
    }
}
