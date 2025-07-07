static HELLO_WORLD: &str = "Hello, world!";

static mut COUNTER: u32 = 0;

fn main() {
    println!("value is: {HELLO_WORLD}");
    add_to_count(3);

    // unsafe {
    // why we can not use?
    // println!("COUNTER: {COUNTER}");
    // }
}

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
