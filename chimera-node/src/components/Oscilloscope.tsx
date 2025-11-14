import React, { useRef, useEffect } from 'react';
import { useAudio } from '../audio/AudioProvider';

const Oscilloscope: React.FC = () => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const { audioEngine, isPlaying } = useAudio();
  const animationFrameRef = useRef<number>();

  useEffect(() => {
    if (!canvasRef.current || !audioEngine) return;

    const canvas = canvasRef.current;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    // Set canvas size
    canvas.width = canvas.offsetWidth;
    canvas.height = canvas.offsetHeight;

    const analyser = audioEngine.getAnalyserNode();
    if (!analyser) return;

    const bufferLength = analyser.fftSize;
    const dataArray = new Uint8Array(bufferLength);

    const draw = () => {
      if (!isPlaying) {
        // Draw empty state with grid
        ctx.fillStyle = '#1a1a1a';
        ctx.fillRect(0, 0, canvas.width, canvas.height);
        drawGrid();
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

export default Oscilloscope;
