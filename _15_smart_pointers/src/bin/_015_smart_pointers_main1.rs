use _15_smart_pointers::CustomSmartPointer;
use std::mem::drop;

fn main() {
    let _c = CustomSmartPointer {
        data: String::from("my stuff 1"),
    };
    println!("CustomSmartPointers  my stuff 1  created.");
    let _c = CustomSmartPointer {
        data: String::from("my stuff 2"),
    };
    println!("CustomSmartPointers  my stuff 2  created.");
    let c = CustomSmartPointer {
        data: String::from("my stuff 3"),
    };
    println!("CustomSmartPointers  my stuff 3  created.");
    drop(c);
    {
        let _d = CustomSmartPointer {
            data: String::from("my stuff 4"),
        };
        println!("CustomSmartPointers  my stuff 4  created.");
        println!("Inner scope ended.");
    }
    println!("Out of inner scope.");
}
