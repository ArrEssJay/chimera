/**
 * Clipping Detection Test Utility
 * 
 * Analyzes WAV files for clipping (samples > 1.0 or < -1.0)
 * and reports detailed statistics about audio levels.
 * 
 * Usage:
 *   node test-clipping.js [file1.wav] [file2.wav] ...
 *   node test-clipping.js  (checks all .wav files in current directory)
 */

import wavefile from 'wavefile';
const { WaveFile } = wavefile;
import fs from 'fs';
import path from 'path';

/**
 * Analyze a WAV file for clipping and level statistics
 * @param {string} filename - Path to WAV file
 * @returns {Object} Analysis results
 */
function analyzeWavFile(filename) {
  const wav = new WaveFile();
  const buffer = fs.readFileSync(filename);
  wav.fromBuffer(buffer);
  
  // Get samples as float array
  const samples = wav.getSamples(false, Float32Array);
  
  let clippedCount = 0;
  let maxPeak = 0;
  let minPeak = 0;
  let sumSquares = 0;
  
  // Gap detection
  let gapCount = 0;
  let longestGap = 0;
  let currentGap = 0;
  const ZERO_THRESHOLD = 0.0001; // Consider values below this as zero
  
  for (let i = 0; i < samples.length; i++) {
    const sample = samples[i];
    
    // Check for clipping (values exceeding ±1.0)
    if (sample > 1.0 || sample < -1.0) {
      clippedCount++;
    }
    
    // Track peaks
    if (sample > maxPeak) maxPeak = sample;
    if (sample < minPeak) minPeak = sample;
    
    // Accumulate for RMS calculation
    sumSquares += sample * sample;
    
    // Gap detection - check for consecutive near-zero samples
    if (Math.abs(sample) < ZERO_THRESHOLD) {
      currentGap++;
      if (currentGap > longestGap) {
        longestGap = currentGap;
      }
    } else {
      if (currentGap > 100) { // Only count significant gaps (>100 samples = ~2ms at 48kHz)
        gapCount++;
      }
      currentGap = 0;
    }
  }
  
  const rms = Math.sqrt(sumSquares / samples.length);
  const rmsDb = 20 * Math.log10(rms);
  const peakDb = 20 * Math.log10(Math.max(Math.abs(maxPeak), Math.abs(minPeak)));
  const crestFactor = (Math.max(Math.abs(maxPeak), Math.abs(minPeak))) / rms;
  const clippingPercentage = (clippedCount / samples.length) * 100;
  
  return {
    filename: path.basename(filename),
    totalSamples: samples.length,
    clippedCount,
    clippingPercentage,
    maxPeak,
    minPeak,
    peakDb,
    rms,
    rmsDb,
    crestFactor,
    hasClipping: clippedCount > 0,
    headroom: 20 * Math.log10(1.0 / Math.max(Math.abs(maxPeak), Math.abs(minPeak))),
    gapCount,
    longestGap,
    hasGaps: gapCount > 0 || longestGap > 100
  };
}

/**
 * Format analysis results for display
 * @param {Object} results - Analysis results
 * @returns {string} Formatted output
 */
function formatResults(results) {
  const status = results.hasClipping ? '❌ CLIPPING' : results.hasGaps ? '⚠️  GAPS' : '✅ OK';
  const basename = results.filename;
  
  let output = `\n${status} ${basename}\n`;
  output += `  Peak Level: ${results.maxPeak.toFixed(6)} / ${results.minPeak.toFixed(6)} (${results.peakDb.toFixed(2)} dBFS)\n`;
  output += `  RMS Level: ${results.rms.toFixed(6)} (${results.rmsDb.toFixed(2)} dBFS)\n`;
  output += `  Crest Factor: ${results.crestFactor.toFixed(2)}\n`;
  output += `  Headroom: ${results.headroom >= 0 ? results.headroom.toFixed(2) : '⚠️  ' + results.headroom.toFixed(2)} dB\n`;
  
  if (results.hasClipping) {
    output += `  ⚠️  Clipped Samples: ${results.clippedCount.toLocaleString()} (${results.clippingPercentage.toFixed(4)}%)\n`;
  }
  
  if (results.hasGaps) {
    output += `  ⚠️  Audio Gaps Detected: ${results.gapCount} gaps, longest: ${results.longestGap} samples (${(results.longestGap / 48000 * 1000).toFixed(2)}ms)\n`;
  }
  
  return output;
}

/**
 * Generate summary statistics
 * @param {Array<Object>} allResults - Array of all analysis results
 * @returns {string} Summary output
 */
function generateSummary(allResults) {
  const totalFiles = allResults.length;
  const clippedFiles = allResults.filter(r => r.hasClipping).length;
  const gapFiles = allResults.filter(r => r.hasGaps).length;
  const cleanFiles = totalFiles - clippedFiles - gapFiles;
  
  const avgHeadroom = allResults.reduce((sum, r) => sum + r.headroom, 0) / totalFiles;
  const minHeadroom = Math.min(...allResults.map(r => r.headroom));
  const maxPeakOverall = Math.max(...allResults.map(r => Math.max(Math.abs(r.maxPeak), Math.abs(r.minPeak))));
  
  let output = '\n' + '='.repeat(60) + '\n';
  output += 'SUMMARY\n';
  output += '='.repeat(60) + '\n';
  output += `Total Files Analyzed: ${totalFiles}\n`;
  output += `Clean Files: ${cleanFiles} (${(cleanFiles / totalFiles * 100).toFixed(1)}%)\n`;
  output += `Files with Gaps: ${gapFiles} (${(gapFiles / totalFiles * 100).toFixed(1)}%)\n`;
  output += `Clipped Files: ${clippedFiles} (${(clippedFiles / totalFiles * 100).toFixed(1)}%)\n`;
  output += `Average Headroom: ${avgHeadroom.toFixed(2)} dB\n`;
  output += `Minimum Headroom: ${minHeadroom.toFixed(2)} dB\n`;
  output += `Maximum Peak: ${maxPeakOverall.toFixed(6)} (${(20 * Math.log10(maxPeakOverall)).toFixed(2)} dBFS)\n`;
  
  if (gapFiles > 0) {
    output += `\n⚠️  WARNING: ${gapFiles} file(s) have audio gaps!\n`;
    output += 'Files with gaps:\n';
    allResults.filter(r => r.hasGaps).forEach(r => {
      output += `  - ${r.filename}: ${r.gapCount} gaps, longest ${r.longestGap} samples\n`;
    });
  }
  
  if (clippedFiles > 0) {
    output += `\n⚠️  WARNING: ${clippedFiles} file(s) have clipping!\n`;
    output += 'Clipped files:\n';
    allResults.filter(r => r.hasClipping).forEach(r => {
      output += `  - ${r.filename}: ${r.clippedCount.toLocaleString()} samples clipped\n`;
    });
  }
  
  if (cleanFiles === totalFiles) {
    output += `\n✅ All files are clean (no clipping or gaps detected)\n`;
  }
  
  return output;
}

// Main execution
async function main() {
  console.log('='.repeat(60));
  console.log('Chimera Audio Clipping Detection Test');
  console.log('='.repeat(60));
  
  // Get list of files to analyze
  let files = process.argv.slice(2);
  
  if (files.length === 0) {
    // No files specified, scan current directory for .wav files
    console.log('No files specified. Scanning for .wav files...\n');
    const allFiles = fs.readdirSync('.');
    files = allFiles.filter(f => f.endsWith('.wav'));
  }
  
  if (files.length === 0) {
    console.error('No WAV files found.');
    process.exit(1);
  }
  
  console.log(`Analyzing ${files.length} file(s)...\n`);
  
  // Analyze each file
  const allResults = [];
  for (const file of files) {
    try {
      const results = analyzeWavFile(file);
      allResults.push(results);
      console.log(formatResults(results));
    } catch (error) {
      console.error(`Error analyzing ${file}: ${error.message}`);
    }
  }
  
  // Print summary
  if (allResults.length > 0) {
    console.log(generateSummary(allResults));
  }
  
  // Exit with error code if any clipping or gaps detected
  const hasAnyIssues = allResults.some(r => r.hasClipping || r.hasGaps);
  process.exit(hasAnyIssues ? 1 : 0);
}

main();
