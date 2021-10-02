use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    sync::mpsc::channel,
    thread,
    time::Instant,
};

use eframe::egui::Label;
// fn read_file<'a>(file_name: &str, input: &String) -> VecDeque<&'a Label> {
//     let file = File::open(file_name).expect("no such file");
//     let buf = BufReader::new(file);
//     let mut out: VecDeque<&'a Label> = VecDeque::new();
//     for line in buf.lines() {
//         if line.as_ref().unwrap().contains(input) {
//             out.push_back(&Label::from(line.as_ref().unwrap().to_string()));
//         }
//     }
//     return out;
// }
pub fn search(input: String) -> String {
    let (send, recv) = channel();
    thread::spawn(move || {
        let file = File::open("C.db").expect("no such file");
        let buf = BufReader::new(file);
        let mut out = String::new();
        for line in buf.lines() {
            if line.as_ref().unwrap().contains("sus") {
                send.send(line);
            }
        }
    });
    let result = recv.recv();
    return result.unwrap().unwrap().to_string();
    // let mut c = read_file("C.db", &input);
    // let mut d = read_file("D.db", &input);
    // c.append(&mut d);
    // return c;
}
