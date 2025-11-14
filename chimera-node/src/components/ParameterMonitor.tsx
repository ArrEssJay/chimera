import React from 'react';
import { useAudio } from '../audio/AudioProvider';

const ParameterMonitor: React.FC = () => {
  const { audioState } = useAudio();
  const { currentFrame } = audioState;

  if (!currentFrame) {
    return (
      <div className="parameter-monitor">
        <div className="param-section">
          <h4>System State</h4>
          <div className="param-grid">
            <div className="param-item inactive">
              <span className="param-label">Status</span>
              <span className="param-value">No Frame Data</span>
            </div>
          </div>
        </div>
      </div>
    );
  }

  const { lfoParams } = currentFrame;

  return (
    <div className="parameter-monitor">
      <div className="param-section">
        <h4>Phase Modulation</h4>
        <div className="param-grid">
          <div className="param-item">
            <span className="param-label">Waveform</span>
            <span className="param-value">{lfoParams.phase.waveform}</span>
          </div>
          <div className="param-item">
            <span className="param-label">Frequency</span>
            <span className="param-value">{lfoParams.phase.frequency.toFixed(2)} Hz</span>
          </div>
          <div className="param-item">
            <span className="param-label">Depth</span>
            <span className="param-value">{(lfoParams.phase.depth * 100).toFixed(0)}%</span>
          </div>
        </div>
      </div>

      <div className="param-section">
        <h4>Frequency Modulation</h4>
        <div className="param-grid">
          <div className="param-item">
            <span className="param-label">Waveform</span>
            <span className="param-value">{lfoParams.freqMod.waveform}</span>
          </div>
          <div className="param-item">
            <span className="param-label">Frequency</span>
            <span className="param-value">{lfoParams.freqMod.frequency.toFixed(2)} Hz</span>
          </div>
          <div className="param-item">
            <span className="param-label">Depth</span>
            <span className="param-value">{(lfoParams.freqMod.depth * 100).toFixed(0)}%</span>
          </div>
        </div>
      </div>

      <div className="param-section">
        <h4>Amplitude Modulation</h4>
        <div className="param-grid">
          <div className="param-item">
            <span className="param-label">Waveform</span>
            <span className="param-value">{lfoParams.ampMod.waveform}</span>
          </div>
          <div className="param-item">
            <span className="param-label">Frequency</span>
            <span className="param-value">{lfoParams.ampMod.frequency.toFixed(2)} Hz</span>
          </div>
          <div className="param-item">
            <span className="param-label">Depth</span>
            <span className="param-value">{(lfoParams.ampMod.depth * 100).toFixed(0)}%</span>
          </div>
        </div>
      </div>

      <div className="param-section">
        <h4>FSK Pattern</h4>
        <div className="param-grid">
          <div className="param-item">
            <span className="param-label">Pattern</span>
            <span className="param-value">{lfoParams.fsk.pattern}</span>
          </div>
          <div className="param-item">
            <span className="param-label">Rate</span>
            <span className="param-value">{lfoParams.fsk.rate.toFixed(1)} Hz</span>
          </div>
        </div>
      </div>
    </div>
  );
};

export default ParameterMonitor;
