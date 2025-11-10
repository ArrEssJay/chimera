/**
 * ThzControlPanel - THz Carrier Modulation Controls
 * 
 * Provides controls for Auditory Intermodulation Distortion (AID) effect simulation.
 * Controls modulation depth, mixing coefficient, and mode (idle/active).
 */

import React, { useState } from 'react';

export interface ThzControlPanelProps {
  disabled?: boolean;
  onModulationModeChange?: (active: boolean) => void;
  onModulationDepthChange?: (depth: number) => void;
  onMixingCoefficientChange?: (coefficient: number) => void;
  onGenerateIdleCarrier?: () => void;
}

const ThzControlPanel: React.FC<ThzControlPanelProps> = ({
  disabled = false,
  onModulationModeChange,
  onModulationDepthChange,
  onMixingCoefficientChange,
  onGenerateIdleCarrier,
}) => {
  const [isActiveMode, setIsActiveMode] = useState(false);
  const [modulationDepth, setModulationDepth] = useState(0.05); // 5% idle default
  const [mixingCoefficient, setMixingCoefficient] = useState(0.7);

  const handleModeToggle = () => {
    const newMode = !isActiveMode;
    setIsActiveMode(newMode);
    
    // Auto-adjust modulation depth based on mode
    const newDepth = newMode ? 0.75 : 0.05;
    setModulationDepth(newDepth);
    
    if (onModulationModeChange) {
      onModulationModeChange(newMode);
    }
    if (onModulationDepthChange) {
      onModulationDepthChange(newDepth);
    }
  };

  const handleDepthChange = (value: number) => {
    setModulationDepth(value);
    if (onModulationDepthChange) {
      onModulationDepthChange(value);
    }
  };

  const handleMixingChange = (value: number) => {
    setMixingCoefficient(value);
    if (onMixingCoefficientChange) {
      onMixingCoefficientChange(value);
    }
  };

  return (
    <div className="thz-control-panel">
      <h3>THz Carrier Modulation</h3>
      <div className="thz-hint">
        Simulates Auditory Intermodulation Distortion (AID) with two THz carriers
      </div>

      {/* Modulation Mode Toggle */}
      <div className="config-section">
        <label className="config-label">
          Modulation Mode
          {!disabled && <span className="live-badge">LIVE</span>}
        </label>
        <div className="mode-toggle-container">
          <button
            className={`mode-toggle-btn ${!isActiveMode ? 'active' : ''}`}
            onClick={() => {
              if (!isActiveMode) return;
              handleModeToggle();
            }}
            disabled={disabled}
          >
            Idle (&lt;5%)
          </button>
          <button
            className={`mode-toggle-btn ${isActiveMode ? 'active' : ''}`}
            onClick={() => {
              if (isActiveMode) return;
              handleModeToggle();
            }}
            disabled={disabled}
          >
            Active (70-80%)
          </button>
        </div>
        <div className="config-hint">
          {isActiveMode 
            ? 'High modulation depth for data transmission' 
            : 'Low modulation depth for baseline carrier'}
        </div>
      </div>

      {/* Modulation Depth Slider */}
      <div className="config-section">
        <label className="config-label">
          Modulation Depth
          {!disabled && <span className="live-badge">LIVE</span>}
        </label>
        <div className="config-slider-container">
          <input
            type="range"
            className="config-slider"
            min="0.01"
            max="1.0"
            step="0.01"
            value={modulationDepth}
            onChange={(e) => handleDepthChange(parseFloat(e.target.value))}
            disabled={disabled}
          />
          <input
            type="number"
            className="config-number"
            value={modulationDepth.toFixed(2)}
            onChange={(e) => handleDepthChange(parseFloat(e.target.value))}
            disabled={disabled}
            step="0.01"
            min="0.01"
            max="1.0"
          />
          <span className="config-unit">{(modulationDepth * 100).toFixed(0)}%</span>
        </div>
        <div className="config-hint">
          Controls AM modulation depth on 1.875 THz data carrier
        </div>
      </div>

      {/* Mixing Coefficient Slider */}
      <div className="config-section">
        <label className="config-label">
          Mixing Coefficient
          {!disabled && <span className="live-badge">LIVE</span>}
        </label>
        <div className="config-slider-container">
          <input
            type="range"
            className="config-slider"
            min="0.0"
            max="1.0"
            step="0.05"
            value={mixingCoefficient}
            onChange={(e) => handleMixingChange(parseFloat(e.target.value))}
            disabled={disabled}
          />
          <input
            type="number"
            className="config-number"
            value={mixingCoefficient.toFixed(2)}
            onChange={(e) => handleMixingChange(parseFloat(e.target.value))}
            disabled={disabled}
            step="0.05"
            min="0.0"
            max="1.0"
          />
        </div>
        <div className="config-hint">
          Third-order intermodulation strength (biological response efficiency)
        </div>
      </div>

      {/* Idle Carrier Generator */}
      <div className="config-section">
        <label className="config-label">Calibration</label>
        <button
          className="btn btn-secondary"
          onClick={onGenerateIdleCarrier}
          disabled={disabled}
          style={{ width: '100%' }}
        >
          Generate Idle Carrier
        </button>
        <div className="config-hint">
          Generate 100ms of idle carrier audio for baseline calibration
        </div>
      </div>

      {/* Technical Info */}
      <div className="thz-info">
        <details>
          <summary>Technical Details</summary>
          <div className="thz-info-content">
            <p><strong>Pump Beam:</strong> 1.998 THz (unmodulated)</p>
            <p><strong>Data Carrier:</strong> 1.875 THz (AM modulated)</p>
            <p><strong>Difference Freq:</strong> 123 GHz → 12 kHz audio</p>
            <p><strong>Mechanism:</strong> Third-order intermodulation (|E|² × Re(E))</p>
          </div>
        </details>
      </div>
    </div>
  );
};

export default ThzControlPanel;
