import React from 'react';
import { useAudio } from '../audio/AudioProvider';
import './FrameStructureViewer.css';

interface FrameStructureViewerProps {
  effectName: string;
}

const FrameStructureViewer: React.FC<FrameStructureViewerProps> = ({ effectName }) => {
  const { audioState } = useAudio();
  const { currentFrame, gocs } = audioState;

  const formatHex = (value: number) => {
    return '0x' + value.toString(16).toUpperCase().padStart(2, '0');
  };

  const decodeVibrationalMode = (mode: number) => {
    const modes: { [key: number]: string } = {
      0x01: 'Breathing',
      0x02: 'Longitudinal',
      0x03: 'Torsional',
    };
    return modes[mode] || 'Unknown';
  };

  const decodeIntensityPattern = (pattern: number) => {
    const patterns: { [key: number]: string } = {
      0x10: 'Smooth Sine',
      0x20: 'Step Function',
      0x30: 'Pulsed',
      0x40: 'Chaotic',
    };
    return patterns[pattern] || 'Unknown';
  };

  const decodeCorticalRegion = (region: number) => {
    const regions: { [key: number]: string } = {
      0x01: 'Auditory',
      0x02: 'Visual',
      0x03: 'Motor',
      0x04: 'Prefrontal',
    };
    return regions[region] || 'Unknown';
  };

  const analyzeFSKPattern = (fskStates: number[]) => {
    if (!fskStates || fskStates.length === 0) return 'unknown';
    
    const ones = fskStates.filter(s => s === 1).length;
    const ratio = ones / fskStates.length;
    
    if (ratio === 0) return 'constant-0';
    if (ratio === 1) return 'constant-1';
    if (ratio > 0.4 && ratio < 0.6) {
      // Check for alternating pattern
      let isAlternating = true;
      for (let i = 1; i < Math.min(32, fskStates.length); i++) {
        if (fskStates[i] === fskStates[i-1]) {
          isAlternating = false;
          break;
        }
      }
      return isAlternating ? 'alternating' : 'mixed';
    }
    return 'random';
  };

  if (!currentFrame) {
    return (
      <div className="frame-structure-viewer">
        <div className="frame-header">FRAME STRUCTURE</div>
        <div className="no-frame">No active frame</div>
      </div>
    );
  }

  // Access the actual frame data structure
  const frameData = (currentFrame as any);
  const targetId = frameData.targetId || {};
  const commandType = frameData.commandType || {};
  const fskStates = frameData.fskStates || [];
  const freqModulation = frameData.freqModulation || [];
  const ampModulation = frameData.ampModulation || [];
  
  // Calculate FSK statistics
  const fskPattern = analyzeFSKPattern(fskStates);
  const fskOnes = fskStates.filter((s: number) => s === 1).length;
  const fskRatio = fskStates.length > 0 ? (fskOnes / fskStates.length) : 0;
  
  // Calculate modulation averages
  const avgFreqMod = freqModulation.length > 0 
    ? freqModulation.reduce((a: number, b: number) => a + b, 0) / freqModulation.length 
    : 0;
  const avgAmpMod = ampModulation.length > 0
    ? ampModulation.reduce((a: number, b: number) => a + b, 0) / ampModulation.length
    : 0;

  // Parse sequencing for current/total frames
  const currentFrameNum = (commandType.sequencing >> 4) & 0x0F;
  const totalFrames = commandType.sequencing & 0x0F;

  return (
    <div className="frame-structure-viewer">
      <div className="frame-header">
        FRAME STRUCTURE: {gocs.currentFunction || effectName || 'Direct Control'}
      </div>
      
      <div className="frame-section">
        <div className="section-title">TARGET ID (32-bit)</div>
        <div className="frame-field">
          <span className="field-label">Brainwave:</span>
          <span className="field-value">{targetId.baselineBrainwave || 0} Hz</span>
          <span className="field-hex">{formatHex(targetId.baselineBrainwave || 0)}</span>
        </div>
        <div className="frame-field">
          <span className="field-label">Hemisphere:</span>
          <span className="field-value">
            {targetId.hemisphereBias === 0x80 ? 'Balanced' : 
             targetId.hemisphereBias < 0x80 ? 'Left' : 'Right'}
          </span>
          <span className="field-hex">{formatHex(targetId.hemisphereBias || 0x80)}</span>
        </div>
        <div className="frame-field">
          <span className="field-label">Cortical:</span>
          <span className="field-value">{decodeCorticalRegion(targetId.corticalRegion || 0)}</span>
          <span className="field-hex">{formatHex(targetId.corticalRegion || 0)}</span>
        </div>
        <div className="frame-field">
          <span className="field-label">Resonance:</span>
          <span className="field-value">Key {targetId.resonanceKey || 0}</span>
          <span className="field-hex">{formatHex(targetId.resonanceKey || 0)}</span>
        </div>
      </div>

      <div className="frame-section">
        <div className="section-title">COMMAND TYPE (32-bit)</div>
        <div className="frame-field">
          <span className="field-label">Vibration:</span>
          <span className="field-value">{decodeVibrationalMode(commandType.vibrationalMode || 0)}</span>
          <span className="field-hex">{formatHex(commandType.vibrationalMode || 0)}</span>
        </div>
        <div className="frame-field">
          <span className="field-label">Pattern:</span>
          <span className="field-value">{decodeIntensityPattern(commandType.intensityPattern || 0)}</span>
          <span className="field-hex">{formatHex(commandType.intensityPattern || 0)}</span>
        </div>
        <div className="frame-field">
          <span className="field-label">Duration:</span>
          <span className="field-value">{commandType.duration || 1} frame(s)</span>
          <span className="field-hex">{formatHex(commandType.duration || 1)}</span>
        </div>
        <div className="frame-field">
          <span className="field-label">Sequencing:</span>
          <span className="field-value">Frame {currentFrameNum} of {totalFrames}</span>
          <span className="field-hex">{formatHex(commandType.sequencing || 0)}</span>
        </div>
      </div>

      <div className="frame-section">
        <div className="section-title">DATA PAYLOAD</div>
        <div className="frame-field">
          <span className="field-label">FSK Pattern:</span>
          <span className="field-value">{fskPattern}</span>
          <span className="field-hex">{(fskRatio * 100).toFixed(1)}% high</span>
        </div>
        <div className="frame-field">
          <span className="field-label">Avg Freq Mod:</span>
          <span className="field-value">{avgFreqMod.toFixed(3)}</span>
          <span className="field-hex">{gocs.intensity ? `(${(gocs.intensity * 100).toFixed(0)}% intensity)` : ''}</span>
        </div>
        <div className="frame-field">
          <span className="field-label">Avg Amp Mod:</span>
          <span className="field-value">{avgAmpMod.toFixed(3)}</span>
        </div>
        <div className="frame-field">
          <span className="field-label">Symbol Rate:</span>
          <span className="field-value">16 Hz</span>
          <span className="field-hex">128 symbols</span>
        </div>
      </div>

      <div className="frame-section">
        <div className="section-title">FRAME TIMING</div>
        <div className="frame-field">
          <span className="field-label">Frame #:</span>
          <span className="field-value">{gocs.frameNumber}</span>
        </div>
        <div className="frame-field">
          <span className="field-label">Time Remaining:</span>
          <span className="field-value">{gocs.frameTimeRemaining.toFixed(2)}s / 8.00s</span>
        </div>
        <div className="frame-field">
          <span className="field-label">Progress:</span>
          <span className="field-value">{((8 - gocs.frameTimeRemaining) / 8 * 100).toFixed(1)}%</span>
        </div>
      </div>
    </div>
  );
};

export default FrameStructureViewer;
