/**
 * ChimeraFrame - Holds pre-rendered control data for one frame (128 symbols, 8 seconds)
 * 
 * Implements frame structure v3.1 spec:
 * - Sync Sequence (16 symbols / 32 bits)
 * - Target ID (16 symbols / 32 bits) 
 * - Command Type (16 symbols / 32 bits)
 * - Data Payload (64 symbols / 128 bits)
 * - ECC (16 symbols / 32 bits)
 * 
 * Each symbol represents 62.5ms (1/16th of a second) at the 16Hz symbol rate.
 * The frame contains:
 * - FSK states (0 or 1) for each symbol
 * - Frequency modulation values (normalized -1.0 to 1.0)
 * - Amplitude modulation values (normalized 0.0 to 1.0)
 * - Phase rotation values (quantized 0-3 for QPSK)
 */

export class ChimeraFrame {
  constructor() {
    this.symbolRate = 16; // Hz
    this.symbolsPerFrame = 128;
    this.frameDuration = 8.0; // seconds
    this.symbolDuration = 1.0 / this.symbolRate; // 0.0625 seconds = 62.5ms
    
    // Frame structure field boundaries
    this.fields = {
      sync: { start: 0, length: 16 },       // Sync Sequence
      targetId: { start: 16, length: 16 },  // Target ID
      commandType: { start: 32, length: 16 }, // Command Type
      dataPayload: { start: 48, length: 64 }, // Data Payload
      ecc: { start: 112, length: 16 }       // ECC
    };
    
    // Control data arrays - one value per symbol
    this.fskStates = new Array(this.symbolsPerFrame).fill(0);
    this.freqModulation = new Array(this.symbolsPerFrame).fill(0.0);
    this.ampModulation = new Array(this.symbolsPerFrame).fill(1.0);
    this.phaseRotation = new Array(this.symbolsPerFrame).fill(0); // QPSK: 0, 1, 2, 3
    
    // Frame structure fields (simulation values)
    this.targetId = {
      baselineBrainwave: 0x08,      // 8 Hz (Alpha)
      hemisphereBias: 0x80,         // Balanced
      corticalRegion: 0x04,         // Prefrontal
      resonanceKey: 0x00            // Simulated (individual-specific in real system)
    };
    
    this.commandType = {
      vibrationalMode: 0x01,        // Breathing mode
      intensityPattern: 0x10,       // Smooth sine
      duration: 0x01,               // Single frame
      sequencing: 0x01              // Frame 0 of 1
    };
    
    // Initialize sync sequence (fixed pattern for hardware sync)
    this.initializeSyncSequence();
  }
  
  /**
   * Initialize sync sequence with fixed pattern
   * Using alternating pattern for simulation (real system would use specific sync pattern)
   */
  initializeSyncSequence() {
    const { sync } = this.fields;
    for (let i = 0; i < sync.length; i++) {
      // Alternating 0,1,2,3 pattern for sync
      this.phaseRotation[sync.start + i] = i % 4;
    }
  }
  
  /**
   * Set FSK state for a specific symbol or range of symbols
   * @param {number|Array} symbolIndex - Symbol index or array of indices
   * @param {number} state - FSK state (0 or 1)
   */
  setFSKState(symbolIndex, state) {
    if (Array.isArray(symbolIndex)) {
      for (const idx of symbolIndex) {
        this.fskStates[idx] = state;
      }
    } else {
      this.fskStates[symbolIndex] = state;
    }
  }
  
  /**
   * Set FSK pattern for entire frame
   * @param {string} pattern - 'constant', 'alternating', 'random'
   * @param {number} value - Base value for constant (0 or 1)
   * @param {number} rate - Rate for alternating pattern (Hz)
   */
  setFSKPattern(pattern, value = 0, rate = 0.5) {
    switch (pattern) {
      case 'constant':
        this.fskStates.fill(value);
        break;
      case 'alternating':
        // Alternate at specified rate (e.g., 0.5 Hz = every 2 seconds = every 32 symbols)
        const symbolsPerCycle = Math.floor(this.symbolRate / rate);
        for (let i = 0; i < this.symbolsPerFrame; i++) {
          this.fskStates[i] = Math.floor(i / symbolsPerCycle) % 2;
        }
        break;
      case 'random':
        for (let i = 0; i < this.symbolsPerFrame; i++) {
          this.fskStates[i] = Math.random() < 0.5 ? 0 : 1;
        }
        break;
    }
  }
  
  /**
   * Set frequency modulation for a specific symbol
   * @param {number} symbolIndex - Symbol index
   * @param {number} value - Modulation value (-1.0 to 1.0)
   */
  setFreqModulation(symbolIndex, value) {
    this.freqModulation[symbolIndex] = Math.max(-1.0, Math.min(1.0, value));
  }
  
  /**
   * Set amplitude modulation for a specific symbol
   * @param {number} symbolIndex - Symbol index
   * @param {number} value - Modulation value (0.0 to 1.0)
   */
  setAmpModulation(symbolIndex, value) {
    this.ampModulation[symbolIndex] = Math.max(0.0, Math.min(1.0, value));
  }
  
  /**
   * Set phase rotation (QPSK) for a specific symbol
   * @param {number} symbolIndex - Symbol index
   * @param {number} value - Phase state (0, 1, 2, or 3)
   */
  setPhaseRotation(symbolIndex, value) {
    this.phaseRotation[symbolIndex] = Math.floor(value) % 4;
  }
  
  /**
   * Get control values for a specific symbol
   * @param {number} symbolIndex - Symbol index (0-127)
   * @returns {Object} Control values for this symbol
   */
  getSymbol(symbolIndex) {
    const idx = Math.floor(symbolIndex) % this.symbolsPerFrame;
    return {
      fskState: this.fskStates[idx],
      freqMod: this.freqModulation[idx],
      ampMod: this.ampModulation[idx],
      phase: this.phaseRotation[idx]
    };
  }
  
  /**
   * Clone this frame
   * @returns {ChimeraFrame} A deep copy of this frame
   */
  clone() {
    const frame = new ChimeraFrame();
    frame.fskStates = [...this.fskStates];
    frame.freqModulation = [...this.freqModulation];
    frame.ampModulation = [...this.ampModulation];
    frame.phaseRotation = [...this.phaseRotation];
    return frame;
  }
  
  /**
   * Set Target ID fields
   * @param {Object} targetId - Target ID configuration
   */
  setTargetId(targetId) {
    if (targetId.baselineBrainwave !== undefined) this.targetId.baselineBrainwave = targetId.baselineBrainwave;
    if (targetId.hemisphereBias !== undefined) this.targetId.hemisphereBias = targetId.hemisphereBias;
    if (targetId.corticalRegion !== undefined) this.targetId.corticalRegion = targetId.corticalRegion;
    if (targetId.resonanceKey !== undefined) this.targetId.resonanceKey = targetId.resonanceKey;
  }
  
  /**
   * Set Command Type fields
   * @param {Object} commandType - Command Type configuration
   */
  setCommandType(commandType) {
    if (commandType.vibrationalMode !== undefined) this.commandType.vibrationalMode = commandType.vibrationalMode;
    if (commandType.intensityPattern !== undefined) this.commandType.intensityPattern = commandType.intensityPattern;
    if (commandType.duration !== undefined) this.commandType.duration = commandType.duration;
    if (commandType.sequencing !== undefined) this.commandType.sequencing = commandType.sequencing;
  }
  
  /**
   * Get frame structure fields
   * @returns {Object} Frame structure data
   */
  getFrameStructure() {
    return {
      targetId: { ...this.targetId },
      commandType: { ...this.commandType }
    };
  }
  
  /**
   * Get statistics about this frame
   * @returns {Object} Frame statistics
   */
  getStats() {
    const avgFreqMod = this.freqModulation.reduce((a, b) => a + b, 0) / this.symbolsPerFrame;
    const avgAmpMod = this.ampModulation.reduce((a, b) => a + b, 0) / this.symbolsPerFrame;
    const fskOnes = this.fskStates.filter(s => s === 1).length;
    
    return {
      symbols: this.symbolsPerFrame,
      duration: this.frameDuration,
      symbolRate: this.symbolRate,
      fskRatio: fskOnes / this.symbolsPerFrame,
      avgFreqMod: avgFreqMod.toFixed(3),
      avgAmpMod: avgAmpMod.toFixed(3),
      targetId: this.targetId,
      commandType: this.commandType
    };
  }
}
