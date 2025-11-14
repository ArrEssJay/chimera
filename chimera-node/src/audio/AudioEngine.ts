// @ts-nocheck - Importing from JS modules
import { ChimeraGOCS } from '@lib/chimera-gocs.js';
import { ChimeraOscillator } from '@lib/chimera-oscillator.js';
import type { AIDConfig, FrameInfo } from '../types/audio';

export class AudioEngine {
  private audioContext: AudioContext | null = null;
  private scriptNode: ScriptProcessorNode | null = null;
  private gocs: any;
  private oscillator: any;
  private isPlaying: boolean = false;
  private sampleRate: number = 48000;
  
  // Frame timing constants (per HCI spec)
  private readonly SYMBOL_RATE = 16; // Hz
  private readonly SYMBOLS_PER_FRAME = 128;
  private readonly FRAME_DURATION = 8.0; // seconds
  
  private currentFrameData: any = null;
  private frameStartSample: number = 0;
  private totalSamplesGenerated: number = 0;
  
  // Callbacks
  private onFrameUpdate: ((frameInfo: FrameInfo) => void) | null = null;
  private onTimeUpdate: ((remaining: number, frameNum: number) => void) | null = null;

  constructor() {
    this.gocs = new ChimeraGOCS();
    this.oscillator = new ChimeraOscillator();
  }

  async initialize(): Promise<void> {
    if (this.audioContext) return;
    
    this.audioContext = new (window.AudioContext || (window as any).webkitAudioContext)();
    this.sampleRate = this.audioContext.sampleRate;
    
    // Use ScriptProcessorNode for now (AudioWorklet can be added later)
    this.scriptNode = this.audioContext.createScriptProcessor(4096, 0, 1);
    this.scriptNode.onaudioprocess = this.processAudio.bind(this);
    
    console.log('AudioEngine initialized at', this.sampleRate, 'Hz');
  }

  setFrameUpdateCallback(callback: (frameInfo: FrameInfo) => void): void {
    this.onFrameUpdate = callback;
  }

  setTimeUpdateCallback(callback: (remaining: number, frameNum: number) => void): void {
    this.onTimeUpdate = callback;
  }

  private processAudio(event: AudioProcessingEvent): void {
    if (!this.isPlaying || !this.oscillator) return;
    
    const outputBuffer = event.outputBuffer;
    const outputData = outputBuffer.getChannelData(0);
    const bufferSize = outputData.length;
    
    // Check if we need a new frame
    const samplesIntoFrame = this.totalSamplesGenerated - this.frameStartSample;
    const frameDurationSamples = this.FRAME_DURATION * this.sampleRate;
    
    if (samplesIntoFrame >= frameDurationSamples || !this.currentFrameData) {
      this.generateNewFrame();
    }
    
    // Generate samples for this buffer
    try {
      const samples = this.oscillator.generateSamples(bufferSize, this.sampleRate);
      if (samples && samples.length === bufferSize) {
        outputData.set(samples);
      } else {
        // Fill with silence if generation fails
        outputData.fill(0);
      }
    } catch (error) {
      console.error('Error generating samples:', error);
      outputData.fill(0);
    }
    
    this.totalSamplesGenerated += bufferSize;
    
    // Update timing info
    if (this.onTimeUpdate) {
      const samplesIntoCurrentFrame = this.totalSamplesGenerated - this.frameStartSample;
      const timeRemaining = this.FRAME_DURATION - (samplesIntoCurrentFrame / this.sampleRate);
      const frameNumber = Math.floor(this.totalSamplesGenerated / frameDurationSamples);
      this.onTimeUpdate(Math.max(0, timeRemaining), frameNumber);
    }
  }

  private generateNewFrame(): void {
    // Generate next frame with current GOCS settings
    // For now, using default parameters - will be controlled by UI
    const frames = this.gocs.induceCalm(1.0, 1);
    this.currentFrameData = frames[0];
    this.oscillator.loadFrame(this.currentFrameData);
    this.frameStartSample = this.totalSamplesGenerated;
    
    // Extract frame info for UI
    if (this.onFrameUpdate && this.currentFrameData) {
      const frameInfo = this.extractFrameInfo(this.currentFrameData);
      this.onFrameUpdate(frameInfo);
    }
    
    console.log('New frame loaded:', this.currentFrameData);
  }

  private extractFrameInfo(frame: any): FrameInfo {
    // Extract relevant information from the frame for display
    return {
      commandType: frame.CommandType || 0,
      targetId: frame.TargetID || 0,
      gocsFunction: 'induceCalm', // Will be dynamic later
      symbols: [], // Will populate with actual symbol data
      lfoParams: {
        phase: { waveform: 'sine', frequency: 0.1, depth: 1.0 },
        freqMod: { waveform: 'sine', frequency: 0.1, depth: 1.0 },
        ampMod: { waveform: 'sine', frequency: 0.1, depth: 1.0 },
        fsk: { pattern: 'idle', rate: 16 }
      }
    };
  }

  executeGOCSFunction(functionName: string, intensity: number, duration: number): void {
    // This will trigger frame generation with the specified GOCS function
    console.log(`Executing GOCS function: ${functionName} with intensity ${intensity}`);
    // Implementation will vary based on GOCS API
  }

  updateAIDConfig(config: Partial<AIDConfig>): void {
    if (!this.oscillator) {
      console.warn('Oscillator not initialized');
      return;
    }

    try {
      if (config.enabled !== undefined && typeof this.oscillator.setAidEnabled === 'function') {
        this.oscillator.setAidEnabled(config.enabled);
      }
      
      if (config.modulationDepth !== undefined || 
          config.mixingCoefficient !== undefined ||
          config.phaseNoiseStd !== undefined ||
          config.bypassSimulation !== undefined) {
        // Update AID configuration in real-time
        const aidConfig = {
          modulationDepth: config.modulationDepth ?? 0.05,
          mixingCoefficient: config.mixingCoefficient ?? 0.7,
          phaseNoiseStd: config.phaseNoiseStd ?? 0.001,
          bypassSimulation: config.bypassSimulation ?? false
        };
        
        if (typeof this.oscillator.setAidConfig === 'function') {
          this.oscillator.setAidConfig(aidConfig);
        } else if (typeof this.oscillator.updateAidConfig === 'function') {
          this.oscillator.updateAidConfig(aidConfig);
        }
      }
    } catch (error) {
      console.error('Error updating AID config:', error);
    }
  }

  async start(): Promise<void> {
    if (!this.audioContext) {
      await this.initialize();
    }
    
    if (this.audioContext!.state === 'suspended') {
      await this.audioContext!.resume();
    }
    
    this.scriptNode!.connect(this.audioContext!.destination);
    this.isPlaying = true;
    this.generateNewFrame(); // Generate first frame
    
    console.log('Audio started');
  }

  stop(): void {
    if (this.scriptNode && this.audioContext) {
      this.scriptNode.disconnect();
    }
    this.isPlaying = false;
    console.log('Audio stopped');
  }

  advanceFrame(): void {
    // Immediately generate and load next frame
    this.generateNewFrame();
    console.log('Frame advanced manually');
  }

  getAnalyserNode(): AnalyserNode | null {
    if (!this.audioContext) return null;
    
    const analyser = this.audioContext.createAnalyser();
    analyser.fftSize = 2048;
    this.scriptNode?.connect(analyser);
    return analyser;
  }

  destroy(): void {
    this.stop();
    if (this.audioContext) {
      this.audioContext.close();
      this.audioContext = null;
    }
  }
}
