import React, { useState } from 'react';

const PATTERN_LIBRARY = [
  { category: 'Coherence & Entrainment', patterns: [
    { id: 'COH.ThetaCalm', name: 'Theta Calm', description: 'Gentle theta entrainment' },
    { id: 'COH.AlphaFocus', name: 'Alpha Focus', description: 'Sustained alpha coherence' },
    { id: 'COH.DeepDrift', name: 'Deep Drift', description: 'Delta wave induction' },
  ]},
  { category: 'Cognitive & Perceptual', patterns: [
    { id: 'COG.PerceptualShift', name: 'Perceptual Shift', description: 'Alters sensory processing' },
    { id: 'COG.AttentionNarrow', name: 'Attention Narrow', description: 'Focused attention' },
    { id: 'COG.TimeDistortion', name: 'Time Distortion', description: 'Temporal perception shift' },
  ]},
  { category: 'Disruption & Denial', patterns: [
    { id: 'DIS.CognitiveScramble', name: 'Cognitive Scramble', description: 'Thought disruption' },
    { id: 'DIS.MotorInterrupt', name: 'Motor Interrupt', description: 'Motor function disruption' },
    { id: 'DIS.MemoryFog', name: 'Memory Fog', description: 'Short-term memory interference' },
  ]},
  { category: 'Utility & Calibration', patterns: [
    { id: 'UTIL.Idle', name: 'Idle', description: 'Baseline idle state' },
    { id: 'UTIL.Calibration', name: 'Calibration', description: 'System calibration pattern' },
  ]},
];

const PatternSelector: React.FC = () => {
  const [selectedPattern, setSelectedPattern] = useState('');

  const handlePatternChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    const patternId = e.target.value;
    setSelectedPattern(patternId);
    console.log('Selected pattern:', patternId);
    // TODO: Apply pattern to GOCS
  };

  return (
    <div className="pattern-selector">
      <div className="control-group">
        <label htmlFor="pattern-select">Select Pattern</label>
        <select 
          id="pattern-select"
          value={selectedPattern}
          onChange={handlePatternChange}
        >
          <option value="">-- Select a pattern --</option>
          {PATTERN_LIBRARY.map(category => (
            <optgroup key={category.category} label={category.category}>
              {category.patterns.map(pattern => (
                <option key={pattern.id} value={pattern.id}>
                  {pattern.name}
                </option>
              ))}
            </optgroup>
          ))}
        </select>
      </div>

      {selectedPattern && (
        <div className="pattern-info">
          <h4>Pattern Details</h4>
          <p>
            {PATTERN_LIBRARY
              .flatMap(cat => cat.patterns)
              .find(p => p.id === selectedPattern)?.description}
          </p>
        </div>
      )}
    </div>
  );
};

export default PatternSelector;
