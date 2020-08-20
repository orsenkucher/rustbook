use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};

pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}

// need to impl Clone ourselves, since Arc is clonable not necessarily when T is clonable
impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        Sender {
            // specifically clone the Arc and not the thing inside the Arc:
            shared: Arc::clone(&self.shared),
            // when self.shared.clone() might clone shared Arc value as well.
            // x. is (*x). and it recurses down
        }
    }
}

impl<T> Sender<T> {
    pub fn send(&mut self, t: T) {
        let mut queue = self.shared.queue.lock().unwrap();
        queue.push_back(t);
        drop(queue); // drop lock to make receiver wakeup
        self.shared.available.notify_one();
    }
}

pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> T {
        let mut queue = self.shared.queue.lock().unwrap();
        loop {
            match queue.pop_front() {
                Some(t) => return t,
                None => {
                    // wait until OS gives a reason to wake, though it's not guaranteed
                    // the reason is what we wait for. So we loop here
                    queue = self.shared.available.wait(queue).unwrap();
                }
            }
        }
    }
}

struct Shared<T> {
    queue: Mutex<VecDeque<T>>,
    available: Condvar,
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let shared = Shared {
        queue: Mutex::default(),
        available: Condvar::new(),
    };
    let shared = Arc::new(shared);
    (
        Sender {
            // Arc::clone(&shared) is more idiomatic? see section about clone above
            shared: shared.clone(),
        },
        Receiver {
            shared: shared.clone(),
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ping_pong() {
        let (mut tx, mut rx) = channel();
        tx.send(42);
        assert_eq!(rx.recv(), 42);
    }

    #[test]
    fn closed() {
        let (tx, mut rx) = channel::<()>();
        let _ = tx;
        let _ = rx.recv();
    }
}
