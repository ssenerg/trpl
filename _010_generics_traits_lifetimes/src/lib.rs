use std::fmt::{Debug, Display};

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// *********
// IMPORTANT
// *********
//
impl<T: Summary> Tut for T {
    fn tut_type() {}
}

pub trait Tut {
    fn tut_type();
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify2_of_same<T: Summary>(item1: &T, item2: &T) {
    notify(item1);
    notify(item2);
}

fn _notify2<T: Summary, U: Summary>(item1: &T, item2: &U) {
    notify(item1);
    notify(item2);
}

pub fn notify2(item1: &impl Summary, item2: &impl Summary) {
    notify(item1);
    notify(item2);
}

// *********
// IMPORTANT
// *********
//
fn _some_fucking_important_function<T, U>(_t: &T, _u: &U) -> i32
where
    T: Display + Clone,
    U: Debug + Clone,
{
    1
}

pub struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}\n", self.x);
        } else {
            println!("The largest member is y = {}\n", self.y);
        }
    }
}

pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub struct ImportantExcerpt<'a> {
    pub part: &'a str,
}
