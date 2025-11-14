import React, { createContext, useContext, useState, useEffect, useCallback, ReactNode } from 'react';
import { AudioEngine } from './AudioEngine';
import type { AudioState, GOCSState, AIDConfig, FrameInfo } from '../types/audio';

interface AudioContextType {
  audioState: AudioState;
  isPlaying: boolean;
  play: () => Promise<void>;
  stop: () => void;
  advanceFrame: () => void;
  setGOCSFunction: (functionName: string, intensity: number, duration: number) => void;
  updateAIDConfig: (config: Partial<AIDConfig>) => void;
  audioEngine: AudioEngine | null;
}

const AudioContext = createContext<AudioContextType | null>(null);

export const useAudio = () => {
  const context = useContext(AudioContext);
  if (!context) {
    throw new Error('useAudio must be used within AudioProvider');
  }
  return context;
};

interface AudioProviderProps {
  children: ReactNode;
}

export const AudioProvider: React.FC<AudioProviderProps> = ({ children }) => {
  const [audioEngine] = useState(() => new AudioEngine());
  const [isPlaying, setIsPlaying] = useState(false);
  
  const [audioState, setAudioState] = useState<AudioState>({
    gocs: {
      currentFunction: 'induceCalm',
      intensity: 1.0,
      duration: 1,
      frameNumber: 0,
      frameTimeRemaining: 8.0,
      isPlaying: false,
    },
    aid: {
      enabled: false,
      modulationDepth: 0.05,
      mixingCoefficient: 0.7,
      phaseNoiseStd: 0.001,
      pumpPower: 1.0,
      dataPower: 0.3,
      bypassSimulation: false,
    },
    currentFrame: null,
  });

  useEffect(() => {
    // Set up callbacks for frame and time updates
    audioEngine.setFrameUpdateCallback((frameInfo: FrameInfo) => {
      setAudioState(prev => ({
        ...prev,
        currentFrame: frameInfo,
      }));
    });

    audioEngine.setTimeUpdateCallback((remaining: number, frameNum: number) => {
      setAudioState(prev => ({
        ...prev,
        gocs: {
          ...prev.gocs,
          frameTimeRemaining: remaining,
          frameNumber: frameNum,
        },
      }));
    });

    return () => {
      audioEngine.destroy();
    };
  }, [audioEngine]);

  const play = useCallback(async () => {
    await audioEngine.start();
    setIsPlaying(true);
    setAudioState(prev => ({
      ...prev,
      gocs: { ...prev.gocs, isPlaying: true },
    }));
  }, [audioEngine]);

  const stop = useCallback(() => {
    audioEngine.stop();
    setIsPlaying(false);
    setAudioState(prev => ({
      ...prev,
      gocs: { ...prev.gocs, isPlaying: false },
    }));
  }, [audioEngine]);

  const advanceFrame = useCallback(() => {
    audioEngine.advanceFrame();
  }, [audioEngine]);

  const setGOCSFunction = useCallback((functionName: string, intensity: number, duration: number) => {
    audioEngine.executeGOCSFunction(functionName, intensity, duration);
    setAudioState(prev => ({
      ...prev,
      gocs: {
        ...prev.gocs,
        currentFunction: functionName,
        intensity,
        duration,
      },
    }));
  }, [audioEngine]);

  const updateAIDConfig = useCallback((config: Partial<AIDConfig>) => {
    audioEngine.updateAIDConfig(config);
    setAudioState(prev => ({
      ...prev,
      aid: { ...prev.aid, ...config },
    }));
  }, [audioEngine]);

  const value: AudioContextType = {
    audioState,
    isPlaying,
    play,
    stop,
    advanceFrame,
    setGOCSFunction,
    updateAIDConfig,
    audioEngine,
  };

  return <AudioContext.Provider value={value}>{children}</AudioContext.Provider>;
};
