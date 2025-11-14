import { AudioProvider, useAudio } from './audio/AudioProvider';
import ControlPanel from './components/ControlPanel';
import Oscilloscope from './components/Oscilloscope';
import SpectrumAnalyzer from './components/SpectrumAnalyzer';
import OscillatorMonitor from './components/OscillatorMonitor';
import FrameStructureViewer from './components/FrameStructureViewer';
import ModulationPlots from './components/ModulationPlots';
import FrameSequencer from './components/FrameSequencer';
import FunctionAnalyzer from './components/FunctionAnalyzer';
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
          <div className="visualization-row">
            <div className="viz-compact">
              <Oscilloscope />
            </div>
            <div className="viz-compact">
              <SpectrumAnalyzer />
            </div>
            <div className="viz-plots">
              <ModulationPlots />
            </div>
          </div>
          
          <div className="analysis-row">
            <div className="analysis-panel">
              <FrameStructureViewer effectName={audioState.gocs.currentFunction} />
            </div>
            <div className="analysis-panel">
              <FrameSequencer />
            </div>
            <div className="analysis-panel">
              <FunctionAnalyzer />
            </div>
          </div>
          
          <div className="monitor-row">
            <OscillatorMonitor />
          </div>
        </div>
      </div>
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
