use lib::write_and_compare_kat_file::write_and_compare_kat_file;

#[allow(unused_imports)]
use lib::benchmark::benchmark;

fn main() {



    #[cfg(not(feature = "bench"))]
    {
        write_and_compare_kat_file();
    }


    #[cfg(feature = "bench")]
    {
        let _res = benchmark(1000);
    }


}
