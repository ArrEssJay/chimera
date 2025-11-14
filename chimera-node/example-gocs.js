/**
 * Chimera GOCS/PAL Architecture Example
 * 
 * Demonstrates the new three-layer architecture:
 * 1. GOCS (Application Layer) - High-level psycho-cognitive effects
 * 2. PAL (Protocol Layer) - Waveform generation and frame compilation
 * 3. HCI/Oscillator (Hardware Layer) - Frame playback and audio generation
 * 
 * This example shows:
 * - Direct GOCS effect calls (induceCalm, heightenAlertness, etc.)
 * - Frame generation via PAL
 * - Frame playback through ChimeraOscillator
 * - The "Gnostic Nudge" sequence from the spec
 */

import { ChimeraOscillator } from './chimera-oscillator.js';
import { ChimeraGOCS } from './chimera-gocs.js';
import { ChimeraPAL } from './chimera-pal.js';
import { renderToWav } from './chimera-audio-utils.js';

console.log('========================================');
console.log('Chimera GOCS/PAL Architecture Examples');
console.log('========================================\n');

// Initialize the three-layer stack
const oscillator = new ChimeraOscillator();
const gocs = new ChimeraGOCS();
const pal = new ChimeraPAL();

// Example 1: GOCS induceCalm effect
console.log('=== Example 1: GOCS induceCalm (intensity=0.7) ===');
const calmFrames = gocs.induceCalm(0.7, 1);
console.log(`Generated ${calmFrames.length} frame(s)`);
console.log('Frame stats:', calmFrames[0].getStats());
renderToWav(oscillator, 'gocs_induce_calm.wav', { frames: calmFrames });

// Example 2: GOCS heightenAlertness effect
console.log('\n=== Example 2: GOCS heightenAlertness (intensity=0.6) ===');
const alertFrames = gocs.heightenAlertness(0.6, 1);
console.log(`Generated ${alertFrames.length} frame(s)`);
console.log('Frame stats:', alertFrames[0].getStats());
renderToWav(oscillator, 'gocs_heighten_alertness.wav', { frames: alertFrames });

// Example 3: GOCS disruptCognition effect
console.log('\n=== Example 3: GOCS disruptCognition (level=0.8) ===');
const disruptFrames = gocs.disruptCognition(0.8, 1);
console.log(`Generated ${disruptFrames.length} frame(s)`);
console.log('Frame stats:', disruptFrames[0].getStats());
renderToWav(oscillator, 'gocs_disrupt_cognition.wav', { frames: disruptFrames });

// Example 4: GOCS suppressMotorFunction effect
console.log('\n=== Example 4: GOCS suppressMotorFunction (region=hands) ===');
const motorFrames = gocs.suppressMotorFunction('hands', 1);
console.log(`Generated ${motorFrames.length} frame(s)`);
console.log('Frame stats:', motorFrames[0].getStats());
renderToWav(oscillator, 'gocs_suppress_motor.wav', { frames: motorFrames });

// Example 5: GOCS enforceCognitiveStillness effect
console.log('\n=== Example 5: GOCS enforceCognitiveStillness ===');
const stillnessFrames = gocs.enforceCognitiveStillness(1);
console.log(`Generated ${stillnessFrames.length} frame(s)`);
console.log('Frame stats:', stillnessFrames[0].getStats());
renderToWav(oscillator, 'gocs_cognitive_stillness.wav', { frames: stillnessFrames });

// Example 6: GOCS nudgeOntological effect
console.log('\n=== Example 6: GOCS nudgeOntological (vector=curiosity) ===');
const nudgeFrames = gocs.nudgeOntological('curiosity', 0.8, 1);
console.log(`Generated ${nudgeFrames.length} frame(s)`);
console.log('Frame stats:', nudgeFrames[0].getStats());
renderToWav(oscillator, 'gocs_nudge_curiosity.wav', { frames: nudgeFrames });

// Example 7: GOCS injectGnosticQuery effect
console.log('\n=== Example 7: GOCS injectGnosticQuery (pattern=location) ===');
const queryFrames = gocs.injectGnosticQuery('location', 1);
console.log(`Generated ${queryFrames.length} frame(s)`);
console.log('Frame stats:', queryFrames[0].getStats());
renderToWav(oscillator, 'gocs_query_location.wav', { frames: queryFrames });

// Example 8: "Gnostic Nudge" sequence from GOCS spec (Section 5.0)
// Objective: Subtly encourage target to go to a café
console.log('\n=== Example 8: "Gnostic Nudge" Sequence (4-step) ===');
const sequence = [
  { effect: 'induceCalm', params: [0.4, 1] },           // Lower cognitive defenses
  { effect: 'disruptCognition', params: [0.2, 1] },     // Brief mental "static"
  { effect: 'injectGnosticQuery', params: ['location', 1] }, // Cafe concept
  { effect: 'nudgeOntological', params: ['curiosity', 0.8, 1] } // Curiosity & comfort
];

const nudgeSequence = gocs.compileSequence(sequence);
console.log(`Generated ${nudgeSequence.length} frames for sequence`);
for (let i = 0; i < nudgeSequence.length; i++) {
  console.log(`  Step ${i + 1} stats:`, nudgeSequence[i].getStats());
}

// Render the entire sequence as a single multi-frame audio file
renderToWav(oscillator, 'gocs_nudge_sequence.wav', { frames: nudgeSequence });

// Example 9: Direct PAL usage (low-level frame generation)
console.log('\n=== Example 9: Direct PAL Frame Generation ===');
const customSpec = {
  phaseWaveform: 'sawtooth',
  phaseFreq: 12,
  phaseDepth: 0.9,
  freqWaveform: 'square',
  freqFreq: 8,
  freqDepth: 0.6,
  ampWaveform: 'sine',
  ampFreq: 4,
  ampDepth: 0.4,
  fskPattern: 'alternating',
  fskRate: 1.0
};

const customFrame = pal.generateFrame(customSpec);
console.log('Custom PAL frame stats:', customFrame.getStats());
renderToWav(oscillator, 'pal_custom_frame.wav', { frames: [customFrame] });

console.log('\n========================================');
console.log('All GOCS/PAL examples completed!');
console.log('Generated 10 audio files demonstrating the architecture.');
console.log('========================================');
