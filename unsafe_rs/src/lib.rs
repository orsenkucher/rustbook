// 1. Raw pointer deref
// 2. Use of unsafe fn
// 3. Mutation of static mut variable
// 4. Unsafe trait impl (like Sync, Send)
// 5. Use of Unions (structs with one value at a time)

// cargo test -- --nocapture
#[cfg(test)]
mod tests {
    #[test]
    fn raw_pointers() {
        let mut num = 5;

        let r1 = &num as *const i32; // immutable raw pointer
        let r2 = &mut num as *mut i32; // mutable raw pointer

        unsafe {
            println!("r1 is: {}", *r1);
            println!("r2 is: {}", *r2);
        }
    }

    unsafe fn dangerous() {}

    #[test]
    fn call_dangerous() {
        unsafe {
            dangerous();
        }
    }

    use std::slice;

    fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = slice.len();
        let ptr = slice.as_mut_ptr();

        assert!(mid <= len);

        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }

    #[test]
    fn split_at_mut_test() {
        let mut v = vec![1, 2, 3, 4, 5, 6];

        let r = &mut v[..];

        let (a, b) = split_at_mut(r, 3);

        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);
    }

    extern "C" {
        fn abs(input: i32) -> i32;
    }

    #[test]
    fn call_c() {
        unsafe {
            println!("Absolute value of -3 according to C: {}", abs(-3));
        }
    }

    #[no_mangle]
    pub extern "C" fn call_from_c() {
        println!("Just called a Rust function from C!");
    }

    #[test]
    fn called_from_c() {
        call_from_c() // and from rust also!
    }

    static HELLO_WORLD: &str = "Hello, world!";

    #[test]
    fn static_filed() {
        println!("name is: {}", HELLO_WORLD);
    }

    static mut COUNTER: u32 = 0;

    unsafe fn add_to_count_unsafe(inc: u32) {
        COUNTER += inc;
    }

    fn add_to_count(inc: u32) {
        unsafe { add_to_count_unsafe(inc) }
    }

    #[test]
    fn static_field_mut() {
        println!("counter is: {}", unsafe { COUNTER });

        unsafe { add_to_count_unsafe(1) }
        add_to_count(3);

        println!("counter is: {}", unsafe { COUNTER });
    }

    #[test]
    fn unsafe_trait() {
        unsafe trait Foo {}

        unsafe impl Foo for i32 {}
    }

    #[repr(C)]
    union MyUnion {
        f1: u32,
        f2: f32,
    }

    #[test]
    fn union() {
        let u = MyUnion { f2: 1.5 };
        // let f1 = unsafe { u.f1 }; // error
        let f2 = unsafe { u.f2 };

        assert_eq!(f2, 1.5f32);
    }

    fn _f(u: MyUnion) {
        unsafe {
            match u {
                MyUnion { f1: 10 } => {
                    println!("ten");
                }
                MyUnion { f2 } => {
                    println!("{}", f2);
                }
            }
        }
    }
}
