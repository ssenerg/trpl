use _19_advanced_features::advanced_traits::*;

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
    assert_eq!(Millimeters(2) + Meters(1), Millimeters(1002));

    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly(); // == Human::fly(&person);

    println!("A baby dog is called a {}", Dog::baby_name());
    // we can not use Animal::baby_name() because it does not support self
    // println!("A baby dog is called a {}", Animal::baby_name());
    //
    // instead we can use Fully qualified syntax
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    Point2 { x: 1, y: 3 }.outline_print();

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {w}");
}
