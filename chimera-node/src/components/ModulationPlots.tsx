import React, { useEffect, useRef } from 'react';
import { useAudio } from '../audio/AudioProvider';
import './ModulationPlots.css';

const ModulationPlots: React.FC = () => {
  const { audioState } = useAudio();
  const { currentFrame } = audioState;
  
  const fskCanvasRef = useRef<HTMLCanvasElement>(null);
  const freqCanvasRef = useRef<HTMLCanvasElement>(null);
  const ampCanvasRef = useRef<HTMLCanvasElement>(null);
  const phaseCanvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    if (!currentFrame) return;

    const frameData = currentFrame as any;
    const fskStates = frameData.fskStates || [];
    const freqModulation = frameData.freqModulation || [];
    const ampModulation = frameData.ampModulation || [];
    const phaseRotation = frameData.phaseRotation || [];

    console.log('ModulationPlots updating with frame data:', {
      fskLen: fskStates.length,
      freqLen: freqModulation.length,
      ampLen: ampModulation.length,
      phaseLen: phaseRotation.length
    });

    // Plot FSK states
    plotBinaryData(fskCanvasRef.current, fskStates, 'FSK STATES', '#00ff00');
    
    // Plot frequency modulation
    plotContinuousData(freqCanvasRef.current, freqModulation, 'FREQ MOD', '#00c853', -0.5, 0.5);
    
    // Plot amplitude modulation
    plotContinuousData(ampCanvasRef.current, ampModulation, 'AMP MOD', '#ffd600', 0.7, 1.1);
    
    // Plot phase rotation
    plotContinuousData(phaseCanvasRef.current, phaseRotation, 'PHASE', '#ff6d00', 0, 4);

  }, [currentFrame]);

  const plotBinaryData = (
    canvas: HTMLCanvasElement | null, 
    data: number[], 
    label: string,
    color: string
  ) => {
    if (!canvas || !data.length) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    const width = canvas.width;
    const height = canvas.height;
    const padding = 25;
    const plotWidth = width - padding * 2;
    const plotHeight = height - padding * 2;

    // Clear canvas
    ctx.fillStyle = '#1a1a1a';
    ctx.fillRect(0, 0, width, height);

    // Draw grid
    ctx.strokeStyle = '#333';
    ctx.lineWidth = 1;
    
    // Horizontal lines
    for (let i = 0; i <= 2; i++) {
      const y = padding + (plotHeight / 2) * i;
      ctx.beginPath();
      ctx.moveTo(padding, y);
      ctx.lineTo(width - padding, y);
      ctx.stroke();
    }

    // Vertical grid lines (every 2 seconds)
    for (let i = 0; i <= 4; i++) {
      const x = padding + (plotWidth / 4) * i;
      ctx.beginPath();
      ctx.moveTo(x, padding);
      ctx.lineTo(x, height - padding);
      ctx.stroke();
    }

    // Draw labels
    ctx.fillStyle = '#888';
    ctx.font = '10px monospace';
    ctx.fillText(label, padding, 15);
    ctx.fillText('1', 5, padding + 5);
    ctx.fillText('0', 5, height - padding + 5);

    // Time labels
    ctx.fillText('0s', padding, height - 5);
    ctx.fillText('4s', width / 2 - 10, height - 5);
    ctx.fillText('8s', width - padding - 10, height - 5);

    // Draw binary data as rectangles
    ctx.fillStyle = color;
    const rectWidth = plotWidth / data.length;
    
    data.forEach((value, i) => {
      if (value === 1) {
        const x = padding + (i / data.length) * plotWidth;
        const y = padding;
        ctx.fillRect(x, y, rectWidth, plotHeight / 2);
      }
    });
  };

  const plotContinuousData = (
    canvas: HTMLCanvasElement | null,
    data: number[],
    label: string,
    color: string,
    minVal: number,
    maxVal: number
  ) => {
    if (!canvas || !data.length) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    const width = canvas.width;
    const height = canvas.height;
    const padding = 25;
    const plotWidth = width - padding * 2;
    const plotHeight = height - padding * 2;

    // Clear canvas
    ctx.fillStyle = '#1a1a1a';
    ctx.fillRect(0, 0, width, height);

    // Draw grid
    ctx.strokeStyle = '#333';
    ctx.lineWidth = 1;

    // Horizontal lines
    for (let i = 0; i <= 4; i++) {
      const y = padding + (plotHeight / 4) * i;
      ctx.beginPath();
      ctx.moveTo(padding, y);
      ctx.lineTo(width - padding, y);
      ctx.stroke();
    }

    // Vertical grid lines
    for (let i = 0; i <= 4; i++) {
      const x = padding + (plotWidth / 4) * i;
      ctx.beginPath();
      ctx.moveTo(x, padding);
      ctx.lineTo(x, height - padding);
      ctx.stroke();
    }

    // Draw labels
    ctx.fillStyle = '#888';
    ctx.font = '10px monospace';
    ctx.fillText(label, padding, 15);
    ctx.fillText(maxVal.toFixed(1), 5, padding + 5);
    ctx.fillText(minVal.toFixed(1), 5, height - padding + 5);

    // Time labels
    ctx.fillText('0s', padding, height - 5);
    ctx.fillText('4s', width / 2 - 10, height - 5);
    ctx.fillText('8s', width - padding - 10, height - 5);

    // Draw waveform
    ctx.strokeStyle = color;
    ctx.lineWidth = 2;
    ctx.beginPath();

    const range = maxVal - minVal;
    
    data.forEach((value, i) => {
      const x = padding + (i / data.length) * plotWidth;
      const normalizedValue = (value - minVal) / range;
      const y = height - padding - (normalizedValue * plotHeight);

      if (i === 0) {
        ctx.moveTo(x, y);
      } else {
        ctx.lineTo(x, y);
      }
    });

    ctx.stroke();
  };

  if (!currentFrame) {
    return (
      <div className="modulation-plots">
        <div className="no-data">No frame data available</div>
      </div>
    );
  }

  return (
    <div className="modulation-plots">
      <canvas ref={fskCanvasRef} width={600} height={120} />
      <canvas ref={freqCanvasRef} width={600} height={120} />
      <canvas ref={ampCanvasRef} width={600} height={120} />
      <canvas ref={phaseCanvasRef} width={600} height={120} />
    </div>
  );
};

export default ModulationPlots;
