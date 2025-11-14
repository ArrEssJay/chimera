import { useRef, useEffect } from 'react';
import { useAudio } from '../audio/AudioProvider';

const SpectrumAnalyzer = () => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const { audioEngine, isPlaying } = useAudio();
  const animationFrameRef = useRef<number>();

  useEffect(() => {
    if (!canvasRef.current || !audioEngine) return;

    const canvas = canvasRef.current;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    const resizeCanvas = () => {
      const rect = canvas.getBoundingClientRect();
      canvas.width = rect.width;
      canvas.height = rect.height;
    };
    resizeCanvas();
    window.addEventListener('resize', resizeCanvas);

    const analyser = audioEngine.getAnalyserNode();
    if (!analyser) return;

    // Increase FFT size for better frequency resolution
    analyser.fftSize = 8192;
    analyser.smoothingTimeConstant = 0.75;

    const bufferLength = analyser.frequencyBinCount;
    const dataArray = new Uint8Array(bufferLength);
    const sampleRate = 48000;

    // Logarithmic frequency scale
    const minFreq = 20;
    const maxFreq = 20000;
    const minLog = Math.log10(minFreq);
    const maxLog = Math.log10(maxFreq);

    const draw = () => {
      if (!isPlaying) {
        ctx.fillStyle = '#1a1a1a';
        ctx.fillRect(0, 0, canvas.width, canvas.height);
        drawGrid();
        drawFrequencyLabels();
        
        // Draw label
        ctx.fillStyle = '#666';
        ctx.font = '10px monospace';
        ctx.fillText('SPECTRUM ANALYZER', 8, 15);
        
        animationFrameRef.current = requestAnimationFrame(draw);
        return;
      }

      analyser.getByteFrequencyData(dataArray);

      ctx.fillStyle = '#1a1a1a';
      ctx.fillRect(0, 0, canvas.width, canvas.height);

      drawGrid();

      // Draw frequency data with logarithmic scale
      const numBars = Math.min(150, canvas.width / 2);
      
      for (let i = 0; i < numBars; i++) {
        const t = i / numBars;
        const logFreq = minLog + t * (maxLog - minLog);
        const freq = Math.pow(10, logFreq);
        
        const bin = Math.floor((freq / sampleRate) * bufferLength * 2);
        
        if (bin >= 0 && bin < bufferLength) {
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
          
          const db = (value / 255) * 60 - 60;
          const normalizedDb = Math.max(0, (db + 60) / 60);
          const barHeight = normalizedDb * (canvas.height - 25);

          const x = (i / numBars) * canvas.width;
          const barWidth = (canvas.width / numBars) * 0.9;

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

      drawFrequencyLabels();
      
      // Draw label
      ctx.fillStyle = '#888';
      ctx.font = '10px monospace';
      ctx.fillText('SPECTRUM ANALYZER', 8, 15);

      animationFrameRef.current = requestAnimationFrame(draw);
    };

    const drawGrid = () => {
      ctx.strokeStyle = '#2d2d2d';
      ctx.lineWidth = 1;

      // Horizontal grid lines (3 lines)
      for (let i = 1; i < 4; i++) {
        const y = (i / 4) * (canvas.height - 20);
        ctx.beginPath();
        ctx.moveTo(0, y);
        ctx.lineTo(canvas.width, y);
        ctx.stroke();
      }

      // Vertical grid lines (decade markers) - fewer for compact view
      const decades = [100, 1000, 10000];
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
      ctx.fillStyle = '#666';
      ctx.font = '9px monospace';
      ctx.textAlign = 'center';

      const labels = [
        { freq: 100, label: '100' },
        { freq: 1000, label: '1K' },
        { freq: 10000, label: '10K' }
      ];

      labels.forEach(({ freq, label }) => {
        const logFreq = Math.log10(freq);
        const x = ((logFreq - minLog) / (maxLog - minLog)) * canvas.width;
        ctx.fillText(label, x, canvas.height - 5);
      });
    };

    draw();

    return () => {
      window.removeEventListener('resize', resizeCanvas);
      if (animationFrameRef.current) {
        cancelAnimationFrame(animationFrameRef.current);
      }
    };
  }, [audioEngine, isPlaying]);

  return (
    <canvas 
      ref={canvasRef}
      style={{ width: '100%', height: '100%', display: 'block' }}
    />
  );
};

export default SpectrumAnalyzer;
