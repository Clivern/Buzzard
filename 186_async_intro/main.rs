// 186. async and await
//
// async fn returns impl Future. .await polls that future until it is ready. The standard
// library has the Future trait but no executor: you need tokio, async-std, or smol to
// run async code. This lesson only shows the types, and block_on from futures is not in
// std, so we poll a ready future by hand.
//
// Run: cargo run --bin 186_async_intro

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

fn ready_waker() -> Waker {
    fn clone(_: *const ()) -> RawWaker {
        raw()
    }
    fn wake(_: *const ()) {}
    fn wake_by_ref(_: *const ()) {}
    fn drop(_: *const ()) {}
    fn raw() -> RawWaker {
        RawWaker::new(std::ptr::null(), &RawWakerVTable::new(clone, wake, wake_by_ref, drop))
    }
    unsafe { Waker::from_raw(raw()) }
}

async fn answer() -> i32 {
    42
}

fn main() {
    let mut fut = Box::pin(answer());
    let waker = ready_waker();
    let mut cx = Context::from_waker(&waker);
    match Pin::as_mut(&mut fut).poll(&mut cx) {
        Poll::Ready(v) => println!("{v}"),
        Poll::Pending => println!("pending"),
    }
}
