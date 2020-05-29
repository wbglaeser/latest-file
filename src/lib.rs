use std::fs;
use std::time::SystemTime;
use chrono::{Local, DateTime};
use std::path::PathBuf;
use std::ffi::OsString;
use std::fs::DirEntry;
use std::fmt;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Cli {
    #[structopt(parse(from_os_str))]
    pub path: std::path::PathBuf,

    #[structopt(short = "e", default_value = " ")]
    pub exclude: String,

    #[structopt(short = "f")]
    pub faulty_files: bool,
}

pub struct FileEntry {
    pub modified_at: SystemTime,
    pub created_at: SystemTime,
    pub is_directory: bool,
    pub full_path: PathBuf,
    pub file_name: OsString,
}

impl FileEntry {
    pub fn new() -> Self {
        Self {
            modified_at: SystemTime::UNIX_EPOCH,
            created_at: SystemTime::UNIX_EPOCH,
            is_directory: false,
            full_path: PathBuf::new(),
            file_name: OsString::new(),
        }
    }

    pub fn update(&mut self, dir_entry: &DirEntry) {

        let meta_data = fs::metadata(dir_entry.path()).unwrap();

        // creation date
        self.created_at = meta_data.created().unwrap();

        // modification date
        self.modified_at = meta_data.modified().unwrap();

        // is directory
        self.is_directory = meta_data.is_dir();

        // full file path
        self.full_path = dir_entry.path();

        // file name
        self.file_name = dir_entry.file_name();
    }
}

impl fmt::Display for FileEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let modified_date: DateTime<Local> = DateTime::from(self.modified_at);
        let created_date: DateTime<Local> = DateTime::from(self.created_at);
        write!(f, "\nFilename: {:?}\nFull path: {:?}\nModified: {:?}\nCreated: {:?}\nDirectory: {:?}\n", self.file_name, self.full_path, modified_date, created_date, self.is_directory)
    }
}

pub fn parse_dir(dir_path: PathBuf, mut latest_file: &mut FileEntry, exclude: &String, faulty: bool) {
    
    if let Ok(dir_list) = fs::read_dir(dir_path) {
        
        for p in dir_list {
        
            if let Ok(path) = p {
            
                if path.file_name().into_string().unwrap().contains(exclude) {
                    continue;
                }

                if let Ok(metadata) = fs::metadata(&path.path()) {
    
                    if metadata.is_dir() == true {
                        parse_dir(path.path(), &mut latest_file, exclude, faulty);
                    }
        
                    if metadata.modified().unwrap() > latest_file.modified_at {
                        latest_file.update(&path)
                    }
           
                } else { 
                    if faulty {
                        println!("Faulty file: {:?}", path.path()); 
                    }
                }
            } else { println!("Faulty file"); }
        }
    } else { println!("Faulty file"); }
}
