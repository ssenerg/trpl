use _19_advanced_features::advanced_functions_and_closures::*;

fn main() {
    let answer = do_twice(add_one, 5); // (5 + 1) + (5 + 1)
    println!("The answer is: {answer}");

    let list_of_numbers: Vec<i32> = vec![1, 2, 3];
    let _list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    let _list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();

    let _list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
}
