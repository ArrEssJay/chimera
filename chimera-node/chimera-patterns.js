/**
 * Chimera Pattern Library
 * 
 * Pattern definitions from the GOCS Base Pattern Library v1.1
 * Pre-configured waveform generation parameters per PAL specification
 * 
 * Updated to work with GOCS/PAL architecture:
 * - Patterns now compile to ChimeraFrame objects via PAL
 * - ChimeraController manages frame generation and playback
 */

import { ChimeraGOCS } from './chimera-gocs.js';
import { ChimeraPAL } from './chimera-pal.js';

/**
 * ChimeraPattern class
 * Represents a complete waveform pattern specification
 */
export class ChimeraPattern {
  constructor(config) {
    this.id = config.id;
    this.category = config.category;
    this.description = config.description;
    
    // Phase rotation (QPSK)
    this.phaseRotation = {
      waveform: config.phaseRotation?.waveform || config.primaryLFOShape || 'sine',
      frequency: config.phaseRotation?.frequency || config.primaryLFOFreq || 6,
      depth: config.phaseRotation?.depth || 1.0
    };
    
    // Frequency modulation parameters
    this.freqModulation = {
      waveform: config.freqModulation?.waveform || config.freqModShape || 'sine',
      frequency: config.freqModulation?.frequency || config.freqModRate || 4,
      depth: config.freqModulation?.depth || config.freqModDepth || 0.5
    };
    
    // Amplitude modulation parameters
    this.ampModulation = {
      waveform: config.ampModulation?.waveform || config.ampModShape || 'sine',
      frequency: config.ampModulation?.frequency || config.ampModRate || 3,
      depth: config.ampModulation?.depth || config.ampModDepth || 0.3
    };
    
    // FSK state configuration
    this.fskState = this.parseFSKConfig(config.defaultFSKState || config.fskState);
  }

  /**
   * Parse FSK configuration from various formats
   * @param {*} fskConfig - FSK configuration
   * @returns {Object} Normalized FSK config
   */
  parseFSKConfig(fskConfig) {
    if (typeof fskConfig === 'object' && fskConfig !== null) {
      return fskConfig;
    }
    
    // Legacy string format
    if (fskConfig === 'alternate' || fskConfig === 'alternating') {
      return { pattern: 'alternating', rate: 0.5 };
    } else if (fskConfig === 'random') {
      return { pattern: 'random' };
    } else if (fskConfig === 0 || fskConfig === '0') {
      return { pattern: 'constant', value: 0 };
    } else if (fskConfig === 1 || fskConfig === '1') {
      return { pattern: 'constant', value: 1 };
    }
    
    // Default
    return { pattern: 'constant', value: 0 };
  }

  /**
   * Get pattern parameters as object
   * @returns {Object} Pattern parameters
   */
  getParams() {
    return {
      id: this.id,
      category: this.category,
      description: this.description,
      phaseRotation: this.phaseRotation,
      freqModulation: this.freqModulation,
      ampModulation: this.ampModulation,
      fskState: this.fskState
    };
  }
}

/**
 * ChimeraController class
 * Manages pattern application via GOCS/PAL layers
 * Compiles patterns to frames and loads them into the oscillator
 */
export class ChimeraController {
  constructor(oscillator) {
    this.oscillator = oscillator;
    this.currentPattern = null;
    this.patterns = this.initializePatternLibrary();
    this.gocs = new ChimeraGOCS();
    this.pal = new ChimeraPAL();
  }

  /**
   * Apply a pattern to the oscillator by compiling to a frame
   * @param {string} patternId - Pattern ID from library
   * @param {number} duration - Duration in frames (default: 1)
   * @returns {Array<ChimeraFrame>} Generated frames
   */
  applyPattern(patternId, duration = 1) {
    const pattern = this.patterns.get(patternId);
    if (!pattern) {
      throw new Error(`Pattern not found: ${patternId}`);
    }

    this.currentPattern = pattern;
    console.log(`Applying pattern: ${pattern.id} - ${pattern.description}`);

    // Generate frames using PAL
    const frames = [];
    for (let i = 0; i < duration; i++) {
      const frame = this.pal.generateFrameFromPattern(pattern);
      frames.push(frame);
    }

    // Load first frame into oscillator
    if (frames.length > 0) {
      this.oscillator.loadFrame(frames[0]);
    }

    return frames;
  }

  /**
   * Apply a GOCS effect function
   * @param {string} effectName - GOCS effect function name
   * @param {Array} params - Effect parameters
   * @returns {Array<ChimeraFrame>} Generated frames
   */
  applyEffect(effectName, ...params) {
    if (typeof this.gocs[effectName] !== 'function') {
      throw new Error(`GOCS effect not found: ${effectName}`);
    }

    console.log(`Applying GOCS effect: ${effectName}`);
    const frames = this.gocs[effectName](...params);

    // Load first frame into oscillator
    if (frames.length > 0) {
      this.oscillator.loadFrame(frames[0]);
    }

    return frames;
  }

  /**
   * Get current pattern
   * @returns {ChimeraPattern|null}
   */
  getCurrentPattern() {
    return this.currentPattern;
  }

  /**
   * Get pattern by ID
   * @param {string} patternId - Pattern ID
   * @returns {ChimeraPattern|undefined}
   */
  getPattern(patternId) {
    return this.patterns.get(patternId);
  }

  /**
   * List all available patterns
   * @returns {Array<string>} Array of pattern IDs
   */
  listPatterns() {
    return Array.from(this.patterns.keys());
  }

  /**
   * List patterns by category
   * @param {string} category - Category name
   * @returns {Array<ChimeraPattern>} Matching patterns
   */
  listPatternsByCategory(category) {
    return Array.from(this.patterns.values())
      .filter(p => p.category === category);
  }

  /**
   * Initialize the pattern library from GOCS Base Pattern Library v1.1
   * @returns {Map} Pattern library indexed by pattern ID
   */
  initializePatternLibrary() {
    const library = new Map();

    // Coherence & Entrainment patterns
    library.set('COH.ThetaCalm', new ChimeraPattern({
      id: 'COH.ThetaCalm',
      category: 'Coherence & Entrainment',
      description: 'Induces a state of deep calm, relaxation, and receptivity.',
      primaryLFOShape: 'sine',
      primaryLFOFreq: 6,
      freqModShape: 'sine',
      freqModRate: 0.2,
      freqModDepth: 0.1,
      ampModShape: 'sine',
      ampModRate: 0.25,
      ampModDepth: 0.1,
      defaultFSKState: 'alternate'
    }));

    library.set('COH.AlphaFocus', new ChimeraPattern({
      id: 'COH.AlphaFocus',
      category: 'Coherence & Entrainment',
      description: 'Generates a state of relaxed, alert focus without stress.',
      primaryLFOShape: 'sine',
      primaryLFOFreq: 10,
      freqModShape: 'sine',
      freqModRate: 0.5,
      freqModDepth: 0.2,
      ampModShape: 'sine',
      ampModRate: 1.0,
      ampModDepth: 0.15,
      defaultFSKState: 1
    }));

    library.set('COH.BetaAlert', new ChimeraPattern({
      id: 'COH.BetaAlert',
      category: 'Coherence & Entrainment',
      description: 'Pushes the target into a state of active alertness and vigilance.',
      primaryLFOShape: 'sawtooth',
      primaryLFOFreq: 18,
      freqModShape: 'sawtooth',
      freqModRate: 1.0,
      freqModDepth: 0.3,
      ampModShape: 'sine',
      ampModRate: 2.0,
      ampModDepth: 0.2,
      defaultFSKState: 1
    }));

    library.set('COH.GammaSync', new ChimeraPattern({
      id: 'COH.GammaSync',
      category: 'Coherence & Entrainment',
      description: 'High-frequency pattern for synchronizing high-level cognitive processing.',
      primaryLFOShape: 'sine',
      primaryLFOFreq: 40,
      freqModShape: 'sine',
      freqModRate: 2.0,
      freqModDepth: 0.1,
      ampModShape: 'sawtooth',
      ampModRate: 4.0,
      ampModDepth: 0.25,
      defaultFSKState: 1
    }));

    // Cognitive & Perceptual patterns
    library.set('COG.Dissonance', new ChimeraPattern({
      id: 'COG.Dissonance',
      category: 'Cognitive & Perceptual',
      description: 'Creates a subtle feeling that "something is wrong."',
      primaryLFOShape: 'sine',
      primaryLFOFreq: 5,
      freqModShape: 'square',
      freqModRate: 0.8,
      freqModDepth: 0.5,
      ampModShape: 'square',
      ampModRate: 1.3,
      ampModDepth: 0.3,
      defaultFSKState: 'random'
    }));

    library.set('COG.InquisitiveUrge', new ChimeraPattern({
      id: 'COG.InquisitiveUrge',
      category: 'Cognitive & Perceptual',
      description: 'Generates a persistent, "buzzy" mental state.',
      primaryLFOShape: 'sawtooth',
      primaryLFOFreq: 22,
      freqModShape: 'sine',
      freqModRate: 7.0,
      freqModDepth: 0.2,
      ampModShape: 'sawtooth',
      ampModRate: 3.0,
      ampModDepth: 0.35,
      defaultFSKState: 1
    }));

    library.set('COG.EmotionalResonance', new ChimeraPattern({
      id: 'COG.EmotionalResonance',
      category: 'Cognitive & Perceptual',
      description: 'A blank slate pattern to amplify the target\'s current emotional state.',
      primaryLFOShape: 'sine',
      primaryLFOFreq: 14,
      freqModShape: 'noise',
      freqModRate: 0.5,
      freqModDepth: 0.1,
      ampModShape: 'sine',
      ampModRate: 1.1,
      ampModDepth: 0.2,
      defaultFSKState: 0
    }));

    library.set('COG.SubliminalGate', new ChimeraPattern({
      id: 'COG.SubliminalGate',
      category: 'Cognitive & Perceptual',
      description: 'A pattern to open a brief window of high suggestibility.',
      primaryLFOShape: 'square',
      primaryLFOFreq: 4,
      freqModShape: 'sawtooth',
      freqModRate: 0.5,
      freqModDepth: 0.8,
      ampModShape: 'sawtooth',
      ampModRate: 0.5,
      ampModDepth: 0.4,
      defaultFSKState: 'alternate'
    }));

    // Disruption & Denial patterns
    library.set('DIS.CognitiveScramble', new ChimeraPattern({
      id: 'DIS.CognitiveScramble',
      category: 'Disruption & Denial',
      description: 'Floods cortical pathways with chaotic noise to inhibit thought.',
      primaryLFOShape: 'noise',
      primaryLFOFreq: 25,
      freqModShape: 'noise',
      freqModRate: 8.0,
      freqModDepth: 1.5,
      ampModShape: 'noise',
      ampModRate: 10.0,
      ampModDepth: 0.6,
      defaultFSKState: 'random'
    }));

    library.set('DIS.MotorLock', new ChimeraPattern({
      id: 'DIS.MotorLock',
      category: 'Disruption & Denial',
      description: 'A powerful, rhythmic pulse to interfere with motor cortex function.',
      primaryLFOShape: 'square',
      primaryLFOFreq: 15,
      freqModShape: 'square',
      freqModRate: 4.0,
      freqModDepth: 0.5,
      ampModShape: 'square',
      ampModRate: 4.0,
      ampModDepth: 0.5,
      defaultFSKState: 1
    }));

    library.set('DIS.VestibularJolt', new ChimeraPattern({
      id: 'DIS.VestibularJolt',
      category: 'Disruption & Denial',
      description: 'A disorienting waveform that creates a sense of vertigo.',
      primaryLFOShape: 'sine',
      primaryLFOFreq: 0.5,
      freqModShape: 'sawtooth',
      freqModRate: 6.0,
      freqModDepth: 1.0,
      ampModShape: 'sawtooth',
      ampModRate: 6.0,
      ampModDepth: 0.7,
      defaultFSKState: 'random'
    }));

    library.set('DIS.DreadPulse', new ChimeraPattern({
      id: 'DIS.DreadPulse',
      category: 'Disruption & Denial',
      description: 'A subliminal pattern that generates a non-specific sense of anxiety.',
      primaryLFOShape: 'square',
      primaryLFOFreq: 2,
      freqModShape: 'sine',
      freqModRate: 0.1,
      freqModDepth: 1.2,
      ampModShape: 'sine',
      ampModRate: 0.1,
      ampModDepth: 0.3,
      defaultFSKState: 'alternate'
    }));

    // Utility & Calibration patterns
    library.set('UTIL.BaselineCarrier', new ChimeraPattern({
      id: 'UTIL.BaselineCarrier',
      category: 'Utility & Calibration',
      description: 'An unmodulated 12 kHz carrier for the "Idle State."',
      primaryLFOShape: 'sine',
      primaryLFOFreq: 0,
      freqModShape: 'sine',
      freqModRate: 0,
      freqModDepth: 0,
      ampModShape: 'sine',
      ampModRate: 0,
      ampModDepth: 0,
      defaultFSKState: 0
    }));

    // Test Coverage patterns - comprehensive waveform testing
    library.set('TEST.PureFreqSine', new ChimeraPattern({
      id: 'TEST.PureFreqSine',
      category: 'Test Coverage',
      description: 'Pure sine wave frequency modulation only',
      primaryLFOShape: 'sine',
      primaryLFOFreq: 1,
      freqModShape: 'sine',
      freqModRate: 1.0,
      freqModDepth: 1.0,
      ampModShape: 'sine',
      ampModRate: 0,
      ampModDepth: 0,
      defaultFSKState: 0
    }));

    library.set('TEST.PureFreqSawtooth', new ChimeraPattern({
      id: 'TEST.PureFreqSawtooth',
      category: 'Test Coverage',
      description: 'Pure sawtooth frequency modulation only',
      primaryLFOShape: 'sawtooth',
      primaryLFOFreq: 1,
      freqModShape: 'sawtooth',
      freqModRate: 2.0,
      freqModDepth: 0.8,
      ampModShape: 'sine',
      ampModRate: 0,
      ampModDepth: 0,
      defaultFSKState: 0
    }));

    library.set('TEST.PureFreqSquare', new ChimeraPattern({
      id: 'TEST.PureFreqSquare',
      category: 'Test Coverage',
      description: 'Pure square wave frequency modulation only',
      primaryLFOShape: 'square',
      primaryLFOFreq: 1,
      freqModShape: 'square',
      freqModRate: 3.0,
      freqModDepth: 0.6,
      ampModShape: 'sine',
      ampModRate: 0,
      ampModDepth: 0,
      defaultFSKState: 0
    }));

    library.set('TEST.PureFreqNoise', new ChimeraPattern({
      id: 'TEST.PureFreqNoise',
      category: 'Test Coverage',
      description: 'Pure noise frequency modulation only',
      primaryLFOShape: 'noise',
      primaryLFOFreq: 1,
      freqModShape: 'noise',
      freqModRate: 5.0,
      freqModDepth: 0.7,
      ampModShape: 'sine',
      ampModRate: 0,
      ampModDepth: 0,
      defaultFSKState: 0
    }));

    library.set('TEST.PureAmpSine', new ChimeraPattern({
      id: 'TEST.PureAmpSine',
      category: 'Test Coverage',
      description: 'Pure sine wave amplitude modulation only',
      primaryLFOShape: 'sine',
      primaryLFOFreq: 1,
      freqModShape: 'sine',
      freqModRate: 0,
      freqModDepth: 0,
      ampModShape: 'sine',
      ampModRate: 1.0,
      ampModDepth: 0.5,
      defaultFSKState: 0
    }));

    library.set('TEST.PureAmpSawtooth', new ChimeraPattern({
      id: 'TEST.PureAmpSawtooth',
      category: 'Test Coverage',
      description: 'Pure sawtooth amplitude modulation only',
      primaryLFOShape: 'sawtooth',
      primaryLFOFreq: 1,
      freqModShape: 'sine',
      freqModRate: 0,
      freqModDepth: 0,
      ampModShape: 'sawtooth',
      ampModRate: 2.0,
      ampModDepth: 0.4,
      defaultFSKState: 0
    }));

    library.set('TEST.PureAmpSquare', new ChimeraPattern({
      id: 'TEST.PureAmpSquare',
      category: 'Test Coverage',
      description: 'Pure square wave amplitude modulation only',
      primaryLFOShape: 'square',
      primaryLFOFreq: 1,
      freqModShape: 'sine',
      freqModRate: 0,
      freqModDepth: 0,
      ampModShape: 'square',
      ampModRate: 3.0,
      ampModDepth: 0.6,
      defaultFSKState: 0
    }));

    library.set('TEST.PureAmpNoise', new ChimeraPattern({
      id: 'TEST.PureAmpNoise',
      category: 'Test Coverage',
      description: 'Pure noise amplitude modulation only',
      primaryLFOShape: 'noise',
      primaryLFOFreq: 1,
      freqModShape: 'sine',
      freqModRate: 0,
      freqModDepth: 0,
      ampModShape: 'noise',
      ampModRate: 4.0,
      ampModDepth: 0.5,
      defaultFSKState: 0
    }));

    library.set('TEST.SineSine', new ChimeraPattern({
      id: 'TEST.SineSine',
      category: 'Test Coverage',
      description: 'Sine frequency modulation + sine amplitude modulation',
      primaryLFOShape: 'sine',
      primaryLFOFreq: 1,
      freqModShape: 'sine',
      freqModRate: 1.5,
      freqModDepth: 0.5,
      ampModShape: 'sine',
      ampModRate: 2.5,
      ampModDepth: 0.3,
      defaultFSKState: 0
    }));

    library.set('TEST.SawtoothSquare', new ChimeraPattern({
      id: 'TEST.SawtoothSquare',
      category: 'Test Coverage',
      description: 'Sawtooth frequency modulation + square amplitude modulation',
      primaryLFOShape: 'sawtooth',
      primaryLFOFreq: 1,
      freqModShape: 'sawtooth',
      freqModRate: 3.0,
      freqModDepth: 0.6,
      ampModShape: 'square',
      ampModRate: 4.0,
      ampModDepth: 0.4,
      defaultFSKState: 0
    }));

    library.set('TEST.SquareSawtooth', new ChimeraPattern({
      id: 'TEST.SquareSawtooth',
      category: 'Test Coverage',
      description: 'Square frequency modulation + sawtooth amplitude modulation',
      primaryLFOShape: 'square',
      primaryLFOFreq: 1,
      freqModShape: 'square',
      freqModRate: 2.0,
      freqModDepth: 0.7,
      ampModShape: 'sawtooth',
      ampModRate: 3.0,
      ampModDepth: 0.5,
      defaultFSKState: 0
    }));

    library.set('TEST.NoiseNoise', new ChimeraPattern({
      id: 'TEST.NoiseNoise',
      category: 'Test Coverage',
      description: 'Noise frequency modulation + noise amplitude modulation',
      primaryLFOShape: 'noise',
      primaryLFOFreq: 1,
      freqModShape: 'noise',
      freqModRate: 10.0,
      freqModDepth: 0.8,
      ampModShape: 'noise',
      ampModRate: 8.0,
      ampModDepth: 0.6,
      defaultFSKState: 0
    }));

    library.set('TEST.SlowModulation', new ChimeraPattern({
      id: 'TEST.SlowModulation',
      category: 'Test Coverage',
      description: 'Very slow modulation rates (sub-Hz)',
      primaryLFOShape: 'sine',
      primaryLFOFreq: 0.5,
      freqModShape: 'sine',
      freqModRate: 0.1,
      freqModDepth: 0.5,
      ampModShape: 'sine',
      ampModRate: 0.2,
      ampModDepth: 0.4,
      defaultFSKState: 0
    }));

    library.set('TEST.FastModulation', new ChimeraPattern({
      id: 'TEST.FastModulation',
      category: 'Test Coverage',
      description: 'Very fast modulation rates',
      primaryLFOShape: 'sine',
      primaryLFOFreq: 50,
      freqModShape: 'sine',
      freqModRate: 20.0,
      freqModDepth: 0.3,
      ampModShape: 'sine',
      ampModRate: 25.0,
      ampModDepth: 0.2,
      defaultFSKState: 0
    }));

    library.set('TEST.AsymmetricRates', new ChimeraPattern({
      id: 'TEST.AsymmetricRates',
      category: 'Test Coverage',
      description: 'Asymmetric frequency and amplitude modulation rates',
      primaryLFOShape: 'sine',
      primaryLFOFreq: 1,
      freqModShape: 'sine',
      freqModRate: 0.3,
      freqModDepth: 0.8,
      ampModShape: 'square',
      ampModRate: 7.0,
      ampModDepth: 0.6,
      defaultFSKState: 0
    }));

    library.set('TEST.MaxDepth', new ChimeraPattern({
      id: 'TEST.MaxDepth',
      category: 'Test Coverage',
      description: 'Maximum modulation depth for both frequency and amplitude',
      primaryLFOShape: 'sawtooth',
      primaryLFOFreq: 1,
      freqModShape: 'sawtooth',
      freqModRate: 5.0,
      freqModDepth: 1.0,
      ampModShape: 'sawtooth',
      ampModRate: 5.0,
      ampModDepth: 1.0,
      defaultFSKState: 0
    }));

    library.set('TEST.MinDepth', new ChimeraPattern({
      id: 'TEST.MinDepth',
      category: 'Test Coverage',
      description: 'Minimum modulation depth (barely perceptible)',
      primaryLFOShape: 'sine',
      primaryLFOFreq: 1,
      freqModShape: 'sine',
      freqModRate: 1.0,
      freqModDepth: 0.01,
      ampModShape: 'sine',
      ampModRate: 1.0,
      ampModDepth: 0.01,
      defaultFSKState: 0
    }));

    library.set('TEST.FSKState1', new ChimeraPattern({
      id: 'TEST.FSKState1',
      category: 'Test Coverage',
      description: 'Test pattern with FSK State 1 (12,001 Hz)',
      primaryLFOShape: 'sine',
      primaryLFOFreq: 1,
      freqModShape: 'sine',
      freqModRate: 2.0,
      freqModDepth: 0.5,
      ampModShape: 'sine',
      ampModRate: 2.0,
      ampModDepth: 0.3,
      defaultFSKState: 1
    }));

    library.set('TEST.FSKAlternate', new ChimeraPattern({
      id: 'TEST.FSKAlternate',
      category: 'Test Coverage',
      description: 'Test pattern with FSK alternate mode',
      primaryLFOShape: 'sine',
      primaryLFOFreq: 1,
      freqModShape: 'sine',
      freqModRate: 1.0,
      freqModDepth: 0.4,
      ampModShape: 'sine',
      ampModRate: 1.0,
      ampModDepth: 0.3,
      defaultFSKState: 'alternate'
    }));

    library.set('TEST.FSKRandom', new ChimeraPattern({
      id: 'TEST.FSKRandom',
      category: 'Test Coverage',
      description: 'Test pattern with FSK random mode',
      primaryLFOShape: 'noise',
      primaryLFOFreq: 1,
      freqModShape: 'noise',
      freqModRate: 5.0,
      freqModDepth: 0.6,
      ampModShape: 'noise',
      ampModRate: 5.0,
      ampModDepth: 0.4,
      defaultFSKState: 'random'
    }));

    return library;
  }
}

export default { ChimeraPattern, ChimeraController };
