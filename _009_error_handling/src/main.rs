use std::error::Error;
use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() -> Result<(), Box<dyn Error>> {
    _ = _read_username_3()?;
    Ok(())
}

fn _lazy_fuck_way() -> File {
    File::open("hello.txt").unwrap()
}

fn _hard_way() -> File {
    match File::open("hello.txt") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(error) => panic!("cannot create file: {error:?}"),
            },
            other => panic!("cannot create file: {other:?}"),
        },
    }
}

fn _moderate_way() -> File {
    File::open("hello.txt").unwrap_or_else(|error| match error.kind() {
        ErrorKind::NotFound => File::create("hello.txt").unwrap(),
        other => panic!("cannot create file: {other:?}"),
    })
}

fn _read_username_1() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn _read_username_2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn _read_username_3() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
