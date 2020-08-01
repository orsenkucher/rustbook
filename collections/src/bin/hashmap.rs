use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(String::from("blue"), 12);
    map.insert(String::from("red"), 15);
    println!("{:?}", map);

    let keys = vec![String::from("blue"), String::from("red")];
    let vals = vec![12, 15];
    let scores: HashMap<_, _> = keys.iter().zip(vals.iter()).collect();
    println!("{:?}", scores);

    let team_name = String::from("blue");
    let score = scores.get(&team_name);
    println!("{:?}", score);

    for (key, val) in &scores {
        println!("{}: {}", key, val);
    }

    // Overwriting a Value
    let mut scores = HashMap::new();
    scores.insert("blue", 12);
    scores.insert("blue", 14);
    println!("{:?}", scores);

    // Only Inserting a Value If the Key Has No Value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    // Updating a Value Based on the Old Value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count: &mut i32 = map.entry(word).or_insert(0);
        *count += 1;
    }

    // TODO: Summary Exercises
}
