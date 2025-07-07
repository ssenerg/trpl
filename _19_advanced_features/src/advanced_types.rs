use std::fmt;

pub type Kilometers = i32;

/// real usage of alias type
pub type Thunk = Box<dyn Fn() + Send + 'static>;

pub fn takes_long_type(f: Thunk) {
    _ = f;
}

pub fn returns_long_type() -> Thunk {
    Box::new(|| {
        println!("hi");
    })
}

/// Important usage of alias types
type Result<T> = std::result::Result<T, std::io::Error>;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}

/// Never return type
pub fn forever() -> ! {
    println!("forever ");

    loop {
        println!("and ever ");
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn alias_example() {
        type Kilometers = i32;
        let x: i32 = 5;
        let y: Kilometers = 5;
        assert_eq!(10, x + y);
    }
}
