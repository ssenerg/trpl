use median::median;

fn main() {
    let vector = vec![4, 9, 8, 7, 6, 5, 5, 5, 4, 3, 2, 1, 2, 3, 5];
    println!("median of {vector:?} is => {}", median(&vector).string());

    let vector = vec![4, 9, 8, 7, 6, 5, 5, 5, 4, 3, 2, 1, 2, 3, 4, 5];
    println!("median of {vector:?} is => {}", median(&vector).string());

    let vector = vec![];
    println!("median of {vector:?} is => {}", median(&vector).string());
}
