import React, { useState } from 'react';
import { useAudio } from '../audio/AudioProvider';

const GOCS_FUNCTIONS = [
  { 
    value: 'induceCalm', 
    label: 'Induce Calm',
    description: 'Theta entrainment (6 Hz) - smooth breathing patterns',
    params: { phase: '6Hz sine', freq: '4Hz sine', amp: '2Hz sine', fsk: '0.5Hz alt' }
  },
  { 
    value: 'heightenAlertness', 
    label: 'Heighten Alertness',
    description: 'Beta/Gamma (40 Hz) - sharp irregular patterns',
    params: { phase: '40Hz saw', freq: '20Hz saw', amp: '10Hz sine', fsk: '8Hz burst' }
  },
  { 
    value: 'disruptCognition', 
    label: 'Disrupt Cognition',
    description: 'Chaotic mixed-freq - aperiodic interference',
    params: { phase: 'chaos', freq: 'random', amp: 'noise', fsk: 'jitter' }
  },
  { 
    value: 'suppressMotorFunction', 
    label: 'Suppress Motor Function',
    description: 'Low freq heavy modulation - motor cortex targeting',
    params: { phase: '2Hz tri', freq: '1Hz sine', amp: '3Hz square', fsk: 'hold' }
  },
  { 
    value: 'enforceCognitiveStillness', 
    label: 'Enforce Cognitive Stillness',
    description: 'Sub-delta (0.5 Hz) - deep meditative state',
    params: { phase: '0.5Hz sine', freq: '0.25Hz sine', amp: '1Hz sine', fsk: 'idle' }
  },
];

const GOCSControls: React.FC = () => {
  const { audioState, setGOCSFunction } = useAudio();
  const [selectedFunction, setSelectedFunction] = useState(audioState.gocs.currentFunction);
  const [intensity, setIntensity] = useState(audioState.gocs.intensity);
  const [duration, setDuration] = useState(audioState.gocs.duration);

  const currentFunc = GOCS_FUNCTIONS.find(f => f.value === selectedFunction);

  const handleFunctionChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    const func = e.target.value;
    setSelectedFunction(func);
    setGOCSFunction(func, intensity, duration);
  };

  const handleIntensityChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const value = parseFloat(e.target.value);
    setIntensity(value);
    setGOCSFunction(selectedFunction, value, duration);
  };

  const handleDurationChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const value = parseInt(e.target.value);
    setDuration(value);
    setGOCSFunction(selectedFunction, intensity, value);
  };

  return (
    <div className="gocs-controls">
      <div className="control-group">
        <label htmlFor="gocs-function">Function / Effect</label>
        <select 
          id="gocs-function"
          value={selectedFunction}
          onChange={handleFunctionChange}
        >
          {GOCS_FUNCTIONS.map(func => (
            <option key={func.value} value={func.value}>
              {func.label}
            </option>
          ))}
        </select>
      </div>

      {currentFunc && (
        <div className="function-details">
          <p className="function-desc">{currentFunc.description}</p>
          <div className="param-preview">
            <div className="param-preview-item">
              <span className="param-name">Phase:</span>
              <span className="param-spec">{currentFunc.params.phase}</span>
            </div>
            <div className="param-preview-item">
              <span className="param-name">Freq:</span>
              <span className="param-spec">{currentFunc.params.freq}</span>
            </div>
            <div className="param-preview-item">
              <span className="param-name">Amp:</span>
              <span className="param-spec">{currentFunc.params.amp}</span>
            </div>
            <div className="param-preview-item">
              <span className="param-name">FSK:</span>
              <span className="param-spec">{currentFunc.params.fsk}</span>
            </div>
          </div>
        </div>
      )}

      <div className="control-group">
        <label htmlFor="gocs-intensity">
          Intensity
        </label>
        <div className="slider-container">
          <input
            id="gocs-intensity"
            type="range"
            min="0"
            max="1"
            step="0.01"
            value={intensity}
            onChange={handleIntensityChange}
          />
          <span className="slider-value">{(intensity * 100).toFixed(0)}%</span>
        </div>
      </div>

      <div className="control-group">
        <label htmlFor="gocs-duration">
          Duration (frames)
        </label>
        <input
          id="gocs-duration"
          type="number"
          min="1"
          max="100"
          value={duration}
          onChange={handleDurationChange}
        />
      </div>

      <div className="info-display">
        <div className="info-item">
          <span className="info-label">Current Frame:</span>
          <span className="info-value">{audioState.gocs.frameNumber}</span>
        </div>
        <div className="info-item">
          <span className="info-label">Time Remaining:</span>
          <span className="info-value">{audioState.gocs.frameTimeRemaining.toFixed(2)}s</span>
        </div>
      </div>
    </div>
  );
};

export default GOCSControls;
