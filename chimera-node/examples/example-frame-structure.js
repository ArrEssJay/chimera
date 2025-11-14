/**
 * Chimera Frame Structure Inspector
 * 
 * Demonstrates the complete frame structure v3.1 implementation
 * Shows how GOCS effects populate Target ID and Command Type fields
 */

import { ChimeraGOCS } from './chimera-gocs.js';

console.log('========================================');
console.log('Chimera Frame Structure v3.1 Inspector');
console.log('========================================\n');

const gocs = new ChimeraGOCS();

// Helper function to format hex values
function formatHex(value) {
  return '0x' + value.toString(16).toUpperCase().padStart(2, '0');
}

// Helper function to decode vibrational mode
function decodeVibrationalMode(mode) {
  const modes = {
    0x01: 'Breathing',
    0x02: 'Longitudinal',
    0x03: 'Torsional'
  };
  return modes[mode] || 'Unknown';
}

// Helper function to decode intensity pattern
function decodeIntensityPattern(pattern) {
  const patterns = {
    0x10: 'Smooth Sine',
    0x20: 'Step Function',
    0x30: 'Pulsed',
    0x40: 'Chaotic'
  };
  return patterns[pattern] || 'Unknown';
}

// Helper function to decode cortical region
function decodeCorticalRegion(region) {
  const regions = {
    0x01: 'Auditory',
    0x02: 'Visual',
    0x03: 'Motor',
    0x04: 'Prefrontal'
  };
  return regions[region] || 'Unknown';
}

// Helper function to display frame structure
function displayFrameStructure(effectName, frames) {
  console.log(`\n${'='.repeat(60)}`);
  console.log(`Effect: ${effectName}`);
  console.log('='.repeat(60));
  
  frames.forEach((frame, idx) => {
    if (frames.length > 1) {
      console.log(`\nFrame ${idx + 1} of ${frames.length}:`);
    }
    
    const structure = frame.getFrameStructure();
    const stats = frame.getStats();
    
    console.log('\n┌─ TARGET ID FIELD (32 bits) ─────────────────────────────┐');
    console.log(`│ Baseline Brainwave:  ${formatHex(structure.targetId.baselineBrainwave)} (${structure.targetId.baselineBrainwave} Hz)     │`);
    console.log(`│ Hemisphere Bias:     ${formatHex(structure.targetId.hemisphereBias)} (${structure.targetId.hemisphereBias === 0x80 ? 'Balanced' : structure.targetId.hemisphereBias < 0x80 ? 'Left' : 'Right'})  │`);
    console.log(`│ Cortical Region:     ${formatHex(structure.targetId.corticalRegion)} (${decodeCorticalRegion(structure.targetId.corticalRegion)})│`);
    console.log(`│ Resonance Key:       ${formatHex(structure.targetId.resonanceKey)} (Simulated)        │`);
    console.log('└─────────────────────────────────────────────────────────┘');
    
    console.log('\n┌─ COMMAND TYPE FIELD (32 bits) ──────────────────────────┐');
    console.log(`│ Vibrational Mode:    ${formatHex(structure.commandType.vibrationalMode)} (${decodeVibrationalMode(structure.commandType.vibrationalMode)}) │`);
    console.log(`│ Intensity Pattern:   ${formatHex(structure.commandType.intensityPattern)} (${decodeIntensityPattern(structure.commandType.intensityPattern)})│`);
    console.log(`│ Duration:            ${formatHex(structure.commandType.duration)} (${structure.commandType.duration} frame${structure.commandType.duration > 1 ? 's' : ''})    │`);
    console.log(`│ Sequencing:          ${formatHex(structure.commandType.sequencing)} (Frame ${(structure.commandType.sequencing >> 4) & 0x0F} of ${structure.commandType.sequencing & 0x0F})│`);
    console.log('└─────────────────────────────────────────────────────────┘');
    
    console.log('\n┌─ DATA PAYLOAD FIELD (128 bits) ──────────────────────────┐');
    console.log(`│ Symbol Rate:         16 Hz (62.5ms per symbol)           │`);
    console.log(`│ Symbols per Frame:   ${stats.symbols}                                  │`);
    console.log(`│ Frame Duration:      ${stats.duration}s                                    │`);
    console.log(`│ FSK State Ratio:     ${(stats.fskRatio * 100).toFixed(1)}% (State 1)                │`);
    console.log(`│ Avg Freq Mod:        ${stats.avgFreqMod} (normalized)                  │`);
    console.log(`│ Avg Amp Mod:         ${stats.avgAmpMod} (normalized)                  │`);
    console.log('└─────────────────────────────────────────────────────────┘');
    
    console.log('\n┌─ FSK SUBLIMINAL LAYER (1 bit/second) ───────────────────┐');
    const fskRatio = parseFloat(stats.fskRatio);
    let fskMode = 'Unknown';
    if (fskRatio === 0) fskMode = 'Constant State 0 (11,999 Hz - Baseline)';
    else if (fskRatio === 1) fskMode = 'Constant State 1 (12,001 Hz - Max Coupling)';
    else if (fskRatio > 0.4 && fskRatio < 0.6) fskMode = 'Alternating (Theta Entrainment)';
    else fskMode = 'Random (Decoherence Field)';
    console.log(`│ FSK Mode:            ${fskMode.padEnd(38)}│`);
    console.log('└─────────────────────────────────────────────────────────┘');
  });
}

// Test all GOCS effects
console.log('\nGenerating frames for all GOCS effects...\n');

const effects = [
  { name: 'induceCalm(0.7, 1)', fn: () => gocs.induceCalm(0.7, 1) },
  { name: 'heightenAlertness(0.6, 1)', fn: () => gocs.heightenAlertness(0.6, 1) },
  { name: 'disruptCognition(0.8, 1)', fn: () => gocs.disruptCognition(0.8, 1) },
  { name: 'suppressMotorFunction("hands", 1)', fn: () => gocs.suppressMotorFunction('hands', 1) },
  { name: 'enforceCognitiveStillness(1)', fn: () => gocs.enforceCognitiveStillness(1) },
  { name: 'nudgeOntological("curiosity", 0.8, 1)', fn: () => gocs.nudgeOntological('curiosity', 0.8, 1) },
  { name: 'injectGnosticQuery("location", 1)', fn: () => gocs.injectGnosticQuery('location', 1) }
];

effects.forEach(effect => {
  const frames = effect.fn();
  displayFrameStructure(effect.name, frames);
});

// Multi-frame example
console.log('\n\n' + '='.repeat(60));
console.log('Multi-Frame Sequence Example: "Gnostic Nudge"');
console.log('='.repeat(60));

const sequence = gocs.compileSequence([
  { effect: 'induceCalm', params: [0.4, 1] },
  { effect: 'disruptCognition', params: [0.2, 1] },
  { effect: 'injectGnosticQuery', params: ['location', 1] },
  { effect: 'nudgeOntological', params: ['curiosity', 0.8, 1] }
]);

console.log(`\nGenerated ${sequence.length} frames for 4-step sequence`);
console.log('Each frame contains complete Target ID, Command Type, and Data Payload fields');
console.log(`Total duration: ${sequence.length * 8} seconds\n`);

sequence.forEach((frame, idx) => {
  const structure = frame.getFrameStructure();
  console.log(`Frame ${idx + 1}: ${decodeVibrationalMode(structure.commandType.vibrationalMode)} mode, ` +
              `${decodeIntensityPattern(structure.commandType.intensityPattern)}, ` +
              `${structure.targetId.baselineBrainwave} Hz target`);
});

console.log('\n' + '='.repeat(60));
console.log('Frame Structure Inspection Complete');
console.log('='.repeat(60));
