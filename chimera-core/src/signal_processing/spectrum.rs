//! Spectrum analysis utilities using FFT

use num_complex::Complex;
use rustfft::{FftPlanner, num_complex::Complex32};

/// Compute baseband spectrum directly from IQ symbols
/// 
/// Performs FFT on the symbol stream with zero-padding and windowing
/// for improved frequency resolution. Returns power spectrum in dB scale.
/// 
/// # Arguments
/// * `symbols` - IQ symbol stream to analyze
/// 
/// # Returns
/// Power spectrum in dB, DC-centered, covering useful bandwidth
pub fn compute_baseband_spectrum(symbols: &[Complex<f64>]) -> Vec<f32> {
    if symbols.len() < 32 {
        return Vec::new(); // Need at least 32 symbols for meaningful spectrum
    }
    
    // Zero-pad to get better frequency resolution
    let fft_size = 512;
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(fft_size);
    
    // Create buffer with zero-padding
    let mut buffer = prepare_fft_buffer(symbols, fft_size);
    
    // Apply Hamming window (better for continuous signals)
    apply_hamming_window(&mut buffer, symbols.len().min(fft_size));
    
    fft.process(&mut buffer);
    
    // Convert to power spectrum in dB
    let spectrum = compute_power_spectrum_db(&buffer, symbols.len().min(fft_size));
    
    // Return DC-centered portion
    extract_centered_spectrum(spectrum, fft_size)
}

/// Prepare FFT buffer with zero-padding
fn prepare_fft_buffer(symbols: &[Complex<f64>], fft_size: usize) -> Vec<Complex32> {
    let mut buffer = Vec::with_capacity(fft_size);
    
    // Add actual symbols
    for symbol in symbols.iter().take(fft_size) {
        buffer.push(Complex32::new(symbol.re as f32, symbol.im as f32));
    }
    
    // Zero pad to FFT size
    while buffer.len() < fft_size {
        buffer.push(Complex32::new(0.0, 0.0));
    }
    
    buffer
}

/// Apply Hamming window to samples
fn apply_hamming_window(buffer: &mut [Complex32], window_size: usize) {
    for i in 0..window_size {
        let window_value = 0.54 - 0.46 * (2.0 * std::f32::consts::PI * i as f32 
            / (window_size as f32 - 1.0)).cos();
        buffer[i] = buffer[i] * window_value;
    }
}

/// Convert FFT output to power spectrum in dB
fn compute_power_spectrum_db(buffer: &[Complex32], actual_samples: usize) -> Vec<f32> {
    let window_power: f32 = 0.397; // Hamming window power
    let scale = 1.0 / (actual_samples as f32 * window_power.sqrt());
    
    buffer.iter().map(|c| {
        let power = c.norm_sqr() * scale * scale;
        if power > 1e-10 {
            10.0 * power.log10()
        } else {
            -100.0
        }
    }).collect()
}

/// Extract DC-centered portion of spectrum
fn extract_centered_spectrum(spectrum: Vec<f32>, fft_size: usize) -> Vec<f32> {
    // FFT output: [0...fs/2, -fs/2...0]
    // We want [-fs/2...fs/2] centered view
    let half = spectrum.len() / 2;
    let mut centered = Vec::with_capacity(spectrum.len());
    centered.extend_from_slice(&spectrum[half..]);
    centered.extend_from_slice(&spectrum[..half]);
    
    // Return middle portion showing useful bandwidth (±32 Hz span)
    let bins_for_64hz = (64.0 * fft_size as f32 / 16.0) as usize;
    let center = centered.len() / 2;
    let start = center.saturating_sub(bins_for_64hz / 2);
    let end = (center + bins_for_64hz / 2).min(centered.len());
    
    centered[start..end].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_complex::Complex64;

    #[test]
    fn test_spectrum_computation() {
        // Create simple test signal
        let symbols: Vec<Complex64> = (0..128)
            .map(|i| Complex64::new((i as f64 * 0.1).cos(), (i as f64 * 0.1).sin()))
            .collect();
        
        let spectrum = compute_baseband_spectrum(&symbols);
        
        assert!(!spectrum.is_empty());
        // Check all values are in reasonable dB range
        for &val in &spectrum {
            assert!(val >= -100.0 && val <= 50.0);
        }
    }

    #[test]
    fn test_insufficient_samples_returns_empty() {
        let symbols = vec![Complex64::new(1.0, 0.0); 16];
        let spectrum = compute_baseband_spectrum(&symbols);
        assert!(spectrum.is_empty());
    }

    #[test]
    fn test_dc_tone_appears_at_center() {
        // DC tone should appear at center of spectrum
        let symbols = vec![Complex64::new(1.0, 0.0); 128];
        let spectrum = compute_baseband_spectrum(&symbols);
        
        assert!(!spectrum.is_empty());
        
        // Find peak
        let (max_idx, &max_val) = spectrum.iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap();
        
        // Peak should be near center
        let center_idx = spectrum.len() / 2;
        let distance_from_center = (max_idx as i32 - center_idx as i32).abs();
        assert!(distance_from_center < 10, "DC peak not near center");
        assert!(max_val > -10.0, "DC peak too weak");
    }
}
