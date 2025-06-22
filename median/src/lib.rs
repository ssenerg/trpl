pub fn median(v: &Vec<i32>) -> I32Median {
    let mut sv = SortedI32Slice::new();
    for i in v {
        sv.push(*i);
    }
    sv.median()
}

pub enum I32Median {
    I32(i32),
    F64(f64),
    None,
}

impl I32Median {
    pub fn string(&self) -> String {
        match self {
            I32Median::I32(i) => format!("{}", *i),
            I32Median::F64(i) => format!("{}", *i),
            I32Median::None => format!("none"),
        }
    }
}

struct SortedI32Slice {
    v: Vec<i32>,
}

impl SortedI32Slice {
    fn new() -> Self {
        Self { v: Vec::new() }
    }

    fn push(&mut self, i: i32) {
        if self.v.len() == 0 {
            self.v.push(i);
            return;
        }
        let index = self.binary_search(i, 0, self.v.len() - 1);
        self.v.insert(index, i);
    }

    fn median(&self) -> I32Median {
        let length = self.v.len();
        if length == 0 {
            return I32Median::None;
        }
        if length % 2 == 1 {
            return I32Median::I32(self.v[length / 2]);
        }
        let v = (self.v[length / 2] + self.v[length / 2 - 1]) as f64;
        return I32Median::F64(v / 2.0);
    }

    fn binary_search(&self, i: i32, l: usize, r: usize) -> usize {
        if l == r {
            if self.v[l] >= i {
                return l;
            }
            return l + 1;
        }
        let mut mid = (r + l) / 2;
        if (l + r) % 2 != 0 {
            mid += 1;
        }
        let mid = mid;
        if self.v[mid] == i {
            return mid;
        }
        if self.v[mid] > i {
            return self.binary_search(i, l, mid - 1);
        }
        return self.binary_search(i, mid + 1, r);
    }
}
