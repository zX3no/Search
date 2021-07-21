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
static GLOBAL_THREAD_COUNT: AtomicUsize = AtomicUsize::new(0);

fn main() -> Result<()> {
    let now = Instant::now();
    let file = Arc::new(Mutex::new(File::create("files.db").unwrap()));
    for entry in WalkDir::new("../../")
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let file = Arc::clone(&file);

        if entry.file_type().is_dir() {
            thread::spawn(move || {
                GLOBAL_THREAD_COUNT.fetch_add(1, Ordering::SeqCst);
                let mut file = file.lock().unwrap();
                for x in WalkDir::new(entry.path())
                    .into_iter()
                    .filter_map(|e| e.ok())
                {
                    //WTF?
                    let temp = x.file_name().to_string_lossy() + "\n";
                    file.write_all(temp.as_bytes()).ok();
                }

                GLOBAL_THREAD_COUNT.fetch_sub(1, Ordering::SeqCst);
            });
        } else {
            let mut file = file.lock().unwrap();
            let temp = entry.file_name().to_string_lossy() + "\n";
            file.write_all(temp.as_bytes()).ok();
        }
    }

    //wait for all threads to finish
    while GLOBAL_THREAD_COUNT.load(Ordering::SeqCst) != 0 {}

    println!("{}", now.elapsed().as_millis());

    Ok(())
}
