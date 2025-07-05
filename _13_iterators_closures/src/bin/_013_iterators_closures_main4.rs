use _13_iterators_closures::shapes::{Rectangle, Shape, Shapes};

fn main() {
    let mut num_sort_operations = 0;
    let mut list: Vec<Rectangle<i32>> = vec![
        Rectangle::new(10, 12),
        Rectangle::new(1 << 21, 1),
        Rectangle::new(11, 12),
        Rectangle::new(1 << 22, 1),
        Rectangle::new(12, 12),
        Rectangle::new(1 << 23, 1),
        Rectangle::new(13, 12),
        Rectangle::new(1 << 24, 1),
        Rectangle::new(14, 12),
        Rectangle::new(1 << 25, 1),
        Rectangle::new(15, 12),
        Rectangle::new(1 << 26, 1),
        Rectangle::new(16, 12),
        Rectangle::new(1 << 27, 1),
        Rectangle::new(17, 12),
        Rectangle::new(1 << 28, 1),
        Rectangle::new(18, 12),
        Rectangle::new(1 << 29, 1),
        Rectangle::new(19, 12),
        Rectangle::new(i32::MAX, 1),
    ];
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width()
    });
    println!("Sorted by width = {}", list.pretty(None));
    println!(
        "Sorting of {} elements took {} operations\n",
        list.len(),
        num_sort_operations / 2 + 1
    );

    let mut list: Vec<Rectangle<i32>> = vec![Rectangle::new(10, 12), Rectangle::new(1 << 30, 1)];
    list.sort();
    println!("Sorted by area = {}\n", list.pretty(None));

    let rec1 = Rectangle::new(10, 12);
    let rec2 = Rectangle::new(1 << 30, 1);
    println!("Area of {:?} = {}", rec1, rec1.area());
    println!("Area of {:?} = {}\n", rec2, rec2.area());

    let recs = vec![
        Rectangle::new(18, 12),
        Rectangle::new(18, 13),
        Rectangle::new(1 << 29, 1),
    ];
    println!("Total area of {} = {}", recs.pretty(None), recs.area());

    let recs: Vec<Rectangle<i32>> = vec![];
    println!("Total area of {} = {}", recs.pretty(None), recs.area());
}
