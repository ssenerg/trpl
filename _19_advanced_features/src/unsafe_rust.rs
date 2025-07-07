/// Notice that we don’t include the unsafe keyword in this code.
/// We can create raw pointers in safe code;
/// we just can’t dereference raw pointers outside an unsafe block.
pub fn fun1() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    (_, _) = (r1, r2);
}

/// Creating a raw pointer to an arbitrary memory address
pub fn fun2() {
    let address: usize = 0x012345;
    let r = address as *const i32;
    _ = r;
}

/// Dereferencing raw pointers within an unsafe block
pub fn fun3() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}

pub fn fun4() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

pub fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

unsafe extern "C" {
    pub unsafe fn abs(input: i32) -> i32;
}

// why we can not use ?????
// #[no_mangle]
// pub unsafe extern "C" fn call_from_c() {
//     println!("Just called a Rust function from C!");
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        fun1();
    }

    #[test]
    fn test_2() {
        fun2();
    }

    #[test]
    fn test_3() {
        fun3();
    }

    #[test]
    fn test_4() {
        fun4();
    }

    #[test]
    fn split_at_mut_test() {
        let mut v = vec![1, 2, 3, 4, 5, 6];

        let r = &mut v[..];
        let (a, b) = split_at_mut(r, 3);

        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);
    }

    #[test]
    fn c_abs() {
        unsafe {
            assert_eq!(3, abs(-3));
        }
    }
}
