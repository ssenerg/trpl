use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    thread::spawn(move || {
        // move is neccessary
        println!("From thread: {:?}", list)
    })
    .join()
    .unwrap();
    //                                             This cause error
    // println!("After defining closure: {:?}", list); // #########
    //                                                 // Important
    //                                                 // #########
}
