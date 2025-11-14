import React from 'react';
import { useAudio } from '../audio/AudioProvider';

const FrameInspector: React.FC = () => {
  const { audioState } = useAudio();
  const { currentFrame, gocs } = audioState;

  return (
    <div className="frame-inspector">
      <div className="info-display">
        <div className="info-item">
          <span className="info-label">GOCS Function:</span>
          <span className="info-value">{gocs.currentFunction}</span>
        </div>
        <div className="info-item">
          <span className="info-label">Intensity:</span>
          <span className="info-value">{(gocs.intensity * 100).toFixed(0)}%</span>
        </div>
        <div className="info-item">
          <span className="info-label">Frame Number:</span>
          <span className="info-value">#{gocs.frameNumber}</span>
        </div>
        <div className="info-item">
          <span className="info-label">Time Remaining:</span>
          <span className="info-value">{gocs.frameTimeRemaining.toFixed(2)}s / 8.00s</span>
        </div>
      </div>

      {currentFrame && currentFrame.lfoParams && (
        <div className="frame-details">
          <h4>Current LFO Configuration</h4>
          <div className="lfo-summary">
            <div className="lfo-row">
              <span className="lfo-type">Phase:</span>
              <span className="lfo-detail">
                {currentFrame.lfoParams.phase.waveform} @ {currentFrame.lfoParams.phase.frequency}Hz
              </span>
            </div>
            <div className="lfo-row">
              <span className="lfo-type">Freq Mod:</span>
              <span className="lfo-detail">
                {currentFrame.lfoParams.freqMod.waveform} @ {currentFrame.lfoParams.freqMod.frequency}Hz
              </span>
            </div>
            <div className="lfo-row">
              <span className="lfo-type">Amp Mod:</span>
              <span className="lfo-detail">
                {currentFrame.lfoParams.ampMod.waveform} @ {currentFrame.lfoParams.ampMod.frequency}Hz
              </span>
            </div>
            <div className="lfo-row">
              <span className="lfo-type">FSK:</span>
              <span className="lfo-detail">
                {currentFrame.lfoParams.fsk.pattern} @ {currentFrame.lfoParams.fsk.rate}Hz
              </span>
            </div>
          </div>
        </div>
      )}

      <div className="frame-timing">
        <div className="progress-bar">
          <div 
            className="progress-fill"
            style={{ width: `${((8 - gocs.frameTimeRemaining) / 8) * 100}%` }}
          />
        </div>
        <p className="timing-label">
          Frame Progress: {((8 - gocs.frameTimeRemaining) / 8 * 100).toFixed(1)}%
        </p>
      </div>
    </div>
  );
};

export default FrameInspector;
