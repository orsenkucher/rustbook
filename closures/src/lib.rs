use std::collections::HashMap;
use std::thread;
use std::time::Duration;

pub fn generate_workout(intensity: u32, random_nuber: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Today, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_nuber == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    values: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Self {
        Self {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        let calc = &self.calculation;
        let k = self.values.entry(arg).or_insert_with(|| calc(arg));
        *k // TODO design it better?
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.value(1);
        assert_eq!(v1, 1);

        let v2 = c.value(2);
        assert_eq!(v2, 2);
    }

    #[test]
    fn use_of_move_keyword() {
        let x = vec![1, 2, 3];

        // move is very useful when closure is executed on different thread
        let equal_to_x = move |z| z == x;

        // println!("can't use x here: {}", x);

        let y = vec![1, 2, 3];
        assert!(equal_to_x(y));
    }
}
