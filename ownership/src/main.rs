fn main() {
    let str_literal = "hello";
    let mut s = String::from(str_literal);

    s.push_str(", world");

    println!("{}", s);

    let x = String::from(str_literal);
    let y = x.clone();

    println!("{}", x);
    println!("{}", y);
}
