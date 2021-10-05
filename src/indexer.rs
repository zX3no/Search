use std::{
    collections::VecDeque,
    fs::File,
    io::{BufWriter, Read, Write},
    ops::RangeBounds,
    path::{Path, PathBuf},
    rc::Rc,
    sync::{Arc, Mutex, RwLock},
    thread,
    time::Instant,
};

use jwalk::WalkDir;
use rayon::{
    iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator},
    str::ParallelString,
};

use crate::index::Index;

pub struct Indexer {
    index: Option<Vec<Index>>,
}

impl Indexer {
    pub fn new() -> Self {
        return Self {
            index: Indexer::read(),
        };
    }

    pub fn is_empty(&self) -> bool {
        return self.index.is_none();
    }
    pub fn update(&mut self) {
        self.index = Indexer::read();
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

    pub fn database_exists() -> bool {
        if Path::new("index.db").exists() {
            return true;
        }
        return false;
    }

    pub fn read() -> Option<Vec<Index>> {
        if !Indexer::database_exists() {
            return None;
        }
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
        let mut s = String::with_capacity(file_len as usize);
        file.read_to_string(&mut s).unwrap();

        let mut out = Vec::with_capacity(s.len() as usize + 1);

        for index in s.split('\t') {
            out.push(Index::new(index.to_string()));
        }

        println!("reading {} items took {:?}", s.len(), now.elapsed());
        // return Some(out);
        return Some(out);
    }

    pub fn search(&self, query: &str) -> VecDeque<String> {
        let now = Instant::now();
        if let Some(index) = &self.index {
            let output: VecDeque<String> = index
                .par_iter()
                .filter_map(|index| {
                    if index.file_name.contains(query) {
                        return Some(index.file_name.clone());
                    }
                    None
                })
                .collect();

            println!("searching took {:?}", now.elapsed());
            return output;
        } else {
            return VecDeque::new();
        };
    }
}
