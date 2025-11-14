/**
 * Example: AID Secondary Intermodulation with External Audio
 * 
 * Demonstrates mixing the AID signal with external audio to simulate
 * the secondary intermodulation effects that occur when the target
 * processes both the anomalous AID signal and conventional acoustic input.
 * 
 * Uses test audio files from chimera-web/audio/
 */

import { ChimeraOscillator } from './chimera-oscillator.js';
import { ThzCarrierConfig, AudioMixingConfig } from './chimera-aid.js';
import { loadAudio, checkFfmpegAvailable, resampleAudio } from './chimera-audio-loader.js';
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// Simple WAV file reader (handles basic 16-bit PCM) - kept as fallback
function readWavFile(filename) {
  const buffer = fs.readFileSync(filename);
  
  // Parse WAV header
  const riff = buffer.toString('ascii', 0, 4);
  if (riff !== 'RIFF') {
    throw new Error('Not a valid WAV file');
  }
  
  const wave = buffer.toString('ascii', 8, 12);
  if (wave !== 'WAVE') {
    throw new Error('Not a valid WAV file');
  }
  
  // Find data chunk
  let offset = 12;
  let dataOffset = 0;
  let dataSize = 0;
  let sampleRate = 44100;
  let numChannels = 1;
  let bitsPerSample = 16;
  
  while (offset < buffer.length) {
    const chunkId = buffer.toString('ascii', offset, offset + 4);
    const chunkSize = buffer.readUInt32LE(offset + 4);
    
    if (chunkId === 'fmt ') {
      sampleRate = buffer.readUInt32LE(offset + 12);
      numChannels = buffer.readUInt16LE(offset + 10);
      bitsPerSample = buffer.readUInt16LE(offset + 22);
    } else if (chunkId === 'data') {
      dataOffset = offset + 8;
      dataSize = chunkSize;
      break;
    }
    
    offset += 8 + chunkSize;
  }
  
  if (dataOffset === 0) {
    throw new Error('No data chunk found');
  }
  
  // Read samples (assuming 16-bit PCM)
  const numSamples = Math.floor(dataSize / (bitsPerSample / 8));
  const samples = new Float32Array(numSamples);
  
  for (let i = 0; i < numSamples; i++) {
    const sampleOffset = dataOffset + i * 2;
    const intSample = buffer.readInt16LE(sampleOffset);
    samples[i] = intSample / 32768.0;
  }
  
  console.log(`Read: ${path.basename(filename)}`);
  console.log(`  Sample rate: ${sampleRate} Hz`);
  console.log(`  Channels: ${numChannels}`);
  console.log(`  Samples: ${numSamples}`);
  console.log(`  Duration: ${(numSamples / sampleRate).toFixed(2)}s`);
  
  return { samples, sampleRate, numChannels };
}

// Write WAV file
function writeWavFile(filename, samples, sampleRate = 48000) {
  const numChannels = 1;
  const bitsPerSample = 16;
  const byteRate = sampleRate * numChannels * bitsPerSample / 8;
  const blockAlign = numChannels * bitsPerSample / 8;
  const dataSize = samples.length * bitsPerSample / 8;
  const fileSize = 36 + dataSize;

  const buffer = Buffer.alloc(44 + dataSize);
  let offset = 0;

  buffer.write('RIFF', offset); offset += 4;
  buffer.writeUInt32LE(fileSize, offset); offset += 4;
  buffer.write('WAVE', offset); offset += 4;
  buffer.write('fmt ', offset); offset += 4;
  buffer.writeUInt32LE(16, offset); offset += 4;
  buffer.writeUInt16LE(1, offset); offset += 2;
  buffer.writeUInt16LE(numChannels, offset); offset += 2;
  buffer.writeUInt32LE(sampleRate, offset); offset += 4;
  buffer.writeUInt32LE(byteRate, offset); offset += 4;
  buffer.writeUInt16LE(blockAlign, offset); offset += 2;
  buffer.writeUInt16LE(bitsPerSample, offset); offset += 2;
  buffer.write('data', offset); offset += 4;
  buffer.writeUInt32LE(dataSize, offset); offset += 4;

  for (let i = 0; i < samples.length; i++) {
    const sample = Math.max(-1, Math.min(1, samples[i]));
    const intSample = Math.floor(sample * 32767);
    buffer.writeInt16LE(intSample, offset);
    offset += 2;
  }

  fs.writeFileSync(filename, buffer);
  console.log(`✓ Written: ${filename} (${(samples.length / sampleRate).toFixed(2)}s)\n`);
}

console.log('=== AID Secondary Intermodulation Examples ===\n');

// Configuration
const sampleRate = 48000;
const duration = 10.0;
const numSamples = Math.floor(sampleRate * duration);

// Check for ffmpeg availability
const hasFFmpeg = await checkFfmpegAvailable();
console.log(`FFmpeg available: ${hasFFmpeg ? '✓ Yes' : '✗ No (will generate synthetic audio)'}\n`);

// Look for test audio files
const audioDir = path.join(__dirname, '../chimera-web/audio');
const testFiles = [
  'music.mp3'
];

let externalAudio = null;
let audioSource = 'synthetic';

// Try to load real audio file if ffmpeg is available
if (hasFFmpeg) {
  for (const testFile of testFiles) {
    const filePath = path.join(audioDir, testFile);
    if (fs.existsSync(filePath)) {
      try {
        console.log(`Loading external audio: ${testFile}...`);
        const audioData = await loadAudio(filePath, { targetSampleRate: sampleRate });
        
        // Trim to desired duration if needed
        const maxSamples = numSamples;
        if (audioData.samples.length > maxSamples) {
          externalAudio = audioData.samples.slice(0, maxSamples);
        } else if (audioData.samples.length < maxSamples) {
          // Pad with zeros if too short
          externalAudio = new Float32Array(maxSamples);
          externalAudio.set(audioData.samples);
        } else {
          externalAudio = audioData.samples;
        }
        
        audioSource = testFile;
        console.log(`✓ Loaded: ${audioData.duration.toFixed(2)}s, ${audioData.sampleRate} Hz\n`);
        break;
      } catch (err) {
        console.warn(`Failed to load ${testFile}:`, err.message);
      }
    }
  }
}

// Fallback to synthetic audio if no real file loaded
if (!externalAudio) {
  console.log('Generating synthetic external audio for demonstration.\n');
  externalAudio = generateSyntheticAudio(numSamples, sampleRate);
}

// Generate synthetic "external audio" (simulating speech/music)
function generateSyntheticAudio(numSamples, sampleRate) {
  const audio = new Float32Array(numSamples);
  
  // Multi-tone complex signal (simulating voice/music harmonics)
  const fundamentals = [200, 300, 450]; // Hz
  const amplitudes = [0.3, 0.2, 0.15];
  
  for (let i = 0; i < numSamples; i++) {
    const t = i / sampleRate;
    let sample = 0;
    
    // Add harmonics
    for (let f = 0; f < fundamentals.length; f++) {
      const freq = fundamentals[f];
      const amp = amplitudes[f];
      
      // Fundamental
      sample += amp * Math.sin(2 * Math.PI * freq * t);
      
      // Harmonics (2nd and 3rd)
      sample += amp * 0.5 * Math.sin(2 * Math.PI * freq * 2 * t);
      sample += amp * 0.3 * Math.sin(2 * Math.PI * freq * 3 * t);
    }
    
    // Add some amplitude modulation (breathing/vibrato)
    const modulation = 0.8 + 0.2 * Math.sin(2 * Math.PI * 3 * t);
    audio[i] = sample * modulation * 0.3;
  }
  
  return audio;
}

// Example 1: AID signal only (no external audio)
console.log('Example 1: AID Signal Only (Baseline)');
console.log('--------------------------------------');
const osc1 = new ChimeraOscillator();
osc1.setFSKState(1); // Active state
osc1.setAmplitude(0.8);
osc1.setAidEnabled(true);

const aidConfig1 = new ThzCarrierConfig();
aidConfig1.modulationDepth = 0.7;
osc1.setAidConfig(aidConfig1);

const aidOnly = osc1.generateSamples(numSamples, sampleRate);
writeWavFile('output_aid_only.wav', aidOnly, sampleRate);

// Example 2: External audio only (for reference)
console.log('Example 2: External Audio Only (Reference)');
console.log('-------------------------------------------');
console.log(`Source: ${audioSource}`);
writeWavFile('output_external_only.wav', externalAudio, sampleRate);

// Example 3: AID + External Audio with secondary intermodulation
console.log('Example 3: AID + External Audio (Secondary Intermodulation)');
console.log('------------------------------------------------------------');
const osc3 = new ChimeraOscillator();
osc3.setFSKState(1);
osc3.setAmplitude(0.8);
osc3.setAidEnabled(true);

const aidConfig3 = new ThzCarrierConfig();
aidConfig3.modulationDepth = 0.7;
osc3.setAidConfig(aidConfig3);

// Generate AID samples first (to initialize processor)
const aidSamples = osc3.generateSamples(numSamples, sampleRate);

// Now add external audio for secondary intermodulation
const mixingConfig = new AudioMixingConfig();
mixingConfig.aidSignalGain = 0.15;  // Subtle AID overlay
mixingConfig.externalAudioGain = 1.0;  // Dominant external audio
mixingConfig.enableSecondOrder = true;
mixingConfig.enableThirdOrder = true;
mixingConfig.secondOrderCoefficient = 0.08;  // Reduced intermod
mixingConfig.thirdOrderCoefficient = 0.04;   // Reduced intermod
mixingConfig.corticalCoefficient = 0.15;     // Reduced blend

osc3.setAidExternalAudio(externalAudio, mixingConfig);

// Regenerate with intermodulation
const mixed = osc3.generateSamples(numSamples, sampleRate);
writeWavFile('output_aid_with_external.wav', mixed, sampleRate);

console.log('Mixing configuration:');
console.log(`  AID gain: ${mixingConfig.aidSignalGain}`);
console.log(`  External gain: ${mixingConfig.externalAudioGain}`);
console.log(`  Second-order: ${mixingConfig.enableSecondOrder} (coeff: ${mixingConfig.secondOrderCoefficient})`);
console.log(`  Third-order: ${mixingConfig.enableThirdOrder} (coeff: ${mixingConfig.thirdOrderCoefficient})`);
console.log(`  Cortical blend: ${mixingConfig.corticalCoefficient}\n`);

// Example 4: Varying intermodulation coefficients
console.log('Example 4: High Intermodulation (Aggressive Mixing)');
console.log('----------------------------------------------------');
const osc4 = new ChimeraOscillator();
osc4.setFSKState(1);
osc4.setAmplitude(0.8);
osc4.setAidEnabled(true);

const aidConfig4 = new ThzCarrierConfig();
aidConfig4.modulationDepth = 0.7;
osc4.setAidConfig(aidConfig4);

osc4.generateSamples(numSamples, sampleRate); // Initialize

const aggressiveMixing = new AudioMixingConfig();
aggressiveMixing.aidSignalGain = 0.6;  // Moderate AID
aggressiveMixing.externalAudioGain = 1.0;  // Dominant external
aggressiveMixing.secondOrderCoefficient = 1.0; // Higher intermod
aggressiveMixing.thirdOrderCoefficient = 1.0;  // Higher intermod
aggressiveMixing.corticalCoefficient = 1.0;    // Higher blend

osc4.setAidExternalAudio(externalAudio, aggressiveMixing);
const aggressive = osc4.generateSamples(numSamples, sampleRate);
writeWavFile('output_aid_aggressive_mixing.wav', aggressive, sampleRate);

// Example 5: Second-order only (cochlear junction)
console.log('Example 5: Second-Order Only (Cochlear Junction)');
console.log('-------------------------------------------------');
const osc5 = new ChimeraOscillator();
osc5.setFSKState(1);
osc5.setAmplitude(0.8);
osc5.setAidEnabled(true);

const aidConfig5 = new ThzCarrierConfig();
aidConfig5.modulationDepth = 0.7;
osc5.setAidConfig(aidConfig5);

osc5.generateSamples(numSamples, sampleRate);

const secondOrderOnly = new AudioMixingConfig();
secondOrderOnly.aidSignalGain = 0.2;
secondOrderOnly.externalAudioGain = 1.0;
secondOrderOnly.enableSecondOrder = true;
secondOrderOnly.enableThirdOrder = false; // Disable
secondOrderOnly.secondOrderCoefficient = 0.15;
secondOrderOnly.corticalCoefficient = 0.08;

osc5.setAidExternalAudio(externalAudio, secondOrderOnly);
const cochlear = osc5.generateSamples(numSamples, sampleRate);
writeWavFile('output_aid_second_order_only.wav', cochlear, sampleRate);

// Example 6: Third-order only (cortical processing)
console.log('Example 6: Third-Order Only (Cortical Processing)');
console.log('--------------------------------------------------');
const osc6 = new ChimeraOscillator();
osc6.setFSKState(1);
osc6.setAmplitude(0.8);
osc6.setAidEnabled(true);

const aidConfig6 = new ThzCarrierConfig();
aidConfig6.modulationDepth = 0.7;
osc6.setAidConfig(aidConfig6);

osc6.generateSamples(numSamples, sampleRate);

const thirdOrderOnly = new AudioMixingConfig();
thirdOrderOnly.aidSignalGain = 0.2;
thirdOrderOnly.externalAudioGain = 1.0;
thirdOrderOnly.enableSecondOrder = false; // Disable
thirdOrderOnly.enableThirdOrder = true;
thirdOrderOnly.thirdOrderCoefficient = 0.1;
thirdOrderOnly.corticalCoefficient = 0.08;

osc6.setAidExternalAudio(externalAudio, thirdOrderOnly);
const cortical = osc6.generateSamples(numSamples, sampleRate);
writeWavFile('output_aid_third_order_only.wav', cortical, sampleRate);

console.log('\n=== Summary ===');
console.log('Generated files:');
console.log('  1. output_aid_only.wav                  - AID signal baseline');
console.log('  2. output_external_only.wav             - External audio reference');
console.log('  3. output_aid_with_external.wav         - Combined with standard mixing');
console.log('  4. output_aid_aggressive_mixing.wav     - High intermodulation coefficients');
console.log('  5. output_aid_second_order_only.wav     - Cochlear junction effects only');
console.log('  6. output_aid_third_order_only.wav      - Cortical processing only');
console.log('\nAll samples: 48 kHz, 3 seconds duration');
console.log('\nListen for differences in:');
console.log('  - Frequency content (sum/difference products)');
console.log('  - Amplitude modulation patterns');
console.log('  - Perceived "texture" and harmonic structure');
