import React from 'react';
import GOCSControls from './GOCSControls';
import AIDControls from './AIDControls';
import './ControlPanel.css';

const ControlPanel: React.FC = () => {
  return (
    <div className="control-panel">
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
