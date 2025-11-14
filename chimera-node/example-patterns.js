/**
 * Chimera Pattern Library - Example
 * 
 * Demonstrates pattern application using the ChimeraController
 */

import { createChimeraOscillator } from './chimera-oscillator.js';
import { ChimeraController } from './chimera-patterns.js';
import { renderToWav, FRAME_CONFIG } from './chimera-audio-utils.js';

console.log('=== Chimera Pattern Library Demo ===\n');
console.log(`Frame Configuration:`);
console.log(`  Symbol Rate: ${FRAME_CONFIG.SYMBOL_RATE} Hz`);
console.log(`  Symbols per Frame: ${FRAME_CONFIG.SYMBOLS_PER_FRAME}`);
console.log(`  Frame Duration: ${FRAME_CONFIG.FRAME_DURATION}s\n`);

// Create oscillator and controller
const oscillator = createChimeraOscillator();
const controller = new ChimeraController(oscillator);

// Display available patterns
console.log('Available Patterns:\n');
const categories = ['Coherence & Entrainment', 'Cognitive & Perceptual', 'Disruption & Denial', 'Utility & Calibration'];
categories.forEach(cat => {
  console.log(`${cat}:`);
  const patterns = controller.listPatterns(cat);
  patterns.forEach(p => {
    console.log(`  - ${p.id}: ${p.description}`);
  });
  console.log();
});

console.log('=== Rendering Pattern Examples ===\n');

// Example 1: COH.ThetaCalm - Calming pattern (1 frame = 8 seconds)
console.log('--- Pattern: COH.ThetaCalm ---');
controller.applyPattern('COH.ThetaCalm', 0.7);
renderToWav(oscillator, 'pattern_theta_calm.wav', { numFrames: 1 });

// Example 2: COH.AlphaFocus - Focus pattern (1 frame)
console.log('--- Pattern: COH.AlphaFocus ---');
controller.applyPattern('COH.AlphaFocus', 1.0);
renderToWav(oscillator, 'pattern_alpha_focus.wav', { numFrames: 1 });

// Example 3: COH.BetaAlert - Alert pattern (1 frame)
console.log('--- Pattern: COH.BetaAlert ---');
controller.applyPattern('COH.BetaAlert', 0.8);
renderToWav(oscillator, 'pattern_beta_alert.wav', { numFrames: 1 });

// Example 4: COG.Dissonance - Dissonance pattern (1 frame)
console.log('--- Pattern: COG.Dissonance ---');
controller.applyPattern('COG.Dissonance', 0.6);
renderToWav(oscillator, 'pattern_dissonance.wav', { numFrames: 1 });

// Example 5: COG.InquisitiveUrge - Buzzy pattern (1 frame)
console.log('--- Pattern: COG.InquisitiveUrge ---');
controller.applyPattern('COG.InquisitiveUrge', 0.9);
renderToWav(oscillator, 'pattern_inquisitive.wav', { numFrames: 1 });

// Example 6: DIS.CognitiveScramble - Disruption pattern (1 frame)
console.log('--- Pattern: DIS.CognitiveScramble ---');
controller.applyPattern('DIS.CognitiveScramble', 0.9);
renderToWav(oscillator, 'pattern_cognitive_scramble.wav', { numFrames: 1 });

// Example 7: DIS.MotorLock - Motor interference pattern (1 frame)
console.log('--- Pattern: DIS.MotorLock ---');
controller.applyPattern('DIS.MotorLock', 0.7);
renderToWav(oscillator, 'pattern_motor_lock.wav', { numFrames: 1 });

// Example 8: DIS.DreadPulse - Anxiety pattern (1 frame)
console.log('--- Pattern: DIS.DreadPulse ---');
controller.applyPattern('DIS.DreadPulse', 0.5);
renderToWav(oscillator, 'pattern_dread_pulse.wav', { numFrames: 1 });

// Example 9: UTIL.BaselineCarrier - Unmodulated carrier (1 frame)
console.log('--- Pattern: UTIL.BaselineCarrier ---');
controller.applyPattern('UTIL.BaselineCarrier', 1.0);
renderToWav(oscillator, 'pattern_baseline.wav', { numFrames: 1 });

// Show current state
console.log('=== Current State ===');
console.log('Oscillator Parameters:');
console.log(JSON.stringify(oscillator.getParams(), null, 2));

if (controller.getCurrentPattern()) {
  console.log('\nCurrent Pattern:');
  console.log(JSON.stringify(controller.getCurrentPattern().getParams(), null, 2));
}

console.log('\n✓ Pattern library demo complete');
console.log('✓ Generated 9 pattern examples');
console.log('✓ All patterns use 16 Hz symbol rate with LFO modulation\n');
