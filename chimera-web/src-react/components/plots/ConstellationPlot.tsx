/**
 * ConstellationPlot - High-performance real-time constellation diagram
 * 
 * Uses Canvas 2D API with requestAnimationFrame for 60fps updates.
 * Displays I/Q symbol points with color-coded decision boundaries.
 * Supports toggling between TX and RX constellations.
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
  width: propWidth = 0,
  height: propHeight = 0,
  maxPoints = 1000,
  showGrid = true,
  showReference = true,
}) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const dataBufferRef = useRef<{ i: number[]; q: number[] }>({ i: [], q: [] });
  const animationFrameRef = useRef<number | null>(null);
  const [dimensions, setDimensions] = useState({ width: propWidth || 400, height: propHeight || 400 });
  const [showTx, setShowTx] = useState(true); // TX enabled by default

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
    const subscriptionId = 'constellation-plot';

    // Subscribe to DSP data updates
    dspService.subscribe(subscriptionId, (data: StreamData) => {
      // Update buffer with new constellation points
      const buffer = dataBufferRef.current;
      
      // Select TX or RX constellation
      const constellationI = showTx ? data.preChannel.txConstellationI : data.postChannel.rxConstellationI;
      const constellationQ = showTx ? data.preChannel.txConstellationQ : data.postChannel.rxConstellationQ;
      
      // Add new points
      for (let i = 0; i < constellationI.length; i++) {
        buffer.i.push(constellationI[i]);
        buffer.q.push(constellationQ[i]);
      }

      // Limit buffer size
      if (buffer.i.length > maxPoints) {
        const excess = buffer.i.length - maxPoints;
        buffer.i.splice(0, excess);
        buffer.q.splice(0, excess);
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
  }, [maxPoints, dimensions, showTx]);

  const renderCanvas = () => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    const buffer = dataBufferRef.current;
    const { width, height } = dimensions;
    const centerX = width / 2;
    const centerY = height / 2;
    const scale = Math.min(width, height) / 4.5; // Tighter fit

    // Clear canvas
    ctx.fillStyle = '#1a1a1a';
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

    // Draw reference constellation points for QPSK (normalized to ±0.707)
    if (showReference) {
      const sqrt2 = 0.707; // √2/2 for normalized QPSK
      const refPoints = [
        { i: sqrt2, q: sqrt2 },   // 00
        { i: -sqrt2, q: sqrt2 },  // 01
        { i: -sqrt2, q: -sqrt2 }, // 11
        { i: sqrt2, q: -sqrt2 },  // 10
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
        
        // Color based on TX/RX and quality
        let color = '#ffffff';
        if (showTx) {
          color = '#00ff88'; // Green for TX (ideal)
        } else {
          // Color code RX by distance from nearest ideal QPSK point
          const sqrt2 = 0.707;
          const idealPoints = [
            { i: sqrt2, q: sqrt2 },
            { i: -sqrt2, q: sqrt2 },
            { i: -sqrt2, q: -sqrt2 },
            { i: sqrt2, q: -sqrt2 },
          ];
          
          // Find minimum distance to ideal points
          const minDistance = Math.min(
            ...idealPoints.map(p => 
              Math.sqrt(Math.pow(iVal - p.i, 2) + Math.pow(qVal - p.q, 2))
            )
          );
          
          if (minDistance < 0.2) {
            color = '#00ff88'; // Green - close to ideal
          } else if (minDistance < 0.4) {
            color = '#ffaa00'; // Orange - moderate error
          } else {
            color = '#ff4444'; // Red - large error
          }
        }
        
        ctx.fillStyle = `${color}${Math.floor(alpha * 255).toString(16).padStart(2, '0')}`;
        
        // Draw point
        ctx.beginPath();
        ctx.arc(x, y, showTx ? 1.5 : 2, 0, 2 * Math.PI);
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
    
    // Draw title
    ctx.fillStyle = '#4a9eff';
    ctx.font = '11px sans-serif';
    ctx.textAlign = 'left';
    ctx.fillText(showTx ? 'TX Constellation' : 'RX Constellation', 5, 15);
    
    // Draw point count
    ctx.fillStyle = '#666';
    ctx.fillText(`Points: ${buffer.i.length}`, 5, height - 5);
  };

  return (
    <div ref={containerRef} className="constellation-plot" style={{ width: '100%', height: '100%', display: 'flex', flexDirection: 'column', position: 'relative' }}>
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
          className={`plot-toggle ${showTx ? 'active-tx' : ''}`}
          onClick={() => setShowTx(true)}
          style={{
            padding: '2px 6px',
            fontSize: '10px',
            background: showTx ? '#ff4444' : '#2a2a2a',
            color: showTx ? '#fff' : '#666',
            border: `1px solid ${showTx ? '#ff4444' : '#333'}`,
            cursor: 'pointer',
            transition: 'all 0.1s',
            textTransform: 'uppercase',
            fontWeight: 500,
          }}
        >
          TX
        </button>
        <button 
          className={`plot-toggle ${!showTx ? 'active-rx' : ''}`}
          onClick={() => setShowTx(false)}
          style={{
            padding: '2px 6px',
            fontSize: '10px',
            background: !showTx ? '#4a9eff' : '#2a2a2a',
            color: !showTx ? '#fff' : '#666',
            border: `1px solid ${!showTx ? '#4a9eff' : '#333'}`,
            cursor: 'pointer',
            transition: 'all 0.1s',
            textTransform: 'uppercase',
            fontWeight: 500,
          }}
        >
          RX
        </button>
      </div>
    </div>
  );
};

export default ConstellationPlot;
