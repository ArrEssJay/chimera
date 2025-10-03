use ldpc::codes::{CssCode, LinearCode};
use ndarray::Array2;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use sparse_bin_mat::SparseBinMat;

use crate::config::{FrameLayout, LDPCConfig};

/// Represents the core matrices and parameters of a classical linear block code.
///
/// This structure holds the generator and parity-check matrices, along with the
/// fundamental dimensions of the code: the number of message bits (k), codeword
/// bits (n), and parity-check bits (n-k).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LDPCMatrices {
    /// The parity-check matrix (H) of the code.
    ///
    /// A binary vector `c` is a valid codeword if `H * c^T = 0`.
    pub parity_check: Array2<u8>,
    /// The generator matrix (G) of the code.
    ///
    /// A message vector `m` is encoded into a codeword `c` by `c = m * G`.
    pub generator: Array2<u8>,
    /// The number of message bits (k), also known as the dimension of the code.
    pub message_bits: usize,
    /// The number of codeword bits (n), also known as the block length.
    pub codeword_bits: usize,
    /// The number of parity-check bits (n-k).
    pub parity_bits: usize,
}

/// A collection of LDPC-related code structures for quantum error correction.
///
/// This suite encapsulates the classical X and Z codes, the resulting quantum
/// CSS code, and the dense matrix representations required for classical encoding
/// and decoding operations.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LDPCSuite {
    /// The classical linear code for correcting Z-type errors (bit-flips).
    pub x_code: LinearCode,
    /// The classical linear code for correcting X-type errors (phase-flips).
    pub z_code: LinearCode,
    /// The Calderbank-Shor-Steane (CSS) quantum code constructed from the X and Z codes.
    pub quantum_css: CssCode,
    /// Dense matrix representations derived from the `x_code` for simulation purposes.
    pub matrices: LDPCMatrices,
}

impl LDPCSuite {
    /// Creates a new `LDPCSuite` based on a given frame layout and configuration.
    ///
    /// This constructor currently builds a simple repetition-like code structure
    /// where the X and Z checks are orthogonal. The `LDPCConfig` is reserved for
    /// future use, such as generating random LDPC code ensembles with specific
    /// properties (e.g., defined `dv` and `dc`).
    ///
    /// # Arguments
    ///
    /// * `layout` - The `FrameLayout` defining the number of message, ECC, and total bits.
    /// * `cfg` - The `LDPCConfig` for the code construction (currently unused).
    ///
    /// # Returns
    ///
    /// A new `LDPCSuite` instance.
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

#[inline]
fn words_for_bits(bits: usize) -> usize {
    bits.div_ceil(64)
}

#[inline]
fn get_bit(words: &[u64], idx: usize) -> u8 {
    let word = idx / 64;
    let bit = idx % 64;
    ((words[word] >> bit) & 1) as u8
}

#[inline]
fn set_bit(words: &mut [u64], idx: usize, value: u8) {
    let word = idx / 64;
    let bit = idx % 64;
    let mask = 1u64 << bit;
    if value & 1 == 1 {
        words[word] |= mask;
    } else {
        words[word] &= !mask;
    }
}

#[inline]
fn xor_suffix(row: &mut [u64], pivot: &[u64], start_bit: usize, total_bits: usize) {
    if start_bit >= total_bits {
        return;
    }

    let start_word = start_bit / 64;
    let bit_offset = start_bit % 64;
    let total_words = words_for_bits(total_bits);
    let last_bits = total_bits % 64;

    for word_idx in start_word..total_words {
        let mut mask = u64::MAX;
        if word_idx == start_word {
            mask &= u64::MAX << bit_offset;
        }
        if last_bits != 0 && word_idx == total_words - 1 {
            mask &= (1u64 << last_bits) - 1;
        }

        row[word_idx] ^= pivot[word_idx] & mask;
    }
}

/// Performs hard-decision decoding of an LDPC codeword.
///
/// This function attempts to recover the original message bits from a (potentially noisy)
/// codeword by solving the linear system `G * m^T = c^T` over the finite field GF(2),
/// where `G` is the generator matrix, `m` is the message vector, and `c` is the codeword.
///
/// The implementation uses parallelized Gaussian elimination to transform an augmented
/// matrix `[G^T | c]` into reduced row echelon form. This method is effective for
/// recovering the message when the codeword has few or no errors. It is not a
/// belief propagation or message-passing decoder and is therefore not robust against
/// high noise levels.
///
/// The `_snr_db` parameter is currently unused but is reserved for future integration
/// with soft-decision decoding algorithms.
///
/// # Arguments
///
/// * `matrices` - A reference to the `LDPCMatrices` containing the generator matrix.
/// * `noisy_codeword` - A slice of `u8` representing the received codeword bits. Its
///   length must equal `matrices.codeword_bits`.
/// * `_snr_db` - The signal-to-noise ratio in decibels (currently unused).
///
/// # Returns
///
/// A `Vec<u8>` containing the recovered message bits. If the system is underdetermined,
/// free variables are resolved to 0.
///
/// # Panics
///
/// Panics in debug builds if `noisy_codeword.len()` does not match `matrices.codeword_bits`.
pub fn decode_ldpc(matrices: &LDPCMatrices, noisy_codeword: &[u8], _snr_db: f64) -> Vec<u8> {
    let message_bits = matrices.message_bits;
    let codeword_bits = matrices.codeword_bits;
    let total_bits = message_bits + 1;
    let word_len = words_for_bits(total_bits);

    debug_assert_eq!(noisy_codeword.len(), codeword_bits);

    // Build the augmented matrix for the linear system G^T * m = c, where G is
    // the generator matrix and c is the received codeword. Each row encodes a
    // single codeword bit equation over GF(2).
    let mut augmented: Vec<Vec<u64>> = (0..codeword_bits)
        .into_iter()
        .map(|col| {
            let mut row = vec![0u64; word_len];
            for row_idx in 0..message_bits {
                if matrices.generator[(row_idx, col)] & 1 == 1 {
                    set_bit(&mut row, row_idx, 1);
                }
            }
            if noisy_codeword[col] & 1 == 1 {
                set_bit(&mut row, message_bits, 1);
            }
            row
        })
        .collect();

    // Gaussian elimination over GF(2) to obtain reduced row echelon form. We
    // keep track of which row becomes the pivot for each message bit column.
    let mut pivot_row = 0usize;
    let mut column_pivots: Vec<Option<usize>> = vec![None; message_bits];

    for (col, pivot) in column_pivots.iter_mut().enumerate().take(message_bits) {
        if pivot_row >= augmented.len() {
            break;
        }

        // Find a row with a leading 1 in this column.
        if let Some(pivot_idx) =
            (pivot_row..augmented.len()).find(|&r| get_bit(&augmented[r], col) == 1)
        {
            augmented.swap(pivot_row, pivot_idx);

            // Eliminate this column from every other row to reach reduced form.
            // The pivot row is now at `pivot_row`. We can borrow it immutably
            // while we modify other rows.
            let (pivot_slice, other_rows) = augmented.split_at_mut(pivot_row);
            let (pivot_row_ref, other_rows_after) = other_rows.split_at_mut(1);

            // XOR the pivot row with all other rows that have a 1 in the pivot column.
            let pivot_row_data = &pivot_row_ref[0];

            let eliminate = |rows: &mut [Vec<u64>]| {
                // Use sequential iteration for small matrices to avoid parallel overhead.
                const PARALLEL_THRESHOLD: usize = 1000;
                if rows.len() < PARALLEL_THRESHOLD {
                    rows.iter_mut()
                        .filter(|row| get_bit(row, col) == 1)
                        .for_each(|row| xor_suffix(row, pivot_row_data, col, total_bits));
                } else {
                    rows.par_iter_mut()
                        .filter(|row| get_bit(row, col) == 1)
                        .for_each(|row| xor_suffix(row, pivot_row_data, col, total_bits));
                }
            };

            eliminate(pivot_slice);
            eliminate(other_rows_after);

            *pivot = Some(pivot_row);
            pivot_row += 1;
        }
    }

    let mut message = vec![0u8; message_bits];
    for (col, pivot) in column_pivots.into_iter().enumerate() {
        if let Some(row_idx) = pivot {
            message[col] = get_bit(&augmented[row_idx], message_bits);
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
            for (col_idx, codeword_bit) in
                codeword.iter_mut().enumerate().take(layout.codeword_bits())
            {
                *codeword_bit ^= matrices.generator[(row_idx, col_idx)] & 1;
            }
        }

        let decoded = decode_ldpc(&matrices, &codeword, 0.0);
        assert_eq!(decoded, message);
    }
}
