use std::cell::RefCell;
use std::fmt::{Debug, Error, Formatter};
use std::ops::Deref;
use std::rc::{Rc, Weak};

pub enum LinkedList<T> {
    Cons(T, Box<LinkedList<T>>),
    Nil,
}

pub enum RcLinkedList<T> {
    Cons(T, Rc<RcLinkedList<T>>),
    Nil,
}

pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct CustomSmartPointer {
    pub data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

impl Deref for CustomSmartPointer {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

pub fn hello(string: &str) {
    println!("Hello {string} from hello!");
}

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent: You're at 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You're at 75% of your quota!");
        }
    }
}

pub enum RefCellLinkedList {
    Cons(Rc<RefCell<i32>>, Rc<RefCellLinkedList>),
    Nil,
}

impl ToString for RefCellLinkedList {
    fn to_string(&self) -> String {
        match self {
            RefCellLinkedList::Cons(head, tail) => {
                let tail_string = tail.to_string();
                if tail_string.is_empty() {
                    return format!("{}", head.borrow());
                }
                format!("{} -> {}", head.borrow(), tail.to_string())
            }
            RefCellLinkedList::Nil => String::new(),
        }
    }
}

impl Debug for RefCellLinkedList {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            RefCellLinkedList::Nil => write!(f, "[]"),
            _ => write!(f, "[ {} ]", self.to_string()),
        }
    }
}

#[derive(Debug)]
pub enum DamnLinkedList {
    Cons(i32, RefCell<Rc<DamnLinkedList>>),
    Nil,
}

impl DamnLinkedList {
    pub fn tail(&self) -> Option<&RefCell<Rc<DamnLinkedList>>> {
        match self {
            DamnLinkedList::Cons(_, item) => Some(item),
            DamnLinkedList::Nil => None,
        }
    }
}

#[derive(Debug)]
pub struct Node {
    pub value: i32,
    pub parent: RefCell<Weak<Node>>,
    pub children: RefCell<Vec<Rc<Node>>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn box_test() {
        let x = 5;
        let y = Box::new(x);
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    #[test]
    fn mybox_test() {
        let x = 5;
        let y = MyBox::new(x);
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    #[test]
    fn deref_example() {
        let m: MyBox<String> = MyBox::new(String::from("Rust"));
        let _c: &String = &(*m);
        let hard_way: &str = &(*m)[..];
        let rust_is_prefect: &MyBox<String> = &m;
        hello(hard_way);
        hello(rust_is_prefect);
    }

    #[test]
    fn rc_test() {
        let a = Rc::new(RcLinkedList::Cons(
            2,
            Rc::new(RcLinkedList::Cons(3, Rc::new(RcLinkedList::Nil))),
        ));
        let _b = RcLinkedList::Cons(4, Rc::clone(&a));
        let _c = RcLinkedList::Cons(5, Rc::clone(&a));
    }

    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
