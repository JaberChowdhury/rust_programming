// CONCEPT: Implement `Future` manually — no async/await, no runtime magic.
// WHY: To understand the core polling mechanism of Rust's async design.

use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
// use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

struct ReadyFuture<T>(Option<T>);

impl<T: Unpin> Future for ReadyFuture<T> {
    type Output = T;
    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("Polling ReadyFuture...");
        Poll::Ready(self.0.take().unwrap())
    }
}

struct PendingOnceFuture {
    polled_once: bool,
}

impl Future for PendingOnceFuture {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("Polling PendingOnceFuture...");
        if self.polled_once {
            Poll::Ready(())
        } else {
            self.polled_once = true;
            // In a real future, we'd store the waker and wake it later.
            // Here, we just wake immediately to re-queue it.
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

// Minimal manual executor
type Task = Pin<Box<dyn Future<Output = ()> + Send>>;

struct DummyWaker;

impl DummyWaker {
    fn raw_waker() -> RawWaker {
        fn no_op(_: *const ()) {}
        fn clone(_: *const ()) -> RawWaker {
            DummyWaker::raw_waker()
        }
        let vtable = &RawWakerVTable::new(clone, no_op, no_op, no_op);
        RawWaker::new(std::ptr::null(), vtable)
    }

    fn waker() -> Waker {
        unsafe { Waker::from_raw(Self::raw_waker()) }
    }
}

fn run_executor(mut tasks: VecDeque<Task>) {
    let waker = DummyWaker::waker();
    let mut cx = Context::from_waker(&waker);

    while let Some(mut task) = tasks.pop_front() {
        if task.as_mut().poll(&mut cx).is_pending() {
            tasks.push_back(task);
        }
    }
}

fn main() {
    let mut tasks: VecDeque<Task> = VecDeque::new();

    tasks.push_back(Box::pin(async {
        let f = ReadyFuture(Some("Hello"));
        let res = f.await;
        println!("ReadyFuture completed with: {}", res);
    }));

    tasks.push_back(Box::pin(async {
        let f = PendingOnceFuture { polled_once: false };
        f.await;
        println!("PendingOnceFuture completed");
    }));

    println!("Starting manual executor...");
    run_executor(tasks);
}

// EXERCISE:
// 1. Create a `PendingTwiceFuture` that returns `Poll::Pending` two times before resolving.
// 2. Add more logging in the executor loop to see when tasks are pushed back to the queue.
