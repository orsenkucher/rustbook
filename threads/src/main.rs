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
    streaming();
}

use std::sync::mpsc;

// but here `move` is required
fn oneshot() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || tx.send(String::from("yo")).unwrap());
    println!("Got: {}", rx.recv().unwrap());
}

fn streaming() {
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec!["hi", "from", "the", "thread"]
            .into_iter()
            .map(String::from);

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(500));
        }

        // tx1 is closed when dropped
    });

    thread::spawn(move || tx.send(String::from("end?")).unwrap());
    // tx closed on when second thread finishes

    for received in rx {
        println!("Got: {}", received);
    }

    // tx1 and tx were closed by this point
}
