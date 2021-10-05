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
    iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelBridge, ParallelIterator},
    str::ParallelString,
};

use crate::index::Index;

pub struct Indexer {
    pub index: Option<Vec<Index>>,
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
        //todo get drives from system
        let drives: Vec<&Path> = vec![Path::new(r"C:\"), Path::new(r"D:\")];
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

        let mut s = String::with_capacity(file_len as usize);
        file.read_to_string(&mut s).unwrap();

        let output = s.par_split('\t').map(|index| Index::new(index)).collect();

        println!("reading {} items took {:?}", s.len(), now.elapsed());

        return Some(output);
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
