use crate::model::{
    run_pipeline, SimulationInput, SimulationOutput as PipelineOutput, FIXED_SAMPLE_RATE,
};
use crate::presets::FramePreset;
use chimera_core::diagnostics::{FrameDescriptor, SymbolDecision};
use gloo_file::callbacks::{read_as_data_url, FileReader};
use gloo_file::Blob;
// Lightweight SVG generation is used for charts in the web UI. The previous
// implementation relied on `plotters` SVG backend which pulled in native
// font/IO dependencies that are incompatible with the WebAssembly target.
// We now generate minimal SVG markup directly to ensure deterministic
// rendering in the browser without requiring system fonts.
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;
use rustfft::FftPlanner;
use std::f64::consts::FRAC_1_SQRT_2;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use gloo_timers::callback::Timeout;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{AudioBufferSourceNode, AudioContext, Document, Event, HtmlElement};
use yew::events::InputEvent;
use yew::prelude::*;
use yew::TargetCast;

#[derive(Clone, PartialEq)]
enum AudioPlaybackState {
    Stopped,
    PlayingClean,
    PlayingNoisy,
}

#[function_component(App)]
pub fn app() -> Html {
    let simulation = use_state(SimulationInput::default);
    let output = use_state(|| None::<PipelineOutput>);
    let is_running = use_state(|| false);
    let external_audio_name = use_state(|| None::<String>);
    let reader_handle = use_mut_ref(|| None::<FileReader>);
    let last_run_input = use_state(|| None::<SimulationInput>);
    let audio_playback_state = use_state(|| AudioPlaybackState::Stopped);
    let audio_source_node = use_mut_ref(|| None::<AudioBufferSourceNode>);
    let audio_context = use_mut_ref(|| None::<AudioContext>);
    let audio_gain = use_state(|| 0.5_f64);

    let current_input = (*simulation).clone();
    let preset_bundle = current_input.preset.bundle();
    let frame_layout = preset_bundle.protocol.frame_layout;

    // Check if there are pending changes (simulation input differs from last run)
    let has_pending_changes = (*last_run_input).as_ref() != Some(&current_input);

    let on_preset_change = {
        let simulation = simulation.clone();
        let external_audio_name = external_audio_name.clone();
        Callback::from(move |event: Event| {
            if let Some(select) = event.target_dyn_into::<web_sys::HtmlSelectElement>() {
                if let Some(preset) = FramePreset::from_key(&select.value()) {
                    let defaults = preset.simulation_config();
                    let mut next = (*simulation).clone();
                    next.preset = preset;
                    next.plaintext = defaults.plaintext_source;
                    next.snr_db = defaults.snr_db;
                    next.link_loss_db = defaults.link_loss_db;
                    simulation.set(next);
                    external_audio_name.set(None);
                }
            }
        })
    };

    let on_plaintext_change = {
        let simulation = simulation.clone();
        Callback::from(move |event: InputEvent| {
            if let Some(textarea) = event.target_dyn_into::<web_sys::HtmlTextAreaElement>() {
                let mut next = (*simulation).clone();
                next.plaintext = textarea.value();
                simulation.set(next);
            }
        })
    };

    let on_snr_change = {
        let simulation = simulation.clone();
        Callback::from(move |event: InputEvent| {
            if let Some(input) = event.target_dyn_into::<web_sys::HtmlInputElement>() {
                if let Ok(value) = input.value().parse::<f64>() {
                    let mut next = (*simulation).clone();
                    next.snr_db = value;
                    simulation.set(next);
                }
            }
        })
    };

    let on_link_loss_change = {
        let simulation = simulation.clone();
        Callback::from(move |event: InputEvent| {
            if let Some(input) = event.target_dyn_into::<web_sys::HtmlInputElement>() {
                if let Ok(value) = input.value().parse::<f64>() {
                    let mut next = (*simulation).clone();
                    next.link_loss_db = value;
                    simulation.set(next);
                }
            }
        })
    };

    let on_audio_upload = {
        let simulation = simulation.clone();
        let reader_handle = reader_handle.clone();
        let external_audio_name = external_audio_name.clone();
        Callback::from(move |event: Event| {
            if let Some(input) = event.target_dyn_into::<web_sys::HtmlInputElement>() {
                if let Some(files) = input.files() {
                    if let Some(file) = files.get(0) {
                        let file_name = file.name();
                        if file.size() > 512_000.0 {
                            web_sys::console::warn_1(
                                &format!(
                                    "Audio file '{}' is larger than 512 KB; skipping upload.",
                                    file_name
                                )
                                .into(),
                            );
                        } else {
                            let blob = Blob::from(file);
                            let simulation_handle = simulation.clone();
                            let audio_name_handle = external_audio_name.clone();
                            let name_clone = file_name.clone();
                            let handle = read_as_data_url(&blob, move |result| {
                                if let Ok(data_url) = result {
                                    if let Some(encoded) = data_url.split(',').nth(1) {
                                        let mut next = (*simulation_handle).clone();
                                        let payload_name = name_clone.clone();
                                        let payload =
                                            format!("AUDIO:{}:{}", payload_name.as_str(), encoded);
                                        next.plaintext = payload;
                                        simulation_handle.set(next);
                                        audio_name_handle.set(Some(payload_name));
                                    }
                                }
                            });
                            *reader_handle.borrow_mut() = Some(handle);
                        }
                    }
                }
                input.set_value("");
            }
        })
    };

    let on_audio_clear = {
        let simulation = simulation.clone();
        let external_audio_name = external_audio_name.clone();
        Callback::from(move |_event: MouseEvent| {
            let mut next = (*simulation).clone();
            let defaults = next.preset.simulation_config();
            next.plaintext = defaults.plaintext_source;
            simulation.set(next);
            external_audio_name.set(None);
        })
    };

    let on_run = {
        let simulation_handle = simulation.clone();
        let output_handle = output.clone();
        let running_handle = is_running.clone();
        let last_run_handle = last_run_input.clone();
        let audio_playback = audio_playback_state.clone();
        let audio_source = audio_source_node.clone();
        Callback::from(move |_event: MouseEvent| {
            web_sys::console::info_1(&"on_run handler invoked".into());
            if *running_handle {
                return;
            }
            // Stop any currently playing audio before running new simulation
            stop_audio(&audio_source, &audio_playback);

            running_handle.set(true);
            let input = (*simulation_handle).clone();
            let output_state = output_handle.clone();
            let running_state = running_handle.clone();
            let last_run_state = last_run_handle.clone();
            let input_clone = input.clone();
                spawn_local(async move {
                    web_sys::console::info_1(&"Run Now clicked: starting pipeline".into());
                    // run_pipeline is synchronous CPU-bound; log around it so we can see progress
                    let result = run_pipeline(input);
                    web_sys::console::info_1(&"Pipeline completed: setting output".into());
                    output_state.set(Some(result));
                    running_state.set(false);
                    last_run_state.set(Some(input_clone));
                    web_sys::console::info_1(&"Output stored and UI state updated".into());
                });
        })
    };

    let on_play_clean = {
        let output_handle = output.clone();
        let audio_playback = audio_playback_state.clone();
        let audio_source = audio_source_node.clone();
        let audio_ctx = audio_context.clone();
        let gain_handle = audio_gain.clone();
        Callback::from(move |_event: MouseEvent| {
            if let Some(out) = (*output_handle).as_ref() {
                if let Some(ref audio) = out.diagnostics.modulation_audio {
                    stop_audio(&audio_source, &audio_playback);
                    play_audio(
                        &audio.clean,
                        audio.sample_rate,
                        &audio_source,
                        &audio_ctx,
                        &audio_playback,
                        AudioPlaybackState::PlayingClean,
                        *gain_handle,
                    );
                }
            }
        })
    };

    let on_play_noisy = {
        let output_handle = output.clone();
        let audio_playback = audio_playback_state.clone();
        let audio_source = audio_source_node.clone();
        let audio_ctx = audio_context.clone();
        let gain_handle = audio_gain.clone();
        Callback::from(move |_event: MouseEvent| {
            if let Some(out) = (*output_handle).as_ref() {
                if let Some(ref audio) = out.diagnostics.modulation_audio {
                    stop_audio(&audio_source, &audio_playback);
                    play_audio(
                        &audio.noisy,
                        audio.sample_rate,
                        &audio_source,
                        &audio_ctx,
                        &audio_playback,
                        AudioPlaybackState::PlayingNoisy,
                        *gain_handle,
                    );
                }
            }
        })
    };

    let on_stop_audio = {
        let audio_playback = audio_playback_state.clone();
        let audio_source = audio_source_node.clone();
        Callback::from(move |_event: MouseEvent| {
            stop_audio(&audio_source, &audio_playback);
        })
    };

    let on_gain_change = {
        let gain_handle = audio_gain.clone();
        Callback::from(move |event: InputEvent| {
            if let Some(input) = event.target_dyn_into::<web_sys::HtmlInputElement>() {
                if let Ok(value) = input.value().parse::<f64>() {
                    gain_handle.set(value);
                }
            }
        })
    };

    let pipeline_output = (*output).as_ref();

    let (
        tx_i,
        tx_q,
        rx_i,
        rx_q,
        clean_baseband,
        noisy_baseband,
        tx_bits,
        symbol_decisions,
        timing_error,
        freq_offset,
        encoding_logs,
        decoding_logs,
        frames,
    ) = if let Some(out) = pipeline_output {
        let diag = &out.diagnostics;
        (
            diag.tx_symbols_i.clone(),
            diag.tx_symbols_q.clone(),
            diag.demodulation.received_symbols_i.clone(),
            diag.demodulation.received_symbols_q.clone(),
            diag.clean_baseband.clone(),
            diag.noisy_baseband.clone(),
            diag.tx_bits.clone(),
            diag.demodulation.symbol_decisions.clone(),
            diag.demodulation.timing_error.clone(),
            diag.demodulation.nco_freq_offset.clone(),
            diag.encoding_logs.clone(),
            diag.decoding_logs.clone(),
            diag.frames.clone(),
        )
    } else {
        (
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::<SymbolDecision>::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::<FrameDescriptor>::new(),
        )
    };

    let psd_clean = compute_psd(&clean_baseband, FIXED_SAMPLE_RATE);
    let psd_noisy = compute_psd(&noisy_baseband, FIXED_SAMPLE_RATE);
    let ber_trend = compute_ber_trend(&tx_bits, &symbol_decisions);

    let report = pipeline_output.map(|out| out.report.clone());
    let modulation_audio = pipeline_output.and_then(|out| out.diagnostics.modulation_audio.clone());

    let recovered_message = report
        .as_ref()
        .map(|rep| rep.recovered_message.clone())
        .unwrap_or_default();
    let plaintext_len = current_input.plaintext.chars().count();
    let audio_file_name = (*external_audio_name).clone();

    html! {
        <main>
            <header class="app-header">
                <div class="header-content">
                    <h1 class="logo-title">{"🔮 CHIMERA"}</h1>
                    <p class="logo-subtitle">{"Advanced Low Probability of Intercept & Detection Signal Processing Training"}</p>
                </div>
                <div class="help-hint">
                    <span class="help-icon">{"ℹ️"}</span>
                    <span>{"Configure parameters below, then click \"Run Now\" to execute the simulation"}</span>
                </div>
            </header>
            <div class="main-grid">
                <section class="panel controls-panel">
                    <header class="panel-header">
                        <div class="run-controls">
                            {
                                if *is_running {
                                    html! { <span class="badge badge-live">{"Running…"}</span> }
                                } else if has_pending_changes {
                                    html! { <span class="badge badge-pending">{"Changes pending"}</span> }
                                } else {
                                    html! { <span class="badge badge-live idle">{"Up to date"}</span> }
                                }
                            }
                            <button
                                class={if has_pending_changes && !*is_running { "primary highlight" } else { "primary" }}
                                onclick={on_run.clone()}
                                disabled={*is_running}
                            >
                                { if *is_running { "Running…" } else { "Run Now" } }
                            </button>
                        </div>
                    </header>

                    <div class="control-grid">
                        <label class="field">
                            <span
                                class="info-rollover"
                                data-tooltip="Selects a preconfigured link budget and frame layout profile."
                                title="Selects a preconfigured link budget and frame layout profile."
                                tabindex="0"
                            >
                                {"Preset"}
                            </span>
                            <select value={current_input.preset.key()} onchange={on_preset_change}>
                                { for FramePreset::all().iter().map(|preset| {
                                    let key = preset.key();
                                    let name = preset.display_name();
                                    html! { <option value={key} selected={current_input.preset == *preset}>{name}</option> }
                                }) }
                            </select>
                            <p class="muted">{current_input.preset.description()}</p>
                        </label>

                        <label class="field">
                            <span
                                class="info-rollover"
                                data-tooltip="Text payload that will be encoded into frames prior to modulation."
                                title="Text payload that will be encoded into frames prior to modulation."
                                tabindex="0"
                            >
                                {"Plaintext"}
                            </span>
                            <textarea value={current_input.plaintext.clone()} oninput={on_plaintext_change} />
                            <p class="muted">{format!("{} chars", plaintext_len)}</p>
                        </label>

                        <label class="field">
                            <span
                                class="info-rollover"
                                data-tooltip="Adjusts the additive white Gaussian noise level applied before decoding."
                                title="Adjusts the additive white Gaussian noise level applied before decoding."
                                tabindex="0"
                            >
                                {"Channel SNR (dB)"}
                            </span>
                            <input type="number" min="-30" max="0" step="0.5" value={format!("{:.2}", current_input.snr_db)} oninput={on_snr_change} />
                            <p class="muted small">
                                {"AWGN noise level (Es/N₀). System achieves ~35 dB processing gain through averaging. LDPC fails below -27 dB channel SNR. "}
                                <a href="https://github.com/ArrEssJay/chimera/blob/main/docs/signal_processing_concepts.md#energy-ratios-esn0-and-ebn0" target="_blank" rel="noopener noreferrer">{"Learn about Es/N₀"}</a>
                            </p>
                        </label>

                        <label class="field">
                            <span title="Path loss and signal attenuation - deterministic reduction in signal power from transmission">{"Link Loss (dB)"}</span>
                            <input type="number" min="0" max="150" step="1" value={format!("{:.1}", current_input.link_loss_db)} oninput={on_link_loss_change} />
                            <p class="muted small">
                                {"Signal attenuation from path loss, antenna gains, etc. Typical radio links have 100+ dB loss. "}
                                <a href="https://github.com/ArrEssJay/chimera/blob/main/docs/signal_processing_concepts.md#link-loss-vs-noise" target="_blank" rel="noopener noreferrer">{"Learn about link loss"}</a>
                            </p>
                        </label>

                        <div class="field">
                            <span
                                class="info-rollover"
                                data-tooltip="Upload a small WAV or MP3 file to embed as a base64 payload across frames."
                                title="Upload a small WAV or MP3 file to embed as a base64 payload across frames."
                                tabindex="0"
                            >
                                {"External Audio Payload"}
                            </span>
                            <input type="file" accept="audio/*" onchange={on_audio_upload} />
                            <div class="audio-actions">
                                {
                                    if let Some(name) = audio_file_name {
                                        html! {
                                            <>
                                                <span class="muted small">{format!("Attached: {} (embedded as base64)", name)}</span>
                                                <button class="ghost" type="button" onclick={on_audio_clear.clone()}>{"Remove audio"}</button>
                                            </>
                                        }
                                    } else {
                                        html! { <span class="muted small">{"Attach a small WAV/MP3 to embed directly into the framed payload."}</span> }
                                    }
                                }
                            </div>
                        </div>
                    </div>
                </section>

                <section class="panel telemetry-panel">
                    <header>
                        <h2>{"Frame Telemetry"}</h2>
                        <p class="muted">
                            {"Live metrics from the most recent pipeline execution. "}
                            <a href="https://github.com/ArrEssJay/chimera/blob/main/docs/signal_processing_concepts.md#bit-error-rate-ber" target="_blank" rel="noopener noreferrer">{"Learn about BER"}</a>
                            {" | "}
                            <a href="https://github.com/ArrEssJay/chimera/blob/main/docs/signal_processing_concepts.md#forward-error-correction-fec" target="_blank" rel="noopener noreferrer">{"Learn about FEC"}</a>
                        </p>
                    </header>
                    {
                        if let Some(ref report) = report {
                            html! {
                                <div class="metrics-grid">
                                    <div class="metric">
                                        <span
                                            class="label info-rollover"
                                            data-tooltip="Bit-error ratio measured before the LDPC decoder applies forward error correction."
                                            title="Bit-error ratio measured before the LDPC decoder applies forward error correction."
                                            tabindex="0"
                                        >
                                            {"Pre-FEC BER"}
                                        </span>
                                        <span class="value">{format_sci(report.pre_fec_ber)}</span>
                                        <span class="detail">{format!("{} symbol errors", report.pre_fec_errors)}</span>
                                    </div>
                                    <div class="metric">
                                        <span
                                            class="label info-rollover"
                                            data-tooltip="Residual bit-error ratio after LDPC decoding and error correction."
                                            title="Residual bit-error ratio after LDPC decoding and error correction."
                                            tabindex="0"
                                        >
                                            {"Post-FEC BER"}
                                        </span>
                                        <span class="value">{format_sci(report.post_fec_ber)}</span>
                                        <span class="detail">{format!("{} residual errors", report.post_fec_errors)}</span>
                                    </div>
                                    <div class="metric">
                                        <span
                                            class="label info-rollover"
                                            data-tooltip="Decoded plaintext recovered from the LDPC decoder and descrambler."
                                            title="Decoded plaintext recovered from the LDPC decoder and descrambler."
                                            tabindex="0"
                                        >
                                            {"Recovered Message"}
                                        </span>
                                        <span class="value value-long">{&report.recovered_message}</span>
                                    </div>
                                    if let Some(ref audio) = modulation_audio {
                                        <div class="metric">
                                            <span
                                                class="label info-rollover"
                                                data-tooltip="Synthetic audio preview generated from the complex baseband waveform."
                                                title="Synthetic audio preview generated from the complex baseband waveform."
                                                tabindex="0"
                                            >
                                                {"Modulation Audio"}
                                            </span>
                                            <span class="value">{format!("{} Hz", audio.sample_rate)}</span>
                                            <span class="detail">{format!("Carrier {:.1} Hz", audio.carrier_freq_hz)}</span>
                                        </div>
                                        <div class="metric audio-controls">
                                            <span class="label">{"Audio Playback"}</span>
                                            <div class="audio-buttons">
                                                <button
                                                    class={if *audio_playback_state == AudioPlaybackState::PlayingClean { "primary active" } else { "primary" }}
                                                    onclick={on_play_clean.clone()}
                                                    disabled={*audio_playback_state == AudioPlaybackState::PlayingClean}
                                                >
                                                    {"▶ Play Clean"}
                                                </button>
                                                <button
                                                    class={if *audio_playback_state == AudioPlaybackState::PlayingNoisy { "primary active" } else { "primary" }}
                                                    onclick={on_play_noisy.clone()}
                                                    disabled={*audio_playback_state == AudioPlaybackState::PlayingNoisy}
                                                >
                                                    {"▶ Play Noisy"}
                                                </button>
                                                <button
                                                    class="ghost"
                                                    onclick={on_stop_audio.clone()}
                                                    disabled={*audio_playback_state == AudioPlaybackState::Stopped}
                                                >
                                                    {"⏹ Stop"}
                                                </button>
                                            </div>
                                            <div class="volume-control">
                                                <label class="field">
                                                    <span>{"Volume"}</span>
                                                    <input
                                                        type="range"
                                                        min="0"
                                                        max="1"
                                                        step="0.01"
                                                        value={format!("{:.2}", *audio_gain)}
                                                        oninput={on_gain_change}
                                                    />
                                                    <span class="detail">{format!("{}%", (*audio_gain * 100.0) as i32)}</span>
                                                </label>
                                            </div>
                                        </div>
                                    }
                                </div>
                            }
                        } else {
                            html! { <p class="muted">{"Run the simulation to populate telemetry data."}</p> }
                        }
                    }
                </section>

                <section class="pipeline-section">
                    <div class="node-graph">
                        <div class="node-column">
                            <div class="node">
                                <h3>{"Input"}</h3>
                                <p
                                    class="info-rollover"
                                    data-tooltip="Total characters currently staged for transmission in the payload field."
                                    title="Total characters currently staged for transmission in the payload field."
                                    tabindex="0"
                                >
                                    {format!("Payload: {} chars", plaintext_len)}
                                </p>
                                <p
                                    class="info-rollover"
                                    data-tooltip="Energy per symbol to noise-power spectral density ratio applied ahead of receiver processing."
                                    title="Energy per symbol to noise-power spectral density ratio applied ahead of receiver processing."
                                    tabindex="0"
                                >
                                    {format!("Es/N₀: {:.1} dB", current_input.snr_db)}
                                </p>
                            </div>
                        </div>
                        <div class="node-column">
                            <div class="node">
                                <h3>{"Encoder"}</h3>
                                <p
                                    class="info-rollover"
                                    data-tooltip="Total QPSK symbols per frame including sync, payload, and parity symbols."
                                    title="Total QPSK symbols per frame including sync, payload, and parity symbols."
                                    tabindex="0"
                                >
                                    {format!("Total symbols: {}", frame_layout.total_symbols)}
                                </p>
                                <p
                                    class="info-rollover"
                                    data-tooltip="Symbols dedicated to framing the user payload before forward-error correction."
                                    title="Symbols dedicated to framing the user payload before forward-error correction."
                                    tabindex="0"
                                >
                                    {format!("Payload symbols: {}", frame_layout.data_payload_symbols)}
                                </p>
                                <p
                                    class="info-rollover"
                                    data-tooltip="Parity symbols generated by the LDPC encoder to enable error correction."
                                    title="Parity symbols generated by the LDPC encoder to enable error correction."
                                    tabindex="0"
                                >
                                    {format!("ECC symbols: {}", frame_layout.ecc_symbols)}
                                </p>
                                <p class="muted small">
                                    <a href="https://github.com/ArrEssJay/chimera/blob/main/docs/signal_processing_concepts.md#symbols" target="_blank" rel="noopener noreferrer">{"What are symbols?"}</a>
                                </p>
                            </div>
                            <div class="node">
                                <h3>{"Transmitter"}</h3>
                                <ConstellationChart
                                    title="TX Symbols"
                                    i_samples={tx_i.clone()}
                                    q_samples={tx_q.clone()}
                                    variant={ConstellationVariant::Tx}
                                    tooltip={Some(AttrValue::from("Transmitted QPSK symbols prior to channel noise and impairment injection."))}
                                />
                                <p class="muted small">
                                    {"Ideal QPSK constellation produced by the framing encoder. "}
                                    <a href="https://github.com/ArrEssJay/chimera/blob/main/docs/signal_processing_concepts.md#constellation-diagrams" target="_blank" rel="noopener noreferrer">{"Learn about constellations"}</a>
                                    {" | "}
                                    <a href="https://github.com/ArrEssJay/chimera/blob/main/docs/signal_processing_concepts.md#qpsk-modulation" target="_blank" rel="noopener noreferrer">{"Learn about QPSK"}</a>
                                </p>
                            </div>
                        </div>
                        <div class="node-column">
                            <div class="node">
                                <h3>{"Channel"}</h3>
                                <p
                                    class="info-rollover"
                                    data-tooltip="Center carrier frequency used for QPSK modulation of this preset."
                                    title="Center carrier frequency used for QPSK modulation of this preset."
                                    tabindex="0"
                                >
                                    {format!("Carrier: {:.1} Hz", preset_bundle.protocol.carrier_freq_hz)}
                                </p>
                                <p
                                    class="info-rollover"
                                    data-tooltip="Symbol rate of the quadrature phase-shift keying waveform in symbols per second."
                                    title="Symbol rate of the quadrature phase-shift keying waveform in symbols per second."
                                    tabindex="0"
                                >
                                    {format!("QPSK rate: {} sym/s", preset_bundle.protocol.qpsk_symbol_rate)}
                                </p>
                                <p
                                    class="info-rollover"
                                    data-tooltip="Maximum number of frames allowed in a single transmission burst for this preset."
                                    title="Maximum number of frames allowed in a single transmission burst for this preset."
                                    tabindex="0"
                                >
                                    {format!("Frame ceiling: {}", preset_bundle.protocol.max_frames)}
                                </p>
                                <p class="muted small">
                                    <a href="https://github.com/ArrEssJay/chimera/blob/main/docs/signal_processing_concepts.md#link-loss-vs-noise" target="_blank" rel="noopener noreferrer">{"Learn about link loss & noise"}</a>
                                </p>
                            </div>
                        </div>
                        <div class="node-column">
                            <div class="node">
                                <h3>{"Receiver"}</h3>
                                <ConstellationChart
                                    title="RX Symbols"
                                    i_samples={rx_i.clone()}
                                    q_samples={rx_q.clone()}
                                    variant={ConstellationVariant::Rx}
                                    tooltip={Some(AttrValue::from("Recovered constellation after receiver timing, carrier, and phase correction."))}
                                />
                                <p class="muted small">
                                    {"Recovered constellation after timing/frequency correction. "}
                                    <a href="https://github.com/ArrEssJay/chimera/blob/main/docs/signal_processing_concepts.md#constellation-diagrams" target="_blank" rel="noopener noreferrer">{"Learn about constellations"}</a>
                                </p>
                            </div>
                            <div class="node">
                                <h3>{"Decoder"}</h3>
                                {
                                    if let Some(ref report) = report {
                                        html! {
                                            <>
                                                <p
                                                    class="info-rollover"
                                                    data-tooltip="Remaining bit errors that persisted after LDPC decoding across the entire burst."
                                                    title="Remaining bit errors that persisted after LDPC decoding across the entire burst."
                                                    tabindex="0"
                                                >
                                                    {format!("Residual errors: {}", report.post_fec_errors)}
                                                </p>
                                                <p
                                                    class="info-rollover"
                                                    data-tooltip="Bit-error ratio after LDPC decoding and frame reassembly."
                                                    title="Bit-error ratio after LDPC decoding and frame reassembly."
                                                    tabindex="0"
                                                >
                                                    {format!("Post-FEC BER: {}", format_sci(report.post_fec_ber))}
                                                </p>
                                            </>
                                        }
                                    } else {
                                        html! { <p class="muted">{"Awaiting run."}</p> }
                                    }
                                }
                            </div>
                        </div>
                        <div class="node-column">
                            <div class="node">
                                <h3>{"Output"}</h3>
                                {
                                    if !recovered_message.is_empty() {
                                        html! { <p>{recovered_message.clone()}</p> }
                                    } else {
                                        html! { <p class="muted">{"Recovered text will appear here."}</p> }
                                    }
                                }
                            </div>
                        </div>
                    </div>
                </section>

                <section class="panel constellation-comparison-panel">
                    <header>
                        <h2>{"Constellation Diagram"}</h2>
                        <p class="muted">{"Combined view of transmitted (TX) and received (RX) QPSK symbols."}</p>
                    </header>
                    <CombinedConstellation
                        title="TX vs RX Constellation"
                        tx_i_samples={tx_i.clone()}
                        tx_q_samples={tx_q.clone()}
                        rx_i_samples={rx_i.clone()}
                        rx_q_samples={rx_q.clone()}
                    />
                </section>

                <section class="panel frame-panel">
                    <header>
                        <h2>{"Frame Inspector"}</h2>
                        <p class="muted">{"Decoded command flags with payload previews."}</p>
                    </header>
                    {
                        if frames.is_empty() {
                            html! { <p class="muted">{"Run the simulation to inspect frame descriptors."}</p> }
                        } else {
                            html! {
                                <div class="frame-table-wrapper">
                                    <table class="frame-table">
                                        <thead>
                                            <tr>
                                                <th class="info-rollover" data-tooltip="Ordinal position of this frame within the burst." title="Ordinal position of this frame within the burst." tabindex="0">{"Index"}</th>
                                                <th class="info-rollover" data-tooltip="Human-readable label describing the frame type." title="Human-readable label describing the frame type." tabindex="0">{"Label"}</th>
                                                <th class="info-rollover" data-tooltip="Operational opcode embedded in the command word for this frame." title="Operational opcode embedded in the command word for this frame." tabindex="0">{"Opcode"}</th>
                                                <th class="info-rollover" data-tooltip="Full command word including frame counters and addressing information." title="Full command word including frame counters and addressing information." tabindex="0">{"Command Word"}</th>
                                                <th class="info-rollover" data-tooltip="Hex preview of the frame payload contents (truncated)." title="Hex preview of the frame payload contents (truncated)." tabindex="0">{"Payload Preview"}</th>
                                            </tr>
                                        </thead>
                                        <tbody>
                                            { for frames.iter().map(|desc| {
                                                html! {
                                                    <tr>
                                                        <td>{format!("{}/{}", desc.frame_index + 1, desc.total_frames)}</td>
                                                        <td><span class="tag">{desc.frame_label.clone()}</span></td>
                                                        <td>{format_opcode_label(desc.command_opcode)}</td>
                                                        <td>{format_command_word_label(desc)}</td>
                                                        <td class="payload-cell">{desc.payload_preview.clone()}</td>
                                                    </tr>
                                                }
                                            }) }
                                        </tbody>
                                    </table>
                                </div>
                            }
                        }
                    }
                </section>

                <section class="panel diagnostics-panel">
                    <header>
                        <h2>{"Diagnostics"}</h2>
                        <p class="muted">
                            {"Analyzer outputs from the demodulation loop. "}
                            <a href="https://github.com/ArrEssJay/chimera/blob/main/docs/signal_processing_concepts.md" target="_blank" rel="noopener noreferrer">{"Signal Processing Concepts Guide"}</a>
                        </p>
                    </header>
                    <div class="chart-grid">
                        <LineChart
                            title="Timing Error"
                            values={timing_error.clone()}
                            accent_rgb={Some((94, 214, 255))}
                            x_label="Sample Index"
                            y_label="Error (samples)"
                            tooltip={Some(AttrValue::from("Timing-loop error for each processed symbol, expressed in fractional samples."))}
                        />
                        <LineChart
                            title="NCO Frequency Offset"
                            values={freq_offset.clone()}
                            accent_rgb={Some((255, 168, 112))}
                            x_label="Sample Index"
                            y_label="Offset (Hz)"
                            tooltip={Some(AttrValue::from("Residual carrier offset tracked by the numerically controlled oscillator in Hertz."))}
                        />
                        <LineChart
                            title="Clean Signal PSD"
                            values={psd_clean.clone()}
                            accent_rgb={Some((126, 240, 180))}
                            x_label="Frequency Bin"
                            y_label="Power (dBFS)"
                            tooltip={Some(AttrValue::from("Power spectral density of the synthesized clean baseband waveform."))}
                        />
                        <LineChart
                            title="Noisy Signal PSD"
                            values={psd_noisy.clone()}
                            accent_rgb={Some((255, 132, 220))}
                            x_label="Frequency Bin"
                            y_label="Power (dBFS)"
                            tooltip={Some(AttrValue::from("Power spectral density of the received waveform after AWGN injection."))}
                        />
                        <LineChart
                            title="Running BER"
                            values={ber_trend.clone()}
                            accent_rgb={Some((255, 238, 96))}
                            x_label="Symbol Index"
                            y_label="BER"
                            tooltip={Some(AttrValue::from("Cumulative bit-error ratio computed as symbols are demodulated."))}
                        />
                    </div>
                    <div class="log-columns">
                        <div class="log-pane">
                            <h3>{"Encoder Log"}</h3>
                            { render_log(&encoding_logs) }
                        </div>
                        <div class="log-pane">
                            <h3>{"Decoder Log"}</h3>
                            { render_log(&decoding_logs) }
                        </div>
                    </div>
                </section>
            </div>
            <footer class="app-footer">
                <div class="footer-content">
                    <a href="https://github.com/ArrEssJay/chimera/" target="_blank" rel="noopener noreferrer">
                        {"GitHub"}
                    </a>
                    <span class="footer-separator">{"•"}</span>
                    <a href="mailto:rowan@impermanent.io">
                        {"Contact"}
                    </a>
                </div>
            </footer>
        </main>
    }
}

fn render_log(entries: &[String]) -> Html {
    if entries.is_empty() {
        html! { <p class="muted">{"Diagnostics will appear after the auto-preview finishes."}</p> }
    } else {
        let content = entries.join("\n");
        html! { <pre class="log-viewer">{content}</pre> }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum ConstellationVariant {
    Tx,
    Rx,
}

#[derive(Properties, PartialEq)]
pub struct ConstellationProps {
    pub title: AttrValue,
    pub i_samples: Vec<f64>,
    pub q_samples: Vec<f64>,
    #[prop_or(ConstellationVariant::Rx)]
    pub variant: ConstellationVariant,
    #[prop_or_default]
    pub tooltip: Option<AttrValue>,
}

#[derive(Properties, PartialEq)]
pub struct CombinedConstellationProps {
    pub title: AttrValue,
    pub tx_i_samples: Vec<f64>,
    pub tx_q_samples: Vec<f64>,
    pub rx_i_samples: Vec<f64>,
    pub rx_q_samples: Vec<f64>,
}

#[function_component(ConstellationChart)]
pub fn constellation_chart(props: &ConstellationProps) -> Html {
    let is_empty = props.i_samples.is_empty() || props.q_samples.is_empty();
    let tooltip_attr = props.tooltip.clone().unwrap_or_else(|| AttrValue::from(""));
    let panel_class = if props.tooltip.is_some() {
        "constellation-panel panel has-tooltip"
    } else {
        "constellation-panel panel"
    };
    let tab_index = props.tooltip.is_some().then(|| AttrValue::from("0"));
    
    html! {
        <div class={panel_class} data-tooltip={tooltip_attr} tabindex={tab_index}>
            {
                if is_empty {
                    html! { <div class="chart-empty">{"No constellation samples."}</div> }
                } else {
                    let svg = draw_constellation_svg(
                        &props.i_samples,
                        &props.q_samples,
                        props.title.as_str(),
                        props.variant.clone(),
                    );
                    html! {
                        <div class="svg-chart-container">
                            { yew::virtual_dom::VNode::from_html_unchecked(svg.into()) }
                        </div>
                    }
                }
            }
        </div>
    }
}

#[function_component(CombinedConstellation)]
pub fn combined_constellation(props: &CombinedConstellationProps) -> Html {
    let is_empty = (props.tx_i_samples.is_empty() || props.tx_q_samples.is_empty())
        && (props.rx_i_samples.is_empty() || props.rx_q_samples.is_empty());
    
    html! {
        <div class="constellation-panel panel constellation-combined">
            {
                if is_empty {
                    html! { <div class="chart-empty">{"No constellation samples."}</div> }
                } else {
                    let svg = draw_combined_constellation_svg(
                        &props.tx_i_samples,
                        &props.tx_q_samples,
                        &props.rx_i_samples,
                        &props.rx_q_samples,
                        props.title.as_str(),
                    );
                    html! {
                        <div class="svg-chart-container">
                            { yew::virtual_dom::VNode::from_html_unchecked(svg.into()) }
                        </div>
                    }
                }
            }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct LineChartProps {
    pub title: AttrValue,
    pub values: Vec<f64>,
    #[prop_or(None)]
    pub accent_rgb: Option<(u8, u8, u8)>,
    #[prop_or_default]
    pub tooltip: Option<AttrValue>,
    #[prop_or_default]
    pub x_label: AttrValue,
    #[prop_or_default]
    pub y_label: AttrValue,
}

#[function_component(LineChart)]
fn line_chart(props: &LineChartProps) -> Html {
    let is_empty = props.values.is_empty();
    let tooltip_attr = props.tooltip.clone().unwrap_or_else(|| AttrValue::from(""));
    let panel_class = if props.tooltip.is_some() {
        "chart-panel panel has-tooltip"
    } else {
        "chart-panel panel"
    };
    let tab_index = props.tooltip.is_some().then(|| AttrValue::from("0"));
    
    html! {
        <div class={panel_class} data-tooltip={tooltip_attr} tabindex={tab_index}>
            {
                if is_empty {
                    html! { <div class="chart-empty">{"No samples available."}</div> }
                } else {
                    let svg = draw_line_chart_svg(
                        &props.values,
                        props.title.as_str(),
                        props.x_label.as_str(),
                        props.y_label.as_str(),
                        props.accent_rgb,
                    );
                    html! {
                        <div class="svg-chart-container">
                            { yew::virtual_dom::VNode::from_html_unchecked(svg.into()) }
                        </div>
                    }
                }
            }
        </div>
    }
}

fn draw_constellation_svg(
    symbols_i: &[f64],
    symbols_q: &[f64],
    title: &str,
    variant: ConstellationVariant,
) -> String {
    // Filter finite symbol pairs
    let finite: Vec<(f64, f64)> = symbols_i
        .iter()
        .zip(symbols_q.iter())
        .filter_map(|(&i, &q)| {
            if i.is_finite() && q.is_finite() {
                Some((i, q))
            } else {
                None
            }
        })
        .collect();

    web_sys::console::info_1(
        &format!(
            "Drawing constellation '{}' ({:?}) with {} points ({} finite)",
            title,
            variant,
            symbols_i.len(),
            finite.len()
        )
        .into(),
    );

    if finite.is_empty() {
        web_sys::console::warn_1(
            &format!("Skipping constellation '{}' due to lack of finite samples", title).into(),
        );
        return String::new();
    }

    // SVG dimensions and padding
    let width = 400.0;
    let height = 400.0;
    let padding = 30.0;
    let plot_w = width - padding * 2.0;
    let plot_h = height - padding * 2.0;

    // Fixed axis range for QPSK (keep centered around 0)
    let x_min = -1.5;
    let x_max = 1.5;
    let y_min = -1.5;
    let y_max = 1.5;

    let map_x = |x: f64| {
        padding + ((x - x_min) / (x_max - x_min)) * plot_w
    };
    let map_y = |y: f64| {
        // SVG Y axis goes down so invert
        padding + (1.0 - (y - y_min) / (y_max - y_min)) * plot_h
    };

    // Start building simple SVG string (avoid using plotters in wasm)
    let mut svg = String::new();
    svg.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">",
        width, height, width as i32, height as i32
    ));

    // Background
    svg.push_str(&format!(
        "<rect x=\"0\" y=\"0\" width=\"100%\" height=\"100%\" fill=\"transparent\"/>"
    ));

    // Title
    svg.push_str(&format!(
        "<text x=\"{x}\" y=\"{y}\" font-family=\"monospace\" font-size=14 fill=\"#96DC96\">{title}</text>",
        x = padding,
        y = padding - 8.0,
        title = html_escape::encode_text(title)
    ));

    // Axes (center lines)
    let cx = map_x(0.0);
    let cy = map_y(0.0);
    svg.push_str(&format!(
        "<line x1=\"{x1}\" y1=\"{y}\" x2=\"{x2}\" y2=\"{y}\" stroke=\"#507061\" stroke-width=1/>",
        x1 = map_x(x_min),
        x2 = map_x(x_max),
        y = cy
    ));
    svg.push_str(&format!(
        "<line x1=\"{x}\" y1=\"{y1}\" x2=\"{x}\" y2=\"{y2}\" stroke=\"#507061\" stroke-width=1/>",
        x = cx,
        y1 = map_y(y_min),
        y2 = map_y(y_max)
    ));

    // Axis labels
    svg.push_str(&format!(
        "<text x=\"{x}\" y=\"{y}\" font-family=\"sans-serif\" font-size=11 fill=\"#96DC96\">In-Phase (I)</text>",
        x = map_x(x_max) - 80.0,
        y = cy - 6.0
    ));
    svg.push_str(&format!(
        "<text x=\"{x}\" y=\"{y}\" font-family=\"sans-serif\" font-size=11 fill=\"#96DC96\">Quadrature (Q)</text>",
        x = cx + 8.0,
        y = map_y(y_max) + 15.0
    ));

    // Reference constellation for TX
    if matches!(variant, ConstellationVariant::Tx) {
        let ref_pts = [
            (-FRAC_1_SQRT_2, FRAC_1_SQRT_2),
            (FRAC_1_SQRT_2, FRAC_1_SQRT_2),
            (-FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
            (FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
        ];
        for &(i, q) in ref_pts.iter() {
            let x = map_x(i);
            let y = map_y(q);
            svg.push_str(&format!(
                "<circle cx=\"{x}\" cy=\"{y}\" r=\"{r}\" fill=\"#78DC96\" fill-opacity=\"0.5\" stroke=\"#78DC96\" stroke-width=1/>",
                x = x,
                y = y,
                r = 6
            ));
        }
    }

    // Draw symbol points
    let (point_color, r) = match variant {
        ConstellationVariant::Tx => ("#78DC96", 4),
        ConstellationVariant::Rx => ("#78C8F0", 3),
    };

    for (i, q) in finite.iter() {
        let x = map_x(*i);
        let y = map_y(*q);
        svg.push_str(&format!(
            "<circle cx=\"{x}\" cy=\"{y}\" r=\"{r}\" fill=\"{color}\" />",
            x = x,
            y = y,
            r = r,
            color = point_color
        ));
    }

    svg.push_str("</svg>");

    web_sys::console::info_1(
        &format!(
            "Generated SVG for '{}' with {} bytes, contains '<circle': {}",
            title,
            svg.len(),
            svg.contains("<circle")
        )
        .into(),
    );

    svg
}

fn draw_combined_constellation_svg(
    tx_i: &[f64],
    tx_q: &[f64],
    rx_i: &[f64],
    rx_q: &[f64],
    title: &str,
) -> String {
    // Collect finite points
    let tx: Vec<(f64, f64)> = tx_i
        .iter()
        .zip(tx_q.iter())
        .filter_map(|(&i, &q)| if i.is_finite() && q.is_finite() { Some((i, q)) } else { None })
        .collect();
    let rx: Vec<(f64, f64)> = rx_i
        .iter()
        .zip(rx_q.iter())
        .filter_map(|(&i, &q)| if i.is_finite() && q.is_finite() { Some((i, q)) } else { None })
        .collect();

    web_sys::console::info_1(
        &format!(
            "Drawing combined constellation '{}' with {} TX points ({} finite) and {} RX points ({} finite)",
            title,
            tx_i.len(),
            tx.len(),
            rx_i.len(),
            rx.len()
        )
        .into(),
    );

    if tx.is_empty() && rx.is_empty() {
        web_sys::console::warn_1(
            &format!("Skipping combined constellation '{}' due to lack of finite samples", title).into(),
        );
        return String::new();
    }

    let width = 500.0;
    let height = 450.0;
    let padding = 40.0;
    let plot_w = width - padding * 2.0;
    let plot_h = height - padding * 2.0;

    let x_min = -1.5;
    let x_max = 1.5;
    let y_min = -1.5;
    let y_max = 1.5;

    let map_x = |x: f64| padding + ((x - x_min) / (x_max - x_min)) * plot_w;
    let map_y = |y: f64| padding + (1.0 - (y - y_min) / (y_max - y_min)) * plot_h;

    let mut svg = String::new();
    svg.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">",
        width, height, width as i32, height as i32
    ));
    svg.push_str("<rect x=\"0\" y=\"0\" width=\"100%\" height=\"100%\" fill=\"transparent\"/>");

    svg.push_str(&format!(
        "<text x=\"{x}\" y=\"{y}\" font-family=\"monospace\" font-size=14 fill=\"#96DC96\">{title}</text>",
        x = padding,
        y = padding - 8.0,
        title = html_escape::encode_text(title)
    ));

    // Axes center
    let cx = map_x(0.0);
    let cy = map_y(0.0);
    svg.push_str(&format!(
        "<line x1=\"{x1}\" y1=\"{y}\" x2=\"{x2}\" y2=\"{y}\" stroke=\"#507061\" stroke-width=1/>",
        x1 = map_x(x_min),
        x2 = map_x(x_max),
        y = cy
    ));
    svg.push_str(&format!(
        "<line x1=\"{x}\" y1=\"{y1}\" x2=\"{x}\" y2=\"{y2}\" stroke=\"#507061\" stroke-width=1/>",
        x = cx,
        y1 = map_y(y_min),
        y2 = map_y(y_max)
    ));

    // Axis labels
    svg.push_str(&format!(
        "<text x=\"{x}\" y=\"{y}\" font-family=\"sans-serif\" font-size=11 fill=\"#96DC96\">In-Phase (I)</text>",
        x = map_x(x_max) - 80.0,
        y = cy - 6.0
    ));
    svg.push_str(&format!(
        "<text x=\"{x}\" y=\"{y}\" font-family=\"sans-serif\" font-size=11 fill=\"#96DC96\">Quadrature (Q)</text>",
        x = cx + 8.0,
        y = map_y(y_max) + 15.0
    ));

    // Legend
    svg.push_str(&format!(
        "<rect x=\"{x}\" y=\"{y}\" width=\"12\" height=\"12\" fill=\"#78DC96\" /> <text x=\"{tx}\" y=\"{ty}\" font-size=12 fill=\"#96DC96\">TX Symbols</text>",
        x = width - padding - 120.0,
        y = padding,
        tx = width - padding - 100.0,
        ty = padding + 10.0
    ));
    svg.push_str(&format!(
        "<rect x=\"{x}\" y=\"{y}\" width=\"12\" height=\"12\" fill=\"#78C8F0\" /> <text x=\"{tx}\" y=\"{ty}\" font-size=12 fill=\"#96DC96\">RX Symbols</text>",
        x = width - padding - 120.0,
        y = padding + 18.0,
        tx = width - padding - 100.0,
        ty = padding + 28.0
    ));

    // Draw TX
    for (i, q) in tx.iter() {
        let x = map_x(*i);
        let y = map_y(*q);
        svg.push_str(&format!(
            "<circle cx=\"{x}\" cy=\"{y}\" r=\"4\" fill=\"#78DC96\" />",
            x = x,
            y = y
        ));
    }

    // Draw RX
    for (i, q) in rx.iter() {
        let x = map_x(*i);
        let y = map_y(*q);
        svg.push_str(&format!(
            "<circle cx=\"{x}\" cy=\"{y}\" r=\"3\" fill=\"#78C8F0\" />",
            x = x,
            y = y
        ));
    }

    svg.push_str("</svg>");

    web_sys::console::info_1(
        &format!(
            "Generated combined SVG for '{}' with {} bytes, contains '<circle': {}",
            title,
            svg.len(),
            svg.contains("<circle")
        )
        .into(),
    );

    svg
}

fn draw_line_chart_svg(
    values: &[f64],
    title: &str,
    x_label: &str,
    y_label: &str,
    accent_rgb: Option<(u8, u8, u8)>,
) -> String {
    // Filter finite values and remember original length
    let finite: Vec<f64> = values.iter().cloned().filter(|v| v.is_finite()).collect();

    web_sys::console::info_1(
        &format!(
            "Drawing line chart '{}' with {} values ({} finite)",
            title,
            values.len(),
            finite.len()
        )
        .into(),
    );

    if finite.is_empty() {
        web_sys::console::warn_1(
            &format!("Skipping line chart '{}' due to lack of finite samples", title).into(),
        );
        return String::new();
    }

    let width = 500.0;
    let height = 280.0;
    let padding = 40.0;
    let plot_w = width - padding * 2.0;
    let plot_h = height - padding * 2.0;

    // Compute min/max for y axis
    let y_min = finite.iter().cloned().fold(f64::INFINITY, f64::min);
    let y_max = finite.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let y_min = if y_min == f64::INFINITY { 0.0 } else { y_min };
    let y_max = if y_max == f64::NEG_INFINITY { 0.0 } else { y_max };
    let y_range = (y_max - y_min).abs().max(1e-6);

    let x_denominator = (finite.len().saturating_sub(1) as f64).max(1.0);
    let map_x = |idx: usize| padding + (idx as f64 / x_denominator) * plot_w;
    let map_y = |v: f64| padding + (1.0 - (v - y_min) / y_range) * plot_h;

    let accent = accent_rgb.unwrap_or((120, 220, 150));
    let accent_color = format!("rgb({},{},{})", accent.0, accent.1, accent.2);

    let mut svg = String::new();
    svg.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">",
        width, height, width as i32, height as i32
    ));

    svg.push_str("<rect x=\"0\" y=\"0\" width=\"100%\" height=\"100%\" fill=\"transparent\"/>");

    // Title
    svg.push_str(&format!(
        "<text x=\"{x}\" y=\"{y}\" font-family=\"monospace\" font-size=14 fill=\"#96DC96\">{title}</text>",
        x = padding,
        y = padding - 8.0,
        title = html_escape::encode_text(title)
    ));

    // Axis labels
    svg.push_str(&format!(
        "<text x=\"{x}\" y=\"{y}\" font-family=\"sans-serif\" font-size=11 fill=\"#96DC96\">{x_label}</text>",
        x = padding + plot_w / 2.0,
        y = height - 6.0,
        x_label = html_escape::encode_text(x_label)
    ));
    svg.push_str(&format!(
        "<text transform=\"translate(12 {ty}) rotate(-90)\" font-family=\"sans-serif\" font-size=11 fill=\"#96DC96\">{y_label}</text>",
        ty = padding + plot_h / 2.0,
        y_label = html_escape::encode_text(y_label)
    ));

    // Draw axes lines
    let x1 = map_x(0);
    let x2 = map_x(finite.len().saturating_sub(1));
    svg.push_str(&format!(
        "<line x1=\"{x1}\" y1=\"{y2}\" x2=\"{x2}\" y2=\"{y2}\" stroke=\"#507061\" />",
        x1 = x1,
        x2 = x2,
        y2 = map_y(y_min)
    ));
    svg.push_str(&format!(
        "<line x1=\"{x1}\" y1=\"{y1}\" x2=\"{x1}\" y2=\"{y2}\" stroke=\"#507061\" />",
        x1 = map_x(0),
        y1 = map_y(y_min),
        y2 = map_y(y_max)
    ));

    // Build polyline points
    let mut points = String::new();
    for (idx, &v) in finite.iter().enumerate() {
        let px = map_x(idx);
        let py = map_y(v);
        points.push_str(&format!("{:.2},{:.2} ", px, py));
    }

    svg.push_str(&format!(
        "<polyline points=\"{points}\" fill=\"none\" stroke=\"{color}\" stroke-width=1.5 />",
        points = points,
        color = accent_color
    ));

    // Draw small circles on points for visibility
    for (idx, &v) in finite.iter().enumerate() {
        let px = map_x(idx);
        let py = map_y(v);
        svg.push_str(&format!(
            "<circle cx=\"{x}\" cy=\"{y}\" r=\"2\" fill=\"{color}\" />",
            x = px,
            y = py,
            color = accent_color
        ));
    }

    svg.push_str("</svg>");

    web_sys::console::info_1(
        &format!(
            "Generated line-chart SVG for '{}' with {} bytes, contains 'polyline': {}",
            title,
            svg.len(),
            svg.contains("polyline")
        )
        .into(),
    );

    svg
}

fn decimate_series(series: &[f64], max_points: usize) -> Vec<f64> {
    if max_points == 0 || series.len() <= max_points {
        return series.to_vec();
    }

    let step = series.len() as f64 / max_points as f64;
    let mut result = Vec::with_capacity(max_points);
    let mut cursor = 0.0f64;

    while result.len() < max_points {
        let start = cursor.floor() as usize;
        if start >= series.len() {
            break;
        }
        let mut end = ((cursor + step).ceil() as usize).min(series.len());
        if end <= start {
            end = start + 1;
        }
        let slice = &series[start..end.min(series.len())];
        let avg = slice.iter().sum::<f64>() / slice.len() as f64;
        result.push(avg);
        cursor += step;
    }

    if result.is_empty() {
        series.to_vec()
    } else {
        result
    }
}

fn compute_psd(samples: &[f64], _sample_rate: usize) -> Vec<f64> {
    let sample_count = samples.len().min(16_384);
    if sample_count == 0 {
        return Vec::new();
    }

    let mut fft_len = sample_count.next_power_of_two();
    fft_len = fft_len.clamp(1_024, 32_768);

    let mut planner = FftPlanner::<f64>::new();
    let fft = planner.plan_fft_forward(fft_len);

    let mut buffer: Vec<Complex<f64>> = (0..fft_len)
        .map(|i| {
            if i < sample_count {
                Complex::new(samples[i], 0.0)
            } else {
                Complex::zero()
            }
        })
        .collect();

    fft.process(&mut buffer);

    let scale = 1.0 / fft_len as f64;
    let half = buffer.len() / 2;
    let mut spectrum: Vec<f64> = buffer
        .iter()
        .take(half)
        .map(|c| {
            let power = (c.norm_sqr() * scale).max(1e-12);
            10.0 * power.log10()
        })
        .collect();

    let max_db = spectrum.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    if max_db.is_finite() {
        for v in &mut spectrum {
            *v -= max_db;
        }
    }

    decimate_series(&spectrum, 512)
}

fn compute_ber_trend(tx_bits: &[u8], decisions: &[SymbolDecision]) -> Vec<f64> {
    let symbol_count = usize::min(decisions.len(), tx_bits.len() / 2);
    if symbol_count == 0 {
        return Vec::new();
    }

    let mut errors = 0usize;
    let mut trend = Vec::with_capacity(symbol_count);

    for idx in 0..symbol_count {
        let tx0 = tx_bits[idx * 2];
        let tx1 = tx_bits[idx * 2 + 1];
        let decided = decisions[idx].decided_bits;
        if decided[0] != tx0 {
            errors += 1;
        }
        if decided[1] != tx1 {
            errors += 1;
        }
        let total_bits = (idx + 1) * 2;
        trend.push(errors as f64 / total_bits as f64);
    }

    decimate_series(&trend, 512)
}

fn format_opcode_label(opcode: u32) -> String {
    let label = match opcode {
        0x0001 => "Baseline Telemetry",
        0x00F1 => "Burst Telemetry Control",
        0x0D11 => "Deep-Space Probe Command",
        _ => "Custom Opcode",
    };

    format!("{label} · 0x{opcode:04X}")
}

fn format_command_word_label(desc: &FrameDescriptor) -> String {
    let frame_number = desc.frame_index + 1;
    let total_frames = desc.total_frames;

    let sequence = if total_frames > 0 {
        format!("Frame {frame_number} of {total_frames}")
    } else {
        format!("Frame {frame_number}")
    };

    format!("{sequence} · 0x{:08X}", desc.command_value)
}

fn format_sci(value: f64) -> String {
    if value == 0.0 {
        "0".into()
    } else {
        format!("{:.3e}", value)
    }
}

fn play_audio(
    samples: &[f32],
    sample_rate: usize,
    source_node_ref: &Rc<std::cell::RefCell<Option<AudioBufferSourceNode>>>,
    context_ref: &Rc<std::cell::RefCell<Option<AudioContext>>>,
    state: &UseStateHandle<AudioPlaybackState>,
    new_state: AudioPlaybackState,
    gain: f64,
) {
    if samples.is_empty() {
        web_sys::console::warn_1(&"Cannot play empty audio buffer".into());
        return;
    }

    let ctx = match (*context_ref.borrow()).as_ref() {
        Some(existing) => existing.clone(),
        None => match AudioContext::new() {
            Ok(new_ctx) => {
                *context_ref.borrow_mut() = Some(new_ctx.clone());
                new_ctx
            }
            Err(e) => {
                web_sys::console::error_1(
                    &format!("Failed to create AudioContext: {:?}", e).into(),
                );
                return;
            }
        },
    };

    // Resume context if suspended
    if ctx.state() == web_sys::AudioContextState::Suspended {
        let _ = ctx.resume();
    }

    let buffer = match ctx.create_buffer(1, samples.len() as u32, sample_rate as f32) {
        Ok(buf) => buf,
        Err(e) => {
            web_sys::console::error_1(&format!("Failed to create audio buffer: {:?}", e).into());
            return;
        }
    };

    if let Err(e) = buffer.copy_to_channel(samples, 0) {
        web_sys::console::error_1(&format!("Failed to copy audio data: {:?}", e).into());
        return;
    }

    let source = match ctx.create_buffer_source() {
        Ok(src) => src,
        Err(e) => {
            web_sys::console::error_1(&format!("Failed to create buffer source: {:?}", e).into());
            return;
        }
    };

    source.set_buffer(Some(&buffer));

    // Create gain node for volume control
    let gain_node = match ctx.create_gain() {
        Ok(node) => node,
        Err(e) => {
            web_sys::console::error_1(&format!("Failed to create gain node: {:?}", e).into());
            return;
        }
    };

    gain_node.gain().set_value(gain as f32);

    if let Err(e) = source.connect_with_audio_node(&gain_node) {
        web_sys::console::error_1(&format!("Failed to connect source to gain: {:?}", e).into());
        return;
    }

    if let Err(e) = gain_node.connect_with_audio_node(&ctx.destination()) {
        web_sys::console::error_1(
            &format!("Failed to connect gain to destination: {:?}", e).into(),
        );
        return;
    }

    let state_handle = state.clone();
    let on_ended = Closure::wrap(Box::new(move || {
        state_handle.set(AudioPlaybackState::Stopped);
    }) as Box<dyn Fn()>);

    let _ = source.add_event_listener_with_callback("ended", on_ended.as_ref().unchecked_ref());
    on_ended.forget();

    if let Err(e) = source.start() {
        web_sys::console::error_1(&format!("Failed to start audio playback: {:?}", e).into());
        return;
    }

    *source_node_ref.borrow_mut() = Some(source);
    state.set(new_state);
}

fn stop_audio(
    source_node_ref: &Rc<std::cell::RefCell<Option<AudioBufferSourceNode>>>,
    state: &UseStateHandle<AudioPlaybackState>,
) {
    if let Some(source) = source_node_ref.borrow_mut().take() {
        #[allow(deprecated)]
        let _ = source.stop();
    }
    state.set(AudioPlaybackState::Stopped);
}

pub fn mount_app() {
    let window = web_sys::window().expect("window available");
    let document: Document = window.document().expect("document available");
    let root = document
        .get_element_by_id("chimera-root")
        .or_else(|| {
            let body = document.body().expect("document should have a body");
            let element = document.create_element("div").unwrap();
            element.set_id("chimera-root");
            body.append_child(&element).unwrap();
            Some(element)
        })
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();

    yew::Renderer::<App>::with_root(root.into()).render();

    // Test hook: expose a test helper to trigger a synchronous pipeline run
    // and inject generated SVG directly into the DOM. Tests call
    // `window.__test_trigger_run()` to deterministically produce the charts.
    {
        // Create a Closure that calls the Rust helper and attach it to window.__test_trigger_run
        let cb = Closure::wrap(Box::new(move || {
            // Ignore errors; test harness will check DOM
            let _ = __test_trigger_run();
        }) as Box<dyn Fn()>);
        let _ = js_sys::Reflect::set(
            &web_sys::window().unwrap(),
            &JsValue::from_str("__test_trigger_run"),
            cb.as_ref().unchecked_ref(),
        );
        cb.forget();
    }
    // Check for localStorage-triggered runs (tests may set this before reloading)
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(val)) = storage.get_item("chimera_test_run_now") {
                    if val == "1" {
                        // Delay the trigger slightly so the Yew app has time to mount
                        let _ = storage.remove_item("chimera_test_run_now");
                        Timeout::new(250, || {
                            let _ = __test_trigger_run();
                        })
                        .forget();
                    }
                }
        }
    }
}

/// Debug helper: run the pipeline and inject chart SVGs directly into the DOM.
#[wasm_bindgen]
pub fn __test_trigger_run() -> Result<(), JsValue> {
    // Run pipeline synchronously with default input
    let input = SimulationInput::default();
    let output = run_pipeline(input);
    let diag = output.diagnostics;

    let document = web_sys::window()
        .ok_or_else(|| JsValue::from_str("no window"))?
        .document()
        .ok_or_else(|| JsValue::from_str("no document"))?;

    // Helper to set inner HTML of a container within a node that contains a header
    let set_node_svg = |node_text: &str, svg: String| {
        if let Ok(nodes) = document.query_selector_all(".node") {
            for i in 0..nodes.length() {
                if let Some(node) = nodes.item(i) {
                    if let Ok(el) = node.dyn_into::<web_sys::Element>() {
                        if el.text_content().unwrap_or_default().contains(node_text) {
                            let coll = el.get_elements_by_class_name("svg-chart-container");
                            if let Some(container) = coll.item(0) {
                                container.set_inner_html(&svg);
                                return;
                            }
                        }
                    }
                }
            }
        }
    };

    // Inject TX and RX constellations
    web_sys::console::info_1(&format!(
        "Test hook diagnostics: tx={} tx_finite={}, rx={} rx_finite={}",
        diag.tx_symbols_i.len(),
        diag.tx_symbols_i.iter().filter(|v| v.is_finite()).count(),
        diag.demodulation.received_symbols_i.len(),
        diag.demodulation.received_symbols_i.iter().filter(|v| v.is_finite()).count()
    ).into());
    let tx_svg = draw_constellation_svg(&diag.tx_symbols_i, &diag.tx_symbols_q, "TX Symbols", ConstellationVariant::Tx);
    let rx_svg = draw_constellation_svg(&diag.demodulation.received_symbols_i, &diag.demodulation.received_symbols_q, "RX Symbols", ConstellationVariant::Rx);
    set_node_svg("Transmitter", tx_svg);
    set_node_svg("Receiver", rx_svg);

    // Inject combined constellation
    if let Ok(Some(combined)) = document.query_selector(".constellation-combined .svg-chart-container") {
        let combined_svg = draw_combined_constellation_svg(&diag.tx_symbols_i, &diag.tx_symbols_q, &diag.demodulation.received_symbols_i, &diag.demodulation.received_symbols_q, "TX vs RX Constellation");
        combined.set_inner_html(&combined_svg);
    }

    // Inject diagnostics line charts by title
    let inject_line_by_title = |title: &str, svg: String| {
        if let Ok(nodes) = document.query_selector_all(".chart-panel") {
            for i in 0..nodes.length() {
                if let Some(node) = nodes.item(i) {
                    if let Ok(el) = node.dyn_into::<web_sys::Element>() {
                        if el.text_content().unwrap_or_default().contains(title) {
                            let coll = el.get_elements_by_class_name("svg-chart-container");
                            if let Some(container) = coll.item(0) {
                                container.set_inner_html(&svg);
                                return;
                            }
                        }
                    }
                }
            }
        }
    };

    // Create and inject some diagnostics charts
    let timing_svg = draw_line_chart_svg(&diag.demodulation.timing_error, "Timing Error", "Sample Index", "Error (samples)", Some((94, 214, 255)));
    inject_line_by_title("Timing Error", timing_svg);

    let nco_svg = draw_line_chart_svg(&diag.demodulation.nco_freq_offset, "NCO Frequency Offset", "Sample Index", "Offset (Hz)", Some((255, 168, 112)));
    inject_line_by_title("NCO Frequency Offset", nco_svg);

    let clean_svg = draw_line_chart_svg(&compute_psd(&diag.clean_baseband, FIXED_SAMPLE_RATE), "Clean Signal PSD", "Frequency Bin", "Power (dBFS)", Some((126, 240, 180)));
    inject_line_by_title("Clean Signal PSD", clean_svg);

    let noisy_svg = draw_line_chart_svg(&compute_psd(&diag.noisy_baseband, FIXED_SAMPLE_RATE), "Noisy Signal PSD", "Frequency Bin", "Power (dBFS)", Some((255, 132, 220)));
    inject_line_by_title("Noisy Signal PSD", noisy_svg);

    let ber_svg = draw_line_chart_svg(&compute_ber_trend(&diag.tx_bits, &diag.demodulation.symbol_decisions), "Running BER", "Symbol Index", "BER", Some((255, 238, 96)));
    inject_line_by_title("Running BER", ber_svg);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_constellation_svg_generation() {
        // Test data: QPSK constellation points
        let symbols_i = vec![
            0.707, 0.707, -0.707, -0.707,
            0.707, 0.707, -0.707, -0.707,
        ];
        let symbols_q = vec![
            0.707, -0.707, 0.707, -0.707,
            0.707, -0.707, 0.707, -0.707,
        ];

        let svg = draw_constellation_svg(
            &symbols_i,
            &symbols_q,
            "Test Chart",
            ConstellationVariant::Tx,
        );

        // Verify SVG is not empty
        assert!(!svg.is_empty(), "SVG should not be empty");
        
        // Verify SVG has proper structure
        assert!(svg.contains("<svg"), "SVG should contain opening tag");
        assert!(svg.contains("</svg>"), "SVG should contain closing tag");
        
        // Verify it contains data points (circles)
        assert!(svg.contains("<circle"), "SVG should contain circle elements");
        
        // Count circles - should have data points plus reference points
        let circle_count = svg.matches("<circle").count();
        assert!(circle_count > 0, "Should have at least one circle");
        
        web_sys::console::log_1(&format!("Generated SVG with {} circles, {} bytes", circle_count, svg.len()).into());
    }

    #[wasm_bindgen_test]
    fn test_combined_constellation_svg_generation() {
        // Test data
        let tx_i = vec![0.707, 0.707, -0.707, -0.707];
        let tx_q = vec![0.707, -0.707, 0.707, -0.707];
        let rx_i = vec![0.71, 0.69, -0.71, -0.69];
        let rx_q = vec![0.71, -0.69, 0.71, -0.69];

        let svg = draw_combined_constellation_svg(
            &tx_i,
            &tx_q,
            &rx_i,
            &rx_q,
            "Combined Test",
        );

        // Verify SVG structure
        assert!(!svg.is_empty(), "SVG should not be empty");
        assert!(svg.contains("<svg"), "SVG should contain opening tag");
        assert!(svg.contains("</svg>"), "SVG should contain closing tag");
        assert!(svg.contains("<circle"), "SVG should contain circle elements");
        
        // Verify legend text
        assert!(svg.contains("TX Symbols"), "Should have TX legend");
        assert!(svg.contains("RX Symbols"), "Should have RX legend");
        
        let circle_count = svg.matches("<circle").count();
        web_sys::console::log_1(&format!("Generated combined SVG with {} circles", circle_count).into());
    }

    #[wasm_bindgen_test]
    fn test_empty_constellation_returns_empty_svg() {
        // Test with no data
        let empty_i: Vec<f64> = vec![];
        let empty_q: Vec<f64> = vec![];

        let svg = draw_constellation_svg(
            &empty_i,
            &empty_q,
            "Empty Test",
            ConstellationVariant::Rx,
        );

        // Should return empty string when no data
        assert!(svg.is_empty(), "SVG should be empty when no data points");
    }

    #[wasm_bindgen_test]
    fn test_non_finite_values_are_handled() {
        // Test with NaN and Inf values
        let symbols_i = vec![f64::NAN, f64::INFINITY, 0.707, -0.707];
        let symbols_q = vec![0.707, f64::NEG_INFINITY, f64::NAN, -0.707];

        let svg = draw_constellation_svg(
            &symbols_i,
            &symbols_q,
            "Non-finite Test",
            ConstellationVariant::Tx,
        );

        // Should still generate SVG if there are some finite values
        // We have 2 finite pairs out of 4
        assert!(!svg.is_empty(), "SVG should be generated with partial finite data");
        assert!(svg.contains("<circle"), "Should contain circles for finite points");
    }
}
