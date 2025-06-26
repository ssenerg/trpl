pub trait Shoes {
    fn in_size(&self, shoe_size: u32) -> Self;
    fn in_size_2(self, shoe_size: u32) -> Self;
}

#[derive(PartialEq, Debug, Clone)]
pub struct Shoe {
    pub size: u32,
    pub style: String,
}

impl Shoes for Vec<Shoe> {
    fn in_size(&self, shoe_size: u32) -> Self {
        self.iter()
            .into_iter()
            .filter(|s| s.size == shoe_size)
            .cloned()
            .collect()
    }
    fn in_size_2(self, shoe_size: u32) -> Self {
        self.into_iter().filter(|s| s.size == shoe_size).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];
        assert_eq!(
            shoes.in_size(10),
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
        assert_eq!(
            shoes.in_size_2(10),
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
        // ca not use shoes anymore
    }
}
