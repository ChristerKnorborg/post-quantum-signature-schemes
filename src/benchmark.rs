use std::time::{Duration, Instant};
use std::fs::{self};

use std::fs::OpenOptions;
use crate::constants::VERSION;

use crate::crypto_primitives::{
    safe_random_bytes, safe_random_bytes_init
};
use crate::mayo_functionality::{api_sign, api_sign_open, compact_key_gen, expand_pk, expand_sk};


use csv::Writer;
use std::error::Error;



#[allow(unused_mut, unused_assignments)]
pub fn benchmark(amount_of_iterations: i32) -> Result<(), Box<dyn Error>> {


    



    
    let implementation_variant = "array_implementation";
    let mut version_string = VERSION.to_string();

    #[cfg(feature = "aes_neon")]
    {
        version_string.push_str("_AES");
    } 

    println!("\nRUNNING BENCHMARKS FOR {} \n", version_string);

    let base_dir = "benchmark_result";
    if !std::path::Path::new(base_dir).exists() {
        fs::create_dir(base_dir)?;
    }

    let file_name = format!("{}_{}.csv", version_string, implementation_variant);
    let file_path = format!("{}/{}", base_dir, file_name);

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file_path)?;
    
    // Create a CSV writer from file handle, automatically handles header writing.
    let mut wtr = Writer::from_writer(file);
    
    // Write the CSV headers
    wtr.write_record(&["Version","keygen", "expand_sk", "expand_pk", "sign+expand_sk", "verify+expand_pk"])?;


    // Flush data to file
    let mut seed_bytes: Vec<u8> = Vec::with_capacity(24);
    let mut entropy_input: Vec<u8> = (0..=47).collect();
    let personalization_string: Vec<u8> = vec![0u8; 47]; // Example, adjust as necessary
    let nbytes: u64 = entropy_input.len() as u64;

    // Init the randombytes like NIST run correctly
    safe_random_bytes_init(&mut entropy_input, &personalization_string, 256);
    safe_random_bytes(&mut entropy_input, nbytes);
    safe_random_bytes_init(&mut seed_bytes, &personalization_string, 256);

    let mut message = [0u8; 32];
    safe_random_bytes(&mut message, 32);
    let _message_vec = message.to_vec();


    let mut durations_keygen = Vec::with_capacity(1000);
    let mut durations_expand_sk = Vec::with_capacity(1000);
    let mut durations_expand_pk = Vec::with_capacity(1000);
    let mut durations_sign = Vec::with_capacity(1000);
    let mut durations_verify = Vec::with_capacity(1000);


    let warm_up_iterations = 50;  







    // KeyGen benchmark
    for _ in 0..warm_up_iterations {
        compact_key_gen();
    }

    for _ in 0..amount_of_iterations{

        let start_keygen = Instant::now(); // Start timer
        compact_key_gen();
        let duration_keygen = start_keygen.elapsed(); // Stop timer

        durations_keygen.push(duration_keygen);
    }

    

   
    // ExpandSK benchmark
    for _ in 0..warm_up_iterations {
        let (_, csk) = compact_key_gen();
        expand_sk(csk);
    }


    for _i in 0..amount_of_iterations{

        let (_ , csk) = compact_key_gen(); // Setup

        let start_expand_sk = Instant::now(); // Start timer
        expand_sk(csk);
        let duration_expand_sk = start_expand_sk.elapsed(); // Stop timer

        durations_expand_sk.push(duration_expand_sk);
    }
    


    

    // ExpandPK benchmark
    for _ in 0..warm_up_iterations {
        let (cpk, _) = compact_key_gen();
        expand_pk(cpk);
    }

    for _ in 0..amount_of_iterations{
        
        let (cpk , _) = compact_key_gen(); // Setup

        let start_expand_pk = Instant::now(); // Start timer
        expand_pk(cpk);
        let duration_expand_pk = start_expand_pk.elapsed(); // Stop timer

        durations_expand_pk.push(duration_expand_pk);
    }
    

    // Sign benchmark
    for _ in 0..warm_up_iterations {
        let (_, csk) = compact_key_gen();
        let mut message = [0u8; 32];
        safe_random_bytes(&mut message, 32);
        let message_vec = message.to_vec();
        let _ = api_sign(message_vec.clone(), csk);
    }


    for _i in 0..amount_of_iterations{

        // Setup
        let (_, csk) = compact_key_gen();
        let mut message = [0u8; 32];
        safe_random_bytes(&mut message, 32);
        let message_vec = message.to_vec();

        let start_sign = Instant::now(); // Start timer
        api_sign(message_vec, csk);
        let duration_sign = start_sign.elapsed(); // Stop timer

        durations_sign.push(duration_sign);
    }
    





    // Verify benchmark
    for _ in 0..warm_up_iterations {
        let (cpk, csk) = compact_key_gen();
        let mut message = [0u8; 32];
        safe_random_bytes(&mut message, 32);
        let message_vec = message.to_vec();
        let signature = api_sign(message_vec.clone(), csk);
        api_sign_open(signature, cpk);
    }


    for _ in 0..amount_of_iterations {

        // Setup
        let (cpk, csk) = compact_key_gen();      
        let mut message = [0u8; 32];
        safe_random_bytes(&mut message, 32);
        let message_vec = message.to_vec();
        let signature = api_sign(message_vec, csk);


        let start_verify = Instant::now(); // Start timer
        api_sign_open(signature, cpk);
        let duration_verify = start_verify.elapsed(); // Stop timer

        durations_verify.push(duration_verify);
    }
    


        let var = 10 as f64;
        let _ = format_duration_as_string(&var);

        durations_keygen.sort();
        durations_expand_sk.sort();
        durations_expand_pk.sort();
        durations_sign.sort();
        durations_verify.sort();
        

        let final_average_duration_keygen = find_median(&durations_keygen);
        let final_average_duration_expand_sk = find_median(&durations_expand_sk);
        let final_average_duration_expand_pk = find_median(&durations_expand_pk);
        let final_average_duration_sign = find_median(&durations_sign);
        let final_average_duration_verify = find_median(&durations_verify);



        let mut res_average_duration_keygen = format_duration_as_nanos(&final_average_duration_keygen);
        let mut res_average_duration_expand_sk = format_duration_as_nanos(&final_average_duration_expand_sk) ;// Replace with format_duration(duration_expand_sk) when enabled
        let mut res_average_duration_expand_pk = format_duration_as_nanos(&final_average_duration_expand_pk);
        let mut res_average_duration_sign = format_duration_as_nanos(&final_average_duration_sign);
        let mut res_average_duration_verify = format_duration_as_nanos(&final_average_duration_verify);
         

        #[cfg(feature = "CCM1")]
        {
            println!("CCM1 is enabled");

            let cpu_speed_hz = 3.2*1e9;

             res_average_duration_keygen = format_duration_as_string(&(cpu_speed_hz * (final_average_duration_keygen.as_nanos() as f64 / 1e9) as f64));
             res_average_duration_expand_sk = format_duration_as_string(&(cpu_speed_hz * (final_average_duration_expand_sk.as_nanos() as f64 / 1e9) as f64));
             res_average_duration_expand_pk = format_duration_as_string(&(cpu_speed_hz * (final_average_duration_expand_pk.as_nanos() as f64 / 1e9) as f64));
             res_average_duration_sign = format_duration_as_string(&(cpu_speed_hz * (final_average_duration_sign.as_nanos() as f64 / 1e9) as f64));
             res_average_duration_verify = format_duration_as_string(&(cpu_speed_hz * (final_average_duration_verify.as_nanos() as f64 / 1e9) as f64));
        }

        #[cfg(feature = "CCODROID-C4")]
        {
            println!("CCODROID-C4 is enabled");

            
            let cpu_speed_hz = 1.91*1e9;

             res_average_duration_keygen = format_duration_as_string(&(cpu_speed_hz * (final_average_duration_keygen.as_nanos() as f64 / 1e9) as f64));
             res_average_duration_expand_sk = format_duration_as_string(&(cpu_speed_hz * (final_average_duration_expand_sk.as_nanos() as f64 / 1e9) as f64));
             res_average_duration_expand_pk = format_duration_as_string(&(cpu_speed_hz * (final_average_duration_expand_pk.as_nanos() as f64 / 1e9) as f64));
             res_average_duration_sign = format_duration_as_string(&(cpu_speed_hz * (final_average_duration_sign.as_nanos() as f64 / 1e9) as f64));
             res_average_duration_verify = format_duration_as_string(&(cpu_speed_hz * (final_average_duration_verify.as_nanos() as f64 / 1e9) as f64));

        
        }


    wtr.write_record(&[
        &VERSION.to_string(),
        &res_average_duration_keygen,
        &res_average_duration_expand_sk, // Replace with format_duration(duration_expand_sk) when enabled
        &res_average_duration_expand_pk,
        &res_average_duration_sign,
        &res_average_duration_verify,
    ])?;


    wtr.flush()?;

    Ok(())

}

fn format_duration_as_nanos(dur: &Duration) -> String {
    format!("{:.5?}", dur.as_nanos())
}

fn format_duration_as_string(dur: &f64) -> String {
    format!("{:.0?}", dur)
}

fn average_duration(d1: Duration, d2: Duration) -> Duration {
    let total_nanos = d1.as_nanos() + d2.as_nanos();
    Duration::from_nanos((total_nanos / 2) as u64)
}

fn find_median(durations: &Vec<Duration>) -> Duration {
    let len = durations.len();
    if len % 2 == 0 {
        // Even number of elements, take the average of the two middle elements
        let mid1 = durations[len / 2 - 1];
        let mid2 = durations[len / 2];
        average_duration(mid1, mid2)
    } else {
        // Odd number of elements, take the middle element
        durations[len / 2]
    }
}