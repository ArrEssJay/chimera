//! Signal Analysis Helper Functions
//!
//! Common utilities for DSP test measurements and analysis

use num_complex::Complex;
use rustfft::{FftPlanner, num_complex::Complex as FftComplex};
use std::f64::consts::TAU;

/// Estimate dominant frequency from time-domain signal using FFT peak
pub fn estimate_frequency(signal: &[f32], sample_rate: f32) -> f32 {
    let len = signal.len();
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(len);
    
    let mut buffer: Vec<FftComplex<f32>> = signal
        .iter()
        .map(|&x| FftComplex::new(x, 0.0))
        .collect();
    
    fft.process(&mut buffer);
    
    // Find peak magnitude (skip DC bin)
    let magnitudes: Vec<f32> = buffer[1..len/2]
        .iter()
        .map(|c| c.norm())
        .collect();
    
    let peak_bin = magnitudes
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(idx, _)| idx + 1)
        .unwrap_or(1);
    
    (peak_bin as f32) * sample_rate / (len as f32)
}

/// Measure signal power in dB (10*log10 of mean square)
pub fn measure_power_db(signal: &[f32]) -> f32 {
    if signal.is_empty() {
        return -std::f32::INFINITY;
    }
    
    let mean_square: f32 = signal.iter().map(|&x| x * x).sum::<f32>() / signal.len() as f32;
    
    if mean_square > 0.0 {
        10.0 * mean_square.log10()
    } else {
        -std::f32::INFINITY
    }
}

/// Compute Total Harmonic Distortion (THD)
/// Returns ratio of harmonic power to fundamental power in dB
pub fn compute_thd(signal: &[f32], fundamental_freq: f32, sample_rate: f32) -> f32 {
    let len = signal.len();
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(len);
    
    let mut buffer: Vec<FftComplex<f32>> = signal
        .iter()
        .map(|&x| FftComplex::new(x, 0.0))
        .collect();
    
    fft.process(&mut buffer);
    
    let freq_resolution = sample_rate / len as f32;
    let fundamental_bin = (fundamental_freq / freq_resolution).round() as usize;
    
    // Measure fundamental power (sum bins around fundamental ±2 bins)
    let fundamental_power: f32 = (fundamental_bin.saturating_sub(2)..=(fundamental_bin + 2).min(len/2))
        .map(|i| buffer[i].norm_sqr())
        .sum();
    
    // Measure harmonic power (2nd through 6th harmonics)
    let mut harmonic_power = 0.0f32;
    for harmonic in 2..=6 {
        let harmonic_bin = fundamental_bin * harmonic;
        if harmonic_bin >= len/2 {
            break;
        }
        
        // Sum bins around harmonic ±2 bins
        harmonic_power += (harmonic_bin.saturating_sub(2)..=(harmonic_bin + 2).min(len/2))
            .map(|i| buffer[i].norm_sqr())
            .sum::<f32>();
    }
    
    if fundamental_power > 0.0 && harmonic_power > 0.0 {
        10.0 * (harmonic_power / fundamental_power).log10()
    } else {
        -std::f32::INFINITY
    }
}

/// Check phase continuity - returns true if no large phase jumps detected
pub fn check_phase_continuity(signal: &[f32], threshold_rad: f32) -> bool {
    if signal.len() < 2 {
        return true;
    }
    
    for i in 1..signal.len() {
        let delta = (signal[i] - signal[i-1]).abs();
        if delta > threshold_rad {
            return false;
        }
    }
    
    true
}

/// Compute Power Spectral Density
pub fn compute_psd(signal: &[f32], sample_rate: f32) -> (Vec<f32>, Vec<f32>) {
    let len = signal.len();
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(len);
    
    let mut buffer: Vec<FftComplex<f32>> = signal
        .iter()
        .map(|&x| FftComplex::new(x, 0.0))
        .collect();
    
    fft.process(&mut buffer);
    
    let freqs: Vec<f32> = (0..len/2)
        .map(|i| (i as f32) * sample_rate / (len as f32))
        .collect();
    
    let psd: Vec<f32> = buffer[0..len/2]
        .iter()
        .map(|c| c.norm_sqr() / len as f32)
        .collect();
    
    (freqs, psd)
}

/// Measure bandwidth at given dB level below peak
pub fn measure_bandwidth(
    freqs: &[f32],
    psd: &[f32],
    center_freq: f32,
    db_below_peak: f32,
) -> f32 {
    if freqs.is_empty() || psd.is_empty() {
        return 0.0;
    }
    
    // Find peak power
    let peak_power = psd.iter().cloned().fold(0.0f32, f32::max);
    let threshold_power = peak_power / 10.0f32.powf(db_below_peak / 10.0);
    
    // Find center bin
    let center_bin = freqs
        .iter()
        .enumerate()
        .min_by_key(|(_, &f)| ((f - center_freq).abs() * 1000.0) as i32)
        .map(|(i, _)| i)
        .unwrap_or(0);
    
    // Find lower edge
    let mut lower_bin = center_bin;
    for i in (0..center_bin).rev() {
        if psd[i] < threshold_power {
            lower_bin = i;
            break;
        }
    }
    
    // Find upper edge
    let mut upper_bin = center_bin;
    for i in center_bin..freqs.len() {
        if psd[i] < threshold_power {
            upper_bin = i;
            break;
        }
    }
    
    freqs[upper_bin] - freqs[lower_bin]
}

/// Measure peak amplitude of signal
pub fn measure_peak_amplitude(signal: &[f32]) -> f32 {
    signal.iter().map(|&x| x.abs()).fold(0.0f32, f32::max)
}

/// Measure RMS amplitude
pub fn measure_rms_amplitude(signal: &[f32]) -> f32 {
    if signal.is_empty() {
        return 0.0;
    }
    
    let mean_square: f32 = signal.iter().map(|&x| x * x).sum::<f32>() / signal.len() as f32;
    mean_square.sqrt()
}

/// Compute SNR from signal and noise samples
pub fn compute_snr_db(signal: &[f32], noisy_signal: &[f32]) -> f32 {
    if signal.len() != noisy_signal.len() || signal.is_empty() {
        return -std::f32::INFINITY;
    }
    
    let signal_power: f32 = signal.iter().map(|&x| x * x).sum::<f32>() / signal.len() as f32;
    
    let noise_power: f32 = signal.iter().zip(noisy_signal.iter())
        .map(|(&s, &n)| {
            let noise = n - s;
            noise * noise
        })
        .sum::<f32>() / signal.len() as f32;
    
    if signal_power > 0.0 && noise_power > 0.0 {
        10.0 * (signal_power / noise_power).log10()
    } else {
        -std::f32::INFINITY
    }
}

/// Compute kurtosis (4th standardized moment) for Gaussian distribution test
pub fn compute_kurtosis(samples: &[f32]) -> f32 {
    if samples.len() < 4 {
        return 0.0;
    }
    
    let n = samples.len() as f32;
    let mean = samples.iter().sum::<f32>() / n;
    
    let variance = samples.iter()
        .map(|&x| (x - mean).powi(2))
        .sum::<f32>() / n;
    
    if variance == 0.0 {
        return 0.0;
    }
    
    let fourth_moment = samples.iter()
        .map(|&x| (x - mean).powi(4))
        .sum::<f32>() / n;
    
    fourth_moment / (variance * variance) - 3.0
}
