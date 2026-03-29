// 203. process exit
//
// process::exit stops the process immediately and skips Drop. That can leak or skip
// flushing. Prefer returning from main with an ExitCode. abort() is even harder: no
// destructors, no atexit. Use Result in main for recoverable failure.
//
// Run: cargo run --bin 203_process_exit

use std::process::ExitCode;

fn main() -> ExitCode {
    println!("ok");
    ExitCode::SUCCESS
}
