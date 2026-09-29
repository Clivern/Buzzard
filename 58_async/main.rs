// 58. async/await, the Future trait, and Pin
//
// An async fn returns a value that implements Future instead of running right away.
// Nothing happens until something polls the future. .await, inside another async fn or
// block, polls a future and pauses the caller until it is ready. The compiler turns each
// async body into a state machine.
//
// Future has one method: poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Output>.
// Poll::Ready(v) means done; Poll::Pending means "not yet". A future that returns
// Pending must arrange for cx.waker() to be woken when it can make progress, so the
// executor knows to poll it again.
//
// The standard library defines Future but ships no executor. Real programs use a
// runtime such as tokio, smol, or async-std. This lesson builds a tiny block_on to show
// what an executor does: poll, and wait to be woken.
//
// Pin<P> promises the pointed-to value will not move again. An async state machine may
// hold references into itself, which moving would break. Most types are Unpin, which
// means pinning them changes nothing. Box::pin and the pin! macro pin a future so it
// can be polled.
//
// Run: cargo run --bin 58_async

use std::future::Future;
use std::pin::{Pin, pin};
use std::sync::{Arc, Condvar, Mutex};
use std::task::{Context, Poll, Wake, Waker};
use std::thread;
use std::time::Duration;

// A minimal executor: poll the future; when it is Pending, sleep until woken.
struct Signal {
    woken: Mutex<bool>,
    cv: Condvar,
}

impl Wake for Signal {
    fn wake(self: Arc<Self>) {
        *self.woken.lock().unwrap() = true;
        self.cv.notify_one();
    }
}

fn block_on<F: Future>(fut: F) -> F::Output {
    let mut fut = pin!(fut);
    let signal = Arc::new(Signal { woken: Mutex::new(false), cv: Condvar::new() });

    let waker = Waker::from(Arc::clone(&signal));
    let mut cx = Context::from_waker(&waker);
    loop {
        if let Poll::Ready(v) = fut.as_mut().poll(&mut cx) {
            return v;
        }

        let mut woken = signal.woken.lock().unwrap();
        while !*woken {
            woken = signal.cv.wait(woken).unwrap();
        }
        *woken = false;
    }
}

async fn answer() -> i32 {
    42
}

async fn double_answer() -> i32 {
    let a = answer().await;
    a * 2
}

// Example 1: async fn is lazy until polled.
fn demo_async_fn() {
    println!("-- async fn --");

    let fut = double_answer(); // nothing has run yet
    println!("created a future of {} bytes", size_of_val(&fut));
    println!("block_on -> {}", block_on(fut));
}

// Example 2: polling by hand with a no-op waker.
fn demo_manual_poll() {
    println!("-- manual poll --");

    let mut fut = Box::pin(answer());
    let mut cx = Context::from_waker(Waker::noop());
    match fut.as_mut().poll(&mut cx) {
        Poll::Ready(v) => println!("ready {v}"),
        Poll::Pending => println!("pending"),
    }
}

// Example 3: a hand-written Future that is pending a few times.
struct CountDown {
    left: u32,
}

impl Future for CountDown {
    type Output = &'static str;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.left == 0 {
            Poll::Ready("liftoff")
        } else {
            println!("  poll: {} left", self.left);
            self.left -= 1;
            cx.waker().wake_by_ref(); // ask to be polled again
            Poll::Pending
        }
    }
}

fn demo_custom_future() {
    println!("-- custom Future --");

    println!("{}", block_on(CountDown { left: 3 }));
}

// Example 4: a timer future woken by another thread.
struct Timer {
    state: Arc<Mutex<(bool, Option<Waker>)>>,
}

impl Timer {
    fn new(d: Duration) -> Self {
        let state = Arc::new(Mutex::new((false, None::<Waker>)));
        let s = Arc::clone(&state);
        thread::spawn(move || {
            thread::sleep(d);
            let mut g = s.lock().unwrap();
            g.0 = true;

            if let Some(w) = g.1.take() {
                w.wake();
            }
        });

        Timer { state }
    }
}

impl Future for Timer {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let mut g = self.state.lock().unwrap();
        if g.0 {
            Poll::Ready(())
        } else {
            g.1 = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

async fn slow_add(a: i32, b: i32) -> i32 {
    Timer::new(Duration::from_millis(5)).await;
    a + b
}

fn demo_timer() {
    println!("-- timer future --");

    let start = std::time::Instant::now();
    let v = block_on(async {
        let x = slow_add(1, 2).await;
        let y = slow_add(x, 10).await;
        y
    });

    println!(
        "result {v} after >= 10ms: {}",
        start.elapsed() >= Duration::from_millis(10)
    );
}

// Example 5: async blocks, captures, and async closures.
fn demo_async_blocks() {
    println!("-- async blocks --");

    let name = String::from("ada");
    let greet = async move { format!("hello {name}") }; // owns name
    println!("{}", block_on(greet));

    let add = async |a: i32, b: i32| a + b;
    println!("async closure {}", block_on(add(2, 3)));

    let futures = vec![
        Box::pin(answer()) as Pin<Box<dyn Future<Output = i32>>>,
        Box::pin(async { 7 }),
    ];

    let total: i32 = futures.into_iter().map(block_on).sum();
    println!("boxed dyn futures sum {total}");
}

// Example 6: running two futures concurrently with a hand-written join.
struct Join2<A: Future, B: Future> {
    a: Pin<Box<A>>,
    b: Pin<Box<B>>,
    ra: Option<A::Output>,
    rb: Option<B::Output>,
}

impl<A: Future, B: Future> Future for Join2<A, B>
where
    A::Output: Unpin,
    B::Output: Unpin,
{
    type Output = (A::Output, B::Output);
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = &mut *self;
        if this.ra.is_none()
            && let Poll::Ready(v) = this.a.as_mut().poll(cx)
        {
            this.ra = Some(v);
        }

        if this.rb.is_none()
            && let Poll::Ready(v) = this.b.as_mut().poll(cx)
        {
            this.rb = Some(v);
        }

        if this.ra.is_some() && this.rb.is_some() {
            Poll::Ready((this.ra.take().unwrap(), this.rb.take().unwrap()))
        } else {
            Poll::Pending
        }
    }
}

fn join2<A: Future, B: Future>(a: A, b: B) -> Join2<A, B> {
    Join2 { a: Box::pin(a), b: Box::pin(b), ra: None, rb: None }
}

fn demo_join() {
    println!("-- concurrent join --");

    let start = std::time::Instant::now();
    let (x, y) = block_on(join2(slow_add(1, 1), slow_add(2, 2)));

    // Both timers ran at the same time, so this takes about 5ms, not 10ms.
    println!(
        "({x}, {y}) in under 10ms: {}",
        start.elapsed() < Duration::from_millis(10)
    );
}

// Example 7: Pin and Unpin.
fn demo_pin() {
    println!("-- Pin --");

    let mut n = 1;
    let mut p = Pin::new(&mut n); // allowed because i32 is Unpin
    *p = 2;
    println!("Unpin values work like normal through Pin: {n}");

    let fut = pin!(async { 5 }); // async blocks are not Unpin, so pin before polling
    println!("pinned async block -> {}", block_on(fut));

    // let mut f = async { 1 };
    // Pin::new(&mut f);
    // error[E0277]: `{async block}` cannot be unpinned
}

fn main() {
    demo_async_fn();
    demo_manual_poll();
    demo_custom_future();
    demo_timer();
    demo_async_blocks();
    demo_join();
    demo_pin();
}
