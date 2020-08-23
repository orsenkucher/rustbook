use std::{thread, time::Duration};

fn main() {
    let v = (0..10).collect::<Vec<_>>();
    let handle = thread::spawn(|| {
        println!("{:?}", v);
        // rust will infer that v is moved,
        // making `move` unnecessary
        for i in v {
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

    oneshot();
}

use std::sync::mpsc;

// but here `move` is required
fn oneshot() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || tx.send(String::from("yo")).unwrap());
    println!("Got: {}", rx.recv().unwrap());
}
