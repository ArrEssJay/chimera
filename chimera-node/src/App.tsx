import { AudioProvider, useAudio } from './audio/AudioProvider';
import ControlPanel from './components/ControlPanel';
import Oscilloscope from './components/Oscilloscope';
import SpectrumAnalyzer from './components/SpectrumAnalyzer';
import OscillatorMonitor from './components/OscillatorMonitor';
import FrameStructureViewer from './components/FrameStructureViewer';
import TransportBar from './components/TransportBar';
import './styles/App.css';

function AppContent() {
  const { audioState } = useAudio();
  
  return (
    <div className="app">
      <header className="app-header">
        <h1>Chimera GOCS Real-time Interface</h1>
        <p className="subtitle">Frame Structure v3.1 | Real-time Parameter Monitoring</p>
      </header>
      
      <div className="app-content">
        <div className="left-panel">
          <ControlPanel />
        </div>
        
        <div className="center-panel">
          <div className="visualization-grid">
            <div className="viz-item">
              <Oscilloscope />
            </div>
            <div className="viz-item">
              <SpectrumAnalyzer />
            </div>
          </div>
          
          <div className="monitor-grid">
            <OscillatorMonitor />
            <FrameStructureViewer effectName={audioState.gocs.currentFunction} />
          </div>
        </div>
      </div>
      
      <TransportBar />
    </div>
  );
}

function App() {
  return (
    <AudioProvider>
      <AppContent />
    </AudioProvider>
  );
}

export default App;
