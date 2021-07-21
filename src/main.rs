use rayon::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;
use walkdir::DirEntry;
use walkdir::WalkDir;

fn main() -> Result<()> {
    let now = Instant::now();

    let mut file = File::create("files.db")?;

    for entry in WalkDir::new("../../").into_iter().filter_map(|e| e.ok()) {
        let temp = entry.file_name().to_string_lossy() + "\n";
        file.write_all(temp.as_bytes())?;
    }

    //wait for all threads to finish

    println!("{}", now.elapsed().as_millis());

    Ok(())
}
