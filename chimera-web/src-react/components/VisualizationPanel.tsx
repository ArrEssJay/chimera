/**
 * VisualizationPanel - Grid layout for real-time plots
 * 
 * Displays constellation diagram and spectrum analyzer with
 * dynamic updates from streaming DSP engine.
 */

import React from 'react';
import ConstellationPlot from './plots/ConstellationPlot';
import SpectrumPlot from './plots/SpectrumPlot';

export interface VisualizationPanelProps {
  isProcessing: boolean;
  showPlots?: boolean;
}

const VisualizationPanel: React.FC<VisualizationPanelProps> = ({ 
  isProcessing, 
  showPlots = true 
}) => {
  if (!showPlots) {
    return (
      <div className="visualization-panel">
        <div className="visualization-empty">
          <h2>Visualizations Hidden</h2>
          <p>Click "Show Plots" to display real-time visualizations</p>
        </div>
      </div>
    );
  }

  if (!isProcessing) {
    return (
      <div className="visualization-panel">
        <div className="visualization-empty">
          <h2>Ready to Process</h2>
          <p>Configure parameters and click "Start DSP" to begin processing</p>
          <div className="visualization-preview">
            <div className="preview-item">
              <div className="preview-icon">📊</div>
              <div className="preview-label">Constellation Diagram</div>
              <div className="preview-desc">View QPSK symbol points in I/Q space</div>
            </div>
            <div className="preview-item">
              <div className="preview-icon">📈</div>
              <div className="preview-label">Spectrum Analyzer</div>
              <div className="preview-desc">Real-time FFT frequency analysis</div>
            </div>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="visualization-panel">
      <div className="visualization-grid">
        <div className="visualization-item">
          <div className="visualization-header">
            <h3>Constellation Diagram</h3>
            <span className="visualization-desc">QPSK Symbol Scatter</span>
          </div>
          <div className="visualization-content">
            <ConstellationPlot
              width={500}
              height={500}
              maxPoints={1000}
              showGrid={true}
              showReference={true}
            />
          </div>
        </div>

        <div className="visualization-item">
          <div className="visualization-header">
            <h3>Spectrum Analyzer</h3>
            <span className="visualization-desc">Frequency Domain Analysis</span>
          </div>
          <div className="visualization-content">
            <SpectrumPlot
              width={700}
              height={400}
              showGrid={true}
              minDb={-80}
              maxDb={0}
              smoothing={0.8}
            />
          </div>
        </div>
      </div>
    </div>
  );
};

export default VisualizationPanel;
