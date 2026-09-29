// 49. Processes, environment, arguments, stdin, and exit codes
//
// std::env reads the process environment. args() iterates the command-line arguments
// as Strings, starting with the program name; args_os() is the version that tolerates
// non-UTF-8. var(name) returns a Result; var_os returns an Option<OsString>.
// current_dir is the working directory that relative paths resolve against. Changing
// the environment with set_var is unsafe in edition 2024, because another thread could
// be reading it at the same moment; configure a child process's environment instead.
//
// std::process::Command starts another program. output() runs it to completion and
// captures stdout and stderr; status() lets it share your terminal; spawn() starts it
// and returns immediately. Always check status.success().
//
// main can return ExitCode to set the exit status. process::exit stops the program
// immediately and skips destructors, so prefer returning from main.
//
// For flag parsing, the clap crate is the usual choice; std only hands you the raw list.
//
// Run: cargo run --bin 49_process_env -- one two --verbose

use std::env;
use std::io::{self, BufRead, IsTerminal, Write};
use std::process::{Command, ExitCode, Stdio};

// Example 1: command-line arguments.
fn demo_args() -> bool {
    println!("-- args --");

    let program = env::args().next().unwrap_or_default();
    println!("program name ends with {:?}", program.rsplit('/').next());

    let args: Vec<String> = env::args().skip(1).collect();
    println!("argc {} {:?}", args.len(), args);

    let verbose = args.iter().any(|a| a == "--verbose" || a == "-v");
    let positional: Vec<&String> = args.iter().filter(|a| !a.starts_with('-')).collect();
    println!("verbose {verbose} positional {positional:?}");

    // A simple --key=value parser.
    for a in &args {
        if let Some((k, v)) = a.strip_prefix("--").and_then(|kv| kv.split_once('=')) {
            println!("option {k} = {v}");
        }
    }

    verbose
}

// Example 2: environment variables.
fn demo_env_vars() {
    println!("-- env vars --");

    match env::var("HOME") {
        Ok(h) => println!("HOME is set ({} chars)", h.len()),
        Err(e) => println!("no HOME: {e}"),
    }

    println!("missing var: {:?}", env::var("BUZZARD_SURELY_UNSET"));
    let port: u16 = env::var("PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(8080);
    println!("PORT or default {port}");
    println!("var count > 0? {}", env::vars().count() > 0);
    println!(
        "os {} arch {} family {}",
        env::consts::OS,
        env::consts::ARCH,
        env::consts::FAMILY
    );

    // unsafe { env::set_var("KEY", "value") }; // edition 2024: set_var is unsafe
}

// Example 3: working directory and executable path.
fn demo_dirs() -> io::Result<()> {
    println!("-- dirs --");

    let cwd = env::current_dir()?;
    println!("current_dir is absolute: {}", cwd.is_absolute());

    let exe = env::current_exe()?;
    println!("current_exe file name {:?}", exe.file_name());
    println!("temp_dir exists: {}", env::temp_dir().exists());
    Ok(())
}

// Example 4: running a command and capturing its output.
fn demo_command_output() -> io::Result<()> {
    println!("-- Command::output --");

    let out = Command::new("echo").arg("hello").arg("world").output()?;
    println!(
        "ok {} stdout {:?}",
        out.status.success(),
        String::from_utf8_lossy(&out.stdout)
    );

    if cfg!(unix) {
        let out =
            Command::new("sh").args(["-c", "echo to-stderr >&2; exit 3"]).output()?;
        println!(
            "code {:?} stderr {:?}",
            out.status.code(),
            String::from_utf8_lossy(&out.stderr)
        );

        let out = Command::new("sh")
            .arg("-c")
            .arg("echo $GREETING from $PWD")
            .env("GREETING", "hi")
            .current_dir(env::temp_dir())
            .output()?;

        println!("with env and cwd: {}", String::from_utf8_lossy(&out.stdout).trim());
    }

    match Command::new("no-such-program-xyz").output() {
        Ok(_) => println!("unexpected"),
        Err(e) => println!("missing program: {:?}", e.kind()),
    }

    Ok(())
}

// Example 5: piping data into a child process.
fn demo_command_pipe() -> io::Result<()> {
    println!("-- piping to a child --");

    if !cfg!(unix) {
        return Ok(());
    }

    let mut child =
        Command::new("sort").stdin(Stdio::piped()).stdout(Stdio::piped()).spawn()?;
    child.stdin.take().unwrap().write_all(b"pear\napple\nfig\n")?; // dropping stdin closes it
    let out = child.wait_with_output()?;
    println!(
        "sorted by the child: {:?}",
        String::from_utf8_lossy(&out.stdout).lines().collect::<Vec<_>>()
    );
    Ok(())
}

// Example 6: reading stdin, but only when something is piped in.
fn demo_stdin() -> io::Result<()> {
    println!("-- stdin --");

    let stdin = io::stdin();
    if stdin.is_terminal() {
        println!(
            "stdin is a terminal; try: printf 'a\\nb\\n' | cargo run --bin 49_process_env"
        );
        return Ok(());
    }

    let mut lines = 0;
    let mut longest = String::new();
    for line in stdin.lock().lines() {
        let line = line?;
        lines += 1;

        if line.len() > longest.len() {
            longest = line;
        }
    }

    println!("read {lines} lines from stdin, longest {longest:?}");
    Ok(())
}

// Example 7: exit codes.
fn main() -> ExitCode {
    let verbose = demo_args();
    demo_env_vars();

    if let Err(e) =
        demo_dirs().and_then(|_| demo_command_output()).and_then(|_| demo_command_pipe())
    {
        eprintln!("error: {e}");
        return ExitCode::FAILURE;
    }

    if let Err(e) = demo_stdin() {
        eprintln!("stdin error: {e}");
        return ExitCode::from(2);
    }

    println!("-- exit --");

    if verbose {
        println!("verbose run finished");
    }

    // std::process::exit(0) would end here too, but would skip Drop for live values.
    println!("returning ExitCode::SUCCESS");
    ExitCode::SUCCESS
}
