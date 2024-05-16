use lib::constants::{M, VERSION};
use lib::write_and_compare_kat_file::write_and_compare_kat_file;

use lib::own_benchmark::benchmark;

fn main() {

    println!("{}" , M);
    println!("{}" , VERSION);

    #[cfg(not(feature = "bench"))]
    {
        write_and_compare_kat_file();
    }


    #[cfg(feature = "bench")]
    {
        println!("bench");
        benchmark(1000);
    }


}