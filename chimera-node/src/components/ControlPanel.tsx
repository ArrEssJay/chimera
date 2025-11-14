import React from 'react';
import GOCSControls from './GOCSControls';
import AIDControls from './AIDControls';
import { useAudio } from '../audio/AudioProvider';
import './ControlPanel.css';

const ControlPanel: React.FC = () => {
  const { isPlaying, play, stop } = useAudio();

  return (
    <div className="control-panel">
      <div className="panel-section transport-section">
        <button 
          className={`transport-btn-large ${isPlaying ? 'playing' : ''}`}
          onClick={isPlaying ? stop : play}
        >
          {isPlaying ? '⏸ PAUSE' : '▶ PLAY'}
        </button>
      </div>
      
      <div className="panel-section">
        <h2>GOCS Control</h2>
        <GOCSControls />
      </div>
      
      <div className="panel-section">
        <h2>AID Simulation</h2>
        <AIDControls />
      </div>
    </div>
  );
};

export default ControlPanel;
