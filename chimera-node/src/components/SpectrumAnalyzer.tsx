import React, { useRef, useEffect } from 'react';
import { useAudio } from '../audio/AudioProvider';

const SpectrumAnalyzer: React.FC = () => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const { audioEngine, isPlaying } = useAudio();
  const animationFrameRef = useRef<number>();

  useEffect(() => {
    if (!canvasRef.current || !audioEngine) return;

    const canvas = canvasRef.current;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    canvas.width = canvas.offsetWidth;
    canvas.height = canvas.offsetHeight;

    const analyser = audioEngine.getAnalyserNode();
    if (!analyser) return;

    // Increase FFT size for better frequency resolution
    analyser.fftSize = 8192;
    analyser.smoothingTimeConstant = 0.75;

    const bufferLength = analyser.frequencyBinCount;
    const dataArray = new Uint8Array(bufferLength);
    const sampleRate = 48000; // Assuming 48kHz

    // Logarithmic frequency scale for better visualization
    const minFreq = 20;
    const maxFreq = 20000;
    const minLog = Math.log10(minFreq);
    const maxLog = Math.log10(maxFreq);

    const draw = () => {
      if (!isPlaying) {
        // Draw empty state
        ctx.fillStyle = '#1a1a1a';
        ctx.fillRect(0, 0, canvas.width, canvas.height);
        drawGrid();
        animationFrameRef.current = requestAnimationFrame(draw);
        return;
      }

      analyser.getByteFrequencyData(dataArray);

      // Clear canvas
      ctx.fillStyle = '#1a1a1a';
      ctx.fillRect(0, 0, canvas.width, canvas.height);

      // Draw grid
      drawGrid();

      // Draw frequency data with logarithmic scale
      const numBars = Math.min(200, canvas.width / 3); // Adaptive bar count
      
      for (let i = 0; i < numBars; i++) {
        const t = i / numBars;
        const logFreq = minLog + t * (maxLog - minLog);
        const freq = Math.pow(10, logFreq);
        
        // Map frequency to FFT bin
        const bin = Math.floor((freq / sampleRate) * bufferLength * 2);
        
        if (bin >= 0 && bin < bufferLength) {
          // Average nearby bins for smoother display
          const avgBins = 3;
          let sum = 0;
          let count = 0;
          for (let j = -avgBins; j <= avgBins; j++) {
            const idx = bin + j;
            if (idx >= 0 && idx < bufferLength) {
              sum += dataArray[idx];
              count++;
            }
          }
          const value = sum / count;
          
          // Convert to dB scale (0-255 -> -60dB to 0dB)
          const db = (value / 255) * 60 - 60;
          const normalizedDb = Math.max(0, (db + 60) / 60);
          const barHeight = normalizedDb * (canvas.height - 40);

          const x = (i / numBars) * canvas.width;
          const barWidth = (canvas.width / numBars) * 0.9;

          // Pro-AV color scheme - green to yellow to red
          let color;
          if (normalizedDb < 0.5) {
            color = `rgba(0, ${Math.floor(normalizedDb * 400)}, 0, 0.9)`;
          } else if (normalizedDb < 0.75) {
            const t = (normalizedDb - 0.5) / 0.25;
            color = `rgba(${Math.floor(t * 255)}, 200, 0, 0.9)`;
          } else {
            const t = (normalizedDb - 0.75) / 0.25;
            color = `rgba(255, ${Math.floor(200 - t * 200)}, 0, 0.9)`;
          }
          
          ctx.fillStyle = color;
          ctx.fillRect(x, canvas.height - barHeight - 20, barWidth, barHeight);
        }
      }

      // Draw frequency labels
      drawFrequencyLabels();

      animationFrameRef.current = requestAnimationFrame(draw);
    };

    const drawGrid = () => {
      ctx.strokeStyle = '#2d2d2d';
      ctx.lineWidth = 1;

      // Horizontal grid lines (dB levels)
      for (let i = 0; i <= 4; i++) {
        const y = (i / 4) * (canvas.height - 20) + 10;
        ctx.beginPath();
        ctx.moveTo(0, y);
        ctx.lineTo(canvas.width, y);
        ctx.stroke();
      }

      // Vertical grid lines (decade markers)
      const decades = [20, 50, 100, 200, 500, 1000, 2000, 5000, 10000, 20000];
      decades.forEach(freq => {
        const logFreq = Math.log10(freq);
        const x = ((logFreq - minLog) / (maxLog - minLog)) * canvas.width;
        ctx.beginPath();
        ctx.moveTo(x, 0);
        ctx.lineTo(x, canvas.height - 20);
        ctx.stroke();
      });
    };

    const drawFrequencyLabels = () => {
      ctx.fillStyle = '#888';
      ctx.font = '11px monospace';
      ctx.textAlign = 'center';

      const labels = [
        { freq: 20, label: '20' },
        { freq: 100, label: '100' },
        { freq: 1000, label: '1K' },
        { freq: 10000, label: '10K' },
        { freq: 20000, label: '20K' }
      ];

      labels.forEach(({ freq, label }) => {
        const logFreq = Math.log10(freq);
        const x = ((logFreq - minLog) / (maxLog - minLog)) * canvas.width;
        ctx.fillText(label, x, canvas.height - 5);
      });
    };

    draw();

    return () => {
      if (animationFrameRef.current) {
        cancelAnimationFrame(animationFrameRef.current);
      }
    };
  }, [audioEngine, isPlaying]);

  return (
    <div className="canvas-container">
      <canvas ref={canvasRef} />
    </div>
  );
};

export default SpectrumAnalyzer;
