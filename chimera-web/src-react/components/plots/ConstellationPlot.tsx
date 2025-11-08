/**
 * ConstellationPlot - High-performance real-time constellation diagram
 * 
 * Uses Canvas 2D API with requestAnimationFrame for 60fps updates.
 * Displays I/Q symbol points with color-coded decision boundaries.
 */

import React, { useEffect, useRef, useState } from 'react';
import { getWASMDSPService, type StreamData } from '../../services/WASMDSPService';

export interface ConstellationPlotProps {
  width?: number;
  height?: number;
  maxPoints?: number;
  showGrid?: boolean;
  showReference?: boolean;
}

const ConstellationPlot: React.FC<ConstellationPlotProps> = ({
  width = 400,
  height = 400,
  maxPoints = 1000,
  showGrid = true,
  showReference = true,
}) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const dataBufferRef = useRef<{ i: number[]; q: number[] }>({ i: [], q: [] });
  const animationFrameRef = useRef<number | null>(null);
  const [pointCount, setPointCount] = useState(0);

  useEffect(() => {
    const dspService = getWASMDSPService();
    const subscriptionId = 'constellation-plot';

    // Subscribe to DSP data updates
    dspService.subscribe(subscriptionId, (data: StreamData) => {
      // Update buffer with new constellation points
      const buffer = dataBufferRef.current;
      
      // Add new points
      for (let i = 0; i < data.constellationI.length; i++) {
        buffer.i.push(data.constellationI[i]);
        buffer.q.push(data.constellationQ[i]);
      }

      // Limit buffer size
      if (buffer.i.length > maxPoints) {
        const excess = buffer.i.length - maxPoints;
        buffer.i.splice(0, excess);
        buffer.q.splice(0, excess);
      }

      setPointCount(buffer.i.length);
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
  }, [maxPoints]);

  const renderCanvas = () => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    const buffer = dataBufferRef.current;
    const centerX = width / 2;
    const centerY = height / 2;
    const scale = Math.min(width, height) / 4; // Scale to fit +/-2 range

    // Clear canvas
    ctx.fillStyle = '#0a0a0a';
    ctx.fillRect(0, 0, width, height);

    // Draw grid
    if (showGrid) {
      ctx.strokeStyle = '#333333';
      ctx.lineWidth = 1;

      // Horizontal and vertical axes
      ctx.beginPath();
      ctx.moveTo(0, centerY);
      ctx.lineTo(width, centerY);
      ctx.moveTo(centerX, 0);
      ctx.lineTo(centerX, height);
      ctx.stroke();

      // Grid lines at +/-1, +/-2
      ctx.strokeStyle = '#222222';
      [-2, -1, 1, 2].forEach((val) => {
        const x = centerX + val * scale;
        const y = centerY - val * scale;
        
        ctx.beginPath();
        ctx.moveTo(x, 0);
        ctx.lineTo(x, height);
        ctx.stroke();
        
        ctx.beginPath();
        ctx.moveTo(0, y);
        ctx.lineTo(width, y);
        ctx.stroke();
      });
    }

    // Draw reference constellation points for QPSK
    if (showReference) {
      const refPoints = [
        { i: 1, q: 1 },   // 00
        { i: -1, q: 1 },  // 01
        { i: -1, q: -1 }, // 11
        { i: 1, q: -1 },  // 10
      ];

      ctx.fillStyle = '#00ff0050';
      ctx.strokeStyle = '#00ff00';
      ctx.lineWidth = 2;

      refPoints.forEach(({ i, q }) => {
        const x = centerX + i * scale;
        const y = centerY - q * scale;
        
        ctx.beginPath();
        ctx.arc(x, y, 8, 0, 2 * Math.PI);
        ctx.fill();
        ctx.stroke();
      });
    }

    // Draw constellation points
    if (buffer.i.length > 0) {
      // Use gradient colors based on recency
      const pointsToRender = Math.min(buffer.i.length, maxPoints);
      
      for (let i = 0; i < pointsToRender; i++) {
        const iVal = buffer.i[i];
        const qVal = buffer.q[i];
        
        // Scale to canvas coordinates
        const x = centerX + iVal * scale;
        const y = centerY - qVal * scale; // Invert Y axis
        
        // Fade older points
        const age = (pointsToRender - i) / pointsToRender;
        const alpha = Math.max(0.1, age * 0.8);
        
        // Color based on quadrant
        let color = '#ffffff';
        if (iVal > 0 && qVal > 0) color = '#00ffff'; // Q1: cyan
        else if (iVal < 0 && qVal > 0) color = '#ffff00'; // Q2: yellow
        else if (iVal < 0 && qVal < 0) color = '#ff00ff'; // Q3: magenta
        else if (iVal > 0 && qVal < 0) color = '#00ff00'; // Q4: green
        
        ctx.fillStyle = `${color}${Math.floor(alpha * 255).toString(16).padStart(2, '0')}`;
        
        // Draw point
        ctx.beginPath();
        ctx.arc(x, y, 2, 0, 2 * Math.PI);
        ctx.fill();
      }
    } else {
      // Show "waiting for data" message when no points
      ctx.fillStyle = '#666666';
      ctx.font = '14px sans-serif';
      ctx.textAlign = 'center';
      ctx.textBaseline = 'middle';
      ctx.fillText('Waiting for data...', centerX, centerY - 20);
      ctx.font = '12px sans-serif';
      ctx.fillStyle = '#555555';
      ctx.fillText('Start DSP processing to see constellation', centerX, centerY + 10);
    }

    // Draw labels
    ctx.fillStyle = '#888888';
    ctx.font = '12px monospace';
    ctx.fillText('I', width - 15, centerY + 20);
    ctx.fillText('Q', centerX - 10, 20);
  };

  return (
    <div className="constellation-plot">
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
        Points: {pointCount} / {maxPoints}
      </div>
    </div>
  );
};

export default ConstellationPlot;
