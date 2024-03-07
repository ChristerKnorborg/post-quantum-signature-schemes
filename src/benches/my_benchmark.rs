use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use gnuplot::{Caption, Color, Figure};

use criterion_cycles_per_byte::CyclesPerByte;

use lib::crypto_primitives::{
    safe_aes_128_ctr, safe_randomBytes, safe_randombytes_init, safe_shake256,
};
use lib::mayo_functionality::{api_sign, compact_key_gen};

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

fn criterion_benchmark(c: &mut Criterion, //<CyclesPerByte>
) {
    let mut seed_bytes: Vec<u8> = Vec::with_capacity(24);

    let mut entropy_input: Vec<u8> = (0..=47).collect();
    let personalization_string: Vec<u8> = vec![0u8; 47]; // Example, adjust as necessary
    let nbytes: u64 = entropy_input.len() as u64;

    // Init the randombytes like NIST correctly
    safe_randombytes_init(&mut entropy_input, &personalization_string, 256);
    safe_randomBytes(&mut entropy_input, nbytes);

    safe_randombytes_init(&mut seed_bytes, &personalization_string, 256);
    //compact_key_gen(seed_bytes.clone());

    // fibonacci(20);
    println!("constants {}", lib::constants::M);

    c.bench_function("compact_key_gen SLOWW", |b| {
        b.iter(|| compact_key_gen(black_box(seed_bytes.clone())))
    });

    c.bench_function("compact_key_gen", |bencher| {
        bencher.iter_batched(
            || seed_bytes.clone(),
            |input| {
                compact_key_gen(black_box(input));
            },
            BatchSize::LargeInput,
        );
    });

    //c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(
    name = my_bench;
    config = Criterion::default()
    //.with_measurement(CyclesPerByte)
;
    targets = criterion_benchmark
);
criterion_main!(my_bench);

// criterion_group!(benches, criterion_benchmark);
// criterion_main!(benches);
