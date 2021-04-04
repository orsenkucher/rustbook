use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicBool, Ordering};

const LOCKED: bool = true;
const UNLOCKED: bool = false;

pub struct Mutex<T> {
    locked: AtomicBool,
    v: UnsafeCell<T>,
}

unsafe impl<T> Sync for Mutex<T> where T: Send {}

impl<T> Mutex<T> {
    pub fn new(t: T) -> Self {
        Self {
            v: UnsafeCell::new(t),
            locked: AtomicBool::new(UNLOCKED),
        }
    }

    pub fn with_lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        while self
            .locked
            // because we are already in a loop
            // we weak can fail for whatever reason
            .compare_exchange_weak(UNLOCKED, LOCKED, Ordering::Relaxed, Ordering::Relaxed)
            .is_err()
        {
            // MESI protocol: stay in S when locked
            while self.locked.load(Ordering::Relaxed) == LOCKED {}
        }
        // Safety: we hold the lock, therefor we can create a mutable reference.
        let ret = f(unsafe { &mut *self.v.get() });
        self.locked.store(UNLOCKED, Ordering::Relaxed);
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

#[test]
fn too_relaxed() {
    use std::sync::atomic::AtomicUsize;
    let x: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));
    let y: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));
    let t1 = spawn(move || {
        let r1 = y.load(Ordering::Relaxed);
        y.store(r1, Ordering::Relaxed);
        r1
    });
    let t2 = spawn(move || {
        let r2 = x.load(Ordering::Relaxed);
        y.store(22, Ordering::Relaxed); // like time travel haha
        r2
    });
    let _r1 = t1.join();
    let _r2 = t2.join();
    // r1 == r2 == 22
    // wow
}
