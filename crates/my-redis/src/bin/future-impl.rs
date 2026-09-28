use std::ops::Add;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

struct Delay {
    when: Instant,
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.when {
            println!("Hello world");
            Poll::Ready("done")
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

#[allow(dead_code)]
struct Interval {
    rem: usize,
    delay: Delay,
}

impl Interval {
    #[allow(dead_code)]
    fn new() -> Self {
        Self {
            rem: 3,
            delay: Delay {
                when: Instant::now(),
            },
        }
    }
}

impl tokio_stream::Stream for Interval {
    type Item = ();

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.rem == 0 {
            return Poll::Ready(None);
        }

        match Pin::new(&mut self.delay).poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(_) => {
                let when = self.delay.when.add(Duration::from_millis(10));
                self.delay = Delay { when };
                self.rem = self.rem.saturating_sub(1);
                Poll::Ready(Some(()))
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let when = Instant::now().add(Duration::from_secs(1));
    let future = Delay { when };

    let out = future.await;
    assert_eq!(out, "done");
}

#[allow(dead_code)]
enum MainFuture {
    // Initialized, never polled
    State0,
    // Waiting on Delay i.e. the future.await line
    State1(Delay),
    // The future has completed
    Terminated,
}

impl Future for MainFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        use MainFuture::{State0, State1, Terminated};

        loop {
            match *self {
                State0 => {
                    let when = Instant::now().add(Duration::from_secs(1));
                    let future = Delay { when };
                    *self = State1(future);
                }
                State1(ref mut my_future) => match Pin::new(my_future).poll(cx) {
                    Poll::Ready(out) => {
                        assert_eq!(out, "done");
                        *self = Terminated;
                        return Poll::Ready(());
                    }
                    Poll::Pending => {
                        return Poll::Pending;
                    }
                },
                #[allow(clippy::panic)]
                Terminated => {
                    panic!("future polled after completion");
                }
            }
        }
    }
}
