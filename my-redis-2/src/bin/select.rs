use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::sync::{mpsc, oneshot};

struct MySelect {
    rx1: oneshot::Receiver<&'static str>,
    rx2: oneshot::Receiver<&'static str>,
}

impl Future for MySelect {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if let Poll::Ready(val) = Pin::new(&mut self.rx1).poll(cx) {
            println!("rx1 completed first with {:?}", val);
            return Poll::Ready(());
        }

        if let Poll::Ready(val) = Pin::new(&mut self.rx2).poll(cx) {
            println!("rx2 completed first with {:?}", val);
            return Poll::Ready(());
        }

        Poll::Pending
    }
}

#[tokio::main]
async fn main() {
    let (tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    tokio::spawn(async {
        let _ = tx1.send("one");
    });
    tokio::spawn(async {
        let _ = tx2.send("two");
    });

    MySelect { rx1, rx2 }.await;

    let (mut tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    tokio::spawn(async {
        // Select on the operation and the oneshot's
        // `closed()` notification.
        tokio::select! {
            val = some_operation()=>{
                let _ = tx1.send(val);
            }
            _ = tx1.closed()=>{
                // `some_operation()` is canceled, the
                // task completes and `tx1` is dropped.
            }
        };
    });

    tokio::spawn(async {
        let _ = tx2.send("two");
    });

    tokio::select! {
        val = rx1 =>{
            println!("rx1 completed first with {:?}", val)
        }
        val = rx2 =>{
            println!("rx2 completed first with {:?}", val)
        }
    }

    let (mut tx1, mut rx1) = mpsc::channel(128);
    let (mut tx2, mut rx2) = mpsc::channel(128);

    tokio::spawn(async move {
        // Do something w/ `tx1` and `tx2`
        tx1.send(1).await;
        tx2.send(2).await;
        // tx1.send(1);
        // tx2.send(2);
    });

    tokio::select! {
        Some(v) = rx1.recv() => {
            println!("Got {:?} from rx1", v);
        }
        Some(v) = rx2.recv() => {
            println!("Got {:?} from rx2", v);
        }
        else => {
            println!("Both channels closed");
        }
    }

    let (mut tx, mut rx) = mpsc::channel::<i32>(128);

    let operation = action(None);
    tokio::pin!(operation);

    loop {
        tokio::select! {
            _ = &mut operation =>break,
            Some(v) = rx.recv()=>{
                if v%2 ==0{
                    break;
                }
            }
        }
    }

    let (tx, mut rx) = mpsc::channel(128);

    let mut done = false;
    let operation = action(None);
    tokio::pin!(operation);

    tokio::spawn(async move {
        let _ = tx.send(1).await;
        let _ = tx.send(3).await;
        let _ = tx.send(2).await;
    });

    loop {
        tokio::select! {
            res = &mut operation, if !done => {
                done = true;

                if let Some(v) = res {
                   println!("GOT {v}") ;
                   return
                }
            }
            Some(v) = rx.recv() => {
                if v % 2 == 0 {
                    // `.set` is a method on `Pin`
                    operation.set(action(Some(v)));
                    done = false;
                }
            }
        };
    }
}

async fn action(input: Option<i32>) -> Option<String> {
    // let i = match input {
    //     Some(input) => input,
    //     None => return None,
    // };
    let i = input?;

    // async logic here
    Some(i.to_string())
}

async fn i32op(i: i32) -> i32 {
    i * i
}

async fn some_operation() -> String {
    String::from("Compute value here")
}
