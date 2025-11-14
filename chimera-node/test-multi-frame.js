/**
 * Test multi-frame GOCS rendering
 * 
 * This test verifies that GOCS methods with duration > 1 
 * properly generate and render multiple frames sequentially.
 */

import { ChimeraOscillator } from './chimera-oscillator.js';
import { ChimeraGOCS } from './chimera-gocs.js';
import { renderToWav } from './chimera-audio-utils.js';

console.log('===========================================');
console.log('Multi-Frame GOCS Rendering Test');
console.log('===========================================\n');

const oscillator = new ChimeraOscillator();
const gocs = new ChimeraGOCS();

// Test 1: induceCalm with 3 frames (should be 24 seconds)
console.log('=== Test 1: induceCalm with duration=3 ===');
const calmFrames3 = gocs.induceCalm(0.7, 3);
console.log(`Generated ${calmFrames3.length} frames`);
renderToWav(oscillator, 'test_calm_3frames.wav', { frames: calmFrames3 });

// Test 2: heightenAlertness with 2 frames (should be 16 seconds)
console.log('=== Test 2: heightenAlertness with duration=2 ===');
const alertFrames2 = gocs.heightenAlertness(0.6, 2);
console.log(`Generated ${alertFrames2.length} frames`);
renderToWav(oscillator, 'test_alert_2frames.wav', { frames: alertFrames2 });

// Test 3: Mixed sequence with varying durations
console.log('=== Test 3: Mixed sequence (5 frames total) ===');
const mixedSequence = gocs.compileSequence([
  { effect: 'induceCalm', params: [0.5, 2] },          // 2 frames
  { effect: 'heightenAlertness', params: [0.7, 2] },   // 2 frames
  { effect: 'disruptCognition', params: [0.3, 1] }     // 1 frame
]);
console.log(`Generated ${mixedSequence.length} frames`);
renderToWav(oscillator, 'test_mixed_5frames.wav', { frames: mixedSequence });

console.log('\n===========================================');
console.log('Multi-Frame Test Complete');
console.log('Check that file durations match expectations:');
console.log('  test_calm_3frames.wav: 24 seconds');
console.log('  test_alert_2frames.wav: 16 seconds');
console.log('  test_mixed_5frames.wav: 40 seconds');
console.log('===========================================');
