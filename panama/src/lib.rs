use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};

// Flavors:
//  - Synchronous channels: Channel where send() can block. Limited capacity.
//   - Mutex + Condvar + VecDeque(head + tail pointers)
//   - Atomic VecDeque (atomic queue) + thread::park + thread::Thread::notify
//  - Asynchronous channels: Channel where send() cannot block. Unbounded.
//   - Mutex + Condvar + VecDeque
//   - Mutex + Condvar + LinkedList (to never resize)
//   - Atomic linked list, linked list of T
//   - Atomic block linked list, linked list of VecDeque<T>
//  - Rendezvous channels: Synchronous channel with capacity = 0. Used for thread synchronization. Also see std::sync::Barrier
//   - Mutex + Condvar
//  - Oneshot channels: Any capacity. In practice, only one call to send(). i.e.: shutdown all.
//   - Atomic None|Some

pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}

// need to impl Clone ourselves, since Arc is clonable not necessarily when T is clonable
impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders += 1;
        drop(inner);

        Sender {
            // specifically clone the Arc and not the thing inside the Arc:
            shared: Arc::clone(&self.shared),
            // when self.shared.clone() might clone shared Arc value as well.
            // x. is (*x). and it recurses down
        }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let mut inner = self.shared.inner.lock().unwrap();
        eprintln!("drop sender, count was: {}", inner.senders);
        inner.senders -= 1;
        let was_last = inner.senders == 0;
        drop(inner);
        if was_last {
            self.shared.available.notify_one(); // unlock our only one receiver is no more senders exist
        }
    }
}

impl<T> Sender<T> {
    pub fn send(&mut self, t: T) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.queue.push_back(t);
        drop(inner); // drop lock to make receiver wakeup
        self.shared.available.notify_one();
    }
}

pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
    buffer: VecDeque<T>,
}

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> Option<T> {
        if let Some(t) = self.buffer.pop_front() {
            return Some(t);
        }
        // by now buffer should be empty

        let mut inner = self.shared.inner.lock().unwrap();
        loop {
            match inner.queue.pop_front() {
                Some(t) => {
                    // Wow haha! Steal all items at once from queue
                    // when acquired this lock anyway
                    // if !inner.queue.is_empty() { // without this branch might be faster?
                    std::mem::swap(&mut self.buffer, &mut inner.queue);

                    return Some(t);
                }
                None if dbg!(inner.senders) == 0 => return None, // dbg!() is debug print macro
                None => {
                    // wait until OS gives a reason to wake, though it's not guaranteed
                    // the reason is what we wait for. So we loop here
                    inner = self.shared.available.wait(inner).unwrap();
                }
            }
        }
    }
}

impl<T> Iterator for Receiver<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.recv()
    }
}

struct Inner<T> {
    queue: VecDeque<T>,
    senders: usize,
}

struct Shared<T> {
    inner: Mutex<Inner<T>>,
    available: Condvar,
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Inner {
        queue: VecDeque::default(),
        senders: 1,
    };
    let shared = Shared {
        inner: Mutex::new(inner),
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
            buffer: VecDeque::default(),
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
        assert_eq!(rx.recv(), Some(42));
    }

    // cargo t -- --test-threads=1 --nocapture // to show prints
    #[test]
    fn closed_tx() {
        let (tx, mut rx) = channel::<()>();
        // let _ = tx; // won't drop tx immediately haha
        drop(tx);
        assert_eq!(rx.recv(), None);
    }

    #[test]
    fn closed_rx() {
        let (mut tx, rx) = channel();
        drop(rx);
        tx.send(42); // go will panic with sending on closed channel here!
    }
}
