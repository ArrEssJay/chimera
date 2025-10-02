//! LDPC matrix generation and decoding abstractions built on the `ldpc` quantum LDPC crate.
use ldpc::codes::{CssCode, LinearCode};
use ndarray::Array2;
use serde::{Deserialize, Serialize};
use sparse_bin_mat::SparseBinMat;

use crate::config::{FrameLayout, LDPCConfig};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

/// Perform hard-decision decoding by solving the linear system defined by the
/// generator matrix. This recovers the original message bits when no channel
/// errors remain after demodulation.
pub fn decode_ldpc(matrices: &LDPCMatrices, noisy_codeword: &[u8], _snr_db: f64) -> Vec<u8> {
    let message_bits = matrices.message_bits;
    let codeword_bits = matrices.codeword_bits;

    debug_assert_eq!(noisy_codeword.len(), codeword_bits);

    // Build the augmented matrix for the linear system G^T * m = c, where G is
    // the generator matrix and c is the received codeword. Each row encodes a
    // single codeword bit equation over GF(2).
    let mut augmented: Vec<Vec<u8>> = Vec::with_capacity(codeword_bits);
    for (col, &codeword_bit) in noisy_codeword.iter().enumerate().take(codeword_bits) {
        let mut row = Vec::with_capacity(message_bits + 1);
        for row_idx in 0..message_bits {
            row.push(matrices.generator[(row_idx, col)] & 1);
        }
        row.push(codeword_bit & 1);
        augmented.push(row);
    }

    // Gaussian elimination over GF(2) to obtain reduced row echelon form. We
    // keep track of which row becomes the pivot for each message bit column.
    let mut pivot_row = 0usize;
    let mut column_pivots: Vec<Option<usize>> = vec![None; message_bits];

    for (col, pivot) in column_pivots.iter_mut().enumerate().take(message_bits) {
        if pivot_row >= augmented.len() {
            break;
        }

        // Find a row with a leading 1 in this column.
        if let Some(pivot_idx) = (pivot_row..augmented.len()).find(|&r| augmented[r][col] == 1) {
            augmented.swap(pivot_row, pivot_idx);

            // Eliminate this column from every other row to reach reduced form.
            for r in 0..augmented.len() {
                if r != pivot_row && augmented[r][col] == 1 {
                    for c in col..=message_bits {
                        augmented[r][c] ^= augmented[pivot_row][c];
                    }
                }
            }

            *pivot = Some(pivot_row);
            pivot_row += 1;
        }
    }

    let mut message = vec![0u8; message_bits];
    for (col, pivot) in column_pivots.into_iter().enumerate() {
        if let Some(row_idx) = pivot {
            message[col] = augmented[row_idx][message_bits];
        } else {
            // Column without a pivot corresponds to a free variable; choose 0.
            message[col] = 0;
        }
    }

    message
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{rngs::StdRng, Rng, SeedableRng};

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
    fn decode_ldpc_recovers_encoded_message() {
        let layout = FrameLayout::default();
        let cfg = LDPCConfig::default();
        let matrices = LDPCSuite::new(&layout, &cfg).matrices;

        let mut rng = StdRng::seed_from_u64(1337);
        let message: Vec<u8> = (0..layout.message_bits())
            .map(|_| if rng.gen_bool(0.5) { 1 } else { 0 })
            .collect();

        let mut codeword = vec![0u8; layout.codeword_bits()];
        for (row_idx, &bit) in message.iter().enumerate() {
            if bit == 0 {
                continue;
            }
            for (col_idx, codeword_bit) in codeword.iter_mut().enumerate().take(layout.codeword_bits()) {
                *codeword_bit ^= matrices.generator[(row_idx, col_idx)] & 1;
            }
        }

        let decoded = decode_ldpc(&matrices, &codeword, 0.0);
        assert_eq!(decoded, message);
    }
}
