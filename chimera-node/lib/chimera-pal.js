/**
 * Protocol Abstraction Layer (PAL) - Waveform Generation Models
 * 
 * Implements the PAL layer from pal_waveform_models_v1.1.spec:
 * - Phase Rotation Sequence Generation (QPSK sampling from LFOs)
 * - Vibrational Texture Generation (Freq/Amp modulation envelopes)
 * - FSK State Selection Logic
 * 
 * The PAL samples LFOs at 16Hz to generate ChimeraFrame objects containing
 * 128 symbols of pre-rendered control data.
 */

import { ChimeraFrame } from './chimera-frame.js';
import { LFO } from './chimera-oscillator.js';

/**
 * PAL - Protocol Abstraction Layer
 * Generates ChimeraFrame objects from waveform specifications
 */
export class ChimeraPAL {
  constructor() {
    this.symbolRate = 16; // Hz - symbol rate from spec
    this.symbolsPerFrame = 128; // 8 seconds at 16 Hz
    this.frameDuration = 8.0; // seconds
  }
  
  /**
   * Generate a frame by sampling LFOs at symbol rate (16 Hz)
   * 
   * @param {Object} spec - Waveform specification
   * @param {string} spec.phaseWaveform - LFO waveform for phase ('sine', 'sawtooth', 'square', 'noise')
   * @param {number} spec.phaseFreq - Frequency of phase LFO in Hz
   * @param {number} spec.phaseDepth - Depth of phase modulation (0.0-1.0)
   * @param {string} spec.freqWaveform - LFO waveform for frequency modulation
   * @param {number} spec.freqFreq - Frequency of freq modulation LFO in Hz
   * @param {number} spec.freqDepth - Depth of freq modulation (0.0-1.0)
   * @param {string} spec.ampWaveform - LFO waveform for amplitude modulation
   * @param {number} spec.ampFreq - Frequency of amp modulation LFO in Hz
   * @param {number} spec.ampDepth - Depth of amp modulation (0.0-1.0)
   * @param {string} spec.fskPattern - FSK pattern ('constant', 'alternating', 'random')
   * @param {number} spec.fskValue - Base FSK state (0 or 1) for 'constant' pattern
   * @param {number} spec.fskRate - Alternation rate in Hz for 'alternating' pattern
   * @returns {ChimeraFrame}
   */
  generateFrame(spec) {
    const frame = new ChimeraFrame();
    
    // Create LFOs for sampling (with depth=1.0, we'll scale by depth parameter later)
    const phaseLFO = new LFO(spec.phaseFreq || 6, spec.phaseWaveform || 'sine', 1.0);
    const freqLFO = new LFO(spec.freqFreq || 4, spec.freqWaveform || 'sine', 1.0);
    const ampLFO = new LFO(spec.ampFreq || 3, spec.ampWaveform || 'sine', 1.0);
    
    const phaseDepth = spec.phaseDepth !== undefined ? spec.phaseDepth : 1.0;
    const freqDepth = spec.freqDepth !== undefined ? spec.freqDepth : 0.5;
    const ampDepth = spec.ampDepth !== undefined ? spec.ampDepth : 0.3;
    
    // Sample LFOs at each symbol (16 Hz)
    for (let symbolIdx = 0; symbolIdx < this.symbolsPerFrame; symbolIdx++) {
      const time = symbolIdx / this.symbolRate; // Time in seconds
      
      // Sample phase LFO and quantize to QPSK (0, 1, 2, 3)
      const phaseValue = phaseLFO.sample(time); // -1.0 to 1.0
      const phaseScaled = (phaseValue + 1.0) * 0.5 * phaseDepth; // 0.0 to 1.0, scaled by depth
      const phaseQPSK = Math.floor(phaseScaled * 4) % 4;
      frame.setPhaseRotation(symbolIdx, phaseQPSK);
      
      // Sample frequency modulation LFO
      const freqValue = freqLFO.sample(time) * freqDepth; // -depth to +depth
      frame.setFreqModulation(symbolIdx, freqValue);
      
      // Sample amplitude modulation LFO
      let ampScaled;
      if (ampDepth === 0) {
        // No amplitude modulation - constant full amplitude
        ampScaled = 1.0;
      } else {
        const ampValue = ampLFO.sample(time); // -1.0 to 1.0
        
        // Map to create a "breathing" effect that doesn't go to silence
        // For maximum depth (1.0), vary between 0.3 and 1.0 (not 0.0 to 1.0)
        // This maintains the signal presence while creating dynamic variation
        const minAmp = 1.0 - (ampDepth * 0.7); // At max depth, minimum is 0.3
        ampScaled = minAmp + ((1.0 + ampValue) * 0.5 * (1.0 - minAmp));
      }
      frame.setAmpModulation(symbolIdx, ampScaled);
    }
    
    // Set FSK pattern
    const fskPattern = spec.fskPattern || 'constant';
    const fskValue = spec.fskValue !== undefined ? spec.fskValue : 0;
    const fskRate = spec.fskRate || 0.5;
    frame.setFSKPattern(fskPattern, fskValue, fskRate);
    
    return frame;
  }
  
  /**
   * Generate a frame from a base pattern specification (simplified interface)
   * 
   * @param {Object} pattern - Base pattern object
   * @param {string} pattern.name - Pattern name
   * @param {string} pattern.category - Pattern category
   * @param {Object} pattern.phaseRotation - Phase LFO config
   * @param {Object} pattern.freqModulation - Freq LFO config
   * @param {Object} pattern.ampModulation - Amp LFO config
   * @param {Object} pattern.fskState - FSK config
   * @returns {ChimeraFrame}
   */
  generateFrameFromPattern(pattern) {
    const spec = {
      // Phase rotation
      phaseWaveform: pattern.phaseRotation?.waveform || 'sine',
      phaseFreq: pattern.phaseRotation?.frequency || 6,
      phaseDepth: pattern.phaseRotation?.depth || 1.0,
      
      // Frequency modulation
      freqWaveform: pattern.freqModulation?.waveform || 'sine',
      freqFreq: pattern.freqModulation?.frequency || 4,
      freqDepth: pattern.freqModulation?.depth || 0.5,
      
      // Amplitude modulation
      ampWaveform: pattern.ampModulation?.waveform || 'sine',
      ampFreq: pattern.ampModulation?.frequency || 3,
      ampDepth: pattern.ampModulation?.depth || 0.3,
      
      // FSK state
      fskPattern: pattern.fskState?.pattern || 'constant',
      fskValue: pattern.fskState?.value !== undefined ? pattern.fskState.value : 0,
      fskRate: pattern.fskState?.rate || 0.5
    };
    
    return this.generateFrame(spec);
  }
  
  /**
   * Generate multiple frames in sequence (for multi-frame operations)
   * 
   * @param {Array<Object>} specs - Array of waveform specifications
   * @returns {Array<ChimeraFrame>}
   */
  generateFrameSequence(specs) {
    return specs.map(spec => this.generateFrame(spec));
  }
  
  /**
   * Create a smooth transition between two frames
   * 
   * @param {ChimeraFrame} frameA - Starting frame
   * @param {ChimeraFrame} frameB - Ending frame
   * @param {number} numSteps - Number of transition frames (default: 4)
   * @returns {Array<ChimeraFrame>}
   */
  createTransition(frameA, frameB, numSteps = 4) {
    const frames = [frameA];
    
    for (let step = 1; step < numSteps; step++) {
      const t = step / numSteps; // 0.0 to 1.0
      const frame = new ChimeraFrame();
      
      for (let i = 0; i < frame.symbolsPerFrame; i++) {
        // Interpolate frequency modulation
        const freqA = frameA.freqModulation[i];
        const freqB = frameB.freqModulation[i];
        frame.setFreqModulation(i, freqA + (freqB - freqA) * t);
        
        // Interpolate amplitude modulation
        const ampA = frameA.ampModulation[i];
        const ampB = frameB.ampModulation[i];
        frame.setAmpModulation(i, ampA + (ampB - ampA) * t);
        
        // Phase rotation - use frameA for first half, frameB for second half
        const phase = t < 0.5 ? frameA.phaseRotation[i] : frameB.phaseRotation[i];
        frame.setPhaseRotation(i, phase);
        
        // FSK state - use frameA for first half, frameB for second half
        const fsk = t < 0.5 ? frameA.fskStates[i] : frameB.fskStates[i];
        frame.setFSKState(i, fsk);
      }
      
      frames.push(frame);
    }
    
    frames.push(frameB);
    return frames;
  }
}
