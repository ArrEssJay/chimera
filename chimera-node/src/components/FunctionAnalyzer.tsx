import React from 'react';
import { useAudio } from '../audio/AudioProvider';
import './FunctionAnalyzer.css';

const FunctionAnalyzer: React.FC = () => {
  const { audioState } = useAudio();
  const { currentFrame, gocs } = audioState;

  if (!gocs.currentFunction) {
    return (
      <div className="function-analyzer">
        <div className="analyzer-header">FUNCTION ANALYZER</div>
        <div className="no-function">
          <div>No function active</div>
          <div className="hint">Select a function from GOCS controls to see analysis</div>
        </div>
      </div>
    );
  }

  const frameData = currentFrame as any;
  const targetId = frameData?.targetId || {};
  const commandType = frameData?.commandType || {};

  // Function parameter mapping
  const functionMappings: { [key: string]: any } = {
    'induceCalm': {
      description: 'Reduces cortical activation, promotes parasympathetic response',
      targetParams: {
        brainwave: '8 Hz (Alpha)',
        hemisphere: 'Balanced',
        region: 'Prefrontal',
      },
      commandParams: {
        vibration: 'Breathing (slow oscillation)',
        pattern: 'Smooth Sine',
      }
    },
    'heightenAlertness': {
      description: 'Increases cortical activation, promotes sympathetic response',
      targetParams: {
        brainwave: '20 Hz (Beta)',
        hemisphere: 'Left dominant',
        region: 'Motor cortex',
      },
      commandParams: {
        vibration: 'Longitudinal (rapid)',
        pattern: 'Pulsed',
      }
    },
    'disruptCognition': {
      description: 'Interferes with normal cognitive processing',
      targetParams: {
        brainwave: '6 Hz (Theta)',
        hemisphere: 'Right bias',
        region: 'Prefrontal',
      },
      commandParams: {
        vibration: 'Torsional (chaotic)',
        pattern: 'Chaotic',
      }
    },
    'enhanceFocus': {
      description: 'Increases attention and concentration',
      targetParams: {
        brainwave: '15 Hz (Beta)',
        hemisphere: 'Left bias',
        region: 'Prefrontal',
      },
      commandParams: {
        vibration: 'Longitudinal',
        pattern: 'Smooth Sine',
      }
    },
    'induceRelaxation': {
      description: 'Deep relaxation, similar to meditation',
      targetParams: {
        brainwave: '10 Hz (Alpha)',
        hemisphere: 'Balanced',
        region: 'Prefrontal',
      },
      commandParams: {
        vibration: 'Breathing (slow)',
        pattern: 'Smooth Sine',
      }
    },
  };

  const mapping = functionMappings[gocs.currentFunction] || {
    description: 'Unknown function',
    targetParams: {},
    commandParams: {},
  };

  // Calculate modulation statistics
  const freqMod = frameData?.freqModulation || [];
  const ampMod = frameData?.ampModulation || [];
  
  const avgFreqMod = freqMod.length > 0 
    ? freqMod.reduce((a: number, b: number) => a + Math.abs(b), 0) / freqMod.length 
    : 0;
  
  const maxFreqMod = freqMod.length > 0 
    ? Math.max(...freqMod.map((v: number) => Math.abs(v)))
    : 0;

  const avgAmpMod = ampMod.length > 0
    ? ampMod.reduce((a: number, b: number) => a + b, 0) / ampMod.length
    : 0;

  const ampModRange = ampMod.length > 0
    ? Math.max(...ampMod) - Math.min(...ampMod)
    : 0;

  return (
    <div className="function-analyzer">
      <div className="analyzer-header">
        FUNCTION: {gocs.currentFunction.toUpperCase()}
      </div>

      <div className="function-description">
        {mapping.description}
      </div>

      <div className="analysis-section">
        <div className="section-title">Input Parameters</div>
        <div className="param-grid">
          <div className="param-row">
            <span className="param-name">Intensity:</span>
            <span className="param-value">{((gocs.intensity || 0.5) * 100).toFixed(0)}%</span>
          </div>
          <div className="param-row">
            <span className="param-name">Duration:</span>
            <span className="param-value">{commandType.duration || 1} frame(s)</span>
          </div>
        </div>
      </div>

      <div className="analysis-section">
        <div className="section-title">Target Mapping</div>
        <div className="mapping-grid">
          <div className="mapping-row">
            <span className="mapping-label">Brainwave:</span>
            <span className="mapping-expected">{mapping.targetParams.brainwave || 'N/A'}</span>
            <span className="mapping-arrow">→</span>
            <span className="mapping-actual">{targetId.baselineBrainwave || 0} Hz</span>
          </div>
          <div className="mapping-row">
            <span className="mapping-label">Hemisphere:</span>
            <span className="mapping-expected">{mapping.targetParams.hemisphere || 'N/A'}</span>
            <span className="mapping-arrow">→</span>
            <span className="mapping-actual">
              {targetId.hemisphereBias === 128 ? 'Balanced' : 
               targetId.hemisphereBias < 128 ? 'Left' : 'Right'}
            </span>
          </div>
          <div className="mapping-row">
            <span className="mapping-label">Region:</span>
            <span className="mapping-expected">{mapping.targetParams.region || 'N/A'}</span>
            <span className="mapping-arrow">→</span>
            <span className="mapping-actual">
              {['Auditory', 'Visual', 'Motor', 'Prefrontal'][targetId.corticalRegion - 1] || 'Unknown'}
            </span>
          </div>
        </div>
      </div>

      <div className="analysis-section">
        <div className="section-title">Modulation Output</div>
        <div className="param-grid">
          <div className="param-row">
            <span className="param-name">Avg Freq Mod:</span>
            <span className="param-value">{avgFreqMod.toFixed(3)}</span>
          </div>
          <div className="param-row">
            <span className="param-name">Max Freq Mod:</span>
            <span className="param-value">{maxFreqMod.toFixed(3)}</span>
          </div>
          <div className="param-row">
            <span className="param-name">Avg Amp Mod:</span>
            <span className="param-value">{avgAmpMod.toFixed(3)}</span>
          </div>
          <div className="param-row">
            <span className="param-name">Amp Range:</span>
            <span className="param-value">{ampModRange.toFixed(3)}</span>
          </div>
        </div>
      </div>

      <div className="analysis-section">
        <div className="section-title">Intensity Effect</div>
        <div className="intensity-visualization">
          <div className="intensity-bar">
            <div 
              className="intensity-fill" 
              style={{ width: `${(gocs.intensity || 0.5) * 100}%` }}
            />
          </div>
          <div className="intensity-impact">
            <div className="impact-item">
              <span className="impact-label">Freq Mod Scale:</span>
              <span className="impact-value">
                {((gocs.intensity || 0.5) * maxFreqMod).toFixed(3)} Hz
              </span>
            </div>
            <div className="impact-item">
              <span className="impact-label">Amp Mod Depth:</span>
              <span className="impact-value">
                {((gocs.intensity || 0.5) * ampModRange).toFixed(3)}
              </span>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default FunctionAnalyzer;
