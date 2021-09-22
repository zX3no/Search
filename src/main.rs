use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Bytes},
    path::{Path, PathBuf},
    time::Instant,
};

use jwalk::WalkDir;
use memmap2::MmapOptions;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::io::Write;
use text_io::read;

fn create_db() {
    let cdrive = Path::new("C:/");

    let mut buffer = File::create("files.db").unwrap();

    let new_line = PathBuf::from("\n");

    for entry in WalkDir::new(cdrive).sort(true).skip_hidden(false) {
        let mut temp = entry.unwrap().path();
        temp.push(&new_line);

        buffer
            .write_all(&temp.to_string_lossy().as_bytes())
            .unwrap();
    }
}

fn main() {
    // let now = Instant::now();
    // create_db();
    // println!("Elapsed {:?}", &now.elapsed());

    println!("Input search:");
    let input: String = read!();

    //this needs to be sped up
    let now = Instant::now();

    let file = File::open("files.db").expect("no such file");
    let buf = BufReader::new(file);
    for file in buf.lines() {
        let f = file.unwrap().clone();
        if f.contains(&input) {
            println!("{}", f);
        }
    }
    println!("Elapsed {:?}", &now.elapsed());
}
