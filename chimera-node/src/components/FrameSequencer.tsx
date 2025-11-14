import React from 'react';
import { useAudio } from '../audio/AudioProvider';
import './FrameSequencer.css';

const FrameSequencer: React.FC = () => {
  const { audioState } = useAudio();
  const { currentFrame, gocs } = audioState;

  if (!currentFrame) {
    return (
      <div className="frame-sequencer">
        <div className="sequencer-header">FRAME SEQUENCE</div>
        <div className="no-sequence">No active sequence</div>
      </div>
    );
  }

  const frameData = currentFrame as any;
  const commandType = frameData.commandType || {};
  
  // Parse sequencing byte: upper 4 bits = current frame, lower 4 bits = total frames
  const currentFrameNum = (commandType.sequencing >> 4) & 0x0F || 1;
  const totalFrames = (commandType.sequencing & 0x0F) || 1;
  const duration = commandType.duration || 1;

  console.log('FrameSequencer - sequencing byte:', commandType.sequencing, 'current:', currentFrameNum, 'total:', totalFrames);

  // Calculate sequence info
  const sequenceLength = Math.max(totalFrames, duration);
  const progress = currentFrameNum / Math.max(1, sequenceLength - 1);
  const timePerFrame = 8; // seconds
  const totalSequenceTime = sequenceLength * timePerFrame;
  const elapsedTime = (currentFrameNum - 1) * timePerFrame + (8 - gocs.frameTimeRemaining);
  const remainingTime = totalSequenceTime - elapsedTime;

  // Generate frame indicators
  const frameIndicators = [];
  for (let i = 1; i <= sequenceLength; i++) {
    const isActive = i === currentFrameNum;
    const isCompleted = i < currentFrameNum;
    
    frameIndicators.push(
      <div 
        key={i} 
        className={`frame-indicator ${isActive ? 'active' : ''} ${isCompleted ? 'completed' : ''}`}
      >
        <div className="frame-number">{i}</div>
        <div className="frame-bar">
          {isActive && (
            <div 
              className="frame-progress" 
              style={{ width: `${((8 - gocs.frameTimeRemaining) / 8 * 100)}%` }}
            />
          )}
        </div>
      </div>
    );
  }

  return (
    <div className="frame-sequencer">
      <div className="sequencer-header">
        FRAME SEQUENCE: {gocs.currentFunction || 'Direct Control'}
      </div>

      <div className="sequence-info">
        <div className="info-row">
          <span className="info-label">Current Frame:</span>
          <span className="info-value">{currentFrameNum} of {sequenceLength}</span>
        </div>
        <div className="info-row">
          <span className="info-label">Frame Duration:</span>
          <span className="info-value">{timePerFrame}s</span>
        </div>
        <div className="info-row">
          <span className="info-label">Sequence Duration:</span>
          <span className="info-value">{totalSequenceTime}s</span>
        </div>
        <div className="info-row">
          <span className="info-label">Time Remaining:</span>
          <span className="info-value">{remainingTime.toFixed(1)}s</span>
        </div>
      </div>

      <div className="sequence-timeline">
        {frameIndicators}
      </div>

      <div className="sequence-progress-bar">
        <div 
          className="sequence-progress-fill" 
          style={{ width: `${(progress * 100)}%` }}
        />
      </div>

      <div className="sequence-details">
        <div className="detail-section">
          <div className="detail-label">Function Parameters</div>
          <div className="detail-value">
            Intensity: {((gocs.intensity || 0.5) * 100).toFixed(0)}%
          </div>
          <div className="detail-value">
            Duration: {duration} frame(s)
          </div>
        </div>
        
        {currentFrameNum < sequenceLength && (
          <div className="detail-section">
            <div className="detail-label">Next Frame</div>
            <div className="detail-value">
              Frame {currentFrameNum + 1} in {gocs.frameTimeRemaining.toFixed(1)}s
            </div>
          </div>
        )}
      </div>
    </div>
  );
};

export default FrameSequencer;
