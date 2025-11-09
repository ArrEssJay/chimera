import React, { useState, useEffect } from 'react';
import ConfigPanel from './components/ConfigPanel';
import VisualizationPanel from './components/VisualizationPanel';
import FrameDecoder from './components/FrameDecoder';
import MessageDecoder from './components/MessageDecoder';
import { getWASMDSPService, StreamData } from './services/WASMDSPService';
import { SimulationConfig } from './types';

const App: React.FC = () => {
  const [config, setConfig] = useState<SimulationConfig>({
    plaintext_source: 'Hello CHIMERA',
    snr_db: 10,
    link_loss_db: 0,
    sample_rate: 48000,
    bit_depth: 'Float32',
  });

  const [isDSPRunning, setIsDSPRunning] = useState(false);
  const [streamData, setStreamData] = useState<StreamData | null>(null);
  const [dspService] = useState(() => getWASMDSPService());

  useEffect(() => {
    // Subscribe to streaming data
    dspService.subscribe('app-stream', (data) => {
      setStreamData(data);
    });

    return () => {
      dspService.unsubscribe('app-stream');
    };
  }, [dspService]);

  const handleStartDSP = async () => {
    try {
      dspService.clearLogs();
      dspService.addLog('Initializing DSP engine...');
      
      // Configure DSP with current settings
      await dspService.configure({
        simulation: {
          sample_rate: config.sample_rate,
          bit_depth: config.bit_depth,
          snr_db: config.snr_db,
          link_loss_db: config.link_loss_db,
          plaintext_source: config.plaintext_source,
          rng_seed: config.rng_seed,
        },
        protocol: {
          carrier_freq_hz: 12000,
          qpsk_symbol_rate: 16,
          qpsk_bandwidth_hz: 20.0,
          fsk_bit_rate: 1.0,
          fsk_freq_zero_hz: 11999,
          fsk_freq_one_hz: 12001,
          command_opcode: 0x0001,
          frame_layout: {
            total_symbols: 128,
            sync_symbols: 16,
            target_id_symbols: 16,
            command_type_symbols: 16,
            data_payload_symbols: 64,
            ecc_symbols: 16,
          },
          sync_sequence_hex: 'A5A5A5A5',
          target_id_hex: 'DEADBEEF',
          max_frames: 256,
          current_frame_shift: 16,
          total_frames_shift: 24,
        },
        ldpc: {
          dv: 2,
          dc: 10,
          seed: 42,
        },
      });

      dspService.addLog('Starting audio processing...');
      await dspService.start();
      
      setIsDSPRunning(true);
      dspService.addLog('DSP engine started successfully');
      
      // TODO: Trigger actual encoding/decoding with the message
      // This would integrate with your Rust backend to process config.plaintext_source
      
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
            disabled={!config.plaintext_source}
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
