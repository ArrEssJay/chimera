import React, { useState, useEffect } from 'react';
import ConfigPanel from './components/ConfigPanel';
import ThzControlPanel from './components/ThzControlPanel';
import DebugControlPanel from './components/DebugControlPanel';
import VisualizationPanel from './components/VisualizationPanel';
import FrameDecoder from './components/FrameDecoder';
import MessageDecoder from './components/MessageDecoder';
import { getWASMDSPService, StreamData } from './services/WASMDSPService';
import { ConfigBundle } from './types';

const App: React.FC = () => {
  const [config, setConfig] = useState<ConfigBundle | null>(null);
  const [isDSPRunning, setIsDSPRunning] = useState(false);
  const [streamData, setStreamData] = useState<StreamData | null>(null);
  const [dspService] = useState(() => getWASMDSPService());

  // Load default preset from WASM after module initializes
  useEffect(() => {
    const loadDefaultPreset = async () => {
      try {
        // Import and initialize WASM module
        const wasmModule = await import('../pkg/chimera_web');
        await wasmModule.default();
        
        // Load default preset (Raman Whisper)
        const bundleJson = wasmModule.get_preset_bundle('raman-whisper');
        if (!bundleJson) {
          throw new Error('Failed to load default preset');
        }
        setConfig(JSON.parse(bundleJson));
      } catch (error) {
        console.error('Failed to initialize WASM and load preset:', error);
      }
    };

    loadDefaultPreset();
  }, []);

  useEffect(() => {
    // Subscribe to streaming data
    dspService.subscribe('app-stream', (data) => {
      setStreamData(data);
    });

    return () => {
      dspService.unsubscribe('app-stream');
    };
  }, [dspService]);

  // Handle runtime config updates (SNR and link_loss_db)
  useEffect(() => {
    if (isDSPRunning && config) {
      // Apply runtime updates to channel parameters only
      dspService.updateChannelParams(config.simulation.snr_db, config.simulation.link_loss_db);
    }
  }, [config?.simulation.snr_db, config?.simulation.link_loss_db, isDSPRunning, dspService]);

  const handleStartDSP = async () => {
    if (!config) return;
    
    try {
      dspService.clearLogs();
      dspService.addLog('Initializing DSP engine...');
      
      // Configure DSP with current settings
      await dspService.configure({
        simulation: config.simulation,
        protocol: config.protocol,
        ldpc: config.ldpc,
      });

      dspService.addLog('Starting audio processing...');
      await dspService.start();
      
      setIsDSPRunning(true);
      dspService.addLog('DSP engine started successfully');
      
      // TODO: Trigger actual encoding/decoding with the message
      // This would integrate with your Rust backend to process config.simulation.plaintext_source
      
    } catch (error) {
      console.error('Failed to start DSP:', error);
      dspService.addLog(`Error: ${error instanceof Error ? error.message : 'Unknown error'}`);
      alert('Failed to start DSP engine. Check console for details.');
    }
  };

  const handleStopDSP = () => {
    dspService.addLog('Stopping DSP engine...');
    dspService.stop();
    setIsDSPRunning(false);
    dspService.addLog('DSP engine stopped');
  };

  // THz Control Handlers
  const handleModulationModeChange = (active: boolean) => {
    dspService.setModulationMode(active);
    dspService.addLog(`THz mode: ${active ? 'Active' : 'Idle'}`);
  };

  const handleModulationDepthChange = (depth: number) => {
    dspService.setModulationDepth(depth);
    dspService.addLog(`THz modulation depth: ${(depth * 100).toFixed(1)}%`);
  };

  const handleMixingCoefficientChange = (coefficient: number) => {
    dspService.setMixingCoefficient(coefficient);
    dspService.addLog(`THz mixing coefficient: ${coefficient.toFixed(2)}`);
  };

  const handleGenerateIdleCarrier = () => {
    const samples = dspService.generateIdleCarrier(100);
    if (samples) {
      dspService.addLog(`Generated ${samples.length} samples of idle carrier`);
      // Optionally play the samples or show them in a visualization
    }
  };

  // Debug Control Handlers
  const handleQpskEnableChange = (enabled: boolean) => {
    dspService.setQpskEnabled(enabled);
    dspService.addLog(`QPSK modulation: ${enabled ? 'ENABLED' : 'DISABLED'}`);
  };

  const handleFskEnableChange = (enabled: boolean) => {
    dspService.setFskEnabled(enabled);
    dspService.addLog(`FSK frequency dithering: ${enabled ? 'ENABLED' : 'DISABLED'}`);
  };

  // Show loading state while WASM initializes
  if (!config) {
    return (
      <div className="app">
        <div style={{ display: 'flex', justifyContent: 'center', alignItems: 'center', height: '100vh' }}>
          <p>Loading CHIMERA...</p>
        </div>
      </div>
    );
  }

  return (
    <div className="app">
      <header className="app-header">
        <div className="header-left">
          <h1>CHIMERA</h1>
          <span className="header-subtitle">SIGINT Workbench</span>
        </div>
        <div className="header-center">
          <button
            onClick={isDSPRunning ? handleStopDSP : handleStartDSP}
            className={`btn ${isDSPRunning ? 'btn-danger' : 'btn-success'}`}
            disabled={!config.simulation.plaintext_source}
          >
            {isDSPRunning ? '⏹ Stop DSP' : '▶ Start DSP'}
          </button>
        </div>
        <div className="header-right">
          <span className="status-indicator">
            <span className={`status-dot ${isDSPRunning ? 'status-active' : 'status-inactive'}`}></span>
            {isDSPRunning ? 'Running' : 'Stopped'}
          </span>
        </div>
      </header>

      <div className="app-body">
        <aside className="sidebar-left">
          <ConfigPanel
            config={config}
            onChange={setConfig}
            disabled={isDSPRunning}
            allowRuntimeUpdate={true}
          />
          <ThzControlPanel
            disabled={!isDSPRunning}
            onModulationModeChange={handleModulationModeChange}
            onModulationDepthChange={handleModulationDepthChange}
            onMixingCoefficientChange={handleMixingCoefficientChange}
            onGenerateIdleCarrier={handleGenerateIdleCarrier}
          />
          <DebugControlPanel
            disabled={!isDSPRunning}
            onQpskEnableChange={handleQpskEnableChange}
            onFskEnableChange={handleFskEnableChange}
          />
        </aside>

        <main className="main-content">
          <VisualizationPanel
            isProcessing={isDSPRunning}
            streamData={streamData}
          />
          <MessageDecoder />
        </main>

        <aside className="sidebar-right">
          <FrameDecoder />
        </aside>
      </div>
    </div>
  );
};

export default App;
