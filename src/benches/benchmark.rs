use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use criterion_cycles_per_byte::CyclesPerByte;
use gnuplot::{Caption, Color, Figure};


use lib::constants::VERSION;
use lib::crypto_primitives::{
    safe_aes_128_ctr, safe_randombytes, safe_randombytes_init, safe_shake256,
};
use lib::mayo_functionality::{api_sign, api_sign_open, compact_key_gen, expand_pk, expand_sk};


fn criterion_benchmark(c: &mut Criterion) {

    println!("\nRUNNING BENCHMARKS FOR {} \n", VERSION);


    

    let mut seed_bytes: Vec<u8> = Vec::with_capacity(24);
    let mut entropy_input: Vec<u8> = (0..=47).collect();
    let personalization_string: Vec<u8> = vec![0u8; 47]; // Example, adjust as necessary
    let nbytes: u64 = entropy_input.len() as u64;

    // Init the randombytes like NIST correctly
    safe_randombytes_init(&mut entropy_input, &personalization_string, 256);
    safe_randombytes(&mut entropy_input, nbytes);

    safe_randombytes_init(&mut seed_bytes, &personalization_string, 256);


    c.bench_function("KeyGen", |bencher| {
        bencher.iter_batched(
            || seed_bytes.clone(),
            |_| {
                compact_key_gen()
            },
            BatchSize::LargeInput,
        );
    });

    c.bench_function("ExpandSK", |bencher| {
        bencher.iter_batched(
            || compact_key_gen(),
            |(_, csk)| {
                expand_sk(&csk)
            },
            BatchSize::LargeInput,
        );
    });

    c.bench_function("ExpandPK", |bencher| {
        bencher.iter_batched(
            || compact_key_gen(),
            |(cpk, _)| {
                expand_pk(cpk)
            },
            BatchSize::LargeInput,
        );
    });

    c.bench_function("ExpandSK + Sign", |bencher| {
        bencher.iter_batched(
            || {
                let (_, csk) = compact_key_gen();
                let mut message = [0u8; 32];
                safe_randombytes(&mut message, 32);
                let message_vec = message.to_vec();

                (message_vec, csk)
            },
            |(message, csk)| {
                api_sign(message, csk)
            },
            BatchSize::LargeInput,
        );
    });

    c.bench_function("ExpandPK + Verify", |bencher| {
        bencher.iter_batched(
            || {
                let (cpk, csk) = compact_key_gen();      
                let mut message = [0u8; 32];
                safe_randombytes(&mut message, 32);
                let message_vec = message.to_vec();

                let signature = api_sign(message_vec, csk);

                (signature, cpk)
            },
            |(message, cpk)| {
                api_sign_open(message, cpk)
            },
            BatchSize::LargeInput,
        );
    });

}

criterion_group!(
    name = my_bench;
    config = Criterion::default(); //.with_measurement(CyclesPerByte)
    targets = criterion_benchmark
    
);
criterion_main!(my_bench);
