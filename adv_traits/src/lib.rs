#[cfg(test)]
mod tests {
    #[test]
    fn associated_types() {
        trait Iter {
            type Item;

            fn next(&mut self) -> Option<Self::Item>;
        }

        impl Iter for Vec<u32> {
            type Item = u32;

            fn next(&mut self) -> Option<u32> {
                None
            }
        }
    }

    #[test]
    fn default_generic_type_param() {
        trait Dot<Rhs = Self> {
            type Output;

            fn dot(self, rhs: Rhs) -> Self::Output;
        }

        impl Dot for Vec<i32> {
            type Output = i32;

            fn dot(self, _rhs: Vec<i32>) -> i32 {
                0
            }
        }

        impl Dot<i32> for Vec<i32> {
            type Output = Vec<i32>;

            fn dot(self, _rhs: i32) -> Vec<i32> {
                vec![]
            }
        }
    }

    #[test]
    fn operator_overloading() {
        use std::ops::Add;

        struct Millimeters(u32);
        struct Meters(u32);

        impl Add<Meters> for Millimeters {
            type Output = Millimeters;

            fn add(self, other: Meters) -> Millimeters {
                Millimeters(self.0 + (other.0 * 1000))
            }
        }
    }

    #[test]
    fn fully_qualified_syntax_method() {
        trait Pilot {
            fn fly(&self);
        }

        trait Wizard {
            fn fly(&self);
        }

        struct Human;

        impl Pilot for Human {
            fn fly(&self) {
                println!("This is your captain speaking.");
            }
        }

        impl Wizard for Human {
            fn fly(&self) {
                println!("Up!");
            }
        }

        impl Human {
            fn fly(&self) {
                println!("*waving arms furiously*");
            }
        }

        let person = Human;
        Pilot::fly(&person);
        Wizard::fly(&person);
        person.fly();
    }

    #[test]
    // <Type as Trait>::function(receiver_if_method, next_arg, ...);
    fn fully_qualified_syntax_associated_function() {
        trait Animal {
            fn baby_name() -> String;
        }

        struct Dog;

        impl Dog {
            fn baby_name() -> String {
                String::from("Spot")
            }
        }

        impl Animal for Dog {
            fn baby_name() -> String {
                String::from("puppy")
            }
        }

        println!("A baby dog is called a {}", Dog::baby_name());
        // println!("A baby dog is called a {}", Animal::baby_name()); // error

        // <Type as Trait>::function(receiver_if_method, next_arg, ...); // fully qualified syntax
        println!("A baby dog is called a {}", <Dog as Animal>::baby_name())
    }

    #[test]
    fn supertraits() {
        use std::fmt;

        trait OutlinePrint: fmt::Display {
            fn outline_print(&self) {
                let output = self.to_string();
                let len = output.len();
                println!("{}", "*".repeat(len + 4));
                println!("*{}*", " ".repeat(len + 2));
                println!("* {} *", output);
                println!("*{}*", " ".repeat(len + 2));
                println!("{}", "*".repeat(len + 4));
            }
        }

        struct Point {
            x: i32,
            y: i32,
        }

        // Point has to impl Display to impl OutlinePrint
        impl OutlinePrint for Point {}

        impl fmt::Display for Point {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "({}, {})", self.x, self.y)
            }
        }

        let p = Point { x: 12, y: 13 };
        p.outline_print();
    }

    #[test]
    fn newtype() {
        use std::fmt;
        use std::ops::{Deref, DerefMut};

        struct Wrapper(Vec<String>);

        impl fmt::Display for Wrapper {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "[{}]", self.0.join(", "))
            }
        }

        impl Deref for Wrapper {
            type Target = Vec<String>;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl DerefMut for Wrapper {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        let mut w = Wrapper(
            vec!["hello", "world"]
                .iter()
                .map(ToString::to_string)
                .collect(),
        );

        println!("w = {}", w);
        w.push(String::from("another"));
        println!("w = {}", w);
    }
}
