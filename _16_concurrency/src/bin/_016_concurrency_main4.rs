use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));

    // Create a scope for the threads
    thread::scope(|s| {
        // Spawn 100 threads
        for _ in 0..10_000 {
            let counter = Arc::clone(&counter);
            // #############
            s.spawn(move || {
                // ### IMPORTANT
                // #############
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
        }
        // All threads are automatically joined when the scope ends
    });

    println!("Result: {}", *counter.lock().unwrap());
}
