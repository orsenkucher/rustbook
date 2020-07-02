use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guessing game");
    let secret = rand::thread_rng().gen_range(1, 101);
    // let secret = rand::thread_rnd().
    println!("Secret is {}", secret);

    loop {
        println!("Enter your number: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("error while readline");

        guess = guess.trim().to_string();

        if guess == "q" {
            println!("quitting");
            break;
        }

        let guess = match guess.parse::<u32>() {
            Ok(val) => val,
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };

        match guess.cmp(&secret) {
            Ordering::Greater => println!("too big"),
            Ordering::Less => println!("too low"),
            Ordering::Equal => {
                println!("you won");
                break;
            }
        };
    }
}
