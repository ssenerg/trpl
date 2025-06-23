pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    fn _it_works() -> Result<(), String> {
        let result = add(2, 2);
        if result == 4 {
            return Ok(());
        }
        return Err(String::from("two plus two does not equal to four"));
    }

    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }
}

pub mod rectangle {

    #[derive(Debug)]
    pub struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        pub fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::thread::sleep;
        use std::time::Duration;

        #[test]
        fn larger_can_hold_smaller() {
            let larger = Rectangle {
                width: 8,
                height: 7,
            };
            let smaller = Rectangle {
                width: 5,
                height: 1,
            };

            assert!(larger.can_hold(&smaller));
        }

        #[test]
        fn smaller_cannot_hold_larger() {
            let larger = Rectangle {
                width: 8,
                height: 7,
            };
            let smaller = Rectangle {
                width: 5,
                height: 1,
            };

            assert!(!smaller.can_hold(&larger));
        }

        #[test]
        #[ignore]
        fn time_consuming() {
            sleep(Duration::new(10, 0));
            assert!(true)
        }
    }
}

pub mod guess {
    pub struct Guess {
        _value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 {
                panic!(
                    "Guess value must be greater than or equal to 1, got {}.",
                    value
                );
            } else if value > 100 {
                panic!(
                    "Guess value must be less than or equal to 100, got {}.",
                    value
                );
            }
            Guess { _value: value }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        #[should_panic(expected = "less than or equal to 100")]
        fn greater_than_100() {
            Guess::new(200);
        }
    }
}
