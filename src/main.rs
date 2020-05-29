use std::fs;
use std::time::SystemTime;
use chrono::{Utc, DateTime};
use std::path::PathBuf;
use std::ffi::OsString;
use std::fs::DirEntry;
use std::fmt;

#[derive(Debug)]
struct FileEntry {
    pub modified_at: SystemTime,
    pub full_path: PathBuf,
    pub file_name: OsString,
}

impl FileEntry {
    pub fn new() -> Self {
        Self {
            modified_at: SystemTime::UNIX_EPOCH,
            full_path: PathBuf::new(),
            file_name: OsString::new(),
        }
    }

    pub fn update(&mut self, dir_entry: &DirEntry) {
        
        // creation date
        let meta_data = fs::metadata(dir_entry.path()).unwrap();
        self.modified_at = meta_data.modified().unwrap();  
        
        // full file path
        self.full_path = dir_entry.path();

        // file name
        self.file_name = dir_entry.file_name();
    }
}

impl fmt::Display for FileEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let parsed_date: DateTime<Utc> = DateTime::from(self.modified_at);
        write!(f, "Filename: {:?}\nFull path: {:?}\nModified: {:?}", self.file_name, self.full_path, parsed_date)
    }
}

fn parse_dir(dir_path: PathBuf, mut latest_file: &mut FileEntry) {

    for p in fs::read_dir(dir_path).unwrap() {
        let path = p.unwrap();
        if fs::metadata(&path.path()).unwrap().is_dir() == true {
            parse_dir(path.path(), &mut latest_file);
        }
        let modified_time = fs::metadata(&path.path()).unwrap().modified().unwrap();
        if modified_time > latest_file.modified_at {
            latest_file.update(&path);
        }
    }
}

fn main() {
    
    let mut dir_name = PathBuf::new();
    dir_name.push(r"./");

    let mut latest_file = FileEntry::new();

    parse_dir(dir_name, &mut latest_file);

    println!("{}", latest_file);

}
