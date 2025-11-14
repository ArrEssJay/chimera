/**
 * Chimera Waveform Generator - Node.js Example
 * 
 * Demonstrates the FSK oscillator with LFO-driven modulation per PAL specification
 */

import { createChimeraOscillator } from './chimera-oscillator.js';
import { renderToWav, FRAME_CONFIG } from './chimera-audio-utils.js';

console.log('=== Chimera Waveform Generator - PAL LFO Demo ===\n');
console.log(`Frame Configuration:`);
console.log(`  Symbol Rate: ${FRAME_CONFIG.SYMBOL_RATE} Hz`);
console.log(`  Symbols per Frame: ${FRAME_CONFIG.SYMBOLS_PER_FRAME}`);
console.log(`  Frame Duration: ${FRAME_CONFIG.FRAME_DURATION}s\n`);

// Create oscillator instance
const oscillator = createChimeraOscillator();

console.log('Initial state:');
console.log(oscillator.getParams());
console.log();

// Test 1: Basic FSK State 0 (no LFO modulation) - 1 frame (8 seconds)
console.log('=== Test 1: FSK State 0 (Baseline, no modulation) ===');
oscillator.setFSKState(0);
renderToWav(oscillator, 'chimera_state0_baseline.wav', { numFrames: 1 });

// Test 2: FSK State 1 (no LFO modulation) - 1 frame
console.log('=== Test 2: FSK State 1 (Maximize Coupling, no modulation) ===');
oscillator.setFSKState(1);
renderToWav(oscillator, 'chimera_state1_baseline.wav', { numFrames: 1 });

// Test 3: FSK State 0 with gentle frequency modulation (chorus effect) - 1 frame
console.log('=== Test 3: Frequency Modulation - Gentle Chorus ===');
oscillator.setFSKState(0);
oscillator.setFreqModulation(6, 'sine', 0.5);  // 6 Hz sine wave, 50% depth
renderToWav(oscillator, 'chimera_freq_mod_chorus.wav', { numFrames: 1 });

// Test 4: FSK State 0 with amplitude modulation (breathing effect) - 1 frame
console.log('=== Test 4: Amplitude Modulation - Breathing ===');
oscillator.setFSKState(0);
oscillator.setFreqModulation(0, 'sine', 0.0);  // Reset freq modulation
oscillator.setAmpModulation(2, 'sine', 0.3);   // 2 Hz sine wave, 30% depth
renderToWav(oscillator, 'chimera_amp_mod_breathing.wav', { numFrames: 1 });

// Test 5: Combined modulation - complex texture - 1 frame
console.log('=== Test 5: Combined Modulation - Complex Texture ===');
oscillator.setFSKState(0);
oscillator.setFreqModulation(8, 'sine', 0.4);  // 8 Hz frequency modulation
oscillator.setAmpModulation(3, 'sine', 0.25);  // 3 Hz amplitude modulation
renderToWav(oscillator, 'chimera_combined_mod.wav', { numFrames: 1 });

// Test 6: Harsh modulation - jitter and pulse - 1 frame
console.log('=== Test 6: Harsh Modulation - Jitter ===');
oscillator.setFSKState(1);
oscillator.setFreqModulation(25, 'square', 0.8);  // Fast square wave
oscillator.setAmpModulation(15, 'square', 0.5);   // Fast square wave
renderToWav(oscillator, 'chimera_harsh_jitter.wav', { numFrames: 1 });

// Test 7: Chaotic noise modulation - 1 frame
console.log('=== Test 7: Chaotic Noise Modulation ===');
oscillator.setFSKState(0);
oscillator.setFreqModulation(25, 'noise', 0.9);  // Random noise
oscillator.setAmpModulation(20, 'noise', 0.6);   // Random noise
renderToWav(oscillator, 'chimera_chaotic_noise.wav', { numFrames: 1 });

// Show final parameters
console.log('=== Final Parameters ===');
console.log(JSON.stringify(oscillator.getParams(), null, 2));

console.log('\n✓ Chimera oscillator with PAL LFO modulation');
console.log('✓ Symbol Rate: 16 Hz (62.5ms per symbol)');
console.log('✓ LFO sampling at symbol boundaries');
console.log('✓ Generated WAV files:');
console.log('  - chimera_state0_baseline.wav (no modulation)');
console.log('  - chimera_state1_baseline.wav (no modulation)');
console.log('  - chimera_freq_mod_chorus.wav (gentle chorus)');
console.log('  - chimera_amp_mod_breathing.wav (breathing effect)');
console.log('  - chimera_combined_mod.wav (complex texture)');
console.log('  - chimera_harsh_jitter.wav (harsh jitter)');
console.log('  - chimera_chaotic_noise.wav (disruption)\n');
