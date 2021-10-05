use std::{thread, time::Duration};

//An index is like a PathBuf but the
//format is optimized for reading and writing

//Data layout:
//File path \n is directory(bool) \n file size(u32) \r
pub struct Index {
    path: String,
    file_name: Option<String>,
    file_size: u32,
}
impl Index {
    pub fn new(slice: String) -> Self {
        Self {
            path: Index::path(&slice),
            file_name: Index::file_name(&slice),
            file_size: Index::file_size(&slice),
        }
    }
    pub fn path(slice: &String) -> String {
        let mut path = String::new();
        for c in slice.chars() {
            if c == '\n' {
                break;
            }

            path.push(c);
        }

        return path;
    }
    pub fn file_name(slice: &String) -> Option<String> {
        //todo this does not return folder names
        if !Index::is_dir(&slice) {
            let path = Index::path(&slice);

            let mut name = String::new();
            let mut backslash = 0;

            for c in path.chars().rev() {
                if c == '\\' {
                    break;
                    // dbg!(&c, &path);
                }
                name.push(c);
            }

            let out = name.chars().rev().collect();
            return Some(out);
        }
        return None;
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
