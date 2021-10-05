use std::{
    collections::VecDeque,
    fs::File,
    io::{BufWriter, Read, Write},
    path::{Path, PathBuf},
    time::Instant,
};

use jwalk::WalkDir;
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::index::Index;

pub struct Indexer {
    index: String,
}

impl Indexer {
    pub fn new() -> Self {
        if Indexer::database_exists() {
            return Self {
                index: Indexer::read(),
            };
        }
        panic!("no database");
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
        let end: &[u8] = &['\t' as u8];

        for drive in drives.iter() {
            for file in WalkDir::new(drive).sort(true).skip_hidden(false) {
                if let Ok(f) = file {
                    let out = &[
                        f.path().to_string_lossy().as_bytes(),
                        newline,
                        &[f.file_type.is_dir() as u8],
                        newline,
                        &[0],
                        end,
                    ]
                    .concat();

                    writer.write_all(out).unwrap();
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

    pub fn search(&self, query: &str) -> VecDeque<String> {
        let now = Instant::now();

        let mut buffer = VecDeque::new();
        for index in self.index.split('\t') {
            if let Some(file_name) = Index::file_name(&index.to_string()) {
                buffer.push_back(file_name);
            }
        }
        // for file in self.index.split("\n") {
        //     if file.contains(query) {
        //         buffer.push_back(file.to_string());
        //     }
        // }

        println!("searching took {:?}", now.elapsed());

        return buffer;
    }
}
