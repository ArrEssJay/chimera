//! Data buffer types for passing data between nodes

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Complex IQ sample
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct IQSample {
    pub i: f32,
    pub q: f32,
}

impl IQSample {
    pub fn new(i: f32, q: f32) -> Self {
        Self { i, q }
    }

    pub fn magnitude(&self) -> f32 {
        (self.i * self.i + self.q * self.q).sqrt()
    }

    pub fn phase(&self) -> f32 {
        self.q.atan2(self.i)
    }
}

/// Data buffer enum for passing data between nodes
///
/// This is the core data structure for node graph execution.
/// All data flowing through the graph must be one of these types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataBuffer {
    /// Sequence of bits (true = 1, false = 0)
    BitStream(Vec<bool>),

    /// Complex IQ samples for RF signals
    IQData(Vec<IQSample>),

    /// Audio samples (mono, normalized -1.0 to 1.0)
    AudioSamples(Vec<f32>),

    /// Metadata key-value pairs
    Metadata(HashMap<String, String>),
}

impl DataBuffer {
    /// Get the data type of this buffer
    pub fn data_type(&self) -> DataType {
        match self {
            DataBuffer::BitStream(_) => DataType::BitStream,
            DataBuffer::IQData(_) => DataType::IQData,
            DataBuffer::AudioSamples(_) => DataType::AudioSamples,
            DataBuffer::Metadata(_) => DataType::Metadata,
        }
    }

    /// Get the size/length of the buffer
    pub fn len(&self) -> usize {
        match self {
            DataBuffer::BitStream(v) => v.len(),
            DataBuffer::IQData(v) => v.len(),
            DataBuffer::AudioSamples(v) => v.len(),
            DataBuffer::Metadata(m) => m.len(),
        }
    }

    /// Check if buffer is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Supported data types for node connections
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataType {
    BitStream,
    IQData,
    AudioSamples,
    Metadata,
}

impl DataType {
    /// Check if two types are compatible for connection
    pub fn is_compatible_with(&self, other: &DataType) -> bool {
        self == other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iq_sample_magnitude() {
        let sample = IQSample::new(3.0, 4.0);
        assert_eq!(sample.magnitude(), 5.0);
    }

    #[test]
    fn test_iq_sample_phase() {
        let sample = IQSample::new(1.0, 1.0);
        assert!((sample.phase() - std::f32::consts::PI / 4.0).abs() < 0.001);
    }

    #[test]
    fn test_data_buffer_type() {
        let buf = DataBuffer::BitStream(vec![true, false, true]);
        assert_eq!(buf.data_type(), DataType::BitStream);
        assert_eq!(buf.len(), 3);
        assert!(!buf.is_empty());
    }

    #[test]
    fn test_data_type_compatibility() {
        assert!(DataType::BitStream.is_compatible_with(&DataType::BitStream));
        assert!(!DataType::BitStream.is_compatible_with(&DataType::IQData));
    }

    #[test]
    fn test_empty_buffer() {
        let buf = DataBuffer::BitStream(vec![]);
        assert!(buf.is_empty());
        assert_eq!(buf.len(), 0);
    }

    #[test]
    fn test_metadata_buffer() {
        let mut meta = HashMap::new();
        meta.insert("key".to_string(), "value".to_string());
        let buf = DataBuffer::Metadata(meta);
        assert_eq!(buf.data_type(), DataType::Metadata);
        assert_eq!(buf.len(), 1);
    }

    #[test]
    fn test_iq_data_buffer() {
        let samples = vec![IQSample::new(1.0, 0.0), IQSample::new(0.0, 1.0)];
        let buf = DataBuffer::IQData(samples);
        assert_eq!(buf.data_type(), DataType::IQData);
        assert_eq!(buf.len(), 2);
    }

    #[test]
    fn test_audio_samples_buffer() {
        let samples = vec![0.1, 0.2, 0.3, -0.1, -0.2];
        let buf = DataBuffer::AudioSamples(samples);
        assert_eq!(buf.data_type(), DataType::AudioSamples);
        assert_eq!(buf.len(), 5);
    }
}
