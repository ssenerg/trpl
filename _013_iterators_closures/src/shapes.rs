use num_traits::Zero;
use std::cmp::Ordering;
use std::fmt::{self, Debug, Display};
use std::ops::{Add, Div, Mul, Rem, Sub};

pub trait Shapes<T>
where
    T: Ord
        + Copy
        + Debug
        + Display
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Rem<Output = T>
        + Zero,
{
    fn area(&self) -> T;
    fn pretty(&self, size: Option<usize>) -> String;
}

impl<S, T> Shapes<T> for Vec<S>
where
    S: Shape<T> + ToString,
    T: Ord
        + Copy
        + Debug
        + Display
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Rem<Output = T>
        + Zero,
{
    fn area(&self) -> T {
        self.iter().fold(T::zero(), |sum, shape| sum + shape.area())
    }

    fn pretty(&self, size: Option<usize>) -> String {
        let size = size.unwrap_or_else(|| 3);
        let mut shape_strings: Vec<String> = vec![];
        for (i, s) in self.iter().enumerate() {
            if i == size {
                shape_strings.push(String::from("..."));
                break;
            }
            shape_strings.push(s.to_string());
        }
        if shape_strings.is_empty() {
            return String::from("[]");
        }
        format!("[ {} ]", shape_strings.join(" "))
    }
}
pub trait Shape<T>
where
    T: Ord
        + Copy
        + Debug
        + Display
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Rem<Output = T>
        + Zero,
{
    fn area(&self) -> T;
    fn can_hold(&self, other: &Self) -> bool;
}

#[derive(Eq, PartialEq)]
pub struct Rectangle<T>
where
    T: Ord
        + Copy
        + Debug
        + Display
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Rem<Output = T>
        + Zero,
{
    width: T,
    height: T,
    area: T,
}

impl<T> Rectangle<T>
where
    T: Ord
        + Copy
        + Debug
        + Display
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Rem<Output = T>
        + Zero,
{
    pub fn new(width: T, height: T) -> Self {
        Self {
            width,
            height,
            area: width * height,
        }
    }

    pub fn width(&self) -> T {
        self.width
    }

    pub fn height(&self) -> T {
        self.height
    }
}

impl<T> Debug for Rectangle<T>
where
    T: Ord
        + Copy
        + Debug
        + Display
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Rem<Output = T>
        + Zero,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl<T> ToString for Rectangle<T>
where
    T: Ord
        + Copy
        + Debug
        + Display
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Rem<Output = T>
        + Zero,
{
    fn to_string(&self) -> String {
        format!("Rectangle::{}x{}", self.width, self.height)
    }
}

impl<T> Shape<T> for Rectangle<T>
where
    T: Ord
        + Copy
        + Debug
        + Display
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Rem<Output = T>
        + Zero,
{
    fn area(&self) -> T {
        self.area
    }
    fn can_hold(&self, other: &Self) -> bool {
        match self.width.cmp(&other.width) {
            Ordering::Greater => match self.height.cmp(&other.height) {
                Ordering::Greater => true,
                _ => false,
            },
            _ => false,
        }
    }
}

impl<T> Ord for Rectangle<T>
where
    T: Ord
        + Copy
        + Debug
        + Display
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Rem<Output = T>
        + Zero,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.area.cmp(&other.area)
    }
}

impl<T> PartialOrd for Rectangle<T>
where
    T: Ord
        + Copy
        + Debug
        + Display
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Rem<Output = T>
        + Zero,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.area.partial_cmp(&other.area)
    }
}

#[cfg(test)]
mod tests {
    use crate::shapes::{Rectangle, Shape};

    #[test]
    fn koon_goshadi() {
        let recs = vec![
            Rectangle::new(10, 12),
            Rectangle::new(10, 13),
            Rectangle::new(11, 12),
            Rectangle::new(11, 13),
        ];
        assert!(recs[3].can_hold(&recs[0]));
        for i in 0..=2 {
            for j in 1..=3 {
                assert!(!recs[i].can_hold(&recs[j]));
            }
        }
        assert!(recs[0] == recs[0]);
        assert!(recs[0] <= recs[0]);
        assert!(recs[0] >= recs[0]);
        assert!(recs[0] < recs[1]);
        assert!(recs[0] <= recs[1]);
        assert!(recs[1] == recs[1]);
        assert!(recs[1] <= recs[1]);
        assert!(recs[1] >= recs[1]);
        assert!(recs[1] > recs[0]);
        assert!(recs[1] >= recs[0]);
    }
}
