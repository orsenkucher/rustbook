use std::{cmp::PartialOrd, fmt::Display};

mod point;
mod trait_obj;

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest2<T: PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest = list[0].clone();

    for item in list {
        let item = item.clone();
        if item > largest {
            largest = item
        }
    }

    largest
}

fn largest3<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn print_largest(list: &[impl PartialOrd + Copy + Display]) {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    println!("{}", largest)
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

mod tweet {
    use std::fmt::Display;

    pub trait Summary {
        fn summarize(&self) -> String;
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    pub fn notify(item: &(impl Summary + Display)) {
        println!("Breaking news for {}! {}", item, item.summarize());
    }

    // pub fn notify<T: Summary>(item: &T) {
    //     println!("Breaking news! {}", item.summarize());
    // }

    // fn notify(item1: &impl Summary, item2: &impl Summary)

    // fn notify<T: Summary>(item1: &T, item2: &T)
}
