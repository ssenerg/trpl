use std::collections::HashMap;
use std::time::{Instant, Duration};

pub enum FibCalculator {
    NonRecursice,
    RecursiveOS,
    RecursiveBC,
    RecursiveSimple,
}

pub fn benchmark(n: u32, calculator: FibCalculator, bc: u32) -> Duration {
    let mut bc_ = bc;
    if bc == 0 {
        bc_ = 1;
    }
    let now = Instant::now();
    for _ in 0..=(bc_ - 1) {
        _ = fibonacci(n, &calculator);
    }
    now.elapsed().div_f64(bc_ as f64)
}

pub fn fibonacci(n: u32, calculator: &FibCalculator) -> u128 {
    match calculator {
        FibCalculator::NonRecursice => fibonacci_non_recursive(n),
        FibCalculator::RecursiveOS => fibonacci_recursive(n),
        FibCalculator::RecursiveBC => fibonacci_recursive_bc(n),
        FibCalculator::RecursiveSimple => fibonacci_recursive_simple(n),
    }
}

fn fibonacci_non_recursive(n: u32) -> u128 {
    let mut a: u128 = 1;
    let mut b: u128 = 1;
    for _ in 2..=n {
        (a, b) = (b, a + b);
    }
    return b;
}

fn fibonacci_recursive(n: u32) -> u128 {
    let store = HashMap::with_capacity(n.try_into().unwrap());
    let (out, _) = fibonacci_recursive_(n, store);
    return out;
}

fn fibonacci_recursive_bc(n: u32) -> u128 {
    let mut store = HashMap::with_capacity(n.try_into().unwrap());
    return fibonacci_recursive_bc_(n, &mut store);
}

fn fibonacci_recursive_simple(n: u32) -> u128 {
    let (n_2, n_1) = fibonacci_recursive_simple_(n);
    n_2 + n_1
}

fn fibonacci_recursive_(n: u32, mut store: HashMap<u32, u128>) -> (u128, HashMap<u32, u128>) {
    if n == 0 || n == 1 {
        return (1, store);
    }
    let n_1 = match store.get(&(n - 1)) {
        Some(v) => *v,
        None => {
            let (n_1_, store_) = fibonacci_recursive_(n - 1, store);
            store = store_;
            n_1_
        }
    };
    let n_2 = match store.get(&(n - 2)) {
        Some(v) => *v,
        None => {
            let (n_2_, store_) = fibonacci_recursive_(n - 2, store);
            store = store_;
            n_2_
        }
    };
    let sum = n_1 + n_2;
    store.insert(n, sum);
    return (sum, store);
}

fn fibonacci_recursive_bc_(n: u32, store: &mut HashMap<u32, u128>) -> u128 {
    if n == 0 || n == 1 {
        return 1;
    }
    let n_1 = match store.get(&(n - 1)) {
        Some(v) => *v,
        None => {
            let n_1_ = fibonacci_recursive_bc_(n - 1, store);
            n_1_
        }
    };
    let n_2 = match store.get(&(n - 2)) {
        Some(v) => *v,
        None => {
            let n_2_ = fibonacci_recursive_bc_(n - 2, store);
            n_2_
        }
    };
    let sum = n_1 + n_2;
    store.insert(n, sum);
    return sum;
}

fn fibonacci_recursive_simple_(n: u32) -> (u128, u128) {
    if n == 0 || n == 1 {
        return (0, 1);
    }
    let (n_3, n_2) = fibonacci_recursive_simple_(n - 1);
    return (n_2, n_3 + n_2);
}