use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicBool, Ordering};

const LOCKED: bool = true;
const UNLOCKED: bool = false;

pub struct Mutex<T> {
    lock: AtomicBool,
    v: UnsafeCell<T>,
}

unsafe impl<T> Sync for Mutex<T> where T: Send {}

impl<T> Mutex<T> {
    pub fn new(t: T) -> Self {
        Self {
            v: UnsafeCell::new(t),
            lock: AtomicBool::new(UNLOCKED),
        }
    }

    pub fn with_lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        while self.lock.load(Ordering::Relaxed) != UNLOCKED {}
        // maybe another thread runs here
        std::thread::yield_now();
        self.lock.store(LOCKED, Ordering::Relaxed);
        // Safety: we hold the lock, therefor we can create a mutable reference.
        let ret = f(unsafe { &mut *self.v.get() });
        self.lock.store(UNLOCKED, Ordering::Relaxed);
        ret
    }
}

use std::thread::spawn;

fn main() {
    let mutex: &'static _ = Box::leak(Box::new(Mutex::new(0)));
    let handles: Vec<_> = (0..100)
        .map(|_| {
            spawn(move || {
                for _ in 0..1000 {
                    mutex.with_lock(|v| {
                        *v += 1;
                    })
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap()
    }

    assert_eq!(mutex.with_lock(|v| *v), 100 * 1000);
}
