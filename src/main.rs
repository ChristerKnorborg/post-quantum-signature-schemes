mod sample;
mod finite_field;

fn main() {
    // Example usage:
    let a = vec![
        vec![1.0, 2.0],
        vec![3.0, 4.0],
    ];
    let y = vec![5.0, 11.0];
    let r = vec![0.5; a[0].len()]; // Random vector r of appropriate size

    match sample::sample_solution(a, y, r) {
        Ok(solution) => println!("Solution x: {:?}", solution),
        Err(error) => println!("Error: {}", error),
    }
}