//! Raman Whisper Protocol Definition
//! 
//! This module defines the canonical protocol specification that is shared
//! across all encoder, decoder, modulator, and demodulator components.
//! 
//! # Protocol Overview
//! 
//! **Physical Layer:**
//! - Carrier: 12 kHz audio tone
//! - QPSK modulation: 16 symbols/second (~20 Hz bandwidth)
//! - FSK layer: ±1 Hz frequency dithering at 1 bit/second
//! 
//! **Frame Structure:**
//! - Total: 128 QPSK symbols (256 bits, 8 seconds per frame)
//! - Sync: 16 symbols (0xA5A5A5A5 pattern)
//! - Target ID: 16 symbols (32 bits for device addressing)
//! - Command: 16 symbols (32 bits: opcode + frame counters)
//! - Payload: 64 symbols (128 bits of user data)
//! - ECC: 16 symbols (32 bits for error correction)

use num_complex::Complex64;
use std::f64::consts::FRAC_1_SQRT_2;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Command types for the protocol
/// 
/// These are the available command opcodes. Users reference these by string name
/// in configuration files, not by hex value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandType {
    /// Send data payload
    SendData,
    /// Request status information
    RequestStatus,
    /// Set a parameter value
    SetParameter,
    /// Get a parameter value
    GetParameter,
    /// Reset the system
    Reset,
    /// Data transfer operation
    DataTransfer,
}

impl CommandType {
    /// Convert command type to opcode
    pub fn to_opcode(self) -> u32 {
        match self {
            CommandType::SendData => 0x0001,
            CommandType::RequestStatus => 0x0002,
            CommandType::SetParameter => 0x0003,
            CommandType::GetParameter => 0x0004,
            CommandType::Reset => 0x0005,
            CommandType::DataTransfer => 0x0006,
        }
    }
    
    /// Convert opcode to command type
    pub fn from_opcode(opcode: u32) -> Option<Self> {
        match opcode {
            0x0001 => Some(CommandType::SendData),
            0x0002 => Some(CommandType::RequestStatus),
            0x0003 => Some(CommandType::SetParameter),
            0x0004 => Some(CommandType::GetParameter),
            0x0005 => Some(CommandType::Reset),
            0x0006 => Some(CommandType::DataTransfer),
            _ => None,
        }
    }
    
    /// Get string representation for user-facing API
    pub fn as_str(self) -> &'static str {
        match self {
            CommandType::SendData => "send_data",
            CommandType::RequestStatus => "request_status",
            CommandType::SetParameter => "set_parameter",
            CommandType::GetParameter => "get_parameter",
            CommandType::Reset => "reset",
            CommandType::DataTransfer => "data_transfer",
        }
    }
    
    /// Parse from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "send_data" => Some(CommandType::SendData),
            "request_status" => Some(CommandType::RequestStatus),
            "set_parameter" => Some(CommandType::SetParameter),
            "get_parameter" => Some(CommandType::GetParameter),
            "reset" => Some(CommandType::Reset),
            "data_transfer" => Some(CommandType::DataTransfer),
            _ => None,
        }
    }
}

impl fmt::Display for CommandType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Raman Whisper Protocol - Physical Layer Constants
/// 
/// These parameters define the modulation scheme and are NOT configurable at runtime.
/// They establish the fundamental communication channel characteristics.
pub struct PhysicalLayer;

impl PhysicalLayer {
    /// Audio carrier frequency in Hz
    pub const CARRIER_FREQ_HZ: f64 = 12000.0;
    
    /// QPSK symbol rate in symbols per second
    pub const SYMBOL_RATE: usize = 16;
    
    /// QPSK bandwidth in Hz (from RRC filter with α=0.25)
    pub const QPSK_BANDWIDTH_HZ: f64 = 20.0;
    
    /// FSK bit rate in bits per second
    pub const FSK_BIT_RATE: f64 = 1.0;
    
    /// FSK frequency shift in Hz (±1 Hz around carrier)
    pub const FSK_SHIFT_HZ: f64 = 1.0;
    
    /// RRC filter rolloff factor
    pub const RRC_ROLLOFF: f64 = 0.25;
    
    /// RRC filter span in symbols
    pub const RRC_FILTER_SPAN: usize = 4;
}

/// Frame structure layout in QPSK symbols
/// 
/// These define the bit-level structure of each transmitted frame.
/// Fixed for protocol compatibility.
pub struct FrameLayout;

impl FrameLayout {
    /// Total symbols per frame (8 seconds @ 16 sym/s)
    pub const TOTAL_SYMBOLS: usize = 128;
    
    /// Sync preamble symbols (known pattern for acquisition)
    pub const SYNC_SYMBOLS: usize = 16;
    
    /// Target ID field symbols (device addressing)
    pub const TARGET_ID_SYMBOLS: usize = 16;
    
    /// Command type field symbols (includes opcode and frame counters)
    pub const COMMAND_TYPE_SYMBOLS: usize = 16;
    
    /// Data payload symbols (user data)
    pub const DATA_PAYLOAD_SYMBOLS: usize = 64;
    
    /// ECC symbols (error correction)
    pub const ECC_SYMBOLS: usize = 16;
    
    /// Sync sequence as hex string (0xA5A5A5A5 = 10101010... pattern)
    pub const SYNC_SEQUENCE_HEX: &'static str = "A5A5A5A5";
    
    /// Bit shift for current frame number in command field
    pub const CURRENT_FRAME_SHIFT: u32 = 16;
    
    /// Bit shift for total frames count in command field
    pub const TOTAL_FRAMES_SHIFT: u32 = 24;
    
    /// Maximum number of frames per transmission
    pub const MAX_FRAMES: usize = 256;
    
    /// Get total bits per frame
    pub const fn total_bits() -> usize {
        Self::TOTAL_SYMBOLS * 2
    }
    
    /// Get sync bits length
    pub const fn sync_bits() -> usize {
        Self::SYNC_SYMBOLS * 2
    }
    
    /// Get target ID bits length
    pub const fn target_id_bits() -> usize {
        Self::TARGET_ID_SYMBOLS * 2
    }
    
    /// Get command bits length
    pub const fn command_bits() -> usize {
        Self::COMMAND_TYPE_SYMBOLS * 2
    }
    
    /// Get payload bits length
    pub const fn payload_bits() -> usize {
        Self::DATA_PAYLOAD_SYMBOLS * 2
    }
    
    /// Get ECC bits length
    pub const fn ecc_bits() -> usize {
        Self::ECC_SYMBOLS * 2
    }
}

/// QPSK Constellation Definition
/// 
/// Standard Gray-coded QPSK constellation at 45°, 135°, 225°, 315°.
/// This mapping MUST be identical in encoder and decoder.
pub struct QPSKConstellation;

impl QPSKConstellation {
    /// QPSK constellation points (Gray-coded)
    pub const POINTS: [(u8, u8, Complex64); 4] = [
        (1, 1, Complex64::new(FRAC_1_SQRT_2, FRAC_1_SQRT_2)),   // 45° - bits: 11
        (0, 1, Complex64::new(-FRAC_1_SQRT_2, FRAC_1_SQRT_2)),  // 135° - bits: 01
        (0, 0, Complex64::new(-FRAC_1_SQRT_2, -FRAC_1_SQRT_2)), // 225° - bits: 00
        (1, 0, Complex64::new(FRAC_1_SQRT_2, -FRAC_1_SQRT_2)),  // 315° - bits: 10
    ];
    
    /// Map two bits to QPSK symbol
    pub fn bits_to_symbol(b0: u8, b1: u8) -> Complex64 {
        match (b0, b1) {
            (1, 1) => Self::POINTS[0].2,
            (0, 1) => Self::POINTS[1].2,
            (0, 0) => Self::POINTS[2].2,
            (1, 0) => Self::POINTS[3].2,
            _ => panic!("Invalid bits: must be 0 or 1"),
        }
    }
    
    /// Map QPSK symbol to nearest constellation point and return bits
    pub fn symbol_to_bits(symbol: Complex64) -> (u8, u8) {
        let mut min_dist = f64::INFINITY;
        let mut best_bits = (0, 0);
        
        for &(b0, b1, point) in &Self::POINTS {
            let dist = (symbol - point).norm_sqr();
            if dist < min_dist {
                min_dist = dist;
                best_bits = (b0, b1);
            }
        }
        
        best_bits
    }
    
    /// Get all constellation points as a slice
    pub fn points() -> &'static [(u8, u8, Complex64)] {
        &Self::POINTS
    }
}

/// Frame field offsets in bits
/// 
/// Helper for parsing received frames
pub struct FrameOffsets;

impl FrameOffsets {
    pub const SYNC_START: usize = 0;
    pub const TARGET_ID_START: usize = FrameLayout::sync_bits();
    pub const COMMAND_START: usize = Self::TARGET_ID_START + FrameLayout::target_id_bits();
    pub const PAYLOAD_START: usize = Self::COMMAND_START + FrameLayout::command_bits();
    pub const ECC_START: usize = Self::PAYLOAD_START + FrameLayout::payload_bits();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_frame_layout_integrity() {
        // Verify frame structure adds up correctly
        let total = FrameLayout::SYNC_SYMBOLS
            + FrameLayout::TARGET_ID_SYMBOLS
            + FrameLayout::COMMAND_TYPE_SYMBOLS
            + FrameLayout::DATA_PAYLOAD_SYMBOLS
            + FrameLayout::ECC_SYMBOLS;
        
        assert_eq!(total, FrameLayout::TOTAL_SYMBOLS);
        assert_eq!(FrameLayout::total_bits(), 256);
    }
    
    #[test]
    fn test_frame_timing() {
        // Verify frame duration
        let frame_duration_sec = FrameLayout::TOTAL_SYMBOLS as f64 / PhysicalLayer::SYMBOL_RATE as f64;
        assert_eq!(frame_duration_sec, 8.0);
    }
    
    #[test]
    fn test_qpsk_constellation_roundtrip() {
        // Test that encoding and decoding are inverse operations
        for &(b0, b1, expected_symbol) in QPSKConstellation::points() {
            let symbol = QPSKConstellation::bits_to_symbol(b0, b1);
            assert!((symbol - expected_symbol).norm() < 1e-10);
            
            let (decoded_b0, decoded_b1) = QPSKConstellation::symbol_to_bits(symbol);
            assert_eq!(decoded_b0, b0);
            assert_eq!(decoded_b1, b1);
        }
    }
    
    #[test]
    fn test_qpsk_gray_coding() {
        // Gray coding means adjacent constellation points differ by only 1 bit
        let points = QPSKConstellation::points();
        
        // Check 45° -> 135° (should differ by 1 bit)
        let hamming_01 = (points[0].0 ^ points[1].0).count_ones() + (points[0].1 ^ points[1].1).count_ones();
        assert_eq!(hamming_01, 1);
        
        // Check 135° -> 225° (should differ by 1 bit)
        let hamming_12 = (points[1].0 ^ points[2].0).count_ones() + (points[1].1 ^ points[2].1).count_ones();
        assert_eq!(hamming_12, 1);
        
        // Check 225° -> 315° (should differ by 1 bit)
        let hamming_23 = (points[2].0 ^ points[3].0).count_ones() + (points[2].1 ^ points[3].1).count_ones();
        assert_eq!(hamming_23, 1);
        
        // Check 315° -> 45° (should differ by 1 bit)
        let hamming_30 = (points[3].0 ^ points[0].0).count_ones() + (points[3].1 ^ points[0].1).count_ones();
        assert_eq!(hamming_30, 1);
    }
    
    #[test]
    fn test_frame_offsets() {
        // Verify offsets are sequential and correct
        assert_eq!(FrameOffsets::SYNC_START, 0);
        assert_eq!(FrameOffsets::TARGET_ID_START, 32);
        assert_eq!(FrameOffsets::COMMAND_START, 64);
        assert_eq!(FrameOffsets::PAYLOAD_START, 96);
        assert_eq!(FrameOffsets::ECC_START, 224);
    }
}
