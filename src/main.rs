use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    time::Instant,
};

use jwalk::{DirEntry, WalkDir};
use std::io::Write;
use text_io::read;

fn create_db(drive: &str) {
    let drive = Path::new(drive);

    let db_name: String = drive.to_str().unwrap()[..1].to_string() + ".db";

    let mut buffer = File::create(Path::new(db_name.as_str())).unwrap();

    let newline: &[u8] = &['\n' as u8];

    let mut path_name = String::new();
    let mut file_size: u64 = 0;
    let mut e: DirEntry<((), ())>;

    for entry in WalkDir::new(drive).sort(true).skip_hidden(false) {
        buffer
            .write_all(
                &[
                    entry.as_ref().unwrap().path().to_string_lossy().as_bytes(),
                    newline,
                ]
                .concat(),
            )
            .unwrap();
    }

    return;

    // for entry in WalkDir::new(drive).sort(true).skip_hidden(false) {
    //     let now = Instant::now();
    //     e = entry.unwrap();

    //     path_name = e.path().to_str().unwrap().to_string();

    //     file_size = match e.metadata() {
    //         Ok(metadata) => metadata.file_size(),
    //         Err(_) => 0,
    //     };

    //     buffer
    //         .write_all(
    //             &[
    //                 path_name.as_bytes(),
    //                 newline,
    //                 &file_size.to_le_bytes(),
    //                 newline,
    //             ]
    //             .concat(),
    //         )
    //         .unwrap();

    //     println!("Loop {:?}", &now.elapsed());
    // }
}
fn read_file(file_name: &str, input: &String) {
    let file = File::open(file_name).expect("no such file");
    let buf = BufReader::new(file);
    for line in buf.lines() {
        if line.as_ref().unwrap().contains(input) {
            println!("{}", line.as_ref().unwrap());
        }
    }
}

fn main() {
    let now = Instant::now();
    // create_db(r"C:\");
    // create_db(r"D:\");
    println!("Elapsed {:?}", &now.elapsed());

    println!("Input search:");
    let input: String = read!();

    let now = Instant::now();
    read_file("C.db", &input);
    read_file("D.db", &input);
    println!("Input was: {}", &input);
    println!("Elapsed {:?}", &now.elapsed());
}
