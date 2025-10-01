//! LDPC matrix generation and decoding abstractions built on the `ldpc` quantum LDPC crate.
use ldpc::codes::{CssCode, LinearCode};
use ndarray::Array2;
use serde::{Deserialize, Serialize};
use sparse_bin_mat::SparseBinMat;

use crate::config::{FrameLayout, LDPCConfig};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LDPCMatrices {
    pub parity_check: Array2<u8>,
    pub generator: Array2<u8>,
    pub message_bits: usize,
    pub codeword_bits: usize,
    pub parity_bits: usize,
}

impl Default for LDPCMatrices {
    fn default() -> Self {
        Self {
            parity_check: Array2::zeros((0, 0)),
            generator: Array2::zeros((0, 0)),
            message_bits: 0,
            codeword_bits: 0,
            parity_bits: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LDPCSuite {
    pub x_code: LinearCode,
    pub z_code: LinearCode,
    pub quantum_css: CssCode,
    pub matrices: LDPCMatrices,
}

impl LDPCSuite {
    pub fn new(layout: &FrameLayout, cfg: &LDPCConfig) -> Self {
        let (x_code, z_code) = build_css_pair(layout);
        let quantum_css = CssCode::new(&x_code, &z_code);
        let matrices = build_ldpc_matrices(&x_code);

        let _ = cfg; // Future work: honour dv/dc to sample random LDPC ensembles.

        Self {
            x_code,
            z_code,
            quantum_css,
            matrices,
        }
    }
}

fn build_css_pair(layout: &FrameLayout) -> (LinearCode, LinearCode) {
    let codeword_bits = layout.codeword_bits();
    let message_bits = layout.message_bits();
    let parity_bits = layout.ecc_bits();

    let x_checks = SparseBinMat::new(
        codeword_bits,
        (0..parity_bits)
            .map(|idx| vec![message_bits + idx])
            .collect::<Vec<_>>(),
    );

    let z_checks = SparseBinMat::new(
        codeword_bits,
        (0..message_bits).map(|idx| vec![idx]).collect::<Vec<_>>(),
    );

    let x_code = LinearCode::from_parity_check_matrix(x_checks);
    let z_code = LinearCode::from_parity_check_matrix(z_checks);
    (x_code, z_code)
}

fn build_ldpc_matrices(code: &LinearCode) -> LDPCMatrices {
    LDPCMatrices {
        parity_check: sparse_to_array(code.parity_check_matrix()),
        generator: sparse_to_array(code.generator_matrix()),
        message_bits: code.dimension(),
        codeword_bits: code.len(),
        parity_bits: code.num_checks(),
    }
}

fn sparse_to_array(matrix: &SparseBinMat) -> Array2<u8> {
    let rows = matrix.number_of_rows();
    let cols = matrix.number_of_columns();
    let mut dense = Array2::<u8>::zeros((rows, cols));

    for (row_idx, row) in matrix.rows().enumerate() {
        for col_idx in row.non_trivial_positions() {
            dense[(row_idx, col_idx)] = 1;
        }
    }

    dense
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
        let suite = LDPCSuite::new(&layout, &cfg);

        assert_eq!(suite.matrices.generator.nrows(), layout.message_bits());
        assert_eq!(suite.matrices.generator.ncols(), layout.codeword_bits());
        assert_eq!(suite.matrices.parity_check.nrows(), layout.ecc_bits());
        assert_eq!(suite.matrices.parity_check.ncols(), layout.codeword_bits());
        assert_eq!(suite.quantum_css.len(), layout.codeword_bits());
    }

    #[test]
    fn decode_ldpc_returns_systematic_bits() {
        let layout = FrameLayout::default();
        let cfg = LDPCConfig::default();
        let matrices = LDPCSuite::new(&layout, &cfg).matrices;

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
