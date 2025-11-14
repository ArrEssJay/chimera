/**
 * Example: AID (Auditory Intermodulation Distortion) Simulation
 * 
 * Demonstrates the THz carrier simulation pipeline:
 * 1. Generate 12 kHz oscillator signal
 * 2. Apply bandpass filter
 * 3. Process through AID simulation (THz heterodyne mixing + biological demodulation)
 * 4. Optional: Mix with external audio for secondary intermodulation
 */

import { ChimeraOscillator } from './chimera-oscillator.js';
import { ThzCarrierConfig, AudioMixingConfig } from './chimera-aid.js';
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// Audio output utilities
function writeWavFile(filename, samples, sampleRate = 48000) {
  const numChannels = 1;
  const bitsPerSample = 16;
  const byteRate = sampleRate * numChannels * bitsPerSample / 8;
  const blockAlign = numChannels * bitsPerSample / 8;
  const dataSize = samples.length * bitsPerSample / 8;
  const fileSize = 36 + dataSize;

  const buffer = Buffer.alloc(44 + dataSize);
  let offset = 0;

  // RIFF header
  buffer.write('RIFF', offset); offset += 4;
  buffer.writeUInt32LE(fileSize, offset); offset += 4;
  buffer.write('WAVE', offset); offset += 4;

  // fmt chunk
  buffer.write('fmt ', offset); offset += 4;
  buffer.writeUInt32LE(16, offset); offset += 4; // Subchunk1Size
  buffer.writeUInt16LE(1, offset); offset += 2;  // AudioFormat (PCM)
  buffer.writeUInt16LE(numChannels, offset); offset += 2;
  buffer.writeUInt32LE(sampleRate, offset); offset += 4;
  buffer.writeUInt32LE(byteRate, offset); offset += 4;
  buffer.writeUInt16LE(blockAlign, offset); offset += 2;
  buffer.writeUInt16LE(bitsPerSample, offset); offset += 2;

  // data chunk
  buffer.write('data', offset); offset += 4;
  buffer.writeUInt32LE(dataSize, offset); offset += 4;

  // Write samples
  for (let i = 0; i < samples.length; i++) {
    const sample = Math.max(-1, Math.min(1, samples[i]));
    const intSample = Math.floor(sample * 32767);
    buffer.writeInt16LE(intSample, offset);
    offset += 2;
  }

  fs.writeFileSync(filename, buffer);
  console.log(`✓ Written: ${filename} (${samples.length} samples, ${(samples.length / sampleRate).toFixed(2)}s)`);
}

console.log('=== Chimera AID Simulation Examples ===\n');

// Configuration
const sampleRate = 48000;
const duration = 2.0; // 2 seconds
const numSamples = Math.floor(sampleRate * duration);

// Example 1: Basic AID simulation (idle state)
console.log('Example 1: Basic AID Simulation - Idle State');
console.log('---------------------------------------------');
const osc1 = new ChimeraOscillator();
osc1.setFSKState(0); // Idle state (11,999 Hz)
osc1.setAmplitude(0.8);
osc1.setAidEnabled(true);

// Configure AID for idle state
const idleConfig = new ThzCarrierConfig();
idleConfig.modulationDepth = 0.05; // 5% modulation (faint hum)
osc1.setAidConfig(idleConfig);

const samples1 = osc1.generateSamples(numSamples, sampleRate);
writeWavFile('output_aid_idle.wav', samples1, sampleRate);
console.log(`Modulation depth: ${idleConfig.modulationDepth * 100}%`);
console.log('Expected: Faint 12 kHz tone (carrier leakage)\n');

// Example 2: AID simulation - Active state
console.log('Example 2: AID Simulation - Active State');
console.log('-----------------------------------------');
const osc2 = new ChimeraOscillator();
osc2.setFSKState(1); // Active state (12,001 Hz)
osc2.setAmplitude(0.8);
osc2.setAidEnabled(true);

// Add some LFO modulation for demonstration
osc2.setFreqModulation(2.0, 'sine', 0.3); // Subtle frequency wobble
osc2.setAmpModulation(4.0, 'sine', 0.2);  // Breathing effect

// Configure AID for active state
const activeConfig = new ThzCarrierConfig();
activeConfig.modulationDepth = 0.8; // 80% modulation (strong signal)
activeConfig.mixingCoefficient = 0.9; // High biological efficiency
osc2.setAidConfig(activeConfig);

const samples2 = osc2.generateSamples(numSamples, sampleRate);
writeWavFile('output_aid_active.wav', samples2, sampleRate);
console.log(`Modulation depth: ${activeConfig.modulationDepth * 100}%`);
console.log(`Mixing coefficient: ${activeConfig.mixingCoefficient}`);
console.log('Expected: Clear 12 kHz tone with modulation\n');

// Example 3: AID bypass mode (validation)
console.log('Example 3: AID Bypass Mode (Validation)');
console.log('----------------------------------------');
const osc3 = new ChimeraOscillator();
osc3.setFSKState(0);
osc3.setAmplitude(0.8);
osc3.setAidEnabled(true);

const bypassConfig = new ThzCarrierConfig();
bypassConfig.bypassSimulation = true; // Bypass AID effects
osc3.setAidConfig(bypassConfig);

const samples3 = osc3.generateSamples(numSamples, sampleRate);
writeWavFile('output_aid_bypass.wav', samples3, sampleRate);
console.log('Bypass enabled: Signal passes through unchanged');
console.log('Expected: Pure 12 kHz sine wave (no AID effects)\n');

// Example 4: Comparison - Filter only vs Filter + AID
console.log('Example 4: Comparison - Filter Only vs Filter + AID');
console.log('----------------------------------------------------');

// 4a: Filter only
const osc4a = new ChimeraOscillator();
osc4a.setFSKState(0);
osc4a.setAmplitude(0.8);
osc4a.setFilterEnabled(true);
osc4a.setAidEnabled(false); // No AID

const samples4a = osc4a.generateSamples(numSamples, sampleRate);
writeWavFile('output_filter_only.wav', samples4a, sampleRate);
console.log('✓ Filter only (no AID)');

// 4b: Filter + AID
const osc4b = new ChimeraOscillator();
osc4b.setFSKState(0);
osc4b.setAmplitude(0.8);
osc4b.setFilterEnabled(true);
osc4b.setAidEnabled(true);

const filterAidConfig = new ThzCarrierConfig();
filterAidConfig.modulationDepth = 0.5;
osc4b.setAidConfig(filterAidConfig);

const samples4b = osc4b.generateSamples(numSamples, sampleRate);
writeWavFile('output_filter_and_aid.wav', samples4b, sampleRate);
console.log('✓ Filter + AID simulation');
console.log('Expected: Audible difference due to THz nonlinear effects\n');

// Example 5: Phase noise comparison
console.log('Example 5: Phase Noise Effects');
console.log('-------------------------------');

// 5a: Low phase noise
const osc5a = new ChimeraOscillator();
osc5a.setFSKState(0);
osc5a.setAmplitude(0.8);
osc5a.setAidEnabled(true);

const lowNoiseConfig = new ThzCarrierConfig();
lowNoiseConfig.modulationDepth = 0.5;
lowNoiseConfig.phaseNoiseStd = 0.0001; // Very low noise
osc5a.setAidConfig(lowNoiseConfig);

const samples5a = osc5a.generateSamples(numSamples, sampleRate);
writeWavFile('output_aid_low_noise.wav', samples5a, sampleRate);
console.log(`✓ Low phase noise (std: ${lowNoiseConfig.phaseNoiseStd})`);

// 5b: High phase noise
const osc5b = new ChimeraOscillator();
osc5b.setFSKState(0);
osc5b.setAmplitude(0.8);
osc5b.setAidEnabled(true);

const highNoiseConfig = new ThzCarrierConfig();
highNoiseConfig.modulationDepth = 0.5;
highNoiseConfig.phaseNoiseStd = 0.01; // High noise (laser instability)
osc5b.setAidConfig(highNoiseConfig);

const samples5b = osc5b.generateSamples(numSamples, sampleRate);
writeWavFile('output_aid_high_noise.wav', samples5b, sampleRate);
console.log(`✓ High phase noise (std: ${highNoiseConfig.phaseNoiseStd})`);
console.log('Expected: High noise sample has more "roughness"\n');

// Summary
console.log('=== Summary ===');
console.log('Generated files:');
console.log('  1. output_aid_idle.wav          - Idle state (5% modulation)');
console.log('  2. output_aid_active.wav        - Active state (80% modulation)');
console.log('  3. output_aid_bypass.wav        - Bypass mode (validation)');
console.log('  4. output_filter_only.wav       - Filter only (no AID)');
console.log('  5. output_filter_and_aid.wav    - Filter + AID');
console.log('  6. output_aid_low_noise.wav     - Low phase noise');
console.log('  7. output_aid_high_noise.wav    - High phase noise');
console.log('\nAll samples are 48 kHz, 2 seconds duration');
console.log('\nNote: These are ultrasonic (12 kHz) test signals.');
console.log('You may need to pitch-shift or use spectrum analysis to evaluate them.');
