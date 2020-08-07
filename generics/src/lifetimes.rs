use std::fmt::Display;

pub fn longest<'a, T>(x: &'a Vec<T>, y: &'a Vec<T>) -> &'a Vec<T> {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // first rule
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    // third rule
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("Part is: {}", i.part);

    let s: &'static str = "I have a static lifetime.";
}

// No way to make it return ref
// In this case, the best fix would be to return an owned data type rather than a reference
//
// fn longest_wrong<'a>(x: &'a str, y: &'a str) -> &'a str {
//     let result = String::from("really long string");
//     let res: &'a str = result.as_str();
//     res
// }
//

// Another example: if you try to use `result` as `part` field, it won't compile.
fn longest_struct<'a>(x: &'a str, y: &'a str) -> ImportantExcerpt<'a> {
    let result = String::from("really long string");
    ImportantExcerpt {
        part: if x.len() > y.len() { x } else { y },
        // part: result.as_str(),
    }
}
