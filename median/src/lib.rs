use std::fmt::{Debug, Display};

pub struct SortedVec<T>
where
    T: PartialOrd + PartialEq + Display + Debug,
{
    v: Vec<T>,
}
impl<T> Debug for SortedVec<T>
where
    T: PartialOrd + PartialEq + Display + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self.v);
        f.write_str(&s)
    }
}

impl<T> SortedVec<T>
where
    T: PartialOrd + PartialEq + Display + Debug,
{
    pub fn new() -> Self {
        Self { v: Vec::new() }
    }

    pub fn from_vec(v: Vec<T>) -> Self {
        let mut obj = Self::new();
        for i in v {
            obj.push(i);
        }
        return obj;
    }

    pub fn push(&mut self, i: T) {
        if self.v.len() == 0 {
            self.v.push(i);
            return;
        }
        let index = self.binary_search(&i, 0, self.v.len() - 1);
        self.v.insert(index, i);
    }

    pub fn median(&self) -> Option<&T> {
        let length = self.v.len();
        if length == 0 {
            return None;
        }
        return Some(&self.v[length / 2]);
    }

    fn binary_search(&self, i: &T, l: usize, r: usize) -> usize {
        if l >= r {
            if l > r || self.v[l] >= *i {
                return l;
            }
            return l + 1;
        }
        let mut mid = (r + l) / 2;
        if (l + r) % 2 != 0 {
            mid += 1;
        }
        let mid = mid;
        if self.v[mid] == *i {
            return mid;
        }
        if self.v[mid] > *i {
            return self.binary_search(i, l, mid - 1);
        }
        return self.binary_search(i, mid + 1, r);
    }
}
