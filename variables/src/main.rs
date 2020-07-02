fn main() {
    const NUM: i32 = 5;
    let mut x = NUM;
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);

    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("Y is : {}", y);

    let spaces = "    ";
    let spaces = spaces.len();
    println!("spaces: {}", spaces);

    let myu8: u8 = 255;
    let myu16: u16 = 3;
    let myu32: u32 = 3;
    let myu64: u64 = 3;
    let myu128: u128 = 3;
    let myusize: usize = 100_200_300_usize;
    //     let myusize: usize = 100_200_300_u128;
    println!(
        "u8: {} {} {} {} {} {}",
        myu8, myu16, myu32, myu64, myu128, myusize,
    );

    // tuples
    let tup = (500, 6.4, 1);
    let tup: (i32, f64, u8) = tup;
    println!("{:#?}", tup);

    let (_, b, _): (i32, f64, u8) = tup;
    println!("{}", b);

    let c: u8 = tup.2;
    println!("{}", c);

    // array is fixed size
    // vec is in ::std and is allowed to grow and shrink
    let a = [1, 2, 3, 4, 5];
    println!("{:?}", a);

    let months: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    println!("{:?}", months);

    let same_values: [i32; 5] = [3; 5];
    println!("{:?}", same_values);

    println!("12-th month: {}", months[11]);
    //     panica
    //     println!("12-th month: {}", months[12]);
}

#[test]
fn hello_tests() {
    let three = 1 + 2;
    println!();
    println!("{:?}", three);
    return ();
}
