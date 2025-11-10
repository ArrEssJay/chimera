/**
 * ModulationControlPanel - System Modulation Controls
 * 
 * Provides controls to enable/disable QPSK, FSK, and THz modulation layers
 * for isolating signal components and debugging audio artifacts.
 */

import React, { useState } from 'react';

export interface ModulationControlPanelProps {
  disabled?: boolean;
  onQpskEnableChange?: (enabled: boolean) => void;
  onFskEnableChange?: (enabled: boolean) => void;
  onThzBypassChange?: (bypass: boolean) => void;
}

const ModulationControlPanel: React.FC<ModulationControlPanelProps> = ({
  disabled = false,
  onQpskEnableChange,
  onFskEnableChange,
  onThzBypassChange,
}) => {
  const [qpskEnabled, setQpskEnabled] = useState(true);
  const [fskEnabled, setFskEnabled] = useState(true);
  const [thzBypassed, setThzBypassed] = useState(false);

  const handleQpskToggle = () => {
    const newState = !qpskEnabled;
    setQpskEnabled(newState);
    if (onQpskEnableChange) {
      onQpskEnableChange(newState);
    }
  };

  const handleFskToggle = () => {
    const newState = !fskEnabled;
    setFskEnabled(newState);
    if (onFskEnableChange) {
      onFskEnableChange(newState);
    }
  };

  const handleThzBypassToggle = () => {
    const newState = !thzBypassed;
    setThzBypassed(newState);
    if (onThzBypassChange) {
      onThzBypassChange(newState);
    }
  };

  const getStatusDescription = () => {
    const parts = [];
    parts.push(qpskEnabled ? 'QPSK' : '');
    parts.push(fskEnabled ? 'FSK' : '');
    parts.push(!thzBypassed ? 'THz' : '');
    const enabled = parts.filter(p => p).join('+');
    
    if (!enabled) {
      return 'Pure 12 kHz sine carrier (all modulation disabled)';
    }
    return `${enabled} modulation layers enabled`;
  };

  return (
    <div className="modulation-control-panel">
      <h3>🎛️ Modulation Control</h3>
      <div className="control-hint">
        Enable/disable modulation layers to isolate signal components
      </div>

      {/* QPSK Toggle */}
      <div className="config-section">
        <label className="config-label">
          <input
            type="checkbox"
            checked={qpskEnabled}
            onChange={handleQpskToggle}
            disabled={disabled}
            className="config-checkbox"
          />
          QPSK Modulation
          {!disabled && <span className="live-badge">LIVE</span>}
        </label>
        <div className="config-hint">
          {qpskEnabled 
            ? '✓ Phase modulation at 16 symbols/second with ~20 Hz bandwidth' 
            : '✗ Disabled - constant carrier phase'}
        </div>
      </div>

      {/* FSK Toggle */}
      <div className="config-section">
        <label className="config-label">
          <input
            type="checkbox"
            checked={fskEnabled}
            onChange={handleFskToggle}
            disabled={disabled}
            className="config-checkbox"
          />
          FSK Frequency Dithering
          {!disabled && <span className="live-badge">LIVE</span>}
        </label>
        <div className="config-hint">
          {fskEnabled 
            ? '✓ ±1 Hz frequency dithering at 1 bit/second (11999/12001 Hz)' 
            : '✗ Disabled - fixed 12 kHz carrier'}
        </div>
      </div>

      {/* THz Bypass Toggle */}
      <div className="config-section">
        <label className="config-label">
          <input
            type="checkbox"
            checked={!thzBypassed}
            onChange={handleThzBypassToggle}
            disabled={disabled}
            className="config-checkbox"
          />
          THz Carrier Simulation
          {!disabled && <span className="live-badge">LIVE</span>}
        </label>
        <div className="config-hint">
          {!thzBypassed 
            ? '✓ THz carrier modulation and non-linear mixing enabled' 
            : '✗ Bypassed - direct audio carrier output'}
        </div>
      </div>

      {/* Status Summary */}
      <div className="control-status">
        <strong>Current Output:</strong>
        <div className="control-status-text">{getStatusDescription()}</div>
      </div>

      {/* Usage Instructions */}
      <div className="control-info">
        <details>
          <summary>How to Use</summary>
          <div className="control-info-content">
            <p><strong>All Disabled:</strong> Pure 12 kHz sine wave. Baseline carrier with no modulation.</p>
            <p><strong>QPSK Only:</strong> Phase modulation at fixed carrier. Isolates QPSK-related effects.</p>
            <p><strong>FSK Only:</strong> ±1 Hz frequency shifts. Isolates FSK-related effects.</p>
            <p><strong>THz Bypassed:</strong> Direct audio output without THz carrier simulation. Bypasses AID effect.</p>
            <p><strong>All Enabled:</strong> Full system operation with FSK+QPSK+THz modulation.</p>
          </div>
        </details>
      </div>
    </div>
  );
};

export default ModulationControlPanel;
