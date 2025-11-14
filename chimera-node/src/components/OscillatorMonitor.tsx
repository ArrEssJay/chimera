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
    if (!isPlaying || !currentFrame || !currentFrame.lfoParams) {
      return;
    }

    // Update state from current frame's LFO parameters
    const { lfoParams } = currentFrame;
    
    setState({
      carrier: {
        frequency: 12000, // Fixed carrier
        amplitude: isPlaying ? 0.5 : 0,
        phase: (Date.now() % 1000) / 1000 * 360, // Simulate phase rotation
      },
      modulation: {
        phaseModFreq: lfoParams.phase.frequency,
        phaseModDepth: lfoParams.phase.depth,
        freqModFreq: lfoParams.freqMod.frequency,
        freqModDepth: lfoParams.freqMod.depth,
        ampModFreq: lfoParams.ampMod.frequency,
        ampModDepth: lfoParams.ampMod.depth,
      },
      fsk: {
        state: Math.random() > 0.5 ? 1 : 0, // Simulate FSK state changes
        pattern: lfoParams.fsk.pattern,
        rate: lfoParams.fsk.rate,
      },
    });
  }, [currentFrame, isPlaying]);

  useEffect(() => {
    if (!isPlaying) return;

    const interval = setInterval(() => {
      setState((prev) => ({
        ...prev,
        carrier: {
          ...prev.carrier,
          phase: (prev.carrier.phase + 36) % 360, // 10 updates/sec
        },
        fsk: {
          ...prev.fsk,
          state: Math.random() > 0.5 ? 1 : 0,
        },
      }));
    }, 100);

    return () => clearInterval(interval);
  }, [isPlaying]);

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
