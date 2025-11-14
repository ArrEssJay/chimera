/**
 * Frame Structure Usage Example
 * 
 * Shows how to programmatically access and use frame structure metadata
 */

import { ChimeraOscillator } from './chimera-oscillator.js';
import { ChimeraGOCS } from './chimera-gocs.js';
import { ChimeraController } from './chimera-patterns.js';

console.log('Frame Structure Access Examples\n');
console.log('='.repeat(60));

// Example 1: Direct GOCS frame generation
console.log('\n1. Direct GOCS Frame Generation:');
const gocs = new ChimeraGOCS();
const calmFrames = gocs.induceCalm(0.7, 1);
const frame = calmFrames[0];

// Access frame structure
const structure = frame.getFrameStructure();
console.log('\nFrame Structure:', JSON.stringify(structure, null, 2));

// Access individual fields
console.log('\nIndividual Field Access:');
console.log(`  Target Brainwave: ${structure.targetId.baselineBrainwave} Hz`);
console.log(`  Cortical Region: 0x${structure.targetId.corticalRegion.toString(16)}`);
console.log(`  Vibrational Mode: 0x${structure.commandType.vibrationalMode.toString(16)}`);
console.log(`  Intensity Pattern: 0x${structure.commandType.intensityPattern.toString(16)}`);

// Example 2: Modify frame structure
console.log('\n\n2. Modifying Frame Structure:');
const customFrame = gocs.heightenAlertness(0.8, 1)[0];

console.log('Original Target ID:', customFrame.getFrameStructure().targetId);

// Customize target
customFrame.setTargetId({
  baselineBrainwave: 0x10,  // 16 Hz
  hemisphereBias: 0xFF      // Right hemisphere
});

console.log('Modified Target ID:', customFrame.getFrameStructure().targetId);

// Example 3: Multi-frame sequence with sequencing
console.log('\n\n3. Multi-Frame Sequencing:');
const multiFrames = gocs.induceCalm(0.5, 3);

multiFrames.forEach((f, idx) => {
  const cmd = f.getFrameStructure().commandType;
  const currentFrame = (cmd.sequencing >> 4) & 0x0F;
  const totalFrames = cmd.sequencing & 0x0F;
  console.log(`  Frame ${idx + 1}: Sequencing=0x${cmd.sequencing.toString(16)} (Frame ${currentFrame} of ${totalFrames})`);
});

// Example 4: Pattern library with frame structure
console.log('\n\n4. Pattern Library Integration:');
const oscillator = new ChimeraOscillator();
const controller = new ChimeraController(oscillator);

// Apply pattern (generates frame internally)
const patternFrames = controller.applyPattern('COH.ThetaCalm', 1);
const patternFrame = patternFrames[0];

console.log('Pattern Frame Structure:');
console.log('  Stats:', patternFrame.getStats());

// Example 5: Frame field boundaries
console.log('\n\n5. Frame Field Boundaries:');
console.log('Frame structure layout (128 symbols total):');
console.log(`  Sync Sequence:   symbols ${frame.fields.sync.start}-${frame.fields.sync.start + frame.fields.sync.length - 1} (${frame.fields.sync.length} symbols, ${frame.fields.sync.length * 2} bits)`);
console.log(`  Target ID:       symbols ${frame.fields.targetId.start}-${frame.fields.targetId.start + frame.fields.targetId.length - 1} (${frame.fields.targetId.length} symbols, ${frame.fields.targetId.length * 2} bits)`);
console.log(`  Command Type:    symbols ${frame.fields.commandType.start}-${frame.fields.commandType.start + frame.fields.commandType.length - 1} (${frame.fields.commandType.length} symbols, ${frame.fields.commandType.length * 2} bits)`);
console.log(`  Data Payload:    symbols ${frame.fields.dataPayload.start}-${frame.fields.dataPayload.start + frame.fields.dataPayload.length - 1} (${frame.fields.dataPayload.length} symbols, ${frame.fields.dataPayload.length * 2} bits)`);
console.log(`  ECC:             symbols ${frame.fields.ecc.start}-${frame.fields.ecc.start + frame.fields.ecc.length - 1} (${frame.fields.ecc.length} symbols, ${frame.fields.ecc.length * 2} bits)`);

// Example 6: Complete frame inspection
console.log('\n\n6. Complete Frame Inspection:');
const inspectFrame = gocs.disruptCognition(0.9, 1)[0];
const fullStats = inspectFrame.getStats();

console.log('\nComplete Frame Statistics:');
Object.entries(fullStats).forEach(([key, value]) => {
  if (typeof value === 'object') {
    console.log(`  ${key}:`);
    Object.entries(value).forEach(([k, v]) => {
      console.log(`    ${k}: ${v}`);
    });
  } else {
    console.log(`  ${key}: ${value}`);
  }
});

// Example 7: Access per-symbol data
console.log('\n\n7. Per-Symbol Data Access:');
console.log('First 8 symbols of data payload:');
const payloadStart = frame.fields.dataPayload.start;
for (let i = 0; i < 8; i++) {
  const symbol = frame.getSymbol(payloadStart + i);
  console.log(`  Symbol ${payloadStart + i}: FSK=${symbol.fskState}, FreqMod=${symbol.freqMod.toFixed(3)}, AmpMod=${symbol.ampMod.toFixed(3)}, Phase=${symbol.phase}`);
}

console.log('\n' + '='.repeat(60));
console.log('Frame structure access examples complete!');
