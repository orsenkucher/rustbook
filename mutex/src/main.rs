use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        // get mutable reference (smart pointer) to num
        let mut guard = m.lock().unwrap();
        *guard = 6;
    }

    println!("m = {:?}", m);

    with_threads();
}

use std::sync::Arc;
use std::thread;

fn with_threads() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    (0..10)
        .map(|_| Arc::clone(&counter))
        .for_each(|c| handles.push(thread::spawn(move || *c.lock().unwrap() += 1)));
    // for _ in 0..10 {
    //     let counter = Arc::clone(&counter);
    //     let handle = thread::spawn(move || *counter.lock().unwrap() += 1);
    //     handles.push(handle);
    // }

    handles.into_iter().for_each(|h| h.join().unwrap());
    // for handle in handles {
    //     handle.join().unwrap();
    // }

    println!("Result: {}", *counter.lock().unwrap());
}
