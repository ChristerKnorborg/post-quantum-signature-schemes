use lib::write_and_compare_kat_file::write_and_compare_kat_file;

use lib::own_benchmark::benchmark;

fn main() {
    write_and_compare_kat_file();

    benchmark(1000);

}
