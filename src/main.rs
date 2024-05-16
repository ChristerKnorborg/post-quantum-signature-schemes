use lib::constants::{M, VERSION};
use lib::write_and_compare_kat_file::write_and_compare_kat_file;

use lib::benchmark::benchmark;

fn main() {



    #[cfg(not(feature = "bench"))]
    {
        write_and_compare_kat_file();
    }


    #[cfg(feature = "bench")]
    {
        benchmark(1000);
    }


}
