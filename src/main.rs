
fn main() {


    #[cfg(not(feature = "bench"))]
    {
        use lib::write_and_compare_kat_file::write_and_compare_kat_file;
        write_and_compare_kat_file();
    }


    #[cfg(feature = "bench")]
    {
        use lib::benchmark::benchmark;
        let _ = benchmark(1000);
    }


}