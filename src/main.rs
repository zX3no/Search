use std::{
    ffi::OsStr,
    fs::{File, Metadata},
    io::{BufRead, BufReader},
    os::windows::prelude::MetadataExt,
    path::Path,
    time::Instant,
};

use app::TemplateApp;
use eframe::NativeOptions;
use jwalk::{DirEntry, Error, Parallelism, WalkDir, WalkDirGeneric};
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::io::Write;
use text_io::read;

mod app;

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
// fn read_file(file_name: &str, input: &String) {
//     let file = File::open(file_name).expect("no such file");
//     let buf = BufReader::new(file);
//     for line in buf.lines() {
//         if line.as_ref().unwrap().contains(input) {
//             println!("{}", line.as_ref().unwrap());
//         }
//     }
// }
// fn search() {
//     let now = Instant::now();
//     // create_db(r"C:\");
//     // create_db(r"D:\");
//     println!("Elapsed {:?}", &now.elapsed());

//     println!("Input search:");
//     let mut input: String = read!("{}\n");
//     //there is a weird "\r" character at the end?
//     input.pop();

//     let now = Instant::now();
//     read_file("C.db", &input);
//     read_file("D.db", &input);
//     println!("Input was: {}", &input);
//     println!("Elapsed {:?}", &now.elapsed());
// }

fn main() {
    let app = TemplateApp::default();
    eframe::run_native(Box::new(app), NativeOptions::default());

    // let path = Path::new(r"C:/");

    // let now = Instant::now();
    // let mut file_name = Box::new("");
    // for file in WalkDir::new(path) {
    //     if let Ok(f) = file {
    //         file_name = Box::new(f.file_name().to_str().unwrap());
    //     };
    // }
    // println!("Elapsed {:?}", &now.elapsed());

    //jwalk does not store data this way it's more like a tree
    //C:/
    //  C:/Windows
    //Win depends on it's root so you can't remove it

    // for entry in WalkDirGeneric::<((bool), (bool))>::new(path)
    //     .skip_hidden(false)
    //     //depth, root path, state of root(skip?), the child dir
    //     .process_read_dir(move |_depth, root, _state, child| {
    //         child.iter_mut().for_each(|path_result| {
    //             if let Ok(path) = path_result {
    //                 if path.path().parent().unwrap_or(Path::new("")) == root {
    //                     path.client_state = true;
    //                 }
    //             }
    //         });
    //         child.retain(move |path_result| {
    //             if let Ok(path) = path_result {
    //                 if path.path().parent().unwrap_or(Path::new("")) == root {
    //                     return true;
    //                 }
    //                 if path.file_name().to_string_lossy().contains(".mp3") {
    //                     return true;
    //                 }
    //             }
    //             return false;
    //         });
    //     })
    // {
    //     if !entry.as_ref().unwrap().path().is_dir() {
    //         println!("{}", entry.unwrap().path().display());
    //     }
    // }
}
