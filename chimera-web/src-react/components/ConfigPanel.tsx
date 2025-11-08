/**
 * ConfigPanel - Simulation configuration controls
 * 
 * Provides form controls for SimulationConfig parameters with
 * validation and preset configurations.
 */

import React, { useState } from 'react';
import { SimulationConfig, SIMULATION_PRESETS } from '../types';

export interface ConfigPanelProps {
  config: SimulationConfig;
  onChange: (config: SimulationConfig) => void;
  disabled?: boolean;
}

const ConfigPanel: React.FC<ConfigPanelProps> = ({ config, onChange, disabled = false }) => {
  const [validationErrors, setValidationErrors] = useState<Record<string, string>>({});

  const validateAndUpdate = (field: keyof SimulationConfig, value: any) => {
    const newConfig = { ...config, [field]: value };
    const errors: Record<string, string> = {};

    // Validation rules
    if (field === 'snr_db' && (value < -10 || value > 30)) {
      errors.snr_db = 'SNR must be between -10 and 30 dB';
    }
    if (field === 'link_loss_db' && (value < 0 || value > 50)) {
      errors.link_loss_db = 'Link loss must be between 0 and 50 dB';
    }
    if (field === 'sample_rate' && value < 8000) {
      errors.sample_rate = 'Sample rate must be at least 8000 Hz';
    }
    if (field === 'plaintext_source' && value.length === 0) {
      errors.plaintext_source = 'Message cannot be empty';
    }

    setValidationErrors(errors);

    // Only update if valid
    if (Object.keys(errors).length === 0) {
      onChange(newConfig);
    }
  };

  const loadPreset = (presetIndex: number) => {
    const preset = SIMULATION_PRESETS[presetIndex];
    if (preset) {
      onChange(preset.config);
      setValidationErrors({});
    }
  };

  return (
    <div className="config-panel">
      <h2>Configuration</h2>

      {/* Presets */}
      <div className="config-section">
        <label className="config-label">Presets</label>
        <select
          className="config-select"
          onChange={(e) => loadPreset(parseInt(e.target.value))}
          disabled={disabled}
          defaultValue=""
        >
          <option value="" disabled>
            Select preset...
          </option>
          {SIMULATION_PRESETS.map((preset, index) => (
            <option key={index} value={index}>
              {preset.name} - {preset.description}
            </option>
          ))}
        </select>
      </div>

      {/* Message */}
      <div className="config-section">
        <label className="config-label">
          Message
          {validationErrors.plaintext_source && (
            <span className="validation-error">{validationErrors.plaintext_source}</span>
          )}
        </label>
        <textarea
          className="config-textarea"
          value={config.plaintext_source}
          onChange={(e) => validateAndUpdate('plaintext_source', e.target.value)}
          disabled={disabled}
          rows={4}
          placeholder="Enter message to encode..."
        />
        <div className="config-hint">
          {config.plaintext_source.length} characters
        </div>
      </div>

      {/* SNR */}
      <div className="config-section">
        <label className="config-label">
          SNR (Es/N0)
          {validationErrors.snr_db && (
            <span className="validation-error">{validationErrors.snr_db}</span>
          )}
        </label>
        <div className="config-slider-container">
          <input
            type="range"
            className="config-slider"
            min="-10"
            max="30"
            step="0.5"
            value={config.snr_db}
            onChange={(e) => validateAndUpdate('snr_db', parseFloat(e.target.value))}
            disabled={disabled}
          />
          <input
            type="number"
            className="config-number"
            value={config.snr_db}
            onChange={(e) => validateAndUpdate('snr_db', parseFloat(e.target.value))}
            disabled={disabled}
            step="0.5"
            min="-10"
            max="30"
          />
          <span className="config-unit">dB</span>
        </div>
        <div className="config-hint">Signal-to-noise ratio</div>
      </div>

      {/* Link Loss */}
      <div className="config-section">
        <label className="config-label">
          Link Loss
          {validationErrors.link_loss_db && (
            <span className="validation-error">{validationErrors.link_loss_db}</span>
          )}
        </label>
        <div className="config-slider-container">
          <input
            type="range"
            className="config-slider"
            min="0"
            max="50"
            step="1"
            value={config.link_loss_db}
            onChange={(e) => validateAndUpdate('link_loss_db', parseFloat(e.target.value))}
            disabled={disabled}
          />
          <input
            type="number"
            className="config-number"
            value={config.link_loss_db}
            onChange={(e) => validateAndUpdate('link_loss_db', parseFloat(e.target.value))}
            disabled={disabled}
            step="1"
            min="0"
            max="50"
          />
          <span className="config-unit">dB</span>
        </div>
        <div className="config-hint">Path loss attenuation</div>
      </div>

      {/* Sample Rate */}
      <div className="config-section">
        <label className="config-label">
          Sample Rate
          {validationErrors.sample_rate && (
            <span className="validation-error">{validationErrors.sample_rate}</span>
          )}
        </label>
        <select
          className="config-select"
          value={config.sample_rate}
          onChange={(e) => validateAndUpdate('sample_rate', parseInt(e.target.value))}
          disabled={disabled}
        >
          <option value={8000}>8 kHz</option>
          <option value={16000}>16 kHz</option>
          <option value={22050}>22.05 kHz</option>
          <option value={44100}>44.1 kHz</option>
          <option value={48000}>48 kHz</option>
          <option value={96000}>96 kHz</option>
        </select>
        <div className="config-hint">Audio sample rate</div>
      </div>

      {/* Bit Depth */}
      <div className="config-section">
        <label className="config-label">Bit Depth</label>
        <select
          className="config-select"
          value={config.bit_depth}
          onChange={(e) => validateAndUpdate('bit_depth', e.target.value)}
          disabled={disabled}
        >
          <option value="Pcm16">16-bit PCM</option>
          <option value="Pcm24">24-bit PCM</option>
          <option value="Pcm32">32-bit PCM</option>
          <option value="Float32">32-bit Float</option>
        </select>
        <div className="config-hint">Audio sample format</div>
      </div>

      {/* RNG Seed */}
      <div className="config-section">
        <label className="config-label">RNG Seed (Optional)</label>
        <input
          type="number"
          className="config-input"
          value={config.rng_seed ?? ''}
          onChange={(e) =>
            validateAndUpdate('rng_seed', e.target.value ? parseInt(e.target.value) : undefined)
          }
          disabled={disabled}
          placeholder="Random"
        />
        <div className="config-hint">For reproducible simulations</div>
      </div>

      {/* Info Panel */}
      <div className="config-info">
        <h3>System Info</h3>
        <div className="info-grid">
          <div className="info-row">
            <span className="info-label">Modulation:</span>
            <span className="info-value">QPSK</span>
          </div>
          <div className="info-row">
            <span className="info-label">FEC:</span>
            <span className="info-value">LDPC (1/2)</span>
          </div>
          <div className="info-row">
            <span className="info-label">Symbol Rate:</span>
            <span className="info-value">16 Hz</span>
          </div>
        </div>
      </div>
    </div>
  );
};

export default ConfigPanel;
