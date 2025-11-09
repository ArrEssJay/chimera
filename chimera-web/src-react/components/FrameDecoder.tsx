import React, { useEffect, useState } from 'react';
import { getWASMDSPService, type FrameData, type StreamData } from '../services/WASMDSPService';

const FrameDecoder: React.FC = () => {
  const [frameData, setFrameData] = useState<FrameData | null>(null);
  const [currentSymbolProgress, setCurrentSymbolProgress] = useState(0);
  const [currentFrameNumber, setCurrentFrameNumber] = useState(0);
  const [lastUpdatedSymbol, setLastUpdatedSymbol] = useState<number | null>(null);
  const [currentSymbols, setCurrentSymbols] = useState<number[]>(new Array(128).fill(0));
  const service = getWASMDSPService();

  // Convert bytes to QPSK symbols (4 symbols per byte, values 0-3)
  const bytesToSymbols = (
    syncData: Uint8Array,
    payloadData: Uint8Array,
    parityData: Uint8Array
  ): number[] => {
    const symbols: number[] = [];
    
    // Process sync (16 symbols = 4 bytes)
    for (let i = 0; i < 4; i++) {
      const byte = syncData[i] || 0;
      symbols.push((byte >> 6) & 0x03);
      symbols.push((byte >> 4) & 0x03);
      symbols.push((byte >> 2) & 0x03);
      symbols.push(byte & 0x03);
    }
    
    // Process payload (96 symbols = 24 bytes for target+command+data)
    for (let i = 0; i < 24; i++) {
      const byte = payloadData[i] || 0;
      symbols.push((byte >> 6) & 0x03);
      symbols.push((byte >> 4) & 0x03);
      symbols.push((byte >> 2) & 0x03);
      symbols.push(byte & 0x03);
    }
    
    // Process parity (16 symbols = 4 bytes)
    for (let i = 0; i < 4; i++) {
      const byte = parityData[i] || 0;
      symbols.push((byte >> 6) & 0x03);
      symbols.push((byte >> 4) & 0x03);
      symbols.push((byte >> 2) & 0x03);
      symbols.push(byte & 0x03);
    }
    
    return symbols;
  };

  useEffect(() => {
    const handleData = (data: StreamData) => {
      if (!data) return;
      if (!data.currentFrameData) return;
      
      const frame = data.currentFrameData;
      const frameNumber = frame.frameNumber;
      const symbolProgress = frame.symbolProgress;
      
      // Convert bytes to symbols (for QPSK: 4 symbols per byte, 2 bits per symbol)
      const newSymbols = bytesToSymbols(
        frame.syncData,
        frame.payloadData,
        frame.parityData
      );
      
      // Check if we've moved to a new frame
      if (frameNumber !== currentFrameNumber) {
        setCurrentFrameNumber(frameNumber);
        setCurrentSymbolProgress(0);
        setLastUpdatedSymbol(null);
        setCurrentSymbols(new Array(128).fill(0));
      } else if (symbolProgress > currentSymbolProgress) {
        // New symbol in current frame - highlight the newly added symbol
        setLastUpdatedSymbol(symbolProgress - 1);
        
        // Clear highlight after animation
        setTimeout(() => setLastUpdatedSymbol(null), 500);
      }
      
      // Update current frame state
      setFrameData(frame);
      setCurrentSymbolProgress(symbolProgress);
      setCurrentSymbols(newSymbols);
    };
    
    service.subscribe('frame-decoder', handleData);
    return () => service.unsubscribe('frame-decoder');
  }, [service, currentFrameNumber, currentSymbolProgress]);

  const getSymbolSection = (symbolIndex: number): 'sync' | 'target' | 'command' | 'payload' | 'parity' => {
    if (symbolIndex < 16) return 'sync';
    if (symbolIndex < 32) return 'target';
    if (symbolIndex < 48) return 'command';
    if (symbolIndex < 112) return 'payload';
    return 'parity';
  };

  return (
    <div className="frame-decoder">
      <h3>Frame Decoder - Real-time Symbol Stream</h3>
      
      {/* Current Frame Progress */}
      <div className="frame-progress">
        <div className="progress-header">
          <span className="progress-label">Frame #{frameData?.frameNumber || '—'}</span>
          <span className="progress-stats">
            {frameData?.symbolProgress || 0}/128 symbols ({Math.floor(((frameData?.symbolProgress || 0) / 128) * 100)}%)
          </span>
        </div>
        <div className="progress-bar">
          <div 
            className="progress-fill"
            style={{ width: `${((frameData?.symbolProgress || 0) / 128) * 100}%` }}
          />
        </div>
      </div>

      {/* Frame Symbol Grid */}
      <div className="symbol-grid">
        <h4>Frame Structure (128 symbols)</h4>
        <div className="grid-container">
          {Array.from({ length: 128 }, (_, i) => {
            const section = getSymbolSection(i);
            const isTransmitted = i < currentSymbolProgress;
            const isJustUpdated = i === lastUpdatedSymbol;
            const symbolValue = currentSymbols[i];
            return (
              <div
                key={i}
                className={`symbol-cell section-${section} ${isTransmitted ? 'transmitted' : 'pending'} ${isJustUpdated ? 'just-updated' : ''}`}
                title={`Symbol ${i} - ${section} - Value: ${symbolValue}`}
              >
                <span className="symbol-value-text">{isTransmitted ? symbolValue : ''}</span>
              </div>
            );
          })}
        </div>
        <div className="grid-legend">
          <span className="legend-item"><span className="section-sync">■</span> Sync (0-15)</span>
          <span className="legend-item"><span className="section-target">■</span> Target (16-31)</span>
          <span className="legend-item"><span className="section-command">■</span> Command (32-47)</span>
          <span className="legend-item"><span className="section-payload">■</span> Payload (48-111)</span>
          <span className="legend-item"><span className="section-parity">■</span> Parity (112-127)</span>
        </div>
      </div>
    </div>
  );
};

export default FrameDecoder;
