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
            .compare_exchange_weak(UNLOCKED, LOCKED, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            // MESI protocol: stay in S when locked
            while self.locked.load(Ordering::Relaxed) == LOCKED {}
        }
        // Safety: we hold the lock, therefor we can create a mutable reference.
        let ret = f(unsafe { &mut *self.v.get() });
        self.locked.store(UNLOCKED, Ordering::Release);
        ret
    }
}

use std::thread::spawn;

// Checkout `loom`
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

#[test]
fn seq_cst() {
    use std::sync::atomic::AtomicUsize;
    let x: &'static _ = Box::leak(Box::new(AtomicBool::new(false)));
    let y: &'static _ = Box::leak(Box::new(AtomicBool::new(false)));
    let z: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));

    spawn(move || x.store(true, Ordering::Release));
    spawn(move || y.store(true, Ordering::Release));

    let t1 = spawn(move || {
        while !x.load(Ordering::Acquire) {}
        if y.load(Ordering::Acquire) {
            z.fetch_add(1, Ordering::Relaxed);
        }
    });

    let t2 = spawn(move || {
        while !y.load(Ordering::Acquire) {}
        if x.load(Ordering::Acquire) {
            z.fetch_add(1, Ordering::Relaxed);
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();

    let _z = z.load(Ordering::SeqCst);
    // What are the possible values for z?
    // 2 - Yes
    // 1 - Yes
    // But 0 - should not be an option, but actually is!
    // So use SeqCst instead of Acquire and Release in a `while !` and `if`.
    // Use loom to spin the threads of all possible states.
    //
    // The stream: https://youtu.be/rMGWeSjctlY
}
