use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use chimera_core::{
    config::{ProtocolConfig, LDPCConfig},
    decoder::{StreamingSymbolDecoder, demodulate_qpsk_symbol},
    ldpc::LDPCSuite,
    encoder::StreamingFrameEncoder,
    utils::string_to_bitstream,
};
use num_complex::Complex64;
use std::f64::consts::FRAC_1_SQRT_2;

fn benchmark_qpsk_demodulation(c: &mut Criterion) {
    let test_symbols = vec![
        Complex64::new(-FRAC_1_SQRT_2, FRAC_1_SQRT_2),
        Complex64::new(FRAC_1_SQRT_2, FRAC_1_SQRT_2),
        Complex64::new(-FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
        Complex64::new(FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
    ];
    
    c.bench_function("demodulate_single_qpsk_symbol", |b| {
        b.iter(|| {
            for symbol in &test_symbols {
                black_box(demodulate_qpsk_symbol(black_box(*symbol)));
            }
        })
    });
}

fn benchmark_symbol_decoding(c: &mut Criterion) {
    let protocol = ProtocolConfig::default();
    let ldpc_config = LDPCConfig::default();
    let ldpc_suite = LDPCSuite::new(&protocol.frame_layout, &ldpc_config);
    
    // Generate test symbols
    let test_message = "Test message for decoder benchmarking.";
    let payload_bits = string_to_bitstream(test_message);
    let mut encoder = StreamingFrameEncoder::new(
        &payload_bits,
        protocol.clone(),
        ldpc_suite.matrices.clone(),
    );
    
    let (tx_symbols, _, _, _, _) = encoder.get_next_symbols(128);
    
    let mut group = c.benchmark_group("symbol_decoding");
    
    for size in [4, 8, 16, 32, 64, 128].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let mut decoder = StreamingSymbolDecoder::new(
                protocol.clone(),
                ldpc_suite.matrices.clone(),
            );
            
            let symbols = &tx_symbols[..size.min(tx_symbols.len())];
            
            b.iter(|| {
                decoder.process_symbols(black_box(symbols))
            })
        });
    }
    
    group.finish();
}

fn benchmark_sync_search(c: &mut Criterion) {
    let protocol = ProtocolConfig::default();
    let ldpc_config = LDPCConfig::default();
    let ldpc_suite = LDPCSuite::new(&protocol.frame_layout, &ldpc_config);
    
    // Generate a full frame with sync
    let test_message = "Message with sync pattern";
    let payload_bits = string_to_bitstream(test_message);
    let mut encoder = StreamingFrameEncoder::new(
        &payload_bits,
        protocol.clone(),
        ldpc_suite.matrices.clone(),
    );
    
    let (tx_symbols, _, _, _, _) = encoder.get_next_symbols(protocol.frame_layout.total_symbols);
    
    c.bench_function("sync_search_full_frame", |b| {
        b.iter(|| {
            let mut decoder = StreamingSymbolDecoder::new(
                protocol.clone(),
                ldpc_suite.matrices.clone(),
            );
            
            decoder.process_symbols(black_box(&tx_symbols))
        })
    });
}

criterion_group!(
    benches,
    benchmark_qpsk_demodulation,
    benchmark_symbol_decoding,
    benchmark_sync_search
);
criterion_main!(benches);
