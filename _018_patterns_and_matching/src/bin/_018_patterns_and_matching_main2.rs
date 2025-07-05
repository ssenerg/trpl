fn main() {
    fun1();
    fun2();
    print_coordinates(&(3, 5));
    match_i32(1);
    fun3();
    multiple_pattern(1);
    range_pattern(4.9999999);
    range_pattern(5.0);
    range_even_ascii('c');
    jeezez_structs();
    fun4();
    previously_on_book();
    fun5();
}


fn fun1() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{top}");
    }
}
fn fun2() {
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({x}, {y})");
}

fn match_i32(x: i32) {
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn fun3() {
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"), // shadowing is possible
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {y}", x);
}

fn multiple_pattern(x: i32) {
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}


fn range_pattern(x: f64) {
    match x {
        1.0..5.0 => println!("one through five"),
        _ => println!("something else"),
    }
}


fn range_even_ascii(x: char) {
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

struct Point {
    x: i32,
    y: i32,
}

fn jeezez_structs() {
    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p;
    let Point { x: a, y: b } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
    assert_eq!(0, a);
    assert_eq!(7, b);
}


fn fun4() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }
}

#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn previously_on_book() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!(
                "The Quit variant has no data to destructure."
            );
        }
        Message::Move { x, y } => {
            println!(
                "Move in the x dir {x}, in the y dir {y}"
            );
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => println!(
            "Change color to red {r}, green {g}, and blue {b}"
        ),
    }
}

fn fun5() {
    let ((feet, inches), Point { x, y }) = 
    ((3, 10), Point { x: 3, y: -10 });
    (_, _, _, _) = (feet, inches, x, y)
}