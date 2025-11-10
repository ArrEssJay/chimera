/**
 * ConfigPanel - Full frame preset configuration controls
 * 
 * Provides form controls for all ProtocolConfig, SimulationConfig, and LDPCConfig
 * parameters with preset selection.
 */

import React, { useState } from 'react';
import {
  ConfigBundle,
  FRAME_PRESETS,
  FramePresetKey,
  ProtocolConfig,
  SimulationConfig,
  LDPCConfig,
  FrameLayout,
} from '../types';

export interface ConfigPanelProps {
  config: ConfigBundle;
  onChange: (config: ConfigBundle) => void;
  disabled?: boolean;
  allowRuntimeUpdate?: boolean;
}

const ConfigPanel: React.FC<ConfigPanelProps> = ({
  config,
  onChange,
  disabled = false,
  allowRuntimeUpdate = false,
}) => {
  const [selectedPreset, setSelectedPreset] = useState<FramePresetKey | ''>('raman-whisper');

  const loadPreset = async (presetKey: FramePresetKey) => {
    try {
      const wasmModule = await import('../../pkg/chimera_web');
      const bundleJson = wasmModule.get_preset_bundle(presetKey);
      if (bundleJson) {
        const bundle: ConfigBundle = JSON.parse(bundleJson);
        onChange(bundle);
        setSelectedPreset(presetKey);
      }
    } catch (error) {
      console.error('Failed to load preset:', error);
    }
  };

  const updateSimulation = <K extends keyof SimulationConfig>(
    field: K,
    value: SimulationConfig[K]
  ) => {
    onChange({
      ...config,
      simulation: {
        ...config.simulation,
        [field]: value,
      },
    });
  };

  const updateProtocol = <K extends keyof ProtocolConfig>(
    field: K,
    value: ProtocolConfig[K]
  ) => {
    onChange({
      ...config,
      protocol: {
        ...config.protocol,
        [field]: value,
      },
    });
  };

  const updateFrameLayout = <K extends keyof FrameLayout>(
    field: K,
    value: FrameLayout[K]
  ) => {
    onChange({
      ...config,
      protocol: {
        ...config.protocol,
        frame_layout: {
          ...config.protocol.frame_layout,
          [field]: value,
        },
      },
    });
  };

  const updateLDPC = <K extends keyof LDPCConfig>(field: K, value: LDPCConfig[K]) => {
    onChange({
      ...config,
      ldpc: {
        ...config.ldpc,
        [field]: value,
      },
    });
  };

  // Determine if field can be updated during runtime
  const canUpdate = (field: 'snr' | 'link_loss' | 'other') => {
    if (!disabled) return true; // Always editable when stopped
    if (!allowRuntimeUpdate) return false; // Runtime updates disabled
    return field === 'snr' || field === 'link_loss'; // Only these during runtime
  };

  return (
    <div className="config-panel">
      <h2>Configuration</h2>

      {/* Frame Preset Selector */}
      <div className="config-section">
        <label className="config-label">Frame Preset</label>
        <select
          className="config-select"
          value={selectedPreset}
          onChange={(e) => {
            const key = e.target.value as FramePresetKey;
            if (key) loadPreset(key);
          }}
          disabled={disabled && !allowRuntimeUpdate}
        >
          <option value="">Custom Configuration</option>
          {FRAME_PRESETS.map((preset) => (
            <option key={preset.key} value={preset.key}>
              {preset.displayName}
            </option>
          ))}
        </select>
        {selectedPreset && (
          <div className="config-hint">
            {FRAME_PRESETS.find((p) => p.key === selectedPreset)?.description}
          </div>
        )}
      </div>

      {/* Simulation Parameters */}
      <div className="config-section-group">
        <h3>Simulation</h3>

        <div className="config-section">
          <label className="config-label">Message</label>
          <textarea
            className="config-textarea"
            value={config.simulation.plaintext_source}
            onChange={(e) => updateSimulation('plaintext_source', e.target.value)}
            disabled={!canUpdate('other')}
            rows={3}
            placeholder="Enter message to encode..."
          />
          <div className="config-hint">{config.simulation.plaintext_source.length} characters</div>
        </div>

        <div className="config-section">
          <label className="config-label">
            SNR (Es/N0)
            {allowRuntimeUpdate && disabled && <span className="live-badge">LIVE</span>}
          </label>
          <div className="config-slider-container">
            <input
              type="range"
              className="config-slider"
              min="-10"
              max="30"
              step="0.5"
              value={config.simulation.snr_db}
              onChange={(e) => updateSimulation('snr_db', parseFloat(e.target.value))}
              disabled={!canUpdate('snr')}
            />
            <input
              type="number"
              className="config-number"
              value={config.simulation.snr_db}
              onChange={(e) => updateSimulation('snr_db', parseFloat(e.target.value))}
              disabled={!canUpdate('snr')}
              step="0.5"
            />
            <span className="config-unit">dB</span>
          </div>
        </div>

        <div className="config-section">
          <label className="config-label">
            Link Loss
            {allowRuntimeUpdate && disabled && <span className="live-badge">LIVE</span>}
          </label>
          <div className="config-slider-container">
            <input
              type="range"
              className="config-slider"
              min="0"
              max="50"
              step="1"
              value={config.simulation.link_loss_db}
              onChange={(e) => updateSimulation('link_loss_db', parseFloat(e.target.value))}
              disabled={!canUpdate('link_loss')}
            />
            <input
              type="number"
              className="config-number"
              value={config.simulation.link_loss_db}
              onChange={(e) => updateSimulation('link_loss_db', parseFloat(e.target.value))}
              disabled={!canUpdate('link_loss')}
              step="1"
            />
            <span className="config-unit">dB</span>
          </div>
        </div>

        <div className="config-section">
          <label className="config-label">RNG Seed (Optional)</label>
          <input
            type="number"
            className="config-input"
            value={config.simulation.rng_seed ?? ''}
            onChange={(e) =>
              updateSimulation('rng_seed', e.target.value ? parseInt(e.target.value) : undefined)
            }
            disabled={!canUpdate('other')}
            placeholder="Random"
          />
        </div>
      </div>

      {/* Protocol Parameters */}
      <div className="config-section-group">
        <h3>Protocol</h3>

        <div className="config-section">
          <label className="config-label">Carrier Frequency</label>
          <div className="config-slider-container">
            <input
              type="number"
              className="config-input"
              value={config.protocol.carrier_freq_hz}
              onChange={(e) => updateProtocol('carrier_freq_hz', parseFloat(e.target.value))}
              disabled={!canUpdate('other')}
              step="0.1"
            />
            <span className="config-unit">Hz</span>
          </div>
        </div>

        <div className="config-section">
          <label className="config-label">QPSK Symbol Rate</label>
          <input
            type="number"
            className="config-input"
            value={config.protocol.qpsk_symbol_rate}
            onChange={(e) => updateProtocol('qpsk_symbol_rate', parseInt(e.target.value))}
            disabled={!canUpdate('other')}
          />
        </div>

        <div className="config-section">
          <label className="config-label">QPSK Bandwidth</label>
          <div className="config-slider-container">
            <input
              type="number"
              className="config-input"
              value={config.protocol.qpsk_bandwidth_hz}
              onChange={(e) => updateProtocol('qpsk_bandwidth_hz', parseFloat(e.target.value))}
              disabled={!canUpdate('other')}
              step="0.1"
            />
            <span className="config-unit">Hz</span>
          </div>
        </div>

        <div className="config-section">
          <label className="config-label">FSK Bit Rate</label>
          <input
            type="number"
            className="config-input"
            value={config.protocol.fsk_bit_rate}
            onChange={(e) => updateProtocol('fsk_bit_rate', parseFloat(e.target.value))}
            disabled={!canUpdate('other')}
            step="0.1"
          />
        </div>

        <div className="config-section">
          <label className="config-label">Sync Sequence (Hex)</label>
          <input
            type="text"
            className="config-input"
            value={config.protocol.sync_sequence_hex}
            onChange={(e) => updateProtocol('sync_sequence_hex', e.target.value)}
            disabled={!canUpdate('other')}
            placeholder="A5A5A5A5"
          />
        </div>

        <div className="config-section">
          <label className="config-label">Target ID (Hex)</label>
          <input
            type="text"
            className="config-input"
            value={config.protocol.target_id_hex}
            onChange={(e) => updateProtocol('target_id_hex', e.target.value)}
            disabled={!canUpdate('other')}
            placeholder="DEADBEEF"
          />
        </div>

        <div className="config-section">
          <label className="config-label">Command Opcode</label>
          <input
            type="number"
            className="config-input"
            value={config.protocol.command_opcode}
            onChange={(e) => updateProtocol('command_opcode', parseInt(e.target.value))}
            disabled={!canUpdate('other')}
          />
        </div>

        <div className="config-section">
          <label className="config-label">Max Frames</label>
          <input
            type="number"
            className="config-input"
            value={config.protocol.max_frames}
            onChange={(e) => updateProtocol('max_frames', parseInt(e.target.value))}
            disabled={!canUpdate('other')}
          />
        </div>
      </div>

      {/* Frame Layout */}
      <div className="config-section-group">
        <h3>Frame Layout</h3>

        <div className="config-section">
          <label className="config-label">Total Symbols</label>
          <input
            type="number"
            className="config-input"
            value={config.protocol.frame_layout.total_symbols}
            onChange={(e) => updateFrameLayout('total_symbols', parseInt(e.target.value))}
            disabled={!canUpdate('other')}
          />
        </div>

        <div className="config-section">
          <label className="config-label">Sync Symbols</label>
          <input
            type="number"
            className="config-input"
            value={config.protocol.frame_layout.sync_symbols}
            onChange={(e) => updateFrameLayout('sync_symbols', parseInt(e.target.value))}
            disabled={!canUpdate('other')}
          />
        </div>

        <div className="config-section">
          <label className="config-label">Data Payload Symbols</label>
          <input
            type="number"
            className="config-input"
            value={config.protocol.frame_layout.data_payload_symbols}
            onChange={(e) => updateFrameLayout('data_payload_symbols', parseInt(e.target.value))}
            disabled={!canUpdate('other')}
          />
        </div>

        <div className="config-section">
          <label className="config-label">ECC Symbols</label>
          <input
            type="number"
            className="config-input"
            value={config.protocol.frame_layout.ecc_symbols}
            onChange={(e) => updateFrameLayout('ecc_symbols', parseInt(e.target.value))}
            disabled={!canUpdate('other')}
          />
        </div>
      </div>

      {/* LDPC Parameters */}
      <div className="config-section-group">
        <h3>LDPC</h3>

        <div className="config-section">
          <label className="config-label">Variable Node Degree (dv)</label>
          <input
            type="number"
            className="config-input"
            value={config.ldpc.dv}
            onChange={(e) => updateLDPC('dv', parseInt(e.target.value))}
            disabled={!canUpdate('other')}
          />
        </div>

        <div className="config-section">
          <label className="config-label">Check Node Degree (dc)</label>
          <input
            type="number"
            className="config-input"
            value={config.ldpc.dc}
            onChange={(e) => updateLDPC('dc', parseInt(e.target.value))}
            disabled={!canUpdate('other')}
          />
        </div>

        <div className="config-section">
          <label className="config-label">LDPC Seed (Optional)</label>
          <input
            type="number"
            className="config-input"
            value={config.ldpc.seed ?? ''}
            onChange={(e) => updateLDPC('seed', e.target.value ? parseInt(e.target.value) : undefined)}
            disabled={!canUpdate('other')}
            placeholder="Random"
          />
        </div>
      </div>
    </div>
  );
};

export default ConfigPanel;
