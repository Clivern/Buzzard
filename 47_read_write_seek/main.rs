// 47. Files and the Read, Write, BufRead, and Seek traits
//
// std::io is built on a few traits. Read::read fills a buffer and returns how many bytes
// it read; read_to_end, read_to_string, and read_exact are helpers on top. Write::write
// may write only part of a buffer; write_all loops until everything is written, and
// flush pushes buffered bytes out. Seek moves the read/write position. BufRead adds
// line-oriented reading (lines, read_line) on top of a buffer.
//
// The same code works on anything that implements the traits: File, Stdin/Stdout, TCP
// streams, &[u8], Vec<u8>, and io::Cursor, which wraps an in-memory buffer and is handy
// in tests. BufReader and BufWriter wrap any reader or writer and batch the underlying
// system calls.
//
// Every I/O call returns io::Result. Bring the traits into scope with use std::io::...
// to get their methods.
//
// Run: cargo run --bin 47_read_write_seek

use std::fs::{self, File, OpenOptions};
use std::io::{
    self, BufRead, BufReader, BufWriter, Cursor, Read, Seek, SeekFrom, Write,
};
use std::path::PathBuf;

fn temp(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!("buzzard-47-{}-{name}", std::process::id()))
}

// Example 1: write a file, read it back, remove it.
fn demo_files() -> io::Result<()> {
    println!("-- files --");

    let path = temp("files.txt");
    let mut f = File::create(&path)?;
    f.write_all(b"hello file\n")?;
    writeln!(f, "line {}", 2)?;
    drop(f); // closes the file

    let s = fs::read_to_string(&path)?;
    print!("{s}");

    let bytes = fs::read(&path)?;
    println!("{} bytes", bytes.len());

    // One-liners for whole files.
    fs::write(&path, "replaced")?;
    println!("after fs::write: {}", fs::read_to_string(&path)?);
    fs::remove_file(&path)?;

    match File::open(&path) {
        Ok(_) => println!("unexpected"),
        Err(e) => println!("open missing file: {:?}", e.kind()),
    }

    Ok(())
}

// Example 2: OpenOptions for append and other modes.
fn demo_open_options() -> io::Result<()> {
    println!("-- OpenOptions --");

    let path = temp("log.txt");
    for i in 1..=3 {
        let mut f = OpenOptions::new().create(true).append(true).open(&path)?;
        writeln!(f, "entry {i}")?;
    }

    print!("{}", fs::read_to_string(&path)?);
    let exists = OpenOptions::new().write(true).create_new(true).open(&path);
    println!("create_new on existing file: {:?}", exists.map_err(|e| e.kind()).err());
    fs::remove_file(&path)
}

// Example 3: BufReader lines and BufWriter.
fn demo_buffered() -> io::Result<()> {
    println!("-- BufReader / BufWriter --");

    let path = temp("buf.txt");
    {
        let mut w = BufWriter::new(File::create(&path)?);
        for i in 1..=4 {
            writeln!(w, "row {i}")?;
        }

        w.flush()?; // drop would also flush, but would hide any error
    }

    let reader = BufReader::new(File::open(&path)?);
    for (n, line) in reader.lines().enumerate() {
        println!("{n}: {}", line?);
    }

    let mut reader = BufReader::new(File::open(&path)?);
    let mut first = String::new();
    let n = reader.read_line(&mut first)?;
    println!("read_line got {n} bytes: {first:?}");
    fs::remove_file(&path)
}

// Example 4: Read and Write over in-memory types.
fn demo_read_write_traits() -> io::Result<()> {
    println!("-- Read / Write traits --");

    let mut src: &[u8] = b"abcdef";
    let mut buf = [0u8; 4];
    let n = src.read(&mut buf)?;
    println!("read {n} bytes {:?}, left {:?}", &buf[..n], src);

    let mut rest = Vec::new();
    src.read_to_end(&mut rest)?;
    println!("rest {rest:?}");

    let mut out: Vec<u8> = Vec::new();
    out.write_all(b"written ")?;
    write!(out, "{}", 42)?;
    println!("{}", String::from_utf8(out).unwrap());

    // take limits a reader, chain joins two.
    let mut limited = String::new();
    b"0123456789".take(3).read_to_string(&mut limited)?;

    let mut joined = String::new();
    b"head-".chain(&b"tail"[..]).read_to_string(&mut joined)?;
    println!("take {limited} chain {joined}");
    Ok(())
}

fn count_words<R: BufRead>(r: R) -> io::Result<usize> {
    let mut total = 0;
    for line in r.lines() {
        total += line?.split_whitespace().count();
    }

    Ok(total)
}

// Example 5: generic functions over readers, tested with Cursor.
fn demo_generic_readers() -> io::Result<()> {
    println!("-- generic over Read --");

    println!("cursor words {}", count_words(Cursor::new("one two\nthree"))?);
    let path = temp("words.txt");
    fs::write(&path, "a b c d")?;
    println!("file words {}", count_words(BufReader::new(File::open(&path)?))?);
    fs::remove_file(&path)
}

// Example 6: Seek and Cursor.
fn demo_seek() -> io::Result<()> {
    println!("-- Seek --");

    let mut c = Cursor::new(b"abcdef".to_vec());
    c.seek(SeekFrom::Start(3))?;

    let mut two = [0u8; 2];
    c.read_exact(&mut two)?;
    println!(
        "after seek(3): {} pos {}",
        std::str::from_utf8(&two).unwrap(),
        c.position()
    );
    c.seek(SeekFrom::End(-1))?;
    c.write_all(b"Z")?; // overwrite the last byte
    c.seek(SeekFrom::Current(-3))?;
    c.write_all(b"XY")?;
    c.rewind()?;

    let mut all = String::new();
    c.read_to_string(&mut all)?;
    println!("edited in place: {all}");

    let inner = c.into_inner();
    println!("into_inner {} bytes", inner.len());

    // Seek works on files too; stdin and pipes usually cannot seek.
    let path = temp("seek.bin");
    let mut f = File::options()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)?;

    f.write_all(&[0u8; 8])?;
    f.seek(SeekFrom::Start(4))?;
    f.write_all(&[0xff])?;
    println!("stream_position {}", f.stream_position()?);
    println!("file bytes {:?}", fs::read(&path)?);
    fs::remove_file(&path)
}

// Example 7: io::copy and the standard streams.
fn demo_copy_and_std() -> io::Result<()> {
    println!("-- io::copy / stdout --");

    let mut src = Cursor::new("copied through io::copy\n");
    let mut stdout = io::stdout().lock();
    let n = io::copy(&mut src, &mut stdout)?;
    writeln!(stdout, "({n} bytes)")?;
    writeln!(io::stderr(), "stderr line")?;

    let mut sink = io::sink();
    io::copy(&mut io::repeat(b'x').take(1000), &mut sink)?;
    writeln!(stdout, "sink swallowed 1000 bytes")?;
    Ok(())
}

fn main() -> io::Result<()> {
    demo_files()?;
    demo_open_options()?;
    demo_buffered()?;
    demo_read_write_traits()?;
    demo_generic_readers()?;
    demo_seek()?;
    demo_copy_and_std()?;
    Ok(())
}
