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
  width = 600,
  height = 300,
  showGrid = true,
  minDb = -80,
  maxDb = 0,
  smoothing = 0.8,
}) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const smoothedMagnitudeRef = useRef<Float32Array | null>(null);
  const animationFrameRef = useRef<number | null>(null);
  const latestDataRef = useRef<Float32Array | null>(null);
  const [peakFreq, setPeakFreq] = useState(0);

  useEffect(() => {
    const dspService = getWASMDSPService();
    const subscriptionId = 'spectrum-plot';

    // Subscribe to DSP data updates
    dspService.subscribe(subscriptionId, (data: StreamData) => {
      // Store latest FFT magnitude data
      latestDataRef.current = data.fftMagnitude;
      
      // Find peak frequency
      if (data.fftMagnitude.length > 0) {
        let maxVal = -Infinity;
        let maxIdx = 0;
        for (let i = 0; i < data.fftMagnitude.length; i++) {
          if (data.fftMagnitude[i] > maxVal) {
            maxVal = data.fftMagnitude[i];
            maxIdx = i;
          }
        }
        
        // Convert bin index to frequency (assuming 48kHz sample rate)
        const sampleRate = 48000;
        const freq = (maxIdx * sampleRate) / (data.fftMagnitude.length * 2);
        setPeakFreq(Math.round(freq));
      }
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
  }, [smoothing]);

  const renderCanvas = () => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    const magnitude = latestDataRef.current;
    if (!magnitude || magnitude.length === 0) {
      // Draw empty state
      ctx.fillStyle = '#0a0a0a';
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

    // Draw spectrum
    const binWidth = width / smoothed.length;
    const dbRange = maxDb - minDb;

    // Create gradient for spectrum
    const gradient = ctx.createLinearGradient(0, 0, 0, height);
    gradient.addColorStop(0, '#ff0000');
    gradient.addColorStop(0.25, '#ff8800');
    gradient.addColorStop(0.5, '#ffff00');
    gradient.addColorStop(0.75, '#00ff00');
    gradient.addColorStop(1, '#0000ff');

    ctx.fillStyle = gradient;

    // Draw spectrum bars
    ctx.beginPath();
    ctx.moveTo(0, height);

    for (let i = 0; i < smoothed.length; i++) {
      const db = smoothed[i];
      const normalizedDb = Math.max(0, Math.min(1, (db - minDb) / dbRange));
      const barHeight = normalizedDb * height;
      const x = i * binWidth;
      const y = height - barHeight;

      if (i === 0) {
        ctx.moveTo(x, y);
      } else {
        ctx.lineTo(x, y);
      }
    }

    ctx.lineTo(width, height);
    ctx.lineTo(0, height);
    ctx.closePath();
    ctx.fill();

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
  };

  return (
    <div className="spectrum-plot">
      <canvas
        ref={canvasRef}
        width={width}
        height={height}
        style={{
          border: '1px solid #333',
          borderRadius: '4px',
          background: '#1a1a1a',
        }}
      />
      <div style={{ marginTop: '8px', color: '#888', fontSize: '12px' }}>
        Peak: {peakFreq} Hz | Smoothing: {(smoothing * 100).toFixed(0)}%
      </div>
    </div>
  );
};

export default SpectrumPlot;
