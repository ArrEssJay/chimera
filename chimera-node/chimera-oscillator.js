/**
 * Chimera Waveform Generator
 * 
 * A FSK sine oscillator with LFO-driven modulation following the PAL specification:
 * - State 0: 11,999 Hz (Baseline)
 * - State 1: 12,001 Hz (Maximize Coupling)
 * 
 * Implements PAL Waveform Generation Models v1.1:
 * - Primary oscillator at 12kHz (FSK controlled)
 * - LFO_Freq for frequency modulation (vibrational texture)
 * - LFO_Amp for amplitude modulation (breathing/pulsing)
 * - Symbol rate: 16 Hz (62.5ms per symbol)
 * 
 * Supports both Web Audio API (browser) and file output (Node.js)
 */

/**
 * Low-Frequency Oscillator (LFO)
 * Used to modulate frequency and amplitude at the symbol rate (16 Hz)
 */
export class LFO {
  constructor(frequency = 1.0, shape = 'sine', depth = 0.0) {
    this.frequency = frequency;  // LFO frequency in Hz
    this.shape = shape;          // 'sine', 'sawtooth', 'square', 'noise'
    this.depth = depth;          // Modulation depth (0.0 to 1.0)
    this.phase = 0.0;            // Current phase (0.0 to 1.0)
  }

  /**
   * Set LFO frequency
   * @param {number} freq - Frequency in Hz
   */
  setFrequency(freq) {
    this.frequency = freq;
  }

  /**
   * Set LFO waveform shape
   * @param {string} shape - 'sine', 'sawtooth', 'square', 'noise'
   */
  setShape(shape) {
    const validShapes = ['sine', 'sawtooth', 'square', 'noise'];
    if (!validShapes.includes(shape)) {
      throw new Error(`Invalid LFO shape. Must be one of: ${validShapes.join(', ')}`);
    }
    this.shape = shape;
  }

  /**
   * Set modulation depth
   * @param {number} depth - Depth from 0.0 to 1.0
   */
  setDepth(depth) {
    this.depth = Math.max(0, Math.min(1, depth));
  }

  /**
   * Get LFO parameters
   * @returns {Object} Current LFO parameters
   */
  getParams() {
    return {
      frequency: this.frequency,
      shape: this.shape,
      depth: this.depth
    };
  }

  /**
   * Sample the LFO at a given time
   * @param {number} time - Time in seconds
   * @returns {number} LFO value (-1.0 to +1.0)
   */
  sample(time) {
    const phase = (time * this.frequency) % 1.0;
    let value = 0;

    switch (this.shape) {
      case 'sine':
        value = Math.sin(2 * Math.PI * phase);
        break;
      case 'sawtooth':
        value = 2 * phase - 1;
        break;
      case 'square':
        value = phase < 0.5 ? -1 : 1;
        break;
      case 'noise':
        value = Math.random() * 2 - 1;
        break;
    }

    return value * this.depth;
  }
}

/**
 * ChimeraOscillator class
 * Manages a sine wave oscillator with FSK control and LFO-driven modulation
 */
export class ChimeraOscillator {
  constructor() {
    this.fskState = 0; // 0 or 1
    this.frequency = 11999; // Hz
    this.amplitude = 1.0;
    this.phase = 0.0;
    this.masterGain = 0.8;

    // FSK frequency mapping
    this.frequencies = {
      0: 11999, // Baseline resonance
      1: 12001  // Maximize coupling
    };

    // LFOs for PAL waveform generation (legacy - now primarily driven by frames)
    this.lfoFreq = new LFO(1.0, 'sine', 0.0);  // Frequency modulation (vibrational texture)
    this.lfoAmp = new LFO(1.0, 'sine', 0.0);   // Amplitude modulation (breathing/pulsing)

    // Symbol rate (PAL spec: 16 Hz)
    this.symbolRate = 16;
    this.symbolDuration = 1.0 / this.symbolRate; // 62.5ms per symbol

    // Frame playback system
    this.currentFrame = null;
    this.frameMode = false; // true = use frame data, false = use LFOs
    this.currentSymbolIndex = 0;
  }

  /**
   * Set the FSK state (0 or 1)
   * @param {number} state - FSK state (0 or 1)
   */
  setFSKState(state) {
    if (state !== 0 && state !== 1) {
      throw new Error('FSK state must be 0 or 1');
    }
    this.fskState = state;
    this.frequency = this.frequencies[state];
    console.log(`FSK State set to ${state}, Frequency: ${this.frequency} Hz`);
  }

  /**
   * Get current frequency
   * @returns {number} Current frequency in Hz
   */
  getFrequency() {
    return this.frequency;
  }

  /**
   * Get current FSK state
   * @returns {number} Current FSK state (0 or 1)
   */
  getFSKState() {
    return this.fskState;
  }

  /**
   * Set amplitude (0.0 to 1.0)
   * @param {number} amp - Amplitude value
   */
  setAmplitude(amp) {
    this.amplitude = Math.max(0, Math.min(1, amp));
  }

  /**
   * Get current amplitude
   * @returns {number} Current amplitude
   */
  getAmplitude() {
    return this.amplitude;
  }

  /**
   * Set phase offset (0.0 to 1.0, where 1.0 = 2π)
   * @param {number} phase - Phase offset
   */
  setPhase(phase) {
    this.phase = phase;
  }

  /**
   * Get current phase
   * @returns {number} Current phase offset
   */
  getPhase() {
    return this.phase;
  }

  /**
   * Get frequency modulation LFO
   * @returns {LFO} Frequency modulation LFO
   */
  getFreqLFO() {
    return this.lfoFreq;
  }

  /**
   * Get amplitude modulation LFO
   * @returns {LFO} Amplitude modulation LFO
   */
  getAmpLFO() {
    return this.lfoAmp;
  }

  /**
   * Configure frequency modulation LFO
   * @param {number} frequency - LFO frequency in Hz
   * @param {string} shape - Waveform shape ('sine', 'sawtooth', 'square', 'noise')
   * @param {number} depth - Modulation depth (0.0 to 1.0)
   */
  setFreqModulation(frequency, shape = 'sine', depth = 0.0) {
    this.lfoFreq.setFrequency(frequency);
    this.lfoFreq.setShape(shape);
    this.lfoFreq.setDepth(depth);
    console.log(`Frequency LFO: ${frequency}Hz, ${shape}, depth ${depth}`);
  }

  /**
   * Configure amplitude modulation LFO
   * @param {number} frequency - LFO frequency in Hz
   * @param {string} shape - Waveform shape ('sine', 'sawtooth', 'square', 'noise')
   * @param {number} depth - Modulation depth (0.0 to 1.0)
   */
  setAmpModulation(frequency, shape = 'sine', depth = 0.0) {
    this.lfoAmp.setFrequency(frequency);
    this.lfoAmp.setShape(shape);
    this.lfoAmp.setDepth(depth);
    console.log(`Amplitude LFO: ${frequency}Hz, ${shape}, depth ${depth}`);
  }

  /**
   * Get symbol rate (PAL spec: 16 Hz)
   * @returns {number} Symbol rate in Hz
   */
  getSymbolRate() {
    return this.symbolRate;
  }

  /**
   * Load a ChimeraFrame for playback
   * @param {ChimeraFrame} frame - Frame to load
   */
  loadFrame(frame) {
    this.currentFrame = frame;
    this.frameMode = true;
    this.currentSymbolIndex = 0;
    console.log('Frame loaded. Frame mode enabled.');
  }

  /**
   * Disable frame mode and return to direct LFO control
   */
  disableFrameMode() {
    this.frameMode = false;
    this.currentFrame = null;
    this.currentSymbolIndex = 0;
    console.log('Frame mode disabled. Using direct LFO control.');
  }

  /**
   * Check if frame mode is enabled
   * @returns {boolean}
   */
  isFrameMode() {
    return this.frameMode;
  }

  /**
   * Get current frame
   * @returns {ChimeraFrame|null}
   */
  getCurrentFrame() {
    return this.currentFrame;
  }

  /**
   * Generate audio samples for the current oscillator state with LFO modulation
   * Samples LFOs at the symbol rate (16 Hz) per PAL specification
   * In frame mode, uses pre-rendered control data from the loaded frame
   * @param {number} numSamples - Number of samples to generate
   * @param {number} sampleRate - Sample rate in Hz
   * @param {number} startTime - Starting time offset in seconds (for continuous playback)
   * @returns {Float32Array} Audio samples
   */
  generateSamples(numSamples, sampleRate = 48000, startTime = 0) {
    const samples = new Float32Array(numSamples);
    const phaseRadians = this.phase * 2 * Math.PI;
    const samplesPerSymbol = sampleRate / this.symbolRate; // 3000 samples at 48kHz
    
    for (let i = 0; i < numSamples; i++) {
      const t = startTime + i / sampleRate;
      
      // Determine which symbol period we're in (16 Hz symbol rate)
      const symbolIndex = Math.floor(t * this.symbolRate);
      const symbolTime = symbolIndex * this.symbolDuration;
      
      let freqMod, ampMod, currentFSK;
      
      if (this.frameMode && this.currentFrame) {
        // Frame mode: use pre-rendered control data
        const symbol = this.currentFrame.getSymbol(symbolIndex);
        freqMod = symbol.freqMod;
        ampMod = symbol.ampMod;
        currentFSK = symbol.fskState;
        
        // Update FSK state at symbol boundaries (every 3000 samples at 48kHz)
        if (i % samplesPerSymbol === 0) {
          this.fskState = currentFSK;
          this.frequency = this.frequencies[currentFSK];
        }
      } else {
        // Legacy mode: sample LFOs at symbol boundaries
        freqMod = this.lfoFreq.sample(symbolTime);
        ampMod = this.lfoAmp.sample(symbolTime);
        currentFSK = this.fskState; // Use manually set FSK state
      }
      
      // Apply frequency modulation (vibrational texture)
      // freqMod ranges from -1.0 to +1.0, scaled to Hz deviation
      const freqDeviation = freqMod * 10; // Max ±10 Hz deviation
      const instantFreq = this.frequency + freqDeviation;
      
      // Apply amplitude modulation (breathing/pulsing)
      // ampMod is normalized 0.0 to 1.0 in frame mode, -depth to +depth in LFO mode
      let instantAmp;
      if (this.frameMode) {
        instantAmp = this.amplitude * ampMod;
      } else {
        instantAmp = this.amplitude * (1.0 + ampMod);
      }
      
      // Generate sample with master gain for headroom
      samples[i] = this.masterGain * instantAmp * Math.sin(2 * Math.PI * instantFreq * t + phaseRadians);
    }
    
    return samples;
  }

  /**
   * Create a Web Audio API oscillator node (browser only)
   * @param {AudioContext} audioContext - Web Audio context
   * @returns {OscillatorNode} Configured oscillator node
   */
  createWebAudioNode(audioContext) {
    if (typeof window === 'undefined') {
      throw new Error('Web Audio API is only available in browser environments');
    }
    
    const oscillator = audioContext.createOscillator();
    const gainNode = audioContext.createGain();
    
    oscillator.type = 'sine';
    oscillator.frequency.value = this.frequency;
    gainNode.gain.value = this.amplitude * this.masterGain;

    oscillator.connect(gainNode);
    
    return { oscillator, gainNode };
  }

  /**
   * Get oscillator parameters as object
   * @returns {Object} Current parameters
   */
  getParams() {
    return {
      fskState: this.fskState,
      frequency: this.frequency,
      amplitude: this.amplitude,
      phase: this.phase,
      symbolRate: this.symbolRate,
      freqLFO: this.lfoFreq.getParams(),
      ampLFO: this.lfoAmp.getParams()
    };
  }
}

/**
 * Create a new Chimera oscillator instance
 * @returns {ChimeraOscillator} New oscillator instance
 */
export function createChimeraOscillator() {
  return new ChimeraOscillator();
}

export default ChimeraOscillator;
