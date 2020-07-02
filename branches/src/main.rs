fn main() {
    if_statement();
    loops();
}

fn if_statement() {
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

    //     let err = if false {5} else {"hello"}
}

fn loops() {
    loop_1();
    loop_2();

    while_1();
    while_2();

    for_1();
}

fn for_1() {}

fn while_2() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
fn while_1() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!!!");
}

fn loop_2() {
    let mut counter = 0;
    // so like from next line til next+5 line is a let statement. End it with ;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Number is {}", result);
}

fn loop_1() {
    let mut count = 0;
    loop {
        count += 1;
        println!("again, and again");
        if count == 3 {
            break;
        }
    }
}
