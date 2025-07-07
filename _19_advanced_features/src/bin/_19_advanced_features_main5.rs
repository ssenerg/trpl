use _19_advanced_features::HelloMacro;
use _19_advanced_features::vec;

fn main() {
    let _a = vec![1, 2, 34];
    let _a = vec![1, 2, 3, 4];
    Pancakes::hello_macro();
}

#[derive(hello_macro_derive::HelloMacro)]
struct Pancakes;
