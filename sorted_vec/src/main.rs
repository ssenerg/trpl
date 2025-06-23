use sorted_vec::SortedVec;

fn main() {
    let inp: Vec<f64> = vec![
        4.0, 9.0, 4.5, 8.0, 7.0, 6.0, 5.0, 5.0, 4.5, 5.0, 4.0, 3.0, 2.0, 1.0, 2.0, 3.0, 5.0,
    ];
    let vector = SortedVec::from_vec(inp);
    let m = *vector.median().unwrap();
    println!("median of {vector:?} is => {m}");

    let inp: Vec<i32> = vec![4, 9, 8, 7, 6, 5, 4, 5, 5, 4, 3, 2, 1, 2, 3, 4, 5];
    let vector = SortedVec::from_vec(inp);
    let m = *vector.median().unwrap();
    println!("median of {vector:?} is => {m}");

    let vector: SortedVec<i32> = SortedVec::<i32>::new();
    match vector.median() {
        None => println!("median of {vector:?} is => none"),
        _ => panic!("won't happen"),
    }
}
