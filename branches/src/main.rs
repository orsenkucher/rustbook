fn main() {
    let number = 5;
    let a = if number < 5 {
        println!("Number is less");
        ()
    } else if number > 5 {
        println!("Number is larger")
    } else {
        println!("Number is 5")
    };
    println!("() is {:?}", a);

    let x = if true { 1 } else { 0 };
    println!("Number is {}", x);
}
