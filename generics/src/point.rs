struct Point<T, U> {
    x: T,
    y: U,
}

impl<T> Point<T, T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }

    fn default() -> Point<T, U>
    where
        T: Default,
        U: Default,
    {
        Point {
            x: T::default(),
            y: U::default(),
        }
    }

    // #![feature(associated_type_bounds)] // Hmm
    // fn default2() -> Point<T: Default, U: Default> {
    //     Point {
    //         x: T::default(),
    //         y: U::default(),
    //     }
    // }
}

impl<T: Default, U: Default> Point<T, U> {
    fn default2() -> Point<T, U> {
        Point {
            x: T::default(),
            y: U::default(),
        }
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
