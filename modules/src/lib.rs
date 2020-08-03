mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        // associated function
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

mod front_of_house;

// just for show
use std::io::{self, Write};

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Weat");
    println!("I'd like {} toast please", meal.toast);
    // Won't compile:
    // meal.seasonal_fruit = String::from("blueberries");
    // println!("I'd like {} fruit please", meal.seasonal_fruit);

    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

trait Appendable {
    fn append(&mut self, other: &mut Self);
}

#[test]
fn trait_test() {
    let mut v = vec![1, 3, 4];
    let mut v2 = vec![2];
    v.append(&mut v2);
    let res = format!("v: {:#?}, v2:{:#?}", v, v2);
    print!("{}", res);

    print!("{:#?}", v);
    ap(&mut v);
}

impl<T> Appendable for Vec<T> {
    fn append(&mut self, other: &mut Self) {
        self.append(other)
    }
}

fn ap(v: &mut impl Appendable) {}

fn ap2<T: Appendable>(v: &mut T) {}

fn ap3<T>(v: &mut T)
where
    T: Appendable,
{
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
