use std::{
    ffi::OsString,
    path::{Path, PathBuf},
    time::Instant,
};

use filesize::PathExt;
use jwalk::WalkDir;
use sysinfo::{DiskExt, System, SystemExt};

pub struct Indexer {}
impl Indexer {
    pub fn create() {
        //~300ms
        let mut sys = System::new_all();
        sys.refresh_all();

        //get all the drives
        // let drives: Vec<&Path> = sys.disks().iter().map(|disk| disk.mount_point()).collect();
        let drives = vec![Path::new("D:\\")];

        //get all the files in the drives
        let mut f = Vec::new();
        for drive in drives {
            for file in WalkDir::new(drive).sort(false).skip_hidden(false) {
                f.push(file.unwrap());
            }
        }

        //get metadata
        let metadata = f.last().unwrap().metadata().unwrap();
        //get filesize
        let realsize = f
            .last()
            .unwrap()
            .path()
            .size_on_disk_fast(&metadata)
            .unwrap();

        // let path = Path::new("Cargo.toml");
        // let metadata = path.symlink_metadata().unwrap();
        // let realsize = path.size_on_disk_fast(&metadata).unwrap();

        dbg!(metadata, realsize);

        let now = Instant::now();
        println!("{:?}", now.elapsed());

        //[slow]
        let paths: Vec<PathBuf> = f.iter().map(|f| f.path()).collect();
    }
}
