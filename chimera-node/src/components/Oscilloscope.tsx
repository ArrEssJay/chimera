import { useRef, useEffect } from 'react';
import { useAudio } from '../audio/AudioProvider';

const Oscilloscope = () => {
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

    const bufferLength = analyser.fftSize;
    const dataArray = new Uint8Array(bufferLength);

    const draw = () => {
      if (!isPlaying) {
        ctx.fillStyle = '#1a1a1a';
        ctx.fillRect(0, 0, canvas.width, canvas.height);
        drawGrid();
        
        // Draw label
        ctx.fillStyle = '#666';
        ctx.font = '10px monospace';
        ctx.fillText('OSCILLOSCOPE', 8, 15);
        
        animationFrameRef.current = requestAnimationFrame(draw);
        return;
      }

      analyser.getByteTimeDomainData(dataArray);

      ctx.fillStyle = '#1a1a1a';
      ctx.fillRect(0, 0, canvas.width, canvas.height);

      // Draw grid
      drawGrid();

      // Draw waveform
      ctx.lineWidth = 1.5;
      ctx.strokeStyle = '#00c853';
      ctx.beginPath();

      const sliceWidth = canvas.width / bufferLength;
      let x = 0;

      for (let i = 0; i < bufferLength; i++) {
        const v = dataArray[i] / 128.0;
        const y = (v * canvas.height) / 2;

        if (i === 0) {
          ctx.moveTo(x, y);
        } else {
          ctx.lineTo(x, y);
        }

        x += sliceWidth;
      }

      ctx.lineTo(canvas.width, canvas.height / 2);
      ctx.stroke();
      
      // Draw label
      ctx.fillStyle = '#888';
      ctx.font = '10px monospace';
      ctx.fillText('OSCILLOSCOPE', 8, 15);

      animationFrameRef.current = requestAnimationFrame(draw);
    };

    const drawGrid = () => {
      ctx.strokeStyle = '#2d2d2d';
      ctx.lineWidth = 1;

      // Horizontal center line (brighter)
      ctx.strokeStyle = '#3a3a3a';
      ctx.beginPath();
      ctx.moveTo(0, canvas.height / 2);
      ctx.lineTo(canvas.width, canvas.height / 2);
      ctx.stroke();

      // Horizontal grid lines
      ctx.strokeStyle = '#2d2d2d';
      for (let i = 1; i < 4; i++) {
        const y = (i / 4) * canvas.height;
        ctx.beginPath();
        ctx.moveTo(0, y);
        ctx.lineTo(canvas.width, y);
        ctx.stroke();
      }

      // Vertical grid lines
      for (let i = 1; i < 8; i++) {
        const x = (i / 8) * canvas.width;
        ctx.beginPath();
        ctx.moveTo(x, 0);
        ctx.lineTo(x, canvas.height);
        ctx.stroke();
      }
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

export default Oscilloscope;
