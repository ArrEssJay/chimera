export interface GOCSState {
  currentFunction: string;
  intensity: number;
  duration: number;
  frameNumber: number;
  frameTimeRemaining: number;
  isPlaying: boolean;
}

export interface AIDConfig {
  enabled: boolean;
  modulationDepth: number;
  mixingCoefficient: number;
  phaseNoiseStd: number;
  pumpPower: number;
  dataPower: number;
  bypassSimulation: boolean;
}

export interface FrameInfo {
  commandType: number;
  targetId: number;
  gocsFunction: string;
  symbols: SymbolInfo[];
  lfoParams: LFOParams;
}

export interface SymbolInfo {
  phaseState: number;  // 0-3 for QPSK
  frequency: number;
  amplitude: number;
  fskState: number;
}

export interface LFOParams {
  phase: { waveform: string; frequency: number; depth: number };
  freqMod: { waveform: string; frequency: number; depth: number };
  ampMod: { waveform: string; frequency: number; depth: number };
  fsk: { pattern: string; rate: number };
}

export interface AudioState {
  gocs: GOCSState;
  aid: AIDConfig;
  currentFrame: FrameInfo | null;
}
