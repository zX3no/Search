use std::{
    collections::VecDeque,
    fs::File,
    io::{BufWriter, Read, Write},
    os::windows::prelude::MetadataExt,
    path::{Path, PathBuf},
    time::Instant,
};

use jwalk::WalkDir;
use rayon::iter::{ParallelBridge, ParallelIterator};

pub struct Indexer {
    index: String,
}
impl Indexer {
    pub fn new() -> Self {
        Self {
            index: Indexer::read(),
        }
    }

    pub fn create() {
        let paths: Vec<&Path> = vec![Path::new(r"C:\"), Path::new(r"D:\")];
        Indexer::scan_and_write(paths);
    }

    pub fn scan_and_write(drives: Vec<&Path>) {
        let now = Instant::now();

        let file = File::create("index.db").unwrap();
        let mut writer = BufWriter::new(file);

        let newline: &[u8] = &['\n' as u8];

        for drive in drives.iter() {
            for file in WalkDir::new(drive).sort(true).skip_hidden(false) {
                if let Ok(f) = file {
                    writer
                        .write(&[f.path().to_string_lossy().as_bytes(), newline].concat())
                        .unwrap();
                };
            }
        }
        println!("indexing took {:?}", now.elapsed());
    }

    fn database_exists() -> bool {
        if Path::new("index.db").exists() {
            return true;
        }
        return false;
    }

    pub fn read() -> String {
        let now = Instant::now();

        let mut file = File::open("index.db").unwrap();
        let file_len = file.metadata().unwrap().len();

        // old faster version
        // let mut v: Vec<u8> = Vec::with_capacity(file_len as usize + 1);
        // file.read_to_end(&mut v).unwrap();
        // println!(
        //     "Finished reading {} items, took: {:?}",
        //     v.len(),
        //     now.elapsed()
        // );

        //6-9 ms slower but uses string instead
        let mut s = String::with_capacity(file_len as usize + 1);
        file.read_to_string(&mut s).unwrap();

        println!("reading {} items took {:?}", s.len(), now.elapsed());

        return s;
    }
    pub fn search(&self, query: &str) -> Vec<String> {
        let now = Instant::now();

        let mut buffer = Vec::new();
        for file in self.index.split("\n") {
            if file.contains(query) {
                buffer.push(file.to_string());
            }
        }

        println!("searching took {:?}", now.elapsed());

        return buffer;
    }
}
