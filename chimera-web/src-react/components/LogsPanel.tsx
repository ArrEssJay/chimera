import React, { useEffect, useState, useRef } from 'react';
import { getWASMDSPService } from '../services/WASMDSPService';

const LogsPanel: React.FC = () => {
  const [logs, setLogs] = useState<string[]>([]);
  const [autoScroll, setAutoScroll] = useState(true);
  const logsEndRef = useRef<HTMLDivElement>(null);
  const service = getWASMDSPService();

  useEffect(() => {
    service.subscribeToLogs('logs-panel', (newLogs) => {
      setLogs(newLogs);
    });

    return () => {
      service.unsubscribeFromLogs('logs-panel');
    };
  }, [service]);

  useEffect(() => {
    if (autoScroll && logsEndRef.current) {
      logsEndRef.current.scrollIntoView({ behavior: 'smooth' });
    }
  }, [logs, autoScroll]);

  const handleClear = () => {
    service.clearLogs();
  };

  return (
    <div className="logs-panel">
      <div className="logs-header">
        <h3>System Logs</h3>
        <div className="logs-controls">
          <label className="logs-checkbox">
            <input
              type="checkbox"
              checked={autoScroll}
              onChange={(e) => setAutoScroll(e.target.checked)}
            />
            Auto-scroll
          </label>
          <button className="btn-small" onClick={handleClear}>
            Clear
          </button>
        </div>
      </div>
      <div className="logs-content">
        {logs.length === 0 ? (
          <div className="logs-empty">No logs yet. Start DSP to see activity.</div>
        ) : (
          logs.map((log, index) => (
            <div key={index} className="log-entry">
              {log}
            </div>
          ))
        )}
        <div ref={logsEndRef} />
      </div>
    </div>
  );
};

export default LogsPanel;
