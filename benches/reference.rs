use base64::{engine::general_purpose, Engine};
use criterion::{criterion_group, criterion_main, Criterion};
use human_bytes::human_bytes;
use rand::{thread_rng, Rng};

pub fn criterion_benchmark(c: &mut Criterion) {
    {
        let input_data = include_bytes!("./rfc6214.txt");

        c.bench_function(
            &format!("reference base64 rfc6214.txt ({} bytes)", input_data.len()),
            |b| {
                b.iter_batched_ref(
                    || -> Vec<u8> { vec![0; input_data.len() * 2] },
                    |v| general_purpose::STANDARD.encode_slice(input_data, v),
                    criterion::BatchSize::SmallInput,
                )
            },
        );
    }
    for length in [
        4096, 8192, 32768, 65536, 524288, 107374182, /*1073741824*/
    ] {
        let mut rng = thread_rng();
        let input = (0..length).map(|_| rng.gen()).collect::<Vec<u8>>();

        c.bench_function(
            &format!("reference base64 random bytes ({})", human_bytes(length as f64)),
            |b| {
                b.iter_batched_ref(
                    || -> Vec<u8> { vec![0; length * 2] },
                    |v| general_purpose::STANDARD.encode_slice(&input, v),
                    criterion::BatchSize::SmallInput,
                )
            },
        );
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
