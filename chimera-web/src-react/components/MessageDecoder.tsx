import React, { useEffect, useState } from 'react';
import { getWASMDSPService, type FrameData, type StreamData } from '../services/WASMDSPService';

interface FrameHistory {
  frameNumber: number;
  timestamp: number;
  decodedText: string;
  symbolCount: number;
  syncData: Uint8Array;
  payloadData: Uint8Array;
  parityData: Uint8Array;
}

interface CompleteMessage {
  timestamp: number;
  frames: FrameHistory[];
  completePayload: string;
}

const MessageDecoder: React.FC = () => {
  const [frameData, setFrameData] = useState<FrameData | null>(null);
  const [frameHistory, setFrameHistory] = useState<FrameHistory[]>([]);
  const [completeMessages, setCompleteMessages] = useState<CompleteMessage[]>([]);
  const [currentDecodedText, setCurrentDecodedText] = useState('');
  const [currentFrameNumber, setCurrentFrameNumber] = useState(0);
  const service = getWASMDSPService();

  useEffect(() => {
    const handleData = (data: StreamData) => {
      if (!data) return;
      if (!data.currentFrameData) return;
      
      const frame = data.currentFrameData;
      const frameNumber = frame.frameNumber;
      
      // Check if we've moved to a new frame
      if (frameNumber !== currentFrameNumber) {
        // Archive the completed previous frame
        if (currentFrameNumber > 0 && frameData) {
          const completedFrame: FrameHistory = {
            frameNumber: currentFrameNumber,
            timestamp: Date.now(),
            decodedText: currentDecodedText,
            symbolCount: 128,
            syncData: frameData.syncData,
            payloadData: frameData.payloadData,
            parityData: frameData.parityData,
          };
          
          setFrameHistory(prev => {
            const updated = [completedFrame, ...prev];
            
            // Check if this completes a multi-frame message
            // Assuming frame 0 indicates end of message
            if (frameNumber === 1) {
              const messageFrames = [completedFrame];
              const completePayload = messageFrames
                .map(f => f.decodedText)
                .join('');
              
              setCompleteMessages(msgs => [
                {
                  timestamp: Date.now(),
                  frames: messageFrames,
                  completePayload,
                },
                ...msgs.slice(0, 9)
              ]);
            }
            
            return updated.slice(0, 20);
          });
        }
        
        setCurrentFrameNumber(frameNumber);
        setCurrentDecodedText('');
      }
      
      // Update current frame state
      setFrameData(frame);
      
      // Update decoded text from the data stream
      if (data.decodedText) {
        setCurrentDecodedText(data.decodedText);
      }
    };
    
    service.subscribe('message-decoder', handleData);
    return () => service.unsubscribe('message-decoder');
  }, [service, currentFrameNumber, currentDecodedText, frameData]);

  const formatHex = (data: Uint8Array | undefined, maxBytes?: number): string => {
    if (!data || data.length === 0) return '—';
    const bytes = Array.from(maxBytes ? data.slice(0, maxBytes) : data);
    const hex = bytes.map(b => b.toString(16).padStart(2, '0')).join(' ');
    return (maxBytes && data.length > maxBytes) ? `${hex}...` : hex;
  };

  const formatPayloadText = (data: Uint8Array | undefined): string => {
    if (!data || data.length === 0) return '';
    return Array.from(data)
      .map(b => (b >= 32 && b <= 126) ? String.fromCharCode(b) : '.')
      .join('');
  };

  const formatTimestamp = (timestamp: number): string => {
    const date = new Date(timestamp);
    const timeStr = date.toLocaleTimeString('en-US', { 
      hour12: false, 
      hour: '2-digit', 
      minute: '2-digit', 
      second: '2-digit'
    });
    const ms = date.getMilliseconds().toString().padStart(3, '0');
    return `${timeStr}.${ms}`;
  };

  return (
    <div className="message-decoder-scroll-container">
      <div className="message-decoder">
        <h3>Message Decoder</h3>

        {/* Complete Frame Structure */}
        <div className="frame-structure-section">
          <h4>Current Frame Structure</h4>
          <div className="frame-data-display">
            <div className="frame-data-row">
              <span className="frame-data-label">Sync:</span>
              <span className="frame-data-hex">{formatHex(frameData?.syncData)}</span>
            </div>
            <div className="frame-data-row">
              <span className="frame-data-label">Payload:</span>
              <span className="frame-data-hex">{formatHex(frameData?.payloadData)}</span>
            </div>
            <div className="frame-data-row">
              <span className="frame-data-label">Parity:</span>
              <span className="frame-data-hex">{formatHex(frameData?.parityData)}</span>
            </div>
            {frameData?.payloadData && frameData.payloadData.length > 0 && (
              <div className="frame-data-row">
                <span className="frame-data-label">Text:</span>
                <span className="frame-data-text">{formatPayloadText(frameData.payloadData) || <em>No printable text</em>}</span>
              </div>
            )}
          </div>
        </div>

        {/* Complete Messages */}
        {completeMessages.length > 0 && (
          <div className="complete-messages">
            <h4>Complete Messages</h4>
            <div className="messages-list">
              {completeMessages.map((msg, idx) => (
                <div key={idx} className="complete-message-item">
                  <div className="message-header">
                    <span className="message-time">{formatTimestamp(msg.timestamp)}</span>
                    <span className="message-frames">{msg.frames.length} frame{msg.frames.length > 1 ? 's' : ''}</span>
                  </div>
                  <div className="message-payload">{msg.completePayload}</div>
                  <div className="message-frames-detail">
                    {msg.frames.map((frame) => (
                      <div key={frame.frameNumber} className="message-frame-item">
                        <span className="frame-num">F#{frame.frameNumber}</span>
                        <span className="frame-hex">{formatHex(frame.payloadData, 8)}</span>
                      </div>
                    ))}
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}

        {/* Frame History */}
        {frameHistory.length > 0 && (
          <div className="frame-history">
            <h4>Recent Frames</h4>
            <div className="history-list">
              {frameHistory.slice(0, 10).map((frame) => (
                <div key={`${frame.frameNumber}-${frame.timestamp}`} className="history-item">
                  <span className="history-frame">F#{frame.frameNumber}</span>
                  <span className="history-hex">{formatHex(frame.syncData, 2)} ... {formatHex(frame.payloadData, 4)}</span>
                  <span className="history-text">{frame.decodedText || '—'}</span>
                  <span className="history-time">{formatTimestamp(frame.timestamp)}</span>
                </div>
              ))}
            </div>
          </div>
        )}
      </div>
    </div>
  );
};

export default MessageDecoder;
