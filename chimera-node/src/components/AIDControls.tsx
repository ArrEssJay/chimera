import React from 'react';
import { useAudio } from '../audio/AudioProvider';

const AIDControls: React.FC = () => {
  const { audioState, updateAIDConfig } = useAudio();
  const { aid } = audioState;

  return (
    <div className="aid-controls">
      <div className="control-group">
        <div className="toggle-group">
          <label htmlFor="aid-enabled">Enable AID Simulation</label>
          <label className="toggle-switch">
            <input
              id="aid-enabled"
              type="checkbox"
              checked={aid.enabled}
              onChange={(e) => updateAIDConfig({ enabled: e.target.checked })}
            />
            <span className="toggle-slider"></span>
          </label>
        </div>
      </div>

      <div className="control-group">
        <label htmlFor="aid-modulation">
          Modulation Depth: {(aid.modulationDepth * 100).toFixed(1)}%
        </label>
        <div className="slider-container">
          <input
            id="aid-modulation"
            type="range"
            min="0"
            max="1"
            step="0.01"
            value={aid.modulationDepth}
            onChange={(e) => updateAIDConfig({ modulationDepth: parseFloat(e.target.value) })}
            disabled={!aid.enabled}
          />
          <span className="slider-value">{(aid.modulationDepth * 100).toFixed(0)}%</span>
        </div>
      </div>

      <div className="control-group">
        <label htmlFor="aid-mixing">
          Mixing Coefficient: {(aid.mixingCoefficient * 100).toFixed(1)}%
        </label>
        <div className="slider-container">
          <input
            id="aid-mixing"
            type="range"
            min="0"
            max="1"
            step="0.01"
            value={aid.mixingCoefficient}
            onChange={(e) => updateAIDConfig({ mixingCoefficient: parseFloat(e.target.value) })}
            disabled={!aid.enabled}
          />
          <span className="slider-value">{(aid.mixingCoefficient * 100).toFixed(0)}%</span>
        </div>
      </div>

      <div className="control-group">
        <label htmlFor="aid-phase-noise">
          Phase Noise (σ): {(aid.phaseNoiseStd * 1000).toFixed(2)} × 10⁻³
        </label>
        <div className="slider-container">
          <input
            id="aid-phase-noise"
            type="range"
            min="0"
            max="0.01"
            step="0.0001"
            value={aid.phaseNoiseStd}
            onChange={(e) => updateAIDConfig({ phaseNoiseStd: parseFloat(e.target.value) })}
            disabled={!aid.enabled}
          />
          <span className="slider-value">{(aid.phaseNoiseStd * 1000).toFixed(1)}</span>
        </div>
      </div>

      <div className="control-group">
        <div className="toggle-group">
          <label htmlFor="aid-bypass">Bypass Mode (Validation)</label>
          <label className="toggle-switch">
            <input
              id="aid-bypass"
              type="checkbox"
              checked={aid.bypassSimulation}
              onChange={(e) => updateAIDConfig({ bypassSimulation: e.target.checked })}
              disabled={!aid.enabled}
            />
            <span className="toggle-slider"></span>
          </label>
        </div>
      </div>

      <div className="aid-info">
        <p className="info-text">
          {aid.enabled 
            ? aid.bypassSimulation 
              ? '⚠️ Bypass mode: Signal passes through unchanged'
              : '✓ AID simulation active'
            : 'ℹ️ AID simulation disabled'}
        </p>
      </div>
    </div>
  );
};

export default AIDControls;
