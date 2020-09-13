#[cfg(test)]
mod tests {
    #[test]
    fn newtype() {
        #[derive(Debug)]
        struct Kilometers(i32);

        fn use_km(km: Kilometers) {
            println!("{:?}", km)
        }

        #[derive(Debug)]
        struct Millimeters(i32);

        fn use_mm(mm: Millimeters) {
            println!("{:?}", mm)
        }

        let km = Kilometers(10);
        use_km(km);

        let mm = Millimeters(10);
        // use_km(mm); // won't compile
        use_mm(mm);
    }

    #[test]
    fn type_aliases() {
        type Kilometers = i32; // Synonymous to i32

        let x: i32 = 5;
        let y: Kilometers = 5;

        println!("x + y = {}", x + y);
    }

    #[test]
    fn type_aliases_thunk() {
        type Thunk = Box<dyn Fn() + Send + 'static>;

        let f: Thunk = Box::new(|| println!("hi"));

        takes_long_type(f);

        fn takes_long_type(f: Thunk) {
            f()
        }

        fn returns_long_type() -> Thunk {
            Box::new(|| ())
        }

        takes_long_type(returns_long_type())
    }

    #[test]
    fn never_type() {
        fn never() -> ! {
            loop {}
        }

        let guess = "123";
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => never(), // on err will stuck forever
        };

        println!("guess: {}", guess);
    }

    #[test]
    fn generics() {
        fn generic<T: Sized>(_t: T) {
            // --snip--
        }

        // Dynamically Sized Types
        // DST are used via reference
        fn relaxed_generic<T: ?Sized>(_t: &T) {
            // --snip--
        }

        generic(());
        relaxed_generic(&());
    }
}
