import React, { useEffect, useRef } from 'react';
import { FSKState } from '../types';

interface FSKIndicatorProps {
  fskState?: FSKState;
}

export const FSKIndicator: React.FC<FSKIndicatorProps> = ({ fskState }) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    if (!fskState || !canvasRef.current) return;

    const canvas = canvasRef.current;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    // Clear canvas
    ctx.fillStyle = 'rgba(0, 0, 0, 0.3)';
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    // Draw FSK bit timeline
    const bitWidth = canvas.width / fskState.bit_history.length;
    const centerIdx = Math.floor(fskState.bit_history.length / 2);

    fskState.bit_history.forEach((bit, idx) => {
      const x = idx * bitWidth;
      const isCurrent = idx === centerIdx;
      
      // Color based on bit value
      ctx.fillStyle = bit === 1 ? '#ff6b6b' : '#4ecdc4';
      ctx.globalAlpha = isCurrent ? 1.0 : 0.5;
      
      // Draw bit as bar
      const height = canvas.height * 0.6;
      const y = canvas.height / 2 - height / 2;
      ctx.fillRect(x, y, bitWidth - 2, height);
      
      // Draw bit value
      if (isCurrent) {
        ctx.globalAlpha = 1.0;
        ctx.fillStyle = '#ffffff';
        ctx.font = '14px monospace';
        ctx.textAlign = 'center';
        ctx.fillText(bit.toString(), x + bitWidth / 2, canvas.height - 10);
      }
    });

    ctx.globalAlpha = 1.0;
  }, [fskState]);

  if (!fskState) {
    return (
      <div className="fsk-indicator">
        <h3>FSK Layer</h3>
        <div className="fsk-status">Waiting for data...</div>
      </div>
    );
  }

  const { 
    current_frequency_hz, 
    frequency_deviation_hz, 
    current_bit, 
    bit_rate_hz 
  } = fskState;

  // Color based on bit value
  const freqColor = current_bit === 1 ? '#ff6b6b' : '#4ecdc4';
  
  return (
    <div className="fsk-indicator">
      <h3>Nested FSK Layer ({bit_rate_hz} bit/s)</h3>
      
      <div className="fsk-frequency">
        <div className="freq-display" style={{ color: freqColor }}>
          {current_frequency_hz.toFixed(1)} Hz
        </div>
        <div className="freq-deviation">
          Δf: {frequency_deviation_hz > 0 ? '+' : ''}{frequency_deviation_hz.toFixed(1)} Hz
        </div>
      </div>
      
      <div className="fsk-bits">
        <div className="bit-label">Bit Stream:</div>
        <div className="bit-stream">
          {fskState.bit_history.map((bit, idx) => {
            const isCurrent = idx === Math.floor(fskState.bit_history.length / 2);
            return (
              <span 
                key={idx}
                className={`fsk-bit ${isCurrent ? 'current' : ''}`}
                style={{ 
                  color: bit === 1 ? '#ff6b6b' : '#4ecdc4',
                  fontWeight: isCurrent ? 'bold' : 'normal'
                }}
              >
                {bit}
              </span>
            );
          })}
        </div>
      </div>
      
      <div className="fsk-visualization">
        <canvas ref={canvasRef} width={300} height={60}></canvas>
      </div>
      
      <div className="fsk-info">
        <span>Rate: {bit_rate_hz} Hz</span>
        <span>•</span>
        <span>{fskState.symbols_per_bit} QPSK symbols/bit</span>
      </div>
    </div>
  );
};
