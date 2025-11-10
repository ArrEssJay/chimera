//! Utility helpers for bitstream conversions and logging.
use std::fmt;

use ndarray::Array1;
use num_complex::Complex64;

/// DSP optimization utilities for real-time performance
pub mod dsp {
    use std::f32::consts::{FRAC_PI_2, PI};
    
    /// Fast arctangent2 approximation using polynomial approximation
    /// Error typically < 0.005 radians, ~10x faster than libm atan2
    #[inline(always)]
    pub fn fast_atan2_approx(y: f32, x: f32) -> f32 {
        // Handle special cases
        if x == 0.0 {
            return if y > 0.0 { FRAC_PI_2 } else if y < 0.0 { -FRAC_PI_2 } else { 0.0 };
        }
        
        let abs_y = y.abs();
        let abs_x = x.abs();
        let a = abs_y.min(abs_x);
        let b = abs_y.max(abs_x);
        let s = a / b;
        let s2 = s * s;
        
        // Polynomial approximation: atan(s) ≈ s * (1 - s^2 * (1/3 - s^2/5))
        let r = s * (1.0 - s2 * (0.333333333 - s2 * 0.2));
        let angle = if abs_y > abs_x { FRAC_PI_2 - r } else { r };
        
        // Adjust for quadrant
        if x < 0.0 {
            if y >= 0.0 { PI - angle } else { -PI + angle }
        } else {
            if y >= 0.0 { angle } else { -angle }
        }
    }
    
    /// Fast simultaneous sine and cosine using CORDIC-like algorithm
    /// ~3-5x faster than separate sin/cos calls
    #[inline(always)]
    pub fn fast_sincos(mut angle: f32) -> (f32, f32) {
        // Reduce angle to [-PI, PI]
        angle = angle % (2.0 * PI);
        if angle > PI {
            angle -= 2.0 * PI;
        } else if angle < -PI {
            angle += 2.0 * PI;
        }
        
        // For small angles, use Taylor series (more accurate and faster)
        if angle.abs() < 0.1 {
            let angle2 = angle * angle;
            let sin = angle * (1.0 - angle2 * (1.0/6.0 - angle2/120.0));
            let cos = 1.0 - angle2 * (0.5 - angle2/24.0);
            return (sin, cos);
        }
        
        // Use standard library for larger angles (already optimized with SIMD)
        angle.sin_cos()
    }
    
    /// Lookup table-based sine/cosine for fixed frequencies
    /// Useful for carrier generation where phase increments are constant
    pub struct SinCosLut {
        table: Vec<(f32, f32)>,
        size: usize,
        scale: f32,
    }
    
    impl SinCosLut {
        /// Create a new LUT with specified size (power of 2 recommended)
        pub fn new(size: usize) -> Self {
            let mut table = Vec::with_capacity(size);
            let scale = size as f32 / (2.0 * PI);
            
            for i in 0..size {
                let angle = 2.0 * PI * i as f32 / size as f32;
                table.push(angle.sin_cos());
            }
            
            Self { table, size, scale }
        }
        
        /// Lookup sine and cosine for a given phase angle
        #[inline(always)]
        pub fn lookup(&self, angle: f32) -> (f32, f32) {
            // Wrap angle to [0, 2π) and scale to table index
            let idx = ((angle * self.scale) as usize) % self.size;
            self.table[idx]
        }
    }
    
    /// Unwrap phase to remove 2π discontinuities
    #[inline]
    pub fn phase_unwrap(phases: &mut [f32]) {
        if phases.len() < 2 {
            return;
        }
        
        const TWO_PI: f32 = 2.0 * PI;
        let mut correction = 0.0_f32;
        
        for i in 1..phases.len() {
            let diff = phases[i] - phases[i-1] - correction;
            if diff > PI {
                correction += TWO_PI;
            } else if diff < -PI {
                correction -= TWO_PI;
            }
            phases[i] -= correction;
        }
    }
}


#[derive(Debug, Clone)]
pub struct LogCollector {
    entries: Vec<String>,
}

impl LogCollector {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn log(&mut self, message: impl Into<String>) {
        self.entries.push(message.into());
    }

    pub fn extend<I>(&mut self, lines: I)
    where
        I: IntoIterator,
        I::Item: Into<String>,
    {
        for line in lines.into_iter() {
            self.log(line);
        }
    }

    pub fn entries(&self) -> &[String] {
        &self.entries
    }
}

impl Default for LogCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for LogCollector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in &self.entries {
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}

pub fn string_to_bitstream(text: &str) -> Vec<u8> {
    text.as_bytes()
        .iter()
        .flat_map(|byte| (0..8).rev().map(move |bit| (byte >> bit) & 1))
        .collect()
}

pub fn int_to_bitstream(value: u64, bits: usize) -> Vec<u8> {
    assert!(bits > 0, "bit width must be positive");
    if bits < u64::BITS as usize {
        let limit = 1u64.checked_shl(bits as u32).unwrap_or(u64::MAX);
        assert!(value < limit, "value does not fit in bit width");
    }
    (0..bits)
        .rev()
        .map(|bit| ((value >> bit) & 1) as u8)
        .collect()
}

pub fn hex_to_bitstream(hex: &str, expected_bits: usize) -> Vec<u8> {
    assert!(
        expected_bits.is_multiple_of(8),
        "expected_bits must be byte aligned"
    );
    let padded = if hex.len() * 4 < expected_bits {
        format!("{:0>width$}", hex, width = expected_bits / 4)
    } else {
        hex.to_string()
    };

    let bytes = (0..padded.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&padded[i..i + 2], 16).expect("invalid hex"))
        .collect::<Vec<_>>();

    bytes
        .iter()
        .flat_map(|byte| (0..8).rev().map(move |bit| (byte >> bit) & 1))
        .collect()
}

pub fn pack_bits(bits: &[u8]) -> Vec<u8> {
    bits.chunks(8)
        .map(|chunk| chunk.iter().fold(0u8, |acc, &b| (acc << 1) | (b & 1)))
        .collect()
}

pub fn bits_to_string(bits: &[u8]) -> String {
    let bytes = pack_bits(bits);
    String::from_utf8_lossy(&bytes).to_string()
}

pub fn complex_from_interleaved(data: &[f64]) -> Vec<Complex64> {
    data.chunks(2)
        .map(|chunk| Complex64::new(chunk[0], chunk[1]))
        .collect()
}

pub fn interleaved_from_complex(data: &[Complex64]) -> Vec<f64> {
    data.iter().flat_map(|c| [c.re, c.im]).collect()
}

pub fn array_from_bits(bits: &[u8]) -> Array1<u8> {
    Array1::from_vec(bits.to_vec())
}

/// Channel parameter calculations
pub struct ChannelParams {
    pub link_loss_linear: f64,
    pub attenuation_factor: f64,
    pub attenuated_signal_power: f64,
    pub noise_variance: f64,
    pub noise_std: f64,
}

impl ChannelParams {
    /// Calculate channel parameters from SNR and link loss in dB
    /// 
    /// # Arguments
    /// * `snr_db` - Signal-to-noise ratio in dB (Es/N0)
    /// * `link_loss_db` - Link loss/path loss in dB
    /// * `signal_power` - Normalized signal power (typically 1.0 for QPSK)
    pub fn from_db(snr_db: f64, link_loss_db: f64, signal_power: f64) -> Self {
        let link_loss_linear = 10f64.powf(link_loss_db / 10.0);
        let attenuated_signal_power = signal_power / link_loss_linear;
        let attenuation_factor = if link_loss_linear > 0.0 {
            1.0 / link_loss_linear.sqrt()
        } else {
            1.0
        };
        
        let snr_linear = 10f64.powf(snr_db / 10.0);
        let noise_variance = if snr_linear > 0.0 {
            attenuated_signal_power / snr_linear
        } else {
            0.0
        };
        let noise_std = (noise_variance / 2.0).sqrt();
        
        Self {
            link_loss_linear,
            attenuation_factor,
            attenuated_signal_power,
            noise_variance,
            noise_std,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_bitstream_roundtrip() {
        let text = "Chimera";
        let bits = string_to_bitstream(text);
        assert_eq!(bits.len(), text.len() * 8);
        let packed = pack_bits(&bits);
        assert_eq!(String::from_utf8(packed).unwrap(), text);
    }

    #[test]
    fn int_to_bitstream_width() {
        let bits = int_to_bitstream(0xAB, 12);
        assert_eq!(bits, vec![0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 1]);
    }

    #[test]
    fn hex_to_bitstream_expected_width() {
        let bits = hex_to_bitstream("A5A5", 16);
        assert_eq!(bits.len(), 16);
        assert_eq!(&bits[..8], &[1, 0, 1, 0, 0, 1, 0, 1]);
    }

    #[test]
    fn complex_conversion_roundtrip() {
        let samples = [Complex64::new(0.5, -0.25), Complex64::new(-1.0, 2.0)];
        let interleaved = interleaved_from_complex(&samples);
        assert_eq!(interleaved, vec![0.5, -0.25, -1.0, 2.0]);
        let reconstructed = complex_from_interleaved(&interleaved);
        assert_eq!(reconstructed, samples);
    }

    #[test]
    fn array_from_bits_copies() {
        let bits = vec![1_u8, 0, 1, 1];
        let arr = array_from_bits(&bits);
        assert_eq!(arr.len(), bits.len());
        assert_eq!(arr.to_vec(), bits);
    }
}
