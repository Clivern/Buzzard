// 48. Paths, directories, and metadata
//
// Path is to PathBuf what str is to String: a borrowed view and an owned buffer. join
// and push append components; parent, file_name, file_stem, and extension take paths
// apart. Paths are OS strings and need not be UTF-8: display() prints them lossily and
// to_str() returns an Option.
//
// std::fs covers directories (create_dir_all, read_dir, remove_dir_all), copying and
// renaming, and metadata: file type, length, permissions, and timestamps.
//
// Checking first and acting later races: a file can disappear between exists() and
// open(). Prefer doing the operation and handling the NotFound error. For walking large
// trees, the walkdir crate is the usual choice; std handles one level at a time.
//
// Run: cargo run --bin 48_fs_paths

use std::fs;
use std::io;
use std::path::{Component, Path, PathBuf};

// Example 1: taking a path apart.
fn demo_path_parts() {
    println!("-- path parts --");

    let p = Path::new("/var/log/app/server.log.gz");
    println!("parent {:?}", p.parent());
    println!(
        "file_name {:?} stem {:?} ext {:?}",
        p.file_name(),
        p.file_stem(),
        p.extension()
    );
    println!("absolute {} relative {}", p.is_absolute(), Path::new("a/b").is_relative());

    let parts: Vec<_> = p.components().collect();
    println!(
        "{} components, first is root: {}",
        parts.len(),
        parts[0] == Component::RootDir
    );

    let names: Vec<_> = p.iter().map(|s| s.to_string_lossy()).collect();
    println!("iter {names:?}");
    println!(
        "ancestors {:?}",
        p.ancestors().map(|a| a.display().to_string()).collect::<Vec<_>>()
    );
}

// Example 2: building and changing paths.
fn demo_path_building() {
    println!("-- building paths --");

    let mut p = PathBuf::from("dir1");
    p.push("dir2");
    p.push("s.log");
    println!("{}", p.display());
    p.set_extension("txt");
    p.set_file_name("other.md");
    println!("{}", p.display());

    let joined = Path::new("/etc").join("nginx").join("nginx.conf");
    println!(
        "{} with_extension {}",
        joined.display(),
        joined.with_extension("bak").display()
    );

    // Joining an absolute path replaces everything before it.
    println!("join absolute: {}", Path::new("/a/b").join("/c").display());
    println!("strip_prefix {:?}", joined.strip_prefix("/etc"));
    println!(
        "starts_with {} ends_with {}",
        joined.starts_with("/etc"),
        joined.ends_with("nginx.conf")
    );
}

// Example 3: paths and strings.
fn demo_path_strings() {
    println!("-- paths and strings --");

    let p = Path::new("café/menu.txt");
    let s: Option<&str> = p.to_str();
    let lossy = p.to_string_lossy();
    let owned: String = p.display().to_string();
    println!("{s:?} {lossy} {owned}");

    let from_string: PathBuf = String::from("x/y").into();
    println!("{}", from_string.display());

    // Functions should take impl AsRef<Path> so callers can pass &str, String, or Path.
    fn ext_of(p: impl AsRef<Path>) -> Option<String> {
        p.as_ref().extension()?.to_str().map(String::from)
    }

    println!("{:?} {:?}", ext_of("a.rs"), ext_of(PathBuf::from("noext")));
}

fn scratch_dir() -> PathBuf {
    std::env::temp_dir().join(format!("buzzard-48-{}", std::process::id()))
}

// Example 4: creating and listing directories.
fn demo_dirs() -> io::Result<()> {
    println!("-- directories --");

    let root = scratch_dir();
    fs::create_dir_all(root.join("sub/deeper"))?;
    fs::write(root.join("a.txt"), b"a")?;
    fs::write(root.join("b.rs"), b"fn main() {}")?;
    fs::write(root.join("sub/c.txt"), b"ccc")?;

    let mut entries: Vec<_> = fs::read_dir(&root)?.collect::<Result<_, _>>()?;
    entries.sort_by_key(|e| e.file_name());

    for e in &entries {
        let kind = if e.file_type()?.is_dir() { "dir " } else { "file" };
        println!("{kind} {}", e.file_name().to_string_lossy());
    }

    Ok(())
}

fn walk(dir: &Path, depth: usize, out: &mut Vec<String>) -> io::Result<()> {
    let mut entries: Vec<_> = fs::read_dir(dir)?.collect::<Result<_, _>>()?;
    entries.sort_by_key(|e| e.file_name());

    for e in entries {
        out.push(format!("{}{}", "  ".repeat(depth), e.file_name().to_string_lossy()));
        if e.file_type()?.is_dir() {
            walk(&e.path(), depth + 1, out)?;
        }
    }

    Ok(())
}

// Example 5: a small recursive walk.
fn demo_walk() -> io::Result<()> {
    println!("-- recursive walk --");

    let mut lines = Vec::new();
    walk(&scratch_dir(), 0, &mut lines)?;
    println!("{}", lines.join("\n"));
    Ok(())
}

// Example 6: metadata, copy, rename, and errors.
fn demo_metadata() -> io::Result<()> {
    println!("-- metadata --");

    let root = scratch_dir();
    let file = root.join("sub/c.txt");
    let meta = fs::metadata(&file)?;
    println!("len {} is_file {} is_dir {}", meta.len(), meta.is_file(), meta.is_dir());
    println!(
        "readonly {} modified known? {}",
        meta.permissions().readonly(),
        meta.modified().is_ok()
    );
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        println!("unix mode {:o}", meta.permissions().mode() & 0o777);
    }

    let copy = root.join("copy.txt");
    let n = fs::copy(&file, &copy)?;
    fs::rename(&copy, root.join("renamed.txt"))?;
    println!(
        "copied {n} bytes, renamed exists? {}",
        root.join("renamed.txt").try_exists()?
    );

    match fs::metadata(root.join("missing")) {
        Err(e) if e.kind() == io::ErrorKind::NotFound => println!("missing: NotFound"),
        other => println!("unexpected {other:?}"),
    }

    let canonical = fs::canonicalize(&root)?;
    println!("canonical is absolute: {}", canonical.is_absolute());
    Ok(())
}

// Example 7: cleanup.
fn demo_cleanup() -> io::Result<()> {
    println!("-- cleanup --");

    let root = scratch_dir();
    fs::remove_file(root.join("a.txt"))?;

    // remove_dir only removes an empty directory.
    println!(
        "remove_dir on non-empty: {:?}",
        fs::remove_dir(&root).map_err(|e| e.kind()).is_err()
    );
    fs::remove_dir_all(&root)?;
    println!("gone? {}", !root.exists());
    Ok(())
}

fn main() -> io::Result<()> {
    demo_path_parts();
    demo_path_building();
    demo_path_strings();
    demo_dirs()?;
    demo_walk()?;
    demo_metadata()?;
    demo_cleanup()?;
    Ok(())
}
