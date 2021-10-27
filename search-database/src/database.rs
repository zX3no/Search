use std::{
    path::{Path, PathBuf},
    str::from_utf8,
};

use filesize::PathExt;
use jwalk::WalkDir;
use patricia_tree::PatriciaSet;
use sysinfo::{DiskExt, System, SystemExt};

pub struct Database {}
impl Database {
    pub fn test() {
        //get all the drives
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

        let paths: Vec<PathBuf> = f.iter().map(|f| f.path()).collect();

        let mut set = PatriciaSet::new();
        for path in paths {
            set.insert(path.to_str().unwrap());
        }

        dbg!(set.contains("D:\\Git"));

        //print all paths as string
        let split: Vec<_> = set.split_by_prefix("D:\\Git\\search!").iter().collect();
        for string in split {
            let _str = from_utf8(&string).unwrap();
            // dbg!(_str);
        }
    }
    pub fn create() -> PatriciaSet {
        let mut sys = System::new_all();
        sys.refresh_all();
        let drives: Vec<&Path> = sys.disks().iter().map(|disk| disk.mount_point()).collect();

        let mut f = Vec::new();
        for drive in drives {
            for file in WalkDir::new(drive).sort(false).skip_hidden(false) {
                f.push(file.unwrap());
            }
        }

        let paths: Vec<PathBuf> = f.iter().map(|f| f.path()).collect();
        let mut set = PatriciaSet::new();

        for path in paths {
            set.insert(path.to_str().unwrap());
        }

        set
    }
}
