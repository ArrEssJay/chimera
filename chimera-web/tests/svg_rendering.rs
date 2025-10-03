use chimera_web::{run_pipeline, SimulationInput};

#[test]
fn test_svg_generation_with_real_data() {
    let input = SimulationInput {
        plaintext: "Test".into(),
        ..Default::default()
    };
    let output = run_pipeline(input);

    // Verify we have data
    assert!(!output.diagnostics.tx_symbols_i.is_empty());
    assert!(!output.diagnostics.tx_symbols_q.is_empty());
    assert!(!output
        .diagnostics
        .demodulation
        .received_symbols_i
        .is_empty());
    assert!(!output
        .diagnostics
        .demodulation
        .received_symbols_q
        .is_empty());

    // Verify all values are finite
    for &val in &output.diagnostics.tx_symbols_i {
        assert!(val.is_finite(), "TX I value {} is not finite", val);
    }
    for &val in &output.diagnostics.tx_symbols_q {
        assert!(val.is_finite(), "TX Q value {} is not finite", val);
    }
    for &val in &output.diagnostics.demodulation.received_symbols_i {
        assert!(val.is_finite(), "RX I value {} is not finite", val);
    }
    for &val in &output.diagnostics.demodulation.received_symbols_q {
        assert!(val.is_finite(), "RX Q value {} is not finite", val);
    }

    println!("All constellation values are finite and valid");
    println!(
        "TX symbols: {} points",
        output.diagnostics.tx_symbols_i.len()
    );
    println!(
        "RX symbols: {} points",
        output.diagnostics.demodulation.received_symbols_i.len()
    );
}

#[test]
fn test_constellation_data_within_expected_range() {
    let input = SimulationInput {
        plaintext: "Test message for constellation".into(),
        snr_db: 20.0, // High SNR for cleaner constellation
        ..Default::default()
    };
    let output = run_pipeline(input);

    // For QPSK, constellation points should be around ±1/√2 (±0.707)
    // With high SNR, most points should be within ±2.0 of the origin
    let mut out_of_range_count = 0;
    let total_tx = output.diagnostics.tx_symbols_i.len();

    for (&i, &q) in output
        .diagnostics
        .tx_symbols_i
        .iter()
        .zip(output.diagnostics.tx_symbols_q.iter())
    {
        let magnitude = (i * i + q * q).sqrt();
        if magnitude > 2.0 {
            out_of_range_count += 1;
        }
    }

    // TX symbols should all be within range (they're ideal)
    assert_eq!(
        out_of_range_count, 0,
        "TX constellation points should all be within expected range"
    );

    // RX symbols might have noise but should mostly be reasonable
    let mut rx_out_of_range = 0;
    for (&i, &q) in output
        .diagnostics
        .demodulation
        .received_symbols_i
        .iter()
        .zip(output.diagnostics.demodulation.received_symbols_q.iter())
    {
        let magnitude = (i * i + q * q).sqrt();
        if magnitude > 3.0 {
            rx_out_of_range += 1;
        }
    }

    let rx_total = output.diagnostics.demodulation.received_symbols_i.len();
    let rx_outlier_ratio = rx_out_of_range as f64 / rx_total as f64;

    println!(
        "TX: {}/{} points out of range",
        out_of_range_count, total_tx
    );
    println!(
        "RX: {}/{} points beyond 3.0 magnitude ({:.1}%)",
        rx_out_of_range,
        rx_total,
        rx_outlier_ratio * 100.0
    );

    // With 20dB SNR, we shouldn't have too many extreme outliers
    assert!(
        rx_outlier_ratio < 0.1,
        "Too many RX points outside expected range: {:.1}%",
        rx_outlier_ratio * 100.0
    );
}

#[test]
fn test_constellation_points_within_ui_axis_range() {
    // The UI uses fixed axis ranges of -1.5 to 1.5
    // This test verifies that constellation points fall within this range
    let input = SimulationInput {
        plaintext: "Testing axis ranges".into(),
        ..Default::default()
    };
    let output = run_pipeline(input);

    let tx_i = &output.diagnostics.tx_symbols_i;
    let tx_q = &output.diagnostics.tx_symbols_q;

    // Calculate ranges
    let min_tx_i = tx_i.iter().copied().fold(f64::INFINITY, f64::min);
    let max_tx_i = tx_i.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let min_tx_q = tx_q.iter().copied().fold(f64::INFINITY, f64::min);
    let max_tx_q = tx_q.iter().copied().fold(f64::NEG_INFINITY, f64::max);

    println!("TX I range: [{:.4}, {:.4}]", min_tx_i, max_tx_i);
    println!("TX Q range: [{:.4}, {:.4}]", min_tx_q, max_tx_q);
    println!("UI axis range: [-1.5, 1.5]");

    // Count points outside visible range
    let mut tx_outside = 0;
    for (&i, &q) in tx_i.iter().zip(tx_q.iter()) {
        if !(-1.5..=1.5).contains(&i) || !(-1.5..=1.5).contains(&q) {
            tx_outside += 1;
        }
    }

    let rx_i = &output.diagnostics.demodulation.received_symbols_i;
    let rx_q = &output.diagnostics.demodulation.received_symbols_q;

    let min_rx_i = rx_i.iter().copied().fold(f64::INFINITY, f64::min);
    let max_rx_i = rx_i.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let min_rx_q = rx_q.iter().copied().fold(f64::INFINITY, f64::min);
    let max_rx_q = rx_q.iter().copied().fold(f64::NEG_INFINITY, f64::max);

    println!("\nRX I range: [{:.4}, {:.4}]", min_rx_i, max_rx_i);
    println!("RX Q range: [{:.4}, {:.4}]", min_rx_q, max_rx_q);

    let mut rx_outside = 0;
    for (&i, &q) in rx_i.iter().zip(rx_q.iter()) {
        if !(-1.5..=1.5).contains(&i) || !(-1.5..=1.5).contains(&q) {
            rx_outside += 1;
        }
    }

    println!(
        "\nTX points outside visible range: {}/{}",
        tx_outside,
        tx_i.len()
    );
    println!(
        "RX points outside visible range: {}/{}",
        rx_outside,
        rx_i.len()
    );

    // TX symbols should all be within the visible range (they're ideal QPSK at ±0.707)
    assert_eq!(
        tx_outside,
        0,
        "TX constellation points outside UI axis range [-1.5, 1.5]: {}/{}",
        tx_outside,
        tx_i.len()
    );

    // With default SNR, most RX points should be visible
    let rx_visible_ratio = 1.0 - (rx_outside as f64 / rx_i.len() as f64);
    println!("\nRX visibility ratio: {:.1}%", rx_visible_ratio * 100.0);

    assert!(
        rx_visible_ratio > 0.8,
        "Too many RX points outside visible range: only {:.1}% visible",
        rx_visible_ratio * 100.0
    );
}
