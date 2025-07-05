// Note that Summary is used once; its because we have to import traits too
use _10_generics_traits_lifetimes::{
    longest, notify2, FirstWord, ImportantExcerpt, NewsArticle, Pair, Summary, Tweet,
};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("new tweet: {}\n", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
            hockey team in the NHL.",
        ),
    };
    println!("new article: {}\n", article.summarize());
    notify2(&tweet, &article);
    println!();

    Pair::new(1, 2).cmp_display();

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}\n");

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let _i = ImportantExcerpt {
        part: first_sentence,
    };

    println!(
        "first word of Kaftar is {}.\n",
        FirstWord::from_string("Kaftar").first()
    );

    let integer = Point::new(1, 2);
    let float = Point::new(3.0, 4.0);
    println!("integer x     is => {}", integer.x());
    println!("integer y     is => {}", integer.y());
    println!("float   x     is => {}", float.x());
    println!("float   y     is => {}", float.y());
    println!("float   norm2 is => {}", float.distance_from_origin());
    // we can not use below line
    // println!("integer norm2 is => {}", float.distance_from_origin());
}

pub struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> &T {
        &self.x
    }

    pub fn y(&self) -> &T {
        &self.y
    }

    pub fn mixup<U>(self, other: Point<U>) -> Point<U> {
        other
    }
}

impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
