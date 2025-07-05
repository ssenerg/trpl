use std::slice::Iter;

fn main() {
    let v1 = vec![1, 2];
    let v1_iter: Iter<'_, i32> = v1.iter();
    for val in &v1 {
        println!("Got: {val}");
    }
    for val in v1_iter {
        println!("Got: {val}");
    }
}

#[test]
fn iterator_demonstration() {
    let mut v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter_mut();
    assert_eq!(v1_iter.next(), Some(&mut 1));
    let a = v1_iter.next().unwrap();
    *a = 7;
    assert_eq!(Some(a), Some(&mut 7));
    assert_eq!(v1_iter.next(), Some(&mut 3));
    assert_eq!(v1_iter.next(), None);
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}

#[test]
fn map() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
    assert_eq!(v1, vec![1, 2, 3]);
}
