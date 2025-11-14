import React from 'react';
import { useAudio } from '../audio/AudioProvider';
import './TransportBar.css';

const TransportBar: React.FC = () => {
  const { isPlaying, play, stop, advanceFrame, audioState } = useAudio();

  return (
    <div className="transport-bar">
      <div className="transport-controls">
        <button 
          className="transport-btn play-pause"
          onClick={isPlaying ? stop : play}
        >
          {isPlaying ? '⏸ Pause' : '▶ Play'}
        </button>
        
        <button 
          className="transport-btn"
          onClick={advanceFrame}
          disabled={!isPlaying}
        >
          ⏭ Next Frame
        </button>
      </div>

      <div className="transport-info">
        <div className="status-indicator">
          <span className={`status-dot ${isPlaying ? 'playing' : 'stopped'}`}></span>
          <span className="status-text">
            {isPlaying ? 'Playing' : 'Stopped'}
          </span>
        </div>
        
        <div className="frame-counter">
          Frame #{audioState.gocs.frameNumber} | {audioState.gocs.frameTimeRemaining.toFixed(1)}s remaining
        </div>
      </div>
    </div>
  );
};

export default TransportBar;
