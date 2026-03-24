// 187. The Future trait
//
// Future::Output is the result type. poll takes Pin<&mut Self> and a Context with a
// Waker. Ready(value) is done. Pending means call poll again when the waker is woken.
// You almost never impl Future by hand; async fn does it.
//
// Run: cargo run --bin 187_future_trait

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct Once(bool);

impl Future for Once {
    type Output = &'static str;
    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.0 {
            Poll::Pending
        } else {
            self.0 = true;
            Poll::Ready("done")
        }
    }
}

fn main() {
    println!("{}", std::mem::size_of::<Once>());
}
