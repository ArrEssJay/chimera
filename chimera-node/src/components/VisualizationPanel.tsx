import React from 'react';
import Oscilloscope from './Oscilloscope';
import SpectrumAnalyzer from './SpectrumAnalyzer';
import ParameterMonitor from './ParameterMonitor';
import './VisualizationPanel.css';

const VisualizationPanel: React.FC = () => {
  return (
    <div className="visualization-panel">
      <div className="viz-section oscilloscope-section">
        <h3>Oscilloscope</h3>
        <Oscilloscope />
      </div>
      
      <div className="viz-section spectrum-section">
        <h3>Spectrum Analyzer</h3>
        <SpectrumAnalyzer />
      </div>
      
      <div className="viz-section parameter-section">
        <h3>Oscillator Parameters</h3>
        <ParameterMonitor />
      </div>
    </div>
  );
};

export default VisualizationPanel;
