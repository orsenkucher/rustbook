use crossbeam::channel;
use futures::lock::Mutex;
use futures::task::{self, ArcWake};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::thread;
use std::time::{Duration, Instant};

struct Delay {
    when: Instant,
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.when {
            println!("Delay done");
            Poll::Ready("done")
        } else {
            // Get a handle to the waker for the current task
            let waker = cx.waker().clone();
            let when = self.when;

            // Spawn a timer thread.
            thread::spawn(move || {
                let now = Instant::now();

                if now < when {
                    thread::sleep(when - now);
                }

                waker.wake();
            });
            Poll::Pending
        }
    }
}

fn main() {
    let mut mini_tokio = MiniTokio::new();

    mini_tokio.spawn(async {
        let when = Instant::now() + Duration::from_millis(10);
        let future = Delay { when };

        let out = future.await;
        assert_eq!(out, "done");
    });

    mini_tokio.run();
}

struct MiniTokio {
    // To receive tasks to schedule
    scheduled: channel::Receiver<Arc<Task>>,
    // To send tasks. For `spawn` fn
    sender: channel::Sender<Arc<Task>>,
}

// type Task = Pin<Box<dyn Future<Output = ()> + Send>>;
struct Task {
    // The `Mutex` is to make `Task` implement `Sync`. Only
    // one thread accesses `future` at any given time. The
    // `Mutex` is not required for correctness. Real Tokio
    // does not use a mutex here, but real Tokio has
    // more lines of code than can fit in a single tutorial
    // page.
    future: Mutex<Pin<Box<dyn Future<Output = ()> + Send>>>,
    // Channel to send tasks for execution.
    executor: channel::Sender<Arc<Task>>,
}

impl Task {
    // Send task to executor
    fn schedule(self: &Arc<Self>) {
        // Arc is cloned
        let _ = self.executor.send(self.clone());
    }
}

impl ArcWake for Task {
    // Waking is done by scheduling the task on executor.
    fn wake_by_ref(arc_self: &Arc<Self>) {
        arc_self.schedule()
    }
}

impl MiniTokio {
    /// Initialize a new mini-tokio instance.
    fn new() -> MiniTokio {
        let (sender, scheduled) = channel::unbounded();
        MiniTokio { scheduled, sender }
    }

    /// Spawn a future onto the mini-tokio instance.
    ///
    /// The given future is wrapped with the `Task` harness and pushed into the
    /// `scheduled` queue. The future will be executed when `run` is called.
    fn spawn<F>(&mut self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        // Passing future and sender half to Task's `spawn`
        Task::spawn(future, &self.sender);
    }

    fn run(&mut self) {
        // Will loop until the receiver is closed
        // and nothing left to process.
        while let Ok(task) = self.scheduled.recv() {
            task.poll();
        }
        // let waker = task::noop_waker();
        // let mut cx = Context::from_waker(&waker);

        // while let Some(mut task) = self.tasks.pop_front() {
        //     if task.as_mut().poll(&mut cx).is_pending() {
        //         self.tasks.push_back(task);
        //     }
        // }
    }
}

impl Task {
    // Poll task for progress
    fn poll(self: Arc<Self>) {
        // Create a waker from the `Task` instance. This
        // uses the `ArcWake` impl from above.
        // waker: W where W: ArcWake
        // Arc::clone
        let waker = task::waker(self.clone());
        // Create context with our waker
        let mut cx = Context::from_waker(&waker);

        // No other thread ever tries to lock the future
        let mut future = self.future.try_lock().unwrap();

        // Poll the future passing our `Context` to it.
        let _ = future.as_mut().poll(&mut cx);
    }

    // Spawns a new task with the given future.
    //
    // Initializes a new Task harness containing the given future and pushes it
    // onto `sender`. The receiver half of the channel will get the task and
    // execute it.
    fn spawn<F>(future: F, sender: &channel::Sender<Arc<Task>>)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let task = Arc::new(Task {
            future: Mutex::new(Box::pin(future)),
            executor: sender.clone(),
        });

        // Send task onto scheduled queue.
        let _ = sender.send(task);
    }
}
