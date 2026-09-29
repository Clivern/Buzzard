// 50. Duration, Instant, SystemTime, and sleep
//
// Duration is a non-negative span of time. Build one with from_secs, from_millis,
// from_micros, from_nanos, or from_secs_f64; read it back with as_secs, as_millis,
// subsec_millis, or as_secs_f64. Durations add, subtract (checked or saturating), and
// multiply.
//
// Instant is a monotonic clock reading: it never goes backwards, so it is the right
// tool for measuring how long something took. It has no meaning outside the running
// process. SystemTime is the wall clock: it can be shown to users and compared with
// UNIX_EPOCH, but it can jump when the OS clock changes. For calendars and time zones,
// use a crate such as chrono or jiff.
//
// thread::sleep pauses the current thread for at least the given Duration. In async
// code, use the runtime's sleep instead so other tasks keep running.
//
// Run: cargo run --bin 50_time

use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

// Example 1: building and reading Durations.
fn demo_duration() {
    println!("-- Duration --");

    let d = Duration::from_millis(1500);
    println!(
        "{}s {}ms subsec {}ms {:.3}s",
        d.as_secs(),
        d.as_millis(),
        d.subsec_millis(),
        d.as_secs_f64()
    );
    println!(
        "{:?} {:?} {:?}",
        Duration::from_secs(90),
        Duration::from_micros(250),
        Duration::from_secs_f64(0.125)
    );
    println!("ZERO {:?} MAX secs {}", Duration::ZERO, Duration::MAX.as_secs());
}

// Example 2: Duration arithmetic.
fn demo_duration_math() {
    println!("-- Duration math --");

    let a = Duration::from_secs(5);
    let b = Duration::from_millis(1500);
    println!("a + b {:?} a - b {:?} a * 3 {:?} a / 4 {:?}", a + b, a - b, a * 3, a / 4);
    println!(
        "checked_sub {:?} saturating_sub {:?}",
        b.checked_sub(a),
        b.saturating_sub(a)
    );
    println!("mul_f64 {:?}", a.mul_f64(0.1));

    let total: Duration = [b, b, b].iter().sum();
    println!("sum {total:?} max {:?}", a.max(b));

    // Duration cannot be negative:
    // let neg = b - a; // panics: overflow when subtracting durations
}

fn format_hms(d: Duration) -> String {
    let s = d.as_secs();
    format!("{:02}:{:02}:{:02}", s / 3600, (s / 60) % 60, s % 60)
}

// Example 3: formatting a Duration for people.
fn demo_formatting() {
    println!("-- formatting --");

    for secs in [59, 3_725, 90_061] {
        println!("{secs}s -> {}", format_hms(Duration::from_secs(secs)));
    }
}

// Example 4: measuring elapsed time with Instant.
fn demo_instant() {
    println!("-- Instant --");

    let start = Instant::now();
    thread::sleep(Duration::from_millis(5));

    let elapsed = start.elapsed();
    println!("slept at least 5ms: {}", elapsed >= Duration::from_millis(5));

    let later = Instant::now();
    println!("monotonic: {}", later >= start);
    println!("duration_since {:?}", later.duration_since(start) >= elapsed);

    // Earlier minus later saturates to zero instead of going negative.
    println!("reverse saturates to {:?}", start.saturating_duration_since(later));

    // Timing a piece of work.
    let t = Instant::now();
    let sum: u64 = (0..1_000_000u64).sum();
    println!(
        "sum {sum} took less than a second: {}",
        t.elapsed() < Duration::from_secs(1)
    );
}

// Example 5: deadlines and timeouts.
fn demo_deadline() {
    println!("-- deadline --");

    let deadline = Instant::now() + Duration::from_millis(20);
    let mut rounds = 0;
    while Instant::now() < deadline {
        rounds += 1;
        thread::sleep(Duration::from_millis(2));
    }

    println!("ran a few rounds before the deadline: {}", rounds > 1);
    let remaining = deadline.checked_duration_since(Instant::now());
    println!("time left after deadline {remaining:?}");
}

// Example 6: wall-clock time.
fn demo_system_time() {
    println!("-- SystemTime --");

    let now = SystemTime::now();
    let since_epoch = now.duration_since(UNIX_EPOCH).expect("clock is after 1970");
    println!("unix seconds > 1.7e9: {}", since_epoch.as_secs() > 1_700_000_000);

    let earlier = now - Duration::from_secs(60);
    println!("one minute ago is earlier: {}", earlier < now);
    println!(
        "duration_since a later time is Err: {}",
        earlier.duration_since(now).is_err()
    );

    let days = since_epoch.as_secs() / 86_400;
    println!("days since epoch > 19000: {}", days > 19_000);

    let t = UNIX_EPOCH + Duration::from_secs(1_000_000_000);
    println!("1e9 seconds after epoch: {:?}", t.duration_since(UNIX_EPOCH).unwrap());
}

// Example 7: sleep and yield.
fn demo_sleep() {
    println!("-- sleep --");

    let t = Instant::now();
    thread::sleep(Duration::from_millis(3));
    println!(
        "sleep is at least as long as asked: {}",
        t.elapsed() >= Duration::from_millis(3)
    );
    thread::yield_now(); // hint to the scheduler; sleep(Duration::ZERO) is not a reliable yield
    println!("awake");
}

fn main() {
    demo_duration();
    demo_duration_math();
    demo_formatting();
    demo_instant();
    demo_deadline();
    demo_system_time();
    demo_sleep();
}
