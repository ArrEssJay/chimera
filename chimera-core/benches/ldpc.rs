use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use chimera_core::{
    config::{ProtocolConfig, LDPCConfig, FrameLayout},
    ldpc::{LDPCSuite, decode_ldpc},
};
use rand::{rngs::StdRng, Rng, SeedableRng};

fn benchmark_ldpc_encoding(c: &mut Criterion) {
    let layout = FrameLayout::default();
    let ldpc_config = LDPCConfig::default();
    let suite = LDPCSuite::new(&layout, &ldpc_config);
    
    let mut rng = StdRng::seed_from_u64(42);
    let message: Vec<u8> = (0..layout.message_bits())
        .map(|_| if rng.gen_bool(0.5) { 1 } else { 0 })
        .collect();
    
    c.bench_function("ldpc_encode_default_size", |b| {
        b.iter(|| {
            let mut codeword = vec![0u8; suite.matrices.codeword_bits];
            for (row_idx, &bit) in black_box(&message).iter().enumerate() {
                if bit == 0 {
                    continue;
                }
                for (col_idx, codeword_bit) in codeword.iter_mut().enumerate() {
                    *codeword_bit ^= suite.matrices.generator[(row_idx, col_idx)] & 1;
                }
            }
            black_box(codeword)
        })
    });
}

fn benchmark_ldpc_decoding(c: &mut Criterion) {
    let layout = FrameLayout::default();
    let ldpc_config = LDPCConfig::default();
    let suite = LDPCSuite::new(&layout, &ldpc_config);
    
    let mut rng = StdRng::seed_from_u64(42);
    let message: Vec<u8> = (0..layout.message_bits())
        .map(|_| if rng.gen_bool(0.5) { 1 } else { 0 })
        .collect();
    
    // Encode
    let mut codeword = vec![0u8; suite.matrices.codeword_bits];
    for (row_idx, &bit) in message.iter().enumerate() {
        if bit == 0 {
            continue;
        }
        for (col_idx, codeword_bit) in codeword.iter_mut().enumerate() {
            *codeword_bit ^= suite.matrices.generator[(row_idx, col_idx)] & 1;
        }
    }
    
    c.bench_function("ldpc_decode_no_errors", |b| {
        b.iter(|| {
            decode_ldpc(black_box(&suite.matrices), black_box(&codeword), 0.0)
        })
    });
    
    // Add some bit errors
    let mut noisy_codeword = codeword.clone();
    for i in (0..noisy_codeword.len()).step_by(20) {
        noisy_codeword[i] ^= 1;
    }
    
    c.bench_function("ldpc_decode_with_errors", |b| {
        b.iter(|| {
            decode_ldpc(black_box(&suite.matrices), black_box(&noisy_codeword), 0.0)
        })
    });
}

fn benchmark_ldpc_construction(c: &mut Criterion) {
    let mut group = c.benchmark_group("ldpc_construction");
    
    for &total_symbols in [64, 128, 256, 512].iter() {
        let layout = FrameLayout {
            total_symbols,
            sync_symbols: total_symbols / 8,
            target_id_symbols: total_symbols / 8,
            command_type_symbols: total_symbols / 8,
            data_payload_symbols: total_symbols / 2,
            ecc_symbols: total_symbols / 8,
        };
        
        group.bench_with_input(
            BenchmarkId::from_parameter(total_symbols),
            &layout,
            |b, layout| {
                let ldpc_config = LDPCConfig::default();
                b.iter(|| {
                    LDPCSuite::new(black_box(layout), black_box(&ldpc_config))
                })
            },
        );
    }
    
    group.finish();
}

fn benchmark_matrix_operations(c: &mut Criterion) {
    let layout = FrameLayout::default();
    let ldpc_config = LDPCConfig::default();
    let suite = LDPCSuite::new(&layout, &ldpc_config);
    
    let test_vector = vec![1u8; suite.matrices.message_bits];
    
    c.bench_function("matrix_vector_multiply", |b| {
        b.iter(|| {
            let mut result = vec![0u8; suite.matrices.codeword_bits];
            for (i, &bit) in black_box(&test_vector).iter().enumerate() {
                if bit == 1 {
                    for j in 0..suite.matrices.codeword_bits {
                        result[j] ^= suite.matrices.generator[(i, j)];
                    }
                }
            }
            black_box(result)
        })
    });
}

criterion_group!(
    benches,
    benchmark_ldpc_encoding,
    benchmark_ldpc_decoding,
    benchmark_ldpc_construction,
    benchmark_matrix_operations
);
criterion_main!(benches);
