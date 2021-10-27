use std::{thread, time::Duration};

//An index is like a PathBuf but the
//format is optimized for reading and writing

//Data layout:
//File path \n is directory(bool) \n file size(u32) \r
//file size is not finished

//todo rename from index? wtf is an index? how about path?

//TODO PATH AND FILES ARE BROKEN

#[derive(Debug, Clone)]
pub struct Index {
    pub path: String,
    pub file_name: String,
    pub file_size: u32,
}
impl Default for Index {
    fn default() -> Self {
        Self {
            path: String::new(),
            file_name: String::new(),
            file_size: 0,
        }
    }
}

impl Index {
    pub fn new(slice: &str) -> Self {
        Self {
            path: Index::path(slice),
            file_name: Index::file_name(slice),
            // file_size: Index::file_size(&slice),
            file_size: 0,
        }
    }
    pub fn dir(&self) {
        todo!("strip file name out of path");
    }
    pub fn path(slice: &str) -> String {
        let mut path = String::new();
        for c in slice.chars() {
            if c == '\n' {
                break;
            }

            path.push(c);
        }

        return path;
    }
    pub fn file_name(slice: &str) -> String {
        let path = Index::path(&slice);

        let mut name = String::new();

        for c in path.chars().rev() {
            if c == '\\' {
                break;
            }
            name.push(c);
        }

        let out = name.chars().rev().collect();
        return out;
    }
    pub fn is_dir(slice: &String) -> bool {
        let mut newlines = 0;
        for c in slice.chars() {
            if newlines == 1 {
                if c == '\u{1}' {
                    return true;
                }
                break;
            }
            if c == '\n' {
                newlines += 1;
            }
        }
        return false;
    }
    pub fn file_size(slice: &String) -> u32 {
        let mut size = String::new();
        slice.chars().for_each(|c| {
            let mut newlines = 0;
            if c == '\n' {
                newlines += 1;
            } else if c == '\r' {
                return;
            }
            if newlines == 2 {
                size.push(c);
            }
        });
        return size.parse::<u32>().unwrap();
    }
}
