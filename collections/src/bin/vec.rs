fn main() {
    let v: Vec<i32> = Vec::new();
    let v = Vec::<i32>::new();

    let v = vec![1, 2, 3];

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let v = vec![1, 2, 3, 4, 5];

    let does_not_exist = &v[100]; // will panic
    let does_not_exist = v.get(100);

    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    // v.push(6); // cannot borrow `v` as mutable because it is also borrowed as immutable

    println!("The first element is: {}", first);

    let v = vec![234, 123, 41, 233];
    for i in &v {
        // borrow &i32
        println!("{}", i);
    }
    for i in v {
        // move i32
        println!("{}", i);
    }

    let mut v = vec![4, 5, 2, 7];
    for i in &mut v {
        /*dereference*/
        *i *= 10;
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let mut row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // for cell in &mut row {
    //     *cell = match cell {
    //         &mut SpreadsheetCell::Int(i) => SpreadsheetCell::Int(i + i),
    //         &mut SpreadsheetCell::Float(f) => SpreadsheetCell::Float(f + f),
    //         &mut SpreadsheetCell::Text(s) => SpreadsheetCell::Text(s),
    //     }
    // }

    for cell in row {
        match cell {
            SpreadsheetCell::Int(i) => SpreadsheetCell::Int(i + i),
            SpreadsheetCell::Float(f) => SpreadsheetCell::Float(f + f),
            SpreadsheetCell::Text(s) => SpreadsheetCell::Text(s /*&s ??*/),
        };
    }
}
