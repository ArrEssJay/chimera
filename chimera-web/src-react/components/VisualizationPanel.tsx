/**
 * VisualizationPanel - Grid layout for real-time plots
 * 
 * Displays constellation diagram and spectrum analyzer with
 * dynamic updates from streaming DSP engine.
 */

import React, { useEffect, useState } from 'react';
import ConstellationPlot from './plots/ConstellationPlot';
import SpectrumPlot from './plots/SpectrumPlot';
import { getWASMDSPService, StreamData } from '../services/WASMDSPService';

export interface VisualizationPanelProps {
  isProcessing: boolean;
  showPlots?: boolean;
  streamData: { decodedText: string; ber: number; constellationI: Float32Array; constellationQ: Float32Array; fftMagnitude: Float32Array } | null;
  logs: string[];
}

const VisualizationPanel: React.FC<VisualizationPanelProps> = ({ 
  isProcessing, 
  showPlots = true,
  logs 
}) => {
  const [liveStreamData, setLiveStreamData] = useState<StreamData | null>(null);
  const [accumulatedText, setAccumulatedText] = useState<string>('');
  
  // Subscribe to streaming data
  useEffect(() => {
    const service = getWASMDSPService();
    const subscriptionId = 'viz-panel';
    
    service.subscribe(subscriptionId, (data) => {
      setLiveStreamData(data);
      if (data.decodedText) {
        setAccumulatedText(prev => prev + data.decodedText);
      }
    });
    
    return () => {
      service.unsubscribe(subscriptionId);
    };
  }, []);
  
  // Reset on stop
  useEffect(() => {
    if (!isProcessing) {
      setAccumulatedText('');
      setLiveStreamData(null);
    }
  }, [isProcessing]);
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

  return (
    <div className="visualization-panel">
      <div className="visualization-grid">
        <div className="visualization-item">
          <div className="visualization-header">
            <h3>Constellation Diagram</h3>
            <span className="visualization-desc">
              {isProcessing ? 'Live QPSK Symbol Scatter' : 'Ready - Start DSP to view data'}
            </span>
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
            <span className="visualization-desc">
              {isProcessing ? 'Live Frequency Domain Analysis' : 'Ready - Start DSP to view data'}
            </span>
          </div>
          <div className="visualization-content">
            <SpectrumPlot
              width={500}
              height={500}
              showGrid={true}
              minDb={-80}
              maxDb={0}
              smoothing={0.8}
            />
          </div>
        </div>
      </div>
      
      {/* Diagnostics Section */}
      <div className="diagnostics-section">
        <div className="diagnostics-grid">
          {/* Signal Quality Metrics */}
          <div className="diagnostic-card">
            <h3>Signal Quality</h3>
            <table className="diagnostic-table">
              <tbody>
                <tr>
                  <td>BER (Post-FEC):</td>
                  <td className="value">{liveStreamData ? liveStreamData.ber.toExponential(2) : '—'}</td>
                </tr>
                <tr>
                  <td>Mean EVM:</td>
                  <td className="value">{liveStreamData ? liveStreamData.meanEvm.toFixed(4) : '—'}</td>
                </tr>
                <tr>
                  <td>Peak EVM:</td>
                  <td className="value">{liveStreamData ? liveStreamData.peakEvm.toFixed(4) : '—'}</td>
                </tr>
                <tr>
                  <td>Timing Error:</td>
                  <td className="value">{liveStreamData ? liveStreamData.timingError.toFixed(4) : '—'}</td>
                </tr>
              </tbody>
            </table>
          </div>
          
          {/* Frame Status */}
          <div className="diagnostic-card">
            <h3>Frame Status</h3>
            <table className="diagnostic-table">
              <tbody>
                <tr>
                  <td>Sync Found:</td>
                  <td className={`value ${liveStreamData?.syncFound ? 'success' : 'error'}`}>
                    {liveStreamData ? (liveStreamData.syncFound ? '✓ Yes' : '✗ No') : '—'}
                  </td>
                </tr>
                <tr>
                  <td>Symbol Count:</td>
                  <td className="value">{liveStreamData ? liveStreamData.symbolCount : '—'}</td>
                </tr>
                <tr>
                  <td>Decoded Bytes:</td>
                  <td className="value">{accumulatedText.length}</td>
                </tr>
              </tbody>
            </table>
          </div>
          
          {/* Decoded Data */}
          <div className="diagnostic-card wide">
            <h3>Decoded Bitstream</h3>
            <div className="decoded-output">
              {accumulatedText || <em className="empty">Waiting for decoded data...</em>}
            </div>
          </div>
          
          {/* Processing Logs */}
          <div className="diagnostic-card wide">
            <h3>Processing Logs</h3>
            <div className="logs-output">
              {logs.length > 0 ? (
                logs.map((log, idx) => (
                  <div key={idx} className="log-line">{log}</div>
                ))
              ) : (
                <em className="empty">No logs yet</em>
              )}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default VisualizationPanel;
