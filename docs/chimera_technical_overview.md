# Chimera Technical Overview

_Chimera_ is a teaching and prototyping environment for modern telemetry links. It combines a GPU-friendly LDPC encoder/decoder, a configurable QPSK framing engine, and an interactive web dashboard that exposes every stage of the transmit (TX) and receive (RX) pipeline. This brief highlights the system architecture, novel properties, and the ways Chimera supports rapid experimentation.

## System architecture

### Framing and modulation (TX)
- **Preset-driven synthesis.** Each frame preset bundles a modulation protocol (symbol rate, interleaving, pilot framing) with LDPC matrices and recommended channel parameters. Switching presets reconfigures the entire pipeline in one step.
- **QPSK mapping.** Plaintext bits are diffused through scrambling and mapped onto QPSK symbols with deterministic pilot placement. The layout is described by the `FrameLayout` object (sync, command, data, ECC partitions).
- **Programmable forward error correction.** Chimera uses quasi-cyclic LDPC codes; degree distributions *(d<sub>v</sub>, d<sub>c</sub>)* can be tuned per preset. The encoder is implemented once in `chimera-core` and re-used by both the CLI and the WASM dashboard.

### Channel modelling
- **Additive white Gaussian noise (AWGN).** Users control the signal-to-noise ratio directly. The mixer displays baseline versus requested SNR to illustrate degradation.
- **Raman feed blending.** Any uploaded audio clip can be folded into the channel as an auxiliary impairment. The non-linear `tanh` vocoder simulates Raman gain spikes and exposes how exotic interference disrupts the telemetry link.
- **Sample-rate adaptation.** External material is re-sampled on the fly to match the simulation sample rate while preserving spectral content.

### Receive and decode (RX)
- **Soft-decision demodulation.** The receiver computes I/Q symbol clouds and soft bit metrics. The constellation and symbol decision panels surface this evidence directly.
- **Iterative LDPC decoding.** Chimera runs belief propagation with run-time configurable iteration limits. Pre- and post-FEC BER statistics quantify error-correction lift.
- **Recovered payload inspection.** The RX stage produces the reconstructed plaintext so learners can visually correlate channel conditions with data integrity.

## Novel properties

1. **End-to-end observability.** Every stage—from plaintext to spectrum—is instrumented. The new pipeline visualiser, dark UI, and FFT panel keep the causal chain visible at all times.
2. **Interactive audio diagnostics.** Users can blend clean carrier, channel noise, and Raman feed while monitoring a live magnitude spectrum. Transport controls maintain deterministic playback, enabling repeated A/B comparisons.
3. **Preset portability.** Because presets encapsulate modulation, LDPC, and simulation defaults, Chimera adapts to new mission profiles without touching the core engine.
4. **Single codebase, multi-target.** `chimera-core` powers the CLI, web dashboard, and future integrations. Shared Rust types keep diagnostic output consistent across targets.
5. **Education-first ergonomics.** Tooltips, descriptive copy, and contextual tags explain DSP jargon. Students can tweak SNR, inspect symbol evidence, and hear channel artefacts in seconds.

## Implementation details

- **Rust workspace.** `chimera-core` hosts the modulation/LDPC logic, while `chimera-web` compiles to WebAssembly via Trunk. The CLI shares the same core for batch experiments.
- **Yew front end.** Function components manage state for presets, simulation runs, and audio transport. The UI communicates with Web Audio API, Plotters, and `rustfft` for visualisations.
- **Plotters canvas charts.** Constellation and FFT displays use `plotters-canvas` for deterministic rendering inside `<canvas>` elements.
- **Web Audio control.** Playback leverages `AudioContext`, `AudioBufferSourceNode`, and `GainNode`. Mixer settings persist across runs; the spectrum panel re-renders whenever the blend changes.
- **Testing hooks.** Acceptance tests live under `chimera-core/tests` and `chimera-web/tests`. The architecture keeps the DSP layer deterministic, so higher-level automation can assert on BER, recovered payloads, and diagnostic traces.

## Advanced Signal Intelligence Characteristics

The Chimera framework, particularly through its Raman feed modeling, allows for the exploration of advanced communication concepts typically associated with signal intelligence (SIGINT). The following sections detail two such areas: achieving a low-detection-footprint and employing unconventional modulation for psychoacoustic effect.

### Low Probability of Intercept / Low Probability of Detection (LPI/LPD)

A key goal in many strategic communication systems is to transmit information without revealing the sender's presence or the signal's existence. Chimera's architecture allows for simulating techniques that achieve this:

-   **Signal Obscuration:** By blending the telemetry signal with a noise-like source (the Raman feed), the energy can be spread, making it difficult to distinguish from the ambient noise floor. This is analogous to real-world systems where signals are buried deep within natural electromagnetic noise, requiring specialized receivers and long integration times to extract.
-   **Exotic Carriers:** While Chimera operates in the audio band, the principles scale to more exotic physical layers. For instance, a narrow-beam THz carrier, as conceptually modeled by the focused nature of the simulation, makes detection possible only if a receiver is placed directly in the signal path with prior knowledge of its frequency and timing.

### Psychoacoustic Modulation and Strategic Parallels

The modulation scheme within Chimera is not optimized for high data throughput but for its potential impact on a receiver, a principle with parallels to specialized military systems like submarine ELF (Extremely Low Frequency) communications.

-   **Strategic "Bell-Ringer" Analogy:** ELF systems are not used for conversation but to send a simple, one-way, high-priority command to a submerged asset (e.g., "surface"). Similarly, the low-rate, complex modulation in Chimera can be seen as a "strategic nudge"—a signal designed to impart a small amount of critical information into a difficult-to-reach environment, prioritizing penetration and stealth over bandwidth.
-   **Psychoacoustic Optimization:** The protocol's use of a hybrid QPSK/FSK nested scheme is inefficient from a data-rate perspective. However, its purpose is not conventional communication but the creation of a complex auditory texture. This shifts the design goal from maximizing bits-per-second to achieving a specific psychoacoustic or neurological effect, where the signal is intended to be processed subconsciously rather than decoded as explicit data. This represents a paradigm shift from optimizing for *efficiency* to optimizing for *impact*.

## Future directions

- Integrate adaptive modulation presets (e.g., 8-PSK, 16-QAM) to broaden comparative studies.
- Add waterfall BER curves that stream results across multiple SNR sweeps.
- Introduce collaborative lesson scripts so instructors can preload channel scenarios and guided questions.

Chimera’s unique blend of rigorous LDPC telemetry and approachable, real-time instrumentation makes it a practical bridge between textbook theory and live-signal intuition.

