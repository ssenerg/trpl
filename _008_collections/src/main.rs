use std::{collections::HashMap, vec};

fn main() {
    let mut _v: Vec<i32> = Vec::new();
    let mut _v: Vec<i32> = Vec::with_capacity(20);
    let mut v: Vec<i32> = vec![1, 2, 3, 4];
    v.push(5);
    v.push(6);
    v.push(7);
    let first = v.get(0);
    match first {
        Some(f) => println!("first elements is: {f}"),
        _ => (),
    }
    for i in &mut v {
        *i += 1;
    }
    let first = &mut v[0];
    *first += 0;
    println!("first elements is: {first}\n");
    println!("------- Spread Sheet");
    let row1 = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    let row2 = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    let row3 = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    let rows = vec![row1, row2, row3];
    for r in rows {
        let mut text = String::from("|");
        for c in &r {
            let val = match c {
                SpreadsheetCell::Int(i) => format!(" {i} |"),
                SpreadsheetCell::Text(s) => format!(" {s} |"),
                SpreadsheetCell::Float(f) => format!(" {f} |"),
            };
            text = text + &val[..];
        }
        println!("{text}");
    }
    println!();
    println!("hash map - scores");
    // weirdly no need to declaration tye
    let mut scores = HashMap::with_capacity(2);
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Black"), 50);
    scores.insert(String::from("Black"), 58);
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
    scores.entry(String::from("Black")).or_insert(50);
    println!("{scores:?}\n");

    println!("hash map - text");
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

