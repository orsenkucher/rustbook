#[cfg(test)]
mod tests {
    // fn type impls Fn FnMut FnOnce
    // so fn can be used where closures are used
    // but not vice versa
    //
    // * FnOnce consumes the variables it captures from its enclosing scope, known as the closure’s environment. To consume the captured variables, the closure must take ownership of these variables and move them into the closure when it is defined. The Once part of the name represents the fact that the closure can’t take ownership of the same variables more than once, so it can be called only once.
    // * FnMut can change the environment because it mutably borrows values.
    // * Fn borrows values from the environment immutably.
    //
    // you can always pass a function pointer as an argument for a function that expects a closure
    #[test]
    fn function_pointers() {
        fn add_one(x: i32) -> i32 {
            x + 1
        }

        fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
            f(arg) + f(arg)
        }

        let answer = do_twice(add_one, 5);
        println!("The answer is: {}", answer);
    }

    #[test]
    fn function_use() {
        let list_of_numbers = vec![1, 2, 3];
        let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
        println!("{:?}", list_of_strings);

        let list_of_numbers = vec![1, 2, 3];
        let list_of_strings: Vec<String> =
            list_of_numbers.iter().map(ToString::to_string).collect();
        println!("{:?}", list_of_strings);
    }

    // Pattern that exploits an implementation detail of tuple structs and tuple-struct enum variants.
    // These types use () as initializer syntax, which looks like a function call.
    #[test]
    fn initializer_functions() {
        #[derive(Debug)]
        enum Status {
            Value(u32),
            Stop,
        }

        let mut list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
        list_of_statuses.push(Status::Stop);

        println!("{:?}", list_of_statuses);
    }

    // box like trait-objects
    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }

    #[test]
    fn returns_closure_test() {
        println!("{}", returns_closure()(1));
    }
}
