/// This test outputs actual SVG files that can be manually inspected
/// to verify that constellation charts are being drawn correctly.
use chimera_web::{run_pipeline, SimulationInput};
use std::fs;
use std::path::PathBuf;

// Helper to create test SVG using the same plotters backend as the UI
fn create_test_svg(
    symbols_i: &[f64],
    symbols_q: &[f64],
    title: &str,
    output_path: &str,
) -> String {
    use plotters::prelude::*;
    use plotters::backend::SVGBackend;
    use std::f64::consts::FRAC_1_SQRT_2;

    let mut svg_string = String::new();
    {
        let backend = SVGBackend::with_string(&mut svg_string, (400, 400));
        let root = backend.into_drawing_area();
        let _ = root.fill(&TRANSPARENT);

        let result = (|| -> Result<(), Box<dyn std::error::Error>> {
            let mut chart = ChartBuilder::on(&root)
                .caption(title, ("Share Tech Mono", 18, &RGBColor(150, 220, 150)))
                .margin(15)
                .x_label_area_size(40)
                .y_label_area_size(50)
                .build_cartesian_2d(-1.5..1.5, -1.5..1.5)?;

            chart
                .configure_mesh()
                .bold_line_style(RGBColor(80, 140, 100).mix(0.5))
                .light_line_style(RGBColor(60, 100, 80).mix(0.3))
                .x_labels(7)
                .y_labels(7)
                .x_label_formatter(&|x| format!("{:.1}", x))
                .y_label_formatter(&|y| format!("{:.1}", y))
                .x_desc("In-Phase (I)")
                .y_desc("Quadrature (Q)")
                .label_style(("Share Tech Mono", 12, &RGBColor(150, 220, 150)))
                .axis_desc_style(("Share Tech Mono", 14, &RGBColor(150, 220, 150)))
                .draw()?;

            // Draw reference QPSK constellation
            let reference = [
                (-FRAC_1_SQRT_2, FRAC_1_SQRT_2),
                (FRAC_1_SQRT_2, FRAC_1_SQRT_2),
                (-FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
                (FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
            ];
            let halo_color = RGBAColor(120, 220, 150, 0.3);
            chart.draw_series(
                reference
                    .iter()
                    .map(|&(i, q)| Circle::new((i, q), 8, halo_color.filled())),
            )?;

            // Draw actual symbols
            let point_color = RGBColor(120, 220, 150);
            let symbols = symbols_i
                .iter()
                .zip(symbols_q.iter())
                .map(|(&i, &q)| (i, q));

            chart.draw_series(symbols.map(|(i, q)| Circle::new((i, q), 6, point_color.filled())))?;

            Ok(())
        })();

        if let Err(e) = result {
            eprintln!("Error drawing: {:?}", e);
        }

        let _ = root.present();
    }

    // Write to file
    if let Err(e) = fs::write(output_path, &svg_string) {
        eprintln!("Failed to write SVG to {}: {}", output_path, e);
    } else {
        println!("Wrote SVG to: {}", output_path);
    }

    svg_string
}

#[test]
fn generate_constellation_svg_samples() {
    // Create output directory
    let output_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test-output");
    fs::create_dir_all(&output_dir).expect("Failed to create output directory");

    println!("Generating test SVG files in: {}", output_dir.display());

    // Run the pipeline to get real data
    let input = SimulationInput {
        plaintext: "Test message for SVG generation".into(),
        ..Default::default()
    };
    let output = run_pipeline(input);

    // Generate TX constellation SVG
    let tx_svg = create_test_svg(
        &output.diagnostics.tx_symbols_i,
        &output.diagnostics.tx_symbols_q,
        "TX Constellation (Test Output)",
        &output_dir.join("tx_constellation.svg").to_string_lossy(),
    );

    // Verify TX SVG has content
    assert!(!tx_svg.is_empty(), "TX SVG should not be empty");
    assert!(tx_svg.contains("<svg"), "TX SVG should have opening tag");
    assert!(tx_svg.contains("</svg>"), "TX SVG should have closing tag");
    assert!(tx_svg.contains("<circle"), "TX SVG should have circles");

    let tx_circle_count = tx_svg.matches("<circle").count();
    println!("TX SVG contains {} circles", tx_circle_count);
    assert!(
        tx_circle_count > 0,
        "TX SVG should have at least one circle"
    );

    // Generate RX constellation SVG
    let rx_svg = create_test_svg(
        &output.diagnostics.demodulation.received_symbols_i,
        &output.diagnostics.demodulation.received_symbols_q,
        "RX Constellation (Test Output)",
        &output_dir.join("rx_constellation.svg").to_string_lossy(),
    );

    // Verify RX SVG has content
    assert!(!rx_svg.is_empty(), "RX SVG should not be empty");
    assert!(rx_svg.contains("<circle"), "RX SVG should have circles");

    let rx_circle_count = rx_svg.matches("<circle").count();
    println!("RX SVG contains {} circles", rx_circle_count);
    assert!(
        rx_circle_count > 0,
        "RX SVG should have at least one circle"
    );

    println!("\n✓ Successfully generated test SVG files");
    println!("  TX: {}", output_dir.join("tx_constellation.svg").display());
    println!("  RX: {}", output_dir.join("rx_constellation.svg").display());
    println!("\nOpen these files in a browser to visually verify the charts render correctly.");
}
