use std::{
    ffi::c_void,
    fs::File,
    io::{BufReader, BufWriter, Write},
    mem,
    os::windows::prelude::{AsRawHandle, RawHandle},
    path::{Path, PathBuf},
    ptr,
    str::from_utf8,
    time::Instant,
};

use bytecodec::{
    io::{IoDecodeExt, IoEncodeExt},
    null::{NullDecoder, NullEncoder},
    Encode,
};
use filesize::PathExt;
use jwalk::WalkDir;
use patricia_tree::{
    node::{NodeDecoder, NodeEncoder},
    PatriciaSet,
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use sysinfo::{DiskExt, System, SystemExt};
use winapi::{
    shared::minwindef::{FILETIME, LPVOID},
    um::{
        fileapi::{
            GetFileAttributesExA, GetFileAttributesExW, GetFileInformationByHandle,
            BY_HANDLE_FILE_INFORMATION, WIN32_FILE_ATTRIBUTE_DATA,
        },
        minwinbase::GetFileExInfoStandard,
    },
};
fn wide_null(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}
pub struct Database {
    set: PatriciaSet,
}
impl Database {
    pub fn para_metadata() {
        let now = Instant::now();

        let mut paths = Vec::new();

        for file in WalkDir::new("D:\\").sort(false).skip_hidden(false) {
            paths.push(file.unwrap().path());
        }

        println!("{:?}", now.elapsed());
        let now = Instant::now();

        //TODO: I feel like it may be possible to speed this up

        let metadata: Vec<_> = paths
            .par_iter()
            .map(|path| {
                let x = std::fs::metadata(path);
                x
            })
            .collect();

        dbg!(metadata.len(), paths.len());

        println!("{:?}", now.elapsed());
    }
    pub fn test() {
        let now = Instant::now();
        let file = File::open("index.db").unwrap();
        println!("{:?}", now.elapsed());

        let ft = FILETIME {
            dwLowDateTime: 0,
            dwHighDateTime: 0,
        };
        let mut info = WIN32_FILE_ATTRIBUTE_DATA {
            dwFileAttributes: 0,
            ftCreationTime: ft,
            ftLastAccessTime: ft,
            ftLastWriteTime: ft,
            nFileSizeHigh: 0,
            nFileSizeLow: 0,
        };

        let now = Instant::now();
        unsafe {
            // let mut info: BY_HANDLE_FILE_INFORMATION = mem::zeroed();
            // GetFileInformationByHandle(file.as_raw_handle(), &mut info);
            // let finfo: LPVOID;

            // let finfo = &mut info as *mut _ as *mut c_void;

            // GetFileAttributesExW(wide_null("in").as_ptr(), GetFileExInfoStandard, finfo);
            // dbg!(test);
        }
        println!("{:?}", now.elapsed());
    }
    pub fn test2() {
        let now = Instant::now();

        let drives = vec![Path::new("C:\\")];

        let file = File::create("index.db").unwrap();
        let mut writer = BufWriter::new(&file);

        //get all the files in the drives

        let mut set = PatriciaSet::new();
        for drive in drives {
            for file in WalkDir::new(drive).sort(false).skip_hidden(false) {
                set.insert(file.as_ref().unwrap().path().to_str().unwrap());

                let now = Instant::now();
                let _ = file.unwrap().metadata();
                println!("{:?}", now.elapsed());
            }
        }
        let out: Vec<_> = set.iter().collect();

        for vec in out {
            writer.write_all(&vec).unwrap();
        }

        println!("{:?}", now.elapsed());
    }
    pub fn test1() -> PatriciaSet {
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

        set
    }
    pub fn create() -> Self {
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
        let file = File::create("files.db").unwrap();

        let mut encoder = NodeEncoder::new(NullEncoder);
        encoder.start_encoding(set.into()).unwrap();
        encoder.encode_all(file).unwrap();

        Database::read()
    }
    //~1s
    //this is way to slow
    pub fn read() -> Self {
        let now = Instant::now();

        let file = File::open("files.db").unwrap();
        let reader = BufReader::new(file);

        let mut decoder = NodeDecoder::new(NullDecoder);
        let node = decoder.decode_exact(reader).unwrap();

        let set = PatriciaSet::from(node);

        println!("reading: {:?}", now.elapsed());

        Self { set }
    }
    //this shouldn't be this slow
    pub fn fzf(&self, query: &str) -> Vec<String> {
        let keys = self.set.iter().collect::<Vec<_>>();
        let mut out = Vec::new();

        for item in keys {
            if let Ok(str) = std::str::from_utf8(&item) {
                if str.contains(query) {
                    out.push(str.to_string());
                }
            }
        }

        out
    }
}
