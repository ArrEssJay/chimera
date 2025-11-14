/**
 * Chimera AID (Auditory Intermodulation Distortion) Simulation
 * 
 * Implements the complete biophysical signal chain from THz carrier mixing
 * to biological demodulation in neural tissue, per the PAL specification.
 * 
 * Physical Model:
 * 1. Heterodyne mixing of two THz carriers:
 *    - Pump beam (F1): 1.998 THz unmodulated
 *    - Data carrier (F2): 1.875 THz AM-modulated with 12 kHz audio
 *    - Difference frequency: 123 GHz (F1 - F2)
 * 
 * 2. Biological demodulation via third-order intermodulation (E1² × E2)
 *    in cortical tissue extracts the 12 kHz audio envelope
 * 
 * 3. Optional secondary intermodulation with external audio
 *    for realistic perceptual simulation
 */

/**
 * Complex number representation for envelope modeling
 */
class Complex {
  constructor(re = 0, im = 0) {
    this.re = re;
    this.im = im;
  }

  /**
   * Calculate magnitude (norm) of complex number
   * @returns {number}
   */
  norm() {
    return Math.sqrt(this.re * this.re + this.im * this.im);
  }

  /**
   * Create complex number from polar coordinates
   * @param {number} r - Magnitude
   * @param {number} theta - Phase angle in radians
   * @returns {Complex}
   */
  static fromPolar(r, theta) {
    return new Complex(
      r * Math.cos(theta),
      r * Math.sin(theta)
    );
  }
}

/**
 * THz Carrier Configuration
 */
export class ThzCarrierConfig {
  constructor() {
    this.pumpFrequency = 1.998e12;     // Pump beam (F1) in Hz
    this.dataFrequency = 1.875e12;     // Data carrier (F2) in Hz
    this.pumpPower = 1.0;              // Relative power
    this.dataPower = 0.3;              // Relative power
    this.modulationDepth = 0.05;       // AM depth (5% idle state)
    this.mixingCoefficient = 0.7;      // Biological response efficiency
    this.phaseNoiseStd = 0.001;        // Phase noise std dev
    this.bypassSimulation = false;     // Bypass for validation
  }

  /**
   * Create configuration with custom parameters
   * @param {Object} params - Configuration parameters
   * @returns {ThzCarrierConfig}
   */
  static create(params = {}) {
    const config = new ThzCarrierConfig();
    Object.assign(config, params);
    return config;
  }
}

/**
 * Audio Mixing Configuration for secondary intermodulation
 */
export class AudioMixingConfig {
  constructor() {
    this.aidSignalGain = 1.0;
    this.externalAudioGain = 1.0;
    this.enableSecondOrder = true;
    this.enableThirdOrder = true;
    this.secondOrderCoefficient = 0.15;
    this.thirdOrderCoefficient = 0.08;
    this.corticalCoefficient = 0.25;
  }

  static create(params = {}) {
    const config = new AudioMixingConfig();
    Object.assign(config, params);
    return config;
  }
}

/**
 * THz Carrier Processor - AID Simulation
 */
export class ThzCarrierProcessor {
  constructor(config = new ThzCarrierConfig(), sampleRate = 48000) {
    this.config = config;
    this.sampleRate = sampleRate;
    this.externalAudio = null;
    this.mixingConfig = null;
  }

  /**
   * Load external audio for secondary intermodulation
   * @param {Float32Array} audio - External audio buffer
   * @param {AudioMixingConfig} mixingConfig - Mixing configuration
   */
  setExternalAudio(audio, mixingConfig = new AudioMixingConfig()) {
    this.externalAudio = audio;
    this.mixingConfig = mixingConfig;
  }

  /**
   * Clear external audio
   */
  clearExternalAudio() {
    this.externalAudio = null;
    this.mixingConfig = null;
  }

  /**
   * Set modulation depth (0.0 to 1.0)
   * @param {number} depth - Modulation depth
   */
  setModulationDepth(depth) {
    this.config.modulationDepth = Math.max(0, Math.min(1, depth));
  }

  /**
   * Set mixing coefficient (biological response efficiency)
   * @param {number} coeff - Mixing coefficient (0.0 to 1.0)
   */
  setMixingCoefficient(coeff) {
    this.config.mixingCoefficient = Math.max(0, Math.min(1, coeff));
  }

  /**
   * Model 1: Heterodyne Mixing Envelope
   * 
   * Simulates AM modulation of data carrier and heterodyne mixing with pump beam.
   * Returns complex envelope of the 123 GHz difference frequency.
   * 
   * @param {Float32Array} audioSignal - Input audio signal
   * @returns {Complex[]} Complex envelope of mixed signal
   */
  modulateDataCarrier(audioSignal) {
    // Bypass mode: return input as complex for validation
    if (this.config.bypassSimulation) {
      return Array.from(audioSignal, sample => new Complex(sample, 0));
    }

    const output = [];
    const diffFreq = Math.abs(this.config.pumpFrequency - this.config.dataFrequency);
    
    for (let i = 0; i < audioSignal.length; i++) {
      const audioSample = audioSignal[i];
      
      // Laser phase noise (random walk)
      const phaseNoise = (Math.random() - 0.5) * this.config.phaseNoiseStd;
      
      // Pump beam envelope (unmodulated, constant power)
      const pumpEnvelope = this.config.pumpPower;
      
      // Data carrier with AM modulation: (1 + m·s(t))
      const modulation = 1.0 + this.config.modulationDepth * audioSample;
      const dataEnvelope = this.config.dataPower * modulation;
      
      // Heterodyne mixing produces envelope at difference frequency
      // Physical: E1·cos(2πF1t) × E2·(1+m·s(t))·cos(2πF2t)
      // Result: 0.5·E1·E2·(1+m·s(t))·[cos(2π(F1-F2)t) + cos(2π(F1+F2)t)]
      const mixedAmplitude = pumpEnvelope * dataEnvelope;
      
      // Phase noise affects amplitude (power fluctuation) and phase
      const amplitudeNoise = 1.0 + phaseNoise * 0.01;
      
      // Represent as complex envelope
      const combined = Complex.fromPolar(
        mixedAmplitude * amplitudeNoise,
        phaseNoise
      );
      
      output.push(combined);
    }
    
    return output;
  }

  /**
   * Model 2 & 3: Biological Demodulation
   * 
   * Simulates third-order intermodulation (E1² × E2) in neural tissue,
   * extracting the audio envelope from the 123 GHz intermediate frequency.
   * Optionally applies secondary intermodulation with external audio.
   * 
   * @param {Complex[]} signal - Complex envelope from modulateDataCarrier
   * @returns {Float32Array} Demodulated audio signal
   */
  nonlinearMixing(signal) {
    // Bypass mode: extract real part only
    if (this.config.bypassSimulation) {
      return new Float32Array(signal.map(c => c.re));
    }

    const output = [];
    
    for (let i = 0; i < signal.length; i++) {
      const sample = signal[i];
      const magnitude = sample.norm();
      
      // Third-order intermodulation: |E|² × Re(E)
      // This is the dominant term in E1² × E2 mixing
      // |E|² represents power (envelope squared)
      // Re(E) provides phase information
      const thirdOrderProduct = magnitude * magnitude * sample.re;
      
      // Apply mixing coefficient (biological response efficiency)
      const demodulated = this.config.mixingCoefficient * thirdOrderProduct;
      
      output.push(demodulated);
    }
    
    // DC blocking (biological AC coupling)
    const mean = output.reduce((a, b) => a + b, 0) / output.length;
    for (let i = 0; i < output.length; i++) {
      output[i] -= mean;
    }
    
    // Normalize to maintain signal amplitude (~0.5 peak for headroom)
    let maxAbs = 0;
    for (let i = 0; i < output.length; i++) {
      const abs = Math.abs(output[i]);
      if (abs > maxAbs) maxAbs = abs;
    }
    
    if (maxAbs > 1e-6) {
      const scale = 0.5 / maxAbs;
      for (let i = 0; i < output.length; i++) {
        output[i] *= scale;
      }
    }
    
    const result = new Float32Array(output);
    
    // Apply secondary intermodulation if external audio present
    if (this.externalAudio && this.mixingConfig) {
      this.applyBiologicalIntermodulation(result, this.externalAudio, this.mixingConfig);
    }
    
    return result;
  }

  /**
   * Apply biological intermodulation between AID signal and external audio
   * 
   * Models secondary mixing at:
   * 1. Cochlear nerve junction (second-order products)
   * 2. Auditory cortex (third-order products)
   * 3. Cortical integration (perceptual blending)
   * 
   * 
   * @param {Float32Array} aidSignal - AID signal (modified in place)
   * @param {Float32Array} externalAudio - External audio
   * @param {AudioMixingConfig} config - Mixing configuration
   */
  applyBiologicalIntermodulation(aidSignal, externalAudio, config) {
    const minLen = Math.min(aidSignal.length, externalAudio.length);
    
    for (let i = 0; i < minLen; i++) {
      const aid = aidSignal[i] * config.aidSignalGain;
      const ext = externalAudio[i] * config.externalAudioGain;
      
      // Second-order intermodulation (cochlear nerve junction)
      // Produces sum and difference frequencies: f_aid ± f_ext
      const secondOrder = config.enableSecondOrder
        ? config.secondOrderCoefficient * aid * ext
        : 0.0;
      
      // Third-order intermodulation (cortical processing)
      // Produces: 2*f_aid ± f_ext, f_aid ± 2*f_ext
      const thirdOrder = config.enableThirdOrder
        ? config.thirdOrderCoefficient * (aid * aid * ext + aid * ext * ext)
        : 0.0;
      
      // Cortical integration (perceptual blending)
      const corticalBlend = config.corticalCoefficient * (aid + ext);
      
      // Mix signals together
      aidSignal[i] = ext + aid + secondOrder + thirdOrder + corticalBlend;
      
    }
  }

  /**
   * Process audio through complete AID simulation pipeline
   * @param {Float32Array} audioSignal - Input audio
   * @returns {Float32Array} Simulated perceived audio
   */
  process(audioSignal) {
    const modulated = this.modulateDataCarrier(audioSignal);
    return this.nonlinearMixing(modulated);
  }

  /**
   * Get current configuration
   * @returns {ThzCarrierConfig}
   */
  getConfig() {
    return this.config;
  }
}

export default ThzCarrierProcessor;
