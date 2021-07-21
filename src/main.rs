use rayon::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
use std::thread;
use std::time::Instant;
use walkdir::DirEntry;
use walkdir::WalkDir;

fn recurse(dir: &str, file: &File) -> Result<()> {
    for entry in WalkDir::new(dir)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        file.write_all(entry.file_name().to_string_lossy().as_bytes())?;
        file.write_all(b"\n")?;
    }

    Ok(())
}

fn main() -> Result<()> {
    let now = Instant::now();
    //let file = File::create("files.db")?;

    for entry in WalkDir::new("../../../../")
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        thread::spawn(move || {
            recurse(&entry.file_name().to_string_lossy(), &file).ok();
        });
    }

    //wait for all threads to finish

    println!("{}", now.elapsed().as_secs());

    Ok(())
}
