//! Frame decoder with hex dump and human-readable field interpretation.

use crate::logging::{DecodedFrame, FrameDecodeEvent, FrameHexDump};
use chimera_core::config::InternalProtocolConfig;
use chrono::Utc;

/// Command opcodes and their descriptions
pub struct CommandOpcode;

impl CommandOpcode {
    pub fn describe(opcode: u32) -> (&'static str, &'static str) {
        match opcode {
            0x0001 => ("STATUS_QUERY", "Request status telemetry"),
            0x0002 => ("SET_MODE", "Configure operating mode"),
            0x0003 => ("DATA_TRANSFER", "Bulk data payload"),
            0x0004 => ("RESET", "System reset command"),
            0x00F1 => ("BURST_TELEMETRY", "High-rate telemetry burst"),
            0x0D11 => ("DEEP_SPACE_BEACON", "Deep-space probe beacon"),
            _ => ("UNKNOWN", "Unrecognized command"),
        }
    }
}

/// Target IDs and their descriptions
pub struct TargetId;

impl TargetId {
    pub fn describe(hex: &str) -> &'static str {
        match hex.to_uppercase().as_str() {
            "DEADBEEF" => "Raman Whisper Station",
            "CAFEBABE" => "Burst Telemetry Node",
            "0D15EA5E" => "Deep-Space Probe",
            _ => "Unknown Target",
        }
    }
}

/// Decode a frame into hex dump and human-readable format
pub struct FrameDecoder {
    protocol: InternalProtocolConfig,
}

impl FrameDecoder {
    pub fn new(protocol: InternalProtocolConfig) -> Self {
        Self { protocol }
    }
    
    /// Decode a complete frame buffer into structured event
    pub fn decode_frame(&self, frame_index: usize, frame_bits: &[u8]) -> FrameDecodeEvent {
        let layout = &self.protocol.frame_layout;
        
        // Extract fields (each symbol = 2 bits)
        let mut bit_offset = 0;
        
        // Sync sequence
        let sync_bits = &frame_bits[bit_offset..bit_offset + layout.sync_symbols * 2];
        let sync_hex = bits_to_hex(sync_bits);
        bit_offset += layout.sync_symbols * 2;
        
        // Target ID
        let target_bits = &frame_bits[bit_offset..bit_offset + layout.target_id_symbols * 2];
        let target_hex = bits_to_hex(target_bits);
        bit_offset += layout.target_id_symbols * 2;
        
        // Command type
        let command_bits = &frame_bits[bit_offset..bit_offset + layout.command_type_symbols * 2];
        let command_hex = bits_to_hex(command_bits);
        bit_offset += layout.command_type_symbols * 2;
        
        // Data payload
        let payload_bits = &frame_bits[bit_offset..bit_offset + layout.data_payload_symbols * 2];
        let payload_hex = bits_to_hex(payload_bits);
        let payload_preview = bits_to_ascii_preview(payload_bits, 32);
        bit_offset += layout.data_payload_symbols * 2;
        
        // ECC
        let ecc_bits = &frame_bits[bit_offset..bit_offset + layout.ecc_symbols * 2];
        let ecc_hex = bits_to_hex(ecc_bits);
        
        // Create hex dump
        let hex_dump = FrameHexDump {
            sync_sequence: format_hex_field(&sync_hex),
            target_id: format_hex_field(&target_hex),
            command_type: format_hex_field(&command_hex),
            payload: format_hex_field(&payload_hex),
            ecc: format_hex_field(&ecc_hex),
        };
        
        // Decode fields
        let command_opcode = self.protocol.get_command_opcode();
        let (opcode_name, opcode_desc) = CommandOpcode::describe(command_opcode);
        let target_name = TargetId::describe(&target_hex);
        
        let decoded = DecodedFrame {
            target_name: target_name.to_string(),
            command_opcode: format!("0x{:04X} ({})", command_opcode, opcode_name),
            command_description: opcode_desc.to_string(),
            payload_preview,
        };
        
        FrameDecodeEvent {
            timestamp: Utc::now(),
            frame_index,
            hex_dump,
            decoded,
        }
    }
}

/// Convert bit array to hex string
fn bits_to_hex(bits: &[u8]) -> String {
    let mut hex = String::new();
    for chunk in bits.chunks(8) {
        let mut byte = 0u8;
        for (i, &bit) in chunk.iter().enumerate() {
            byte |= bit << (7 - i);
        }
        hex.push_str(&format!("{:02X}", byte));
    }
    hex
}

/// Convert bit array to u32 (MSB first)
fn bits_to_u32(bits: &[u8]) -> u32 {
    let mut value = 0u32;
    for (i, &bit) in bits.iter().enumerate().take(32) {
        value |= (bit as u32) << (31 - i);
    }
    value
}

/// Convert bits to ASCII preview (show printable chars, . for others)
fn bits_to_ascii_preview(bits: &[u8], max_chars: usize) -> String {
    let mut preview = String::new();
    for chunk in bits.chunks(8).take(max_chars) {
        let mut byte = 0u8;
        for (i, &bit) in chunk.iter().enumerate() {
            byte |= bit << (7 - i);
        }
        if byte >= 32 && byte <= 126 {
            preview.push(byte as char);
        } else {
            preview.push('.');
        }
    }
    preview
}

/// Format hex with spaces every 4 characters for readability
fn format_hex_field(hex: &str) -> String {
    hex.chars()
        .collect::<Vec<_>>()
        .chunks(4)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_bits_to_hex() {
        let bits = vec![1, 0, 1, 0, 0, 1, 0, 1];
        assert_eq!(bits_to_hex(&bits), "A5");
        
        let bits = vec![1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0];
        assert_eq!(bits_to_hex(&bits), "F0AA");
    }
    
    #[test]
    fn test_format_hex_field() {
        assert_eq!(format_hex_field("DEADBEEF"), "DEAD BEEF");
        assert_eq!(format_hex_field("A5A5A5A5"), "A5A5 A5A5");
    }
}
