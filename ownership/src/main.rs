fn main() {
    let str_literal = "hello";
    let mut s = String::from(str_literal);

    s.push_str(", world");

    println!("{}", s);

    let x = String::from(str_literal);
    let y = x.clone();
    println!("{} {}", x, y);

    let a = 5;
    let b = a.clone();
    println!("{} {}", a, b);

    let c = a;
    println!("{} {}", a, c);

    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);

    let r3 = &mut s;
    println!("{}", r3);
    // println!("{}", r1); // will cause an error

    let phrase = String::from("hello world");
    let hello = first_word(&phrase);
    println!("first word: {}", hello);

    let hello = better_first_word(&phrase[..]);
    println!("first word: {}", hello);
    let literal = "hello world";

    let hello = better_first_word(literal);
    println!("first word: {}", hello);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// here fn receives `&str` string slice
// thus string literals can be used
fn better_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
