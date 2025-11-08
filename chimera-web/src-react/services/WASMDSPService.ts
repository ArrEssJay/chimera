/**
 * WASM DSP Service
 * 
 * Bridges Rust WASM streaming DSP engine with React components.
 * Manages Web Workers, audio I/O, and data streaming.
 */

import type { WASMStreamingDSP, WASMStreamOutput } from '../../pkg/chimera_web';
import type { SimulationConfig, ProtocolConfig, LDPCConfig } from '../types';

export interface StreamConfig {
  simulation: SimulationConfig;
  protocol: ProtocolConfig;
  ldpc: LDPCConfig;
}

export interface StreamData {
  audio: Float32Array;
  constellationI: Float32Array;
  constellationQ: Float32Array;
  fftMagnitude: Float32Array;
  ber: number;
  decodedText: string;
  timingError: number;
  meanEvm: number;
  peakEvm: number;
  syncFound: boolean;
  symbolCount: number;
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
  private targetFPS = 60;
  private frameInterval = 1000 / 60; // 16.67ms
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

    // Rate limit to target FPS
    const now = performance.now();
    const elapsed = now - this.lastFrameTime;
    
    if (elapsed < this.frameInterval) {
      return;
    }

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

      // Notify subscribers with processed data
      const streamData: StreamData = {
        audio: output.audio,
        constellationI: output.constellation_i,
        constellationQ: output.constellation_q,
        fftMagnitude: output.fft_magnitude,
        ber: output.ber,
        decodedText: output.decoded_text,
        timingError: output.timing_error,
        meanEvm: output.mean_evm,
        peakEvm: output.peak_evm,
        syncFound: output.sync_found,
        symbolCount: output.symbol_count,
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
   * Set target FPS for data updates
   */
  setTargetFPS(fps: number): void {
    this.targetFPS = Math.max(1, Math.min(fps, 120));
    this.frameInterval = 1000 / this.targetFPS;
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
