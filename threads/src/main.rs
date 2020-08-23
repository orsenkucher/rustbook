use std::{thread, time::Duration};

fn main() {
    let v = (0..10).collect::<Vec<_>>();
    let handle = thread::spawn(move || {
        println!("{:?}", v);
        // with for i in v, rust will infer that v is moved,
        // making `move` unnecessary
        for i in v.iter() {
            println!("spawned: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // 5 inclusive
    for i in 0..=5 {
        println!("main: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
