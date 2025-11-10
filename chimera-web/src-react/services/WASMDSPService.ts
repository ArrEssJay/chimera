/**
 * WASM DSP Service
 * 
 * Bridges Rust WASM streaming DSP engine with React components.
 * Manages Web Workers, audio I/O, and data streaming.
 */

import type { WASMStreamingDSP, WASMStreamOutput } from '../../pkg/chimera_web';
import type { SimulationConfig, ProtocolConfig, LDPCConfig, FSKState } from '../types';

export interface StreamConfig {
  simulation: SimulationConfig;
  protocol: ProtocolConfig;
  ldpc: LDPCConfig;
}

export interface DSPConfig {
  message: string;
  snrDb: number;
  linkLossDb: number;
  rngSeed?: number;
}

export interface PreChannelDiagnostics {
  frameCount: number;
  totalFrames: number;
  symbolCount: number;
  txConstellationI: Float32Array;
  txConstellationQ: Float32Array;
  txSpectrumMagnitude: Float32Array;
  spectrumFreqStartHz: number;
  spectrumFreqEndHz: number;
  carrierFreqHz: number;
  symbolRateHz: number;
  modulationType: string;
  fecRate: string;
  frameLayout: {
    syncBytes: number;
    dataBytes: number;
    parityBytes: number;
    totalBytes: number;
  };
}

export interface PostChannelDiagnostics {
  rxConstellationI: Float32Array;
  rxConstellationQ: Float32Array;
  rxSpectrumMagnitude: Float32Array;
  spectrumFreqStartHz: number;
  spectrumFreqEndHz: number;
  timingError: Float32Array;
  frequencyOffsetHz: number;
  phaseOffsetRad: number;
  evmPercent: number;
  snrEstimateDb: number;
  berInstantaneous: number;
  berAverage: number;
  syncStatus: boolean;
  lockStatus: string;
}

export interface FrameData {
  frameNumber: number;
  syncData: Uint8Array;
  payloadData: Uint8Array;
  parityData: Uint8Array;
  decodedText: string;
  symbolProgress: number; // Number of symbols transmitted so far in current frame
}

export interface StreamData {
  audio: Float32Array;
  preChannel: PreChannelDiagnostics;
  postChannel: PostChannelDiagnostics;
  decodedText: string;
  framesProcessed: number;
  symbolsDecoded: number;
  fecCorrections: number;
  currentFrameData: FrameData;
  fskState?: FSKState; // Add FSK state
  timestamp: number;
}

export type DataCallback = (data: StreamData) => void;
export type MetricsCallback = (metrics: any) => void;
export type LogsCallback = (logs: string[]) => void;

/**
 * WASM DSP Service for streaming audio processing
 */
export class WASMDSPService {
  private dsp: WASMStreamingDSP | null = null;
  private audioContext: AudioContext | null = null;
  private scriptNode: ScriptProcessorNode | null = null;
  private subscribers: Map<string, DataCallback> = new Map();
  private metricsSubscribers: Map<string, MetricsCallback> = new Map();
  private logsSubscribers: Map<string, LogsCallback> = new Map();
  private isRunning = false;
  private frameCount = 0;
  private lastFrameTime = 0;
  private accumulatedLogs: string[] = [];
  private isInitialized = false;

  constructor() {
    // Don't initialize in constructor - do it lazily
  }

  /**
   * Initialize WASM module and audio context
   */
  private async initialize(): Promise<void> {
    if (this.isInitialized) {
      return;
    }

    try {
      // Dynamically import WASM module
      const wasmModule = await import('../../pkg/chimera_web');
      
      // Initialize the WASM module (loads the .wasm file)
      await wasmModule.default();
      
      // Create DSP instance
      this.dsp = new wasmModule.WASMStreamingDSP();
      this.isInitialized = true;
      console.log('WASM Streaming DSP initialized');
    } catch (error) {
      console.error('Failed to initialize WASM DSP:', error);
      throw error;
    }
  }

  /**
   * Start audio processing
   */
  async start(): Promise<void> {
    // Ensure WASM is initialized first
    if (!this.isInitialized) {
      await this.initialize();
    }

    if (this.isRunning) {
      console.warn('Service already running');
      return;
    }

    // Create audio context if not exists
    if (!this.audioContext) {
      this.audioContext = new AudioContext({ sampleRate: 48000 });
    }

    // Resume audio context if suspended
    if (this.audioContext.state === 'suspended') {
      await this.audioContext.resume();
    }

    // Create script processor node for audio processing
    // Note: ScriptProcessorNode is deprecated but still widely supported
    // TODO: Migrate to AudioWorklet for better performance
    const bufferSize = 4096;
    this.scriptNode = this.audioContext.createScriptProcessor(
      bufferSize,
      1, // mono input
      1  // mono output
    );

    // Set up audio processing callback
    this.scriptNode.onaudioprocess = (event) => {
      this.processAudioChunk(event);
    };

    // Connect to audio destination
    this.scriptNode.connect(this.audioContext.destination);

    this.isRunning = true;
    this.lastFrameTime = performance.now();
    console.log('Audio processing started');
  }

  /**
   * Stop audio processing
   */
  stop(): void {
    if (!this.isRunning) {
      return;
    }

    // Disconnect and cleanup
    if (this.scriptNode) {
      this.scriptNode.disconnect();
      this.scriptNode = null;
    }

    this.isRunning = false;
    console.log('Audio processing stopped');
  }

  /**
   * Process audio chunk through WASM pipeline
   */
  private processAudioChunk(event: AudioProcessingEvent): void {
    if (!this.dsp) {
      console.warn('DSP not initialized');
      return;
    }

    const now = performance.now();

    try {
      // Get input audio
      const inputBuffer = event.inputBuffer;
      const inputData = inputBuffer.getChannelData(0);
      
      // Convert to Float32Array
      const inputArray = new Float32Array(inputData);

      // Process through WASM
      const output: WASMStreamOutput = this.dsp.process_audio(inputArray);

      // Write output audio
      const outputBuffer = event.outputBuffer;
      const outputData = outputBuffer.getChannelData(0);
      const audioOut = output.audio;
      
      for (let i = 0; i < Math.min(outputData.length, audioOut.length); i++) {
        outputData[i] = audioOut[i];
      }

      // Only notify subscribers if we have actual data (not rate-limited empty output)
      // Check if we have constellation data OR a valid frame number (not zero/undefined)
      const hasData = output.tx_constellation_i.length > 0 || 
                     output.rx_constellation_i.length > 0 ||
                     (output.frame_number && output.frame_number > 0);
      
      if (!hasData) {
        return; // Skip empty updates
      }

      // Notify subscribers with processed data
      const streamData: StreamData = {
        audio: output.audio,
        preChannel: {
          frameCount: output.frame_count,
          totalFrames: output.total_frames || 1,
          symbolCount: output.symbol_count,
          txConstellationI: output.tx_constellation_i,
          txConstellationQ: output.tx_constellation_q,
          txSpectrumMagnitude: output.tx_spectrum_magnitude,
          spectrumFreqStartHz: output.tx_spectrum_freq_start_hz || 0,
          spectrumFreqEndHz: output.tx_spectrum_freq_end_hz || 24000,
          carrierFreqHz: output.carrier_freq_hz,
          symbolRateHz: output.symbol_rate_hz,
          modulationType: output.modulation_type,
          fecRate: output.fec_rate,
          frameLayout: {
            syncBytes: output.sync_bytes || 0,
            dataBytes: output.data_bytes || 0,
            parityBytes: output.parity_bytes || 0,
            totalBytes: output.total_frame_bytes || 0,
          },
        },
        postChannel: {
          rxConstellationI: output.rx_constellation_i,
          rxConstellationQ: output.rx_constellation_q,
          rxSpectrumMagnitude: output.rx_spectrum_magnitude,
          spectrumFreqStartHz: output.rx_spectrum_freq_start_hz || 0,
          spectrumFreqEndHz: output.rx_spectrum_freq_end_hz || 24000,
          timingError: output.timing_error,
          frequencyOffsetHz: output.frequency_offset_hz,
          phaseOffsetRad: output.phase_offset_rad,
          evmPercent: output.evm_percent,
          snrEstimateDb: output.snr_estimate_db,
          berInstantaneous: output.ber_instantaneous,
          berAverage: output.ber_average,
          syncStatus: output.sync_status,
          lockStatus: output.lock_status,
        },
        decodedText: output.decoded_text,
        framesProcessed: output.frames_processed,
        symbolsDecoded: output.symbols_decoded,
        fecCorrections: output.fec_corrections,
        currentFrameData: {
          frameNumber: output.frame_number || 0,
          syncData: new Uint8Array(output.frame_sync_data || []),
          payloadData: new Uint8Array(output.frame_payload_data || []),
          parityData: new Uint8Array(output.frame_parity_data || []),
          decodedText: output.frame_decoded_text || '',
          symbolProgress: output.frame_symbol_progress || 0, // Use actual symbol progress from backend
        },
        fskState: output.fsk_state ? {
          current_frequency_hz: output.fsk_state.current_frequency_hz,
          frequency_deviation_hz: output.fsk_state.frequency_deviation_hz,
          current_bit: output.fsk_state.current_bit,
          bit_index: output.fsk_state.bit_index,
          bit_history: Array.from(output.fsk_state.bit_history),
          symbols_per_bit: output.fsk_state.symbols_per_bit,
          bit_rate_hz: output.fsk_state.bit_rate_hz,
        } : undefined,
        timestamp: now,
      };

      this.notifySubscribers(streamData);

      this.frameCount++;
      this.lastFrameTime = now;
    } catch (error) {
      console.error('Error processing audio chunk:', error);
    }
  }

  /**
   * Notify all subscribers with new data
   */
  private notifySubscribers(data: StreamData): void {
    this.subscribers.forEach((callback) => {
      try {
        callback(data);
      } catch (error) {
        console.error('Error in subscriber callback:', error);
      }
    });
  }

  /**
   * Notify metrics subscribers
   */
  // @ts-expect-error - Reserved for future metrics functionality
  private notifyMetricsSubscribers(metrics: any): void {
    this.metricsSubscribers.forEach((callback) => {
      try {
        callback(metrics);
      } catch (error) {
        console.error('Error in metrics subscriber callback:', error);
      }
    });
  }

  /**
   * Notify logs subscribers
   */
  private notifyLogsSubscribers(logs: string[]): void {
    this.logsSubscribers.forEach((callback) => {
      try {
        callback(logs);
      } catch (error) {
        console.error('Error in logs subscriber callback:', error);
      }
    });
  }

  /**
   * Add log entry
   */
  addLog(message: string): void {
    const timestamp = new Date().toLocaleTimeString();
    const logEntry = `[${timestamp}] ${message}`;
    this.accumulatedLogs.push(logEntry);
    this.notifyLogsSubscribers(this.accumulatedLogs);
  }

  /**
   * Subscribe to data stream
   */
  subscribe(id: string, callback: DataCallback): void {
    this.subscribers.set(id, callback);
  }

  /**
   * Unsubscribe from data stream
   */
  unsubscribe(id: string): void {
    this.subscribers.delete(id);
  }

  /**
   * Subscribe to metrics updates
   */
  subscribeToMetrics(id: string, callback: MetricsCallback): void {
    this.metricsSubscribers.set(id, callback);
  }

  /**
   * Unsubscribe from metrics updates
   */
  unsubscribeFromMetrics(id: string): void {
    this.metricsSubscribers.delete(id);
  }

  /**
   * Subscribe to logs updates
   */
  subscribeToLogs(id: string, callback: LogsCallback): void {
    this.logsSubscribers.set(id, callback);
  }

  /**
   * Unsubscribe from logs updates
   */
  unsubscribeFromLogs(id: string): void {
    this.logsSubscribers.delete(id);
  }

  /**
   * Clear accumulated logs
   */
  clearLogs(): void {
    this.accumulatedLogs = [];
    this.notifyLogsSubscribers(this.accumulatedLogs);
  }

  /**
   * Get current logs
   */
  getLogs(): string[] {
    return [...this.accumulatedLogs];
  }

  /**
   * Configure DSP pipeline
   */
  async configure(config: StreamConfig): Promise<void> {
    // Ensure WASM is initialized first
    if (!this.isInitialized) {
      await this.initialize();
    }

    if (!this.dsp) {
      throw new Error('DSP not initialized');
    }

    const configJson = JSON.stringify(config);
    this.dsp.configure(configJson);
  }

  /**
   * Get current configuration
   */
  getConfig(): StreamConfig {
    if (!this.dsp) {
      throw new Error('DSP not initialized');
    }

    const configJson = this.dsp.get_config();
    return JSON.parse(configJson);
  }

  /**
   * Get performance metrics
   */
  getMetrics(): { fps: number; frameCount: number } {
    const elapsed = performance.now() - this.lastFrameTime;
    const fps = elapsed > 0 ? 1000 / elapsed : 0;

    return {
      fps: Math.round(fps * 10) / 10,
      frameCount: this.frameCount,
    };
  }

  /**
   * Set target FPS for data updates (no longer used - updates on every audio chunk)
   */
  setTargetFPS(fps: number): void {
    // Deprecated - keeping for API compatibility
    console.log(`Target FPS set to ${fps} (deprecated - now updates continuously)`);
  }

  /**
   * Update channel parameters (SNR and link loss) during runtime without restarting
   */
  updateChannelParams(snr_db: number, link_loss_db: number): void {
    if (!this.dsp) {
      console.warn('DSP not initialized');
      return;
    }
    
    try {
      this.dsp.update_channel(snr_db, link_loss_db);
      console.log(`Updated channel params: SNR=${snr_db} dB, Link Loss=${link_loss_db} dB`);
    } catch (error) {
      console.error('Failed to update channel params:', error);
    }
  }

  /**
   * Set THz modulation mode (idle or active)
   */
  setModulationMode(active: boolean): void {
    if (!this.dsp) {
      console.warn('DSP not initialized');
      return;
    }
    
    try {
      this.dsp.set_modulation_mode(active);
      console.log(`THz modulation mode: ${active ? 'Active (70-80%)' : 'Idle (<5%)'}`);
    } catch (error) {
      console.error('Failed to set modulation mode:', error);
    }
  }

  /**
   * Set custom modulation depth (0.0 to 1.0)
   */
  setModulationDepth(depth: number): void {
    if (!this.dsp) {
      console.warn('DSP not initialized');
      return;
    }
    
    try {
      this.dsp.set_modulation_depth(depth);
      console.log(`THz modulation depth: ${(depth * 100).toFixed(1)}%`);
    } catch (error) {
      console.error('Failed to set modulation depth:', error);
    }
  }

  /**
   * Set mixing coefficient for third-order intermodulation
   */
  setMixingCoefficient(coefficient: number): void {
    if (!this.dsp) {
      console.warn('DSP not initialized');
      return;
    }
    
    try {
      this.dsp.set_mixing_coefficient(coefficient);
      console.log(`THz mixing coefficient: ${coefficient.toFixed(2)}`);
    } catch (error) {
      console.error('Failed to set mixing coefficient:', error);
    }
  }

  /**
   * Generate idle carrier audio for calibration
   */
  generateIdleCarrier(durationMs: number = 100): Float32Array | null {
    if (!this.dsp) {
      console.warn('DSP not initialized');
      return null;
    }
    
    try {
      const sampleRate = 48000;
      const numSamples = Math.floor((durationMs / 1000) * sampleRate);
      const samples = this.dsp.generate_idle_carrier(numSamples);
      console.log(`Generated ${numSamples} samples (${durationMs}ms) of idle carrier`);
      return samples;
    } catch (error) {
      console.error('Failed to generate idle carrier:', error);
      return null;
    }
  }

  /**
   * Cleanup resources
   */
  dispose(): void {
    this.stop();
    
    if (this.audioContext) {
      this.audioContext.close();
      this.audioContext = null;
    }

    this.subscribers.clear();
    this.metricsSubscribers.clear();
    this.logsSubscribers.clear();
    this.accumulatedLogs = [];
    this.dsp = null;
  }
}

// Singleton instance
let serviceInstance: WASMDSPService | null = null;

/**
 * Get or create singleton service instance
 */
export function getWASMDSPService(): WASMDSPService {
  if (!serviceInstance) {
    serviceInstance = new WASMDSPService();
  }
  return serviceInstance;
}
