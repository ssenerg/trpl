use fibonacci::*;

fn main() {
    let n = 185;
    let bench_count = 100000;
    
    let dur = benchmark(n, FibCalculator::NonRecursice, bench_count);
    println!("NonRecursice took {dur:?}");

    let dur = benchmark(n, FibCalculator::RecursiveSimple, bench_count);
    println!("RecursiveSimple took {dur:?}");

    let dur = benchmark(n, FibCalculator::RecursiveBC, bench_count);
    println!("RecursiveBC took {dur:?}");

    let dur = benchmark(n, FibCalculator::RecursiveOS, bench_count);
    println!("RecursiveOS took {dur:?}");
}
