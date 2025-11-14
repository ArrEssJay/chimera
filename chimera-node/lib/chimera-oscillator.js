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
 * - Optional AID (Auditory Intermodulation Distortion) simulation
 * 
 * Supports both Web Audio API (browser) and file output (Node.js)
 */

import { ThzCarrierProcessor, ThzCarrierConfig, AudioMixingConfig } from './chimera-aid.js';

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
 * Biquad bandpass filter
 * Implements a second-order IIR filter for bandpass filtering
 */
class BiquadBandpass {
  constructor(centerFreq, bandwidth, sampleRate) {
    this.centerFreq = centerFreq;
    this.bandwidth = bandwidth;
    this.sampleRate = sampleRate;
    
    // Filter state variables
    this.x1 = 0; // x[n-1]
    this.x2 = 0; // x[n-2]
    this.y1 = 0; // y[n-1]
    this.y2 = 0; // y[n-2]
    
    // Calculate filter coefficients
    this.updateCoefficients();
  }
  
  /**
   * Update filter coefficients based on center frequency and bandwidth
   */
  updateCoefficients() {
    const omega = 2 * Math.PI * this.centerFreq / this.sampleRate;
    const bw = 2 * Math.PI * this.bandwidth / this.sampleRate;
    const alpha = Math.sin(bw) / 2;
    
    // Biquad coefficients for bandpass filter
    const b0 = alpha;
    const b1 = 0;
    const b2 = -alpha;
    const a0 = 1 + alpha;
    const a1 = -2 * Math.cos(omega);
    const a2 = 1 - alpha;
    
    // Normalize by a0
    this.b0 = b0 / a0;
    this.b1 = b1 / a0;
    this.b2 = b2 / a0;
    this.a1 = a1 / a0;
    this.a2 = a2 / a0;
  }
  
  /**
   * Process a single sample through the filter
   * @param {number} input - Input sample
   * @returns {number} Filtered output sample
   */
  process(input) {
    const output = this.b0 * input + this.b1 * this.x1 + this.b2 * this.x2
                   - this.a1 * this.y1 - this.a2 * this.y2;
    
    // Update state variables
    this.x2 = this.x1;
    this.x1 = input;
    this.y2 = this.y1;
    this.y1 = output;
    
    return output;
  }
  
  /**
   * Process an array of samples
   * @param {Float32Array} samples - Input samples
   * @returns {Float32Array} Filtered output samples
   */
  processBlock(samples) {
    const output = new Float32Array(samples.length);
    for (let i = 0; i < samples.length; i++) {
      output[i] = this.process(samples[i]);
    }
    return output;
  }
  
  /**
   * Reset filter state
   */
  reset() {
    this.x1 = 0;
    this.x2 = 0;
    this.y1 = 0;
    this.y2 = 0;
  }
  
  /**
   * Update center frequency
   * @param {number} freq - New center frequency in Hz
   */
  setCenterFrequency(freq) {
    this.centerFreq = freq;
    this.updateCoefficients();
    this.reset();
  }
  
  /**
   * Update bandwidth
   * @param {number} bw - New bandwidth in Hz
   */
  setBandwidth(bw) {
    this.bandwidth = bw;
    this.updateCoefficients();
    this.reset();
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
    
    // Bandpass filter (20 Hz bandwidth centered at 12 kHz)
    this.bandpassFilter = null;
    this.filterEnabled = true;
    
    // AID (Auditory Intermodulation Distortion) simulation
    this.aidProcessor = null;
    this.aidEnabled = false;
    this.aidConfig = new ThzCarrierConfig();
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
   * Enable or disable bandpass filtering
   * @param {boolean} enabled - Whether to enable the filter
   */
  setFilterEnabled(enabled) {
    this.filterEnabled = enabled;
  }

  /**
   * Check if bandpass filter is enabled
   * @returns {boolean}
   */
  isFilterEnabled() {
    return this.filterEnabled;
  }

  /**
   * Get filter parameters
   * @returns {Object|null} Filter parameters or null if not initialized
   */
  getFilterParams() {
    if (!this.bandpassFilter) {
      return null;
    }
    return {
      centerFreq: this.bandpassFilter.centerFreq,
      bandwidth: this.bandpassFilter.bandwidth,
      sampleRate: this.bandpassFilter.sampleRate,
      enabled: this.filterEnabled
    };
  }

  /**
   * Enable or disable AID simulation
   * @param {boolean} enabled - Whether to enable AID simulation
   */
  setAidEnabled(enabled) {
    this.aidEnabled = enabled;
  }

  /**
   * Check if AID simulation is enabled
   * @returns {boolean}
   */
  isAidEnabled() {
    return this.aidEnabled;
  }

  /**
   * Configure AID simulation
   * @param {ThzCarrierConfig} config - AID configuration
   */
  setAidConfig(config) {
    this.aidConfig = config;
    // Reinitialize processor if already created
    if (this.aidProcessor) {
      this.aidProcessor = null;
    }
  }

  /**
   * Get AID processor (creates if needed)
   * @param {number} sampleRate - Sample rate
   * @returns {ThzCarrierProcessor}
   */
  getAidProcessor(sampleRate) {
    if (!this.aidProcessor) {
      this.aidProcessor = new ThzCarrierProcessor(this.aidConfig, sampleRate);
    }
    return this.aidProcessor;
  }

  /**
   * Set external audio for AID secondary intermodulation
   * @param {Float32Array} audio - External audio buffer
   * @param {AudioMixingConfig} mixingConfig - Mixing configuration
   */
  setAidExternalAudio(audio, mixingConfig) {
    if (!this.aidProcessor) {
      throw new Error('AID processor not initialized. Generate samples first.');
    }
    this.aidProcessor.setExternalAudio(audio, mixingConfig);
  }

  /**
   * Clear external audio from AID processor
   */
  clearAidExternalAudio() {
    if (this.aidProcessor) {
      this.aidProcessor.clearExternalAudio();
    }
  }

  /**
   * Get AID configuration and status
   * @returns {Object|null}
   */
  getAidParams() {
    if (!this.aidEnabled) {
      return { enabled: false };
    }
    return {
      enabled: true,
      config: this.aidConfig,
      hasExternalAudio: this.aidProcessor && this.aidProcessor.externalAudio !== null
    };
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
    // Initialize bandpass filter per PAL spec: 20 Hz bandwidth centered at 12 kHz carrier
    if (!this.bandpassFilter) {
      this.bandpassFilter = new BiquadBandpass(12000, 20, sampleRate);
    } else if (this.bandpassFilter.sampleRate !== sampleRate) {
      // Update filter if sample rate changed
      this.bandpassFilter = new BiquadBandpass(12000, 20, sampleRate);
    }
    
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
    
    // Apply bandpass filter if enabled
    if (this.filterEnabled) {
      const filtered = this.bandpassFilter.processBlock(samples);
      
      // Apply AID simulation if enabled
      if (this.aidEnabled) {
        const aidProcessor = this.getAidProcessor(sampleRate);
        return aidProcessor.process(filtered);
      }
      
      return filtered;
    }
    
    // Apply AID simulation if enabled (without filter)
    if (this.aidEnabled) {
      const aidProcessor = this.getAidProcessor(sampleRate);
      return aidProcessor.process(samples);
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
      ampLFO: this.lfoAmp.getParams(),
      filter: this.getFilterParams(),
      aid: this.getAidParams()
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
