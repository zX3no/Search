use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader},
    path::Path,
    time::Instant,
};

use jwalk::WalkDir;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::io::Write;
use text_io::read;

fn create_db() {
    let cdrive = Path::new("C:/");

    File::create("files.db").unwrap();

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("files.db")
        .unwrap();

    for entry in WalkDir::new(cdrive).sort(true).skip_hidden(false) {
        // let de = entry.as_ref().unwrap();
        // let owned = de.file_name().to_owned();
        // let string = owned.to_string_lossy();

        if let Err(e) = writeln!(file, "{}", entry.as_ref().unwrap().path().to_string_lossy()) {
            eprintln!("Couldn't write to file: {}", e);
        }
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
    create_db();
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
