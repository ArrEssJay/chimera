use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use chimera_core::{
    config::{ProtocolConfig, LDPCConfig},
    encoder::StreamingFrameEncoder,
    ldpc::LDPCSuite,
    utils::string_to_bitstream,
};

fn benchmark_frame_encoding(c: &mut Criterion) {
    let protocol = ProtocolConfig::default();
    let ldpc_config = LDPCConfig::default();
    let ldpc_suite = LDPCSuite::new(&protocol.frame_layout, &ldpc_config);
    
    let test_message = "This is a test message for benchmarking the encoder.";
    let payload_bits = string_to_bitstream(test_message);
    
    c.bench_function("frame_encoding_initialization", |b| {
        b.iter(|| {
            StreamingFrameEncoder::new(
                black_box(&payload_bits),
                black_box(protocol.clone()),
                black_box(ldpc_suite.matrices.clone()),
            )
        })
    });
    
    let mut encoder = StreamingFrameEncoder::new(
        &payload_bits,
        protocol.clone(),
        ldpc_suite.matrices.clone(),
    );
    
    c.bench_function("generate_16_symbols", |b| {
        b.iter(|| {
            encoder.get_next_symbols(black_box(16))
        })
    });
    
    c.bench_function("generate_128_symbols", |b| {
        b.iter(|| {
            encoder.get_next_symbols(black_box(128))
        })
    });
}

fn benchmark_symbol_generation_sizes(c: &mut Criterion) {
    let protocol = ProtocolConfig::default();
    let ldpc_config = LDPCConfig::default();
    let ldpc_suite = LDPCSuite::new(&protocol.frame_layout, &ldpc_config);
    
    let test_message = "Benchmark message for testing different symbol generation sizes.";
    let payload_bits = string_to_bitstream(test_message);
    
    let mut group = c.benchmark_group("symbol_generation");
    
    for size in [4, 8, 16, 32, 64, 128, 256].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let mut encoder = StreamingFrameEncoder::new(
                &payload_bits,
                protocol.clone(),
                ldpc_suite.matrices.clone(),
            );
            
            b.iter(|| {
                encoder.get_next_symbols(black_box(size))
            })
        });
    }
    
    group.finish();
}

fn benchmark_generator_matrix_multiply(c: &mut Criterion) {
    let protocol = ProtocolConfig::default();
    let ldpc_config = LDPCConfig::default();
    let ldpc_suite = LDPCSuite::new(&protocol.frame_layout, &ldpc_config);
    
    let message_bits = vec![1u8; ldpc_suite.matrices.message_bits];
    
    c.bench_function("generator_matrix_encode", |b| {
        b.iter(|| {
            let mut codeword = vec![0u8; ldpc_suite.matrices.codeword_bits];
            for (row_idx, &bit) in black_box(&message_bits).iter().enumerate() {
                if bit == 0 {
                    continue;
                }
                for (col_idx, codeword_bit) in codeword.iter_mut().enumerate() {
                    *codeword_bit ^= ldpc_suite.matrices.generator[(row_idx, col_idx)] & 1;
                }
            }
            black_box(codeword)
        })
    });
}

criterion_group!(
    benches,
    benchmark_frame_encoding,
    benchmark_symbol_generation_sizes,
    benchmark_generator_matrix_multiply
);
criterion_main!(benches);
