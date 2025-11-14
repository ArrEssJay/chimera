import React from 'react';
import { AudioProvider } from './audio/AudioProvider';
import ControlPanel from './components/ControlPanel';
import VisualizationPanel from './components/VisualizationPanel';
import TransportBar from './components/TransportBar';
import './styles/App.css';

function App() {
  return (
    <AudioProvider>
      <div className="app">
        <header className="app-header">
          <h1>Chimera GOCS Real-time Interface</h1>
          <p className="subtitle">Real-time waveform generation with AID monitoring</p>
        </header>
        
        <div className="app-content">
          <ControlPanel />
          <VisualizationPanel />
        </div>
        
        <TransportBar />
      </div>
    </AudioProvider>
  );
}

export default App;
