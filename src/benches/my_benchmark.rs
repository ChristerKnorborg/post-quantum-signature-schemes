use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use gnuplot::{Caption, Color, Figure};

use criterion_cycles_per_byte::CyclesPerByte;

use lib::crypto_primitives::{
    safe_aes_128_ctr, safe_randomBytes, safe_randombytes_init, safe_shake256,
};
use lib::mayo_functionality::{api_sign, compact_key_gen, expand_pk, expand_sk};

fn fibonacci(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;

    match n {
        0 => b,
        _ => {
            for _ in 0..n {
                let c = a + b;
                a = b;
                b = c;
            }
            b
        }
    }
}

fn criterion_benchmark(c: &mut Criterion<CyclesPerByte>) {
    let mut seed_bytes: Vec<u8> = Vec::with_capacity(24);

    let mut entropy_input: Vec<u8> = (0..=47).collect();
    let personalization_string: Vec<u8> = vec![0u8; 47]; // Example, adjust as necessary
    let nbytes: u64 = entropy_input.len() as u64;

    // Init the randombytes like NIST correctly
    safe_randombytes_init(&mut entropy_input, &personalization_string, 256);
    safe_randomBytes(&mut entropy_input, nbytes);

    safe_randombytes_init(&mut seed_bytes, &personalization_string, 256);
    //compact_key_gen(seed_bytes.clone());

    println!("constants {}", lib::constants::M);

    // c.bench_function("compact_key_gen SLOWW", |bencher| {
    //     bencher.iter(|| compact_key_gen(seed_bytes.clone()))
    // });

    c.bench_function("compact_key_gen", |bencher| {
        bencher.iter_batched(
            || seed_bytes.clone(),
            |input| {
                compact_key_gen(input);
            },
            BatchSize::LargeInput,
        );
    });

    c.bench_function("expand secret key", |bencher| {
        bencher.iter_batched(
            || compact_key_gen(seed_bytes.clone()),
            |(_, csk)| {
                expand_sk(&csk);
            },
            BatchSize::LargeInput,
        );
    });

    c.bench_function("expand public key", |bencher| {
        bencher.iter_batched(
            || compact_key_gen(seed_bytes.clone()),
            |(cpk, _)| {
                expand_pk(cpk);
            },
            BatchSize::LargeInput,
        );
    });

    // let mut message: Vec<u8> = (0..=255).collect();

    // c.bench_function("api sign", |bencher| {
    //     bencher.iter_batched(
    //         || {
    //             let (cpk, csk) = compact_key_gen(seed_bytes.clone());

    //         },
    //         |(_, csk)| {
    //             api_sign(message, csk);
    //         },
    //         BatchSize::LargeInput,
    //     );
    // });

    //c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(
    name = my_bench;
    config = Criterion::default().with_measurement(CyclesPerByte);
    targets = criterion_benchmark
);
criterion_main!(my_bench);

// criterion_group!(benches, criterion_benchmark);
// criterion_main!(benches);
