fn main() {
    let s = String::from("JEEZEZ");
    let _a = (&s, &s);
    let _arr = [&s, &s, &s];
}

fn _loop_label() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn _nth_fibonacci(n: u32) -> u128 {
    if n == 1 || n == 0 {
        return 1;
    }
    return _nth_fibonacci(n - 1) + _nth_fibonacci(n - 2);
}
