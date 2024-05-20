use std::time::{Duration, Instant};
use std::fs::{self};

use std::fs::OpenOptions;
use crate::constants::{VERSION};

use crate::crypto_primitives::{
    safe_random_bytes, safe_random_bytes_init
};
use crate::mayo_functionality::{api_sign, api_sign_open, compact_key_gen, expand_pk, expand_sk};


use csv::Writer;
use std::error::Error;


pub fn benchmark(amount_of_iterations: i32) -> Result<(), Box<dyn Error>> {


    println!("\nRUNNING BENCHMARKS FOR {} \n", VERSION);
    
let implementation_variant = "Array_implementation";

    let base_dir = "benchmark_result";
    if !std::path::Path::new(base_dir).exists() {
        fs::create_dir(base_dir)?;
    }

    

            // Construct the file name with the specified pattern including the implementation variant
            let file_name = format!("benchmark-{}-{}.csv", VERSION, implementation_variant);

            // Combine the directory path and file name to get the full file path
            let file_path = format!("{}/{}", base_dir, file_name);

    

    // Open a file in write mode, this will create or truncate the file if it exists.
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file_path)?;
    
    // Create a CSV writer from file handle, automatically handles header writing.
    let mut wtr = Writer::from_writer(file);
    
    // Write the CSV headers
    wtr.write_record(&["Version","keygen", "expand_sk", "expand_pk", "sign+expand_sk", "verify+expand_pk"])?;

    // Here you would include your benchmarking logic and write the data like so:
    // wtr.write_record(&[keygen_result, expand_sk_result, expand_pk_result, sign_result, verify_result])?;

    // Ensure all data is flushed to the file

    let mut seed_bytes: Vec<u8> = Vec::with_capacity(24);
    let mut entropy_input: Vec<u8> = (0..=47).collect();
    let personalization_string: Vec<u8> = vec![0u8; 47]; // Example, adjust as necessary
    let nbytes: u64 = entropy_input.len() as u64;

    // Init the randombytes like NIST correctly
    safe_random_bytes_init(&mut entropy_input, &personalization_string, 256);
    safe_random_bytes(&mut entropy_input, nbytes);

    safe_random_bytes_init(&mut seed_bytes, &personalization_string, 256);

    let mut message = [0u8; 32];
    safe_random_bytes(&mut message, 32);
    let _message_vec = message.to_vec();



    //this loop runs the benchmark for the keygen function
    let mut total_duration_keygen = Duration::new(0, 0);
    for _ in 0..amount_of_iterations{


        let start_keygen = Instant::now();

        compact_key_gen();
        
        let duration_keygen = start_keygen.elapsed();

        total_duration_keygen += duration_keygen;
    }

    let final_average_duration_keygen = total_duration_keygen / amount_of_iterations.try_into().unwrap();



        //this loop runs the benchmark for the expand sk function
        let mut total_duration_expand_sk = Duration::new(0, 0);
        for _i in 0..amount_of_iterations{

        let (_ , csk) = compact_key_gen();
    
    
        let start_expand_sk = Instant::now();
    

            expand_sk(csk);
        //  expand_pk(cpk);
    
        //  let signature = api_sign(message_vec, csk);
    
        // api_sign_open(signature, cpk);
        
        let duration_expand_sk = start_expand_sk.elapsed();

        total_duration_expand_sk += duration_expand_sk;
        }
    
        let final_average_duration_expand_sk = total_duration_expand_sk / amount_of_iterations.try_into().unwrap();


        
        //this loop runs the benchmark for the expand_pk function
        let mut total_duration_expand_pk = Duration::new(0, 0);
        for _i in 0..amount_of_iterations{

        let (cpk , _) = compact_key_gen();
    
    
        let start_expand_pk = Instant::now();
    

            //expand_sk(csk);
            expand_pk(cpk);
    
        //  let signature = api_sign(message_vec, csk);
    
        // api_sign_open(signature, cpk);
        
        let duration_expand_pk = start_expand_pk.elapsed();

        total_duration_expand_pk += duration_expand_pk;
        }
    
        let final_average_duration_expand_pk = total_duration_expand_pk / amount_of_iterations.try_into().unwrap();


        //this loop runs the benchmark for the sign function
        let mut total_duration_sign = Duration::new(0, 0);
        for _i in 0..amount_of_iterations{

            let (_, csk) = compact_key_gen();
            let mut message = [0u8; 32];
            safe_random_bytes(&mut message, 32);
            let message_vec = message.to_vec();
    
        let start_sign = Instant::now();
    
        api_sign(message_vec, csk);
    
        let duration_sign = start_sign.elapsed();

        total_duration_sign += duration_sign;
        }
    
        let final_average_duration_sign = total_duration_sign / amount_of_iterations.try_into().unwrap();


        //this loop runs the benchmark for the verify function
        let mut total_duration_verify = Duration::new(0, 0);
        for _i in 0..amount_of_iterations{

            let (cpk, csk) = compact_key_gen();      
            let mut message = [0u8; 32];
            safe_random_bytes(&mut message, 32);
            let message_vec = message.to_vec();

            let signature = api_sign(message_vec, csk);
    
        let start_verify = Instant::now();

        api_sign_open(signature, cpk);
        
        let duration_verify = start_verify.elapsed();

        total_duration_verify += duration_verify;
        }
    
        let final_average_duration_verify = total_duration_verify / amount_of_iterations.try_into().unwrap();



        #[cfg(feature = "CCM1")]
        {
            println!("CCM1 is enabled");

            let cpu_speed_hz = 3.2*1e9;



            let res_average_duration_keygen = format_duration_as_string(&(cpu_speed_hz * (final_average_duration_keygen.as_nanos() as f64 / 1e9) as f64));
            let res_average_duration_expand_sk = format_duration_as_string(&(cpu_speed_hz * (final_average_duration_expand_sk.as_nanos() as f64 / 1e9) as f64));
            let res_average_duration_expand_pk = format_duration_as_string(&(cpu_speed_hz * (final_average_duration_expand_pk.as_nanos() as f64 / 1e9) as f64));
            let res_average_duration_sign = format_duration_as_string(&(cpu_speed_hz * (final_average_duration_sign.as_nanos() as f64 / 1e9) as f64));
            let res_average_duration_verify = format_duration_as_string(&(cpu_speed_hz * (final_average_duration_verify.as_nanos() as f64 / 1e9) as f64));


            wtr.write_record(&[
                &VERSION.to_string(),
                &res_average_duration_keygen,
                &res_average_duration_expand_sk, // Replace with format_duration(duration_expand_sk) when enabled
                &res_average_duration_expand_pk,
                &res_average_duration_sign,
                &res_average_duration_verify,
            ])?;
        
        
            wtr.flush()?;
        
            return Ok(());


        }

        #[cfg(feature = "CCODROID-C4")]
        {
            println!("CCODROID-C4 is enabled");

            
            let cpu_speed_hz = 1.91*1e9;

            let res_average_duration_keygen = format_duration_as_string(&(cpu_speed_hz * (final_average_duration_keygen.as_nanos() as f64 / 1e9) as f64));
            let res_average_duration_expand_sk = format_duration_as_string(&(cpu_speed_hz * (final_average_duration_expand_sk.as_nanos() as f64 / 1e9) as f64));
            let res_average_duration_expand_pk = format_duration_as_string(&(cpu_speed_hz * (final_average_duration_expand_pk.as_nanos() as f64 / 1e9) as f64));
            let res_average_duration_sign = format_duration_as_string(&(cpu_speed_hz * (final_average_duration_sign.as_nanos() as f64 / 1e9) as f64));
            let res_average_duration_verify = format_duration_as_string(&(cpu_speed_hz * (final_average_duration_verify.as_nanos() as f64 / 1e9) as f64));


            wtr.write_record(&[
                &VERSION.to_string(),
                &res_average_duration_keygen,
                &res_average_duration_expand_sk, // Replace with format_duration(duration_expand_sk) when enabled
                &res_average_duration_expand_pk,
                &res_average_duration_sign,
                &res_average_duration_verify,
            ])?;
        
        
            wtr.flush()?;
        
            return Ok(());
        
        }



        let  res_average_duration_keygen = format_duration_as_nanos(&final_average_duration_keygen);
        let  res_average_duration_expand_sk =format_duration_as_nanos(&final_average_duration_expand_sk) ;// Replace with format_duration(duration_expand_sk) when enabled
        let  res_average_duration_expand_pk = format_duration_as_nanos(&final_average_duration_expand_pk);
        let  res_average_duration_sign = format_duration_as_nanos(&final_average_duration_sign);
        let  res_average_duration_verify = format_duration_as_nanos(&final_average_duration_verify);
         


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