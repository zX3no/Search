use std::{io::Write, path::PathBuf};

use jwalk::WalkDir;
fn main() -> std::io::Result<()> {
    let now = std::time::Instant::now();
    let mut buffer = std::fs::File::create("files.db")?;
    let mut new_line: PathBuf = PathBuf::new();
    new_line.push("\n");
    let path: PathBuf = [r"D:\"].iter().collect();

    for entry in WalkDir::new(path).sort(true) {
        let mut temp = entry?.path();
        temp.push(&new_line);
        buffer.write_all(&temp.to_string_lossy().as_bytes())?;
        // println!("{}", entry?.path().display());
    }

    println!("{}", now.elapsed().as_millis());
    Ok(())
}
