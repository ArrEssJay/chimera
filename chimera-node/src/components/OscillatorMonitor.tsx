import React, { useEffect, useState } from 'react';
import { useAudio } from '../audio/AudioProvider';
import './OscillatorMonitor.css';

interface OscillatorState {
  carrier: {
    frequency: number;
    amplitude: number;
    phase: number;
  };
  modulation: {
    phaseModFreq: number;
    phaseModDepth: number;
    freqModFreq: number;
    freqModDepth: number;
    ampModFreq: number;
    ampModDepth: number;
  };
  fsk: {
    state: number;
    pattern: string;
    rate: number;
  };
}

const OscillatorMonitor: React.FC = () => {
  const { audioState, isPlaying } = useAudio();
  const { currentFrame } = audioState;
  
  const [state, setState] = useState<OscillatorState>({
    carrier: { frequency: 12000, amplitude: 0.5, phase: 0 },
    modulation: {
      phaseModFreq: 0,
      phaseModDepth: 0,
      freqModFreq: 0,
      freqModDepth: 0,
      ampModFreq: 0,
      ampModDepth: 0,
    },
    fsk: { state: 0, pattern: 'idle', rate: 16 },
  });

  useEffect(() => {
    if (!isPlaying || !currentFrame) {
      return;
    }

    const interval = setInterval(() => {
      const frameData = currentFrame as any;
      const gocs = audioState.gocs;
      
      // Calculate current position in frame
      const frameProgress = 1 - (gocs.frameTimeRemaining / 8.0);
      const symbolIndex = Math.floor(frameProgress * 128);
      const clampedIndex = Math.min(127, Math.max(0, symbolIndex));
      
      // Get current values from frame arrays
      const fskStates = frameData.fskStates || [];
      const freqModulation = frameData.freqModulation || [];
      const ampModulation = frameData.ampModulation || [];
      const phaseRotation = frameData.phaseRotation || [];
      
      const currentFskState = fskStates[clampedIndex] || 0;
      const currentFreqMod = freqModulation[clampedIndex] || 0;
      const currentAmpMod = ampModulation[clampedIndex] || 0.9;
      const currentPhase = phaseRotation[clampedIndex] || 0;
      
      // Calculate carrier frequency based on FSK state and freq modulation
      const baseCarrier = 12000;
      const fskOffset = currentFskState === 0 ? -1 : 1; // ±1 Hz for FSK
      const actualFreq = baseCarrier + fskOffset + currentFreqMod;
      
      // Calculate modulation statistics
      const avgFreqMod = freqModulation.length > 0 
        ? freqModulation.reduce((a: number, b: number) => a + Math.abs(b), 0) / freqModulation.length 
        : 0;
      const maxFreqMod = freqModulation.length > 0 
        ? Math.max(...freqModulation.map((v: number) => Math.abs(v)))
        : 0;
        
      const avgAmpMod = ampModulation.length > 0
        ? ampModulation.reduce((a: number, b: number) => a + b, 0) / ampModulation.length
        : 0.9;
      const ampModRange = ampModulation.length > 0
        ? Math.max(...ampModulation) - Math.min(...ampModulation)
        : 0;
      
      setState({
        carrier: {
          frequency: actualFreq,
          amplitude: currentAmpMod,
          phase: (currentPhase / 4) * 360, // Phase rotation is 0-3, convert to degrees
        },
        modulation: {
          phaseModFreq: 2.0, // From frame structure
          phaseModDepth: ampModRange > 0 ? 0.175 : 0,
          freqModFreq: 2.0,
          freqModDepth: maxFreqMod > 0 ? (maxFreqMod / 0.3) : 0,
          ampModFreq: 2.0,
          ampModDepth: ampModRange > 0 ? (ampModRange / 0.175) : 0,
        },
        fsk: {
          state: currentFskState,
          pattern: clampedIndex < 32 ? 'low' : clampedIndex < 64 ? 'high' : clampedIndex < 96 ? 'low' : 'high',
          rate: 16, // Symbol rate
        },
      });
    }, 50); // Update 20 times per second

    return () => clearInterval(interval);
  }, [currentFrame, isPlaying, audioState.gocs]);

  const formatFreq = (freq: number) => {
    if (freq < 1) return `${(freq * 1000).toFixed(0)} mHz`;
    if (freq < 1000) return `${freq.toFixed(2)} Hz`;
    return `${(freq / 1000).toFixed(2)} kHz`;
  };

  const formatPercent = (value: number) => `${(value * 100).toFixed(0)}%`;

  return (
    <div className="oscillator-monitor">
      <div className="monitor-header">OSCILLATOR STATE</div>

      <div className="monitor-section">
        <div className="section-label">CARRIER</div>
        <div className="monitor-row">
          <span className="param-name">Frequency:</span>
          <span className="param-value">{formatFreq(state.carrier.frequency)}</span>
          <div className="param-bar">
            <div
              className="param-fill"
              style={{ width: `${((state.carrier.frequency - 11900) / 200) * 100}%` }}
            ></div>
          </div>
        </div>
        <div className="monitor-row">
          <span className="param-name">Amplitude:</span>
          <span className="param-value">{formatPercent(state.carrier.amplitude)}</span>
          <div className="param-bar">
            <div
              className="param-fill"
              style={{ width: `${state.carrier.amplitude * 100}%` }}
            ></div>
          </div>
        </div>
        <div className="monitor-row">
          <span className="param-name">Phase:</span>
          <span className="param-value">{state.carrier.phase.toFixed(0)}°</span>
          <div className="param-bar">
            <div
              className="param-fill"
              style={{ width: `${(state.carrier.phase / 360) * 100}%` }}
            ></div>
          </div>
        </div>
      </div>

      <div className="monitor-section">
        <div className="section-label">MODULATION</div>
        {state.modulation.phaseModDepth > 0 ? (
          <div className="monitor-row">
            <span className="param-name">Phase LFO:</span>
            <span className="param-value">{formatFreq(state.modulation.phaseModFreq)}</span>
            <span className="param-depth">{formatPercent(state.modulation.phaseModDepth)}</span>
          </div>
        ) : null}
        {state.modulation.freqModDepth > 0 ? (
          <div className="monitor-row">
            <span className="param-name">Freq LFO:</span>
            <span className="param-value">{formatFreq(state.modulation.freqModFreq)}</span>
            <span className="param-depth">{formatPercent(state.modulation.freqModDepth)}</span>
          </div>
        ) : null}
        {state.modulation.ampModDepth > 0 ? (
          <div className="monitor-row">
            <span className="param-name">Amp LFO:</span>
            <span className="param-value">{formatFreq(state.modulation.ampModFreq)}</span>
            <span className="param-depth">{formatPercent(state.modulation.ampModDepth)}</span>
          </div>
        ) : null}
        {state.modulation.phaseModDepth === 0 &&
        state.modulation.freqModDepth === 0 &&
        state.modulation.ampModDepth === 0 ? (
          <div className="no-modulation">No active modulation</div>
        ) : null}
      </div>

      <div className="monitor-section">
        <div className="section-label">FSK SUBLIMINAL</div>
        <div className="monitor-row">
          <span className="param-name">State:</span>
          <span className="param-value">{state.fsk.state === 0 ? 'LOW (11.999kHz)' : 'HIGH (12.001kHz)'}</span>
          <div className="fsk-indicator" data-state={state.fsk.state}></div>
        </div>
        <div className="monitor-row">
          <span className="param-name">Pattern:</span>
          <span className="param-value">{state.fsk.pattern}</span>
        </div>
        <div className="monitor-row">
          <span className="param-name">Rate:</span>
          <span className="param-value">{formatFreq(state.fsk.rate)}</span>
        </div>
      </div>
    </div>
  );
};

export default OscillatorMonitor;
