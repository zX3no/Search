use jwalk::WalkDir;
use std::fs;
use std::io::BufWriter;
use std::{io::Write, path::PathBuf};
fn main() -> std::io::Result<()> {
    let now = std::time::Instant::now();
    let file = fs::File::create("files.db")?;
    let mut file = BufWriter::new(file);

    let new_line: PathBuf = ["\n"].iter().collect();
    let path: PathBuf = [r"C:\"].iter().collect();

    for entry in WalkDir::new(path).sort(true) {
        let mut temp = entry?.path();
        temp.push(&new_line);
        file.write_all(&temp.to_string_lossy().as_bytes())?;
    }

    println!("{}", now.elapsed().as_millis());
    Ok(())
}
