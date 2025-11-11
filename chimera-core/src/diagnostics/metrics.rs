//! Signal quality metrics

use num_complex::Complex64;

/// Calculate Error Vector Magnitude (EVM) between transmitted and received symbols
/// 
/// EVM quantifies the difference between transmitted and received constellation points.
/// Returns percentage where 0.0 = perfect match, 100.0 = maximum error.
/// 
/// When TX symbols are provided, computes symbol-by-symbol error (requires alignment).
/// Both TX and RX symbols are normalized to unit average power before comparison.
/// 
/// # Arguments
/// * `tx_symbols` - Transmitted (reference) symbols  
/// * `rx_symbols` - Received symbols
/// 
/// # Returns
/// EVM as percentage (0-100)
pub fn compute_evm(tx_symbols: &[Complex64], rx_symbols: &[Complex64]) -> f32 {
    if rx_symbols.is_empty() || tx_symbols.is_empty() {
        return 0.0;
    }
    
    let count = rx_symbols.len().min(tx_symbols.len());
    
    // Compute average power of both sets
    let tx_power: f64 = tx_symbols[..count].iter()
        .map(|s| s.norm_sqr())
        .sum::<f64>() / count as f64;
    
    let rx_power: f64 = rx_symbols[..count].iter()
        .map(|s| s.norm_sqr())
        .sum::<f64>() / count as f64;
    
    if tx_power <= 0.0 || rx_power <= 0.0 {
        return 0.0;
    }
    
    // Normalize both to unit power
    let tx_scale = (1.0 / tx_power).sqrt();
    let rx_scale = (1.0 / rx_power).sqrt();
    
    // Compute RMS error between normalized symbols
    let mut error_sum = 0.0;
    for i in 0..count {
        let tx_norm = tx_symbols[i] * tx_scale;
        let rx_norm = rx_symbols[i] * rx_scale;
        let error = rx_norm - tx_norm;
        error_sum += error.norm_sqr();
    }
    
    // EVM as percentage of normalized reference power (which is 1.0)
    100.0 * (error_sum / count as f64).sqrt() as f32
}

/// Calculate constellation-based EVM (without requiring TX/RX alignment)
///
/// Measures how far received symbols deviate from the ideal QPSK constellation.
/// Useful when TX/RX symbol correspondence is not available or not yet aligned.
///
/// # Arguments
/// * `rx_symbols` - Received symbols to evaluate
///
/// # Returns
/// EVM as percentage (0-100)
pub fn compute_constellation_evm(rx_symbols: &[Complex64]) -> f32 {
    if rx_symbols.is_empty() {
        return 0.0;
    }
    
    // Ideal QPSK constellation points (unit magnitude)
    let ideal_points = [
        Complex64::new(std::f64::consts::FRAC_1_SQRT_2, std::f64::consts::FRAC_1_SQRT_2),
        Complex64::new(-std::f64::consts::FRAC_1_SQRT_2, std::f64::consts::FRAC_1_SQRT_2),
        Complex64::new(-std::f64::consts::FRAC_1_SQRT_2, -std::f64::consts::FRAC_1_SQRT_2),
        Complex64::new(std::f64::consts::FRAC_1_SQRT_2, -std::f64::consts::FRAC_1_SQRT_2),
    ];
    
    // Compute average power for normalization
    let rx_power: f64 = rx_symbols.iter()
        .map(|s| s.norm_sqr())
        .sum::<f64>() / rx_symbols.len() as f64;
    
    if rx_power <= 0.0 {
        return 0.0;
    }
    
    let rx_scale = (1.0 / rx_power).sqrt();
    
    // For each RX symbol, find nearest ideal point and measure error
    let mut error_sum = 0.0;
    let mut count = 0;
    
    for &rx_symbol in rx_symbols {
        let rx_norm = rx_symbol * rx_scale;
        
        // Find nearest ideal constellation point
        let mut min_dist_sq = f64::INFINITY;
        for &ideal in &ideal_points {
            let dist_sq = (rx_norm - ideal).norm_sqr();
            if dist_sq < min_dist_sq {
                min_dist_sq = dist_sq;
            }
        }
        
        error_sum += min_dist_sq;
        count += 1;
    }
    
    // RMS EVM as percentage
    100.0 * (error_sum / count as f64).sqrt() as f32
}

/// Estimate Signal-to-Noise Ratio from received symbols
/// 
/// Estimates SNR by measuring signal and noise power separately.
/// Signal power is estimated from the received symbol power.
/// Noise power is estimated from variance around scaled constellation points.
/// 
/// # Arguments
/// * `rx_symbols` - Received symbol constellation
/// 
/// # Returns
/// Estimated SNR in dB (higher is better)
pub fn estimate_snr(rx_symbols: &[Complex64]) -> f32 {
    if rx_symbols.is_empty() {
        return 0.0;
    }
    
    // Ideal QPSK constellation points (unit power: each point has magnitude 1/√2)
    let ideal_points = [
        Complex64::new(std::f64::consts::FRAC_1_SQRT_2, std::f64::consts::FRAC_1_SQRT_2),
        Complex64::new(-std::f64::consts::FRAC_1_SQRT_2, std::f64::consts::FRAC_1_SQRT_2),
        Complex64::new(-std::f64::consts::FRAC_1_SQRT_2, -std::f64::consts::FRAC_1_SQRT_2),
        Complex64::new(std::f64::consts::FRAC_1_SQRT_2, -std::f64::consts::FRAC_1_SQRT_2),
    ];
    
    // Compute average symbol power
    let rx_power: f64 = rx_symbols.iter()
        .map(|s| s.norm_sqr())
        .sum::<f64>() / rx_symbols.len() as f64;
    
    if rx_power <= 0.0 {
        return 0.0;
    }
    
    // Scale ideal constellation to match received signal power
    let scale = rx_power.sqrt();
    
    // Compute noise power by measuring deviation from nearest scaled ideal point
    let mut noise_power = 0.0;
    
    for &rx_symbol in rx_symbols {
        // Find nearest ideal constellation point (scaled to match RX power)
        let mut min_dist_sq = f64::INFINITY;
        for &ideal in &ideal_points {
            let scaled_ideal = ideal * scale;
            let dist_sq = (rx_symbol - scaled_ideal).norm_sqr();
            if dist_sq < min_dist_sq {
                min_dist_sq = dist_sq;
            }
        }
        
        noise_power += min_dist_sq;
    }
    
    noise_power /= rx_symbols.len() as f64;
    
    if noise_power > 0.0 && rx_power > 0.0 {
        10.0 * (rx_power / noise_power).log10() as f32
    } else {
        40.0 // Very high SNR
    }
}

/// Calculate Bit Error Rate between transmitted and received bits
/// 
/// Compares two bit sequences and returns the fraction of mismatched bits.
/// 
/// # Arguments
/// * `tx_bits` - Transmitted bits (0 or 1)
/// * `rx_bits` - Received bits (0 or 1)
/// 
/// # Returns
/// BER as ratio (0.0 = no errors, 1.0 = all errors)
pub fn compute_ber(tx_bits: &[u8], rx_bits: &[u8]) -> f32 {
    if tx_bits.is_empty() || rx_bits.is_empty() {
        return 0.0;
    }
    
    let count = tx_bits.len().min(rx_bits.len());
    let errors = (0..count)
        .filter(|&i| tx_bits[i] != rx_bits[i])
        .count();
    
    errors as f32 / count as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evm_zero_for_perfect_match() {
        let symbols = vec![
            Complex64::new(0.707, 0.707),
            Complex64::new(-0.707, 0.707),
        ];
        
        let evm = compute_evm(&symbols, &symbols);
        assert!(evm < 0.001, "EVM should be near zero for identical symbols");
    }

    #[test]
    fn test_evm_increases_with_noise() {
        let tx = vec![Complex64::new(1.0, 0.0)];
        let rx_clean = vec![Complex64::new(1.0, 0.0)];
        let rx_noisy = vec![Complex64::new(0.9, 0.1)];
        
        let evm_clean = compute_evm(&tx, &rx_clean);
        let evm_noisy = compute_evm(&tx, &rx_noisy);
        
        assert!(evm_noisy > evm_clean);
    }

    #[test]
    fn test_snr_estimation() {
        let perfect_symbols = vec![Complex64::new(1.0, 0.0); 100];
        let snr_perfect = estimate_snr(&perfect_symbols);
        
        // Perfect symbols should have very high SNR
        assert!(snr_perfect > 30.0);
        
        let noisy_symbols: Vec<Complex64> = (0..100)
            .map(|i| Complex64::new(1.0 + (i as f64) * 0.01, (i as f64) * 0.01))
            .collect();
        let snr_noisy = estimate_snr(&noisy_symbols);
        
        // Noisy symbols should have lower SNR
        assert!(snr_noisy < snr_perfect);
    }

    #[test]
    fn test_ber_zero_for_perfect_match() {
        let bits = vec![0, 1, 0, 1, 1, 0];
        let ber = compute_ber(&bits, &bits);
        
        assert_eq!(ber, 0.0);
    }

    #[test]
    fn test_ber_calculation() {
        let tx = vec![0, 1, 0, 1, 1, 0, 1, 0];
        let rx = vec![0, 1, 1, 1, 1, 0, 0, 0]; // 2 errors out of 8
        
        let ber = compute_ber(&tx, &rx);
        assert!((ber - 0.25).abs() < 0.001); // 2/8 = 0.25
    }

    #[test]
    fn test_empty_input_returns_zero() {
        let empty: Vec<Complex64> = vec![];
        assert_eq!(compute_evm(&empty, &empty), 0.0);
        assert_eq!(estimate_snr(&empty), 0.0);
        
        let empty_bits: Vec<u8> = vec![];
        assert_eq!(compute_ber(&empty_bits, &empty_bits), 0.0);
    }
}
