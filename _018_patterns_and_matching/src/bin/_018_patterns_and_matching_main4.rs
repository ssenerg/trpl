fn main() {
    khesus_nested_();
    some();
    remaining_part_with_dot_dot();
    remaining_part_with_dot_dot_even_in_tuple();
}

fn khesus_nested_() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);
}

fn some() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}");
        }
    }
}


#[allow(dead_code)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn remaining_part_with_dot_dot() { // IMPORTANT
    let origin = Point { x: 0, y: 0, z: 0 };
    match origin {
        Point { x, .. } => println!("x is {x}"),
    }
}

fn remaining_part_with_dot_dot_even_in_tuple() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("First and Last numbers: {first}, {last}");
        }
    }
    // xan not use two .. in tuple
    // match numbers {
    //     (.., second, ..) => {
    //         println!("Second number: {second}");
    //     }
    // }
}