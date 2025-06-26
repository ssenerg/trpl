fn main() {
    let move_ = Message::Move { x: 3, y: 4 };
    let val = move_.call();
    println!("{val}");

    println!("some num");
    let some_num: Option<i32> = Some(7);
    match some_num {
        Some(x) => println!("{}", x),
        None => println!("none"),
    }

    println!("dice");
    let dice_roll = 9;
    let rolled = match dice_roll {
        3 => 3,
        7 => 3,
        other => other,
    };
    println!("{rolled}");

    let _value: Option<i32> = Some(4);
    let value: Option<i32> = None;
    if let Some(val) = value {
        println!("value is : {val}")
    } else if let None = value {
        println!("value is : none")
    }
}

pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) -> i32 {
        match self {
            Message::Quit => 0,
            Message::Move { y: 2, x } => *x,
            Message::Move {
                y: i32::MIN..=1, ..
            } => 11,
            Message::Move {
                y: 2..=i32::MAX, ..
            } => 12,
            Message::Write(..) => 2,
            Message::ChangeColor(..) => 3,
        }
    }
}
