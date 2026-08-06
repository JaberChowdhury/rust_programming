// CONCEPT: Build a real `Future` implementation from scratch.
// WHY: To prove that you understand exactly how Tokio's event loop interacts with `Waker`.

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::thread;
use std::time::{Duration, Instant};

// 1. TimerFuture: Spawns a background OS thread to wait, then wakes the async task.
struct TimerFuture {
    deadline: Instant,
    started: bool,
}

impl Future for TimerFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.deadline {
            Poll::Ready(())
        } else {
            if !self.started {
                let waker = cx.waker().clone();
                let wait_time = self.deadline.duration_since(Instant::now());

                // Spawn a blocking thread just to sleep and wake.
                // In a real runtime like Tokio, epoll/kqueue handles this without extra threads.
                thread::spawn(move || {
                    thread::sleep(wait_time);
                    waker.wake(); // Tell the executor to poll us again!
                });
                self.started = true;
            }
            Poll::Pending
        }
    }
}

// 2. YieldNow: Returns Pending exactly once, immediately queuing itself to run again.
struct YieldNow(bool);

impl Future for YieldNow {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.0 {
            Poll::Ready(())
        } else {
            self.0 = true;
            // Wake immediately so the executor schedules us on the next tick
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

// 3. Poll3Times Wrapper
struct Poll3Times<F> {
    inner: F,
    count: u8,
}

impl<F: Future + Unpin> Future for Poll3Times<F> {
    type Output = F::Output;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.count += 1;
        println!("Poll3Times: Poll #{}", self.count);

        let inner = Pin::new(&mut self.inner);
        inner.poll(cx)
    }
}

#[tokio::main]
async fn main() {
    println!("--- TimerFuture ---");
    let timer = TimerFuture {
        deadline: Instant::now() + Duration::from_millis(500),
        started: false,
    };
    println!("Awaiting timer...");
    timer.await;
    println!("Timer finished!");

    println!("\n--- YieldNow ---");
    println!("Before yield");
    YieldNow(false).await;
    println!("After yield");

    println!("\n--- Poll3Times Wrapper ---");
    let wrapped = Poll3Times {
        inner: YieldNow(false),
        count: 0,
    };
    wrapped.await;
}

// EXERCISE:
// 1. Create a future that never wakes up (returns `Poll::Pending` without cloning/calling the waker). What happens when you await it?
// 2. Modify `TimerFuture` to support being polled multiple times with new deadlines.
