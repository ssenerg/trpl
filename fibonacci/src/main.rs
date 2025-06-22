use fibonacci::*;

fn main() {
    let n = 185;
    println!();
    benchmark(n, FibCalculator::NonRecursice, fibonacci_print);
    benchmark(n, FibCalculator::RecursiveOS, fibonacci_print);
    benchmark(n, FibCalculator::RecursiveBC, fibonacci_print);
}
