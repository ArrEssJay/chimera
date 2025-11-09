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
  streamData: StreamData | null;
}

const VisualizationPanel: React.FC<VisualizationPanelProps> = ({ 
  isProcessing,
  streamData: externalStreamData
}) => {
  const [liveStreamData, setLiveStreamData] = useState<StreamData | null>(null);
  const service = getWASMDSPService();
  
  // Use external data if provided, otherwise use live subscription
  const streamData = externalStreamData || liveStreamData;
  
  // Subscribe to streaming data
  useEffect(() => {
    const subscriptionId = 'viz-panel';
    
    service.subscribe(subscriptionId, (data) => {
      setLiveStreamData(data);
    });
    
    return () => {
      service.unsubscribe(subscriptionId);
    };
  }, [service]);
  
  // Reset on stop
  useEffect(() => {
    if (!isProcessing) {
      setLiveStreamData(null);
    }
  }, [isProcessing]);

  const formatValue = (value: number | undefined, decimals: number = 2): string => {
    if (value === undefined || value === null) return '—';
    if (Math.abs(value) < 0.001 && value !== 0) return value.toExponential(2);
    return value.toFixed(decimals);
  };

  const formatStatus = (status: string | boolean | undefined): string => {
    if (typeof status === 'boolean') return status ? 'YES' : 'NO';
    return status || '—';
  };

  return (
    <div className="visualization-panel">
      <div className="plot-container">
        <div className="plot-wrapper">
          <ConstellationPlot
            width={0}
            height={0}
            maxPoints={1000}
            showGrid={true}
            showReference={true}
          />
        </div>
        <div className="plot-wrapper">
          <SpectrumPlot
            width={0}
            height={0}
            showGrid={true}
            minDb={-80}
            maxDb={0}
            smoothing={0.8}
          />
        </div>
      </div>
      
      <div className="diagnostics-container">
        {/* Pre-Channel (Transmitter) Diagnostics */}
        <div className="diagnostic-section">
          <h3>TX (Pre-Channel)</h3>
          <table className="diagnostic-table">
            <tbody>
              <tr>
                <td className="diag-label">Frame</td>
                <td className="diag-value">
                  {liveStreamData?.preChannel.frameCount || 0}/{liveStreamData?.preChannel.totalFrames || 1}
                </td>
              </tr>
              <tr>
                <td className="diag-label">Symbols</td>
                <td className="diag-value">{liveStreamData?.preChannel.symbolCount || 0}</td>
              </tr>
              <tr>
                <td className="diag-label">Carrier</td>
                <td className="diag-value">{formatValue(liveStreamData?.preChannel.carrierFreqHz, 0)} Hz</td>
              </tr>
              <tr>
                <td className="diag-label">Rate</td>
                <td className="diag-value">{liveStreamData?.preChannel.symbolRateHz || 0} sym/s</td>
              </tr>
              <tr>
                <td className="diag-label">Modulation</td>
                <td className="diag-value">{liveStreamData?.preChannel.modulationType || 'QPSK'}</td>
              </tr>
              <tr>
                <td className="diag-label">FEC Rate</td>
                <td className="diag-value">{liveStreamData?.preChannel.fecRate || '64/128'}</td>
              </tr>
            </tbody>
          </table>
        </div>

        {/* Post-Channel (Receiver) Diagnostics */}
        <div className="diagnostic-section">
          <h3>RX (Post-Channel)</h3>
          <table className="diagnostic-table">
            <tbody>
              <tr>
                <td className="diag-label">Lock Status</td>
                <td className={`diag-value ${liveStreamData?.postChannel.lockStatus === 'LOCKED' ? 'status-good' : 'status-warn'}`}>
                  {liveStreamData?.postChannel.lockStatus || 'UNLOCKED'}
                </td>
              </tr>
              <tr>
                <td className="diag-label">Sync</td>
                <td className={`diag-value ${liveStreamData?.postChannel.syncStatus ? 'status-good' : 'status-bad'}`}>
                  {formatStatus(liveStreamData?.postChannel.syncStatus)}
                </td>
              </tr>
              <tr>
                <td className="diag-label">Freq Offset</td>
                <td className="diag-value">{formatValue(liveStreamData?.postChannel.frequencyOffsetHz, 1)} Hz</td>
              </tr>
              <tr>
                <td className="diag-label">Phase Offset</td>
                <td className="diag-value">{formatValue(liveStreamData?.postChannel.phaseOffsetRad, 3)} rad</td>
              </tr>
              <tr>
                <td className="diag-label">Timing Error</td>
                <td className="diag-value">
                  {liveStreamData?.postChannel.timingError && liveStreamData.postChannel.timingError.length > 0 
                    ? formatValue(liveStreamData.postChannel.timingError[liveStreamData.postChannel.timingError.length - 1], 4)
                    : '—'}
                </td>
              </tr>
            </tbody>
          </table>
        </div>

        {/* Signal Quality Metrics */}
        <div className="diagnostic-section">
          <h3>Signal Quality</h3>
          <table className="diagnostic-table">
            <tbody>
              <tr>
                <td className="diag-label">EVM</td>
                <td className={`diag-value ${(liveStreamData?.postChannel.evmPercent || 0) < 10 ? 'status-good' : 'status-warn'}`}>
                  {formatValue(liveStreamData?.postChannel.evmPercent, 1)}%
                </td>
              </tr>
              <tr>
                <td className="diag-label">SNR Est</td>
                <td className={`diag-value ${(liveStreamData?.postChannel.snrEstimateDb || 0) > 10 ? 'status-good' : 'status-warn'}`}>
                  {formatValue(liveStreamData?.postChannel.snrEstimateDb, 1)} dB
                </td>
              </tr>
              <tr>
                <td className="diag-label">BER (Inst)</td>
                <td className={`diag-value ${(liveStreamData?.postChannel.berInstantaneous || 0) < 0.01 ? 'status-good' : 'status-bad'}`}>
                  {formatValue(liveStreamData?.postChannel.berInstantaneous, 6)}
                </td>
              </tr>
              <tr>
                <td className="diag-label">BER (Avg)</td>
                <td className={`diag-value ${(liveStreamData?.postChannel.berAverage || 0) < 0.01 ? 'status-good' : 'status-warn'}`}>
                  {formatValue(liveStreamData?.postChannel.berAverage, 6)}
                </td>
              </tr>
            </tbody>
          </table>
        </div>

        {/* Frame Layout */}
        <div className="diagnostic-section">
          <h3>Frame Layout</h3>
          <table className="diagnostic-table">
            <tbody>
              <tr>
                <td className="diag-label">Sync</td>
                <td className="diag-value">{liveStreamData?.preChannel.frameLayout?.syncBytes || 0} B</td>
              </tr>
              <tr>
                <td className="diag-label">Data</td>
                <td className="diag-value">{liveStreamData?.preChannel.frameLayout?.dataBytes || 0} B</td>
              </tr>
              <tr>
                <td className="diag-label">Parity</td>
                <td className="diag-value">{liveStreamData?.preChannel.frameLayout?.parityBytes || 0} B</td>
              </tr>
              <tr>
                <td className="diag-label">Total</td>
                <td className="diag-value">{liveStreamData?.preChannel.frameLayout?.totalBytes || 0} B</td>
              </tr>
            </tbody>
          </table>
        </div>

        {/* Decoder Performance */}
        <div className="diagnostic-section">
          <h3>Decoder</h3>
          <table className="diagnostic-table">
            <tbody>
              <tr>
                <td className="diag-label">Frames</td>
                <td className="diag-value">{liveStreamData?.framesProcessed || 0}</td>
              </tr>
              <tr>
                <td className="diag-label">Symbols</td>
                <td className="diag-value">{liveStreamData?.symbolsDecoded || 0}</td>
              </tr>
              <tr>
                <td className="diag-label">FEC Fixes</td>
                <td className="diag-value">{liveStreamData?.fecCorrections || 0}</td>
              </tr>
              <tr>
                <td className="diag-label">Output</td>
                <td className="diag-value">{streamData?.decodedText ? `${streamData.decodedText.length} bytes` : '—'}</td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
      
      {/* Frame Decoder moved to sidebar */}
    </div>
  );
};

export default VisualizationPanel;
