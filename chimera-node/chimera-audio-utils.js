/**
 * Chimera Audio Utilities
 * 
 * Utility functions for audio file generation and rendering
 */

import pkg from 'wavefile';
const { WaveFile } = pkg;
import { writeFileSync } from 'fs';

/**
 * Frame timing constants from HCI specification
 * - Symbol rate: 16 Hz (16 symbols per second)
 * - Frame size: 128 symbols
 * - Frame duration: 128 / 16 = 8 seconds per frame
 */
export const FRAME_CONFIG = {
  SYMBOL_RATE: 16,           // Hz - from HCI spec
  SYMBOLS_PER_FRAME: 128,    // symbols - from frame structure spec
  FRAME_DURATION: 8.0,       // seconds (128 symbols / 16 Hz)
};

/**
 * Render oscillator to WAV file
 * Generates audio for a specified number of complete frames
 * 
 * @param {ChimeraOscillator} oscillator - The oscillator to render
 * @param {string} filename - Output filename
 * @param {Object} options - Rendering options
 * @param {number|Array<ChimeraFrame>} options.frames - Number of frames to render (default: 1) OR array of ChimeraFrame objects to render sequentially
 * @param {number} options.sampleRate - Sample rate in Hz (default: 48000)
 * @param {string} options.bitDepth - Bit depth ('32f' for 32-bit float, default: '32f')
 * @param {boolean} options.verbose - Log rendering details (default: true)
 * @returns {Object} Rendering statistics
 */
export function renderToWav(oscillator, filename, options = {}) {
  const {
    frames = 1,
    sampleRate = 48000,
    bitDepth = '32f',
    verbose = true
  } = options;

  // Handle both frame arrays and frame counts
  const isFrameArray = Array.isArray(frames);
  const numFrames = isFrameArray ? frames.length : frames;

  // Calculate duration based on number of complete frames
  const duration = numFrames * FRAME_CONFIG.FRAME_DURATION;
  const numSamples = Math.floor(sampleRate * duration);
  const samplesPerFrame = Math.floor(sampleRate * FRAME_CONFIG.FRAME_DURATION);

  if (verbose) {
    console.log(`\nRendering: ${filename}`);
    console.log(`  Frames: ${numFrames} (${FRAME_CONFIG.FRAME_DURATION}s each)`);
    console.log(`  Mode: ${isFrameArray ? 'Frame Array (sequential)' : 'Single Frame (repeated)'}`);
    console.log(`  Duration: ${duration.toFixed(1)}s`);
    console.log(`  Sample Rate: ${sampleRate} Hz`);
    console.log(`  Bit Depth: ${bitDepth}`);
    console.log(`  Total Samples: ${numSamples.toLocaleString()}`);
  }

  // Generate samples
  const startTime = Date.now();
  let samples;

  if (isFrameArray && numFrames > 1) {
    // Multi-frame rendering: concatenate samples from each frame
    const allSamples = new Float32Array(numSamples);
    let offset = 0;

    for (let i = 0; i < frames.length; i++) {
      oscillator.loadFrame(frames[i]);
      
      if (verbose) {
        const params = oscillator.getParams();
        console.log(`  Frame ${i + 1}/${numFrames}: FSK ${params.fskState}, Freq ${params.freqLFO.frequency}Hz ${params.freqLFO.shape}, Amp ${params.ampLFO.frequency}Hz ${params.ampLFO.shape}`);
      }
      
      const frameStartTime = i * FRAME_CONFIG.FRAME_DURATION;
      const frameSamples = oscillator.generateSamples(samplesPerFrame, sampleRate, frameStartTime);
      allSamples.set(frameSamples, offset);
      offset += samplesPerFrame;
    }

    samples = allSamples;
  } else {
    // Single frame rendering (original behavior)
    if (verbose) {
      const params = oscillator.getParams();
      console.log(`  FSK State: ${params.fskState} (${params.frequency} Hz)`);
      console.log(`  Freq LFO: ${params.freqLFO.frequency}Hz ${params.freqLFO.shape} depth:${params.freqLFO.depth}`);
      console.log(`  Amp LFO: ${params.ampLFO.frequency}Hz ${params.ampLFO.shape} depth:${params.ampLFO.depth}`);
    }
    
    samples = oscillator.generateSamples(numSamples, sampleRate);
  }

  const generationTime = Date.now() - startTime;

  // Post-generation normalization to prevent clipping
  let maxPeak = 0;
  for (let i = 0; i < samples.length; i++) {
    const peak = Math.abs(samples[i]);
    if (peak > maxPeak) {
      maxPeak = peak;
    }
  }
  
  const TARGET_PEAK_DB = -1.0;
  const TARGET_PEAK_AMPLITUDE = Math.pow(10, TARGET_PEAK_DB / 20);
  let gain = 1.0;

  if (maxPeak > TARGET_PEAK_AMPLITUDE) {
    gain = TARGET_PEAK_AMPLITUDE / maxPeak;
    for (let i = 0; i < samples.length; i++) {
      samples[i] *= gain;
    }
    if (verbose) {
      console.log(`  Normalization Applied: peak was ${maxPeak.toFixed(4)}, gain reduced by ${(gain).toFixed(2)}x`);
    }
  }

  // Create WAV file
  const wav = new WaveFile();
  wav.fromScratch(1, sampleRate, bitDepth, samples);

  // Write to file
  writeFileSync(filename, wav.toBuffer());

  if (verbose) {
    console.log(`  ✓ Generated in ${generationTime}ms`);
    console.log(`  ✓ Written to ${filename}\n`);
  }

  return {
    filename,
    numFrames,
    duration,
    numSamples,
    sampleRate,
    bitDepth,
    generationTime,
    fileSize: wav.toBuffer().length
  };
}

/**
 * Render multiple patterns to WAV files
 * 
 * @param {ChimeraController} controller - Pattern controller
 * @param {Array<Object>} patterns - Array of pattern configurations
 * @param {Object} options - Rendering options (passed to renderToWav)
 * @returns {Array<Object>} Array of rendering statistics
 */
export function renderPatterns(controller, patterns, options = {}) {
  const results = [];
  
  for (const patternConfig of patterns) {
    const { patternId, intensity = 1.0, filename } = patternConfig;
    
    controller.applyPattern(patternId, intensity);
    const stats = renderToWav(controller.oscillator, filename, options);
    
    results.push({
      patternId,
      intensity,
      ...stats
    });
  }
  
  return results;
}

/**
 * Calculate file size for given parameters
 * 
 * @param {number} numFrames - Number of frames
 * @param {number} sampleRate - Sample rate in Hz
 * @param {string} bitDepth - Bit depth
 * @returns {number} Estimated file size in bytes
 */
export function estimateFileSize(numFrames, sampleRate = 48000, bitDepth = '32f') {
  const duration = numFrames * FRAME_CONFIG.FRAME_DURATION;
  const numSamples = Math.floor(sampleRate * duration);
  
  let bytesPerSample;
  switch (bitDepth) {
    case '32f': bytesPerSample = 4; break;
    case '16': bytesPerSample = 2; break;
    case '24': bytesPerSample = 3; break;
    case '8': bytesPerSample = 1; break;
    default: bytesPerSample = 4;
  }
  
  // WAV header is 44 bytes + data
  return 44 + (numSamples * bytesPerSample);
}

/**
 * Format file size as human-readable string
 * 
 * @param {number} bytes - Size in bytes
 * @returns {string} Formatted string (e.g., "1.5 MB")
 */
export function formatFileSize(bytes) {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}

export default {
  FRAME_CONFIG,
  renderToWav,
  renderPatterns,
  estimateFileSize,
  formatFileSize
};
