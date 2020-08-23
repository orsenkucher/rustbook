use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        // get mutable reference (smart pointer) to num
        let mut guard = m.lock().unwrap();
        *guard = 6;
    }

    println!("m = {:?}", m);
}
