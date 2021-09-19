use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Bytes},
    path::{Path, PathBuf},
    time::Instant,
};

use jwalk::WalkDir;
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
fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
fn main() {
    // let now = Instant::now();
    // create_db();
    // println!("Elapsed {:?}", &now.elapsed());

    println!("Input search:");
    let input: String = read!();

    let now = Instant::now();
    //this needs to be sped up
    let data = lines_from_file("files.db");

    //it's not that fast but it's okay
    data.par_iter().for_each(|file| {
        if file.contains(&input) {
            println!("{}", file);
        }
    });

    println!("Elapsed {:?}", &now.elapsed());
}
