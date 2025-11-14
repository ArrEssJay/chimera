import React, { useRef } from 'react';
import { useAudio } from '../audio/AudioProvider';
import './AudioLoader.css';

const AudioLoader: React.FC = () => {
  const { audioEngine } = useAudio();
  const fileInputRef = useRef<HTMLInputElement>(null);

  const handleFileUpload = async (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (!file || !audioEngine) return;

    try {
      const arrayBuffer = await file.arrayBuffer();
      const audioContext = new AudioContext();
      const audioBuffer = await audioContext.decodeAudioData(arrayBuffer);
      
      // Convert to Float32Array for the oscillator
      const audioData = audioBuffer.getChannelData(0);
      
      console.log('Loading external audio:', file.name, audioData.length, 'samples');
      // audioEngine.loadExternalAudio(audioData);
    } catch (error) {
      console.error('Error loading audio file:', error);
    }
  };

  const loadPreset = (presetName: string) => {
    console.log('Loading preset:', presetName);
    // Implement preset loading
    // audioEngine.loadExternalAudio(generatePreset(presetName));
  };

  return (
    <div className="audio-loader">
      <div className="control-group">
        <label>External Audio</label>
        <input
          ref={fileInputRef}
          type="file"
          accept="audio/*"
          onChange={handleFileUpload}
          style={{ display: 'none' }}
        />
        <button
          className="upload-btn"
          onClick={() => fileInputRef.current?.click()}
        >
          📁 Upload Audio File
        </button>
      </div>

      <div className="control-group">
        <label>Presets</label>
        <div className="preset-buttons">
          <button
            className="preset-btn"
            onClick={() => loadPreset('pink-noise')}
          >
            Pink Noise
          </button>
          <button
            className="preset-btn"
            onClick={() => loadPreset('white-noise')}
          >
            White Noise
          </button>
          <button
            className="preset-btn"
            onClick={() => loadPreset('test-tone')}
          >
            Test Tone
          </button>
        </div>
      </div>
    </div>
  );
};

export default AudioLoader;
