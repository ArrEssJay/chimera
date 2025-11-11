//! External audio file loading and processing for intermodulation simulation
//!
//! Supports loading MP3, M4A, WAV, FLAC files and resampling to 48kHz
//! Also supports generating test signals (pink noise, tones, sweeps)

use crate::audio_generator::{generate_audio, GeneratorType};
use crate::config::{AudioSource, GeneratorPreset};
use crate::errors::{ChimeraError, Result};
use std::fs::File;
use std::path::Path;
use symphonia::core::audio::{AudioBufferRef, Signal};
use symphonia::core::codecs::{DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use rubato::{FastFixedIn, Resampler};

/// Load or generate audio based on AudioSource configuration
pub fn load_or_generate_audio(
    audio_source: &AudioSource,
    target_sample_rate: usize,
) -> Result<Vec<f32>> {
    match audio_source {
        AudioSource::None => Ok(Vec::new()),
        AudioSource::File { path, loop_audio: _ } => {
            load_audio_file(Path::new(path), target_sample_rate)
        }
        AudioSource::Generator { preset, duration_secs } => {
            let generator_type = match preset {
                GeneratorPreset::PinkNoise => GeneratorType::PinkNoise,
                GeneratorPreset::Tone1kHz => GeneratorType::Tone(1000.0),
                GeneratorPreset::Tone(freq_hz) => GeneratorType::Tone(*freq_hz),
                GeneratorPreset::SweepLinear => GeneratorType::SweepLinear,
                GeneratorPreset::SweepLog => GeneratorType::SweepLog,
            };
            Ok(generate_audio(generator_type, *duration_secs, target_sample_rate))
        }
    }
}

/// Load audio file and resample to target sample rate
pub fn load_audio_file(path: &Path, target_sample_rate: usize) -> Result<Vec<f32>> {
    // Open the media source
    let file = File::open(path).map_err(|e| ChimeraError::AudioLoadError {
        path: path.to_string_lossy().into_owned(),
        reason: format!("Failed to open file: {}", e),
    })?;
    
    let mss = MediaSourceStream::new(Box::new(file), Default::default());
    
    // Create a probe hint using the file extension
    let mut hint = Hint::new();
    if let Some(extension) = path.extension() {
        if let Some(ext_str) = extension.to_str() {
            hint.with_extension(ext_str);
        }
    }
    
    // Probe the media source
    let format_opts = FormatOptions::default();
    let metadata_opts = MetadataOptions::default();
    let decoder_opts = DecoderOptions::default();
    
    let probed = symphonia::default::get_probe()
        .format(&hint, mss, &format_opts, &metadata_opts)
        .map_err(|e| ChimeraError::AudioLoadError {
            path: path.to_string_lossy().into_owned(),
            reason: format!("Failed to probe format: {}", e),
        })?;
    
    let mut format = probed.format;
    
    // Find the first audio track
    let track = format
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
        .ok_or_else(|| ChimeraError::AudioLoadError {
            path: path.to_string_lossy().into_owned(),
            reason: "No audio track found".to_string(),
        })?;
    
    let track_id = track.id;
    let source_sample_rate = track.codec_params.sample_rate.ok_or_else(|| {
        ChimeraError::AudioLoadError {
            path: path.to_string_lossy().into_owned(),
            reason: "Unknown sample rate".to_string(),
        }
    })?;
    
    // Create a decoder for the track
    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &decoder_opts)
        .map_err(|e| ChimeraError::AudioLoadError {
            path: path.to_string_lossy().into_owned(),
            reason: format!("Failed to create decoder: {}", e),
        })?;
    
    // Decode all packets
    let mut samples = Vec::new();
    
    loop {
        let packet = match format.next_packet() {
            Ok(packet) => packet,
            Err(_) => break, // End of stream
        };
        
        // Skip packets not from our track
        if packet.track_id() != track_id {
            continue;
        }
        
        // Decode the packet
        match decoder.decode(&packet) {
            Ok(decoded) => {
                // Convert to mono f32
                let mono_samples = convert_to_mono_f32(&decoded);
                samples.extend_from_slice(&mono_samples);
            }
            Err(_) => continue, // Skip decode errors
        }
    }
    
    if samples.is_empty() {
        return Err(ChimeraError::AudioLoadError {
            path: path.to_string_lossy().into_owned(),
            reason: "No audio samples decoded".to_string(),
        });
    }
    
    // Resample if necessary
    if source_sample_rate as usize != target_sample_rate {
        resample_audio(&samples, source_sample_rate as usize, target_sample_rate)
    } else {
        Ok(samples)
    }
}

/// Convert audio buffer to mono f32 samples
fn convert_to_mono_f32(buffer: &AudioBufferRef) -> Vec<f32> {
    match buffer {
        AudioBufferRef::F32(buf) => {
            let channels = buf.spec().channels.count();
            let frames = buf.frames();
            let mut mono = Vec::with_capacity(frames);
            
            if channels == 1 {
                // Already mono
                mono.extend_from_slice(buf.chan(0));
            } else {
                // Mix down to mono by averaging channels
                for i in 0..frames {
                    let mut sum = 0.0;
                    for ch in 0..channels {
                        sum += buf.chan(ch)[i];
                    }
                    mono.push(sum / channels as f32);
                }
            }
            
            mono
        }
        AudioBufferRef::U8(buf) => {
            let channels = buf.spec().channels.count();
            let frames = buf.frames();
            let mut mono = Vec::with_capacity(frames);
            
            for i in 0..frames {
                let mut sum = 0.0;
                for ch in 0..channels {
                    // Convert u8 to f32: [0, 255] -> [-1.0, 1.0]
                    sum += (buf.chan(ch)[i] as f32 - 128.0) / 128.0;
                }
                mono.push(sum / channels as f32);
            }
            
            mono
        }
        AudioBufferRef::U16(buf) => {
            let channels = buf.spec().channels.count();
            let frames = buf.frames();
            let mut mono = Vec::with_capacity(frames);
            
            for i in 0..frames {
                let mut sum = 0.0;
                for ch in 0..channels {
                    // Convert u16 to f32: [0, 65535] -> [-1.0, 1.0]
                    sum += (buf.chan(ch)[i] as f32 - 32768.0) / 32768.0;
                }
                mono.push(sum / channels as f32);
            }
            
            mono
        }
        AudioBufferRef::U24(buf) => {
            let channels = buf.spec().channels.count();
            let frames = buf.frames();
            let mut mono = Vec::with_capacity(frames);
            
            for i in 0..frames {
                let mut sum = 0.0;
                for ch in 0..channels {
                    // Convert u24 to f32
                    let val = buf.chan(ch)[i].inner();
                    sum += (val as i32 - 8388608) as f32 / 8388608.0;
                }
                mono.push(sum / channels as f32);
            }
            
            mono
        }
        AudioBufferRef::U32(buf) => {
            let channels = buf.spec().channels.count();
            let frames = buf.frames();
            let mut mono = Vec::with_capacity(frames);
            
            for i in 0..frames {
                let mut sum = 0.0;
                for ch in 0..channels {
                    // Convert u32 to f32
                    sum += (buf.chan(ch)[i] as f32 - 2147483648.0) / 2147483648.0;
                }
                mono.push(sum / channels as f32);
            }
            
            mono
        }
        AudioBufferRef::S8(buf) => {
            let channels = buf.spec().channels.count();
            let frames = buf.frames();
            let mut mono = Vec::with_capacity(frames);
            
            for i in 0..frames {
                let mut sum = 0.0;
                for ch in 0..channels {
                    sum += buf.chan(ch)[i] as f32 / 128.0;
                }
                mono.push(sum / channels as f32);
            }
            
            mono
        }
        AudioBufferRef::S16(buf) => {
            let channels = buf.spec().channels.count();
            let frames = buf.frames();
            let mut mono = Vec::with_capacity(frames);
            
            for i in 0..frames {
                let mut sum = 0.0;
                for ch in 0..channels {
                    sum += buf.chan(ch)[i] as f32 / 32768.0;
                }
                mono.push(sum / channels as f32);
            }
            
            mono
        }
        AudioBufferRef::S24(buf) => {
            let channels = buf.spec().channels.count();
            let frames = buf.frames();
            let mut mono = Vec::with_capacity(frames);
            
            for i in 0..frames {
                let mut sum = 0.0;
                for ch in 0..channels {
                    let val = buf.chan(ch)[i].inner();
                    sum += val as f32 / 8388608.0;
                }
                mono.push(sum / channels as f32);
            }
            
            mono
        }
        AudioBufferRef::S32(buf) => {
            let channels = buf.spec().channels.count();
            let frames = buf.frames();
            let mut mono = Vec::with_capacity(frames);
            
            for i in 0..frames {
                let mut sum = 0.0;
                for ch in 0..channels {
                    sum += buf.chan(ch)[i] as f32 / 2147483648.0;
                }
                mono.push(sum / channels as f32);
            }
            
            mono
        }
        AudioBufferRef::F64(buf) => {
            let channels = buf.spec().channels.count();
            let frames = buf.frames();
            let mut mono = Vec::with_capacity(frames);
            
            for i in 0..frames {
                let mut sum = 0.0;
                for ch in 0..channels {
                    sum += buf.chan(ch)[i] as f32;
                }
                mono.push(sum / channels as f32);
            }
            
            mono
        }
    }
}

/// Resample audio from source to target sample rate
fn resample_audio(
    samples: &[f32],
    source_rate: usize,
    target_rate: usize,
) -> Result<Vec<f32>> {
    if samples.is_empty() {
        return Ok(Vec::new());
    }
    
    // Calculate chunk size for processing
    let chunk_size = source_rate;
    
    // Create resampler
    let mut resampler = FastFixedIn::<f32>::new(
        target_rate as f64 / source_rate as f64,
        1.0, // max relative ratio difference
        rubato::PolynomialDegree::Linear,
        chunk_size,
        1, // mono
    ).map_err(|e| ChimeraError::AudioLoadError {
        path: "resampler".to_string(),
        reason: format!("Failed to create resampler: {}", e),
    })?;
    
    let mut output = Vec::new();
    let mut input_buf = vec![Vec::new(); 1]; // Single channel
    
    // Process in chunks
    for chunk in samples.chunks(chunk_size) {
        input_buf[0].clear();
        input_buf[0].extend_from_slice(chunk);
        
        // Pad last chunk if needed
        if input_buf[0].len() < chunk_size {
            input_buf[0].resize(chunk_size, 0.0);
        }
        
        let out = resampler.process(&input_buf, None).map_err(|e| {
            ChimeraError::AudioLoadError {
                path: "resampler".to_string(),
                reason: format!("Resampling failed: {}", e),
            }
        })?;
        
        output.extend_from_slice(&out[0]);
    }
    
    Ok(output)
}

/// Prepare external audio for mixing - loop or truncate to match target length
pub fn prepare_audio_for_mixing(
    audio: &[f32],
    target_length: usize,
    loop_audio: bool,
) -> Vec<f32> {
    if audio.is_empty() {
        return vec![0.0; target_length];
    }
    
    if audio.len() == target_length {
        return audio.to_vec();
    }
    
    if audio.len() > target_length {
        // Truncate
        return audio[..target_length].to_vec();
    }
    
    // Audio is shorter than target
    if !loop_audio {
        // Pad with zeros
        let mut result = audio.to_vec();
        result.resize(target_length, 0.0);
        return result;
    }
    
    // Loop the audio
    let mut result = Vec::with_capacity(target_length);
    let full_loops = target_length / audio.len();
    let remainder = target_length % audio.len();
    
    for _ in 0..full_loops {
        result.extend_from_slice(audio);
    }
    if remainder > 0 {
        result.extend_from_slice(&audio[..remainder]);
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_prepare_audio_exact_match() {
        let audio = vec![1.0, 2.0, 3.0];
        let result = prepare_audio_for_mixing(&audio, 3, false);
        assert_eq!(result, audio);
    }
    
    #[test]
    fn test_prepare_audio_truncate() {
        let audio = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = prepare_audio_for_mixing(&audio, 3, false);
        assert_eq!(result, vec![1.0, 2.0, 3.0]);
    }
    
    #[test]
    fn test_prepare_audio_pad() {
        let audio = vec![1.0, 2.0];
        let result = prepare_audio_for_mixing(&audio, 5, false);
        assert_eq!(result, vec![1.0, 2.0, 0.0, 0.0, 0.0]);
    }
    
    #[test]
    fn test_prepare_audio_loop() {
        let audio = vec![1.0, 2.0];
        let result = prepare_audio_for_mixing(&audio, 5, true);
        assert_eq!(result, vec![1.0, 2.0, 1.0, 2.0, 1.0]);
    }
    
    #[test]
    fn test_prepare_audio_empty() {
        let audio: Vec<f32> = vec![];
        let result = prepare_audio_for_mixing(&audio, 3, false);
        assert_eq!(result, vec![0.0, 0.0, 0.0]);
    }
}
