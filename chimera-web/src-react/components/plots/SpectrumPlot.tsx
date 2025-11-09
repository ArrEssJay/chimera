/**
 * SpectrumPlot - High-performance real-time FFT spectrum analyzer
 * 
 * Uses Canvas 2D API with requestAnimationFrame for 60fps updates.
 * Displays frequency spectrum with logarithmic magnitude scale.
 */

import React, { useEffect, useRef, useState } from 'react';
import { getWASMDSPService, type StreamData } from '../../services/WASMDSPService';

export interface SpectrumPlotProps {
  width?: number;
  height?: number;
  showGrid?: boolean;
  minDb?: number;
  maxDb?: number;
  smoothing?: number; // 0-1, higher = smoother
}

const SpectrumPlot: React.FC<SpectrumPlotProps> = ({
  width: propWidth = 0,
  height: propHeight = 0,
  showGrid = true,
  minDb = -80,
  maxDb = 0,
  smoothing = 0.8,
}) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const smoothedMagnitudeRef = useRef<Float32Array | null>(null);
  const animationFrameRef = useRef<number | null>(null);
  const latestDataRef = useRef<Float32Array | null>(null);
  const [dimensions, setDimensions] = useState({ width: propWidth || 600, height: propHeight || 300 });
  const [showTx, setShowTx] = useState(false); // Toggle between TX and RX

  // Handle responsive sizing
  useEffect(() => {
    if (propWidth && propHeight) {
      setDimensions({ width: propWidth, height: propHeight });
      return;
    }

    const updateSize = () => {
      if (containerRef.current) {
        const rect = containerRef.current.getBoundingClientRect();
        setDimensions({ width: rect.width, height: rect.height });
      }
    };

    updateSize();
    window.addEventListener('resize', updateSize);
    return () => window.removeEventListener('resize', updateSize);
  }, [propWidth, propHeight]);

  useEffect(() => {
    const dspService = getWASMDSPService();
    const subscriptionId = 'spectrum-plot';

    // Subscribe to DSP data updates
    dspService.subscribe(subscriptionId, (data: StreamData) => {
      // Store latest FFT magnitude data - select TX or RX
      latestDataRef.current = showTx 
        ? data.preChannel.txSpectrumMagnitude 
        : data.postChannel.rxSpectrumMagnitude;
    });

    // Start rendering loop
    const render = () => {
      renderCanvas();
      animationFrameRef.current = requestAnimationFrame(render);
    };
    render();

    // Cleanup
    return () => {
      dspService.unsubscribe(subscriptionId);
      if (animationFrameRef.current) {
        cancelAnimationFrame(animationFrameRef.current);
      }
    };
  }, [smoothing, dimensions, showTx]);

  const renderCanvas = () => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    const { width, height } = dimensions;
    const magnitude = latestDataRef.current;
    
    if (!magnitude || magnitude.length === 0) {
      // Draw empty state
      ctx.fillStyle = '#1a1a1a';
      ctx.fillRect(0, 0, width, height);
      
      // Draw grid even when no data
      if (showGrid) {
        ctx.strokeStyle = '#222222';
        ctx.lineWidth = 1;
        
        // Horizontal lines
        for (let i = 0; i <= 4; i++) {
          const y = (i / 4) * height;
          ctx.beginPath();
          ctx.moveTo(0, y);
          ctx.lineTo(width, y);
          ctx.stroke();
        }
        
        // Vertical lines
        for (let i = 0; i <= 10; i++) {
          const x = (i / 10) * width;
          ctx.beginPath();
          ctx.moveTo(x, 0);
          ctx.lineTo(x, height);
          ctx.stroke();
        }
      }
      
      ctx.fillStyle = '#666666';
      ctx.font = '14px sans-serif';
      ctx.textAlign = 'center';
      ctx.textBaseline = 'middle';
      ctx.fillText('Waiting for data...', width / 2, height / 2 - 10);
      ctx.font = '12px sans-serif';
      ctx.fillStyle = '#555555';
      ctx.fillText('Start DSP processing to see spectrum', width / 2, height / 2 + 15);
      return;
    }

    // Initialize smoothed buffer
    if (!smoothedMagnitudeRef.current || smoothedMagnitudeRef.current.length !== magnitude.length) {
      smoothedMagnitudeRef.current = new Float32Array(magnitude);
    }

    // Apply smoothing
    const smoothed = smoothedMagnitudeRef.current;
    for (let i = 0; i < magnitude.length; i++) {
      smoothed[i] = smoothed[i] * smoothing + magnitude[i] * (1 - smoothing);
    }

    // Clear canvas
    ctx.fillStyle = '#1a1a1a';
    ctx.fillRect(0, 0, width, height);

    // Draw grid
    if (showGrid) {
      ctx.strokeStyle = '#333333';
      ctx.lineWidth = 1;

      // Horizontal grid lines (dB levels)
      const dbRange = maxDb - minDb;
      const dbStep = 10; // 10 dB steps
      
      for (let db = minDb; db <= maxDb; db += dbStep) {
        const y = height - ((db - minDb) / dbRange) * height;
        
        ctx.beginPath();
        ctx.moveTo(0, y);
        ctx.lineTo(width, y);
        ctx.stroke();
        
        // Label
        ctx.fillStyle = '#666666';
        ctx.font = '10px monospace';
        ctx.textAlign = 'right';
        ctx.fillText(`${db} dB`, width - 5, y - 2);
      }

      // Vertical grid lines (frequency)
      const freqStep = width / 10;
      for (let i = 0; i <= 10; i++) {
        const x = i * freqStep;
        
        ctx.strokeStyle = '#222222';
        ctx.beginPath();
        ctx.moveTo(x, 0);
        ctx.lineTo(x, height);
        ctx.stroke();
      }
    }

    // Draw spectrum as a line with peak marker
    const binWidth = width / smoothed.length;
    const dbRange = maxDb - minDb;

    // Find peak for display
    let peakValue = minDb;
    let peakIndex = 0;
    
    for (let i = 0; i < smoothed.length; i++) {
      if (smoothed[i] > peakValue) {
        peakValue = smoothed[i];
        peakIndex = i;
      }
    }

    // Draw spectrum line
    ctx.strokeStyle = showTx ? '#00ff88' : '#4a9eff';
    ctx.lineWidth = 1.5;
    ctx.beginPath();

    for (let i = 0; i < smoothed.length; i++) {
      const db = Math.max(minDb, Math.min(maxDb, smoothed[i])); // Clamp to range
      const normalizedDb = (db - minDb) / dbRange;
      const y = height - normalizedDb * height;
      const x = i * binWidth;

      if (i === 0) {
        ctx.moveTo(x, y);
      } else {
        ctx.lineTo(x, y);
      }
    }

    ctx.stroke();

    // Draw peak marker
    if (peakValue > minDb) {
      const peakX = peakIndex * binWidth;
      const normalizedPeak = (peakValue - minDb) / dbRange;
      const peakY = height - normalizedPeak * height;
      
      // Peak dot
      ctx.fillStyle = '#ff4444';
      ctx.beginPath();
      ctx.arc(peakX, peakY, 3, 0, Math.PI * 2);
      ctx.fill();
      
      // Peak value label
      ctx.fillStyle = '#ff4444';
      ctx.font = '10px monospace';
      ctx.textAlign = 'center';
      ctx.fillText(`${peakValue.toFixed(1)} dB`, peakX, peakY - 8);
    }

    // Draw frequency axis labels
    ctx.fillStyle = '#888888';
    ctx.font = '11px monospace';
    ctx.textAlign = 'center';
    
    const sampleRate = 48000;
    const nyquist = sampleRate / 2;
    
    [0, 0.25, 0.5, 0.75, 1].forEach((frac) => {
      const freq = frac * nyquist;
      const x = frac * width;
      const label = freq >= 1000 ? `${(freq / 1000).toFixed(1)}k` : `${freq.toFixed(0)}`;
      ctx.fillText(label, x, height - 5);
    });
    
    // Draw title
    ctx.fillStyle = '#4a9eff';
    ctx.font = '11px sans-serif';
    ctx.textAlign = 'left';
    ctx.fillText(showTx ? 'TX Spectrum' : 'RX Spectrum', 5, 15);
  };

  return (
    <div ref={containerRef} className="spectrum-plot" style={{ width: '100%', height: '100%', display: 'flex', flexDirection: 'column', position: 'relative' }}>
      <canvas
        ref={canvasRef}
        width={dimensions.width}
        height={dimensions.height}
        style={{
          width: '100%',
          height: '100%',
          border: '1px solid #2a2a2a',
          background: '#1a1a1a',
          cursor: 'pointer',
        }}
        onClick={() => setShowTx(!showTx)}
        title="Click to toggle TX/RX"
      />
      <div className="plot-controls" style={{
        position: 'absolute',
        top: '4px',
        right: '4px',
        display: 'flex',
        gap: '4px',
      }}>
        <button 
          className={`plot-toggle ${showTx ? 'active' : ''}`}
          onClick={() => setShowTx(!showTx)}
          style={{
            padding: '2px 6px',
            fontSize: '10px',
            background: showTx ? '#4a9eff' : '#2a2a2a',
            color: showTx ? '#fff' : '#888',
            border: `1px solid ${showTx ? '#4a9eff' : '#333'}`,
            cursor: 'pointer',
            transition: 'all 0.1s',
            textTransform: 'uppercase',
            fontWeight: 500,
          }}
        >
          {showTx ? 'TX' : 'RX'}
        </button>
      </div>
    </div>
  );
};

export default SpectrumPlot;
