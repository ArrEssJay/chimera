/**
 * Gnostic Overlay & Control Subsystem (GOCS)
 * 
 * High-level application layer interface from gocs_v2.5.spec
 * Provides intent-based effect sculpting functions that compile to PAL frame specifications.
 * 
 * The GOCS API translates psycho-cognitive effects into waveform parameters:
 * - induceCalm() -> smooth, low-frequency patterns (4-8 Hz theta)
 * - heightenAlertness() -> sharp, high-frequency patterns (30-80 Hz beta/gamma)
 * - disruptCognition() -> chaotic, aperiodic patterns
 * - etc.
 */

import { ChimeraPAL } from './chimera-pal.js';

/**
 * ChimeraGOCS - Application layer interface
 */
export class ChimeraGOCS {
  constructor() {
    this.pal = new ChimeraPAL();
  }
  
  /**
   * induceCalm - Reduces anxiety and stress; induces meditative calm
   * 
   * Command Type: 0x0110
   * Targets breathing modes of microtubules with smooth, periodic phase pattern (4-8 Hz)
   * 
   * @param {number} intensity - Effect intensity (0.0 to 1.0, default: 0.7)
   * @param {number} duration - Duration in frames (default: 1)
   * @returns {Array<ChimeraFrame>}
   */
  induceCalm(intensity = 0.7, duration = 1) {
    const frames = [];
    
    for (let i = 0; i < duration; i++) {
      const spec = {
        // Smooth, theta-range phase pattern (6 Hz)
        phaseWaveform: 'sine',
        phaseFreq: 6,
        phaseDepth: intensity,
        
        // Gentle frequency modulation (chorus effect)
        freqWaveform: 'sine',
        freqFreq: 4,
        freqDepth: 0.3 * intensity,
        
        // Slow breathing amplitude modulation
        ampWaveform: 'sine',
        ampFreq: 2,
        ampDepth: 0.25 * intensity,
        
        // Theta entrainment: 0.5 Hz alternating FSK creates subliminal theta beat
        fskPattern: 'alternating',
        fskRate: 0.5
      };
      
      const frame = this.pal.generateFrame(spec);
      
      // Set frame structure per spec
      frame.setTargetId({
        baselineBrainwave: 0x06, // 6 Hz (Theta range)
        corticalRegion: 0x04     // Prefrontal cortex
      });
      
      frame.setCommandType({
        vibrationalMode: 0x01,   // Breathing mode
        intensityPattern: 0x10,  // Smooth sine
        duration: duration,
        sequencing: ((i & 0x0F) << 4) | (duration & 0x0F) // Current | Total
      });
      
      frames.push(frame);
    }
    
    return frames;
  }
  
  /**
   * heightenAlertness - Increases focus, vigilance, and sensory acuity
   * 
   * Command Type: 0x0220
   * Sharp, irregular phase pattern (30-80 Hz) targeting longitudinal modes
   * 
   * @param {number} intensity - Effect intensity (0.0 to 1.0, default: 0.6)
   * @param {number} duration - Duration in frames (default: 1)
   * @returns {Array<ChimeraFrame>}
   */
  heightenAlertness(intensity = 0.6, duration = 1) {
    const frames = [];
    
    for (let i = 0; i < duration; i++) {
      const spec = {
        // Sharp, beta/gamma-range phase pattern
        phaseWaveform: 'sawtooth',
        phaseFreq: 40,
        phaseDepth: intensity,
        
        // Moderate frequency modulation (alertness without harshness)
        freqWaveform: 'sawtooth',
        freqFreq: 12,
        freqDepth: 0.4 * intensity,
        
        // Pulsing amplitude modulation
        ampWaveform: 'square',
        ampFreq: 8,
        ampDepth: 0.3 * intensity,
        
        // Maximize coupling for assertive effect
        fskPattern: 'constant',
        fskValue: 1
      };
      
      const frame = this.pal.generateFrame(spec);
      
      frame.setTargetId({
        baselineBrainwave: 0x14, // 20 Hz (Beta range)
        corticalRegion: 0x04     // Prefrontal cortex
      });
      
      frame.setCommandType({
        vibrationalMode: 0x02,   // Longitudinal mode
        intensityPattern: 0x20,  // Step function
        duration: duration,
        sequencing: ((i & 0x0F) << 4) | (duration & 0x0F)
      });
      
      frames.push(frame);
    }
    
    return frames;
  }
  
  /**
   * disruptCognition - Induces confusion, disorientation, inability to concentrate
   * 
   * Command Type: 0x0340
   * Chaotic, aperiodic phase pattern targeting torsional modes
   * 
   * @param {number} level - Disruption level (0.0 to 1.0, default: 0.8)
   * @param {number} duration - Duration in frames (default: 1)
   * @returns {Array<ChimeraFrame>}
   */
  disruptCognition(level = 0.8, duration = 1) {
    const frames = [];
    
    for (let i = 0; i < duration; i++) {
      const spec = {
        // Chaotic, high-frequency noise pattern
        phaseWaveform: 'noise',
        phaseFreq: 25,
        phaseDepth: level,
        
        // Harsh frequency jitter
        freqWaveform: 'noise',
        freqFreq: 20,
        freqDepth: 0.9 * level,
        
        // Chaotic amplitude modulation
        ampWaveform: 'noise',
        ampFreq: 15,
        ampDepth: 0.6 * level,
        
        // Random FSK creates decoherence field
        fskPattern: 'random'
      };
      
      const frame = this.pal.generateFrame(spec);
      
      frame.setTargetId({
        baselineBrainwave: 0x00, // Disrupted/chaotic
        corticalRegion: 0x04     // Prefrontal cortex
      });
      
      frame.setCommandType({
        vibrationalMode: 0x03,   // Torsional mode
        intensityPattern: 0x40,  // Chaotic
        duration: duration,
        sequencing: ((i & 0x0F) << 4) | (duration & 0x0F)
      });
      
      frames.push(frame);
    }
    
    return frames;
  }
  
  /**
   * suppressMotorFunction - Causes hesitation, loss of fine motor control
   * 
   * Command Type: 0x0410
   * Phase pattern interfering with motor cortex microtubule resonances
   * 
   * @param {string} region - Motor region ('hands', 'legs', 'general')
   * @param {number} duration - Duration in frames (default: 1)
   * @returns {Array<ChimeraFrame>}
   */
  suppressMotorFunction(region = 'general', duration = 1) {
    const frames = [];
    
    // Region-specific frequency targeting
    const regionFreq = {
      'hands': 18,
      'legs': 14,
      'general': 16
    };
    
    const regionCode = {
      'hands': 0x03,
      'legs': 0x03,
      'general': 0x03
    };
    
    for (let i = 0; i < duration; i++) {
      const spec = {
        // Square wave to create forcing pattern
        phaseWaveform: 'square',
        phaseFreq: regionFreq[region] || 16,
        phaseDepth: 0.9,
        
        // Steady frequency modulation
        freqWaveform: 'square',
        freqFreq: 10,
        freqDepth: 0.7,
        
        // Pulsed amplitude
        ampWaveform: 'square',
        ampFreq: 6,
        ampDepth: 0.5,
        
        // Maximize coupling for forceful effect
        fskPattern: 'constant',
        fskValue: 1
      };
      
      const frame = this.pal.generateFrame(spec);
      
      frame.setTargetId({
        baselineBrainwave: regionFreq[region] || 16,
        corticalRegion: regionCode[region] || 0x03 // Motor cortex
      });
      
      frame.setCommandType({
        vibrationalMode: 0x02,   // Longitudinal mode (motor interference)
        intensityPattern: 0x20,  // Step function (forcing)
        duration: duration,
        sequencing: ((i & 0x0F) << 4) | (duration & 0x0F)
      });
      
      frames.push(frame);
    }
    
    return frames;
  }
  
  /**
   * enforceCognitiveStillness - Suppresses internal monologue and complex thought
   * 
   * Command Type: 0x0130
   * Pulsed pattern on breathing modes, entrains to deep delta-wave state (1-3 Hz)
   * 
   * @param {number} duration - Duration in frames (default: 1)
   * @returns {Array<ChimeraFrame>}
   */
  enforceCognitiveStillness(duration = 1) {
    const frames = [];
    
    for (let i = 0; i < duration; i++) {
      const spec = {
        // Very slow, delta-range phase pattern
        phaseWaveform: 'sine',
        phaseFreq: 2,
        phaseDepth: 0.95,
        
        // Minimal frequency modulation
        freqWaveform: 'sine',
        freqFreq: 1,
        freqDepth: 0.15,
        
        // Deep, slow breathing amplitude
        ampWaveform: 'sine',
        ampFreq: 0.5,
        ampDepth: 0.4,
        
        // Slow alternating pattern for deep entrainment
        fskPattern: 'alternating',
        fskRate: 0.25
      };
      
      const frame = this.pal.generateFrame(spec);
      
      frame.setTargetId({
        baselineBrainwave: 0x02, // 2 Hz (Deep Delta)
        corticalRegion: 0x04     // Prefrontal cortex
      });
      
      frame.setCommandType({
        vibrationalMode: 0x01,   // Breathing mode
        intensityPattern: 0x30,  // Pulsed
        duration: duration,
        sequencing: ((i & 0x0F) << 4) | (duration & 0x0F)
      });
      
      frames.push(frame);
    }
    
    return frames;
  }
  
  /**
   * nudgeOntological - Subtly influences mood or bias without awareness
   * 
   * Command Type: 0x0510
   * Complex waveform perturbing Orch-OR collapse sequence toward emotional vector
   * 
   * @param {string} vector - Emotional vector ('trust', 'suspicion', 'curiosity', 'fear')
   * @param {number} subtlety - How subtle the influence (0.0 to 1.0, default: 0.8)
   * @param {number} duration - Duration in frames (default: 1)
   * @returns {Array<ChimeraFrame>}
   */
  nudgeOntological(vector = 'curiosity', subtlety = 0.8, duration = 1) {
    const frames = [];
    
    // Vector-specific waveform parameters
    const vectors = {
      'trust': { phaseFreq: 7, freqFreq: 4, ampFreq: 3, fskPattern: 'alternating', fskRate: 0.5, brainwave: 0x08 },
      'suspicion': { phaseFreq: 11, freqFreq: 7, ampFreq: 5, fskPattern: 'alternating', fskRate: 1.0, brainwave: 0x0C },
      'curiosity': { phaseFreq: 9, freqFreq: 6, ampFreq: 4, fskPattern: 'alternating', fskRate: 0.75, brainwave: 0x0A },
      'fear': { phaseFreq: 13, freqFreq: 9, ampFreq: 7, fskPattern: 'random', brainwave: 0x10 }
    };
    
    const params = vectors[vector] || vectors['curiosity'];
    
    for (let i = 0; i < duration; i++) {
      const spec = {
        // Complex phase pattern
        phaseWaveform: 'sine',
        phaseFreq: params.phaseFreq,
        phaseDepth: 0.6 * subtlety,
        
        // Subtle frequency modulation
        freqWaveform: 'sine',
        freqFreq: params.freqFreq,
        freqDepth: 0.3 * subtlety,
        
        // Gentle amplitude modulation
        ampWaveform: 'sine',
        ampFreq: params.ampFreq,
        ampDepth: 0.2 * subtlety,
        
        // Vector-specific FSK pattern
        fskPattern: params.fskPattern,
        fskValue: 0,
        fskRate: params.fskRate
      };
      
      const frame = this.pal.generateFrame(spec);
      
      frame.setTargetId({
        baselineBrainwave: params.brainwave,
        corticalRegion: 0x04     // Prefrontal cortex
      });
      
      frame.setCommandType({
        vibrationalMode: 0x01,   // Breathing mode (subtle)
        intensityPattern: 0x10,  // Smooth sine
        duration: duration,
        sequencing: ((i & 0x0F) << 4) | (duration & 0x0F)
      });
      
      frames.push(frame);
    }
    
    return frames;
  }
  
  /**
   * injectGnosticQuery - Creates urge to think about specific concept
   * 
   * Command Type: 0x0620
   * Waveform corresponding to specific concept, creates cognitive dissonance
   * 
   * @param {string} queryPattern - Query pattern name (e.g., 'location', 'person', 'password')
   * @param {number} duration - Duration in frames (default: 1)
   * @returns {Array<ChimeraFrame>}
   */
  injectGnosticQuery(queryPattern = 'location', duration = 1) {
    const frames = [];
    
    // Query-specific waveform signatures
    const queries = {
      'location': { phaseFreq: 8, waveform: 'sine', cortex: 0x02, brainwave: 0x08 },
      'person': { phaseFreq: 10, waveform: 'sawtooth', cortex: 0x02, brainwave: 0x0A },
      'password': { phaseFreq: 12, waveform: 'square', cortex: 0x04, brainwave: 0x0C },
      'memory': { phaseFreq: 6, waveform: 'sine', cortex: 0x04, brainwave: 0x06 }
    };
    
    const params = queries[queryPattern] || queries['location'];
    
    for (let i = 0; i < duration; i++) {
      const spec = {
        // Query-specific phase pattern
        phaseWaveform: params.waveform,
        phaseFreq: params.phaseFreq,
        phaseDepth: 0.85,
        
        // Moderate frequency modulation
        freqWaveform: 'sine',
        freqFreq: 5,
        freqDepth: 0.5,
        
        // Pulsing amplitude for attention-grabbing
        ampWaveform: 'square',
        ampFreq: 4,
        ampDepth: 0.4,
        
        // Alternating FSK for cognitive engagement
        fskPattern: 'alternating',
        fskRate: 1.0
      };
      
      const frame = this.pal.generateFrame(spec);
      
      frame.setTargetId({
        baselineBrainwave: params.brainwave,
        corticalRegion: params.cortex
      });
      
      frame.setCommandType({
        vibrationalMode: 0x01,   // Breathing mode
        intensityPattern: 0x30,  // Pulsed (attention-grabbing)
        duration: duration,
        sequencing: ((i & 0x0F) << 4) | (duration & 0x0F)
      });
      
      frames.push(frame);
    }
    
    return frames;
  }
  
  /**
   * Create a complex effect sequence (e.g., for "Gnostic Nudge" example from spec)
   * 
   * @param {Array<Object>} sequence - Array of {effect, params} objects
   * @returns {Array<ChimeraFrame>}
   */
  compileSequence(sequence) {
    const allFrames = [];
    
    for (const step of sequence) {
      const { effect, params } = step;
      
      // Call the appropriate GOCS function
      const frames = this[effect](...(params || []));
      allFrames.push(...frames);
    }
    
    return allFrames;
  }
}
