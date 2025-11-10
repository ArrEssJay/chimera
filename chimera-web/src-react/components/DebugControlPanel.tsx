/**
 * DebugControlPanel - Modulation Debug Controls
 * 
 * Provides controls to enable/disable QPSK and FSK modulation layers
 * for debugging audio artifacts. With both disabled, outputs pure 12kHz sine.
 */

import React, { useState } from 'react';

export interface DebugControlPanelProps {
  disabled?: boolean;
  onQpskEnableChange?: (enabled: boolean) => void;
  onFskEnableChange?: (enabled: boolean) => void;
}

const DebugControlPanel: React.FC<DebugControlPanelProps> = ({
  disabled = false,
  onQpskEnableChange,
  onFskEnableChange,
}) => {
  const [qpskEnabled, setQpskEnabled] = useState(true);
  const [fskEnabled, setFskEnabled] = useState(true);

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

  const getStatusDescription = () => {
    if (qpskEnabled && fskEnabled) {
      return 'Full FSK+QPSK modulation (normal operation)';
    } else if (!qpskEnabled && !fskEnabled) {
      return 'Pure 12 kHz sine carrier (debugging)';
    } else if (qpskEnabled && !fskEnabled) {
      return 'QPSK only at fixed 12 kHz carrier';
    } else {
      return 'FSK frequency dithering only (±1 Hz)';
    }
  };

  return (
    <div className="debug-control-panel">
      <h3>🔧 Debug Controls</h3>
      <div className="debug-hint">
        Enable/disable modulation layers to isolate audio artifacts
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

      {/* Status Summary */}
      <div className="debug-status">
        <strong>Current Output:</strong>
        <div className="debug-status-text">{getStatusDescription()}</div>
      </div>

      {/* Usage Instructions */}
      <div className="debug-info">
        <details>
          <summary>How to Use</summary>
          <div className="debug-info-content">
            <p><strong>Both OFF:</strong> Pure 12 kHz sine wave. If you hear clicking/drumming, the problem is in carrier generation or THz processing.</p>
            <p><strong>QPSK OFF, FSK ON:</strong> Only ±1 Hz frequency shifts. Isolates FSK-related artifacts.</p>
            <p><strong>QPSK ON, FSK OFF:</strong> Only phase modulation at fixed carrier. Isolates QPSK-related artifacts.</p>
            <p><strong>Both ON:</strong> Normal operation with full FSK+QPSK modulation.</p>
          </div>
        </details>
      </div>
    </div>
  );
};

export default DebugControlPanel;
