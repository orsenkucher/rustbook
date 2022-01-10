use bytes::{Buf, BytesMut};
use mini_redis::{frame::Error::Incomplete, Frame, Result};
use std::io::Cursor;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt, BufWriter};
use tokio::net::TcpStream;

pub struct Connection {
    stream: BufWriter<TcpStream>,
    buffer: BytesMut,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Connection {
            stream: BufWriter::new(stream),
            // Allocate the buffer with 4kb of capacity
            buffer: BytesMut::with_capacity(4096),
        }
    }

    /// Read a frame from the connection.
    ///
    /// Returns `None` if EOF is reached
    pub async fn read_frame(&mut self) -> Result<Option<Frame>> {
        loop {
            // Attempt to parse a frame from the buffered data. If
            // enough data has been buffered, the frame is
            // returned.
            if let Some(frame) = self.parse_frame()? {
                return Ok(Some(frame));
            }

            // There is not enough buffered data to read a frame.
            // Attempt to read more data from socket.
            //
            // On success, the number of bytes is returned. `0`
            // indicates "end of stream".
            if 0 == self.stream.read_buf(&mut self.buffer).await? {
                // The remote closed the connection. For this to be
                // a clean shutdown, there should be no data in the
                // read buffer. If there is, this means that the
                // peer closed the socket while sending a frame.
                if self.buffer.is_empty() {
                    return Ok(None);
                } else {
                    return Err("connection reset by peer".into());
                }
            }
        }
    }

    fn parse_frame(&mut self) -> Result<Option<Frame>> {
        // Create the `T: Buf` type
        let mut buf = Cursor::new(&self.buffer[..]);

        // Check whether a full frame is available
        match Frame::check(&mut buf) {
            Ok(_) => {
                // Get the byte length of the frame
                let len = buf.position() as usize;

                // Reset the internal cursor for the call to `parse`.
                buf.set_position(0);

                // Parse the frame
                let frame = Frame::parse(&mut buf)?;

                // Discard the frame from the buffer
                self.buffer.advance(len);

                // Return the frame to the caller.
                Ok(Some(frame))
            }
            // Not enough data has been buffered
            Err(Incomplete) => Ok(None),
            // An error was encountered
            Err(e) => Err(e.into()),
        }
    }

    /// Write a frame to the connection.
    pub async fn write_frame(&mut self, frame: &Frame) -> io::Result<()> {
        match frame {
            Frame::Simple(val) => {
                self.stream.write_u8(b'+').await?;
                self.stream.write_all(val.as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            Frame::Error(val) => {
                self.stream.write_u8(b'-').await?;
                self.stream.write_all(val.as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            Frame::Integer(val) => {
                self.stream.write_u8(b':').await?;
                self.write_decimal(*val).await?;
            }
            Frame::Null => {
                self.stream.write_all(b"$-1\r\n").await?;
            }
            Frame::Bulk(val) => {
                let len = val.len();

                self.stream.write_u8(b'$').await?;
                self.write_decimal(len as u64).await?;
                self.stream.write_all(val).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            Frame::Array(_val) => unimplemented!(),
        }

        self.stream.flush().await?;

        Ok(())
    }

    /// Write a decimal frame to the stream
    async fn write_decimal(&mut self, val: u64) -> io::Result<()> {
        use std::io::Write;

        // Convert the value to a string
        let mut buf = [0u8; 12];
        let mut buf = Cursor::new(&mut buf[..]);
        write!(&mut buf, "{}", val)?;

        let pos = buf.position() as usize;
        self.stream.write_all(&buf.get_ref()[..pos]).await?;
        self.stream.write_all(b"\r\n").await?;

        Ok(())
    }
}

mod fut {
    use std::future::Future;
    use std::pin::Pin;
    use std::sync::{Arc, Mutex};
    use std::task::{Context, Poll, Waker};
    use std::thread;
    use std::time::{Duration, Instant};
    use tokio_stream::Stream;

    pub struct Delay {
        when: Instant,
        // This Some when we have spawned a thread, and None otherwise.
        waker: Option<Arc<Mutex<Waker>>>,
    }

    impl Delay {
        fn new(when: Instant) -> Self {
            Delay { when, waker: None }
        }
    }

    impl Future for Delay {
        type Output = &'static str;

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            // First, if this is the first time the future is called, spawn the
            // timer thread. If the timer thread is already running, ensure the
            // stored `Waker` matches the current task's waker.
            if let Some(waker) = &self.waker {
                let mut waker = waker.lock().unwrap();

                // Check if the stored waker matches the current task's waker.
                // This is necessary as the `Delay` future instance may move to
                // a different task between calls to `poll`. If this happens, the
                // waker contained by the given `Context` will differ and we
                // must update our stored waker to reflect this change.
                if !waker.will_wake(cx.waker()) {
                    *waker = cx.waker().clone();
                }
            } else {
                // Spawn new thread
                let when = self.when;
                let waker = Arc::new(Mutex::new(cx.waker().clone()));
                self.waker = Some(waker.clone());

                // This is the first time `poll` is called, spawn the timer thread.
                thread::spawn(move || {
                    let now = Instant::now();

                    if now < when {
                        thread::sleep(when - now);
                    }

                    // The duration has elapsed. Notify the caller by invoking
                    // the waker.
                    let waker = waker.lock().unwrap();
                    waker.wake_by_ref();
                });
            }

            // Once the waker is stored and the timer thread is started, it is
            // time to check if the delay has completed. This is done by
            // checking the current instant. If the duration has elapsed, then
            // the future has completed and `Poll::Ready` is returned.
            if Instant::now() >= self.when {
                println!("Delay done");
                Poll::Ready("done")
            } else {
                // The duration has not elapsed, the future has not completed so
                // return `Poll::Pending`.
                //
                // The `Future` trait contract requires that when `Pending` is
                // returned, the future ensures that the given waker is signalled
                // once the future should be polled again. In our case, by
                // returning `Pending` here, we are promising that we will
                // invoke the given waker included in the `Context` argument
                // once the requested duration has elapsed. We ensure this by
                // spawning the timer thread above.
                //
                // If we forget to invoke the waker, the task will hang
                // indefinitely.
                Poll::Pending
            }
        }
    }

    pub struct Interval {
        rem: usize,
        delay: Delay,
    }

    impl Stream for Interval {
        type Item = ();

        fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            if self.rem == 0 {
                // No more delays
                return Poll::Ready(None);
            }

            match Pin::new(&mut self.delay).poll(cx) {
                Poll::Ready(_) => {
                    let when = self.delay.when + Duration::from_millis(10);
                    self.delay = Delay::new(when);
                    self.rem -= 1;
                    Poll::Ready(Some(()))
                }
                Poll::Pending => Poll::Pending,
            }
        }
    }

    pub fn using_async_stream() -> impl Stream {
        use async_stream::stream;

        stream! {
            let mut when = Instant::now();
            for _ in 0..3 {
               let delay = Delay::new(when);
               delay.await;
               yield ();
               when += Duration::from_millis(10);
            }
        }
    }
}
