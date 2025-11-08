/**
 * MetricsPanel - Display processing metrics and logs
 * 
 * Shows BER statistics, frame information, recovered message,
 * and processing logs from the backend.
 */

import React, { useEffect, useRef, useState } from 'react';
import { SimulationReport, DemodulationDiagnostics } from '../types';
import { getWASMDSPService, StreamData } from '../services/WASMDSPService';

export interface MetricsPanelProps {
  report: SimulationReport | null;
  diagnostics: DemodulationDiagnostics | null;
  logs: string[];
  isProcessing: boolean;
  decodedBitstream?: string;
  ber?: number;
}

const MetricsPanel: React.FC<MetricsPanelProps> = ({ 
  report, 
  diagnostics, 
  logs, 
  isProcessing,
  decodedBitstream: propDecodedBitstream,
  ber: propBer 
}) => {
  const logsEndRef = useRef<HTMLDivElement>(null);
  const [streamData, setStreamData] = useState<StreamData | null>(null);
  const [accumulatedText, setAccumulatedText] = useState<string>('');  
  
  // Subscribe to streaming data
  useEffect(() => {
    const service = getWASMDSPService();
    const subscriptionId = 'metrics-panel';
    
    service.subscribe(subscriptionId, (data) => {
      setStreamData(data);
      if (data.decodedText) {
        setAccumulatedText(prev => prev + data.decodedText);
      }
    });
    
    return () => {
      service.unsubscribe(subscriptionId);
    };
  }, []);
  
  // Reset accumulated text when processing stops
  useEffect(() => {
    if (!isProcessing) {
      setAccumulatedText('');
      setStreamData(null);
    }
  }, [isProcessing]);
  
  const decodedBitstream = accumulatedText || propDecodedBitstream;
  const ber = streamData?.ber ?? propBer;

  // Auto-scroll logs to bottom
  useEffect(() => {
    logsEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [logs]);

  const formatBER = (ber: number): string => {
    if (ber === 0) return '0';
    if (ber < 0.0001) return ber.toExponential(2);
    return ber.toFixed(6);
  };

  const formatNumber = (num: number): string => {
    return num.toLocaleString();
  };

  return (
    <div className="metrics-panel">
      <h2>Metrics</h2>

      {/* Processing Status */}
      <div className="metrics-section">
        <div className={`processing-status ${isProcessing ? 'active' : 'idle'}`}>
          {isProcessing ? (
            <>
              <span className="status-spinner"></span>
              <span>Processing...</span>
            </>
          ) : (
            <>
              <span className="status-dot"></span>
              <span>Ready</span>
            </>
          )}
        </div>
      </div>

      {/* Streaming Data */}
      {isProcessing && (
        <>
          <div className="metrics-section">
            <h3>Live Decoded Data</h3>
            <div className="recovered-message">
              {decodedBitstream || <em className="empty-message">Waiting for data...</em>}
            </div>
            {decodedBitstream && (
              <div className="metric-detail">
                {decodedBitstream.length} characters decoded
              </div>
            )}
          </div>

          {ber !== undefined && (
            <div className="metrics-section">
              <h3>Current BER</h3>
              <div className="metric-card post-fec">
                <div className="metric-value">{formatBER(ber)}</div>
                <div className="metric-detail">Real-time bit error rate</div>
              </div>
            </div>
          )}
          
          {streamData && (
            <div className="metrics-section">
              <h3>Signal Quality</h3>
              <div className="info-grid">
                <div className="info-row">
                  <span className="info-label">Constellation Points:</span>
                  <span className="info-value">{streamData.constellationI.length}</span>
                </div>
                <div className="info-row">
                  <span className="info-label">FFT Bins:</span>
                  <span className="info-value">{streamData.fftMagnitude.length}</span>
                </div>
              </div>
            </div>
          )}
        </>
      )}

      {/* BER Statistics */}
      {report && (
        <>
          <div className="metrics-section">
            <h3>Bit Error Rate</h3>
            <div className="metrics-grid">
              <div className="metric-card pre-fec">
                <div className="metric-label">Pre-FEC BER</div>
                <div className="metric-value">{formatBER(report.pre_fec_ber)}</div>
                <div className="metric-detail">
                  {formatNumber(report.pre_fec_errors)} / {formatNumber(report.total_bits)} errors
                </div>
              </div>
              <div className="metric-card post-fec">
                <div className="metric-label">Post-FEC BER</div>
                <div className="metric-value">{formatBER(report.post_fec_ber)}</div>
                <div className="metric-detail">
                  {formatNumber(report.post_fec_errors)} / {formatNumber(report.total_bits)} errors
                </div>
              </div>
            </div>
            
            {/* FEC Improvement */}
            {report.pre_fec_ber > 0 && (
              <div className="metric-improvement">
                FEC Improvement:{' '}
                {((1 - report.post_fec_ber / report.pre_fec_ber) * 100).toFixed(1)}%
              </div>
            )}
          </div>

          {/* Sync Status */}
          <div className="metrics-section">
            <h3>Synchronization</h3>
            <div className="info-grid">
              <div className="info-row">
                <span className="info-label">Sync Found:</span>
                <span className={`info-value ${report.sync_found ? 'success' : 'error'}`}>
                  {report.sync_found ? '✓ Yes' : '✗ No'}
                </span>
              </div>
              {report.sync_found && (
                <div className="info-row">
                  <span className="info-label">Sync Position:</span>
                  <span className="info-value">{report.sync_position} bits</span>
                </div>
              )}
            </div>
          </div>

          {/* Recovered Message */}
          <div className="metrics-section">
            <h3>Recovered Message</h3>
            <div className="recovered-message">
              {report.recovered_message || <em className="empty-message">No message recovered</em>}
            </div>
          </div>
        </>
      )}

      {/* Symbol Diagnostics */}
      {diagnostics && (
        <div className="metrics-section">
          <h3>Symbol Quality</h3>
          <div className="info-grid">
            <div className="info-row">
              <span className="info-label">Mean EVM:</span>
              <span className="info-value">{diagnostics.mean_evm.toFixed(3)}</span>
            </div>
            <div className="info-row">
              <span className="info-label">Mean Distance:</span>
              <span className="info-value">{diagnostics.mean_distance.toFixed(3)}</span>
            </div>
            <div className="info-row">
              <span className="info-label">Symbols:</span>
              <span className="info-value">{formatNumber(diagnostics.symbol_decisions.length)}</span>
            </div>
          </div>
        </div>
      )}

      {/* Processing Logs */}
      <div className="metrics-section logs-section">
        <h3>Processing Logs</h3>
        <div className="logs-container">
          {logs.length === 0 ? (
            <div className="logs-empty">No logs yet</div>
          ) : (
            <>
              {logs.map((log, index) => (
                <div key={index} className="log-entry">
                  <span className="log-timestamp">[{new Date().toLocaleTimeString()}]</span>
                  <span className="log-message">{log}</span>
                </div>
              ))}
              <div ref={logsEndRef} />
            </>
          )}
        </div>
      </div>

      {/* Empty State */}
      {!report && !isProcessing && (
        <div className="metrics-empty">
          <p>Configure parameters and start processing to view metrics</p>
        </div>
      )}
    </div>
  );
};

export default MetricsPanel;
