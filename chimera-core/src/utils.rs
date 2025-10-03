//! Utility helpers for bitstream conversions and logging.
use std::fmt;

use ndarray::Array1;
use num_complex::Complex64;

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
    assert!(expected_bits % 8 == 0, "expected_bits must be byte aligned");
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
