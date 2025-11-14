/**
 * Chimera Audio Loader
 * 
 * Universal audio file loading for both browser and Node.js environments.
 * Supports WAV, MP3, M4A, and other common formats.
 * 
 * Browser: Uses Web Audio API (native MP3/M4A support)
 * Node.js: Uses ffmpeg for decoding (must be installed)
 */

import { spawn } from 'child_process';
import { existsSync } from 'fs';
import { readFile } from 'fs/promises';

/**
 * Detect if running in browser or Node.js
 */
const isBrowser = typeof window !== 'undefined' && typeof window.AudioContext !== 'undefined';

/**
 * Check if ffmpeg is available (Node.js only)
 * @returns {Promise<boolean>}
 */
async function checkFfmpegAvailable() {
  if (isBrowser) return false;
  
  return new Promise((resolve) => {
    const ffmpeg = spawn('ffmpeg', ['-version']);
    ffmpeg.on('error', () => resolve(false));
    ffmpeg.on('close', (code) => resolve(code === 0));
  });
}

/**
 * Load audio file in browser using Web Audio API
 * @param {File|Blob|ArrayBuffer|string} source - Audio source (File, Blob, ArrayBuffer, or URL)
 * @param {AudioContext} audioContext - Web Audio context
 * @returns {Promise<Float32Array>} Decoded audio samples (mono)
 */
async function loadAudioBrowser(source, audioContext) {
  let arrayBuffer;
  
  if (source instanceof ArrayBuffer) {
    arrayBuffer = source;
  } else if (source instanceof Blob || source instanceof File) {
    arrayBuffer = await source.arrayBuffer();
  } else if (typeof source === 'string') {
    // Fetch from URL
    const response = await fetch(source);
    if (!response.ok) {
      throw new Error(`Failed to fetch audio: ${response.statusText}`);
    }
    arrayBuffer = await response.arrayBuffer();
  } else {
    throw new Error('Invalid audio source type');
  }
  
  // Decode audio data (supports MP3, M4A, WAV, etc.)
  const audioBuffer = await audioContext.decodeAudioData(arrayBuffer);
  
  // Convert to mono if needed
  let samples;
  if (audioBuffer.numberOfChannels === 1) {
    samples = audioBuffer.getChannelData(0);
  } else {
    // Mix down to mono
    const numSamples = audioBuffer.length;
    samples = new Float32Array(numSamples);
    for (let i = 0; i < numSamples; i++) {
      let sum = 0;
      for (let ch = 0; ch < audioBuffer.numberOfChannels; ch++) {
        sum += audioBuffer.getChannelData(ch)[i];
      }
      samples[i] = sum / audioBuffer.numberOfChannels;
    }
  }
  
  return {
    samples,
    sampleRate: audioBuffer.sampleRate,
    duration: audioBuffer.duration,
    numberOfChannels: audioBuffer.numberOfChannels
  };
}

/**
 * Load audio file in Node.js using ffmpeg
 * @param {string} filePath - Path to audio file
 * @param {number} targetSampleRate - Target sample rate (default: 48000)
 * @returns {Promise<Float32Array>} Decoded audio samples (mono)
 */
async function loadAudioNode(filePath, targetSampleRate = 48000) {
  if (!existsSync(filePath)) {
    throw new Error(`Audio file not found: ${filePath}`);
  }
  
  const hasFFmpeg = await checkFfmpegAvailable();
  if (!hasFFmpeg) {
    throw new Error('ffmpeg not found. Please install: brew install ffmpeg (macOS) or apt-get install ffmpeg (Linux)');
  }
  
  return new Promise((resolve, reject) => {
    const chunks = [];
    
    // Use ffmpeg to decode to raw PCM
    const ffmpeg = spawn('ffmpeg', [
      '-i', filePath,           // Input file
      '-f', 's16le',            // Output format: signed 16-bit little-endian PCM
      '-acodec', 'pcm_s16le',   // Audio codec
      '-ar', targetSampleRate.toString(), // Sample rate
      '-ac', '1',               // Mono
      'pipe:1'                  // Output to stdout
    ]);
    
    ffmpeg.stdout.on('data', (chunk) => {
      chunks.push(chunk);
    });
    
    ffmpeg.stderr.on('data', (data) => {
      // ffmpeg writes progress to stderr, we can ignore most of it
      // Only log actual errors
      const message = data.toString();
      if (message.includes('Error') || message.includes('Invalid')) {
        console.error('FFmpeg error:', message);
      }
    });
    
    ffmpeg.on('close', (code) => {
      if (code !== 0) {
        reject(new Error(`ffmpeg exited with code ${code}`));
        return;
      }
      
      // Concatenate all chunks
      const buffer = Buffer.concat(chunks);
      
      // Convert 16-bit PCM to Float32Array
      const numSamples = buffer.length / 2; // 2 bytes per sample
      const samples = new Float32Array(numSamples);
      
      for (let i = 0; i < numSamples; i++) {
        const int16 = buffer.readInt16LE(i * 2);
        samples[i] = int16 / 32768.0; // Normalize to -1.0 to 1.0
      }
      
      resolve({
        samples,
        sampleRate: targetSampleRate,
        duration: numSamples / targetSampleRate,
        numberOfChannels: 1
      });
    });
    
    ffmpeg.on('error', (err) => {
      reject(new Error(`Failed to spawn ffmpeg: ${err.message}`));
    });
  });
}

/**
 * Simple WAV file reader (fallback for when ffmpeg is not available)
 * Only supports uncompressed PCM WAV files
 * @param {string} filePath - Path to WAV file
 * @returns {Promise<Object>} Audio data
 */
async function loadWavFile(filePath) {
  const buffer = await readFile(filePath);
  
  // Parse WAV header
  const riff = buffer.toString('ascii', 0, 4);
  if (riff !== 'RIFF') {
    throw new Error('Not a valid WAV file');
  }
  
  const wave = buffer.toString('ascii', 8, 12);
  if (wave !== 'WAVE') {
    throw new Error('Not a valid WAV file');
  }
  
  // Find fmt and data chunks
  let offset = 12;
  let dataOffset = 0;
  let dataSize = 0;
  let sampleRate = 44100;
  let numChannels = 1;
  let bitsPerSample = 16;
  let audioFormat = 1;
  
  while (offset < buffer.length) {
    const chunkId = buffer.toString('ascii', offset, offset + 4);
    const chunkSize = buffer.readUInt32LE(offset + 4);
    
    if (chunkId === 'fmt ') {
      audioFormat = buffer.readUInt16LE(offset + 8);
      numChannels = buffer.readUInt16LE(offset + 10);
      sampleRate = buffer.readUInt32LE(offset + 12);
      bitsPerSample = buffer.readUInt16LE(offset + 22);
    } else if (chunkId === 'data') {
      dataOffset = offset + 8;
      dataSize = chunkSize;
      break;
    }
    
    offset += 8 + chunkSize;
  }
  
  if (dataOffset === 0) {
    throw new Error('No data chunk found in WAV file');
  }
  
  if (audioFormat !== 1) {
    throw new Error('Only uncompressed PCM WAV files are supported without ffmpeg');
  }
  
  // Read samples
  const bytesPerSample = bitsPerSample / 8;
  const numSamples = Math.floor(dataSize / (bytesPerSample * numChannels));
  const samples = new Float32Array(numSamples);
  
  for (let i = 0; i < numSamples; i++) {
    let sum = 0;
    
    for (let ch = 0; ch < numChannels; ch++) {
      const sampleOffset = dataOffset + (i * numChannels + ch) * bytesPerSample;
      let value = 0;
      
      if (bitsPerSample === 16) {
        value = buffer.readInt16LE(sampleOffset) / 32768.0;
      } else if (bitsPerSample === 8) {
        value = (buffer.readUInt8(sampleOffset) - 128) / 128.0;
      } else if (bitsPerSample === 24) {
        // 24-bit is stored as 3 bytes
        const byte1 = buffer.readUInt8(sampleOffset);
        const byte2 = buffer.readUInt8(sampleOffset + 1);
        const byte3 = buffer.readInt8(sampleOffset + 2);
        const int24 = (byte3 << 16) | (byte2 << 8) | byte1;
        value = int24 / 8388608.0;
      } else if (bitsPerSample === 32) {
        value = buffer.readInt32LE(sampleOffset) / 2147483648.0;
      }
      
      sum += value;
    }
    
    samples[i] = sum / numChannels; // Mix to mono
  }
  
  return {
    samples,
    sampleRate,
    duration: numSamples / sampleRate,
    numberOfChannels: numChannels
  };
}

/**
 * Universal audio loader - works in both browser and Node.js
 * @param {string|File|Blob|ArrayBuffer} source - Audio source
 * @param {Object} options - Options
 * @param {AudioContext} options.audioContext - Web Audio context (browser only)
 * @param {number} options.targetSampleRate - Target sample rate for Node.js (default: 48000)
 * @returns {Promise<Object>} Audio data with samples, sampleRate, duration, numberOfChannels
 */
export async function loadAudio(source, options = {}) {
  const { audioContext, targetSampleRate = 48000 } = options;
  
  if (isBrowser) {
    if (!audioContext) {
      throw new Error('AudioContext required for browser environment');
    }
    return await loadAudioBrowser(source, audioContext);
  } else {
    // Node.js environment
    if (typeof source !== 'string') {
      throw new Error('File path required for Node.js environment');
    }
    
    const extension = source.toLowerCase().split('.').pop();
    
    // Try ffmpeg first for all formats
    try {
      return await loadAudioNode(source, targetSampleRate);
    } catch (err) {
      // If ffmpeg fails and it's a WAV file, try simple WAV parser
      if (extension === 'wav') {
        console.warn('ffmpeg not available, using simple WAV parser (uncompressed PCM only)');
        return await loadWavFile(source);
      }
      throw err;
    }
  }
}

/**
 * Resample audio to a different sample rate (simple linear interpolation)
 * @param {Float32Array} samples - Input samples
 * @param {number} fromRate - Original sample rate
 * @param {number} toRate - Target sample rate
 * @returns {Float32Array} Resampled audio
 */
export function resampleAudio(samples, fromRate, toRate) {
  if (fromRate === toRate) return samples;
  
  const ratio = fromRate / toRate;
  const newLength = Math.floor(samples.length / ratio);
  const resampled = new Float32Array(newLength);
  
  for (let i = 0; i < newLength; i++) {
    const srcIndex = i * ratio;
    const srcIndexFloor = Math.floor(srcIndex);
    const srcIndexCeil = Math.min(srcIndexFloor + 1, samples.length - 1);
    const t = srcIndex - srcIndexFloor;
    
    resampled[i] = samples[srcIndexFloor] * (1 - t) + samples[srcIndexCeil] * t;
  }
  
  return resampled;
}

/**
 * Check if ffmpeg is available (Node.js only)
 * @returns {Promise<boolean>}
 */
export { checkFfmpegAvailable };

/**
 * Get supported formats for current environment
 * @returns {Array<string>} Array of supported extensions
 */
export function getSupportedFormats() {
  if (isBrowser) {
    // Browser supports most formats via Web Audio API
    return ['wav', 'mp3', 'm4a', 'aac', 'ogg', 'webm', 'flac'];
  } else {
    // Node.js depends on ffmpeg availability
    return ['wav', 'mp3', 'm4a', 'aac', 'ogg', 'flac', 'wma', 'aiff'];
  }
}

export default {
  loadAudio,
  resampleAudio,
  checkFfmpegAvailable,
  getSupportedFormats
};
