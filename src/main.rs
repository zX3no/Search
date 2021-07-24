use jwalk::{DirEntry, WalkDir, WalkDirGeneric};
use std::cmp::Ordering;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use sysinfo::{DiskExt, SystemExt};

fn main() -> std::io::Result<()> {
    //Get Disks
    let mut disks: Vec<&str> = Vec::new();
    let system = sysinfo::System::new_all();

    for disk in system.disks() {
        disks.push(disk.mount_point().to_str().unwrap());
    }
    let walk_dir = WalkDirGeneric::<((usize), (bool))>::new(r"C:\tools").process_read_dir(
        |depth, path, read_dir_state, children| {
            // 2. Custom filter
            children.retain(|dir_entry_result| {
                dir_entry_result
                    .as_ref()
                    .map(|dir_entry| {
                        dir_entry
                            .file_name
                            .to_str()
                            .map(|s| s.starts_with('.'))
                            .unwrap_or(false)
                    })
                    .unwrap_or(false)
            });
        },
    );

    let path: PathBuf = [disks[0]].iter().collect();
    let drive = WalkDir::new(&path).sort(true);

    for entry in walk_dir {
        dbg!(entry)?;
    }
    let now = std::time::Instant::now();

    //Don't write to file
    //Store drive in memory
    //Get user input that searches the memory

    println!("{}", now.elapsed().as_millis());

    /*
    //todo get first character from disks[0]
    let db_name: &str = disks[0];
    let file = fs::File::create(db_name)?;
    let mut file = BufWriter::new(file);
    let mut files: Vec<OsString> = vec![OsString::new()];

    // This is faster than putting the file writes
    // where the vector push is
    for entry in drive {
        files.push(entry?.file_name);
    }

    for f in files {
        file.write_all(f.to_string_lossy().as_bytes())?;
        file.write_all("\n".as_bytes())?;
    }
    */

    Ok(())
}
